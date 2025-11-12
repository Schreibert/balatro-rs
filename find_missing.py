import re

# Read implemented jokers from joker.rs
with open('core/src/joker.rs', 'r', encoding='utf-8') as f:
    content = f.read()

macro_match = re.search(r'make_jokers!\((.*?)\);', content, re.DOTALL)
if macro_match:
    macro_content = macro_match.group(1)
    implemented = re.findall(r'([A-Z][a-zA-Z0-9]*)', macro_content)
    implemented = [j for j in implemented if j not in ['Missing', 'jokers', 'adding', 'to', 'reach', 'total']]
    implemented_lower = {j.lower(): j for j in implemented}

# Read JOKERS.md
with open('JOKERS.md', 'r', encoding='utf-8') as f:
    lines = f.readlines()

# Build a mapping of original names to details
joker_details = {}
for i, line in enumerate(lines):
    match = re.match(r'\|\s*(\d+)\s*\|\s*([^|]+?)\s*\|\s*\$(\d+)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|', line)
    if match:
        num, name, cost, effect, unlock = match.groups()
        name = name.strip()
        joker_details[name] = {
            'num': num,
            'cost': cost,
            'effect': effect.strip(),
            'unlock': unlock.strip()
        }

# Convert to CamelCase with full mapping
def to_camel_case(name):
    if name == 'Joker':
        return 'TheJoker'
    if name == 'Oops! All 6s':
        return 'OopsAll6s'
    if name == '8 Ball':
        return 'EightBall'
    if name == 'Mail-In Rebate':
        return 'MailInRebate'
    if name == 'Riff-Raff':
        return 'RiffRaff'
    if name == 'Walkie Talkie':
        return 'WalkieTalkie'
    if name == 'Séance':
        return 'Seance'
    if name == 'Sock and Buskin':
        return 'SockAndBuskin'
    if name == "Driver's License":
        return 'DriverLicense'

    name = name.replace("'", '').replace('!', '').replace('-', ' ')
    words = name.split()
    return ''.join(word.capitalize() for word in words)

# Find missing
spec_to_impl = {}
missing_names = []

for orig_name in joker_details.keys():
    camel = to_camel_case(orig_name)
    if camel.lower() not in implemented_lower:
        missing_names.append(orig_name)
    spec_to_impl[orig_name] = camel

print(f'Total in JOKERS.md: {len(joker_details)}')
print(f'Total implemented: {len(implemented)}')
print(f'Missing: {len(missing_names)}')
print()
print('Missing jokers with details:')
print('=' * 80)
for name in sorted(missing_names):
    details = joker_details[name]
    camel = to_camel_case(name)
    print(f'Name: {name} (CamelCase: {camel})')
    print(f'  Cost: ${details["cost"]}')
    print(f'  Effect: {details["effect"]}')
    print(f'  Unlock: {details["unlock"]}')
    print()
