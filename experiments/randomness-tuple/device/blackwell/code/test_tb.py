#!/usr/bin/env python3
# conforms: blackwell-probe-tuple-held-field-for-field
# conforms: blackwell-probe-elected-series-from-the-tuple
# conforms: blackwell-probe-schedule-validated-whole
# conforms: blackwell-probe-falsifier-halts-after-unload
# conforms: blackwell-probe-one-command-one-seat-per-step
# conforms: blackwell-probe-approval-gates-every-step
# conforms: blackwell-probe-wait-verifies-when-the-state-moves
# conforms: blackwell-probe-halt-is-evidence
# conforms: blackwell-probe-root-receives-bytes-never-a-path
# conforms: blackwell-probe-operator-input-read-once
# conforms: blackwell-probe-served-tree-locked-and-verified
# conforms: blackwell-probe-model-in-custody-on-both-paths
# conforms: blackwell-probe-installation-refuses-to-adopt
# conforms: blackwell-probe-load-stands-on-the-interlock
# conforms: blackwell-probe-inventory-covers-every-served-file
# conforms: blackwell-probe-comparison-takes-b1-then-b2
# conforms: blackwell-probe-identity-bound-to-approved-stacks
# conforms: blackwell-probe-refeed-completes-against-a-verified-source
# conforms: blackwell-probe-exactness-is-bitwise-over-elected-readings
"""Host-only tests. All host mutations/admin/GPU calls use stub boundaries."""
import contextlib
import copy
import hashlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import golden
import prepare
import sections
import tb_driver as driver
import tb_order as order
import tb_payload as payload


def event(line, **fields):
    """A real event from golden.py with named fields replaced."""
    e = json.loads(line)
    e.update(fields)
    return e


def ndjson(*events):
    """Events as the recorder writes them: compact JSON, one per line."""
    return ''.join(json.dumps(e, separators=(',', ':')) + '\n' for e in events)


def ldd_output(root, cuda_dir):
    """golden's real ldd capture, with the three CUDA libraries resolved in
    cuda_dir, as they would be for an installed stack."""
    lines = []
    for line in golden.LDD_B1_LIBGGML_CUDA.replace('{DEPOSIT}', str(root)).splitlines():
        name = line.strip().split(' ')[0]
        if name.startswith(('libcudart.so', 'libcublas.so', 'libcublasLt.so')):
            line = line.split(' => ')[0] + f' => {cuda_dir}/{name} ' + line.rsplit(' ', 1)[1]
        lines.append(line)
    return '\n'.join(lines) + '\n'


def derived(artifact, sink, held=None, **values):
    """golden's real derive output naming artifact and sink. With held, a plan
    tuple, it carries that tuple's seed, identity, context capacity and token
    cap in the form derive renders them (compact JSON); values override one."""
    text = (golden.DERIVE_DECLARATION.replace(json.dumps(golden.DERIVE_ARTIFACT), json.dumps(str(artifact)))
            .replace('{SINK}', str(sink)))
    if held is None:
        return text
    fields = {'seed': held['seeds'][0], 'context-capacity': held['context_capacity'],
              'max-tokens-per-turn': held['max_tokens'],
              'identity': [{'role': 'system', 'content': [{'type': 'text', 'text': held['identity']}]}]}
    fields.update(values)
    lines = []
    for line in text.splitlines():
        key = line.strip().partition(':')[0]
        if key in fields:
            line = line[:line.index(key)] + key + ': ' + json.dumps(fields[key], separators=(',', ':'))
        lines.append(line)
    return '\n'.join(lines) + '\n'


@contextlib.contextmanager
def swapped_after_hashing(files):
    """Rewrite each file the moment its current bytes have been hashed, as an
    operator could between a check and a second read. Any code that reads a
    file again after checking it then consumes the rewritten bytes."""
    real=hashlib.sha256
    pending={p.read_bytes():(p,new) for p,new in files.items()}
    class Hooked:
        def __init__(s,data=b''):s.h=real(data);s.seen=bytearray(data)
        def update(s,b):s.h.update(b);s.seen+=b
        def hexdigest(s):
            digest=s.h.hexdigest();hit=pending.pop(bytes(s.seen),None)
            if hit:hit[0].write_bytes(hit[1])
            return digest
    with patch('hashlib.sha256',side_effect=Hooked):yield pending


