#!/usr/bin/env python3
"""Host-only tests. All host mutations/admin/GPU calls use stub boundaries."""
import contextlib
import copy
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import prepare
import sections
import tb_driver as driver
import tb_order as order
import tb_payload as payload


class Fixture(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.plan = prepare.template(self.root, 'todd', 1000)
        self.plan['rulings'] = {k: '#679/ruling' for k in self.plan['rulings']}
        self.source = self.root / 'source.ndjson'
        self.source.write_text('{}\n')
        self.plan['files'] = {str(self.source): order.sha(self.source)}
        self.plan['arms'][1]['jobs'] = [dict(id=cell, source_cell=cell, kind='refeed', stack='B1',
                                            source_trace=str(self.source), source_run='r') for cell in ['ampere', 'ada']]
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
            p['files']={str(self.root/'unreviewed'): 'x'} if field=='absent' else {str(self.source):'wrong'}
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
               ('order',lambda p:p['arms'][1]['jobs'][0].update(source_trace=None)),
               ('device',lambda p:p['arms'][1].update(jobs=[])),
               ('kernel',lambda p:p['arms'][2].update(jobs=[]))]
        for name, edit in edits:
            p=copy.deepcopy(self.plan);edit(p)
            with self.subTest(name=name),self.assertRaises(order.Refused):order.validate_plan(p)

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

    def test_wait_succeeds_only_with_receipt_and_own_live_lease(self):
        self.due('measure:B1-s451234785645-n1')
        self.o.wait('measure:B1-s451234785645-n1',timeout=0)
        self.state['driver']['ticks']='dead';self.save()
        with self.assertRaises(order.Refused):self.o.wait('measure:B1-s451234785645-n1',timeout=0)
        self.due('load:B1-s451234785645-n1')
        with self.assertRaises(order.Refused):self.o.wait('measure:B1-s451234785645-n1',timeout=0)
        self.due('start:TB0')
        with self.assertRaisesRegex(order.Refused,'wait-order'):self.o.wait('measure:B1-s451234785645-n1',timeout=0)

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

    def test_private_payload_closed_stdin_cleanup(self):
        # argv is sudo,python,-I,FILE,PLAN,HASH,STEP.
        def inspect(argv,**kw):
            self.assertIs(kw['stdin'],subprocess.DEVNULL)
            file=Path(argv[3]);self.assertEqual(file.stat().st_mode&0o777,0o600)
            self.assertEqual(order.sha(file),self.state['review']['artifacts'][str(Path(__file__).with_name('tb_payload.py').resolve())])
            self.assertEqual(argv[:3],['sudo','/usr/bin/python3','-I'])
            return subprocess.CompletedProcess(argv,0)
        with patch('tb_order.subprocess.run',side_effect=inspect):self.assertEqual(order.payload(self.state,'provision',self.root/'log'),0)
        self.assertEqual(list(self.root.glob('.payload-*')),[])
        self.state['review']['artifacts'][str(Path(__file__).with_name('tb_payload.py').resolve())]='bad'
        with self.assertRaises(order.Refused):order.payload(self.state,'provision',self.root/'log')

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
        return dict(output_tokens=[1],entropies=[0.5],field={'20':dict(ranked=[{'token':i,'probability':.005} for i in range(200)],realized=1)})

    def test_exact_rejects_empty_partial_and_changed(self):
        a=self.rec();b=copy.deepcopy(a)
        self.assertTrue(driver.exact(a,b))
        b['field']={20:b['field']['20']};self.assertTrue(driver.exact(a,b))
        b['entropies']=[.5000000000000001];self.assertFalse(driver.exact(a,b))
        b=copy.deepcopy(a);b['field']['20']['ranked'].pop()
        with self.assertRaises(order.Refused):driver.exact(a,b)
        b=copy.deepcopy(a);b['entropies']=[]
        with self.assertRaises(order.Refused):driver.exact(a,b)
        self.assertNotEqual(driver.float_bits([0.0]),driver.float_bits([-0.0]))

    def test_trace_close_and_missing(self):
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)/'trace';p.write_text('{"kind":"turn.closed"}\n')
            self.assertEqual(driver.until_closed(p,'turn.closed',1),[{'kind':'turn.closed'}])
            with self.assertRaises(order.Refused):driver.until_closed(p,'replay.closed',0)

    def test_section_comparison_detects_instruction_change(self):
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)/'a';q=Path(tmp)/'b'
            section=dict(name='.text',sha256='a',executable=True)
            manifest=dict(hosts={'lib':dict(sections=[section])},members={'cubin':{'lib.1.sm_120a.cubin':dict(size=4,sha256='a',sections=[section])},'ptx':{'lib.1.sm_75.ptx':dict(size=4,sha256='a')}})
            p.write_text(json.dumps(manifest));q.write_text(json.dumps(manifest))
            self.assertTrue(sections.compare(p,q)['executable_identity'])
            changed=copy.deepcopy(manifest);changed['members']['cubin']['lib.1.sm_120a.cubin']['sections'][0]['sha256']='b'
            q.write_text(json.dumps(changed));r=sections.compare(p,q)
            self.assertFalse(r['executable_identity']);self.assertEqual(r['cuda']['cubin']['sm_120a']['code_equal'],0)


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
        probe('LoadState=not-found\nActiveState=inactive\n')
        for out,rc,door,procs in [('LoadState=loaded\nActiveState=active\n',0,False,[]),('LoadState=loaded\nActiveState=inactive\n',1,False,[]),('LoadState=loaded\nActiveState=inactive\n',0,True,[]),('LoadState=loaded\nActiveState=inactive\n',0,False,[SimpleNamespace(name='42',stat=lambda:SimpleNamespace(st_uid=1000))])]:
            with self.assertRaises(RuntimeError):probe(out,rc,door,procs)

    def test_admin_reply_not_merely_exit_zero(self):
        def cp(state,kind='state'):return subprocess.CompletedProcess([],0,json.dumps(dict(kind=kind,state=state))+'\n','')
        with patch('tb_payload.run',return_value=cp('unloaded')),contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(payload.answer('B1','show','unloaded')['state'],'unloaded')
        with patch('tb_payload.run',return_value=cp('idle')),self.assertRaises(RuntimeError):payload.answer('B1','show','unloaded')

    def test_save_refuses_symlink(self):
        p=self.base/'file';payload.save(p,'a');self.assertEqual(p.read_text(),'a')
        link=self.base/'link';link.symlink_to(p)
        with self.assertRaises(RuntimeError):payload.save(link,'b')
        self.assertEqual(p.read_text(),'a')

    def provision(self):
        with patch('tb_payload.pwd.getpwnam',side_effect=KeyError),patch('tb_payload.grp.getgrnam',return_value=SimpleNamespace(gr_gid=os.getgid())),patch('tb_payload.os.chown'),patch('tb_payload.run'),contextlib.redirect_stdout(io.StringIO()):payload.provision(self.plan)

    def test_provision_and_installed_hash_checks(self):
        self.stacks();self.provision()
        self.assertTrue((self.root/'config/B1/allow-list').is_file())
        self.assertEqual((self.root/'config/B1/allow-list').read_text(),'bravo\n')
        payload.installed(self.plan)
        file=self.root/'stacks/B1/bin/pyworker';file.write_text('changed')
        with self.assertRaises(RuntimeError):payload.installed(self.plan)
        file.write_text('stub');self.model.write_text('changed')
        with self.assertRaises(RuntimeError):payload.installed(self.plan)
        self.model.write_text('weights');(self.root/'plan-sha256').write_text('changed')
        with self.assertRaises(RuntimeError):payload.installed(self.plan)
        with self.assertRaises(RuntimeError):self.provision()

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
        with patch('tb_payload.pwd.getpwnam',return_value=self.user),self.assertRaises(RuntimeError):payload.provision(self.plan)

    def setup_load(self):
        self.root.mkdir();(self.root/'sinks').mkdir();(self.root/'agents/B1').mkdir(parents=True)
        return dict(id='job',kind='free',stack='B1',seed=7)

    def invoke_load(self,job,gpu=None,ldd=None,loader_rc=0,door=True,artifact=True):
        default_gpu='0, NVIDIA RTX PRO 5000 Blackwell Laptop GPU, 615.71.09'
        default_ldd='\n'.join(f'{lib} => {self.root}/stacks/B1/cuda-lib/{lib}' for lib in ['libcudart.so','libcublas.so','libcublasLt.so'])
        def run(argv,**kwargs):
            if 'nvidia-smi' in argv[0]:return subprocess.CompletedProcess(argv,0,default_gpu if gpu is None else gpu,'')
            if 'ldd' in argv[0]:return subprocess.CompletedProcess(argv,0,default_ldd if ldd is None else ldd,'')
            if 'derive' in argv:
                Path(argv[-1]).write_text(f'artifact: {self.model}\n' if artifact else 'artifact: wrong\n')
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
        for gpu,ldd in [('1, Wrong GPU, 615.71.09',None),(None,'libcublas.so => /opt/cuda/lib.so'),(None,'\n'.join(f'{lib} => {self.root}/stacks/B1/cuda-lib/{lib}' for lib in ['libcudart.so','libcublas.so','libcublasLt.so'])+'\nlibother.so => not found')]:
            with self.assertRaises(RuntimeError):self.invoke_load(job,gpu=gpu,ldd=ldd)
        self.invoke_load(job)
        self.assertIn('seed: 7',(self.root/'agents/B1/bravo.yaml').read_text())

    def test_replay_load_orders_preload_before_wait(self):
        job=self.setup_load();source=self.base/'source';source.write_text('{}')
        job.update(kind='refeed',source_trace=str(source));self.plan['files'][str(source)]=order.sha(source)
        self.invoke_load(job)
        self.assertTrue((self.root/'sinks/job/load.log').exists())

    def test_replay_refusals(self):
        job=self.setup_load();source=self.base/'source';source.write_text('{}')
        job.update(kind='refeed',source_trace=str(source));self.plan['files'][str(source)]=order.sha(source)
        for setting in [dict(artifact=False),dict(door=False),dict(loader_rc=1)]:
            with self.assertRaises(RuntimeError):self.invoke_load(job,**setting)
            import shutil
            shutil.rmtree(self.root/'sinks/job')
        self.plan['files'][str(source)]='wrong'
        with self.assertRaises(RuntimeError):self.invoke_load(job)

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


