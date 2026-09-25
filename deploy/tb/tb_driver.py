#!/usr/bin/env python3
"""One blocking TB arm. Privilege belongs exclusively to operator `next`.

Uses #516's pinned probe readers from a local, reviewed git-archive extraction.
It never calls the old driver's sudo/admin verbs or rewrites source records.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import struct
import sys
import time

from tb_order import Order, Refused, atomic, check, notice, sha


def readers(plan):
    root = Path(plan['instrument'])
    for name, path in [('confirm_cells', root / 'cross-precision-repro/confirm_cells.py'),
                       ('weaver_probe', root / 'weaver-probe/weaver_probe.py')]:
        # One read: these bytes are hashed and these bytes are compiled. The
        # path names the code in tracebacks only; no cached pyc and no second
        # read of the file is ever executed.
        data = path.read_bytes()
        check('reader-approved', hashlib.sha256(data).hexdigest() == plan['files'][str(path)])
        module = importlib.util.module_from_spec(importlib.util.spec_from_file_location(name, path))
        sys.modules[name] = module
        exec(compile(data, str(path), 'exec'), module.__dict__)
    return module


def parse(data):
    return [json.loads(line) for line in data.splitlines() if line.strip()]


def events(path):
    return parse(Path(path).read_bytes())


def until_closed(path, kind, timeout=3600):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if path.exists():
            # Ignore only the currently incomplete final line while the writer
            # runs. A malformed complete line is a refusal, not dropped evidence.
            with path.open('rb') as stream:
                stream.seek(0, 2)
                size = stream.tell()
                stream.seek(max(0, size - 65536))
                tail = stream.read()
            lines = tail.split(b'\n')[1:] if size > 65536 else tail.split(b'\n')
            for line in lines[:-1]:
                if line and json.loads(line).get('kind') == kind:
                    return events(path)
        time.sleep(1)
    raise Refused(f'{kind} not recorded inside bound')


def enough(rec):
    n = len(rec['output_tokens'])
    check('nonempty-measurement', n > 0 and len(rec['entropies']) == n and len(rec['field']) == n)
    check('field-depth', all(len(value['ranked']) == 200 for value in rec['field'].values()))


def float_bits(values):
    return b''.join(struct.pack('!d', v) for v in values)


def exact(source, replay):
    enough(source)
    enough(replay)
    return (source['output_tokens'] == replay['output_tokens'] and
            float_bits(source['entropies']) == float_bits(replay['entropies']) and
            {int(k): v for k, v in source['field'].items()} == {int(k): v for k, v in replay['field'].items()})


def source_record(plan, job, probe):
    if job.get('source_job'):
        return json.loads((Path(plan['deposit']) / 'runs' / job['source_job'] / 'run.json').read_text())
    path = Path(job['source_trace'])
    # One read, hashed and parsed: the record the report compares against is
    # cut from the bytes the manifest approved, the same selection the payload
    # makes for the replay it feeds.
    data = path.read_bytes()
    check('source-trace', hashlib.sha256(data).hexdigest() == plan['files'][str(path)])
    rows = [e for e in parse(data) if e['run'] == job['source_run']]
    check('source-measurement', sum(e['kind'] == 'model.measurement' for e in rows) == 1)
    return dict(probe.extract_run(rows), trace=str(path), run=job['source_run'])


def measure(plan, job, probe):
    trace = Path(plan['install_root']) / 'sinks' / job['id'] / 'trace.ndjson'
    root = Path(plan['deposit'])
    dest = root / ('runs' if job['kind'] == 'free' else 'refeeds') / job['id']
    dest.mkdir(parents=True, exist_ok=False)
    if job['kind'] == 'free':
        close = probe.base.gate_turn({'gate_socket': '/run/weaver-bravo/gate.sock'}, probe.ESSAY_PROMPT, timeout=3600)
        check('gate-answer', close.get('kind') == 'answered' and bool(close.get('run')))
        rows = until_closed(trace, 'turn.closed')
        mine = [e for e in rows if e['run'] == close['run']]
        check('single-turn', sum(e['kind'] == 'model.measurement' for e in mine) == 1)
        rec = probe.extract_run(mine)
        enough(rec)
        check('seed-held', rec['declared_seed'] == job['seed'])
        check('weights-held', rec['weights_hash'] == plan['tuple']['weights_sha256'])
        rec.update(name=job['id'], arm='TB0', seed=job['seed'], run=close['run'], trace=str(trace), verdict='RAN')
        target = dest / 'run.json'
    else:
        rows = until_closed(trace, 'replay.closed')
        closes = [e for e in rows if e['kind'] == 'replay.closed']
        check('single-replay', len(closes) == 1)
        close = closes[0]
        outcome = close['payload']['outcome']
        check('replay-completed', outcome['kind'] in ['certified', 'diverged'])
        mine = probe.measured_events(rows, close['run'])
        check('replay-measurement', mine is not None)
        refed = probe.extract_run(mine)
        enough(refed)
        src = source_record(plan, job, probe)
        # A divergence is read as a device or kernel effect only when both sides
        # ran the tuple's weights, as the free-run path already requires.
        check('source-weights-held', src.get('weights_hash') == plan['tuple']['weights_sha256'])
        check('replay-weights-held', refed.get('weights_hash') == plan['tuple']['weights_sha256'])
        reading = probe.reading_two(src, refed)
        # Historical #516 stack coordinate: input-plus-output, not resident.
        div = outcome.get('divergence') or {}
        ordinal = int(div['position']) - refed['input_tokens'] if div.get('kind') == 'token_path' else None
        free_readings = []
        for candidate in plan['arms'][0]['jobs']:
            if candidate['kind'] == 'free' and candidate['seed'] == src.get('declared_seed'):
                path = root / 'runs' / candidate['id'] / 'run.json'
                if path.is_file():
                    free_readings.append(dict(target=candidate['id'], reading=probe.reading_one(src, json.loads(path.read_text()))))
        rec = dict(free_readings=free_readings, name=job['id'], trace=str(trace), run=close['run'], verdict='RAN',
                   replay_outcome=outcome['kind'], replay_divergence=div,
                   replay_divergence_ordinal=ordinal, reading_two=reading,
                   exact=exact(src, refed), refed=refed)
        target = dest / 'refeed.json'
    atomic(target, rec)
    with (root / 'probe.jsonl').open('a') as out:
        out.write(json.dumps(dict(job=job, result=str(target), sha256=sha(target))) + '\n')
    return target


def assess(plan, arm, probe):
    root = Path(plan['deposit'])
    results = []
    for job in arm['jobs']:
        filename = root / ('runs' if job['kind'] == 'free' else 'refeeds') / job['id'] / ('run.json' if job['kind'] == 'free' else 'refeed.json')
        results.append((job, json.loads(filename.read_text())))
    report = dict(arm=arm['name'], jobs=len(results))
    if arm['name'] == 'TB0':
        free = [(j, r) for j, r in results if j['kind'] == 'free']
        pairs = []
        for seed in plan['tuple']['seeds']:
            records = [r for j, r in free if j['seed'] == seed]
            check('pair-count', len(records) == 2)
            a, b = records
            pairs.append(dict(seed=seed, equal=exact(a, b) and a['emission'] == b['emission'],
                              reading=probe.reading_one(a, b)))
        report['pairs'] = pairs
        report['own_refeeds'] = [dict(id=j['id'], exact=r['exact'], certified=r['replay_outcome'] == 'certified')
                                 for j, r in results if j['kind'] == 'refeed']
        report['changed_seeds'] = [dict(a=a['seed'], b=b['seed'], first_difference=probe.first_divergence(a['output_tokens'], b['output_tokens']))
                                   for i, (_, a) in enumerate(free[:8]) for _, b in free[:8][i+1:]]
        report['control_passed'] = all(p['equal'] for p in pairs) and all(r['exact'] and r['certified'] for r in report['own_refeeds'])
        report['changed_seed_prediction'] = all(p['first_difference'] is not None and p['first_difference'] < 24 for p in report['changed_seeds'])
    else:
        report['readings'] = [dict(id=j['id'], **{k: r[k] for k in ['exact', 'replay_outcome', 'reading_two', 'replay_divergence_ordinal', 'free_readings']}) for j, r in results]
        report['executable_identity'] = arm.get('executable_identity', False)
    target = root / f'{arm["name"]}-result.json'
    atomic(target, report)
    if arm['name'] == 'TB0':
        check('control-falsifier', report['control_passed'])
        check('changed-seed-prediction', report['changed_seed_prediction'])
    return target


def drive(order, arm_name):
    state = order.read()
    plan = order.approved(state)
    arm = next(a for a in plan['arms'] if a['name'] == arm_name)
    probe = readers(plan)
    root = Path(plan['deposit'])
    root.mkdir(exist_ok=True)
    start = root / f'{arm_name}-start.json'
    check('fresh-arm', not start.exists())
    atomic(start, dict(arm=arm_name, pid=os.getpid(), plan=state['review']['artifacts'][state['plan']]))
    order.coding(f'start:{arm_name}', start)
    for job in arm['jobs']:
        notice('load:' + job['id'])
        order.wait('measure:' + job['id'])
        result = measure(plan, job, probe)
        order.coding('measure:' + job['id'], result)
        notice('unload:' + job['id'])
        index = arm['jobs'].index(job)
        order.wait('settle:' + job['id'])
        # Stop immediately after the operator has unloaded a falsified control.
        rec = json.loads(result.read_text())
        if arm_name == 'TB0' and job['kind'] == 'refeed':
            check('own-refeed-falsifier', rec['exact'] and rec['replay_outcome'] == 'certified')
        if arm_name == 'TB0' and job['kind'] == 'free':
            earlier = [j for j in arm['jobs'][:index] if j['kind'] == 'free' and j['seed'] == job['seed']]
            if earlier:
                old = json.loads((root / 'runs' / earlier[0]['id'] / 'run.json').read_text())
                check('pair-falsifier', exact(old, rec) and old['emission'] == rec['emission'])
        order.coding('settle:' + job['id'], result)
    result = assess(plan, arm, probe)
    order.coding('finish:' + arm_name, result)
    print(f'DONE: {arm_name}; results {result}', flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--state', required=True)
    parser.add_argument('arm', choices=['TB0', 'TB-d', 'TB-k', 'report'])
    parser.add_argument('path', nargs='?', help='report only: the report file to record')
    args = parser.parse_args()
    order = Order(args.state)
    try:
        check('driver-not-root', os.geteuid() != 0)
        if args.arm == 'report':
            # Due only once every arm's finish: evidence is recorded and still
            # hash-intact; the review that follows is the review seat's edit.
            check('report-path', bool(args.path))
            order.coding('report', Path(args.path).resolve())
            print(f'DONE: report {args.path}; NEXT: review seat - review', flush=True)
        else:
            drive(order, args.arm)
    except (Exception, KeyboardInterrupt) as error:
        try:
            order.fail(error)
        except Exception as failure:
            print(f'Cannot persist refusal: {failure}', file=sys.stderr)
        print(f'REFUSED: {error}; NEXT: review seat - inspect state and arrange cleanup if loaded', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
