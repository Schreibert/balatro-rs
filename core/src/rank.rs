#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Level {
    pub level: usize,
    pub chips: usize,
    pub mult: usize,
}

impl Level {
    /// Create a new level with specified values
    pub fn new(level: usize, chips: usize, mult: usize) -> Self {
        Self { level, chips, mult }
    }

    /// Upgrade this level to the next level using Balatro formula:
    /// Level 1→2: +30 chips, +3 mult
    /// Level 2→3: +25 chips, +2 mult
    /// Level 3+: +20 chips, +2 mult
    pub fn upgrade(&self) -> Self {
        let (chip_bonus, mult_bonus) = match self.level {
            1 => (30, 3),
            2 => (25, 2),
            _ => (20, 2),
        };
        Self {
            level: self.level + 1,
            chips: self.chips + chip_bonus,
            mult: self.mult + mult_bonus,
        }
    }

    /// Downgrade this level to the previous level (reverse of upgrade)
    /// Used by The Arm boss modifier
    pub fn downgrade(&self) -> Self {
        if self.level <= 1 {
            return *self; // Can't downgrade below level 1
        }
        let (chip_penalty, mult_penalty) = match self.level {
            2 => (30, 3), // Level 2→1
            3 => (25, 2), // Level 3→2
            _ => (20, 2), // Level 4+→previous
        };
        Self {
            level: self.level - 1,
            chips: self.chips.saturating_sub(chip_penalty),
            mult: self.mult.saturating_sub(mult_penalty),
        }
    }
}

/// All the different possible hand ranks.
/// For each hand rank the u32 corresponds to
/// the strength of the hand in comparison to others
/// of the same rank.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

impl HandRank {
    pub(crate) fn level(&self) -> Level {
        match self {
            Self::HighCard => Level {
                level: 1,
                chips: 5,
                mult: 1,
            },
            Self::OnePair => Level {
                level: 1,
                chips: 10,
                mult: 2,
            },
            Self::TwoPair => Level {
                level: 1,
                chips: 20,
                mult: 2,
            },
            Self::ThreeOfAKind => Level {
                level: 1,
                chips: 30,
                mult: 3,
            },
            Self::Straight => Level {
                level: 1,
                chips: 30,
                mult: 4,
            },
            Self::Flush => Level {
                level: 1,
                chips: 35,
                mult: 4,
            },
            Self::FullHouse => Level {
                level: 1,
                chips: 40,
                mult: 4,
            },
            Self::FourOfAKind => Level {
                level: 1,
                chips: 60,
                mult: 7,
            },
            Self::StraightFlush => Level {
                level: 1,
                chips: 100,
                mult: 8,
            },
            Self::RoyalFlush => Level {
                level: 1,
                chips: 100,
                mult: 8,
            },
            Self::FiveOfAKind => Level {
                level: 1,
                chips: 120,
                mult: 12,
            },
            Self::FlushHouse => Level {
                level: 1,
                chips: 140,
                mult: 14,
            },
            Self::FlushFive => Level {
                level: 1,
                chips: 160,
                mult: 16,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_new() {
        let level = Level::new(1, 10, 2);
        assert_eq!(level.level, 1);
        assert_eq!(level.chips, 10);
        assert_eq!(level.mult, 2);
    }

    #[test]
    fn test_level_upgrade_1_to_2() {
        // Level 1→2: +30 chips, +3 mult
        let level1 = Level::new(1, 10, 2);
        let level2 = level1.upgrade();
        assert_eq!(level2.level, 2);
        assert_eq!(level2.chips, 40); // 10 + 30
        assert_eq!(level2.mult, 5); // 2 + 3
    }

    #[test]
    fn test_level_upgrade_2_to_3() {
        // Level 2→3: +25 chips, +2 mult
        let level2 = Level::new(2, 40, 5);
        let level3 = level2.upgrade();
        assert_eq!(level3.level, 3);
        assert_eq!(level3.chips, 65); // 40 + 25
        assert_eq!(level3.mult, 7); // 5 + 2
    }

    #[test]
    fn test_level_upgrade_3_to_4() {
        // Level 3+: +20 chips, +2 mult
        let level3 = Level::new(3, 65, 7);
        let level4 = level3.upgrade();
        assert_eq!(level4.level, 4);
        assert_eq!(level4.chips, 85); // 65 + 20
        assert_eq!(level4.mult, 9); // 7 + 2
    }

    #[test]
    fn test_level_upgrade_chain() {
        // Test upgrading from level 1 through level 5
        let mut level = Level::new(1, 10, 2);

        // 1→2
        level = level.upgrade();
        assert_eq!(level, Level::new(2, 40, 5));

        // 2→3
        level = level.upgrade();
        assert_eq!(level, Level::new(3, 65, 7));

        // 3→4
        level = level.upgrade();
        assert_eq!(level, Level::new(4, 85, 9));

        // 4→5
        level = level.upgrade();
        assert_eq!(level, Level::new(5, 105, 11));
    }

    #[test]
    fn test_hand_rank_default_levels() {
        // Verify default level 1 values for all hand ranks
        assert_eq!(HandRank::HighCard.level(), Level::new(1, 5, 1));
        assert_eq!(HandRank::OnePair.level(), Level::new(1, 10, 2));
        assert_eq!(HandRank::TwoPair.level(), Level::new(1, 20, 2));
        assert_eq!(HandRank::ThreeOfAKind.level(), Level::new(1, 30, 3));
        assert_eq!(HandRank::Straight.level(), Level::new(1, 30, 4));
        assert_eq!(HandRank::Flush.level(), Level::new(1, 35, 4));
        assert_eq!(HandRank::FullHouse.level(), Level::new(1, 40, 4));
        assert_eq!(HandRank::FourOfAKind.level(), Level::new(1, 60, 7));
        assert_eq!(HandRank::StraightFlush.level(), Level::new(1, 100, 8));
        assert_eq!(HandRank::RoyalFlush.level(), Level::new(1, 100, 8));
        assert_eq!(HandRank::FiveOfAKind.level(), Level::new(1, 120, 12));
        assert_eq!(HandRank::FlushHouse.level(), Level::new(1, 140, 14));
        assert_eq!(HandRank::FlushFive.level(), Level::new(1, 160, 16));
    }
}
