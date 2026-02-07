import os
import yaml
import json

def fix_file(filepath):
    print(f"Fixing {filepath}...")
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # Only treat the file as having frontmatter if it starts with a YAML delimiter
    if not content.lstrip().startswith('---'):
        return {}

    parts = content.split('---', 2)
    if len(parts) < 3:
        return {}

    fm_raw = parts[1]
    body = parts[2]

    junk_data = {}
    body_lines = body.splitlines()
    if body_lines and body_lines[-1].strip() == '---':
        junk_fields = ['description:', 'globs:', 'alwaysApply:', 'category:']
        is_junk = False
        for i in range(1, 11):
            if len(body_lines) >= i:
                line = body_lines[-i].strip()
                if line == '---' and i > 1:
                    is_junk = True
                    break
                if ':' in line:
                    key = line.split(':', 1)[0].strip()
                    if key in [f.replace(':','') for f in junk_fields]:
                        val = line.split(':', 1)[1].strip()
                        if val:
                            try:
                                junk_data[key] = yaml.safe_load(val)
                            except:
                                junk_data[key] = val

        if is_junk:
            print(f"  Merging junk from end of {filepath}: {junk_data}")
            while body_lines and (body_lines[-1].strip() == '---' or
                                 any(body_lines[-1].strip().startswith(field) for field in junk_fields) or
                                 not body_lines[-1].strip()):
                body_lines.pop()
            body = '\n'.join(body_lines)

    try:
        fm = yaml.safe_load(fm_raw)
    except:
        fm = {}

    if not fm: fm = {}
    for k, v in junk_data.items():
        if k not in fm or fm[k] is None or fm[k] == 'null':
            fm[k] = v
        elif k == 'alwaysApply' and v is True:
            fm[k] = True

    if 'tools' in fm and isinstance(fm['tools'], str) and ',' in fm['tools']:
        fm['tools'] = [t.strip() for t in fm['tools'].split(',')]

    new_fm = yaml.dump(fm, sort_keys=False, default_flow_style=False, allow_unicode=True, width=1000)
    clean_body = body.strip()
    new_content = f"---\n{new_fm}---\n\n{clean_body}\n"

    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(new_content)
    return fm

agents_dir = 'agents'
file_map = {}
for filename in sorted(os.listdir(agents_dir)):
    if filename.endswith('.md'):
        filepath = os.path.join(agents_dir, filename)
        fm = fix_file(filepath)
        if fm:
            file_map[filename] = fm

# Rename architect.md to code-architect.md as per user expectations
if 'architect.md' in file_map:
    print("Renaming architect.md to code-architect.md")
    os.rename('agents/architect.md', 'agents/code-architect.md')
    file_map['code-architect.md'] = file_map.pop('architect.md')

# Update agents.json
json_path = 'agents/agents.json'
if os.path.exists(json_path):
    with open(json_path, 'r') as f:
        data = json.load(f)

    for agent in data.get('agents', []):
        filename = agent.get('file')
        if filename == 'architect.md':
            agent['file'] = 'code-architect.md'
            filename = 'code-architect.md'

        if filename in file_map:
            fm = file_map[filename]
            agent['id'] = fm.get('name', agent['id'])
            agent['category'] = fm.get('category', agent['category'])
            agent['description'] = fm.get('description', agent['description'])

    with open(json_path, 'w') as f:
        json.dump(data, f, indent=2)
