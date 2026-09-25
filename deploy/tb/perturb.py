#!/usr/bin/env python3
"""Remove and invert each named guard in disposable copies; never edit the subject."""
import ast
import json
import os
import signal
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


def guards(path):
    return [(node.lineno, node.args[0].value) for node in ast.walk(ast.parse(path.read_text()))
            if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id in ['check','need']
            and len(node.args) == 2 and isinstance(node.args[0], ast.Constant)]


def suite(target):
    child=subprocess.Popen([sys.executable,'-B','-m','unittest','test_tb'],cwd=target,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,start_new_session=True,env=dict(os.environ,PYTHONDONTWRITEBYTECODE='1'))
    try:
        out,err=child.communicate(timeout=5)
        return subprocess.CompletedProcess(child.args,child.returncode,out,err)
    except subprocess.TimeoutExpired:
        os.killpg(child.pid,signal.SIGKILL)
        out,err=child.communicate()
        return subprocess.CompletedProcess(child.args,124,out,err+'\nTIMEOUT: mutation prevented bounded completion')


def main():
    root=Path(__file__).resolve().parent
    records=[]
    with tempfile.TemporaryDirectory(prefix='tb-mutations-') as temporary:
        target=Path(temporary)
        for p in root.glob('*'):
            if p.suffix in ['.py','.sh']:shutil.copy2(p,target/p.name)
        # A kill is a failure the mutation caused. Against a suite that already
        # fails, every mutation reads as killed, so the unmodified suite must pass.
        base=suite(target)
        if base.returncode!=0:
            print('BASELINE FAILED: the unmodified suite does not pass; no mutation was run\n'+base.stderr[-1800:],file=sys.stderr)
            return 2
        for name in ['tb_order.py','tb_payload.py','tb_driver.py']:
            source=root/name
            for line, label in guards(source):
                for mode in ['removed','inverted']:
                    shutil.rmtree(target/'__pycache__',ignore_errors=True)
                    tree=ast.parse(source.read_text())
                    for node in ast.walk(tree):
                        if isinstance(node,ast.Call) and node.lineno==line and isinstance(node.func,ast.Name) and node.func.id in ['check','need']:
                            node.args[1]=ast.Constant(value=True) if mode=='removed' else ast.UnaryOp(op=ast.Not(),operand=node.args[1])
                    (target/name).write_text(ast.unparse(ast.fix_missing_locations(tree))+'\n')
                    result=suite(target)
                    records.append(dict(file=name,line=line,guard=label,mode=mode,killed=result.returncode!=0,
                                        evidence=result.stderr[-1800:]))
                    shutil.copy2(source,target/name)
        print(json.dumps(records,indent=2))
    return int(any(not r['killed'] for r in records))


if __name__=='__main__':sys.exit(main())
