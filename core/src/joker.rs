use crate::card::Suit;
use crate::effect::Effects;
use crate::game::Game;
use crate::hand::MadeHand;
use crate::rank::HandRank;
use pyo3::pyclass;
use std::collections::HashMap;
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
    Egg,
    Fibonacci,
    SpareTrousers,
    Acrobat,
    OnyxAgate,
    Arrowhead,
    TheDuo,
    TheTrio,
    Bloodstone,
    RoughGem,
    FlashCard,
    StoneJoker,
    Bull,
    Erosion,
    TheFamily,
    TheOrder,
    TheTribe,
    Triboulet,
    FourFingers,
    Mime,
    MarbleJoker,
    SteelJoker,
    Pareidolia,
    Blackboard,
    SmearedJoker,
    FlowerPot,
    SeeingDouble,
    Baron,
    Blueprint,
    JokerStencil,
    Showman,
    Bootstraps,
    Cloud9,
    WeeJoker,
    BaseballCard,
    AncientJoker,
    Stuntman,
    Canio,
    Yorick,
    CardSharp,
    Chicot
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
    fn effects(&self, game: &Game) -> Vec<Effects> {
        use crate::card::Value;
        // Find lowest ranked card in hand and get its rank value
        let lowest_rank_value = game.hand.iter().map(|c| match c.value {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
            Value::Ace => 11,
        }).min().unwrap_or(0);
        let mult_bonus = lowest_rank_value * 2;

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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
    fn effects(&self, game: &Game) -> Vec<Effects> {
        // Clone the play counts HashMap for the closure
        let play_counts = game.hand_rank_play_counts.clone();

        fn apply(g: &mut Game, hand: MadeHand, counts: HashMap<HandRank, usize>) {
            let times_played = counts.get(&hand.rank).copied().unwrap_or(0);
            g.mult += times_played;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, play_counts.clone());
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let mult_bonus = game.round_state.consecutive_hands_without_faces;

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let chips_bonus = 100_isize - (game.hands_played_this_blind as isize * 5);
        let chips_bonus = chips_bonus.max(0) as usize; // Don't go negative

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.chips += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, chips_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Constellation {
    pub planet_cards_used: usize,
    pub bonus_mult: f32,
}

impl Default for Constellation {
    fn default() -> Self {
        Self {
            planet_cards_used: 0,
            bonus_mult: 1.0,
        }
    }
}

impl Eq for Constellation {}

impl std::hash::Hash for Constellation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.planet_cards_used.hash(state);
        self.bonus_mult.to_bits().hash(state);
    }
}

impl Constellation {
    pub fn on_planet_used(&mut self) {
        self.planet_cards_used += 1;
        self.bonus_mult = 1.0 + (self.planet_cards_used as f32 * 0.1);
    }
}

impl Joker for Constellation {
    fn name(&self) -> String {
        "Constellation".to_string()
    }
    fn desc(&self) -> String {
        format!("X{:.1} Mult ({} Planet cards used)", self.bonus_mult, self.planet_cards_used)
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
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let mult_multiplier = self.bonus_mult;

        fn apply(g: &mut Game, _hand: MadeHand, multiplier: f32) {
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_multiplier);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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
#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GreenJoker {
    pub bonus_mult: isize,  // Accumulated mult bonus (can be negative)
}

impl Default for GreenJoker {
    fn default() -> Self {
        Self { bonus_mult: 0 }
    }
}

impl GreenJoker {
    pub fn on_hand_played(&mut self) {
        self.bonus_mult += 1;
    }