class Fixture(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.plan = prepare.template(self.root, 'todd', 1000)
        self.plan['rulings'] = {k: '#679/ruling' for k in self.plan['rulings']}
        self.source = self.root / 'source.ndjson'
        self.source.write_text(ndjson(event(golden.MODEL_MEASUREMENT, run='r')))
        # One trace per source device, as the historical cells recorded them.
        self.ada_source = self.root / 'ada-source.ndjson'
        self.ada_source.write_text(ndjson(event(golden.MODEL_MEASUREMENT, run='r-ada')))
        self.plan['files'] = {str(p): order.sha(p) for p in [self.source, self.ada_source]}
        self.plan['arms'][1]['jobs'] = [dict(id=cell, source_cell=cell, kind='refeed', stack='B1',
                                            source_trace=str(trace), source_run=run)
                                        for cell, trace, run in [('ampere', self.source, 'r'), ('ada', self.ada_source, 'r-ada')]]
        self.planpath = self.root / 'plan.json'
        order.atomic(self.planpath, self.plan)
        scripts = Path(__file__).resolve().parent
        self.state = dict(hold=False, cursor=0, done={}, driver=None, halt=None,
                          plan=str(self.planpath), review=dict(status='PASS', reference='#679/pass',
                          seat='thinkpad-CC-WeaverTools-ReviewSeat', artifacts={str(p): order.sha(p) for p in scripts.iterdir() if p.suffix in ['.py','.sh']}))
        self.state['review']['artifacts'].update({str(self.planpath): order.sha(self.planpath), **self.plan['files']})
        self.statepath = self.root / 'state.json'
        self.o = order.Order(self.statepath)
        self.save()

    def save(self):
        order.atomic(self.statepath, self.state)

    def due(self, wanted):
        self.state['cursor'] = [s for s,_ in order.schedule(self.plan)].index(wanted)
        for step,_ in order.schedule(self.plan)[:self.state['cursor']]:
            p = self.root / (step.replace(':','-') + '.log')
            p.write_text('success\n')
            self.state['done'][step] = dict(status='SUCCESS', path=str(p), sha256=order.sha(p))
        self.state['driver'] = dict(pid=os.getpid(), ticks=order.ticks(os.getpid()))
        self.save()

    def test_held_next_does_not_invoke_runner(self):
        for change in [('hold',True), ('review',dict(status='PENDING'))]:
            s=copy.deepcopy(self.state);s[change[0]]=change[1];order.atomic(self.statepath,s)
            with patch('tb_order.payload') as runner, contextlib.redirect_stdout(io.StringIO()) as out:
                self.o.operator()
            runner.assert_not_called()
            self.assertIn('WAITING ON: review seat',out.getvalue())

    def test_approval_refusals(self):
        changes = [lambda s:s.update(hold=True), lambda s:s['review'].update(status='PENDING'),
                   lambda s:s['review'].update(seat='coding seat'), lambda s:s['review'].update(reference=None),
                   lambda s:s['review']['artifacts'].pop(str(self.planpath)),
                   lambda s:s['review']['artifacts'].update({str(self.source):'wrong'}), lambda s:s.update(halt='refusal')]
        for change in changes:
            s=copy.deepcopy(self.state);change(s)
            with self.subTest(change=change), self.assertRaises(order.Refused):self.o.approved(s)
        self.assertEqual(self.o.approved(self.state), self.plan)

    def test_manifest_cannot_hide_inputs(self):
        for field in ['absent','mismatch']:
            p=copy.deepcopy(self.plan)
            p['files']={**p['files'],str(self.root/'unreviewed'):'x'} if field=='absent' else {**p['files'],str(self.source):'wrong'}
            order.atomic(self.planpath,p)
            self.state['review']['artifacts'][str(self.planpath)]=order.sha(self.planpath)
            with self.assertRaises(order.Refused):self.o.approved(self.state)

    def test_plan_validation(self):
        order.validate_plan(self.plan)
        edits=[('schema',lambda p:p.update(version=2)), ('agent',lambda p:p.update(agent='m1')),
               ('root',lambda p:p.update(install_root='/etc/weaver/admin')),
               ('tuple',lambda p:p['tuple'].update(context_capacity=100)),
               ('ruling',lambda p:p['rulings'].update(control_count=None)),
               ('arm',lambda p:p['arms'][0].update(name='WRONG')),
               ('identities',lambda p:p['arms'][1]['jobs'][0].update(id=p['arms'][1]['jobs'][1]['id'])),
               ('kind',lambda p:p['arms'][0]['jobs'].append(dict(id='extra',kind='shell',stack='B1',source_trace='/trace',source_run='r'))),
               ('control',lambda p:p['arms'][0]['jobs'][0].update(seed=9)),
               ('own',lambda p:p['arms'][0]['jobs'].pop()),
               ('order',lambda p:p['arms'][0]['jobs'].insert(0,p['arms'][0]['jobs'].pop())),
               ('device',lambda p:p['arms'][1].update(jobs=[])),
               ('kernel',lambda p:p['arms'][2].update(jobs=[])),
               # #683 thread 36: identity true is defined to empty the leg, so a
               # full kernel schedule claiming it is a contradiction, refused.
               ('kernel-identity-with-jobs',lambda p:p['arms'][2].update(executable_identity=True))]
        for name, edit in edits:
            p=copy.deepcopy(self.plan);edit(p)
            with self.subTest(name=name),self.assertRaises(order.Refused):order.validate_plan(p)

    def test_device_arm_measures_each_source_once(self):
        # #683 finding 18: the two cells must not replay one selection, or one
        # trace file between them.
        order.validate_plan(self.plan)
        for fault in ['same-selection','shared-trace','repeated-within-cell']:
            with self.subTest(fault=fault):
                bad=copy.deepcopy(self.plan);jobs=bad['arms'][1]['jobs']
                if fault=='same-selection':jobs[1].update(source_trace=jobs[0]['source_trace'],source_run=jobs[0]['source_run'])
                if fault=='shared-trace':jobs[1].update(source_trace=jobs[0]['source_trace'],source_run='r-other')
                if fault=='repeated-within-cell':jobs.append(dict(jobs[0],id='ampere-again'))
                with self.assertRaisesRegex(order.Refused,'device-selections-distinct'):order.validate_plan(bad)

    def test_device_arm_replays_only_reviewed_external_traces(self):
        # #683 finding 6: a TB-d job naming a local source_job, or a trace the
        # manifest does not hash, would replay a B1 run under a device label.
        order.validate_plan(self.plan)
        free=[j['id'] for j in self.plan['arms'][0]['jobs'] if j['kind']=='free']
        for fault in ['source_job','no-trace','unhashed-trace','no-run']:
            with self.subTest(fault=fault):
                bad=copy.deepcopy(self.plan);job=bad['arms'][1]['jobs'][0]
                if fault=='source_job':job['source_job']=free[0]
                if fault=='no-trace':job.pop('source_trace')
                if fault=='unhashed-trace':job['source_trace']=str(self.root/'elsewhere.ndjson')
                if fault=='no-run':job.pop('source_run')
                with self.assertRaises(order.Refused):order.validate_plan(bad)

    def test_an_emptied_kernel_arm_stands_on_its_comparison(self):
        # #683 findings 26 and 29: executable_identity alone emptied TB-k. The
        # plan must name a hashed comparison report; approval requires it to
        # be sections.compare's B1-against-B2 verdict, true, read from two
        # reviewed inventories whose host hashes are exactly the approved
        # hashes of every file under each stack, so a stale verdict does not
        # outlive the stacks it judged.
        report=self.root/'identity.json';manifests={}
        for stack in ['B1','B2']:
            src=Path(self.plan['stacks'][stack]);(src/'bin').mkdir(parents=True,exist_ok=True)
            (src/'bin/worker').write_text(stack);self.plan['files'][str(src/'bin/worker')]=order.sha(src/'bin/worker')
            m=self.root/f'{stack}-manifest.json';m.write_text(json.dumps(dict(stack=stack,hosts={'bin/worker':dict(file_sha256=order.sha(src/'bin/worker'))})))
            manifests[stack]=m;self.plan['files'][str(m)]=order.sha(m)
        def inputs(**over):return {s:dict(dict(manifest=str(m),sha256=order.sha(m)),**over.get(s,{})) for s,m in manifests.items()}
        def emptied(verdict,named=True):
            p=copy.deepcopy(self.plan);p['arms'][2]=dict(name='TB-k',executable_identity=True,jobs=[])
            report.write_text(json.dumps(verdict))
            if named:p['arms'][2]['identity_report']=str(report);p['files'][str(report)]=order.sha(report)
            order.atomic(self.planpath,p);self.state['review']['artifacts'].update({str(self.planpath):order.sha(self.planpath),str(report):order.sha(report),**{str(m):order.sha(m) for m in manifests.values()},**{p:h for p,h in self.plan['files'].items()}})
            return p
        with self.assertRaisesRegex(order.Refused,'kernel-schedule'):order.validate_plan(emptied({},named=False))
        good=dict(stacks=['B1','B2'],executable_identity=True,inputs=inputs())
        for label,guard,verdict in [('false','identity-evidence',dict(good,executable_identity=False)),('unpaired','identity-evidence',dict(executable_identity=True)),
                                    ('same-stack','identity-evidence',dict(good,stacks=['B1','B1'])),('no-inputs','identity-inputs',dict(good,inputs={})),
                                    ('unreviewed-manifest','identity-inputs',dict(good,inputs=inputs(B2=dict(manifest=str(self.root/'elsewhere.json'))))),
                                    ('wrong-digest','identity-inputs',dict(good,inputs=inputs(B2=dict(sha256='0'*64))))]:
            with self.subTest(label=label):
                emptied(verdict)
                with self.assertRaisesRegex(order.Refused,guard):self.o.approved(self.state)
        emptied(good);self.assertEqual(self.o.approved(self.state)['arms'][2]['jobs'],[])
        # The stacks move under a still-hashed report: the verdict no longer binds.
        src=Path(self.plan['stacks']['B2'])/'bin/worker';src.write_text('rebuilt');self.plan['files'][str(src)]=order.sha(src)
        emptied(good)
        with self.assertRaisesRegex(order.Refused,'identity-binds-stacks'):self.o.approved(self.state)

    def test_operator_sequence_success_and_repeat(self):
        def runner(s,step,log):log.write_text('SUCCESS: '+step+'\n');return 0
        with contextlib.redirect_stdout(io.StringIO()):self.o.operator('next',runner)
        s=self.o.read();self.assertEqual(s['cursor'],1)
        with self.assertRaises(order.Refused):self.o.operator('provision',runner)
        with patch('tb_order.payload') as call,contextlib.redirect_stdout(io.StringIO()):self.o.operator()
        call.assert_not_called()

    def test_failed_payload_never_advances(self):
        for rc, line in [(1,'SUCCESS: provision\n'),(0,'no receipt\n')]:
            self.save()
            def runner(s,step,log):log.write_text(line);return rc
            with self.assertRaises(order.Refused):self.o.operator(runner=runner)
            s=self.o.read();self.assertEqual(s['cursor'],0);self.assertTrue(s['halt'])

    def test_history_order_seat_lease(self):
        self.due('load:B1-s451234785645-n1')
        good=copy.deepcopy(self.state)
        cases=[('step',lambda s:None,'unload:B1-s451234785645-n1','operator'),
               ('seat',lambda s:None,'load:B1-s451234785645-n1','coding seat'),
               ('repeat',lambda s:s['done'].update({'load:B1-s451234785645-n1':{}}),'load:B1-s451234785645-n1','operator'),
               ('prior',lambda s:s['done']['provision'].update(status='REFUSED'),'load:B1-s451234785645-n1','operator'),
               ('evidence',lambda s:s['done']['provision'].update(sha256='bad'),'load:B1-s451234785645-n1','operator'),
               ('lease',lambda s:s['driver'].update(ticks='wrong'),'load:B1-s451234785645-n1','operator')]
        for name, edit, step, seat in cases:
            s=copy.deepcopy(good);edit(s)
            with self.subTest(name=name),self.assertRaises(order.Refused):self.o.guard(s,self.plan,step,seat)
        self.o.guard(good,self.plan,'load:B1-s451234785645-n1','operator')
        for cursor in [-1,9999,False]:
            s=copy.deepcopy(good);s['cursor']=cursor
            with self.assertRaises(order.Refused):self.o.due(s,self.plan)

    def test_lease_start_and_coding_owner(self):
        self.due('start:TB0')
        p=self.root/'start.json';p.write_text('{}')
        with self.assertRaises(order.Refused):self.o.coding('start:TB0',p)
        self.state['driver']=None;self.save();self.o.coding('start:TB0',p)
        self.assertEqual(self.o.read()['driver']['pid'],os.getpid())
        self.due('measure:B1-s451234785645-n1')
        with patch('tb_order.live',return_value=True):
            self.state['driver']['pid']=-1
            with self.assertRaises(order.Refused):self.o.guard(self.state,self.plan,'measure:B1-s451234785645-n1','coding seat')

    def test_report_executor(self):
        # #683 finding 12, ruling (a): report runs in a fresh process with its
        # own lease, only after every arm's finish: evidence is recorded and
        # intact, and hands the cursor to the review seat.
        report=self.root/'report.json';report.write_text('{}')
        self.due('finish:TB-k');self.state['driver']=None;self.save()
        with self.assertRaisesRegex(order.Refused,'step-order'):self.o.coding('report',report)
        self.due('report');done=self.state['done'].pop('finish:TB-k');self.state['driver']=None;self.save()
        with self.assertRaisesRegex(order.Refused,'prior-success'):self.o.coding('report',report)
        self.state['done']['finish:TB-k']=done;Path(done['path']).write_text('changed');self.save()
        with self.assertRaisesRegex(order.Refused,'prior-evidence'):self.o.coding('report',report)
        Path(done['path']).write_text('success\n');self.due('report')
        with self.assertRaisesRegex(order.Refused,'no-live-driver'):self.o.coding('report',report)
        self.state['driver']=None;self.save()
        with self.assertRaisesRegex(order.Refused,'report-evidence'):self.o.coding('report',self.root/'absent.json')
        self.o.coding('report',report)
        after=self.o.read();steps=[s for s,_ in order.schedule(self.plan)]
        self.assertEqual(after['cursor'],steps.index('review'))
        self.assertEqual((after['done']['report']['path'],after['driver']['pid']),(str(report),os.getpid()))
        with contextlib.redirect_stdout(io.StringIO()) as out:self.o.operator()
        self.assertEqual(out.getvalue(),'WAITING ON: review seat - review\n')

    def test_next_reports_completion_after_the_review(self):
        steps=order.schedule(self.plan);self.due(steps[-1][0])
        p=self.root/'review.log';p.write_text('PASS\n')
        self.state['done']['review']=dict(status='SUCCESS',path=str(p),sha256=order.sha(p));self.state['cursor']=len(steps);self.save()
        with contextlib.redirect_stdout(io.StringIO()) as out:self.o.operator()
        self.assertTrue(out.getvalue().startswith('COMPLETE:'))
        self.assertNotIn('refusals',self.o.read())
        # #683 finding 15: completion claims every receipt, so a receipt
        # changed after the review refuses it rather than printing COMPLETE.
        p.write_text('changed after the review\n')
        with contextlib.redirect_stdout(io.StringIO()) as out,self.assertRaisesRegex(order.Refused,'prior-evidence'):self.o.operator()
        self.assertNotIn('COMPLETE',out.getvalue())

    def test_driver_report_command(self):
        report=self.root/'report.json';report.write_text('{}');self.due('report');self.state['driver']=None;self.save()
        with patch('sys.argv',['driver','--state',str(self.statepath),'report']),contextlib.redirect_stderr(io.StringIO()) as err:
            self.assertEqual(driver.main(),1)
        self.assertIn('report-path',err.getvalue())
        self.state['halt']=None;self.save()
        with patch('sys.argv',['driver','--state',str(self.statepath),'report',str(report)]),contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(driver.main(),0)
        self.assertIn('report',self.o.read()['done'])

    def test_wait_succeeds_only_with_receipt_and_own_live_lease(self):
        self.due('measure:B1-s451234785645-n1')
        self.o.wait('measure:B1-s451234785645-n1',timeout=0)
        self.state['driver']['ticks']='dead';self.save()
        with self.assertRaises(order.Refused):self.o.wait('measure:B1-s451234785645-n1',timeout=0)
        self.due('load:B1-s451234785645-n1')
        with self.assertRaises(order.Refused):self.o.wait('measure:B1-s451234785645-n1',timeout=0)
        self.due('start:TB0')
        with self.assertRaisesRegex(order.Refused,'wait-order'):self.o.wait('measure:B1-s451234785645-n1',timeout=0)

    def test_wait_verifies_approval_only_when_the_state_moves(self):
        # #683 finding 28: approval hashes the whole reviewed deposit, the model
        # included, under the lock next needs; polling it every second starved
        # the operator. It now runs once per state, and again once the state
        # moves, before the wait returns.
        self.due('load:B1-s451234785645-n1');calls=[];real=order.Order.approved
        def counting(o,s):calls.append(s['cursor']);return real(o,s)
        with patch.object(order.Order,'approved',counting):
            with self.assertRaisesRegex(order.Refused,'wait-deadline'):self.o.wait('measure:B1-s451234785645-n1',timeout=0.3,poll=0.01)
            self.assertEqual(len(calls),1)
            self.due('measure:B1-s451234785645-n1');self.o.wait('measure:B1-s451234785645-n1',timeout=0.3,poll=0.01)
        self.assertEqual(len(calls),2)

    def test_process_wait_stays_alive_until_operator_receipt(self):
        self.due('load:B1-s451234785645-n1')
        script="""import os,sys
from tb_order import *
o=Order(sys.argv[1]);s=o.read();s['driver']={'pid':os.getpid(),'ticks':ticks(os.getpid())};atomic(o.path,s)
print('WAITING',flush=True);o.wait('measure:B1-s451234785645-n1',timeout=10,poll=.02);print('DONE',flush=True)
"""
        child=subprocess.Popen([sys.executable,'-c',script,str(self.statepath)],cwd=Path(__file__).parent,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
        try:
            self.assertEqual(child.stdout.readline().strip(),'WAITING')
            self.assertIsNone(child.poll())
            def runner(s,step,log):log.write_text('SUCCESS: '+step+'\n');return 0
            for _ in range(100):
                try:
                    with contextlib.redirect_stdout(io.StringIO()):self.o.operator(runner=runner)
                    break
                except order.Busy:time.sleep(.01)
            out,err=child.communicate(timeout=10)
            self.assertEqual(child.returncode,0,err);self.assertIn('DONE',out)
        finally:
            if child.poll() is None:child.kill();child.wait()
            child.stdout.close();child.stderr.close()

    def test_root_receives_the_verified_bytes_not_a_path(self):
        # #683 finding 17: argv is sudo,python,-I,-c,SOURCE,PLAN,DIGEST,STEP.
        # Whatever path sudo is handed, a process of the operator's UID can
        # swap before root opens it; so the run below swaps any payload path
        # it is given, and what root would execute must still be exactly the
        # bytes whose digest the review seat recorded.
        source=Path(order.__file__).with_name('tb_payload.py');recorded=self.state['review']['artifacts'][str(source.resolve())]
        planted=self.root/'.payload-planted.py';planted.write_text('raise SystemExit("planted")\n')
        before=sorted(p.name for p in self.root.iterdir());seen=[]
        def as_root(argv,**kw):
            if argv[3]=='-c':executed=argv[4].encode()
            else:Path(argv[3]).write_bytes(planted.read_bytes());executed=Path(argv[3]).read_bytes()
            seen.append((argv,kw['stdin'],executed));return subprocess.CompletedProcess(argv,0)
        with patch('tb_order.subprocess.run',side_effect=as_root):self.assertEqual(order.payload(self.state,'provision',self.root/'log'),0)
        (argv,stdin,executed),=seen
        self.assertEqual(hashlib.sha256(executed).hexdigest(),recorded)
        self.assertIs(stdin,subprocess.DEVNULL)
        self.assertEqual(argv[:4],['sudo','/usr/bin/python3','-I','-c'])
        self.assertEqual(argv[5:],[str(self.planpath),self.state['review']['artifacts'][str(self.planpath)],'provision'])
        self.assertEqual([a for a in argv[2:] if os.path.exists(a)],[str(self.planpath)])
        self.assertEqual(sorted(p.name for p in self.root.iterdir()),sorted(before+['log']))
        self.state['review']['artifacts'][str(source.resolve())]='bad'
        with self.assertRaisesRegex(order.Refused,'payload-hash'):order.payload(self.state,'provision',self.root/'log')
    def test_sudo_receives_digest_recorded_at_approval(self):
        # #683 finding 2: a plan edited after approved() must reach sudo with
        # the review seat's digest, so the payload's plan-hash check refuses it.
        recorded=self.state['review']['artifacts'][str(self.planpath)]
        self.planpath.write_text(self.planpath.read_text().replace('"bravo"','"karl"'))
        self.assertNotEqual(order.sha(self.planpath),recorded)
        seen=[]
        def inspect(argv,**kw):seen.append(argv[5:]);return subprocess.CompletedProcess(argv,0)
        with patch('tb_order.subprocess.run',side_effect=inspect):order.payload(self.state,'provision',self.root/'log')
        self.assertEqual(seen,[[str(self.planpath),recorded,'provision']])

    def test_approval_parses_the_plan_bytes_it_hashed(self):
        # A plan swapped between the artifact-hash read and the parse is refused,
        # never returned as approved.
        altered=dict(self.plan,note='swapped after hashing')
        real_sha=order.sha;swapped=[]
        def hook(path):
            digest=real_sha(path)
            if Path(path)==self.planpath and not swapped:
                swapped.append(1);self.planpath.write_text(json.dumps(altered))
            return digest
        with patch('tb_order.sha',side_effect=hook),self.assertRaises(order.Refused):self.o.approved(self.state)
        self.assertEqual(swapped,[1])

    def test_best_effort_notice(self):
        for error in [FileNotFoundError(),subprocess.TimeoutExpired('gh',20)]:
            with patch('tb_order.subprocess.run',side_effect=error),contextlib.redirect_stdout(io.StringIO()):order.notice('load:one')

    def test_lock_and_atomic_state(self):
        with self.o.locked():
            with self.assertRaises(order.Busy):
                with self.o.locked():pass
        self.o.fail('finding');self.assertEqual(self.o.read()['halt'],'finding')
        self.assertFalse(list(self.root.glob('.tb-*')))


class ReadingTests(unittest.TestCase):
    def rec(self):
        # extract_run's real key set (golden.EXTRACT_RUN_KEYS); field keys are
        # ints as extract_run builds them, strings once a record is JSON.
        return dict(dict.fromkeys(golden.EXTRACT_RUN_KEYS),output_tokens=[1],entropies=[0.5],surprisals=[0.25],
                    field={20:dict(ranked=[{'token':i,'probability':.005} for i in range(200)],realized=1)})

    def test_exact_rejects_empty_partial_and_changed(self):
        t=prepare.template(Path('/deposit'),'todd',1000)['tuple']
        a=self.rec();b=copy.deepcopy(a)
        self.assertTrue(driver.exact(t,a,b))
        b['field']={'20':b['field'][20]};self.assertTrue(driver.exact(t,a,b))
        b['entropies']=[.5000000000000001];self.assertFalse(driver.exact(t,a,b))
        b=copy.deepcopy(a);b['field'][20]['ranked'].pop()
        with self.assertRaises(order.Refused):driver.exact(t,a,b)
        b=copy.deepcopy(a);b['entropies']=[]
        with self.assertRaises(order.Refused):driver.exact(t,a,b)
        # #683 finding 23: the tuple elects surprisal, so the surprisals are
        # required and compared like the entropies: absent or empty refuses,
        # one bit changed is unequal.
        self.assertTrue(t['surprisal'])
        for missing in [[],None]:
            b=copy.deepcopy(a);b['surprisals']=missing
            with self.assertRaisesRegex(order.Refused,'nonempty-measurement'):driver.exact(t,a,b)
        b=copy.deepcopy(a);b['surprisals']=[.25000000000000006];self.assertFalse(driver.exact(t,a,b))
        # An unelected reading stays unrequired, and a true election with no
        # record key fails loudly instead of going unread.
        b=copy.deepcopy(a);b['surprisals']=None;self.assertTrue(driver.exact(dict(t,surprisal=False),dict(a,surprisals=None),b))
        with self.assertRaises(KeyError):driver.exact(dict(t,residual=True),a,a)
        self.assertNotEqual(driver.float_bits([0.0]),driver.float_bits([-0.0]))

    def test_trace_close_and_missing(self):
        with tempfile.TemporaryDirectory() as tmp:
            closed=event(golden.TURN_CLOSED);p=Path(tmp)/'trace';p.write_text(ndjson(closed))
            self.assertEqual(driver.until_closed(p,'turn.closed',1),[closed])
            with self.assertRaises(order.Refused):driver.until_closed(p,'replay.closed',0)

    def test_section_comparison_detects_instruction_change(self):
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)/'a';q=Path(tmp)/'b'
            section=dict(name='.text',sha256='a',executable=True)
            manifest=dict(stack='B1',hosts={'lib':dict(file_sha256='f',sections=[section])},members={'cubin':{'lib.1.sm_120a.cubin':dict(size=4,sha256='a',sections=[section])},'ptx':{'lib.1.sm_75.ptx':dict(size=4,sha256='a')}})
            p.write_text(json.dumps(manifest));q.write_text(json.dumps(dict(manifest,stack='B2')))
            self.assertTrue(sections.compare(p,q)['executable_identity'])
            changed=dict(copy.deepcopy(manifest),stack='B2');changed['members']['cubin']['lib.1.sm_120a.cubin']['sections'][0]['sha256']='b'
            q.write_text(json.dumps(changed));r=sections.compare(p,q)
            self.assertFalse(r['executable_identity']);self.assertEqual(r['cuda']['cubin']['sm_120a']['code_equal'],0)

    def test_host_identity_requires_the_whole_file(self):
        # #683 finding 3: equal section records do not make equal hosts. A file
        # hash that differs with no section change, or that was never recorded,
        # refuses identity.
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)/'a';q=Path(tmp)/'b'
            section=dict(name='.text',sha256='a',executable=True)
            members={'cubin':{'lib.1.sm_120a.cubin':dict(size=4,sha256='a',sections=[section])},'ptx':{'lib.1.sm_75.ptx':dict(size=4,sha256='a')}}
            p.write_text(json.dumps(dict(stack='B1',hosts={'lib':dict(file_sha256='f',sections=[section])},members=members)))
            for host in [dict(file_sha256='g',sections=[section]),dict(sections=[section])]:
                q.write_text(json.dumps(dict(stack='B2',hosts={'lib':host},members=members)))
                r=sections.compare(p,q)
                self.assertEqual(r['host']['lib']['changes'],[])
                self.assertFalse(r['executable_identity'],host)

    def test_identity_needs_a_b1_b2_pair_with_scope(self):
        # #683 finding 24: the same inventory twice, the pair reversed, or an
        # inventory with no hosts or members would make identity vacuous or
        # self-evident; each refuses rather than returning a verdict.
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)/'a';q=Path(tmp)/'b'
            section=dict(name='.text',sha256='a',executable=True)
            b1=dict(stack='B1',hosts={'lib':dict(file_sha256='f',sections=[section])},
                    members={'cubin':{'lib.1.sm_120a.cubin':dict(size=4,sha256='a',sections=[section])},'ptx':{'lib.1.sm_75.ptx':dict(size=4,sha256='a')}})
            p.write_text(json.dumps(b1));q.write_text(json.dumps(dict(b1,stack='B2')))
            self.assertEqual(sections.compare(p,q)['stacks'],['B1','B2'])
            for label,first,second in [('same',b1,b1),('reversed',dict(b1,stack='B2'),b1),
                                       ('no-ptx',b1,dict(b1,stack='B2',members=dict(b1['members'],ptx={}))),
                                       ('no-cubin',b1,dict(b1,stack='B2',members=dict(b1['members'],cubin={}))),
                                       ('no-hosts',dict(b1,hosts={}),dict(b1,stack='B2',hosts={}))]:
                with self.subTest(label=label):
                    p.write_text(json.dumps(first));q.write_text(json.dumps(second))
                    with self.assertRaises(ValueError):sections.compare(p,q)

    def test_identity_covers_every_file_a_load_reaches(self):
        # #683 finding 27: the inventory read only ELF files under bin and
        # engine-lib, so a different cuda-lib library, which every load puts on
        # LD_LIBRARY_PATH, or a different bin/basic_loop.py, which the worker
        # runs, left executable_identity true. The real inventory() runs here.
        elf=Path(sys.executable).resolve().read_bytes()
        with tempfile.TemporaryDirectory() as tmp:
            deposit=Path(tmp)
            def build(stack,cuda=elf,loop='print(1)\n'):
                root=deposit/'stacks'/stack
                for d in ['bin','engine-lib','cuda-lib']:(root/d).mkdir(parents=True,exist_ok=True)
                (root/'bin/worker').write_bytes(elf);(root/'bin/basic_loop.py').write_text(loop)
                (root/'engine-lib/libggml-cuda.so').write_bytes(elf);(root/'cuda-lib/libcudart.so.13').write_bytes(cuda)
                out=deposit/'sections'/stack
                for d in ['cubin','ptx']:(out/d).mkdir(parents=True,exist_ok=True)
                (out/'cubin/x.1.sm_120a.cubin').write_bytes(elf);(out/'ptx/x.1.sm_75.ptx').write_text('ptx\n')
            def inventory(stack):
                with patch('sys.argv',['sections.py',str(deposit),stack]),contextlib.redirect_stdout(io.StringIO()):sections.inventory()
                return deposit/'sections'/stack/'section-manifest.json'
            def verdict():return sections.compare(inventory('B1'),inventory('B2'))
            build('B1');build('B2')
            r=verdict();self.assertTrue(r['executable_identity'])
            self.assertIn('cuda-lib/libcudart.so.13',r['host']);self.assertIn('bin/basic_loop.py',r['host'])
            changed=bytearray(elf);changed[-1]^=1
            build('B2',cuda=bytes(changed));r=verdict()
            self.assertFalse(r['executable_identity']);self.assertFalse(r['host']['cuda-lib/libcudart.so.13']['file_equal'])
            build('B2',loop='print(2)\n');r=verdict()
            self.assertFalse(r['executable_identity']);self.assertFalse(r['host']['bin/basic_loop.py']['file_equal'])
            build('B2');(deposit/'stacks/B2/cuda-lib/libcudart.so').symlink_to('libcudart.so.13')
            with self.assertRaises(ValueError):inventory('B2')

    def test_host_headers_are_inventoried_and_compared(self):
        # Real ELF bytes: change the entry point, then one segment's permissions.
        # Neither lives in a section payload, so the section records stay equal.
        import struct
        with tempfile.TemporaryDirectory() as tmp:
            original=Path(tmp)/'original';original.write_bytes(Path(sys.executable).resolve().read_bytes())
            base=sections.elf_sections(original)
            self.assertGreater(len(base['program_headers']),0)
            members={'cubin':{'x.1.sm_120a.cubin':dict(size=4,sha256='a',sections=[])},'ptx':{'x.1.sm_75.ptx':dict(size=4,sha256='a')}}
            def manifest(record,stack='B2'):return dict(stack=stack,hosts={'bin/x':record},members=members)
            p=Path(tmp)/'a';p.write_text(json.dumps(manifest(base,'B1')))
            q=Path(tmp)/'b';q.write_text(json.dumps(manifest(base)))
            self.assertTrue(sections.compare(p,q)['executable_identity'])
            flags=base['elf_header']['phoff']+4
            for offset,fmt,part in [(24,'<Q','elf_header'),(flags,'<I','program_headers')]:
                data=bytearray(original.read_bytes())
                struct.pack_into(fmt,data,offset,struct.unpack_from(fmt,data,offset)[0]^1)
                changed=Path(tmp)/'changed';changed.write_bytes(data)
                record=sections.elf_sections(changed)
                self.assertEqual(record['sections'],base['sections'])
                q.write_text(json.dumps(manifest(record)));r=sections.compare(p,q)
                self.assertEqual(r['host']['bin/x']['changes'],[])
                # The first LOAD segment spans the ELF header, so an entry change moves its hash too.
                self.assertIn(part,[h['part'] for h in r['host']['bin/x']['header_changes']])
                self.assertFalse(r['executable_identity'],part)


