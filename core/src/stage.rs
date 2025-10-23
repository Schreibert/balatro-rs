#[cfg(feature = "python")]
use pyo3::{pyclass, pymethods};
use std::fmt;

use crate::boss_modifier::BossModifier;

/// Types of blinds
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum Blind {
    Small,
    Big,
    Boss,
}

impl Blind {
    /// reward is money earned for beating the blind
    pub fn reward(&self) -> usize {
        match self {
            Self::Small => 3,
            Self::Big => 4,
            Self::Boss => 5,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Small => Self::Big,
            Self::Big => Self::Boss,
            Self::Boss => Self::Small,
        }
    }
}

impl fmt::Display for Blind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Small => write!(f, "Small Blind"),
            Self::Big => write!(f, "Big Blind"),
            Self::Boss => write!(f, "Boss Blind"),
        }
    }
}

/// Game ending
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum End {
    Win,
    Lose,
}

/// Stages of playing.
// Playing through an ante looks like:
// Pre -> Small -> Post -> Shop -> Pre -> Big -> Post -> Shop -> Boss -> Post -> Shop
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum Stage {
    // See blind conditions, choose blind (or skip blind)
    PreBlind(),
    // Play blind (with optional boss modifier for Boss blinds)
    Blind(Blind, Option<BossModifier>),
    // Collect payout, optionally play consumables
    PostBlind(),
    // Buy jokers, consumables
    Shop(),
    // Game ending
    End(End),
}

impl Stage {
    pub(crate) fn is_blind(&self) -> bool {
        return match self {
            Stage::Blind(_, _) => true,
            _ => false,
        };
    }

    /// Get the blind if this is a Blind stage
    pub fn blind(&self) -> Option<Blind> {
        match self {
            Stage::Blind(blind, _) => Some(*blind),
            _ => None,
        }
    }

    /// Get the boss modifier if this is a Boss Blind stage
    pub fn boss_modifier(&self) -> Option<BossModifier> {
        match self {
            Stage::Blind(Blind::Boss, modifier) => *modifier,
            _ => None,
        }
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl Stage {
    fn int(&self) -> usize {
        match self {
            Self::PreBlind() => 0,
            Self::Blind(blind, _) => match blind {
                Blind::Small => 1,
                Blind::Big => 2,
                Blind::Boss => 3,
            },
            Self::PostBlind() => 4,
            Self::Shop() => 5,
            Self::End(end) => match end {
                End::Win => 6,
                End::Lose => 7,
            },
        }
    }
}
