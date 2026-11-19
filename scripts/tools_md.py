"""Generate docs/tools.md from tools.json (the tool list mcp.sanecode.co serves).

    python scripts/tools_md.py
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PRO = {'sanecode_set_failover', 'sanecode_schedule_continue', 'sanecode_send_prompt'}


def cell(text):
    return str(text).replace('|', r'\|').replace('\n', ' ')


def type_of(spec):
    t = spec.get('type', '')
    if t == 'array':
        t = f"array of {spec.get('items', {}).get('type', '')}".strip()
    extra = []
    if 'enum' in spec:
        extra.append('one of ' + ', '.join(f'`{v}`' for v in spec['enum']))
    for key, label in (('minimum', 'min'), ('maximum', 'max'), ('minItems', 'min items'), ('maxItems', 'max items')):
        if key in spec:
            extra.append(f'{label} {spec[key]}')
    return t + (f" ({'; '.join(extra)})" if extra else '')


def main():
    data = json.loads((ROOT / 'tools.json').read_text(encoding='utf-8'))
    out = [
        '# SaneCode MCP tools',
        '',
        f"Generated from [`tools.json`](../tools.json) (version {data['version']}) by "
        '`python scripts/tools_md.py`. Do not edit by hand.',
        '',
        'Plan is what the tool needs inside your Claude accounts. Every tool used from outside apps '
        'through `https://mcp.sanecode.co/mcp` needs SaneCode Pro.',
        '',
        '## Server instructions',
        '',
        'Sent to the client on `initialize`:',
        '',
        *('> ' + line if line else '>' for line in data['instructions'].split('\n')),
        '',
        '## Tools',
        '',
    ]
    for t in data['tools']:
        out.append(f"- [`{t['name']}`](#{t['name']}): {t.get('title', '')}")
    out.append('')
    for t in data['tools']:
        out += [f"### {t['name']}", '', f"**{t.get('title', '')}**", '']
        out += [f"Plan: {'Pro' if t['name'] in PRO else 'Free'}", '']
        out += [t['description'], '']
        schema = t.get('inputSchema', {})
        props = schema.get('properties', {})
        required = set(schema.get('required', []))
        if props:
            out += ['| Parameter | Type | Required | Description |', '| --- | --- | --- | --- |']
            for name, spec in props.items():
                req = 'yes' if name in required else 'no'
                out.append(f"| `{name}` | {cell(type_of(spec))} | {req} | {cell(spec.get('description', ''))} |")
        else:
            out.append('No parameters.')
        out.append('')
        ann = t.get('annotations', {})
        if ann:
            out.append('Annotations: ' + ', '.join(f'`{k}: {json.dumps(v)}`' for k, v in ann.items()))
            out.append('')
    (ROOT / 'docs' / 'tools.md').write_text('\n'.join(out).rstrip() + '\n', encoding='utf-8', newline='\n')
    print('wrote docs/tools.md')


if __name__ == '__main__':
    main()
