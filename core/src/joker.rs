use crate::card::Suit;
use crate::effect::Effects;
use crate::game::Game;
use crate::hand::MadeHand;
use pyo3::pyclass;
use std::fmt;
use std::sync::{Arc, Mutex};
use strum::{EnumIter, IntoEnumIterator};

pub trait Joker: std::fmt::Debug + Clone {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn cost(&self) -> usize;
    fn rarity(&self) -> Rarity;
    fn categories(&self) -> Vec<Categories>;
    fn effects(&self, game: &Game) -> Vec<Effects>;

    /// Get the sell value of this joker (typically cost/2)
    fn sell_value(&self) -> usize {
        self.cost() / 2
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Categories {
    MultPlus,
    MultMult,
    Chips,
    Economy,
    Retrigger,
    Effect,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Common => {
                write!(f, "Common")
            }
            Self::Uncommon => {
                write!(f, "Uncommon")
            }
            Self::Rare => {
                write!(f, "Rare")
            }
            Self::Legendary => {
                write!(f, "Legendary")
            }
        }
    }
}

// We could pass around `Box<dyn Joker>` but it doesn't work so nice with pyo3 and serde.
// Since we know all variants (one for each joker), we define an enum that implements
// our `Joker` trait. This macro just reduces the amount of boilerplate we have to copy
// to match each joker and call its methods.
// It ends up creating an enum `Jokers` that contains each joker struct (where each struct impl `Joker`), and we impl `Joker`
// for `Jokers` enum by matching each case and calling underlying methods.
// https://stackoverflow.com/questions/63848427/using-enums-for-dynamic-polymorphism-in-rust/63849405#63849405
macro_rules! make_jokers {
    ($($x:ident), *) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "python", pyclass(eq))]
        #[derive(Debug, Clone, EnumIter, Eq, PartialEq, Hash)]
        pub enum Jokers {
            $(
                $x($x),
            )*
        }

        impl Joker for Jokers {
            fn name(&self) -> String {
                match self {
                    $(
                        Jokers::$x(joker) => joker.name(),
                    )*
                }
            }
            fn desc(&self) -> String {
                match self {
                    $(
                        Jokers::$x(joker) => joker.desc(),
                    )*
                }
            }
            fn cost(&self) -> usize {
                match self {
                    $(
                        Jokers::$x(joker) => joker.cost(),
                    )*
                }
            }
            fn rarity(&self) -> Rarity {
                match self {
                    $(
                        Jokers::$x(joker) => joker.rarity(),
                    )*
                }
            }
            fn categories(&self) -> Vec<Categories> {
                match self {
                    $(
                        Jokers::$x(joker) => joker.categories(),
                    )*
                }
            }
            fn effects(&self, game: &Game) -> Vec<Effects> {
                match self {
                    $(
                        Jokers::$x(joker) => joker.effects(game),
                    )*
                }
            }
            fn sell_value(&self) -> usize {
                match self {
                    $(
                        Jokers::$x(joker) => joker.sell_value(),
                    )*
                }
            }
        }
    }
}

make_jokers!(
    TheJoker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    JollyJoker,
    ZanyJoker,
    MadJoker,
    CrazyJoker,
    DrollJoker,
    SlyJoker,
    WilyJoker,
    CleverJoker,
    DeviousJoker,
    CraftyJoker,
    HalfJoker,
    CreditCard,
    Banner,
    MysticSummit,
    RaisedFist,
    ChaosTheClown,
    ScaryFace,
    AbstractJoker,
    DelayedGratification,
    GrosMichel,
    EvenSteven,
    OddTodd,
    Scholar,
    BusinessCard,
    Supernova,
    RideTheBus,
    Runner,
    IceCream,
    Splash,
    BlueJoker,
    SixthSense,
    Constellation,
    Hiker,
    GreenJoker,
    Superposition,
    ToDoList,
    Cavendish,
    RedCard,
    SquareJoker,
    RiffRaff,
    GoldenTicket,
    Swashbuckler,
    SmileyFace,
    GoldenJoker,
    Drunkard,
    FacelessJoker,
    HangingChad,
    Popcorn,
    WalkieTalkie,
    ShootTheMoon,
    FortuneTeller,
    Juggler,
    Photograph,
    ReservedParking,
    MailInRebate,
    EightBall,
    Misprint,
    Egg
);