TWO_RUNS=ndjson(event(golden.TURN_STARTED,run='r1'),event(golden.TURN_STARTED,run='r2'),event(golden.TURN_CLOSED,run='r2'))


class PayloadTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.addCleanup(self.tmp.cleanup)
        self.base=Path(self.tmp.name);self.root=self.base/'install';self.model=self.base/'model'
        self.plan=prepare.template(self.base/'deposit','todd',1000)
        self.planpath=self.base/'plan';self.planpath.write_text('{}')
        for name,value in [('ROOT',self.root),('MODEL',self.model)]:
            p=patch.object(payload,name,value);p.start();self.addCleanup(p.stop)
        p=patch('sys.argv',['payload',str(self.planpath),order.sha(self.planpath),'provision']);p.start();self.addCleanup(p.stop)
        self.user=SimpleNamespace(pw_uid=1000)

    def stacks(self):
        for stack in ['B1','B2']:
            root=Path(self.plan['stacks'][stack])
            for directory in ['bin','engine-lib','cuda-lib']:(root/directory).mkdir(parents=True)
            (root/'bin/pyworker').write_text('stub')
            self.plan['files'][str(root/'bin/pyworker')]=order.sha(root/'bin/pyworker')
        src=Path(self.plan['model_source']);src.parent.mkdir(parents=True);src.write_text('weights')
        self.plan['tuple']['weights_sha256']=order.sha(src)
        self.plan['files'][str(src)]=order.sha(src)

    def test_m1_interlock(self):
        def probe(stdout,rc=0,door=False,procs=None):
            with patch('tb_payload.subprocess.run',return_value=subprocess.CompletedProcess([],rc,stdout,'')),patch('tb_payload.Path.exists',return_value=door),patch('tb_payload.Path.iterdir',return_value=procs or []),patch('tb_payload.pwd.getpwnam',return_value=self.user):
                payload.m1_unloaded()
        # golden.SYSTEMCTL_SHOW_M1 is this box's real reading; the refusals
        # change one value in that real format.
        probe(golden.SYSTEMCTL_SHOW_M1)
        loaded=golden.SYSTEMCTL_SHOW_M1.replace('LoadState=not-found','LoadState=loaded')
        for out,rc,door,procs in [(loaded.replace('ActiveState=inactive','ActiveState=active'),0,False,[]),(loaded,1,False,[]),(loaded,0,True,[]),(loaded,0,False,[SimpleNamespace(name='42',stat=lambda:SimpleNamespace(st_uid=1000))])]:
            with self.assertRaises(RuntimeError):probe(out,rc,door,procs)

    def test_admin_reply_not_merely_exit_zero(self):
        def cp(state):return subprocess.CompletedProcess([],0,golden.ADMIN_STATE.format(state)+'\n','')
        with patch('tb_payload.run',return_value=cp('unloaded')),contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(payload.answer('B1','show','unloaded')['state'],'unloaded')
        with patch('tb_payload.run',return_value=cp('idle')),self.assertRaises(RuntimeError):payload.answer('B1','show','unloaded')
        # A real refusal is one line with exit 1: run() keeps the line in the
        # transcript and refuses by name, never an unnamed exception.
        refusal=subprocess.CompletedProcess(['weaver-admin'],1,golden.ADMIN_NO_RESIDENCY+'\n','')
        with patch('tb_payload.subprocess.run',return_value=refusal),contextlib.redirect_stdout(io.StringIO()) as out,contextlib.redirect_stderr(io.StringIO()),self.assertRaisesRegex(RuntimeError,'command-exit'):payload.answer('B1','unload','unloaded')
        self.assertIn(golden.ADMIN_NO_RESIDENCY,out.getvalue())

    def test_save_refuses_symlink(self):
        p=self.base/'file';payload.save(p,'a');self.assertEqual(p.read_text(),'a')
        link=self.base/'link';link.symlink_to(p)
        with self.assertRaises(RuntimeError):payload.save(link,'b')
        self.assertEqual(p.read_text(),'a')

    def provision(self):
        with patch('tb_payload.pwd.getpwnam',side_effect=KeyError),patch('tb_payload.grp.getgrnam',return_value=SimpleNamespace(gr_gid=os.getgid())),patch('tb_payload.os.chown'),patch('tb_payload.run'),contextlib.redirect_stdout(io.StringIO()):payload.provision(self.plan,'d'*64)

    def test_provision_and_installed_hash_checks(self):
        self.stacks();self.provision()
        self.assertTrue((self.root/'config/B1/allow-list').is_file())
        self.assertEqual((self.root/'config/B1/allow-list').read_text(),'bravo\n')
        payload.installed(self.plan,'d'*64)
        file=self.root/'stacks/B1/bin/pyworker';file.write_text('changed')
        with self.assertRaises(RuntimeError):payload.installed(self.plan,'d'*64)
        file.write_text('stub');self.model.write_text('changed')
        with self.assertRaises(RuntimeError):payload.installed(self.plan,'d'*64)
        self.model.write_text('weights');(self.root/'plan-sha256').write_text('changed')
        with self.assertRaises(RuntimeError):payload.installed(self.plan,'d'*64)
        with self.assertRaises(RuntimeError):self.provision()

    def test_provision_refuses_stack_links_before_mutation(self):
        # #683 finding 4: a linked directory is never descended and a linked file
        # is hashed at its target, so either would pass coverage and carry
        # unreviewed bytes into the root-owned install. Both refuse before ROOT.
        self.stacks()
        outside=self.base/'outside';outside.mkdir();(outside/'lib.so').write_text('unreviewed')
        twin=self.base/'twin';twin.write_text('stub')
        stack=Path(self.plan['stacks']['B1'])
        for kind in ['directory','file']:
            with self.subTest(kind=kind):
                if kind=='directory':(stack/'engine-lib/linked').symlink_to(outside,target_is_directory=True)
                else:(stack/'bin/pyworker').unlink();(stack/'bin/pyworker').symlink_to(twin)
                with self.assertRaisesRegex(RuntimeError,'stack-no-symlinks'):self.provision()
                self.assertFalse(self.root.exists())
                if kind=='directory':(stack/'engine-lib/linked').unlink()
                else:(stack/'bin/pyworker').unlink();(stack/'bin/pyworker').write_text('stub')
        self.provision();self.assertFalse((self.root/'stacks/B1/bin/pyworker').is_symlink())

    def test_provision_verifies_the_copy_it_installed(self):
        # #683 finding 7: a stack file changed after the pre-copy checks is
        # caught in the installed copy before provisioning reports success.
        self.stacks();real=shutil.copytree
        def racing(src,dst,*args,**kw):
            if (Path(src)/'bin').is_dir():(Path(src)/'bin/pyworker').write_text('changed after the check')
            return real(src,dst,*args,**kw)
        with patch('tb_payload.shutil.copytree',side_effect=racing),self.assertRaisesRegex(RuntimeError,'installed-stack-hash'):self.provision()

    def test_served_model_must_be_in_custody(self):
        # #683 finding 19: a model already present is accepted only as a
        # regular, single-named file only the payload's user can write.
        self.stacks();src=Path(self.plan['model_source'])
        for fault in ['symlink','hard-link','group-writable']:
            with self.subTest(fault=fault):
                if fault=='symlink':self.model.symlink_to(src)
                if fault=='hard-link':os.link(src,self.model)
                if fault=='group-writable':self.model.write_bytes(src.read_bytes());self.model.chmod(0o664)
                try:
                    with self.assertRaisesRegex(RuntimeError,'existing-model-custody'):self.provision()
                    self.assertFalse(self.root.exists())
                finally:self.model.unlink()
        self.provision();payload.installed(self.plan,'d'*64)
        self.model.unlink();self.model.symlink_to(src)
        with self.assertRaisesRegex(RuntimeError,'installed-model-custody'):payload.installed(self.plan,'d'*64)

    def test_served_directories_are_locked_and_stay_locked(self):
        # #683 finding 22: copytree copies the source root's mode onto the
        # installed root, and rglob never yields the root, so a world-writable
        # source left stacks/B1 writable. Every served directory is locked when
        # it is made, and installed() refuses one reopened later, by name.
        self.stacks();Path(self.plan['stacks']['B1']).chmod(0o777)
        self.provision()
        installed=self.root/'stacks/B1'
        self.assertEqual(installed.stat().st_mode&0o777,0o755)
        for d in payload.served_directories():self.assertEqual(d.stat().st_mode&0o022,0,d)
        payload.installed(self.plan,'d'*64)
        installed.chmod(0o777)
        with self.assertRaisesRegex(RuntimeError,'installed-stack-custody'):payload.installed(self.plan,'d'*64)
        installed.chmod(0o755);(self.root/'config/B1').chmod(0o777)
        with self.assertRaisesRegex(RuntimeError,'served-directory-custody'):payload.installed(self.plan,'d'*64)

    def test_new_model_must_be_in_custody(self):
        # #683 finding 30: a model this provisioning creates is held like an
        # existing one; a group-writable parent lets another process replace
        # the new file after its hash, so provisioning refuses before success.
        self.stacks();self.model.parent.chmod(0o775)
        try:
            with self.assertRaisesRegex(RuntimeError,'new-model-custody'):self.provision()
        finally:self.model.parent.chmod(0o700)

    def test_inventory_roots_pin_the_served_set(self):
        # #683 finding 31: the inventory walks tb_payload.STACK_ROOTS, so the
        # served set is derived here from the payload's own output, the
        # LD_LIBRARY_PATH it exports and the binaries it configures, and pinned
        # to that constant. A directory served from outside it fails here.
        self.stacks();self.provision()
        stack=self.root/'stacks/B1';served=set()
        for entry in payload.environment('B1')['LD_LIBRARY_PATH'].split(':'):
            served.add(str(Path(entry).relative_to(stack)))
        for name in ['worker-binary','spu-binary','gate-binary']:
            served.add(str(Path((self.root/'config/B1'/name).read_text().strip()).relative_to(stack).parent))
        self.assertTrue(served,served)
        self.assertLessEqual(served,set(payload.STACK_ROOTS),f'served outside the inventoried roots: {served-set(payload.STACK_ROOTS)}')
        self.assertIs(sections.STACK_ROOTS,payload.STACK_ROOTS)

    def test_installed_copy_holds_exactly_the_reviewed_bytes(self):
        self.stacks();self.provision();payload.installed(self.plan,'d'*64)
        installed=self.root/'stacks/B1/bin/pyworker'
        extra=self.root/'stacks/B1/engine-lib/extra.so';extra.write_text('unreviewed')
        with self.assertRaisesRegex(RuntimeError,'installed-stack-coverage'):payload.installed(self.plan,'d'*64)
        extra.unlink();twin=self.base/'twin';twin.write_text('stub')
        installed.unlink();installed.symlink_to(twin)
        with self.assertRaisesRegex(RuntimeError,'installed-no-symlinks'):payload.installed(self.plan,'d'*64)
        installed.unlink();installed.write_text('stub');payload.installed(self.plan,'d'*64)

    def test_provision_preconditions_before_mutation(self):
        self.stacks()
        for fault in ['weights','existing-model','libraries','coverage']:
            with self.subTest(fault=fault):
                changed=copy.deepcopy(self.plan)
                if fault=='weights':changed['tuple']['weights_sha256']='bad'
                if fault=='existing-model':self.model.write_text('bad')
                if fault=='libraries':(Path(changed['stacks']['B1'])/'cuda-lib').rmdir()
                if fault=='coverage':changed['files']={}
                with patch.object(self,'plan',changed),self.assertRaises(RuntimeError):self.provision()
                self.assertFalse(self.root.exists())
                self.model.unlink(missing_ok=True)
                (Path(self.plan['stacks']['B1'])/'cuda-lib').mkdir(exist_ok=True)
        with patch('tb_payload.pwd.getpwnam',return_value=self.user),self.assertRaisesRegex(RuntimeError,'no-bravo-account'):payload.provision(self.plan,'d'*64)

    def setup_load(self):
        self.root.mkdir();(self.root/'sinks').mkdir();(self.root/'agents/B1').mkdir(parents=True)
        return dict(id='job',kind='free',stack='B1',seed=7)

    def invoke_load(self,job,gpu=None,ldd=None,loader_rc=0,door=True,artifact=True,on_run=None,declared=None):
        default_gpu=golden.NVIDIA_SMI.strip()
        default_ldd=ldd_output(self.base,f'{self.root}/stacks/B1/cuda-lib')
        def run(argv,**kwargs):
            if on_run:on_run(argv)
            if 'nvidia-smi' in argv[0]:return subprocess.CompletedProcess(argv,0,default_gpu if gpu is None else gpu,'')
            if 'ldd' in argv[0]:return subprocess.CompletedProcess(argv,0,default_ldd if ldd is None else ldd,'')
            if 'derive' in argv:
                Path(argv[-1]).write_text(derived(self.model if artifact else golden.DERIVE_ARTIFACT,argv[argv.index('--sink')+1],self.plan['tuple'],**(declared or {})))
            return subprocess.CompletedProcess(argv,0,'','')
        sockets=[]
        def loader(*args,**kw):
            import socket
            if door:
                state=self.root/'sinks/job/state';state.mkdir()
                sock=socket.socket(socket.AF_UNIX);sock.bind(str(state/'preload.sock'));sockets.append(sock)
            return SimpleNamespace(poll=lambda:None if door else 1,wait=lambda **kw:loader_rc,kill=lambda:None)
        try:
            with patch('tb_payload.m1_unloaded'),patch('tb_payload.answer'),patch('tb_payload.run',side_effect=run),patch('tb_payload.os.chown'),patch('tb_payload.grp.getgrnam',return_value=SimpleNamespace(gr_gid=os.getgid())),patch('tb_payload.subprocess.Popen',side_effect=loader),contextlib.redirect_stdout(io.StringIO()):payload.load(self.plan,job)
        finally:
            for sock in sockets:sock.close()

    def test_load_tuple_and_libraries(self):
        job=self.setup_load()
        # The host-resolved case is golden's real capture: B1 has no cuda-lib.
        for gpu,ldd in [(golden.NVIDIA_SMI.strip().replace('0, ','1, ',1),None),(None,golden.LDD_B1_LIBGGML_CUDA.replace('{DEPOSIT}',str(self.base))),(None,ldd_output(self.base,f'{self.root}/stacks/B1/cuda-lib')+'\tlibother.so => not found\n')]:
            with self.assertRaises(RuntimeError):self.invoke_load(job,gpu=gpu,ldd=ldd)
        self.invoke_load(job)
        self.assertIn('seed: 7',(self.root/'agents/B1/bravo.yaml').read_text())

    def test_replay_load_orders_preload_before_wait(self):
        job=self.setup_load();source=self.base/'source';source.write_text(TWO_RUNS)
        job.update(kind='refeed',source_trace=str(source),source_run='r2');self.plan['files'][str(source)]=order.sha(source)
        self.invoke_load(job)
        self.assertTrue((self.root/'sinks/job/load.log').exists())

    def test_replay_refusals(self):
        job=self.setup_load();source=self.base/'source';source.write_text(TWO_RUNS)
        job.update(kind='refeed',source_trace=str(source),source_run='r2');self.plan['files'][str(source)]=order.sha(source)
        for setting in [dict(artifact=False),dict(door=False),dict(loader_rc=1)]:
            with self.assertRaises(RuntimeError):self.invoke_load(job,**setting)
            import shutil
            shutil.rmtree(self.root/'sinks/job');shutil.rmtree(self.root/'snapshots/job')
        self.plan['files'][str(source)]='wrong'
        with self.assertRaisesRegex(RuntimeError,'snapshot-hash'):self.invoke_load(job)
        self.assertEqual(list((self.root/'snapshots/job').iterdir()),[])

    def test_replay_feeds_one_verified_snapshot_of_the_selected_run(self):
        # #683 findings 9 and 10: derive and preload each read their input.
        # Both must receive one root-owned file cut from a single verified
        # read, holding only the named run, whatever happens to the source.
        job=self.setup_load();source=self.base/'source';source.write_text(TWO_RUNS)
        job.update(kind='refeed',source_trace=str(source),source_run='r2');self.plan['files'][str(source)]=order.sha(source)
        seen={}
        def on_run(argv):
            for verb in ['derive','preload']:
                if verb in argv:
                    fed=Path(argv[argv.index(verb)+1]);seen[verb]=(fed,fed.read_bytes())
                    source.write_text(ndjson(event(golden.TURN_CLOSED,run='r2',sequence='98')))
        with swapped_after_hashing({source:ndjson(event(golden.TURN_CLOSED,run='r2',sequence='99')).encode()}):self.invoke_load(job,on_run=on_run)
        selected=''.join(l for l in TWO_RUNS.splitlines(keepends=True) if json.loads(l)['run']=='r2').encode()
        self.assertNotEqual(seen['derive'][0],source)
        self.assertEqual(seen['derive'],seen['preload'])
        self.assertEqual(seen['derive'][1],selected)

    def test_derived_artifact_is_read_as_derive_renders_it(self):
        # #683 finding 13: derive quotes the artifact (golden's real capture).
        # The check parses the value, so the real form passes and a declaration
        # naming another model, or no parseable model, refuses.
        self.assertEqual(payload.declared_artifacts(golden.DERIVE_DECLARATION),[golden.DERIVE_ARTIFACT])
        self.assertEqual(payload.declared_artifacts(derived(self.model,'/sink')),[str(self.model)])
        self.assertEqual(payload.declared_artifacts(f'      artifact: {self.model}\n'),[None])
        job=self.setup_load();source=self.base/'source';source.write_text(TWO_RUNS)
        job.update(kind='refeed',source_trace=str(source),source_run='r2');self.plan['files'][str(source)]=order.sha(source)
        self.invoke_load(job)
        shutil.rmtree(self.root/'sinks/job');shutil.rmtree(self.root/'snapshots/job')
        with self.assertRaisesRegex(RuntimeError,'derived-artifact'):self.invoke_load(job,artifact=False)

    def test_replay_source_must_hold_the_tuple(self):
        # #683 finding 20: derive carries the source run's own seed, identity,
        # context capacity and token cap into the replay, so each must hold the
        # plan's tuple; golden's real W4a declaration, another tuple, refuses.
        job=self.setup_load();source=self.base/'source';source.write_text(TWO_RUNS)
        job.update(kind='refeed',source_trace=str(source),source_run='r2');self.plan['files'][str(source)]=order.sha(source)
        faults=[{'seed':999},{'context-capacity':32768},{'max-tokens-per-turn':4096},
                {'identity':[{'role':'system','content':[{'type':'text','text':'another identity'}]}]}]
        for fault in faults:
            with self.subTest(fault=list(fault)[0]):
                try:
                    with self.assertRaisesRegex(RuntimeError,'derived-tuple'):self.invoke_load(job,declared=fault)
                finally:shutil.rmtree(self.root/'sinks/job',ignore_errors=True);shutil.rmtree(self.root/'snapshots/job',ignore_errors=True)
        # A local source holds its own free job's seed, not merely a tuple seed.
        free=next(j for j in self.plan['arms'][0]['jobs'] if j['kind']=='free')
        (self.root/'sinks'/free['id']).mkdir();(self.root/'sinks'/free['id']/'trace.ndjson').write_text(TWO_RUNS)
        local=dict(id='job',kind='refeed',stack='B1',source_job=free['id'])
        other=next(s for s in self.plan['tuple']['seeds'] if s!=free['seed'])
        with self.assertRaisesRegex(RuntimeError,'derived-tuple'):self.invoke_load(local,declared={'seed':other})
        shutil.rmtree(self.root/'sinks/job');shutil.rmtree(self.root/'snapshots/job')
        self.invoke_load(local,declared={'seed':free['seed']})
        self.assertFalse(payload.holds_tuple(self.plan,job,derived(self.model,'/sink')))

    def test_replay_refuses_a_run_the_trace_does_not_hold(self):
        job=self.setup_load();source=self.base/'source';source.write_text(TWO_RUNS)
        job.update(kind='refeed',source_trace=str(source),source_run='r9');self.plan['files'][str(source)]=order.sha(source)
        with self.assertRaisesRegex(RuntimeError,'source-run-selected'):self.invoke_load(job)

    def test_provision_serves_only_verified_model_bytes(self):
        # #683 finding 8: model bytes changed after the model-source check are
        # refused in the installed copy, which is removed, before success.
        self.stacks();src=Path(self.plan['model_source'])
        with swapped_after_hashing({src:b'unreviewed weights'}),self.assertRaisesRegex(RuntimeError,'snapshot-hash'):self.provision()
        self.assertFalse(self.model.exists())

    def test_payload_entry_checks(self):
        p=copy.deepcopy(self.plan);p['install_root']=str(self.root);p['files']={}
        def invoke(p,uid=0,sudo='1000',digest=None,step='provision'):
            self.planpath.write_text(json.dumps(p))
            with patch('sys.argv',['payload',str(self.planpath),digest or order.sha(self.planpath),step]),patch('tb_payload.os.geteuid',return_value=uid),patch.dict(os.environ,{'SUDO_UID':sudo}),patch('tb_payload.pwd.getpwnam',return_value=self.user),patch('tb_payload.provision'),patch('tb_payload.installed'),patch('tb_payload.answer'),contextlib.redirect_stdout(io.StringIO()):payload.main()
        invoke(p)
        for change,kw in [(lambda p:None,dict(uid=1000)),(lambda p:None,dict(digest='wrong')),(lambda p:p.update(agent='karl'),{}),(lambda p:None,dict(sudo='9')),(lambda p:p['files'].update({str(self.planpath):'bad'}),{}),(lambda p:None,dict(step='unload:missing'))]:
            bad=copy.deepcopy(p);change(bad)
            with self.assertRaises(RuntimeError):invoke(bad,**kw)
        invoke(p,step='unload:B1-s7-n1')
        with self.assertRaisesRegex(RuntimeError,'known-step'):invoke(p,step='frob:B1-s7-n1')

    def test_payload_parses_the_snapshot_it_verified(self):
        # #683 finding 2: sudo verifies one snapshot. A plan swapped on disk after
        # the digest check must not be what provision receives.
        p=copy.deepcopy(self.plan);p['install_root']=str(self.root);p['files']={}
        self.planpath.write_text(json.dumps(p));expected=order.sha(self.planpath)
        altered=dict(p,note='swapped after hashing')
        real=payload.hashlib.sha256;swapped=[]
        class Hooked:
            def __init__(s,*a):s.h=real(*a)
            def update(s,b):s.h.update(b)
            def hexdigest(s):
                r=s.h.hexdigest()
                if not swapped:swapped.append(1);self.planpath.write_text(json.dumps(altered))
                return r
        with patch('sys.argv',['payload',str(self.planpath),expected,'provision']),patch('tb_payload.os.geteuid',return_value=0),patch.dict(os.environ,{'SUDO_UID':'1000'}),patch('tb_payload.pwd.getpwnam',return_value=self.user),patch('tb_payload.provision') as provision,patch('tb_payload.hashlib.sha256',side_effect=Hooked),contextlib.redirect_stdout(io.StringIO()):payload.main()
        self.assertEqual(swapped,[1])
        self.assertEqual(provision.call_args.args,(p,expected))