class DriverTests(unittest.TestCase):
    save = Fixture.save

    def setUp(self):
        Fixture.setUp(self)
        self.plan['install_root']=str(self.root/'installed')
        self.plan['deposit']=str(self.root/'deposit');Path(self.plan['deposit']).mkdir()
        self.free=ReadingTests().rec()
        self.free.update(emission='essay',declared_seed=7,weights_hash=self.plan['tuple']['weights_sha256'],input_tokens=10)
        self.probe=SimpleNamespace(base=SimpleNamespace(gate_turn=lambda *a,**kw:dict(kind='answered',run='r')),
                                   ESSAY_PROMPT='fixed',extract_run=lambda rows:copy.deepcopy(self.free),
                                   measured_events=lambda rows,run:rows,reading_two=lambda a,b:dict(positions_compared=1),
                                   reading_one=lambda a,b:{},first_divergence=lambda a,b:0)

    def job(self,kind='free'):
        return dict(id='job',kind=kind,stack='B1',seed=7,source_trace=str(self.source),source_run='r')

    def close(self,replay=False):
        if replay:return [dict(kind='replay.closed',run='r',payload=dict(outcome=dict(kind='certified')))]
        return [dict(kind='model.measurement',run='r'),dict(kind='turn.closed',run='r')]

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
            if fault=='answer':self.probe.base.gate_turn=lambda *a,**k:dict(kind='refused')
            with patch('tb_driver.until_closed',return_value=rows),self.assertRaises(order.Refused):driver.measure(self.plan,j,self.probe)
            self.probe.base.gate_turn=gate;self.free=old;self.cleanup_job()

    def test_refeed_requires_close_measurement_and_source(self):
        j=self.job('refeed')
        self.source.write_text(json.dumps(dict(kind='model.measurement',run='r'))+'\n');self.plan['files'][str(self.source)]=order.sha(self.source)
        with patch('tb_driver.until_closed',return_value=self.close(True)):
            result=driver.measure(self.plan,j,self.probe)
            self.assertTrue(json.loads(result.read_text())['exact'])
        self.cleanup_job()
        for fault in ['close','outcome','measurement','source-hash','source-measurement']:
            rows=self.close(True)
            if fault=='close':rows=rows*2
            if fault=='outcome':rows[0]['payload']['outcome']['kind']='refused'
            if fault=='source-hash':self.plan['files'][str(self.source)]='bad'
            if fault=='source-measurement':self.source.write_text('{"run":"r","kind":"other"}\n');self.plan['files'][str(self.source)]=order.sha(self.source)
            self.probe.measured_events=(lambda rows,run:None) if fault=='measurement' else (lambda rows,run:rows)
            with patch('tb_driver.until_closed',return_value=rows),self.assertRaises((order.Refused,KeyError)):driver.measure(self.plan,j,self.probe)
            self.cleanup_job();self.source.write_text(json.dumps(dict(kind='model.measurement',run='r'))+'\n');self.plan['files'][str(self.source)]=order.sha(self.source)

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

    def test_driver_settles_before_next_load_and_stops_falsifier(self):
        # Small schedule exercises the blocking orchestration; full count and
        # source schedule are independently guarded by validate_plan tests.
        arm=dict(name='TB0',jobs=[dict(id='a',kind='free',seed=7),dict(id='b',kind='free',seed=7),dict(id='c',kind='refeed')])
        self.plan['arms']=[arm]
        state=dict(plan=str(self.planpath));seen=[]
        o=SimpleNamespace(read=lambda:state,approved=lambda s:self.plan,
                          coding=lambda step,path:seen.append(step),wait=lambda step:seen.append('wait:'+step))
        def measure(plan,job,probe):
            p=Path(plan['deposit'])/'runs'/job['id']/'run.json';p.parent.mkdir(parents=True,exist_ok=True)
            order.atomic(p,dict(self.free,exact=False,replay_outcome='diverged'));return p
        with patch('tb_driver.readers',return_value=self.probe),patch('tb_driver.notice'),patch('tb_driver.measure',side_effect=measure),self.assertRaises(order.Refused):driver.drive(o,'TB0')
        self.assertIn('settle:b',seen);self.assertIn('wait:settle:c',seen);self.assertNotIn('settle:c',seen)
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

    def test_payload_mode_and_copy_tampering(self):
        real_temp=tempfile.mkstemp
        def public(*args,**kw):
            fd,name=real_temp(*args,**kw);os.chmod(name,0o644);return fd,name
        with patch('tb_order.tempfile.mkstemp',side_effect=public),patch('tb_order.subprocess.run',return_value=subprocess.CompletedProcess([],0)),self.assertRaises(order.Refused):
            order.payload(self.state,'provision',self.root/'log')
        real_sha=order.sha
        def tampered(path):return 'bad' if Path(path).name.startswith('.payload-') else real_sha(path)
        with patch('tb_order.sha',side_effect=tampered),patch('tb_order.subprocess.run',return_value=subprocess.CompletedProcess([],0)),self.assertRaises(order.Refused):
            order.payload(self.state,'provision',self.root/'log')


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
