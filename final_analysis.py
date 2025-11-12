#!/usr/bin/env python3
"""
Final analysis of missing jokers in balatro-rs implementation
"""

# Implemented jokers (146 total) - extracted from make_jokers! macro
implemented = {
    'AbstractJoker', 'Acrobat', 'AncientJoker', 'Arrowhead', 'Astronomer', 'Banner',
    'Baron', 'BaseballCard', 'Blackboard', 'Bloodstone', 'BlueJoker', 'Blueprint',
    'Bootstraps', 'Brainstorm', 'Bull', 'Burglar', 'BurntJoker', 'BusinessCard',
    'Campfire', 'Canio', 'CardSharp', 'Cartomancer', 'Castle', 'Cavendish',
    'CeremonialDagger', 'ChaosTheClown', 'Chicot', 'CleverJoker', 'Cloud9',
    'Constellation', 'CraftyJoker', 'CrazyJoker', 'CreditCard', 'DNA',
    'DelayedGratification', 'DeviousJoker', 'DietCola', 'DriverLicense', 'DrollJoker',
    'Drunkard', 'Dusk', 'Egg', 'EightBall', 'Erosion', 'EvenSteven', 'FacelessJoker',
    'Fibonacci', 'FlashCard', 'FlowerPot', 'FortuneTeller', 'FourFingers', 'GlassJoker',
    'GluttonousJoker', 'GoldenJoker', 'GoldenTicket', 'GreedyJoker', 'GreenJoker',
    'GrosMichel', 'Hack', 'HalfJoker', 'HangingChad', 'Hiker', 'HitTheRoad', 'Hologram',
    'IceCream', 'InvisibleJoker', 'JokerStencil', 'JollyJoker', 'Juggler',
    'LoyaltyCard', 'Luchador', 'LuckyCat', 'LustyJoker', 'MadJoker', 'MailInRebate',
    'MarbleJoker', 'Matador', 'MerryAndy', 'MidasMask', 'Mime', 'Misprint', 'MrBones',
    'MysticSummit', 'Obelisk', 'OddTodd', 'OnyxAgate', 'OopsAll6s', 'Pareidolia',
    'Perkeo', 'Photograph', 'Popcorn', 'RaisedFist', 'Ramen', 'RedCard',
    'ReservedParking', 'RideTheBus', 'RiffRaff', 'Rocket', 'RoughGem', 'Runner',
    'Satellite', 'ScaryFace', 'Scholar', 'Seance', 'SeeingDouble', 'Seltzer',
    'ShootTheMoon', 'Shortcut', 'Showman', 'SixthSense', 'SlyJoker', 'SmearedJoker',
    'SmileyFace', 'SockAndBuskin', 'SpaceJoker', 'SpareTrousers', 'Splash',
    'SquareJoker', 'SteelJoker', 'StoneJoker', 'Stuntman', 'Supernova', 'Superposition',
    'Swashbuckler', 'TheDuo', 'TheFamily', 'TheIdol', 'TheJoker', 'TheOrder',
    'TheTribe', 'TheTrio', 'Throwback', 'ToDoList', 'ToTheMoon', 'TradingCard',
    'Triboulet', 'Troubadour', 'TurtleBean', 'Vagabond', 'Vampire', 'WalkieTalkie',
    'WeeJoker', 'WilyJoker', 'WrathfulJoker', 'Yorick', 'ZanyJoker'
}