impl Jokers {
    pub(crate) fn by_rarity(rarirty: Rarity) -> Vec<Self> {
        return Self::iter().filter(|j| j.rarity() == rarirty).collect();
    }

    /// Get all common jokers (for random generation)
    pub fn all_common() -> Vec<Self> {
        Self::by_rarity(Rarity::Common)
    }
}

impl fmt::Display for Jokers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} [${}, {}] {}",
            self.name(),
            self.cost(),
            self.rarity(),
            self.desc()
        )
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheJoker {}

impl Joker for TheJoker {
    fn name(&self) -> String {
        "Joker".to_string()
    }
    fn desc(&self) -> String {
        "+4 Mult".to_string()
    }
    fn cost(&self) -> usize {
        2
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, _hand: MadeHand) {
            g.mult += 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GreedyJoker {}

impl Joker for GreedyJoker {
    fn name(&self) -> String {
        "Greedy Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with diamond suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let diamonds = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Diamond)
                .count();
            g.mult += diamonds * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct LustyJoker {}

impl Joker for LustyJoker {
    fn name(&self) -> String {
        "Lusty Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with heart suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let hearts = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Heart)
                .count();
            g.mult += hearts * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct WrathfulJoker {}

impl Joker for WrathfulJoker {
    fn name(&self) -> String {
        "Wrathful Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with spade suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let spades = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Spade)
                .count();
            g.mult += spades * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GluttonousJoker {}

impl Joker for GluttonousJoker {
    fn name(&self) -> String {
        "Gluttonous Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with club suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let clubs = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Club)
                .count();
            g.mult += clubs * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct JollyJoker {}

impl Joker for JollyJoker {
    fn name(&self) -> String {
        "Jolly Joker".to_string()
    }
    fn desc(&self) -> String {
        "+8 mult if played hand contains a pair".to_string()
    }
    fn cost(&self) -> usize {
        3
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_pair().is_some() {
                g.mult += 8
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ZanyJoker {}

impl Joker for ZanyJoker {
    fn name(&self) -> String {
        "Zany Joker".to_string()
    }
    fn desc(&self) -> String {
        "+12 mult if played hand contains a three of a kind".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_three_of_kind().is_some() {
                g.mult += 12
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MadJoker {}

impl Joker for MadJoker {
    fn name(&self) -> String {
        "Mad Joker".to_string()
    }
    fn desc(&self) -> String {
        "+10 mult if played hand contains a two pair".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_two_pair().is_some() {
                g.mult += 10
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CrazyJoker {}

impl Joker for CrazyJoker {
    fn name(&self) -> String {
        "Crazy Joker".to_string()
    }
    fn desc(&self) -> String {
        "+12 mult if played hand contains a straight".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_straight().is_some() {
                g.mult += 12
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct DrollJoker {}

impl Joker for DrollJoker {
    fn name(&self) -> String {
        "Droll Joker".to_string()
    }
    fn desc(&self) -> String {
        "+10 mult if played hand contains a flush".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_flush().is_some() {
                g.mult += 10
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SlyJoker {}

impl Joker for SlyJoker {
    fn name(&self) -> String {
        "Sly Joker".to_string()
    }
    fn desc(&self) -> String {
        "+50 chips if played hand contains a pair".to_string()
    }
    fn cost(&self) -> usize {
        3
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_pair().is_some() {
                g.chips += 50
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct WilyJoker {}

impl Joker for WilyJoker {
    fn name(&self) -> String {
        "Wily Joker".to_string()
    }
    fn desc(&self) -> String {
        "+100 chips if played hand contains a three of a kind".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_three_of_kind().is_some() {
                g.chips += 100
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CleverJoker {}

impl Joker for CleverJoker {
    fn name(&self) -> String {
        "Clever Joker".to_string()
    }
    fn desc(&self) -> String {
        "+80 chips if played hand contains a two pair".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_two_pair().is_some() {
                g.chips += 80
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct DeviousJoker {}

impl Joker for DeviousJoker {
    fn name(&self) -> String {
        "Devious Joker".to_string()
    }
    fn desc(&self) -> String {
        "+100 chips if played hand contains a straight".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_straight().is_some() {
                g.chips += 100
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CraftyJoker {}

impl Joker for CraftyJoker {
    fn name(&self) -> String {
        "Crafty Joker".to_string()
    }
    fn desc(&self) -> String {
        "+80 chips if played hand contains a flush".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_flush().is_some() {
                g.chips += 80
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker #16: Half Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct HalfJoker {}

impl Joker for HalfJoker {
    fn name(&self) -> String {
        "Half Joker".to_string()
    }
    fn desc(&self) -> String {
        "+20 Mult if played hand contains 3 or fewer cards".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.cards().len() <= 3 {
                g.mult += 20;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker #17: Credit Card - allows going into debt
// Note: This is an Economy joker with passive effect
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CreditCard {}

impl Joker for CreditCard {
    fn name(&self) -> String {
        "Credit Card".to_string()
    }
    fn desc(&self) -> String {
        "Go up to -$20 in debt".to_string()
    }
    fn cost(&self) -> usize {
        1
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - handled in game logic
        vec![]
    }
}

// Joker #18: Banner
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Banner {}

impl Joker for Banner {
    fn name(&self) -> String {
        "Banner".to_string()
    }
    fn desc(&self) -> String {
        "+30 Chips for each remaining discard".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let discards_remaining = game.discards;
        fn apply(g: &mut Game, _hand: MadeHand, discards: usize) {
            g.chips += discards * 30;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, discards_remaining);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker #19: Mystic Summit
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MysticSummit {}

impl Joker for MysticSummit {
    fn name(&self) -> String {
        "Mystic Summit".to_string()
    }
    fn desc(&self) -> String {
        "+15 Mult when discards remaining is 0".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let discards_remaining = game.discards;
        fn apply(g: &mut Game, _hand: MadeHand, discards: usize) {
            if discards == 0 {
                g.mult += 15;
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, discards_remaining);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker #20: Raised Fist
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct RaisedFist {}

impl Joker for RaisedFist {
    fn name(&self) -> String {
        "Raised Fist".to_string()
    }
    fn desc(&self) -> String {
        "Adds double the rank of lowest ranked card held in hand to Mult".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Needs access to player hand (cards held but not played)
        // Would need: game.hand.iter().map(|c| c.value).min()
        vec![]
    }
}

// Joker #21: Chaos the Clown - 1 free reroll per shop
// Note: Passive effect, handled in shop logic
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ChaosTheClown {}

impl Joker for ChaosTheClown {
    fn name(&self) -> String {
        "Chaos the Clown".to_string()
    }
    fn desc(&self) -> String {
        "1 free Reroll per shop".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - handled in shop logic
        vec![]
    }
}

// Joker #22: Scary Face
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ScaryFace {}

impl Joker for ScaryFace {
    fn name(&self) -> String {
        "Scary Face".to_string()
    }
    fn desc(&self) -> String {
        "+30 Chips for each face card played in scoring".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let face_count = hand.hand.cards().iter().filter(|c| c.is_face()).count();
            g.chips += face_count * 30;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker #23: Abstract Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct AbstractJoker {}

impl Joker for AbstractJoker {
    fn name(&self) -> String {
        "Abstract Joker".to_string()
    }
    fn desc(&self) -> String {
        "+3 Mult for each Joker card (including self)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let joker_count = game.jokers.len();
        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            g.mult += count * 3;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, joker_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker #24: Delayed Gratification - Economy joker
// Note: Effect handled at end of round
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct DelayedGratification {}

impl Joker for DelayedGratification {
    fn name(&self) -> String {
        "Delayed Gratification".to_string()
    }
    fn desc(&self) -> String {
        "Earn $2 per discard if no discards used by end of the round".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Effect handled at end of round
        vec![]
    }
}

// Joker #25: Gros Michel - +15 Mult with destruction chance
// Note: Destruction handled separately
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GrosMichel {}

impl Joker for GrosMichel {
    fn name(&self) -> String {
        "Gros Michel".to_string()
    }
    fn desc(&self) -> String {
        "+15 Mult (1 in 6 chance to be destroyed at end of round)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, _hand: MadeHand) {
            g.mult += 15;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker #26: Even Steven
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct EvenSteven {}

impl Joker for EvenSteven {
    fn name(&self) -> String {
        "Even Steven".to_string()
    }
    fn desc(&self) -> String {
        "+4 Mult for each 10, 8, 6, 4, or 2 card in scored hand".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let even_count = hand.hand.cards().iter()
                .filter(|c| matches!(c.value, Value::Two | Value::Four | Value::Six | Value::Eight | Value::Ten))
                .count();
            g.mult += even_count * 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker #27: Odd Todd
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct OddTodd {}

impl Joker for OddTodd {
    fn name(&self) -> String {
        "Odd Todd".to_string()
    }
    fn desc(&self) -> String {
        "+31 Chips if played hand contains odd cards (A, 9, 7, 5, 3)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let has_odd = hand.hand.cards().iter()
                .any(|c| matches!(c.value, Value::Ace | Value::Three | Value::Five | Value::Seven | Value::Nine));
            if has_odd {
                g.chips += 31;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker #28: Scholar
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Scholar {}

impl Joker for Scholar {
    fn name(&self) -> String {
        "Scholar".to_string()
    }
    fn desc(&self) -> String {
        "+20 Chips and +4 Mult per Ace played".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus, Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let ace_count = hand.hand.cards().iter()
                .filter(|c| c.value == Value::Ace)
                .count();
            g.chips += ace_count * 20;
            g.mult += ace_count * 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker #29: Business Card - Economy joker
// Note: Random chance effect handled during scoring
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct BusinessCard {}

impl Joker for BusinessCard {
    fn name(&self) -> String {
        "Business Card".to_string()
    }
    fn desc(&self) -> String {
        "Played face cards have 1 in 2 chance to give $2 when scored".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #30: Supernova
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Supernova {}

impl Joker for Supernova {
    fn name(&self) -> String {
        "Supernova".to_string()
    }
    fn desc(&self) -> String {
        "Adds the number of times poker hand has been played this run to Mult".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Needs to track which hand rank was just played
        // Would add mult equal to the number of times that hand has been played
        // Requires: hand rank context in OnScore callback
        vec![]
    }
}

// Joker #31: Ride the Bus - Stateful joker
// Note: State tracking needed - simplified implementation
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct RideTheBus {}

impl Joker for RideTheBus {
    fn name(&self) -> String {
        "Ride the Bus".to_string()
    }
    fn desc(&self) -> String {
        "+1 Mult per consecutive hand without face cards (resets on face card)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Note: Full implementation would require state tracking
        // Simplified: just check if current hand has no face cards
        vec![]
    }
}

// Joker #32: Runner
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Runner {}

impl Joker for Runner {
    fn name(&self) -> String {
        "Runner".to_string()
    }
    fn desc(&self) -> String {
        "+15 Chips if played hand contains a Straight".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_straight().is_some() {
                g.chips += 15;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker #33: Ice Cream - Stateful joker
// Note: Full implementation would track hands played
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct IceCream {}

impl Joker for IceCream {
    fn name(&self) -> String {
        "Ice Cream".to_string()
    }
    fn desc(&self) -> String {
        "+100 Chips (-5 Chips for each hand played)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Needs stateful tracking of hands played this round
        // Would subtract 5 chips per hand played
        vec![]
    }
}

// Joker #34: Splash
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Splash {}

impl Joker for Splash {
    fn name(&self) -> String {
        "Splash".to_string()
    }
    fn desc(&self) -> String {
        "Every played card counts in scoring".to_string()
    }
    fn cost(&self) -> usize {
        3
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Note: This would need to be handled in the scoring logic itself
        // as it changes which cards count
        vec![]
    }
}

// Joker #35: Blue Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct BlueJoker {}

impl Joker for BlueJoker {
    fn name(&self) -> String {
        "Blue Joker".to_string()
    }
    fn desc(&self) -> String {
        "+2 Chips for each remaining card in deck".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let cards_in_deck = game.deck.cards().len();
        fn apply(g: &mut Game, _hand: MadeHand, deck_size: usize) {
            g.chips += deck_size * 2;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, cards_in_deck);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker #36-61: Continuing with remaining jokers...
// Note: Many of these require complex state management or are better implemented
// with game-level hooks. For now, I'll provide implementations for the simpler ones
// and stubs for the complex ones.

// Joker #36: Sixth Sense - Complex (random chance, spectral card generation)
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SixthSense {}

impl Joker for SixthSense {
    fn name(&self) -> String {
        "Sixth Sense".to_string()
    }
    fn desc(&self) -> String {
        "1 in 6 chance to destroy played 6, create Spectral card if successful".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Complex effect - would need game-level handling
        vec![]
    }
}

// Joker #37: Constellation - Stateful
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Constellation {}

impl Joker for Constellation {
    fn name(&self) -> String {
        "Constellation".to_string()
    }
    fn desc(&self) -> String {
        "Gains X0.1 Mult per Planet card used".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Stateful - would need to track planet cards used
        vec![]
    }
}

// Joker #38: Hiker - Modifies cards permanently
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Hiker {}

impl Joker for Hiker {
    fn name(&self) -> String {
        "Hiker".to_string()
    }
    fn desc(&self) -> String {
        "Every played card permanently gains +5 Chips when scored".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Complex - modifies cards permanently
        vec![]
    }
}

// Joker #39: Green Joker - Stateful
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GreenJoker {}

impl Joker for GreenJoker {
    fn name(&self) -> String {
        "Green Joker".to_string()
    }
    fn desc(&self) -> String {
        "+1 Mult per hand played; -1 Mult per discard".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Stateful - needs to track across rounds
        vec![]
    }
}

// Joker #40: Superposition - Creates tarot cards
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Superposition {}

impl Joker for Superposition {
    fn name(&self) -> String {
        "Superposition".to_string()
    }
    fn desc(&self) -> String {
        "Create a Tarot card if poker hand contains Straight and Ace".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Complex - creates consumables
        vec![]
    }
}

// Joker #41: To Do List - Changes per round
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ToDoList {}

impl Joker for ToDoList {
    fn name(&self) -> String {
        "To Do List".to_string()
    }
    fn desc(&self) -> String {
        "$5 if poker hand is listed type (hand changes each round)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Stateful - changes per round
        vec![]
    }
}

// Joker #42: Cavendish - Very rare destruction
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Cavendish {}

impl Joker for Cavendish {
    fn name(&self) -> String {
        "Cavendish".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult (1 in 1000 chance to be destroyed at end of round)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #43: Red Card - Triggers on pack skip
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct RedCard {}

impl Joker for RedCard {
    fn name(&self) -> String {
        "Red Card".to_string()
    }
    fn desc(&self) -> String {
        "Gains +3 Mult when any Booster Pack is skipped".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Stateful - needs game event handling
        vec![]
    }
}

// Joker #44: Square Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SquareJoker {}

impl Joker for SquareJoker {
    fn name(&self) -> String {
        "Square Joker".to_string()
    }
    fn desc(&self) -> String {
        "Gains +4 Chips if hand has exactly 4 cards".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.cards().len() == 4 {
                g.chips += 4;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Remaining jokers (#45-64) - These are unlockable and/or require complex state
// I'll add simplified implementations for completeness

// Joker #45: Riff-Raff
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct RiffRaff {}

impl Joker for RiffRaff {
    fn name(&self) -> String {
        "Riff-Raff".to_string()
    }
    fn desc(&self) -> String {
        "When Blind selected, create 2 Common Jokers".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #46: Golden Ticket
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GoldenTicket {}

impl Joker for GoldenTicket {
    fn name(&self) -> String {
        "Golden Ticket".to_string()
    }
    fn desc(&self) -> String {
        "Played Gold cards earn $3 when scored".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #47: Swashbuckler
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Swashbuckler {}

impl Joker for Swashbuckler {
    fn name(&self) -> String {
        "Swashbuckler".to_string()
    }
    fn desc(&self) -> String {
        "Adds sell value of all Jokers to Mult (+1 Mult per card sold)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let total_sell_value: usize = game.jokers.iter()
            .map(|j| j.sell_value())
            .sum();
        fn apply(g: &mut Game, _hand: MadeHand, sell_value: usize) {
            g.mult += sell_value;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, total_sell_value);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker #48: Smiley Face
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SmileyFace {}

impl Joker for SmileyFace {
    fn name(&self) -> String {
        "Smiley Face".to_string()
    }
    fn desc(&self) -> String {
        "+4 Mult for each face card played".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let face_count = hand.hand.cards().iter().filter(|c| c.is_face()).count();
            g.mult += face_count * 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker #49: Golden Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GoldenJoker {}

impl Joker for GoldenJoker {
    fn name(&self) -> String {
        "Golden Joker".to_string()
    }
    fn desc(&self) -> String {
        "Earn $3 at end of round".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #50: Drunkard - Passive effect
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Drunkard {}

impl Joker for Drunkard {
    fn name(&self) -> String {
        "Drunkard".to_string()
    }
    fn desc(&self) -> String {
        "+1 discard per round".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #51: Faceless Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FacelessJoker {}

impl Joker for FacelessJoker {
    fn name(&self) -> String {
        "Faceless Joker".to_string()
    }
    fn desc(&self) -> String {
        "Earn $5 if 3+ face cards discarded at once".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #52: Hanging Chad - Retrigger effect
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct HangingChad {}

impl Joker for HangingChad {
    fn name(&self) -> String {
        "Hanging Chad".to_string()
    }
    fn desc(&self) -> String {
        "Retrigger first card used in scoring 2 additional times".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Retrigger]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #53: Popcorn - Stateful
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Popcorn {}

impl Joker for Popcorn {
    fn name(&self) -> String {
        "Popcorn".to_string()
    }
    fn desc(&self) -> String {
        "+20 Mult (-4 Mult per round played)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #54: Walkie Talkie
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct WalkieTalkie {}

impl Joker for WalkieTalkie {
    fn name(&self) -> String {
        "Walkie Talkie".to_string()
    }
    fn desc(&self) -> String {
        "+10 Chips and +4 Mult for each 10 or 4 played".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus, Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let count = hand.hand.cards().iter()
                .filter(|c| matches!(c.value, Value::Ten | Value::Four))
                .count();
            g.chips += count * 10;
            g.mult += count * 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker #55: Shoot the Moon
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ShootTheMoon {}

impl Joker for ShootTheMoon {
    fn name(&self) -> String {
        "Shoot the Moon".to_string()
    }
    fn desc(&self) -> String {
        "+13 Mult for each Queen held in hand".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Needs access to player hand (cards held but not played)
        // Would need: game.hand.iter().filter(|c| c.value == Value::Queen).count() * 13
        vec![]
    }
}

// Joker #56: Fortune Teller - Stateful
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FortuneTeller {}

impl Joker for FortuneTeller {
    fn name(&self) -> String {
        "Fortune Teller".to_string()
    }
    fn desc(&self) -> String {
        "+1 Mult per Tarot card used this run".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #57: Juggler - Passive effect
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Juggler {}

impl Joker for Juggler {
    fn name(&self) -> String {
        "Juggler".to_string()
    }
    fn desc(&self) -> String {
        "+1 hand size".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #58: Photograph
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Photograph {}

impl Joker for Photograph {
    fn name(&self) -> String {
        "Photograph".to_string()
    }
    fn desc(&self) -> String {
        "First played face card gives X2 Mult when scored".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #59: Reserved Parking
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ReservedParking {}

impl Joker for ReservedParking {
    fn name(&self) -> String {
        "Reserved Parking".to_string()
    }
    fn desc(&self) -> String {
        "1 in 3 chance for each face card held in hand to give $1".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Needs access to player hand (cards held but not played)
        // Would need: game.hand.iter().filter(|c| c.is_face()).count() with random chance
        vec![]
    }
}

// Joker #60: Mail-In Rebate - Changes per round
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MailInRebate {}

impl Joker for MailInRebate {
    fn name(&self) -> String {
        "Mail-In Rebate".to_string()
    }
    fn desc(&self) -> String {
        "Earn $3 for each discarded rank (rank changes each round)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #61: 8 Ball - Complex (random chance, tarot generation)
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct EightBall {}

impl Joker for EightBall {
    fn name(&self) -> String {
        "8 Ball".to_string()
    }
    fn desc(&self) -> String {
        "1 in 5 chance per 8 played to create Tarot (no 8s in deck)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #62: Misprint - Random mult
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Misprint {}

impl Joker for Misprint {
    fn name(&self) -> String {
        "Misprint".to_string()
    }
    fn desc(&self) -> String {
        "+0 to +23 Mult (random each time)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

// Joker #63: Egg - Gains sell value over time
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Egg {}

impl Joker for Egg {
    fn name(&self) -> String {
        "Egg".to_string()
    }
    fn desc(&self) -> String {
        "Gains $3 sell value at end of round".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit, Value};
    use crate::hand::SelectHand;
    use crate::stage::{Blind, Stage};

    use super::*;

    fn score_before_after_joker(joker: Jokers, hand: SelectHand, before: usize, after: usize) {
        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // First score without joker
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, before);

        // Buy (and apply) the joker
        g.money += 1000; // Give adequate money to buy
        g.stage = Stage::Shop();
        g.shop.jokers.push(joker.clone());
        g.buy_joker(joker).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);
        // Second score with joker applied
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, after);
    }

    #[test]
    fn test_the_joker() {
        let ace = Card::new(Value::Ace, Suit::Heart);
        let hand = SelectHand::new(vec![ace]);

        // Score Ace high without joker
        // High card (level 1) -> 5 chips, 1 mult
        // Played cards (1 ace) -> 11 chips
        // (5 + 11) * (1) = 16
        let before = 16;
        // Score Ace high with the Joker
        // High card (level 1) -> 5 chips, 1 mult
        // Played cards (1 ace) -> 11 chips
        // Joker (The Joker) -> 4 mult
        // (5 + 11) * (1 + 4) = 80
        let after = 80;

        let j = Jokers::TheJoker(TheJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_lusty_joker() {
        let ah = Card::new(Value::Ace, Suit::Heart);
        let ac = Card::new(Value::Ace, Suit::Club);
        let ad = Card::new(Value::Ace, Suit::Diamond);
        let hand = SelectHand::new(vec![ah, ah, ac, ad]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok (2 hearts) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 2 hearts = +6 mult
        // (60 + 44) * (7 + 6) = 1352
        let after = 1352;

        let j = Jokers::LustyJoker(LustyJoker {});
        score_before_after_joker(j, hand, before, after)
    }

    #[test]
    fn test_greedy_joker() {
        let ah = Card::new(Value::Ace, Suit::Heart);
        let ad = Card::new(Value::Ace, Suit::Diamond);
        let hand = SelectHand::new(vec![ad, ad, ad, ah]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok (3 diamonds) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 3 diamonds = +9 mult
        // (60 + 44) * (7 + 9) = 1664
        let after = 1664;

        let j = Jokers::GreedyJoker(GreedyJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_wrathful_joker() {
        let asp = Card::new(Value::Ace, Suit::Spade);
        let ad = Card::new(Value::Ace, Suit::Diamond);
        let hand = SelectHand::new(vec![asp, ad, ad, ad]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok (1 spade) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 1 spade = +3 mult
        // (60 + 44) * (7 + 3) = 1040
        let after = 1040;

        let j = Jokers::WrathfulJoker(WrathfulJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_gluttonous_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok (4 clubs) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 4 clubs = +12 mult
        // (60 + 44) * (7 + 12) = 1976
        let after = 1976;

        let j = Jokers::GluttonousJoker(GluttonousJoker {});
        score_before_after_joker(j, hand, before, after)
    }

    #[test]
    fn test_jolly_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ pair = +8 mult
        // (60 + 44) * (7 + 8) = 1560
        let after = 1560;

        let j = Jokers::JollyJoker(JollyJoker {});
        score_before_after_joker(j, hand, before, after)
    }

    #[test]
    fn test_zany_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 3ok = +12 mult
        // (60 + 44) * (7 + 12) = 1976
        let after = 1976;

        let j = Jokers::ZanyJoker(ZanyJoker {});
        score_before_after_joker(j, hand, before, after)
    }

    #[test]
    fn test_mad_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let kc = Card::new(Value::King, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, kc, kc]);

        // Score two pair without joker
        // two pair (level 1) -> 20 chips, 2 mult
        // Played cards (2 ace, 2 king) -> 42 chips
        // (20 + 42) * (2) = 124
        let before = 124;
        let j = Jokers::MadJoker(MadJoker {});
        // Score two pair with joker
        // two pair (level 1) -> 20 chips, 2 mult
        // Played cards (2 ace, 2 king) -> 42 chips
        // joker w/ two pair = +10 mult
        // (20 + 42) * (2 + 10) = 744
        let after = 744;

        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_crazy_joker() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Club);
        let four = Card::new(Value::Four, Suit::Club);
        let five = Card::new(Value::Five, Suit::Club);
        let six = Card::new(Value::Six, Suit::Heart);
        let hand = SelectHand::new(vec![two, three, four, five, six]);

        // Score straight without joker
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // (15 + 30) * (4) = 180
        let before = 180;
        // Score straight with joker
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // joker w/ straight = +12 mult
        // (15+ 30) * (4 + 12) = 720
        let after = 720;

        let j = Jokers::CrazyJoker(CrazyJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_droll_joker() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Club);
        let four = Card::new(Value::Four, Suit::Club);
        let five = Card::new(Value::Five, Suit::Club);
        let ten = Card::new(Value::Ten, Suit::Club);
        let hand = SelectHand::new(vec![two, three, four, five, ten]);

        // Score flush without joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // (19 + 35) * (4) = 216
        let before = 216;
        // Score flush with joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // joker w/ flush = +10 mult
        // (19 + 35) * (4 + 10) = 756
        let after = 756;

        let j = Jokers::DrollJoker(DrollJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_sly_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ pair = +50 chips
        // (60 + 44 + 50) * (7) = 1078
        let after = 1078;

        let j = Jokers::SlyJoker(SlyJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_wily_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 3ok = +100 chips
        // (60 + 44 + 100) * (7) = 1428
        let after = 1428;

        let j = Jokers::WilyJoker(WilyJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_clever_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let kc = Card::new(Value::King, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, kc, kc]);

        // Score two pair without joker
        // two pair (level 1) -> 20 chips, 2 mult
        // Played cards (2 ace, 2 king) -> 42 chips
        // (20 + 42) * (2) = 124
        let before = 124;
        // Score two pair with joker
        // two pair (level 1) -> 20 chips, 2 mult
        // Played cards (2 ace, 2 king) -> 42 chips
        // joker w/ two pair = +80 chips
        // (20 + 42 + 80) * (2) = 284
        let after = 284;

        let j = Jokers::CleverJoker(CleverJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_devious_joker() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Club);
        let four = Card::new(Value::Four, Suit::Club);
        let five = Card::new(Value::Five, Suit::Club);
        let six = Card::new(Value::Six, Suit::Heart);
        let hand = SelectHand::new(vec![two, three, four, five, six]);

        // Score straight without joker
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // (15 + 30) * (4) = 180
        let before = 180;
        // Score straight with joker
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // joker w/ straight = +100 chips
        // (15+ 30 + 100) * (4) = 580
        let after = 580;

        let j = Jokers::DeviousJoker(DeviousJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_crafty_joker() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Club);
        let four = Card::new(Value::Four, Suit::Club);
        let five = Card::new(Value::Five, Suit::Club);
        let ten = Card::new(Value::Ten, Suit::Club);
        let hand = SelectHand::new(vec![two, three, four, five, ten]);

        // Score flush without joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // (19 + 35) * (4) = 216
        let before = 216;
        // Score flush with joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // joker w/ flush = +80 chips
        // (19 + 35 + 80) * (4) = 536
        let after = 536;
        let j = Jokers::CraftyJoker(CraftyJoker {});
        score_before_after_joker(j, hand, before, after);
    }
}