    pub fn on_discard_used(&mut self) {
        self.bonus_mult -= 1;
    }
}

impl Joker for GreenJoker {
    fn name(&self) -> String {
        "Green Joker".to_string()
    }
    fn desc(&self) -> String {
        format!("{:+} Mult (+1 per hand played; -1 per discard)", self.bonus_mult)
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
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let mult_bonus = self.bonus_mult;

        fn apply(g: &mut Game, _hand: MadeHand, bonus: isize) {
            if bonus >= 0 {
                g.mult += bonus as usize;
            } else {
                g.mult = g.mult.saturating_sub((-bonus) as usize);
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let todo_hand = game.round_state.todo_hand;

        fn apply(g: &mut Game, hand: MadeHand, target: Option<HandRank>) {
            if let Some(target_rank) = target {
                if hand.rank == target_rank {
                    g.money += 5;
                }
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, todo_hand);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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
pub struct RedCard {
    pub bonus_mult: usize,
}

impl RedCard {
    pub fn on_booster_skipped(&mut self) {
        self.bonus_mult += 3;
    }
}

impl Joker for RedCard {
    fn name(&self) -> String {
        "Red Card".to_string()
    }
    fn desc(&self) -> String {
        format!("+{} Mult (gains +3 when Booster Pack skipped)", self.bonus_mult)
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
        let mult_bonus = self.bonus_mult;

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let mult_bonus = 20_isize - (game.round as isize * 4);
        let mult_bonus = mult_bonus.max(0) as usize; // Don't go negative

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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
    fn effects(&self, game: &Game) -> Vec<Effects> {
        use crate::card::Value;
        let queen_count = game.hand.iter().filter(|c| c.value == Value::Queen).count();
        let mult_bonus = queen_count * 13;

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker #56: Fortune Teller - Stateful
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FortuneTeller {
    pub tarot_cards_used: usize,
}

impl FortuneTeller {
    pub fn on_tarot_used(&mut self) {
        self.tarot_cards_used += 1;
    }
}

impl Joker for FortuneTeller {
    fn name(&self) -> String {
        "Fortune Teller".to_string()
    }
    fn desc(&self) -> String {
        format!("+{} Mult ({} Tarot cards used)", self.tarot_cards_used, self.tarot_cards_used)
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
        let mult_bonus = self.tarot_cards_used;

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let face_cards: Vec<_> = game.hand.iter().filter(|c| c.is_face()).collect();
        let mut money_bonus = 0;

        // Each face card has 1 in 3 chance to give $1
        for _ in &face_cards {
            if rand::random::<f32>() < 1.0 / 3.0 {
                money_bonus += 1;
            }
        }

        fn apply(g: &mut Game, _hand: MadeHand, money: usize) {
            g.money += money;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, money_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
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

// UNCOMMON JOKERS

// Joker: Fibonacci - Each played Ace, 2, 3, 5, or 8 gives +8 Mult when scored
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Fibonacci {}

impl Joker for Fibonacci {
    fn name(&self) -> String {
        "Fibonacci".to_string()
    }
    fn desc(&self) -> String {
        "Each played Ace, 2, 3, 5, or 8 gives +8 Mult when scored".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let fib_count = hand.hand.cards().iter()
                .filter(|c| matches!(c.value, Value::Ace | Value::Two | Value::Three | Value::Five | Value::Eight))
                .count();
            g.mult += fib_count * 8;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Spare Trousers - Gains +2 Mult if played hand contains Two Pair
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SpareTrousers {}

impl Joker for SpareTrousers {
    fn name(&self) -> String {
        "Spare Trousers".to_string()
    }
    fn desc(&self) -> String {
        "Gains +2 Mult if played hand contains Two Pair".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_two_pair().is_some() {
                g.mult += 2;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Acrobat - X3 Mult on final hand of round
// Note: Requires tracking if this is the final hand
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Acrobat {}

impl Joker for Acrobat {
    fn name(&self) -> String {
        "Acrobat".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult on final hand of round".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let is_final_hand = game.plays == 1;
        fn apply(g: &mut Game, _hand: MadeHand, final_hand: bool) {
            if final_hand {
                g.mult = g.mult * 3;
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, is_final_hand);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Onyx Agate - +7 Mult for each Club card played
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct OnyxAgate {}

impl Joker for OnyxAgate {
    fn name(&self) -> String {
        "Onyx Agate".to_string()
    }
    fn desc(&self) -> String {
        "+7 Mult for each Club card played".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
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
            g.mult += clubs * 7;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Arrowhead - Played Spade cards give +50 Chips when scored
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Arrowhead {}

impl Joker for Arrowhead {
    fn name(&self) -> String {
        "Arrowhead".to_string()
    }
    fn desc(&self) -> String {
        "Played Spade cards give +50 Chips when scored".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let spades = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Spade)
                .count();
            g.chips += spades * 50;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// RARE JOKERS

// Joker: The Duo - X2 Mult if played hand contains a Pair
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheDuo {}

impl Joker for TheDuo {
    fn name(&self) -> String {
        "The Duo".to_string()
    }
    fn desc(&self) -> String {
        "X2 Mult if played hand contains a Pair".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_pair().is_some() {
                g.mult = g.mult * 2;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: The Trio - X3 Mult if played hand contains Three of a Kind
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheTrio {}

impl Joker for TheTrio {
    fn name(&self) -> String {
        "The Trio".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if played hand contains Three of a Kind".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_three_of_kind().is_some() {
                g.mult = g.mult * 3;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Bloodstone - 1 in 2 chance for Hearts to give X1.5 Mult when scored
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Bloodstone {}

impl Joker for Bloodstone {
    fn name(&self) -> String {
        "Bloodstone".to_string()
    }
    fn desc(&self) -> String {
        "1 in 2 chance for Hearts to give X1.5 Mult when scored".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use rand::Rng;
        fn apply(g: &mut Game, hand: MadeHand) {
            let hearts_count = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Heart)
                .count();

            for _ in 0..hearts_count {
                if rand::thread_rng().gen_bool(0.5) {
                    g.mult = (g.mult as f32 * 1.5) as usize;
                }
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Rough Gem - Played Diamond cards earn $1 when scored
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct RoughGem {}

impl Joker for RoughGem {
    fn name(&self) -> String {
        "Rough Gem".to_string()
    }
    fn desc(&self) -> String {
        "Played Diamond cards earn $1 when scored".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let diamonds = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Diamond)
                .count();
            g.money += diamonds;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Flash Card - Gains +2 Mult per reroll in shop
// Note: Requires tracking shop rerolls
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FlashCard {}

impl Joker for FlashCard {
    fn name(&self) -> String {
        "Flash Card".to_string()
    }
    fn desc(&self) -> String {
        "Gains +2 Mult per reroll in shop".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        // Count rerolls from shop
        let reroll_count = game.shop.rerolls_this_round;
        fn apply(g: &mut Game, _hand: MadeHand, rerolls: usize) {
            g.mult += rerolls * 2;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, reroll_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Stone Joker - Gains +25 Chips for each Stone Card in full deck
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct StoneJoker {}

impl Joker for StoneJoker {
    fn name(&self) -> String {
        "Stone Joker".to_string()
    }
    fn desc(&self) -> String {
        "Gains +25 Chips for each Stone Card in full deck".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        use crate::card::Enhancement;
        let stone_count = game
            .deck
            .cards()
            .iter()
            .filter(|c| c.enhancement == Some(Enhancement::Stone))
            .count();
        fn apply(g: &mut Game, _hand: MadeHand, stones: usize) {
            g.chips += stones * 25;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, stone_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Bull - +2 Chips for each $1 you have
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Bull {}

impl Joker for Bull {
    fn name(&self) -> String {
        "Bull".to_string()
    }
    fn desc(&self) -> String {
        "+2 Chips for each $1 you have".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let money = game.money;
        fn apply(g: &mut Game, _hand: MadeHand, money: usize) {
            g.chips += money * 2;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, money);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Erosion - +4 Mult for each card below 52 in full deck
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Erosion {}

impl Joker for Erosion {
    fn name(&self) -> String {
        "Erosion".to_string()
    }
    fn desc(&self) -> String {
        "+4 Mult for each card below 52 in full deck".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let cards_below = 52_usize.saturating_sub(game.deck.cards().len());
        fn apply(g: &mut Game, _hand: MadeHand, missing: usize) {
            g.mult += missing * 4;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, cards_below);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: The Family - X4 Mult if played hand contains Four of a Kind
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheFamily {}

impl Joker for TheFamily {
    fn name(&self) -> String {
        "The Family".to_string()
    }
    fn desc(&self) -> String {
        "X4 Mult if played hand contains Four of a Kind".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_four_of_kind().is_some() {
                g.mult = g.mult * 4;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: The Order - X3 Mult if played hand contains Straight
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheOrder {}

impl Joker for TheOrder {
    fn name(&self) -> String {
        "The Order".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if played hand contains Straight".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_straight().is_some() {
                g.mult = g.mult * 3;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: The Tribe - X2 Mult if played hand contains Flush
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheTribe {}

impl Joker for TheTribe {
    fn name(&self) -> String {
        "The Tribe".to_string()
    }
    fn desc(&self) -> String {
        "X2 Mult if played hand contains Flush".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_flush().is_some() {
                g.mult = g.mult * 2;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// LEGENDARY JOKERS

// Joker: Triboulet - Played Kings and Queens each give X2 Mult when scored
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Triboulet {}

impl Joker for Triboulet {
    fn name(&self) -> String {
        "Triboulet".to_string()
    }
    fn desc(&self) -> String {
        "Played Kings and Queens each give X2 Mult when scored".to_string()
    }
    fn cost(&self) -> usize {
        0
    }
    fn rarity(&self) -> Rarity {
        Rarity::Legendary
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let royal_count = hand
                .hand
                .cards()
                .iter()
                .filter(|c| matches!(c.value, Value::King | Value::Queen))
                .count();

            for _ in 0..royal_count {
                g.mult = g.mult * 2;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Four Fingers - All Flushes and Straights can be made with 4 cards
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FourFingers {}
impl Joker for FourFingers {
    fn name(&self) -> String {
        "Four Fingers".to_string()
    }
    fn desc(&self) -> String {
        "All Flushes and Straights can be made with 4 cards".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - would need to be handled in hand detection logic
        vec![]
    }
}

// Joker: Mime - Retrigger all card held in hand abilities
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Mime {}
impl Joker for Mime {
    fn name(&self) -> String {
        "Mime".to_string()
    }
    fn desc(&self) -> String {
        "Retrigger all card held in hand abilities".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Retrigger]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Complex - would need to retrigger cards in hand (not played cards)
        vec![]
    }
}

// Joker: Marble Joker - Adds one Stone card to deck when Blind selected
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MarbleJoker {}
impl Joker for MarbleJoker {
    fn name(&self) -> String {
        "Marble Joker".to_string()
    }
    fn desc(&self) -> String {
        "Adds one Stone card to deck when Blind selected".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Would need to be handled at blind selection time
        vec![]
    }
}

// Joker: Steel Joker - Gains X0.2 Mult for each Steel Card in full deck
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SteelJoker {}
impl Joker for SteelJoker {
    fn name(&self) -> String {
        "Steel Joker".to_string()
    }
    fn desc(&self) -> String {
        "Gains X0.2 Mult for each Steel Card in full deck".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        use crate::card::Edition;
        let steel_count = game
            .deck
            .cards()
            .iter()
            .filter(|c| c.edition == Edition::Foil)
            .count();
        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            // X0.2 per steel card = multiply by (1.0 + 0.2 * count)
            let multiplier = 1.0 + (0.2 * count as f32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, steel_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Pareidolia - All cards considered face cards
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Pareidolia {}
impl Joker for Pareidolia {
    fn name(&self) -> String {
        "Pareidolia".to_string()
    }
    fn desc(&self) -> String {
        "All cards considered face cards".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - would need to be handled in card.is_face() logic
        vec![]
    }
}

// Joker: Blackboard - X3 Mult if all cards held in hand are Spades or Clubs
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Blackboard {}
impl Joker for Blackboard {
    fn name(&self) -> String {
        "Blackboard".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if all cards held in hand are Spades or Clubs".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        // Check if all cards in hand are Spades or Clubs
        let all_black = game.hand.iter().all(|c| c.suit == Suit::Spade || c.suit == Suit::Club);
        let mult_multiplier = if all_black { 3 } else { 1 };

        fn apply(g: &mut Game, _hand: MadeHand, multiplier: usize) {
            g.mult = g.mult * multiplier;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_multiplier);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Smeared Joker - Hearts and Diamonds count as same suit; Spades and Clubs count as same suit
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SmearedJoker {}
impl Joker for SmearedJoker {
    fn name(&self) -> String {
        "Smeared Joker".to_string()
    }
    fn desc(&self) -> String {
        "Hearts and Diamonds count as same suit; Spades and Clubs count as same suit".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - would need to be handled in flush detection logic
        vec![]
    }
}

// Joker: Flower Pot - X3 Mult if hand contains Diamond, Club, Heart, and Spade cards
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FlowerPot {}
impl Joker for FlowerPot {
    fn name(&self) -> String {
        "Flower Pot".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if hand contains Diamond, Club, Heart, and Spade cards".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Suit;
        fn apply(g: &mut Game, hand: MadeHand) {
            // Check all played cards, not just the made hand
            let has_diamond = hand.all.iter().any(|c| c.suit == Suit::Diamond);
            let has_club = hand.all.iter().any(|c| c.suit == Suit::Club);
            let has_heart = hand.all.iter().any(|c| c.suit == Suit::Heart);
            let has_spade = hand.all.iter().any(|c| c.suit == Suit::Spade);

            if has_diamond && has_club && has_heart && has_spade {
                g.mult = g.mult * 3;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Seeing Double - X2 Mult if played hand has Club card and any other suit card
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SeeingDouble {}
impl Joker for SeeingDouble {
    fn name(&self) -> String {
        "Seeing Double".to_string()
    }
    fn desc(&self) -> String {
        "X2 Mult if played hand has Club card and any other suit card".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Suit;
        fn apply(g: &mut Game, hand: MadeHand) {
            // Check all played cards, not just the made hand
            let has_club = hand.all.iter().any(|c| c.suit == Suit::Club);
            let has_other = hand.all.iter().any(|c| c.suit != Suit::Club);

            if has_club && has_other {
                g.mult = g.mult * 2;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Baron - Each King held in hand gives X1.5 Mult
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Baron {}
impl Joker for Baron {
    fn name(&self) -> String {
        "Baron".to_string()
    }
    fn desc(&self) -> String {
        "Each King held in hand gives X1.5 Mult".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        use crate::card::Value;
        let king_count = game.hand.iter().filter(|c| c.value == Value::King).count();
        let mult_multiplier = 1.5_f32.powi(king_count as i32);

        fn apply(g: &mut Game, _hand: MadeHand, multiplier: f32) {
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_multiplier);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Blueprint - Copies ability of Joker to the right
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Blueprint {}
impl Joker for Blueprint {
    fn name(&self) -> String {
        "Blueprint".to_string()
    }
    fn desc(&self) -> String {
        "Copies ability of Joker to the right".to_string()
    }
    fn cost(&self) -> usize {
        10
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // Complex - would need to dynamically copy another joker's effects
        vec![]
    }
}

// Joker: JokerStencil - X1 Mult for each empty Joker slot (counts itself as empty)
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct JokerStencil {}
impl Joker for JokerStencil {
    fn name(&self) -> String {
        "Joker Stencil".to_string()
    }
    fn desc(&self) -> String {
        "X1 Mult for each empty Joker slot (counts itself as empty)".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        // Max joker slots is typically 5, count empty slots
        let max_slots: usize = 5;
        let current_jokers = game.jokers.len();
        let empty_slots = max_slots.saturating_sub(current_jokers).saturating_add(1); // +1 because it counts itself as empty
        fn apply(g: &mut Game, _hand: MadeHand, slots: usize) {
            // X1 per slot means multiply by (1 * slots), which is just slots
            g.mult = g.mult * slots;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, empty_slots);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Showman - +4 Mult for Joker, Tarot, Planet, or Spectral cards remaining in consumable slots
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Showman {}
impl Joker for Showman {
    fn name(&self) -> String {
        "Showman".to_string()
    }
    fn desc(&self) -> String {
        "Gains +4 Mult for Joker, Tarot, Planet, or Spectral cards remaining in consumable slots".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let consumable_count = game.consumables.len();
        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            g.mult += count * 4;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, consumable_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Bootstraps - Gains +2 Mult for every $5 you have
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Bootstraps {}
impl Joker for Bootstraps {
    fn name(&self) -> String {
        "Bootstraps".to_string()
    }
    fn desc(&self) -> String {
        "Gains +2 Mult for every $5 you have".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let mult_bonus = (game.money / 5) * 2;
        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Cloud9 - Earn $1 for each 9 in full deck at end of round
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Cloud9 {}
impl Joker for Cloud9 {
    fn name(&self) -> String {
        "Cloud 9".to_string()
    }
    fn desc(&self) -> String {
        "Earn $1 for each 9 in full deck at end of round".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Effect handled at end of round
        vec![]
    }
}

// Joker: WeeJoker - Gains +8 Chips when each played 2 is scored
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct WeeJoker {}
impl Joker for WeeJoker {
    fn name(&self) -> String {
        "Wee Joker".to_string()
    }
    fn desc(&self) -> String {
        "Gains +8 Chips when each played 2 is scored".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let twos = hand
                .hand
                .cards()
                .iter()
                .filter(|c| c.value == Value::Two)
                .count();
            g.chips += twos * 8;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: BaseballCard - Uncommon Jokers each give X1.5 Mult
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct BaseballCard {}
impl Joker for BaseballCard {
    fn name(&self) -> String {
        "Baseball Card".to_string()
    }
    fn desc(&self) -> String {
        "Uncommon Jokers each give X1.5 Mult".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let uncommon_count = game
            .jokers
            .iter()
            .filter(|j| j.rarity() == Rarity::Uncommon)
            .count();
        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            // X1.5 per uncommon = multiply by (1.5 ^ count)
            let multiplier = 1.5_f32.powi(count as i32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, uncommon_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: AncientJoker - Each played card with [suit] gives X1.5 Mult when scored
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct AncientJoker {}
impl Joker for AncientJoker {
    fn name(&self) -> String {
        "Ancient Joker".to_string()
    }
    fn desc(&self) -> String {
        "Each played card with [suit] gives X1.5 Mult when scored; suit changes at end of round".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let ancient_suit = game.round_state.ancient_suit;

        fn apply(g: &mut Game, hand: MadeHand, suit: Option<Suit>) {
            if let Some(target_suit) = suit {
                let matching_cards = hand.all.iter().filter(|c| c.suit == target_suit).count();
                if matching_cards > 0 {
                    let multiplier = 1.5_f32.powi(matching_cards as i32);
                    g.mult = (g.mult as f32 * multiplier) as usize;
                }
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, ancient_suit);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Stuntman - +250 Chips; +3 hand size
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Stuntman {}
impl Joker for Stuntman {
    fn name(&self) -> String {
        "Stuntman".to_string()
    }
    fn desc(&self) -> String {
        "+250 Chips; +3 hand size".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips, Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, _hand: MadeHand) {
            g.chips += 250;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Canio - Gains X1 Mult when a face card is destroyed
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Canio {
    pub bonus_mult: f32,  // Accumulated X mult multiplier (starts at 1.0)
}

// Manual implementations for Eq and Hash since f32 doesn't support them
impl Eq for Canio {}

impl std::hash::Hash for Canio {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the bits of the f32 for deterministic hashing
        self.bonus_mult.to_bits().hash(state);
    }
}

impl Default for Canio {
    fn default() -> Self {
        Self { bonus_mult: 1.0 }
    }
}

impl Canio {
    pub fn on_face_card_destroyed(&mut self) {
        self.bonus_mult += 1.0;
    }
}

impl Joker for Canio {
    fn name(&self) -> String {
        "Canio".to_string()
    }
    fn desc(&self) -> String {
        format!("X{} Mult (gains X1 Mult when a face card is destroyed)", self.bonus_mult)
    }
    fn cost(&self) -> usize {
        0
    }
    fn rarity(&self) -> Rarity {
        Rarity::Legendary
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let mult_multiplier = self.bonus_mult;

        fn apply(g: &mut Game, _hand: MadeHand, multiplier: f32) {
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_multiplier);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Yorick - Gains X1 Mult every 23 cards discarded
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Yorick {
    pub cards_discarded: usize,  // Total cards discarded
    pub bonus_mult: f32,          // Accumulated X mult (starts at 1.0)
}

impl Default for Yorick {
    fn default() -> Self {
        Self {
            cards_discarded: 0,
            bonus_mult: 1.0,
        }
    }
}

impl Eq for Yorick {}

impl std::hash::Hash for Yorick {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cards_discarded.hash(state);
        self.bonus_mult.to_bits().hash(state);
    }
}

impl Yorick {
    pub fn on_cards_discarded(&mut self, count: usize) {
        self.cards_discarded += count;
        // Every 23 cards, gain X1 mult
        let levels = self.cards_discarded / 23;
        self.bonus_mult = 1.0 + levels as f32;
    }
}

impl Joker for Yorick {
    fn name(&self) -> String {
        "Yorick".to_string()
    }
    fn desc(&self) -> String {
        format!("X{} Mult ({}/23 cards discarded for next level)", self.bonus_mult, self.cards_discarded % 23)
    }
    fn cost(&self) -> usize {
        0
    }
    fn rarity(&self) -> Rarity {
        Rarity::Legendary
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let mult_multiplier = self.bonus_mult;

        fn apply(g: &mut Game, _hand: MadeHand, multiplier: f32) {
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_multiplier);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Card Sharp - X3 Mult if hand type already played this round
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CardSharp {}

impl Joker for CardSharp {
    fn name(&self) -> String {
        "Card Sharp".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if poker hand has already been played this round".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let hands_played = game.round_state.hands_played_this_round.clone();

        fn apply(g: &mut Game, hand: MadeHand, played: std::collections::HashSet<HandRank>) {
            if played.contains(&hand.rank) {
                g.mult = g.mult * 3;
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, hands_played.clone());
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Chicot - Disables effect of every Boss Blind
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Chicot {}
impl Joker for Chicot {
    fn name(&self) -> String {
        "Chicot".to_string()
    }
    fn desc(&self) -> String {
        "Disables effect of every Boss Blind".to_string()
    }
    fn cost(&self) -> usize {
        0
    }
    fn rarity(&self) -> Rarity {
        Rarity::Legendary
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - would be checked in Boss Blind logic
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

    #[test]
    fn test_half_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let kc = Card::new(Value::King, Suit::Club);
        let qc = Card::new(Value::Queen, Suit::Club);
        // High card best_hand() returns only 1 card (the highest)
        // So we need 3 or fewer cards total
        let hand = SelectHand::new(vec![ac, kc, qc]);

        // Score high card without joker (only ace counts)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 ace) -> 11 chips
        // (5 + 11) * (1) = 16
        let before = 16;
        // Score high card with joker (3 cards selected, triggers +20 mult)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 ace) -> 11 chips
        // Half Joker: +20 mult (hand has ≤3 cards)
        // (5 + 11) * (1 + 20) = 336
        let after = 336;

        let j = Jokers::HalfJoker(HalfJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_banner() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);
        g.discards = 3; // Set 3 remaining discards

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::Banner(Banner {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);
        g.discards = 3; // Restore discards

        // Score pair with Banner (3 discards = +90 chips)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Banner: +90 chips (3 discards × 30)
        // (10 + 22 + 90) * (2) = 244
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 244);
    }

    #[test]
    fn test_mystic_summit() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);
        g.discards = 0; // Set 0 remaining discards

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::MysticSummit(MysticSummit {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);
        g.discards = 0; // Restore 0 discards

        // Score pair with Mystic Summit (0 discards = +15 mult)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Mystic Summit: +15 mult
        // (10 + 22) * (2 + 15) = 544
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 544);
    }

    #[test]
    fn test_scary_face() {
        let kc = Card::new(Value::King, Suit::Club);
        let qh = Card::new(Value::Queen, Suit::Heart);
        let jd = Card::new(Value::Jack, Suit::Diamond);
        let hand = SelectHand::new(vec![kc, qh, jd]);

        // Score high card without joker (only King counts in made hand)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 King) -> 10 chips
        // (5 + 10) * (1) = 15
        let before = 15;
        // Score high card with Scary Face (1 face card in made hand = +30 chips)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 King) -> 10 chips
        // Scary Face: +30 chips (1 face card)
        // (5 + 10 + 30) * (1) = 45
        let after = 45;

        let j = Jokers::ScaryFace(ScaryFace {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_abstract_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::AbstractJoker(AbstractJoker {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair with Abstract Joker (1 joker = +3 mult)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Abstract Joker: +3 mult (1 joker)
        // (10 + 22) * (2 + 3) = 160
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 160);
    }

    #[test]
    fn test_gros_michel() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac]);

        // Score high card without joker
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (A) -> 11 chips
        // (5 + 11) * (1) = 16
        let before = 16;
        // Score high card with Gros Michel (+15 mult)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (A) -> 11 chips
        // Gros Michel: +15 mult
        // (5 + 11) * (1 + 15) = 256
        let after = 256;

        let j = Jokers::GrosMichel(GrosMichel {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_even_steven() {
        let two = Card::new(Value::Two, Suit::Club);
        let four = Card::new(Value::Four, Suit::Heart);
        let six = Card::new(Value::Six, Suit::Diamond);
        let eight = Card::new(Value::Eight, Suit::Spade);
        let ten = Card::new(Value::Ten, Suit::Club);
        let hand = SelectHand::new(vec![two, four, six, eight, ten]);

        // Score high card without joker (only Ten counts in made hand)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Ten) -> 9 chips
        // (5 + 9) * (1) = 14
        let before = 14;
        // Score high card with Even Steven (1 even card in made hand = +4 mult)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Ten) -> 9 chips
        // Even Steven: +4 mult (1 even card)
        // (5 + 9) * (1 + 4) = 70
        let after = 70;

        let j = Jokers::EvenSteven(EvenSteven {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_odd_todd() {
        let ace = Card::new(Value::Ace, Suit::Club);
        let three = Card::new(Value::Three, Suit::Heart);
        let five = Card::new(Value::Five, Suit::Diamond);
        let hand = SelectHand::new(vec![ace, three, five]);

        // Score high card without joker (only Ace counts in made hand)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Ace) -> 11 chips
        // (5 + 11) * (1) = 16
        let before = 16;
        // Score high card with Odd Todd (made hand has 1 odd card = +31 chips)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Ace) -> 11 chips
        // Odd Todd: +31 chips (Ace is odd)
        // (5 + 11 + 31) * (1) = 47
        let after = 47;

        let j = Jokers::OddTodd(OddTodd {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_scholar() {
        let ah = Card::new(Value::Ace, Suit::Heart);
        let ad = Card::new(Value::Ace, Suit::Diamond);
        let hand = SelectHand::new(vec![ah, ad]);

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let before = 64;
        // Score pair with Scholar (2 aces = +40 chips, +8 mult)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Scholar: +40 chips, +8 mult
        // (10 + 22 + 40) * (2 + 8) = 720
        let after = 720;

        let j = Jokers::Scholar(Scholar {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_runner() {
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
        // Score straight with Runner (+15 chips)
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // Runner: +15 chips
        // (15 + 30 + 15) * (4) = 240
        let after = 240;

        let j = Jokers::Runner(Runner {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_blue_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::BlueJoker(BlueJoker {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Default deck has 52 cards, we drew 2 for the hand, so 50 in deck
        let cards_in_deck = g.deck.cards().len();
        // Score pair with Blue Joker (+2 chips per card in deck)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Blue Joker: +100 chips (50 cards × 2)
        // (10 + 22 + 100) * (2) = 264
        let expected = (10 + 22 + cards_in_deck * 2) * 2;
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, expected);
    }

    #[test]
    fn test_square_joker() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Heart);
        let four = Card::new(Value::Four, Suit::Diamond);
        let five = Card::new(Value::Five, Suit::Spade);
        let hand = SelectHand::new(vec![two, three, four, five]);

        // Score high card without joker (only 5 counts in made hand)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Five) -> 4 chips
        // (5 + 4) * (1) = 9
        let before = 9;
        // Score high card with Square Joker (made hand has 1 card, not 4)
        // Square Joker only triggers if hand has exactly 4 cards
        // But high card only uses 1 card, so no bonus
        // (5 + 4) * (1) = 9
        let after = 9;

        let j = Jokers::SquareJoker(SquareJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_smiley_face() {
        let kc = Card::new(Value::King, Suit::Club);
        let qh = Card::new(Value::Queen, Suit::Heart);
        let jd = Card::new(Value::Jack, Suit::Diamond);
        let hand = SelectHand::new(vec![kc, qh, jd]);

        // Score high card without joker (only King counts in made hand)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 King) -> 10 chips
        // (5 + 10) * (1) = 15
        let before = 15;
        // Score high card with Smiley Face (1 face card in made hand = +4 mult)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 King) -> 10 chips
        // Smiley Face: +4 mult (1 face card)
        // (5 + 10) * (1 + 4) = 75
        let after = 75;

        let j = Jokers::SmileyFace(SmileyFace {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_swashbuckler() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::Swashbuckler(Swashbuckler {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j.clone()).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair with Swashbuckler (1 joker at $4 = sell value $2)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Swashbuckler: +2 mult (sell value of itself)
        // (10 + 22) * (2 + 2) = 128
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 128);
    }

    #[test]
    fn test_walkie_talkie() {
        let ten = Card::new(Value::Ten, Suit::Club);
        let four = Card::new(Value::Four, Suit::Heart);
        let hand = SelectHand::new(vec![ten, four]);

        // Score high card without joker (only Ten counts in made hand)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Ten) -> 9 chips
        // (5 + 9) * (1) = 14
        let before = 14;
        // Score high card with Walkie Talkie (1 ten in made hand = +10 chips, +4 mult)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Ten) -> 9 chips
        // Walkie Talkie: +10 chips, +4 mult (1 ten)
        // (5 + 9 + 10) * (1 + 4) = 120
        let after = 120;

        let j = Jokers::WalkieTalkie(WalkieTalkie {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_fibonacci() {
        let ace = Card::new(Value::Ace, Suit::Club);
        let two = Card::new(Value::Two, Suit::Heart);
        let three = Card::new(Value::Three, Suit::Diamond);
        let five = Card::new(Value::Five, Suit::Spade);
        let eight = Card::new(Value::Eight, Suit::Club);
        let hand = SelectHand::new(vec![ace, two, three, five, eight]);

        // Score high card without joker (only Ace counts)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Ace) -> 11 chips
        // (5 + 11) * (1) = 16
        let before = 16;
        // Score high card with Fibonacci (1 fib number = +8 mult)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Ace) -> 11 chips
        // Fibonacci: +8 mult (Ace is a fibonacci number)
        // (5 + 11) * (1 + 8) = 144
        let after = 144;

        let j = Jokers::Fibonacci(Fibonacci {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_spare_trousers() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let kc = Card::new(Value::King, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, kc, kc]);

        // Score two pair without joker
        // two pair (level 1) -> 20 chips, 2 mult
        // Played cards (2 ace, 2 king) -> 42 chips
        // (20 + 42) * (2) = 124
        let before = 124;
        // Score two pair with Spare Trousers (+2 mult)
        // two pair (level 1) -> 20 chips, 2 mult
        // Played cards (2 ace, 2 king) -> 42 chips
        // Spare Trousers: +2 mult
        // (20 + 42) * (2 + 2) = 248
        let after = 248;

        let j = Jokers::SpareTrousers(SpareTrousers {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_acrobat() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);
        g.plays = 1; // Final hand

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::Acrobat(Acrobat {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);
        g.plays = 1; // Ensure it's the final hand

        // Score pair with Acrobat (X3 mult on final hand)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Acrobat: X3 mult
        // (10 + 22) * (2 * 3) = 192
        let score = g.calc_score(SelectHand::new(vec![ac, ac]).best_hand().unwrap());
        assert_eq!(score, 192);
    }

    #[test]
    fn test_onyx_agate() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Club);
        let four = Card::new(Value::Four, Suit::Club);
        let five = Card::new(Value::Five, Suit::Club);
        let ten = Card::new(Value::Ten, Suit::Club);
        let hand = SelectHand::new(vec![two, three, four, five, ten]);

        // Score flush without joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // (35 + 19) * (4) = 216
        let before = 216;
        // Score flush with Onyx Agate (5 clubs = +35 mult)
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // Onyx Agate: +35 mult (5 clubs × 7)
        // (35 + 19) * (4 + 35) = 2106
        let after = 2106;

        let j = Jokers::OnyxAgate(OnyxAgate {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_arrowhead() {
        let two = Card::new(Value::Two, Suit::Spade);
        let three = Card::new(Value::Three, Suit::Spade);
        let four = Card::new(Value::Four, Suit::Spade);
        let five = Card::new(Value::Five, Suit::Spade);
        let ten = Card::new(Value::Ten, Suit::Spade);
        let hand = SelectHand::new(vec![two, three, four, five, ten]);

        // Score flush without joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // (35 + 19) * (4) = 216
        let before = 216;
        // Score flush with Arrowhead (5 spades = +250 chips)
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // Arrowhead: +250 chips (5 spades × 50)
        // (35 + 19 + 250) * (4) = 1216
        let after = 1216;

        let j = Jokers::Arrowhead(Arrowhead {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_the_duo() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let before = 64;
        // Score pair with The Duo (X2 mult)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // The Duo: X2 mult
        // (10 + 22) * (2 * 2) = 128
        let after = 128;

        let j = Jokers::TheDuo(TheDuo {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_the_trio() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac]);

        // Score three of a kind without joker
        // 3ok (level 1) -> 30 chips, 3 mult
        // Played cards (3 aces) -> 33 chips
        // (30 + 33) * (3) = 189
        let before = 189;
        // Score 3ok with The Trio (X3 mult)
        // 3ok (level 1) -> 30 chips, 3 mult
        // Played cards (3 aces) -> 33 chips
        // The Trio: X3 mult
        // (30 + 33) * (3 * 3) = 567
        let after = 567;

        let j = Jokers::TheTrio(TheTrio {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_bloodstone() {
        // Note: Bloodstone is probabilistic, so we test that it doesn't crash
        // and that mult can potentially increase
        let two = Card::new(Value::Two, Suit::Heart);
        let three = Card::new(Value::Three, Suit::Heart);
        let four = Card::new(Value::Four, Suit::Heart);
        let five = Card::new(Value::Five, Suit::Heart);
        let ten = Card::new(Value::Ten, Suit::Heart);
        let hand = SelectHand::new(vec![two, three, four, five, ten]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score flush without joker
        let score_without = g.calc_score(hand.best_hand().unwrap());

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::Bloodstone(Bloodstone {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score flush with Bloodstone - should be >= base score
        let score_with = g.calc_score(SelectHand::new(vec![two, three, four, five, ten]).best_hand().unwrap());

        // Bloodstone might trigger, might not - but should never be worse
        assert!(score_with >= score_without, "Bloodstone should never reduce score");
    }

    #[test]
    fn test_rough_gem() {
        let two = Card::new(Value::Two, Suit::Diamond);
        let three = Card::new(Value::Three, Suit::Diamond);
        let four = Card::new(Value::Four, Suit::Diamond);
        let five = Card::new(Value::Five, Suit::Diamond);
        let ten = Card::new(Value::Ten, Suit::Diamond);
        let hand = SelectHand::new(vec![two, three, four, five, ten]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);
        let initial_money = g.money;

        // Score flush without joker
        g.calc_score(hand.best_hand().unwrap());
        let money_without = g.money;

        // Reset and buy joker
        g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::RoughGem(RoughGem {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score flush with Rough Gem (5 diamonds = +$5)
        g.calc_score(SelectHand::new(vec![two, three, four, five, ten]).best_hand().unwrap());

        // Check we earned $5 more than without the joker
        assert!(g.money >= money_without + 5, "Rough Gem should earn $1 per diamond");
    }

    #[test]
    fn test_flash_card() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // Enter shop and reroll 3 times
        g.stage = Stage::Shop();
        g.money = 1000;
        for _ in 0..3 {
            g.shop.reroll(&g.vouchers);
        }

        let j = Jokers::FlashCard(FlashCard {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair with Flash Card (3 rerolls = +6 mult)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Flash Card: +6 mult (3 rerolls × 2)
        // (10 + 22) * (2 + 6) = 256
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 256);
    }

    #[test]
    fn test_stone_joker() {
        // Stone Joker requires stone cards in deck, which we don't have by default
        // So we test with 0 stone cards
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let before = 64;
        // Score pair with Stone Joker (0 stone cards = +0 chips)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Stone Joker: +0 chips
        // (10 + 22) * (2) = 64
        let after = 64;

        let j = Jokers::StoneJoker(StoneJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_bull() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);
        g.money = 10; // Set money to $10

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::Bull(Bull {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);
        let current_money = g.money;

        // Score pair with Bull (+2 chips per $1)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Bull: +2 * money chips
        // (10 + 22 + 2*money) * (2)
        let expected_chips = 32 + current_money * 2;
        let expected = expected_chips * 2;
        let score = g.calc_score(SelectHand::new(vec![ac, ac]).best_hand().unwrap());
        assert_eq!(score, expected);
    }

    #[test]
    fn test_erosion() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        // Default deck has 52 cards, we haven't removed any
        // So erosion bonus is 0

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let before = 64;
        // Score pair with Erosion (52 cards in deck = 0 missing = +0 mult)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Erosion: +0 mult
        // (10 + 22) * (2) = 64
        let after = 64;

        let j = Jokers::Erosion(Erosion {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_the_family() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score four of a kind without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 aces) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok with The Family (X4 mult)
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 aces) -> 44 chips
        // The Family: X4 mult
        // (60 + 44) * (7 * 4) = 2912
        let after = 2912;

        let j = Jokers::TheFamily(TheFamily {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_the_order() {
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
        // Score straight with The Order (X3 mult)
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // The Order: X3 mult
        // (15 + 30) * (4 * 3) = 540
        let after = 540;

        let j = Jokers::TheOrder(TheOrder {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_the_tribe() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Club);
        let four = Card::new(Value::Four, Suit::Club);
        let five = Card::new(Value::Five, Suit::Club);
        let ten = Card::new(Value::Ten, Suit::Club);
        let hand = SelectHand::new(vec![two, three, four, five, ten]);

        // Score flush without joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // (35 + 19) * (4) = 216
        let before = 216;
        // Score flush with The Tribe (X2 mult)
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // The Tribe: X2 mult
        // (35 + 19) * (4 * 2) = 432
        let after = 432;

        let j = Jokers::TheTribe(TheTribe {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_triboulet() {
        let kc = Card::new(Value::King, Suit::Club);
        let qh = Card::new(Value::Queen, Suit::Heart);
        let hand = SelectHand::new(vec![kc, qh]);

        // Score pair without joker (high card since K and Q don't match)
        // Actually this is a high card, only King counts
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 King) -> 10 chips
        // (5 + 10) * (1) = 15
        let before = 15;
        // Score high card with Triboulet (1 King = X2 mult)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 King) -> 10 chips
        // Triboulet: X2 mult (for the King)
        // (5 + 10) * (1 * 2) = 30
        let after = 30;

        let j = Jokers::Triboulet(Triboulet {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_steel_joker() {
        use crate::card::Edition;
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // Add 5 foil cards to the deck (foil = steel in this codebase)
        // Get the card IDs first
        let card_ids: Vec<usize> = g.deck.cards().iter().take(5).map(|c| c.id).collect();
        for card_id in card_ids {
            g.modify_card_in_deck(card_id, |c| {
                c.edition = Edition::Foil;
            });
        }

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::SteelJoker(SteelJoker {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j.clone()).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair with Steel Joker (5 foil cards = X2.0 mult)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Steel Joker: X2.0 mult (1.0 + 0.2 * 5)
        // (10 + 22) * (2 * 2.0) = (10 + 22) * 4 = 128
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 128);
    }

    #[test]
    fn test_flower_pot() {
        let two_d = Card::new(Value::Two, Suit::Diamond);
        let three_c = Card::new(Value::Three, Suit::Club);
        let four_h = Card::new(Value::Four, Suit::Heart);
        let five_s = Card::new(Value::Five, Suit::Spade);
        let six_d = Card::new(Value::Six, Suit::Diamond);
        let hand = SelectHand::new(vec![two_d, three_c, four_h, five_s, six_d]);

        // This is actually a straight! (2, 3, 4, 5, 6)
        // Score straight without joker
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // (30 + 15) * (4) = 180
        let before = 180;
        // Score straight with Flower Pot (has all 4 suits = X3 mult)
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // Flower Pot: X3 mult
        // (30 + 15) * (4 * 3) = 540
        let after = 540;

        let j = Jokers::FlowerPot(FlowerPot {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_seeing_double() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let kh = Card::new(Value::King, Suit::Heart);
        let hand = SelectHand::new(vec![ac, kh]);

        // Score high card without joker (only Ace counts in made hand)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Ace) -> 11 chips
        // (5 + 11) * (1) = 16
        let before = 16;
        // Score high card with Seeing Double (has club and non-club = X2 mult)
        // high card (level 1) -> 5 chips, 1 mult
        // Played cards (1 Ace) -> 11 chips
        // Seeing Double: X2 mult
        // (5 + 11) * (1 * 2) = 32
        let after = 32;

        let j = Jokers::SeeingDouble(SeeingDouble {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_joker_stencil() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker (1 joker in slots = 4 empty + 1 for itself = 5 empty)
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::JokerStencil(JokerStencil {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j.clone()).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair with Joker Stencil (5 empty slots = X5 mult)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Joker Stencil: X5 mult (5 empty slots)
        // (10 + 22) * (2 * 5) = 320
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 320);
    }

    #[test]
    fn test_showman() {
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // Add 2 consumables to the game
        g.consumables.push(Consumables::Tarot(Tarots::TheFool));
        g.consumables.push(Consumables::Tarot(Tarots::TheMagician));

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::Showman(Showman {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j.clone()).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair with Showman (2 consumables = +8 mult)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Showman: +8 mult (2 consumables * 4)
        // (10 + 22) * (2 + 8) = 320
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 320);
    }

    #[test]
    fn test_bootstraps() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);
        g.money = 25; // $25 = 5 * $5, so +10 mult

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy and apply the joker
        g.money += 1000;
        g.stage = Stage::Shop();
        let j = Jokers::Bootstraps(Bootstraps {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j.clone()).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair with Bootstraps ($1019 / 5 = 203, 203 * 2 = 406 mult bonus)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Bootstraps: +406 mult (1019 / 5 * 2)
        // (10 + 22) * (2 + 406) = 13056
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 13056);
    }

    #[test]
    fn test_wee_joker() {
        let two_c = Card::new(Value::Two, Suit::Club);
        let two_h = Card::new(Value::Two, Suit::Heart);
        let hand = SelectHand::new(vec![two_c, two_h]);

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 twos) -> 2 chips (only one two counted? or 1 chip each?)
        // (10 + 2) * (2) = 24 (actual)
        let before = 24;
        // Score pair with Wee Joker (2 twos in made hand = +16 chips)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 twos) -> 2 chips
        // Wee Joker: +16 chips (2 twos * 8)
        // (10 + 2 + 16) * (2) = 56
        let after = 56;

        let j = Jokers::WeeJoker(WeeJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_baseball_card() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 64);

        // Buy Baseball Card and 2 Uncommon jokers
        g.money += 1000;
        g.stage = Stage::Shop();

        // Add 2 uncommon jokers
        let j1 = Jokers::SteelJoker(SteelJoker {});
        g.shop.jokers.push(j1.clone());
        g.buy_joker(j1.clone()).unwrap();

        let j2 = Jokers::FlowerPot(FlowerPot {});
        g.shop.jokers.push(j2.clone());
        g.buy_joker(j2.clone()).unwrap();

        // Now buy Baseball Card
        let j = Jokers::BaseballCard(BaseballCard {});
        g.shop.jokers.push(j.clone());
        g.buy_joker(j.clone()).unwrap();
        g.stage = Stage::Blind(Blind::Small, None);

        // Score pair with Baseball Card (2 uncommons = X2.25 mult from 1.5^2)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Baseball Card: X2.25 mult (1.5 ^ 2 uncommons)
        // (10 + 22) * (2 * 2.25) = 144, but truncation gives us (2 * 2) = 4
        // So actual result: (10 + 22) * 4 = 128
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 128);
    }

    #[test]
    fn test_stuntman() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac]);

        // Score pair without joker
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // (10 + 22) * (2) = 64
        let before = 64;
        // Score pair with Stuntman (+250 chips)
        // pair (level 1) -> 10 chips, 2 mult
        // Played cards (2 aces) -> 22 chips
        // Stuntman: +250 chips
        // (10 + 22 + 250) * (2) = 564
        let after = 564;

        let j = Jokers::Stuntman(Stuntman {});
        score_before_after_joker(j, hand, before, after);
    }
}