class DriverTests(unittest.TestCase):
    save = Fixture.save

    def setUp(self):
        Fixture.setUp(self)
        self.plan['install_root']=str(self.root/'installed')
        self.plan['deposit']=str(self.root/'deposit');Path(self.plan['deposit']).mkdir()
        self.free=ReadingTests().rec()
        self.free.update(emission='essay',declared_seed=7,weights_hash=self.plan['tuple']['weights_sha256'],input_tokens=10)
        self.probe=SimpleNamespace(base=SimpleNamespace(gate_turn=lambda *a,**kw:dict(json.loads(golden.GATE_ANSWERED),run='r')),
                                   ESSAY_PROMPT='fixed',extract_run=lambda rows:copy.deepcopy(self.free),
                                   measured_events=lambda rows,run:rows,reading_two=lambda a,b:dict(positions_compared=1),
                                   reading_one=lambda a,b:{},first_divergence=lambda a,b:0)

    def job(self,kind='free'):
        return dict(id='job',kind=kind,stack='B1',seed=7,source_trace=str(self.source),source_run='r')

    def close(self,replay=False):
        if replay:return [event(golden.REPLAY_CLOSED_CERTIFIED,run='r')]
        return [event(golden.MODEL_MEASUREMENT,run='r'),event(golden.TURN_CLOSED,run='r')]

    def cleanup_job(self):
        import shutil
        for part in ['runs','refeeds']:
            shutil.rmtree(Path(self.plan['deposit'])/part,ignore_errors=True)

    def test_free_measurement_refusals(self):
        j=self.job()
        with patch('tb_driver.until_closed',return_value=self.close()):
            result=driver.measure(self.plan,j,self.probe)
            self.assertEqual(json.loads(result.read_text())['run'],'r')
        self.cleanup_job()
        for fault in ['answer','single-turn','seed','weights']:
            old=copy.deepcopy(self.free)
            if fault=='seed':self.free['declared_seed']=8
            if fault=='weights':self.free['weights_hash']='bad'
            rows=[] if fault=='single-turn' else self.close()
            gate=self.probe.base.gate_turn
            if fault=='answer':self.probe.base.gate_turn=lambda *a,**k:json.loads(golden.GATE_REFUSED)
            with patch('tb_driver.until_closed',return_value=rows),self.assertRaises(order.Refused):driver.measure(self.plan,j,self.probe)
            self.probe.base.gate_turn=gate;self.free=old;self.cleanup_job()

    def test_refeed_requires_close_measurement_and_source(self):
        j=self.job('refeed')
        self.source.write_text(ndjson(event(golden.MODEL_MEASUREMENT,run='r')));self.plan['files'][str(self.source)]=order.sha(self.source)
        with patch('tb_driver.until_closed',return_value=self.close(True)):
            result=driver.measure(self.plan,j,self.probe)
            self.assertTrue(json.loads(result.read_text())['exact'])
        self.cleanup_job()
        for fault in ['close','outcome','measurement','source-hash','source-measurement']:
            rows=self.close(True)
            if fault=='close':rows=rows*2
            if fault=='outcome':rows[0]['payload']['outcome']=dict(kind='abandoned',reason=dict(kind='replay_ask_unanswered'))
            if fault=='source-hash':self.plan['files'][str(self.source)]='bad'
            if fault=='source-measurement':self.source.write_text(ndjson(event(golden.TURN_CLOSED,run='r')));self.plan['files'][str(self.source)]=order.sha(self.source)
            self.probe.measured_events=(lambda rows,run:None) if fault=='measurement' else (lambda rows,run:rows)
            with patch('tb_driver.until_closed',return_value=rows),self.assertRaises((order.Refused,KeyError)):driver.measure(self.plan,j,self.probe)
            self.cleanup_job();self.source.write_text(ndjson(event(golden.MODEL_MEASUREMENT,run='r')));self.plan['files'][str(self.source)]=order.sha(self.source)

    def test_assess_control_pass_and_falsifiers(self):
        arm=self.plan['arms'][0]
        for j in arm['jobs']:
            directory=Path(self.plan['deposit'])/('runs' if j['kind']=='free' else 'refeeds')/j['id'];directory.mkdir(parents=True)
            r=dict(self.free,seed=j.get('seed')) if j['kind']=='free' else dict(exact=True,replay_outcome='certified')
            order.atomic(directory/('run.json' if j['kind']=='free' else 'refeed.json'),r)
        result=driver.assess(self.plan,arm,self.probe)
        self.assertTrue(json.loads(result.read_text())['control_passed'])
        j=arm['jobs'][-1];p=Path(self.plan['deposit'])/'refeeds'/j['id']/'refeed.json'
        order.atomic(p,dict(exact=False,replay_outcome='diverged'))
        with self.assertRaises(order.Refused):driver.assess(self.plan,arm,self.probe)
        order.atomic(p,dict(exact=True,replay_outcome='certified'))
        self.probe.first_divergence=lambda a,b:24
        with self.assertRaises(order.Refused):driver.assess(self.plan,arm,self.probe)
        shorter=copy.deepcopy(arm);shorter['jobs'].pop(0)
        with self.assertRaises(order.Refused):driver.assess(self.plan,shorter,self.probe)

    def test_changed_seeds_compare_one_run_per_distinct_seed(self):
        # #683 finding 16: the validator constrains only the multiset of free
        # seeds, so a valid plan may put both runs of one seed first. Each seed
        # has its own tokens here and the real first_divergence contract (None
        # only for identical lists), so a same-seed pair would falsify TB0.
        arm=copy.deepcopy(self.plan['arms'][0]);free=sorted((j for j in arm['jobs'] if j['kind']=='free'),key=lambda j:j['seed'])
        arm['jobs']=free+[j for j in arm['jobs'] if j['kind']=='refeed']
        order.validate_plan(dict(self.plan,install_root='/var/lib/weaver-tb',arms=[arm]+self.plan['arms'][1:]))
        for j in arm['jobs']:
            directory=Path(self.plan['deposit'])/('runs' if j['kind']=='free' else 'refeeds')/j['id'];directory.mkdir(parents=True)
            r=dict(self.free,seed=j.get('seed'),output_tokens=[j['seed']%1000]) if j['kind']=='free' else dict(exact=True,replay_outcome='certified')
            order.atomic(directory/('run.json' if j['kind']=='free' else 'refeed.json'),r)
        self.probe.first_divergence=lambda a,b:None if a==b else next((i for i,(x,y) in enumerate(zip(a,b)) if x!=y),min(len(a),len(b)))
        report=json.loads(driver.assess(self.plan,arm,self.probe).read_text())
        seeds=self.plan['tuple']['seeds']
        self.assertEqual({(c['a'],c['b']) for c in report['changed_seeds']},{(a,b) for i,a in enumerate(seeds) for b in seeds[i+1:]})
        self.assertTrue(report['changed_seed_prediction'])

    def test_driver_settles_before_next_load_and_stops_falsifier(self):
        # Small schedule exercises the blocking orchestration; full count and
        # source schedule are independently guarded by validate_plan tests.
        arm=dict(name='TB0',jobs=[dict(id='a',kind='free',seed=7),dict(id='b',kind='free',seed=7),dict(id='c',kind='refeed')])
        self.plan['arms']=[arm]
        state=dict(plan=str(self.planpath),review=dict(artifacts={str(self.planpath):'recorded-at-approval'}));seen=[]
        o=SimpleNamespace(read=lambda:state,approved=lambda s:self.plan,
                          coding=lambda step,path:seen.append(step),wait=lambda step:seen.append('wait:'+step))
        def measure(plan,job,probe):
            p=Path(plan['deposit'])/'runs'/job['id']/'run.json';p.parent.mkdir(parents=True,exist_ok=True)
            order.atomic(p,dict(self.free,exact=False,replay_outcome='diverged'));return p
        with patch('tb_driver.readers',return_value=self.probe),patch('tb_driver.notice'),patch('tb_driver.measure',side_effect=measure),self.assertRaises(order.Refused):driver.drive(o,'TB0')
        self.assertIn('settle:b',seen);self.assertIn('wait:settle:c',seen);self.assertNotIn('settle:c',seen)
        # The start record names the digest the review seat approved, not a re-hash.
        self.assertEqual(json.loads((Path(self.plan['deposit'])/'TB0-start.json').read_text())['plan'],'recorded-at-approval')
        with patch('tb_driver.readers',return_value=self.probe),self.assertRaises(order.Refused):driver.drive(o,'TB0')
        (Path(self.plan['deposit'])/'TB0-start.json').unlink()
        def bad_pair(plan,job,probe):
            p=measure(plan,job,probe)
            if job['id']=='b':
                r=json.loads(p.read_text());r['emission']='changed';order.atomic(p,r)
            return p
        with patch('tb_driver.readers',return_value=self.probe),patch('tb_driver.notice'),patch('tb_driver.measure',side_effect=bad_pair),self.assertRaisesRegex(order.Refused,'pair-falsifier'):driver.drive(o,'TB0')

    def test_reader_hash_and_no_cached_code(self):
        root=self.root/'instrument'
        for parent,file,content in [('cross-precision-repro','confirm_cells.py','VALUE=1'),('weaver-probe','weaver_probe.py','VALUE=2')]:
            p=root/parent/file;p.parent.mkdir(parents=True);p.write_text(content);self.plan['files'][str(p)]=order.sha(p)
        self.plan['instrument']=str(root)
        self.assertEqual(driver.readers(self.plan).VALUE,2)
        (root/'weaver-probe/weaver_probe.py').write_text('VALUE=3')
        with self.assertRaises(order.Refused):driver.readers(self.plan)

    def test_refeed_requires_the_tuple_weights_on_both_sides(self):
        # #683 finding 11: a reviewed trace from other weights, or a replay
        # that ran them, must not be read as a device effect.
        j=self.job('refeed')
        self.source.write_text(ndjson(event(golden.MODEL_MEASUREMENT,run='r')));self.plan['files'][str(self.source)]=order.sha(self.source)
        for side in ['source','replay']:
            with self.subTest(side=side):
                other=dict(copy.deepcopy(self.free),weights_hash='f'*64)
                self.probe.extract_run=(lambda rows,side=side,other=other:copy.deepcopy(other) if (side=='replay')==any(e['kind']=='replay.closed' for e in rows) else copy.deepcopy(self.free))
                try:
                    with patch('tb_driver.until_closed',return_value=self.close(True)),self.assertRaisesRegex(order.Refused,f'{side}-weights-held'):driver.measure(self.plan,j,self.probe)
                finally:self.cleanup_job()

    def test_refeed_requires_the_source_seed_on_both_sides(self):
        # #683 finding 21: a replay that ran another seed, or a source outside
        # the tuple's seeds, must not be read as a device or kernel effect.
        j=self.job('refeed')
        self.source.write_text(ndjson(event(golden.MODEL_MEASUREMENT,run='r')));self.plan['files'][str(self.source)]=order.sha(self.source)
        held=self.plan['tuple']['seeds'][0];other=self.plan['tuple']['seeds'][1]
        for guard,source_seed,replay_seed in [('source-seed-held',424242,424242),('replay-seed-held',held,other)]:
            with self.subTest(guard=guard):
                def extract(rows,s=source_seed,r=replay_seed):
                    replay=any(e['kind']=='replay.closed' for e in rows)
                    return dict(copy.deepcopy(self.free),declared_seed=r if replay else s)
                self.probe.extract_run=extract
                try:
                    with patch('tb_driver.until_closed',return_value=self.close(True)),self.assertRaisesRegex(order.Refused,guard):driver.measure(self.plan,j,self.probe)
                finally:self.cleanup_job()
        self.probe.extract_run=lambda rows:dict(copy.deepcopy(self.free),declared_seed=held)
        with patch('tb_driver.until_closed',return_value=self.close(True)):driver.measure(self.plan,j,self.probe)

    def test_readers_compile_the_bytes_they_hashed(self):
        # Codex pass 4 class, driver side: a reader rewritten after its hash
        # check must not be what runs.
        root=self.root/'instrument';files={}
        for parent,file,content in [('cross-precision-repro','confirm_cells.py','VALUE=1'),('weaver-probe','weaver_probe.py','VALUE=2')]:
            p=root/parent/file;p.parent.mkdir(parents=True);p.write_text(content);self.plan['files'][str(p)]=order.sha(p);files[p]=b'VALUE=99'
        self.plan['instrument']=str(root)
        with swapped_after_hashing(files) as pending:module=driver.readers(self.plan)
        self.assertEqual(pending,{})
        self.assertEqual((module.VALUE,sys.modules['confirm_cells'].VALUE),(2,1))

    def test_source_record_parses_the_bytes_it_hashed(self):
        j=self.job('refeed');seen=[]
        self.source.write_text(ndjson(event(golden.MODEL_MEASUREMENT,run='r')));self.plan['files'][str(self.source)]=order.sha(self.source)
        self.probe.extract_run=lambda rows:seen.append(rows) or copy.deepcopy(self.free)
        with swapped_after_hashing({self.source:ndjson(event(golden.MODEL_MEASUREMENT,run='r',sequence='99')).encode()}):driver.source_record(self.plan,j,self.probe)
        self.assertEqual(seen,[[event(golden.MODEL_MEASUREMENT,run='r')]])

    def test_entry_refuses_root(self):
        with patch('sys.argv',['driver','--state',str(self.statepath),'TB0']),patch('tb_driver.os.geteuid',return_value=0),patch('tb_driver.drive'),contextlib.redirect_stderr(io.StringIO()):
            self.assertEqual(driver.main(),1)
        self.assertTrue(self.o.read()['halt'])



