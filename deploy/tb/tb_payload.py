#!/usr/bin/env python3
"""Operator-only TB payload; tb_order hands root its verified source by -c, never a path.

Stdlib only: sudo runs this with -I and closed stdin. Never run it directly.
All mutations belong to the isolated TB installation and the bravo account.
"""
import hashlib
import json
import os
from pathlib import Path
import pwd
import grp
import shutil
import stat
import subprocess
import sys
import time

ROOT = Path('/var/lib/weaver-tb')
MODEL = Path('/opt/weaver/models/Qwen3-8B-Q8_0.gguf')


def need(name, condition):
    if not condition:
        raise RuntimeError(name)


def sha(path):
    h = hashlib.sha256()
    with open(path, 'rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def freeze(source, destination):
    """Read an operator-owned file once into a new root-owned private file.

    Every consumer after this receives the destination, never the source path:
    a check on the source followed by a second read of it binds nothing.
    """
    fd = os.open(destination, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    try:
        with os.fdopen(fd, 'wb') as dst, open(source, 'rb') as src:
            shutil.copyfileobj(src, dst, 1024 * 1024)
            dst.flush()
            os.fsync(dst.fileno())
    except BaseException:
        destination.unlink(missing_ok=True)
        raise
    return destination


def snapshot(source, digest, destination):
    """freeze(), then verify the frozen bytes, not the source, against the recorded digest."""
    freeze(source, destination)
    try:
        need('snapshot-hash', sha(destination) == digest)
    except RuntimeError:
        destination.unlink()
        raise
    return destination


def select_run(whole, run_id, destination):
    """The one run a replay names, cut from a snapshot: derive refuses a record holding two."""
    lines = [line for line in whole.read_bytes().splitlines(keepends=True)
             if line.strip() and json.loads(line)['run'] == run_id]
    need('source-run-selected', bool(lines))
    fd = os.open(destination, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    with os.fdopen(fd, 'wb') as dst:
        dst.writelines(lines)
        dst.flush()
        os.fsync(dst.fileno())
    return destination


def run(argv, **kw):
    return subprocess.run(argv, stdin=subprocess.DEVNULL, check=True, text=True, **kw)


def m1_unloaded():
    # No installed admin invocation. Missing unit alone is not enough: also
    # refuse a surviving coordination door or member/worker process.
    answer = subprocess.run(['/usr/bin/systemctl', 'show', 'weaver-worker@m1.service',
                             '--property=ActiveState', '--property=LoadState'],
                            stdin=subprocess.DEVNULL, capture_output=True, text=True)
    values = dict(line.split('=', 1) for line in answer.stdout.splitlines() if '=' in line)
    need('m1-state-readable', answer.returncode == 0 and values.get('LoadState') in ['loaded', 'not-found'])
    need('m1-inactive', values.get('ActiveState') == 'inactive')
    need('m1-no-door', not Path('/run/weaver-m1/coordination.sock').exists())
    try:
        uid = pwd.getpwnam('weaver-m1').pw_uid
    except KeyError:
        uid = None
    for entry in Path('/proc').iterdir():
        if not entry.name.isdigit():
            continue
        try:
            need('m1-no-process', uid is None or entry.stat().st_uid != uid)
        except FileNotFoundError:
            pass


def config(stack):
    return ROOT / 'config' / stack


def environment(stack):
    return dict(PATH='/usr/bin:/bin', WEAVER_ADMIN_CONFIG=str(config(stack)),
                LD_LIBRARY_PATH=f'{ROOT}/stacks/{stack}/engine-lib:{ROOT}/stacks/{stack}/cuda-lib')


def admin(stack, verb):
    return [str(ROOT / 'stacks' / stack / 'bin/weaver-admin'), verb, 'bravo']


def answer(stack, verb, expected):
    result = run(admin(stack, verb), env=environment(stack), capture_output=True, timeout=600)
    print(result.stdout, end='')
    print(result.stderr, end='', file=sys.stderr)
    rows = [json.loads(line) for line in result.stdout.splitlines() if line.startswith('{')]
    need('admin-answer', bool(rows) and rows[-1].get('kind') == 'state' and rows[-1].get('state') == expected)
    return rows[-1]


def save(path, text, mode=0o644):
    need('no-symlink-destination', not path.is_symlink())
    path.write_text(text)
    path.chmod(mode)


def locked(path):
    """Only this payload's user can change the entry: owned by it, no group or
    world write, not a link. A hash binds bytes only where nobody else can
    replace them, and for a directory that means its entries too."""
    entry = os.lstat(path)
    return (not stat.S_ISLNK(entry.st_mode) and entry.st_uid == os.geteuid() and
            not entry.st_mode & 0o022)


def lock(path, mode):
    """Set the served mode on path, a directory or a file, and every entry
    beneath a directory, the root included: rglob yields descendants only."""
    path.chmod(mode)
    if path.is_dir():
        for p in path.rglob('*'):
            if not p.is_symlink():
                p.chmod(0o755 if p.is_dir() or p.parent.name == 'bin' else 0o644)


def served_directories():
    """Every directory root serves the agent from, outside the stack trees."""
    return [ROOT, ROOT / 'stacks', ROOT / 'config', ROOT / 'agents',
            *(config(s) for s in ['B1', 'B2']), *(ROOT / 'agents' / s for s in ['B1', 'B2'])]


def verify_stack(plan, stack, root):
    """The installed copy holds exactly the reviewed files, by bytes, and no
    link, in a tree nobody but this payload's user can change."""
    source = Path(plan['stacks'][stack])
    entries = list(root.rglob('*'))
    need('installed-no-symlinks', not root.is_symlink() and not any(p.is_symlink() for p in entries))
    need('installed-stack-custody', locked(root) and all(locked(p) for p in entries))
    expected = {Path(p).relative_to(source): d for p, d in plan['files'].items() if Path(p).is_relative_to(source)}
    need('installed-stack-coverage', {p.relative_to(root) for p in entries if p.is_file()} == expected.keys())
    need('installed-stack-hash', all(sha(root / r) == d for r, d in expected.items()))


def provision(plan, plan_sha256):
    need('fresh-install-root', not ROOT.exists())
    try:
        pwd.getpwnam('weaver-bravo')
    except KeyError:
        pass
    else:
        raise RuntimeError('bravo account already exists: review its custody before provisioning')
    need('model-source', sha(plan['model_source']) == plan['tuple']['weights_sha256'])
    if MODEL.exists() or MODEL.is_symlink():
        # Accepted only as a file already in custody; a link or a shared name
        # to operator-writable bytes would pass the hash and change after it.
        need('existing-model-custody', model_custody())
        need('existing-model', sha(MODEL) == plan['tuple']['weights_sha256'])
    # Verify every input before the first write; an interrupted provision is
    # retained as a refusal for the seat, never silently resumed or removed.
    for stack in ['B1', 'B2']:
        source = Path(plan['stacks'][stack])
        # B1 is copied by bytes. rglob does not descend a linked directory and
        # a linked file is hashed at its target, so a link anywhere in a stack
        # would carry unreviewed or mutable bytes into the root-owned install.
        need('stack-no-symlinks', not any(p.is_symlink() for p in [source, *source.rglob('*')]))
        need('stack-libraries', all((source / x).is_dir() for x in ['bin', 'engine-lib', 'cuda-lib']))
        files = [p for p in source.rglob('*') if p.is_file()]
        need('stack-file-coverage', bool(files) and all(str(p) in plan['files'] for p in files))
    # mkdir's mode is masked by the umask, so every served directory is set
    # explicitly as it is made.
    ROOT.mkdir(mode=0o755)
    lock(ROOT, 0o755)
    save(ROOT / 'plan-sha256', plan_sha256 + '\n')
    if not MODEL.exists():
        made = not MODEL.parent.exists()
        MODEL.parent.mkdir(parents=True, exist_ok=True)
        if made:
            lock(MODEL.parent, 0o755)
        # The model-source check above refuses early; what root serves is the
        # copy, verified here before provisioning can report success, and held
        # in custody like an existing one: a writable parent lets another
        # process replace the new file after its hash.
        snapshot(plan['model_source'], plan['tuple']['weights_sha256'], MODEL)
        MODEL.chmod(0o644)
        need('new-model-custody', model_custody())
    run(['/usr/bin/groupadd', '--system', 'weaver-bravo'])
    run(['/usr/bin/useradd', '--system', '--gid', 'weaver-bravo', '--home-dir', str(ROOT / 'home'),
         '--create-home', '--shell', '/usr/bin/nologin', 'weaver-bravo'])
    run(['/usr/bin/usermod', '-aG', 'weaver-bravo', plan['operator']])
    for directory in [ROOT / 'stacks', ROOT / 'config', ROOT / 'agents']:
        directory.mkdir()
        lock(directory, 0o755)
    for stack in ['B1', 'B2']:
        dst = ROOT / 'stacks' / stack
        shutil.copytree(plan['stacks'][stack], dst, symlinks=True)
        # copytree copies the source root's mode onto dst; lock the whole tree,
        # dst included, before the copy is verified as what root serves.
        lock(dst, 0o755)
        verify_stack(plan, stack, dst)
        cfg = config(stack)
        cfg.mkdir()
        lock(cfg, 0o755)
        (ROOT / 'agents' / stack).mkdir()
        lock(ROOT / 'agents' / stack, 0o755)
        values = {'allow-list': 'bravo', 'coordination-root': '/run',
                  'log-path': str(ROOT / f'admin-{stack}.log'),
                  'agent-config-directory': str(ROOT / 'agents' / stack),
                  'run-tool': '/usr/bin/systemd-run', 'control-tool': '/usr/bin/systemctl',
                  'worker-binary': str(dst / 'bin/pyworker'),
                  'spu-binary': str(dst / 'bin/weaver-spu'), 'gate-binary': str(dst / 'bin/weaver-gate'),
                  'unit-properties': 'Environment=LD_LIBRARY_PATH=' + environment(stack)['LD_LIBRARY_PATH']}
        for name, value in values.items():
            save(cfg / name, value + '\n')
    sink = ROOT / 'sinks'
    sink.mkdir()
    os.chown(sink, 0, grp.getgrnam(plan['operator']).gr_gid)
    sink.chmod(0o2750)
    print('Provisioned bravo only. Start a fresh operator shell with the weaver-bravo group before the driver.')


def model_custody():
    """The served model is a regular file only this payload's user can change:
    not a link, one name, not group- or world-writable, in a directory held the
    same way. A hash only binds bytes nobody else can rewrite before load."""
    entry, parent = os.lstat(MODEL), os.stat(MODEL.parent)
    return (stat.S_ISREG(entry.st_mode) and entry.st_nlink == 1 and entry.st_uid == os.geteuid() and
            not entry.st_mode & 0o022 and parent.st_uid == os.geteuid() and not parent.st_mode & 0o022)


def installed(plan, plan_sha256):
    need('installation-plan', (ROOT / 'plan-sha256').read_text().strip() == plan_sha256)
    need('installed-model-custody', model_custody())
    need('served-directory-custody', all(locked(p) for p in served_directories()))
    need('installed-model', sha(MODEL) == plan['tuple']['weights_sha256'])
    for stack in ['B1', 'B2']:
        verify_stack(plan, stack, ROOT / 'stacks' / stack)


def declared(text, key):
    """Every value a derived declaration gives key. derive renders each of the
    keys read here as JSON (weaver-analysis declare.rs at e69916a: artifact
    183-186, identity 197, the tunable values 199-201), so each is parsed as
    JSON; a value that is not JSON names nothing."""
    found = []
    for line in text.splitlines():
        name, _, value = line.strip().partition(':')
        if name == key:
            try:
                found.append(json.loads(value))
            except ValueError:
                found.append(None)
    return found


def declared_artifacts(text):
    return declared(text, 'artifact')


def holds_tuple(plan, job, text):
    """derive carries the source run's own seed, identity, context capacity and
    token cap into the replay, so a source recorded under another tuple would
    replay another experiment under this one's name. A local source holds its
    free job's seed; an external one, one of the tuple's seeds."""
    t = plan['tuple']
    own = [j['seed'] for arm in plan['arms'] for j in arm['jobs'] if j['id'] == job.get('source_job')]
    identity = [{'role': 'system', 'content': [{'type': 'text', 'text': t['identity']}]}]
    seeds = declared(text, 'seed')
    return (len(seeds) == 1 and seeds[0] in (own or t['seeds']) and
            declared(text, 'context-capacity') == [t['context_capacity']] and
            declared(text, 'max-tokens-per-turn') == [t['max_tokens']] and
            declared(text, 'identity') == [identity])


def declaration(plan, job, sink):
    t = plan['tuple']
    return (f'session: tb-{job["id"]}\nspu-instruction:\n  decoder:\n    model-binding:\n'
            f'      artifact: {MODEL}\n      devices: [0]\n    residual-readout-election: false\n'
            '    surprisal-election: true\n    field-election:\n      depth: 200\n    identity:\n'
            '      - role: system\n        content:\n          - type: text\n'
            f'            text: {json.dumps(t["identity"])}\n    tunable-values:\n'
            f'      seed: {job["seed"]}\n      context-capacity: 12288\n      max-tokens-per-turn: 8192\n'
            f'loop-file: {ROOT}/stacks/{job["stack"]}/bin/basic_loop.py\n'
            'tool-set: []\npermission-mode: ask\ngate-instruction:\n  access-rule:\n'
            f'    allowed-uids: [{plan["operator_uid"]}]\n    allowed-gids: []\n    denied-uids: []\n'
            f'trace-sink:\n  kind: file\n  path: {sink}\n  create: true\n')


def load(plan, job):
    m1_unloaded()
    stack = job['stack']
    answer(stack, 'show', 'unloaded')
    # Every load checks the driver version as well as declared device ordinal.
    gpu = run(['/usr/bin/nvidia-smi', '--query-gpu=index,name,driver_version', '--format=csv,noheader'],
              capture_output=True).stdout.strip().splitlines()
    need('gpu-tuple', len(gpu) == 1 and gpu[0].split(', ')[0] == '0' and
         'RTX PRO 5000 Blackwell' in gpu[0] and gpu[0].endswith(plan['tuple']['driver']))
    # Fail if ELF resolution falls back to the host toolkit for B1 or B2.
    lib = ROOT / 'stacks' / stack / 'engine-lib/libggml-cuda.so'
    resolved = run(['/usr/bin/ldd', str(lib)], env=environment(stack), capture_output=True).stdout
    need('resolved-libraries', 'not found' not in resolved)
    for name in ['libcudart.so', 'libcublas.so', 'libcublasLt.so']:
        lines = [line for line in resolved.splitlines() if name in line]
        need('cuda-local', len(lines) == 1 and f'{ROOT}/stacks/{stack}/cuda-lib/' in lines[0])
    print(resolved)
    directory = ROOT / 'sinks' / job['id']
    directory.mkdir()  # Never truncate/reuse a run, even after refusal.
    os.chown(directory, 0, grp.getgrnam(plan['operator']).gr_gid)
    directory.chmod(0o2750)
    sink = directory / 'trace.ndjson'
    target = ROOT / 'agents' / stack / 'bravo.yaml'
    if job['kind'] == 'free':
        save(target, declaration(plan, job, sink))
        answer(stack, 'load', 'idle')
        return
    # derive and preload each read their input again, so both receive one
    # root-owned private file cut from a single verified read.
    frozen = ROOT / 'snapshots' / job['id']
    frozen.mkdir(mode=0o700, parents=True)
    if job.get('source_job'):
        # Our own sink, written by the agent and root-owned: frozen, with no
        # reviewed digest to hold it to, and holding the one source run.
        source = freeze(ROOT / 'sinks' / job['source_job'] / 'trace.ndjson', frozen / 'source.ndjson')
    else:
        whole = snapshot(job['source_trace'], plan['files'][str(job['source_trace'])], frozen / 'whole.ndjson')
        source = select_run(whole, job['source_run'], frozen / 'source.ndjson')
    analysis = str(ROOT / 'stacks' / stack / 'bin/weaver-analysis')
    run([analysis, 'derive', str(source), '--devices', '0', '--sink', str(sink),
         '--field-depth', '200', '--surprisal', '--out', str(target)], env=environment(stack))
    # Derive preserves the recorded artifact; never rewrite it to evade identity.
    need('derived-artifact', declared_artifacts(target.read_text()) == [str(MODEL)])
    need('derived-tuple', holds_tuple(plan, job, target.read_text()))
    with (directory / 'load.log').open('w') as log:
        loader = subprocess.Popen(admin(stack, 'load'), stdin=subprocess.DEVNULL,
                                  stdout=log, stderr=subprocess.STDOUT, env=environment(stack))
        try:
            door = directory / 'state/preload.sock'
            deadline = time.monotonic() + 120
            while not door.exists() and loader.poll() is None and time.monotonic() < deadline:
                time.sleep(0.5)
            need('preload-door', door.exists() and stat.S_ISSOCK(door.stat().st_mode))
            run([analysis, 'preload', str(source), str(door)], env=environment(stack), timeout=600)
            need('diagnostic-load', loader.wait(timeout=600) == 0)
        finally:
            if loader.poll() is None:
                loader.kill()
                loader.wait()
    print((directory / 'load.log').read_text())


def main():
    need('root-payload', os.geteuid() == 0)
    plan_path, expected, step = sys.argv[1:]
    # One snapshot, checked against the digest recorded at approval, then parsed.
    data = Path(plan_path).read_bytes()
    need('plan-hash', hashlib.sha256(data).hexdigest() == expected)
    plan = json.loads(data)
    need('fixed-root-agent', plan['install_root'] == str(ROOT) and plan['agent'] == 'bravo')
    need('operator', pwd.getpwnam(plan['operator']).pw_uid == plan['operator_uid'] == int(os.environ['SUDO_UID']))
    for path, digest in plan['files'].items():
        need('source-file-hash', sha(path) == digest)
    if step == 'provision':
        provision(plan, expected)
    else:
        installed(plan, expected)
        verb, identity = step.split(':', 1)
        jobs = [j for arm in plan['arms'] for j in arm['jobs'] if j['id'] == identity]
        need('job-found', len(jobs) == 1)
        job = jobs[0]
        if verb == 'load':
            load(plan, job)
        elif verb == 'unload':
            answer(job['stack'], 'unload', 'unloaded')
            answer(job['stack'], 'show', 'unloaded')
        else:
            raise RuntimeError('unknown step')
    print(f'SUCCESS: {step}', flush=True)


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        print(f'REFUSED: {error}; NEXT: review seat - inspect partial state, do not retry blindly', file=sys.stderr)
        sys.exit(1)
