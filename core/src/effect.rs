use crate::game::Game;
use crate::hand::MadeHand;
use crate::joker::{Joker, Jokers};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct EffectRegistry {
    pub on_play: Vec<Effects>,
    pub on_discard: Vec<Effects>,
    pub on_score: Vec<Effects>,
    pub on_handrank: Vec<Effects>,
    pub on_round_begin: Vec<Effects>,
    pub on_round_end: Vec<Effects>,
    pub on_blind_select: Vec<Effects>,
    pub on_sell: Vec<Effects>,
    pub on_pack_open: Vec<Effects>,      // For Hallucination joker
    pub on_shop_end: Vec<Effects>,       // For Perkeo joker
    pub on_boss_blind_trigger: Vec<Effects>, // For Matador joker
}

impl EffectRegistry {
    pub fn new() -> Self {
        return Self {
            on_play: Vec::new(),
            on_discard: Vec::new(),
            on_score: Vec::new(),
            on_handrank: Vec::new(),
            on_round_begin: Vec::new(),
            on_round_end: Vec::new(),
            on_blind_select: Vec::new(),
            on_sell: Vec::new(),
            on_pack_open: Vec::new(),
            on_shop_end: Vec::new(),
            on_boss_blind_trigger: Vec::new(),
        };
    }
    pub(crate) fn register_jokers(&mut self, jokers: Vec<Jokers>, game: &Game) {
        for (i, j) in jokers.iter().enumerate() {
            // Handle effect copying jokers specially
            let effects = match j {
                // Blueprint: Copy effects from joker to the right
                Jokers::Blueprint(_) => {
                    if i + 1 < jokers.len() {
                        jokers[i + 1].effects(game)
                    } else {
                        vec![]
                    }
                }
                // Brainstorm: Copy effects from leftmost joker
                Jokers::Brainstorm(_) => {
                    if i > 0 {
                        // Not the leftmost, copy from index 0
                        jokers[0].effects(game)
                    } else if jokers.len() > 1 {
                        // Brainstorm IS leftmost, copy from second joker
                        jokers[1].effects(game)
                    } else {
                        // Only Brainstorm exists
                        vec![]
                    }
                }
                // All other jokers: get their own effects
                _ => j.effects(game),
            };

            // Register the effects
            for e in effects {
                match e {
                    Effects::OnPlay(_) => self.on_play.push(e),
                    Effects::OnDiscard(_) => self.on_discard.push(e),
                    Effects::OnScore(_) => self.on_score.push(e),
                    Effects::OnHandRank(_) => self.on_handrank.push(e),
                    Effects::OnRoundBegin(_) => self.on_round_begin.push(e),
                    Effects::OnRoundEnd(_) => self.on_round_end.push(e),
                    Effects::OnBlindSelect(_) => self.on_blind_select.push(e),
                    Effects::OnSell(_) => self.on_sell.push(e),
                    Effects::OnPackOpen(_) => self.on_pack_open.push(e),
                    Effects::OnShopEnd(_) => self.on_shop_end.push(e),
                    Effects::OnBossBlindTrigger(_) => self.on_boss_blind_trigger.push(e),
                }
            }
        }
    }
}

#[derive(Clone)]
// signature of these callbacks are more complicated so they
// can be used by pyo3 as part of python class.
pub enum Effects {
    OnPlay(Arc<Mutex<dyn Fn(&mut Game, MadeHand) + Send + 'static>>),
    OnDiscard(Arc<Mutex<dyn Fn(&mut Game, MadeHand) + Send + 'static>>),
    OnScore(Arc<Mutex<dyn Fn(&mut Game, MadeHand) + Send + 'static>>),
    OnHandRank(Arc<Mutex<dyn Fn(&mut Game) + Send + 'static>>),
    OnRoundBegin(Arc<Mutex<dyn Fn(&mut Game) + Send + 'static>>),
    OnRoundEnd(Arc<Mutex<dyn Fn(&mut Game) + Send + 'static>>),
    OnBlindSelect(Arc<Mutex<dyn Fn(&mut Game) + Send + 'static>>),
    OnSell(Arc<Mutex<dyn Fn(&mut Game) + Send + 'static>>),
    OnPackOpen(Arc<Mutex<dyn Fn(&mut Game) + Send + 'static>>),      // Hallucination
    OnShopEnd(Arc<Mutex<dyn Fn(&mut Game) + Send + 'static>>),       // Perkeo
    OnBossBlindTrigger(Arc<Mutex<dyn Fn(&mut Game) + Send + 'static>>), // Matador
}

impl std::fmt::Debug for Effects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OnPlay(_) => write!(f, "OnPlay"),
            Self::OnDiscard(_) => write!(f, "OnDiscard"),
            Self::OnScore(_) => write!(f, "OnScore"),
            Self::OnHandRank(_) => write!(f, "OnHandRank"),
            Self::OnRoundBegin(_) => write!(f, "OnRoundBegin"),
            Self::OnRoundEnd(_) => write!(f, "OnRoundEnd"),
            Self::OnBlindSelect(_) => write!(f, "OnBlindSelect"),
            Self::OnSell(_) => write!(f, "OnSell"),
            Self::OnPackOpen(_) => write!(f, "OnPackOpen"),
            Self::OnShopEnd(_) => write!(f, "OnShopEnd"),
            Self::OnBossBlindTrigger(_) => write!(f, "OnBossBlindTrigger"),
        }
    }
}