class AdditionalTests(unittest.TestCase):
    save = Fixture.save
    setUp = Fixture.setUp
    def test_changed_artifact_contents(self):
        self.source.write_text('changed')
        with self.assertRaises(order.Refused):self.o.approved(self.state)

    def test_operator_and_driver_entry_uid(self):
        for uid, expected in [(0,1),(1000,0)]:
            with patch('sys.argv',['operator','--state',str(self.statepath)]),patch('tb_order.os.geteuid',return_value=uid),patch('tb_order.Order.operator'),contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(order.main(),expected)
            with patch('sys.argv',['driver','--state',str(self.statepath),'TB0']),patch('tb_driver.os.geteuid',return_value=uid),patch('tb_driver.drive'),contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(driver.main(),expected)

class PerturbationBaselineTests(unittest.TestCase):
    def test_a_failing_baseline_refuses_before_any_mutation(self):
        # #683 finding 5: a suite that already fails would read every mutation
        # as killed. The runner must refuse instead of reporting full detection.
        with tempfile.TemporaryDirectory() as tmp:
            here=Path(__file__).resolve().parent;copy_dir=Path(tmp)
            for p in here.glob('*'):
                if p.suffix in ['.py','.sh']:(copy_dir/p.name).write_bytes(p.read_bytes())
            (copy_dir/'test_tb.py').write_text('import unittest\nclass T(unittest.TestCase):\n    def test_broken(self):self.fail("baseline broken")\n')
            result=subprocess.run([sys.executable,'-B',str(copy_dir/'perturb.py')],capture_output=True,text=True,timeout=120,env=dict(os.environ,PYTHONDONTWRITEBYTECODE='1'))
            self.assertNotEqual(result.returncode,0)
            self.assertIn('BASELINE FAILED',result.stderr)
            self.assertEqual(result.stdout,'')