# All 150 jokers from Balatro (per wiki)
all_150 = {
    'TheJoker', 'GreedyJoker', 'LustyJoker', 'WrathfulJoker', 'GluttonousJoker',
    'JollyJoker', 'ZanyJoker', 'MadJoker', 'CrazyJoker', 'DrollJoker', 'SlyJoker',
    'WilyJoker', 'CleverJoker', 'DeviousJoker', 'CraftyJoker', 'HalfJoker',
    'CreditCard', 'Banner', 'MysticSummit', 'RaisedFist', 'JokerStencil',
    'ChaosTheClown', 'ScaryFace', 'AbstractJoker', 'DelayedGratification',
    'GrosMichel', 'EvenSteven', 'OddTodd', 'Scholar', 'BusinessCard', 'Supernova',
    'RideTheBus', 'Runner', 'IceCream', 'Splash', 'BlueJoker', 'SixthSense',
    'Constellation', 'Hiker', 'GreenJoker', 'Superposition', 'ToDoList', 'Cavendish',
    'RedCard', 'SquareJoker', 'RiffRaff', 'GoldenTicket', 'Swashbuckler', 'SmileyFace',
    'GoldenJoker', 'Drunkard', 'FacelessJoker', 'HangingChad', 'Popcorn', 'WalkieTalkie',
    'ShootTheMoon', 'FortuneTeller', 'Juggler', 'Photograph', 'ReservedParking',
    'MailInRebate', 'EightBall', 'Misprint', 'Egg', 'FourFingers', 'Mime',
    'CeremonialDagger', 'MarbleJoker', 'LoyaltyCard', 'Dusk', 'Fibonacci', 'SteelJoker',
    'Hack', 'Pareidolia', 'SpaceJoker', 'Burglar', 'Blackboard', 'SmearedJoker',
    'Troubadour', 'Certificate', 'Acrobat', 'SpareTrousers', 'TradingCard', 'FlashCard',
    'OnyxAgate', 'Bloodstone', 'Arrowhead', 'RoughGem', 'Seance', 'GlassJoker',
    'Showman', 'FlowerPot', 'MerryAndy', 'StoneJoker', 'MrBones', 'Cloud9', 'Rocket',
    'Bull', 'Luchador', 'DietCola', 'Seltzer', 'Castle', 'SockAndBuskin', 'Hologram',
    'Matador', 'ToTheMoon', 'CardSharp', 'Madness', 'Ramen', 'Throwback', 'MidasMask',
    'Satellite', 'Bootstraps', 'TurtleBean', 'Erosion', 'OopsAll6s', 'TheIdol',
    'SeeingDouble', 'LuckyCat', 'Cartomancer', 'Astronomer', 'Vampire', 'Shortcut',
    'DNA', 'Vagabond', 'Baron', 'Obelisk', 'BaseballCard', 'AncientJoker', 'Campfire',
    'Stuntman', 'InvisibleJoker', 'Blueprint', 'Brainstorm', 'WeeJoker', 'HitTheRoad',
    'TheDuo', 'TheTrio', 'TheFamily', 'TheOrder', 'TheTribe', 'DriverLicense',
    'BurntJoker', 'Canio', 'Triboulet', 'Yorick', 'Chicot', 'Perkeo'
}

# Joker details (name -> rarity, cost, effect)
joker_info = {
    'Certificate': {
        'rarity': 'Uncommon',
        'cost': '$6',
        'effect': 'When round begins, add random playing card with random seal to hand',
        'unlock': 'Start'
    },
    'Madness': {
        'rarity': 'Uncommon',
        'cost': '$7',
        'effect': 'When Small or Big Blind selected, destroy random Joker and create 2 free Jokers',
        'unlock': 'Start'
    }
}

print("=" * 80)
print("FINAL ANALYSIS: MISSING JOKERS IN BALATRO-RS")
print("=" * 80)
print()

print(f"Implemented: {len(implemented)} jokers")
print(f"Target (per wiki): 150 jokers")
print(f"Unique in JOKERS.md: 148 jokers (5 are duplicates)")
print()

missing = all_150 - implemented
print(f"Missing from implementation: {len(missing)} jokers")
print()

if missing:
    print("Missing jokers:")
    for joker in sorted(missing):
        if joker in joker_info:
            info = joker_info[joker]
            print(f"  {joker}")
            print(f"    Rarity: {info['rarity']}")
            print(f"    Cost: {info['cost']}")
            print(f"    Effect: {info['effect']}")
            print(f"    Unlock: {info['unlock']}")
        else:
            print(f"  {joker}")
    print()

print("=" * 80)
print("SUMMARY")
print("=" * 80)
print()
print("Status: 146 of 150 jokers implemented (97.3% complete)")
print(f"To reach 150: Need to implement {len(missing)} joker(s)")
if len(missing) > 0:
    print(f"  - {', '.join(sorted(missing))}")
print()
print("Note: JOKERS.MD has 5 duplicate entries (listed in both Common and Uncommon")
print("sections). The true count is 148 unique jokers, not 150.")
print()
print("Recommended actions:")
print("1. Verify the true count in official Balatro (should be 150)")
print("2. Implement missing joker(s)")
print("3. Fix JOKERS.MD duplicates OR add 2 genuinely new jokers if 150 is correct")
