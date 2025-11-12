use crate::card::{Card, Suit, Value};
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

// Import sub-modules
mod common;
mod uncommon;
mod rare;
mod legendary;

// Re-export all joker structs
pub use common::*;
pub use uncommon::*;
pub use rare::*;
pub use legendary::*;

// Create the main Jokers enum with all variants
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
    HitTheRoad,
    Satellite,
    Throwback,
    LoyaltyCard,
    Campfire,
    Hologram,
    Obelisk,
    TheIdol,
    SpaceJoker,
    Burglar,
    Rocket,
    MerryAndy,
    OopsAll6s,
    Ramen,
    Castle,
    GlassJoker,
    LuckyCat,
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
    Chicot,
    Shortcut,
    Troubadour,
    TurtleBean,
    TradingCard,
    Matador,
    ToTheMoon,
    Vagabond,
    Seance,
    MrBones,
    Luchador,
    DietCola,
    CeremonialDagger,
    Cartomancer,
    Astronomer,
    Vampire,
    DriverLicense,
    BurntJoker,
    InvisibleJoker,
    Brainstorm,
    Perkeo,
    DNA,
    Hack,
    Dusk,
    SockAndBuskin,
    Seltzer,
    MidasMask,
    Madness,
    Certificate,
    GiftCard,
    Hallucination
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

// Tests in separate module
#[cfg(test)]
mod tests;
