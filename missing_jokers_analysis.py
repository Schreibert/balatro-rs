import re

# Read implemented jokers from joker.rs
with open('core/src/joker.rs', 'r', encoding='utf-8') as f:
    content = f.read()

macro_match = re.search(r'make_jokers!\((.*?)\);', content, re.DOTALL)
if macro_match:
    macro_content = macro_match.group(1)
    implemented = re.findall(r'([A-Z][a-zA-Z0-9]*)', macro_content)
    implemented = [j for j in implemented if j not in ['Missing', 'jokers', 'adding', 'to', 'reach', 'total']]
    implemented_set = {j.lower() for j in implemented}

# Read JOKERS.md
with open('JOKERS.md', 'r', encoding='utf-8') as f:
    lines = f.readlines()

# Build a mapping of original names to details with line numbers
joker_details = {}
current_section = "Unknown"
for i, line in enumerate(lines, 1):
    if '# Common Jokers' in line:
        current_section = "Common"
    elif '# Uncommon Jokers' in line:
        current_section = "Uncommon"
    elif '# Rare Jokers' in line:
        current_section = "Rare"
    elif '# Legendary Jokers' in line:
        current_section = "Legendary"

    match = re.match(r'\|\s*(\d+)\s*\|\s*([^|]+?)\s*\|\s*\$(\d+)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|', line)
    if match:
        num, name, cost, effect, unlock = match.groups()
        name = name.strip()

        if name not in joker_details:
            joker_details[name] = []

        joker_details[name].append({
            'line': i,
            'section': current_section,
            'num': num,
            'cost': cost,
            'effect': effect.strip(),
            'unlock': unlock.strip()
        })

# Find duplicates
duplicates = {name: details for name, details in joker_details.items() if len(details) > 1}

# Convert to CamelCase
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

# Find genuinely missing jokers (not duplicates)
missing_names = []
for name in joker_details.keys():
    camel = to_camel_case(name)
    if camel.lower() not in implemented_set:
        missing_names.append(name)

print("=" * 80)
print("MISSING JOKERS ANALYSIS")
print("=" * 80)
print()
print(f"Total implemented: {len(implemented)}")
print(f"Total unique in JOKERS.md: {len(joker_details)}")
print(f"Duplicates in JOKERS.md: {len(duplicates)}")
print(f"Missing from implementation: {len(missing_names)}")
print()

print("=" * 80)
print("DUPLICATES IN JOKERS.MD (DOCUMENTATION ERROR)")
print("=" * 80)
print()
for name in sorted(duplicates.keys()):
    print(f"{name}:")
    for entry in duplicates[name]:
        print(f"  - Line {entry['line']}: {entry['section']} section, #{entry['num']}, ${entry['cost']}")
    print()

print("=" * 80)
print("GENUINELY MISSING JOKERS FROM IMPLEMENTATION")
print("=" * 80)
print()
for name in sorted(missing_names):
    camel = to_camel_case(name)
    details = joker_details[name][0]  # Take first occurrence
    print(f"Name: {name}")
    print(f"  CamelCase: {camel}")
    print(f"  Rarity: {details['section']}")
    print(f"  Cost: ${details['cost']}")
    print(f"  Effect: {details['effect']}")
    print(f"  Unlock: {details['unlock']}")
    print()

print("=" * 80)
print("SUMMARY")
print("=" * 80)
print()
print(f"JOKERS.MD declares 150 total jokers but has documentation errors:")
print(f"  - 153 total table entries")
print(f"  - 148 unique jokers")
print(f"  - 5 duplicates (listed in both Common and Uncommon sections)")
print()
print(f"Implementation status:")
print(f"  - 147 jokers implemented")
print(f"  - {len(missing_names)} jokers missing: {', '.join(sorted(missing_names))}")
print()
print(f"To reach true 150 total:")
print(f"  - Fix JOKERS.MD duplicates (determine correct rarity for 5 jokers)")
print(f"  - Add {len(missing_names)} missing joker(s)")
print(f"  - Add 3 more new jokers (if aiming for 150 total unique)")
