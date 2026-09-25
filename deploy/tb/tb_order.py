#!/usr/bin/env python3
"""TB sequencing, adapted from the W4a flock/atomic-state/driver-lease design.

This is experiment tooling for #679, not a product or conformance assertion.
Approval is supplied by the designated review seat, never by this program.
"""
import argparse
import contextlib
import fcntl
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time


class Refused(RuntimeError):
    pass


class Busy(Refused):
    pass


def check(name, condition):
    if not condition:
        raise Refused(name)


def sha(path):
    h = hashlib.sha256()
    with open(path, 'rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def atomic(path, value):
    path = Path(path)
    fd, name = tempfile.mkstemp(prefix='.tb-', dir=path.parent)
    try:
        with os.fdopen(fd, 'w') as stream:
            json.dump(value, stream, indent=2)
            stream.write('\n')
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(name, path)
    finally:
        Path(name).unlink(missing_ok=True)


def ticks(pid):
    try:
        return Path(f'/proc/{pid}/stat').read_text().rsplit(')', 1)[1].split()[19]
    except (FileNotFoundError, ProcessLookupError):
        return None


def live(lease):
    return bool(lease and ticks(lease['pid']) == lease['ticks'])


def leases(step):
    """Steps a fresh coding-seat process takes: an arm's start, and the report
    after the last arm's driver has exited. Every other coding-seat step must
    come from the process holding the lease."""
    return step.startswith('start:') or step == 'report'


def schedule(plan):
    steps = [('provision', 'operator')]
    for arm in plan['arms']:
        steps.append((f'start:{arm["name"]}', 'coding seat'))
        for job in arm['jobs']:
            steps.extend((f'{verb}:{job["id"]}', seat) for verb, seat in
                         [('load', 'operator'), ('measure', 'coding seat'), ('unload', 'operator'), ('settle', 'coding seat')])
        steps.append((f'finish:{arm["name"]}', 'coding seat'))
    return steps + [('report', 'coding seat'), ('review', 'review seat')]


def validate_plan(plan):
    check('schema', plan.get('version') == 1)
    check('agent', plan.get('agent') == 'bravo')
    check('isolated-root', plan.get('install_root') == '/var/lib/weaver-tb')
    check('tuple', plan.get('tuple') == {
        'artifact': '/opt/weaver/models/Qwen3-8B-Q8_0.gguf',
        'weights_sha256': '0cfbf745760f07a76ddeb358dd025a27f2e11d1ca9c9a4169a373d52990fe86e',
        'devices': [0], 'context_capacity': 12288, 'max_tokens': 8192,
        'field_depth': 200, 'surprisal': True, 'residual': False,
        'identity': 'You are Karl, a careful writer. Answer plainly and at length when asked, and do not stop early.',
        'seeds': [451234785645, 1156316220, 7, 1000003, 123456789, 987654321, 2718281828, 3141592653],
        'runs_per_seed': 2, 'driver': '615.71.09'})
    check('rulings', all(plan.get('rulings', {}).get(k) for k in
                         ['hold_lifted', 'cuda_provenance', 'control_count']))
    arms = plan['arms']
    check('arm-order', [a['name'] for a in arms] == ['TB0', 'TB-d', 'TB-k'])
    jobs = [j for a in arms for j in a['jobs']]
    import re
    check('job-identities', len({j['id'] for j in jobs}) == len(jobs) and
          all(re.fullmatch(r'[A-Za-z0-9_-]+', j['id']) for j in jobs))
    check('job-types', all(j['kind'] in ['free', 'refeed'] and j['stack'] in ['B1', 'B2'] for j in jobs))
    control = arms[0]['jobs']
    free = [j for j in control if j['kind'] == 'free']
    check('control-schedule', sorted(j['seed'] for j in free) == sorted(plan['tuple']['seeds'] * 2)
          and all(j['stack'] == 'B1' for j in control))
    own = [j for j in control if j['kind'] == 'refeed']
    check('own-refeeds', sorted(j['source_job'] for j in own) == sorted(j['id'] for j in free))
    prior = set()
    for j in jobs:
        check('source-order', j['kind'] == 'free' or
              (j.get('source_job') in prior) or (bool(j.get('source_trace')) and bool(j.get('source_run'))))
        prior.add(j['id'])
    check('device-sources', bool(arms[1]['jobs']) and
          {j.get('source_cell') for j in arms[1]['jobs']} == {'ampere', 'ada'} and
          all(j['kind'] == 'refeed' and j['stack'] == 'B1' for j in arms[1]['jobs']))
    # A TB-d job replays another device's reviewed trace. The loader and the
    # driver both prefer source_job, so one here would replay a local B1 run
    # under an ampere or ada label.
    check('device-traces', all(not j.get('source_job') and j.get('source_run') and
                               j.get('source_trace') in plan.get('files', {}) for j in arms[1]['jobs']))
    check('kernel-schedule', (arms[2].get('executable_identity') is True and not arms[2]['jobs']) or
          (sorted(j.get('source_job') for j in arms[2]['jobs']) == sorted(j['id'] for j in free) and
           all(j['kind'] == 'refeed' and j['stack'] == 'B2' for j in arms[2]['jobs'])))


class Order:
    def __init__(self, state):
        self.path = Path(state).resolve()
        self.directory = self.path.parent

    def read(self):
        return json.loads(self.path.read_text())

    @contextlib.contextmanager
    def locked(self):
        with self.path.with_suffix('.lock').open('a') as lock:
            try:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except BlockingIOError as error:
                raise Busy('another step is running') from error
            yield

    def approved(self, s):
        check('hold', s.get('hold') is False)
        review = s.get('review', {})
        check('review', review.get('status') == 'PASS' and review.get('reference') and
              review.get('seat') == 'thinkpad-CC-WeaverTools-ReviewSeat')
        files = review.get('artifacts', {})
        required = {str(Path(__file__).resolve().parent / name) for name in
                    ['tb_order.py', 'tb_payload.py', 'tb_driver.py', 'tb-operator.sh',
                     'test_tb.py', 'golden.py', 'perturb.py', 'sections.py', 'prepare.py']}
        required.add(s['plan'])
        check('approval-coverage', required <= files.keys())
        check('artifact-hashes', all(Path(p).is_file() and sha(p) == h for p, h in files.items()))
        check('halt', not s.get('halt'))
        # One snapshot: parse the bytes that matched the recorded digest, never a re-read.
        data = Path(s['plan']).read_bytes()
        check('plan-snapshot', hashlib.sha256(data).hexdigest() == files[s['plan']])
        plan = json.loads(data)
        validate_plan(plan)
        check('manifest-coverage', bool(plan.get('files')) and set(plan['files']) <= files.keys())
        check('manifest-hashes', all(files[p] == h for p, h in plan['files'].items()))
        return plan

    def due(self, s, plan):
        steps = schedule(plan)
        check('cursor', type(s.get('cursor')) is int and 0 <= s['cursor'] < len(steps))
        return steps[s['cursor']]

    def previous(self, s, plan):
        for step, _ in schedule(plan)[:s['cursor']]:
            done = s.get('done', {}).get(step, {})
            check('prior-success', done.get('status') == 'SUCCESS')
            check('prior-evidence', Path(done['path']).is_file() and sha(done['path']) == done['sha256'])

    def guard(self, s, plan, requested, seat):
        due, owner = self.due(s, plan)
        check('step-order', requested == due)
        check('seat', owner == seat)
        check('not-repeated', requested not in s.get('done', {}))
        self.previous(s, plan)
        if requested.startswith(('load:', 'measure:', 'unload:', 'settle:')):
            check('driver-live', live(s.get('driver')))
        if seat == 'coding seat' and not leases(requested):
            check('driver-owner', s.get('driver', {}).get('pid') == os.getpid())

    def finish(self, s, step, evidence):
        s.setdefault('done', {})[step] = dict(status='SUCCESS', path=str(evidence), sha256=sha(evidence))
        s['cursor'] += 1
        atomic(self.path, s)

    def fail(self, error):
        with self.locked():
            s = self.read()
            s['halt'] = str(error)
            s.setdefault('refusals', []).append(dict(at=time.time(), reason=str(error)))
            atomic(self.path, s)

    def operator(self, requested='next', runner=None):
        try:
            return self._operator(requested, runner)
        except Busy:
            raise
        except (Refused, OSError, ValueError, KeyError) as error:
            with self.locked():
                s = self.read()
                s.setdefault('refusals', []).append(dict(at=time.time(), reason=str(error), requested=requested))
                atomic(self.path, s)
            raise

    def _operator(self, requested, runner):
        with self.locked():
            s = self.read()
            if requested == 'next' and (s.get('hold') is not False or s.get('review', {}).get('status') != 'PASS'):
                print('WAITING ON: review seat - lift HOLD and record approval hashes')
                return
            plan = self.approved(s)
            if requested == 'next' and s.get('cursor') == len(schedule(plan)):
                print('COMPLETE: every step is recorded, the review seat\'s review included')
                return
            due, seat = self.due(s, plan)
            if requested == 'next' and seat != 'operator':
                print(f'WAITING ON: {seat} - {due}')
                return
            step = due if requested == 'next' else requested
            self.guard(s, plan, step, 'operator')
            fd, name = tempfile.mkstemp(prefix='scheduled-', suffix='.log', dir=self.directory)
            os.close(fd)
            log = Path(name)
            try:
                result = (runner or payload)(s, step, log)
                check('payload-exit', result == 0)
                check('payload-receipt', log.read_text().splitlines()[-1:] == [f'SUCCESS: {step}'])
                self.finish(s, step, log)
                print(f'DONE: {step} (transcript {log})')
                nxt, owner = self.due(s, plan)
                print(f'NEXT: {owner} - {nxt}')
            except BaseException as error:
                s['halt'] = f'{step}: {error}; transcript {log}'
                atomic(self.path, s)
                raise

    def coding(self, step, evidence):
        with self.locked():
            s = self.read()
            plan = self.approved(s)
            self.guard(s, plan, step, 'coding seat')
            if leases(step):
                check('no-live-driver', not live(s.get('driver')))
                s['driver'] = dict(pid=os.getpid(), ticks=ticks(os.getpid()))
            if step == 'report':
                check('report-evidence', Path(evidence).is_file())
            self.finish(s, step, evidence)

    def wait(self, step, timeout=14400, poll=1):
        deadline = time.monotonic() + timeout
        while True:
            try:
                with self.locked():
                    s = self.read()
                    plan = self.approved(s)
                    check('wait-owner', live(s.get('driver')) and s['driver']['pid'] == os.getpid())
                    if self.due(s, plan)[0] == step:
                        self.previous(s, plan)
                        return
                    check('wait-order', self.due(s, plan)[1] == 'operator')
            except Busy:
                pass
            check('wait-deadline', time.monotonic() < deadline)
            time.sleep(min(poll, max(0, deadline - time.monotonic())))


def notice(step):
    message = f'WAITING ON OPERATOR: TB {step}; run handoffs/tb/tb-operator.sh next only after HOLD is lifted and approval recorded.'
    print(message, flush=True)
    try:
        subprocess.run(['gh', 'issue', 'comment', '679', '--repo', 'toddwbucy/WeaverTools', '--body', message],
                       stdin=subprocess.DEVNULL, capture_output=True, timeout=20, check=False)
    except (OSError, subprocess.TimeoutExpired):
        pass


def payload(state, step, log):
    source = Path(__file__).with_name('tb_payload.py')
    data = source.read_bytes()
    check('payload-hash', hashlib.sha256(data).hexdigest() == state['review']['artifacts'][str(source.resolve())])
    fd, name = tempfile.mkstemp(prefix='.payload-', suffix='.py', dir=log.parent)
    try:
        with os.fdopen(fd, 'wb') as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        check('private-payload', Path(name).stat().st_mode & 0o777 == 0o600)
        check('copied-payload-hash', sha(name) == hashlib.sha256(data).hexdigest())
        with log.open('w') as out:
            # The digest recorded at approval: a plan edited after approved()
            # must fail the payload's check, not be re-hashed into passing it.
            return subprocess.run(['sudo', '/usr/bin/python3', '-I', name, state['plan'],
                                   state['review']['artifacts'][state['plan']], step],
                                  stdin=subprocess.DEVNULL,
                                  stdout=out, stderr=subprocess.STDOUT).returncode
    finally:
        Path(name).unlink(missing_ok=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--state', required=True)
    parser.add_argument('step', nargs='?', default='next')
    args = parser.parse_args()
    try:
        check('operator-not-root', os.geteuid() != 0)
        Order(args.state).operator(args.step)
    except (Refused, OSError, ValueError, KeyError) as error:
        print(f'REFUSED: {error}; NEXT: review seat - rule on refusal', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
