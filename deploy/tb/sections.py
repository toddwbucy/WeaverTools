#!/usr/bin/env python3
"""Inventory ELF sections and extracted CUDA members without loading libraries."""
import argparse
import collections
import hashlib
import json
import pathlib
import struct


def digest(data):
    return hashlib.sha256(data).hexdigest()


def elf_sections(path):
    data = path.read_bytes()
    if data[:6] != b'\x7fELF\x02\x01':
        raise ValueError(f'{path}: expected ELF64 little-endian')
    header = struct.unpack_from('<16sHHIQQQIHHHHHH', data)
    offset, size, count, string_index = header[6], header[11], header[12], header[13]
    def section(index):
        return struct.unpack_from('<IIQQQQIIQQ', data, offset + size * index)
    if count == 0:
        count = section(0)[5]
    if string_index == 0xffff:
        string_index = section(0)[6]
    strings_header = section(string_index)
    strings = data[strings_header[4]:strings_header[4] + strings_header[5]]
    result = []
    for index in range(1, count):
        name_offset, kind, flags, address, start, length, link, info, align, entry_size = section(index)
        name = strings[name_offset:strings.index(b'\0', name_offset)].decode()
        payload = None if kind == 8 else data[start:start + length]
        if payload is not None and len(payload) != length:
            raise ValueError(f'{path}: truncated section {name}')
        result.append(dict(name=name, type=kind, flags=flags, executable=bool(flags & 4),
                           allocated=bool(flags & 2), size=length,
                           sha256=None if payload is None else digest(payload)))
    return dict(file_sha256=digest(data), sections=result)


def inventory():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('deposit', type=pathlib.Path)
    parser.add_argument('stack', choices=['B1', 'B2'])
    args = parser.parse_args()
    root = args.deposit
    stack = root / 'stacks' / args.stack
    output = root / 'sections' / args.stack
    hosts = {}
    for folder in ['bin', 'engine-lib']:
        for path in sorted((stack / folder).rglob('*')):
            if path.is_file() and not path.is_symlink():
                with path.open('rb') as stream:
                    magic = stream.read(4)
                if magic == b'\x7fELF':
                    hosts[str(path.relative_to(stack))] = elf_sections(path)
    members = {}
    for kind in ['cubin', 'ptx']:
        files = sorted((output / kind).glob('*'))
        members[kind] = {path.name: dict(size=path.stat().st_size, sha256=digest(path.read_bytes()))
                         for path in files if path.is_file()}
        if kind == 'cubin':
            for path in files:
                if path.is_file():
                    members[kind][path.name]['sections'] = elf_sections(path)['sections']
        if not members[kind]:
            raise ValueError(f'No {kind} extracted for {args.stack}')
    if not hosts:
        raise ValueError('No host ELF files found')
    result = dict(stack=args.stack, hosts=hosts, members=members,
                  scope='All host ELF sections; every extracted cubin and PTX member. No GPU code executed.')
    destination = output / 'section-manifest.json'
    destination.write_text(json.dumps(result, indent=2) + '\n')
    architectures = collections.Counter(name.split('.')[-2] for name in members['cubin'])
    print(json.dumps(dict(manifest=str(destination), host_elfs=len(hosts),
                          cubins=len(members['cubin']), ptx=len(members['ptx']),
                          architectures=dict(architectures))))


def compare(first, second):
    a, b = [json.loads(pathlib.Path(p).read_text()) for p in [first, second]]
    report = {"host": {}, "cuda": {}}
    for name in sorted(a['hosts'].keys() | b['hosts'].keys()):
        sa = {s['name']: s for s in a['hosts'].get(name, {}).get('sections', [])}
        sb = {s['name']: s for s in b['hosts'].get(name, {}).get('sections', [])}
        changes = []
        for section in sorted(sa.keys() | sb.keys()):
            if sa.get(section) != sb.get(section):
                changes.append(dict(section=section, B1=sa.get(section), B2=sb.get(section)))
        report['host'][name] = dict(text_equal=sa.get('.text') == sb.get('.text') and '.text' in sa,
                                   changes=changes)
    for kind in ['cubin', 'ptx']:
        ma, mb = a['members'][kind], b['members'][kind]
        groups = {}
        for name in sorted(ma.keys() | mb.keys()):
            arch = name.split('.')[-2]
            group = groups.setdefault(arch, dict(equal=0, code_equal=0, changed=[], missing=[]))
            if name not in ma or name not in mb:
                group['missing'].append(name)
            elif ma[name] == mb[name]:
                group['equal'] += 1
            else:
                group['changed'].append(name)
            if kind == 'cubin' and name in ma and name in mb:
                ca = [s for s in ma[name]['sections'] if s['executable']]
                cb = [s for s in mb[name]['sections'] if s['executable']]
                if ca and ca == cb:
                    group['code_equal'] += 1
        report['cuda'][kind] = groups
    # Conservative: relocations/read-only data also affect execution. Never
    # conclude held identity from .text alone or hide missing members.
    report['executable_identity'] = (not any(h['changes'] for h in report['host'].values()) and
        all(not g['changed'] and not g['missing'] for groups in report['cuda'].values() for g in groups.values()))
    return report


if __name__ == '__main__':
    import sys
    if len(sys.argv) == 5 and sys.argv[1] == 'compare':
        report = compare(sys.argv[2], sys.argv[3])
        pathlib.Path(sys.argv[4]).write_text(json.dumps(report, indent=2) + '\n')
        print(json.dumps(dict(host_text_equal=sum(h['text_equal'] for h in report['host'].values()),
                             host_files=len(report['host']), cuda={k:{arch:dict(equal=g['equal'], code_equal=g['code_equal'], changed=len(g['changed']), missing=len(g['missing'])) for arch,g in v.items()} for k,v in report['cuda'].items()},
                             executable_identity=report['executable_identity'])))
    else:
        inventory()