class StagingTests(unittest.TestCase):
    def test_staging_starts_held_and_cannot_overwrite_state(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp)
            command=[sys.executable,str(Path(__file__).with_name('prepare.py')),
                     '--handoffs',str(root/'handoffs'),'--deposit',str(root/'deposit')]
            staged=subprocess.run(command,capture_output=True,text=True)
            self.assertEqual(staged.returncode,0,staged.stderr)
            state=root/'handoffs/tb-evidence/tb-state.json'
            original=state.read_bytes()
            self.assertTrue(json.loads(original)['hold'])
            self.assertEqual(json.loads(original)['review']['artifacts'],{})
            duplicate=subprocess.run(command,capture_output=True,text=True)
            self.assertNotEqual(duplicate.returncode,0)
            self.assertEqual(state.read_bytes(),original)
            wrapper=root/'handoffs/tb/tb-operator.sh'
            result=subprocess.run(['bash',str(wrapper)],capture_output=True,text=True)
            self.assertEqual(result.returncode,0,result.stderr)
            self.assertIn('WAITING ON: review seat',result.stdout)
            named=subprocess.run(['bash',str(wrapper),'provision'],capture_output=True,text=True)
            self.assertNotEqual(named.returncode,0)
            self.assertIn('REFUSED: hold',named.stderr)
            after=json.loads(state.read_text())
            self.assertEqual(after['cursor'],0)
            self.assertTrue(after['hold'])
            self.assertEqual(after['review'],json.loads(original)['review'])
            self.assertIn('hold',after['refusals'][-1]['reason'])


if __name__ == '__main__':unittest.main()
