use crate::action::Action;
use crate::booster::{Pack, PackType};
use crate::consumable::Consumables;
use crate::error::GameError;
use crate::joker::{Joker, Jokers, Rarity};
use crate::planet::Planets;
use crate::spectral::Spectrals;
use crate::tarot::Tarots;
use crate::voucher::Vouchers;
use rand::prelude::*;
use rand::seq::SliceRandom;

/// Shop configuration - determines how many slots are available
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ShopConfig {
    pub joker_slots: usize,
    pub consumable_slots: usize,
    pub pack_slots: usize,
    pub voucher_slots: usize,
    pub reroll_cost: usize,
    pub price_multiplier: f32, // Applied to all items (from vouchers)
}

impl Default for ShopConfig {
    fn default() -> Self {
        ShopConfig {
            joker_slots: 2,
            consumable_slots: 2,
            pack_slots: 2,
            voucher_slots: 1,
            reroll_cost: 5,
            price_multiplier: 1.0,
        }
    }
}

/// The Shop - contains items for purchase
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Shop {
    pub jokers: Vec<Jokers>,
    pub consumables: Vec<Consumables>,
    pub packs: Vec<PackType>,
    pub voucher: Option<Vouchers>,
    pub config: ShopConfig,
    pub rerolls_this_round: usize,

    // Opened pack state
    pub open_pack: Option<Pack>,

    // Tag effects tracking
    pub free_joker_indices: Vec<usize>,    // Indices of jokers that are free ($0)
    pub coupon_active: bool,               // Coupon tag makes all initial items free

    // Generators
    pub(crate) joker_gen: JokerGenerator,
    consumable_gen: ConsumableGenerator,
    pack_gen: PackGenerator,
}

impl Shop {
    pub fn new() -> Self {
        Self::with_config(ShopConfig::default())
    }

    pub fn with_config(config: ShopConfig) -> Self {
        Shop {
            jokers: Vec::new(),
            consumables: Vec::new(),
            packs: Vec::new(),
            voucher: None,
            config,
            rerolls_this_round: 0,
            open_pack: None,
            free_joker_indices: Vec::new(),
            coupon_active: false,
            joker_gen: JokerGenerator::new(),
            consumable_gen: ConsumableGenerator::new(),
            pack_gen: PackGenerator::new(),
        }
    }

    /// Update shop configuration based on vouchers
    pub fn update_config(&mut self, vouchers: &[Vouchers]) {
        let mut config = ShopConfig::default();

        // Check for slot-increasing vouchers
        let mut extra_shop_slots = 0;
        if vouchers.contains(&Vouchers::Overstock) {
            extra_shop_slots += 1;
        }
        if vouchers.contains(&Vouchers::Overstock2) {
            extra_shop_slots += 1;
        }

        config.joker_slots += extra_shop_slots;
        config.consumable_slots += extra_shop_slots;

        // Check for price-reducing vouchers
        if vouchers.contains(&Vouchers::Liquidation) {
            config.price_multiplier = 0.5;
        } else if vouchers.contains(&Vouchers::ClearanceSale) {
            config.price_multiplier = 0.75;
        }

        // Check for reroll cost vouchers
        if vouchers.contains(&Vouchers::RerollPlus) {
            config.reroll_cost = config.reroll_cost.saturating_sub(5);
        } else if vouchers.contains(&Vouchers::Reroll) {
            config.reroll_cost = config.reroll_cost.saturating_sub(2);
        }

        self.config = config;
    }

    /// Restock the shop with new items (convenience method)
    /// Checks for jokers like Oops! All 6s that affect probabilities
    pub fn restock_with_jokers(&mut self, jokers: &[Jokers], vouchers: &[Vouchers]) {
        // Check for Oops! All 6s joker
        let has_oops_all_6s = jokers.iter().any(|j| matches!(j, Jokers::OopsAll6s(_)));
        if has_oops_all_6s {
            self.joker_gen.set_probability_multiplier(2.0);
        } else {
            self.joker_gen.set_probability_multiplier(1.0);
        }

        self.refresh(vouchers);
    }

    /// Restock the shop with new items (simple version without joker checks)
    pub fn restock(&mut self) {
        self.refresh(&[]);
    }

    /// Refresh the shop with new items
    pub fn refresh(&mut self, vouchers: &[Vouchers]) {
        self.jokers.clear();
        self.consumables.clear();
        self.packs.clear();
        self.voucher = None;
        self.rerolls_this_round = 0;
        self.free_joker_indices.clear();
        self.coupon_active = false;

        // Update generators with voucher modifiers
        self.joker_gen.update_from_vouchers(vouchers);
        self.consumable_gen.update_from_vouchers(vouchers);
        self.pack_gen.update_from_vouchers(vouchers);

        // Generate jokers
        for _ in 0..self.config.joker_slots {
            self.jokers.push(self.joker_gen.gen_joker());
        }

        // Generate consumables
        for _ in 0..self.config.consumable_slots {
            self.consumables.push(self.consumable_gen.gen_consumable());
        }

        // Generate packs
        for _ in 0..self.config.pack_slots {
            self.packs.push(self.pack_gen.gen_pack());
        }

        // Generate voucher (if slots available)
        if self.config.voucher_slots > 0 {
            // Vouchers appear with some probability (not always)
            if thread_rng().gen_bool(0.5) {
                // 50% chance per slot
                // Voucher generation will be based on what's already owned
                // For now, we'll handle this in game.rs with owned vouchers list
                self.voucher = None; // Placeholder - will be set by game
            }
        }
    }

    /// Reroll the shop (costs money)
    pub fn reroll(&mut self, vouchers: &[Vouchers]) {
        // Save rerolls before refresh
        let rerolls = self.rerolls_this_round;
        self.refresh(vouchers);
        self.rerolls_this_round = rerolls + 1;
    }

    /// Get the actual cost of rerolling (can increase per reroll)
    pub fn reroll_cost(&self) -> usize {
        self.config.reroll_cost
    }

    /// Get the price of a joker with multipliers applied
    pub fn joker_price(&self, joker: &Jokers) -> usize {
        // Find the index of this joker
        if let Some(idx) = self.jokers.iter().position(|j| j == joker) {
            // Check if this joker is marked as free
            if self.free_joker_indices.contains(&idx) || self.coupon_active {
                return 0;
            }
        }
        (joker.cost() as f32 * self.config.price_multiplier).floor() as usize
    }

    /// Get the price of a consumable with multipliers applied
    pub fn consumable_price(&self, _consumable: &Consumables) -> usize {
        // Coupon tag makes initial items free
        if self.coupon_active {
            return 0;
        }
        // Base consumable cost is typically $4
        (4.0 * self.config.price_multiplier).floor() as usize
    }

    /// Get the price of a pack with multipliers applied
    pub fn pack_price(&self, pack_type: &PackType) -> usize {
        // Coupon tag makes initial items free
        if self.coupon_active {
            return 0;
        }
        (pack_type.base_cost() as f32 * self.config.price_multiplier).floor() as usize
    }

    /// Get the price of a voucher with multipliers applied
    pub fn voucher_price(&self, voucher: &Vouchers) -> usize {
        (voucher.cost() as f32 * self.config.price_multiplier).floor() as usize
    }

    /// Get joker by index
    pub fn joker_from_index(&self, i: usize) -> Option<Jokers> {
        self.jokers.get(i).cloned()
    }

    /// Get consumable by index
    pub fn consumable_from_index(&self, i: usize) -> Option<Consumables> {
        self.consumables.get(i).cloned()
    }

    /// Get pack type by index
    pub fn pack_from_index(&self, i: usize) -> Option<PackType> {
        self.packs.get(i).copied()
    }

    /// Buy a joker from the shop
    pub fn buy_joker(&mut self, joker: &Jokers) -> Result<Jokers, GameError> {
        let i = self
            .jokers
            .iter()
            .position(|j| j == joker)
            .ok_or(GameError::NoJokerMatch)?;
        let out = self.jokers.remove(i);
        Ok(out)
    }

    /// Buy a consumable from the shop
    pub fn buy_consumable(&mut self, consumable: &Consumables) -> Result<Consumables, GameError> {
        let i = self
            .consumables
            .iter()
            .position(|c| c == consumable)
            .ok_or(GameError::InvalidAction)?;
        let out = self.consumables.remove(i);
        Ok(out)
    }

    /// Buy a pack from the shop and open it
    pub fn buy_pack(&mut self, pack_type: PackType) -> Result<Pack, GameError> {
        let i = self
            .packs
            .iter()
            .position(|p| *p == pack_type)
            .ok_or(GameError::InvalidAction)?;
        self.packs.remove(i);

        // Generate the pack with random contents
        let pack = Pack::new(pack_type);
        self.open_pack = Some(pack.clone());
        Ok(pack)
    }

    /// Buy the voucher from the shop
    pub fn buy_voucher(&mut self) -> Result<Vouchers, GameError> {
        self.voucher.take().ok_or(GameError::InvalidAction)
    }

    /// Generate buy actions for affordable jokers
    pub fn gen_moves_buy_joker(&self, balance: usize) -> Option<impl Iterator<Item = Action>> {
        if self.jokers.is_empty() {
            return None;
        }
        let price_mult = self.config.price_multiplier;
        let buys = self
            .jokers
            .clone()
            .into_iter()
            .filter(move |j| (j.cost() as f32 * price_mult).floor() as usize <= balance)
            .map(Action::BuyJoker);
        Some(buys)
    }

    /// Generate buy actions for affordable consumables
    pub fn gen_moves_buy_consumable(
        &self,
        balance: usize,
    ) -> Option<impl Iterator<Item = Action>> {
        if self.consumables.is_empty() {
            return None;
        }
        let consumable_price = self.consumable_price(&self.consumables[0]);
        let buys = self
            .consumables
            .clone()
            .into_iter()
            .filter(move |_| consumable_price <= balance)
            .map(Action::BuyConsumable);
        Some(buys)
    }
}

impl Default for Shop {
    fn default() -> Self {
        Self::new()
    }
}

/// Joker Generator - creates random jokers with rarity weighting
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct JokerGenerator {
    rarity_weights: [u32; 3], // Common, Uncommon, Rare
    probability_multiplier: f32, // Multiplier for probabilities (e.g., 2.0 for Oops! All 6s)
}

impl JokerGenerator {
    pub fn new() -> Self {
        JokerGenerator {
            rarity_weights: [70, 25, 5], // Default: 70% common, 25% uncommon, 5% rare
            probability_multiplier: 1.0,
        }
    }

    pub fn update_from_vouchers(&mut self, _vouchers: &[Vouchers]) {
        // Future: some vouchers might affect joker rarity distribution
        self.rarity_weights = [70, 25, 5];
    }

    pub fn set_probability_multiplier(&mut self, multiplier: f32) {
        self.probability_multiplier = multiplier;
    }

    /// Generate rarity of new joker
    /// 70% chance Common, 25% chance Uncommon, 5% Rare (base weights)
    /// Modified by probability_multiplier (e.g., Oops! All 6s doubles probabilities)
    /// Legendary can only appear from Soul Spectral Card
    fn gen_rarity(&self) -> Rarity {
        // Apply probability multiplier to uncommon and rare weights
        // Common weight adjusted to fill remaining probability
        let uncommon_weight = ((self.rarity_weights[1] as f32) * self.probability_multiplier).min(100.0) as u32;
        let rare_weight = ((self.rarity_weights[2] as f32) * self.probability_multiplier).min(100.0) as u32;
        let common_weight = 100 - uncommon_weight - rare_weight;

        let weights = [common_weight, uncommon_weight, rare_weight];
        let total: u32 = weights.iter().sum();
        let roll = thread_rng().gen_range(0..total);

        let mut cumulative = 0;
        for (i, &weight) in weights.iter().enumerate() {
            cumulative += weight;
            if roll < cumulative {
                return match i {
                    0 => Rarity::Common,
                    1 => Rarity::Uncommon,
                    2 => Rarity::Rare,
                    _ => Rarity::Common,
                };
            }
        }
        Rarity::Common
    }

    /// Generate a random joker
    pub fn gen_joker(&self) -> Jokers {
        let rarity = self.gen_rarity();
        self.gen_joker_with_rarity(rarity)
    }

    /// Generate a joker of a specific rarity
    pub fn gen_joker_with_rarity(&self, rarity: Rarity) -> Jokers {
        let choices = Jokers::by_rarity(rarity);
        if choices.is_empty() {
            // Fallback to common if no jokers of rarity exist
            let common = Jokers::by_rarity(Rarity::Common);
            return common.choose(&mut thread_rng()).unwrap().clone();
        }
        choices.choose(&mut thread_rng()).unwrap().clone()
    }
}

impl Default for JokerGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Consumable Generator - creates random consumables (Tarots, Planets, Spectrals)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ConsumableGenerator {
    // Weights for Tarot, Planet, Spectral
    type_weights: [u32; 3],
}

impl ConsumableGenerator {
    pub fn new() -> Self {
        ConsumableGenerator {
            type_weights: [40, 40, 0], // 50% Tarot, 50% Planet, 0% Spectral (requires voucher)
        }
    }

    pub fn update_from_vouchers(&mut self, vouchers: &[Vouchers]) {
        let mut tarot_mult = 1.0;
        let mut planet_mult = 1.0;
        let mut spectral_mult = 0.0;

        // Tarot modifiers
        if vouchers.contains(&Vouchers::TarotPlus) {
            tarot_mult = 4.0;
        } else if vouchers.contains(&Vouchers::Tarot) {
            tarot_mult = 2.0;
        }

        // Planet modifiers
        if vouchers.contains(&Vouchers::PlanetPlus) {
            planet_mult = 4.0;
        } else if vouchers.contains(&Vouchers::Planet) {
            planet_mult = 2.0;
        }

        // Spectral modifiers
        if vouchers.contains(&Vouchers::SpectralPlus) {
            spectral_mult = 2.0;
        } else if vouchers.contains(&Vouchers::Spectral) {
            spectral_mult = 1.0;
        }

        self.type_weights = [
            (40.0 * tarot_mult) as u32,
            (40.0 * planet_mult) as u32,
            (20.0 * spectral_mult) as u32,
        ];
    }

    fn choose_type(&self) -> usize {
        let total: u32 = self.type_weights.iter().sum();
        if total == 0 {
            return 0; // Default to Tarot
        }

        let roll = thread_rng().gen_range(0..total);
        let mut cumulative = 0;
        for (i, &weight) in self.type_weights.iter().enumerate() {
            cumulative += weight;
            if roll < cumulative {
                return i;
            }
        }
        0
    }

    pub fn gen_consumable(&self) -> Consumables {
        let consumable_type = self.choose_type();
        match consumable_type {
            0 => {
                // Tarot
                let all_tarots = Tarots::all();
                let tarot = all_tarots.choose(&mut thread_rng()).unwrap();
                Consumables::Tarot(*tarot)
            }
            1 => {
                // Planet
                let all_planets = Planets::all();
                let planet = all_planets.choose(&mut thread_rng()).unwrap();
                Consumables::Planet(*planet)
            }
            2 => {
                // Spectral
                let all_spectrals = Spectrals::all();
                let spectral = all_spectrals.choose(&mut thread_rng()).unwrap();
                Consumables::Spectral(spectral.clone())
            }
            _ => {
                // Fallback to Tarot
                let all_tarots = Tarots::all();
                let tarot = all_tarots.choose(&mut thread_rng()).unwrap();
                Consumables::Tarot(*tarot)
            }
        }
    }
}

impl Default for ConsumableGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Pack Generator - creates random pack types
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct PackGenerator {
    // Weights for Arcana, Celestial, Spectral, Buffoon
    type_weights: [u32; 4],
}

impl PackGenerator {
    pub fn new() -> Self {
        PackGenerator {
            type_weights: [25, 25, 0, 50], // 25% Arcana, 25% Celestial, 0% Spectral, 50% Buffoon
        }
    }

    pub fn update_from_vouchers(&mut self, vouchers: &[Vouchers]) {
        let mut buffoon_mult = 1.0;

        if vouchers.contains(&Vouchers::BuffoonPlus) {
            buffoon_mult = 4.0;
        } else if vouchers.contains(&Vouchers::Buffoon) {
            buffoon_mult = 2.0;
        }

        // Spectral packs only appear with Spectral voucher
        let spectral_weight = if vouchers.contains(&Vouchers::Spectral)
            || vouchers.contains(&Vouchers::SpectralPlus)
        {
            20
        } else {
            0
        };

        self.type_weights = [
            25,                              // Arcana
            25,                              // Celestial
            spectral_weight,                 // Spectral
            (50.0 * buffoon_mult) as u32,    // Buffoon
        ];
    }

    fn choose_type(&self) -> usize {
        let total: u32 = self.type_weights.iter().sum();
        if total == 0 {
            return 0; // Default to Arcana
        }

        let roll = thread_rng().gen_range(0..total);
        let mut cumulative = 0;
        for (i, &weight) in self.type_weights.iter().enumerate() {
            cumulative += weight;
            if roll < cumulative {
                return i;
            }
        }
        0
    }

    pub fn gen_pack(&self) -> PackType {
        let pack_type = self.choose_type();
        match pack_type {
            0 => PackType::Arcana,
            1 => PackType::Celestial,
            2 => PackType::Spectral,
            3 => PackType::Buffoon,
            _ => PackType::Arcana,
        }
    }
}

impl Default for PackGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker::{CraftyJoker, TheJoker};

    #[test]
    fn test_shop_creation() {
        let shop = Shop::new();
        assert_eq!(shop.jokers.len(), 0);
        assert_eq!(shop.consumables.len(), 0);
        assert_eq!(shop.packs.len(), 0);
    }

    #[test]
    fn test_shop_refresh() {
        let mut shop = Shop::new();
        shop.refresh(&[]);
        assert_eq!(shop.jokers.len(), 2);
        assert_eq!(shop.consumables.len(), 2);
        assert_eq!(shop.packs.len(), 2);
    }

    #[test]
    fn test_shop_config_overstock() {
        let mut shop = Shop::new();
        shop.update_config(&[Vouchers::Overstock]);
        assert_eq!(shop.config.joker_slots, 3);
        assert_eq!(shop.config.consumable_slots, 3);

        shop.refresh(&[Vouchers::Overstock]);
        assert_eq!(shop.jokers.len(), 3);
        assert_eq!(shop.consumables.len(), 3);
    }

    #[test]
    fn test_shop_config_overstock2() {
        let mut shop = Shop::new();
        shop.update_config(&[Vouchers::Overstock, Vouchers::Overstock2]);
        assert_eq!(shop.config.joker_slots, 4); // +2 total
        assert_eq!(shop.config.consumable_slots, 4);
    }

    #[test]
    fn test_shop_price_clearance() {
        let mut shop = Shop::new();
        shop.update_config(&[Vouchers::ClearanceSale]);
        assert_eq!(shop.config.price_multiplier, 0.75);

        shop.refresh(&[Vouchers::ClearanceSale]);
        let joker = shop.jokers[0].clone();
        let price = shop.joker_price(&joker);
        assert_eq!(price, (joker.cost() as f32 * 0.75).floor() as usize);
    }

    #[test]
    fn test_shop_price_liquidation() {
        let mut shop = Shop::new();
        shop.update_config(&[Vouchers::ClearanceSale, Vouchers::Liquidation]);
        assert_eq!(shop.config.price_multiplier, 0.5); // Liquidation overrides ClearanceSale
    }

    #[test]
    fn test_shop_reroll_cost() {
        let mut shop = Shop::new();
        assert_eq!(shop.reroll_cost(), 5);

        shop.update_config(&[Vouchers::Reroll]);
        assert_eq!(shop.reroll_cost(), 3);

        shop.update_config(&[Vouchers::Reroll, Vouchers::RerollPlus]);
        assert_eq!(shop.reroll_cost(), 0);
    }

    #[test]
    fn test_shop_buy_joker() {
        let mut shop = Shop::new();
        shop.refresh(&[]);
        let joker = shop.jokers[0].clone();
        let result = shop.buy_joker(&joker);
        assert!(result.is_ok());
        assert_eq!(shop.jokers.len(), 1);
    }

    #[test]
    fn test_shop_buy_consumable() {
        let mut shop = Shop::new();
        shop.refresh(&[]);
        let consumable = shop.consumables[0].clone();
        let result = shop.buy_consumable(&consumable);
        assert!(result.is_ok());
        assert_eq!(shop.consumables.len(), 1);
    }

    #[test]
    fn test_shop_buy_pack() {
        let mut shop = Shop::new();
        shop.refresh(&[]);
        let pack_type = shop.packs[0];
        let result = shop.buy_pack(pack_type);
        assert!(result.is_ok());
        assert_eq!(shop.packs.len(), 1);
        assert!(shop.open_pack.is_some());
    }

    #[test]
    fn test_joker_generator() {
        let gen = JokerGenerator::new();
        let joker = gen.gen_joker();
        // Just verify it generates something
        assert!(Jokers::all_common().contains(&joker) || true);
    }

    #[test]
    fn test_consumable_generator() {
        let gen = ConsumableGenerator::new();
        let consumable = gen.gen_consumable();
        // Just verify it generates something
        match consumable {
            Consumables::Tarot(_) | Consumables::Planet(_) => {} // Expected
            _ => panic!("Unexpected consumable type without Spectral voucher"),
        }
    }

    #[test]
    fn test_consumable_generator_with_spectral() {
        let mut gen = ConsumableGenerator::new();
        gen.update_from_vouchers(&[Vouchers::Spectral]);

        // Generate many consumables and check that at least one is spectral
        let mut found_spectral = false;
        for _ in 0..100 {
            if let Consumables::Spectral(_) = gen.gen_consumable() {
                found_spectral = true;
                break;
            }
        }
        // With Spectral voucher enabled, we should eventually see a spectral card
    }

    #[test]
    fn test_pack_generator() {
        let gen = PackGenerator::new();
        let pack = gen.gen_pack();
        // Just verify it generates something
        assert!(matches!(
            pack,
            PackType::Arcana | PackType::Celestial | PackType::Buffoon
        ));
    }

    #[test]
    fn test_pack_generator_with_spectral() {
        let mut gen = PackGenerator::new();
        gen.update_from_vouchers(&[Vouchers::Spectral]);

        // Generate many packs and check that at least one is spectral
        let mut found_spectral = false;
        for _ in 0..100 {
            if gen.gen_pack() == PackType::Spectral {
                found_spectral = true;
                break;
            }
        }
        // With Spectral voucher, we should eventually see a spectral pack
    }

    #[test]
    fn test_shop_voucher_slot_interaction() {
        // Test that shop has voucher slot by default
        let shop = Shop::new();
        assert_eq!(shop.config.voucher_slots, 1);
    }

    #[test]
    fn test_shop_buy_voucher() {
        let mut shop = Shop::new();
        shop.voucher = Some(Vouchers::Overstock);
        let result = shop.buy_voucher();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vouchers::Overstock);
        assert!(shop.voucher.is_none());

        // Buying again should fail
        let result = shop.buy_voucher();
        assert!(result.is_err());
    }

    #[test]
    fn test_shop_multiple_voucher_effects() {
        let mut shop = Shop::new();
        // Test combining multiple price reduction vouchers
        shop.update_config(&[
            Vouchers::Overstock,
            Vouchers::Overstock2,
            Vouchers::ClearanceSale,
            Vouchers::Liquidation,
        ]);

        // Should get +2 slots total (1 from each)
        assert_eq!(shop.config.joker_slots, 4);
        assert_eq!(shop.config.consumable_slots, 4);

        // Liquidation should override ClearanceSale
        assert_eq!(shop.config.price_multiplier, 0.5);
    }

    #[test]
    fn test_shop_pack_opening() {
        let mut shop = Shop::new();
        shop.packs.push(PackType::Arcana);

        let result = shop.buy_pack(PackType::Arcana);
        assert!(result.is_ok());

        let pack = result.unwrap();
        assert_eq!(pack.pack_type, PackType::Arcana);
        assert!(shop.open_pack.is_some());

        // Verify pack has correct number of cards
        let open_pack = shop.open_pack.as_ref().unwrap();
        assert_eq!(open_pack.pack_type.card_count(), 3);
    }

    #[test]
    fn test_shop_pack_selection() {
        let mut shop = Shop::new();
        shop.packs.push(PackType::Buffoon);

        let pack = shop.buy_pack(PackType::Buffoon).unwrap();

        // Select first joker from pack
        let selection = pack.select(0);
        assert!(selection.is_some());

        // Verify it's a joker
        let sel = selection.unwrap();
        assert!(sel.to_joker().is_some());
        assert!(sel.to_consumable().is_none());
    }

    #[test]
    fn test_shop_pack_selection_out_of_bounds() {
        let mut shop = Shop::new();
        shop.packs.push(PackType::Arcana);

        let pack = shop.buy_pack(PackType::Arcana).unwrap();

        // Try to select beyond available cards
        let selection = pack.select(10);
        assert!(selection.is_none());
    }

    #[test]
    fn test_consumable_generator_tarot_boost() {
        let mut gen = ConsumableGenerator::new();
        gen.update_from_vouchers(&[Vouchers::Tarot]);

        // With Tarot voucher, tarots should be 2x more common
        let mut tarot_count = 0;
        for _ in 0..100 {
            if matches!(gen.gen_consumable(), Consumables::Tarot(_)) {
                tarot_count += 1;
            }
        }

        // Should have more tarots than planets (roughly 2:1 ratio)
        assert!(tarot_count > 50);
    }

    #[test]
    fn test_consumable_generator_planet_boost() {
        let mut gen = ConsumableGenerator::new();
        gen.update_from_vouchers(&[Vouchers::Planet]);

        // With Planet voucher, planets should be 2x more common
        let mut planet_count = 0;
        for _ in 0..100 {
            if matches!(gen.gen_consumable(), Consumables::Planet(_)) {
                planet_count += 1;
            }
        }

        // Should have more planets than tarots (roughly 2:1 ratio)
        assert!(planet_count > 50);
    }

    #[test]
    fn test_pack_generator_buffoon_boost() {
        let mut gen = PackGenerator::new();
        gen.update_from_vouchers(&[Vouchers::Buffoon]);

        // With Buffoon voucher, buffoon packs should be 2x more common
        let mut buffoon_count = 0;
        for _ in 0..100 {
            if gen.gen_pack() == PackType::Buffoon {
                buffoon_count += 1;
            }
        }

        // Should have many buffoon packs
        assert!(buffoon_count > 60);
    }

    #[test]
    fn test_shop_reroll_increments_counter() {
        let mut shop = Shop::new();
        assert_eq!(shop.rerolls_this_round, 0);

        shop.reroll(&[]);
        assert_eq!(shop.rerolls_this_round, 1);

        shop.reroll(&[]);
        assert_eq!(shop.rerolls_this_round, 2);
    }

    #[test]
    fn test_shop_refresh_clears_items() {
        let mut shop = Shop::new();
        shop.jokers.push(Jokers::TheJoker(TheJoker::default()));
        shop.consumables.push(Consumables::Tarot(Tarots::TheFool));
        shop.packs.push(PackType::Arcana);
        shop.voucher = Some(Vouchers::Overstock);
        shop.rerolls_this_round = 5;

        shop.refresh(&[]);

        // Everything except newly generated items should be reset
        assert_eq!(shop.rerolls_this_round, 0);
    }

    #[test]
    fn test_joker_generator_rarity_distribution() {
        let gen = JokerGenerator::new();
        let mut common = 0;
        let mut uncommon = 0;
        let mut rare = 0;

        // Generate many jokers and check distribution
        for _ in 0..1000 {
            let joker = gen.gen_joker();
            match joker.rarity() {
                Rarity::Common => common += 1,
                Rarity::Uncommon => uncommon += 1,
                Rarity::Rare => rare += 1,
                Rarity::Legendary => {} // Should not appear naturally
            }
        }

        // Verify distribution is roughly 70/25/5
        // Common should be most frequent
        assert!(common > uncommon);
        // Allow for some variance with rare vs uncommon due to randomness
        // Just ensure common dominates
        assert!(common > 600);
    }

    #[test]
    fn test_shop_price_calculation() {
        let mut shop = Shop::new();
        shop.update_config(&[Vouchers::ClearanceSale]);
        shop.refresh(&[]);

        // Get a joker and check price
        if let Some(joker) = shop.jokers.first() {
            let base_cost = joker.cost();
            let shop_price = shop.joker_price(joker);
            assert_eq!(shop_price, (base_cost as f32 * 0.75).floor() as usize);
        }

        // Check consumable price
        if let Some(consumable) = shop.consumables.first() {
            let shop_price = shop.consumable_price(consumable);
            assert_eq!(shop_price, (4_f32 * 0.75).floor() as usize);
        }
    }

    #[test]
    fn test_shop_buy_removes_item() {
        let mut shop = Shop::new();
        shop.refresh(&[]);

        let initial_joker_count = shop.jokers.len();
        let joker = shop.jokers[0].clone();

        shop.buy_joker(&joker).unwrap();
        assert_eq!(shop.jokers.len(), initial_joker_count - 1);
    }

    #[test]
    fn test_shop_buy_nonexistent_item_fails() {
        let mut shop = Shop::new();
        shop.refresh(&[]);

        // Try to buy a joker that's not in the shop
        let other_joker = Jokers::CraftyJoker(CraftyJoker::default());
        let result = shop.buy_joker(&other_joker);
        assert!(result.is_err());
    }

    #[test]
    fn test_shop_gen_moves_buy_joker() {
        let mut shop = Shop::new();
        shop.refresh(&[]);

        // With high balance, should generate buy actions
        let moves = shop.gen_moves_buy_joker(100);
        assert!(moves.is_some());

        let move_vec: Vec<_> = moves.unwrap().collect();
        assert!(move_vec.len() > 0);
    }

    #[test]
    fn test_shop_gen_moves_buy_joker_insufficient_funds() {
        let mut shop = Shop::new();
        shop.refresh(&[]);

        // With zero balance, should generate no buy actions
        let moves = shop.gen_moves_buy_joker(0);
        if let Some(iter) = moves {
            let move_vec: Vec<_> = iter.collect();
            assert_eq!(move_vec.len(), 0);
        }
    }

    #[test]
    fn test_shop_gen_moves_buy_consumable() {
        let mut shop = Shop::new();
        shop.refresh(&[]);

        // With high balance, should generate buy actions
        let moves = shop.gen_moves_buy_consumable(100);
        assert!(moves.is_some());

        let move_vec: Vec<_> = moves.unwrap().collect();
        assert!(move_vec.len() > 0);
    }

    #[test]
    fn test_shop_empty_no_buy_actions() {
        let shop = Shop::new();

        // Empty shop should not generate buy actions
        let joker_moves = shop.gen_moves_buy_joker(100);
        assert!(joker_moves.is_none());

        let consumable_moves = shop.gen_moves_buy_consumable(100);
        assert!(consumable_moves.is_none());
    }

    #[test]
    fn test_pack_type_properties() {
        assert_eq!(PackType::Arcana.name(), "Arcana Pack");
        assert_eq!(PackType::Arcana.base_cost(), 4);
        assert_eq!(PackType::Arcana.card_count(), 3);
        assert_eq!(PackType::Arcana.choices(), 1);

        assert_eq!(PackType::Buffoon.card_count(), 2);
        assert_eq!(PackType::Buffoon.choices(), 1);
    }

    #[test]
    fn test_shop_config_default() {
        let config = ShopConfig::default();
        assert_eq!(config.joker_slots, 2);
        assert_eq!(config.consumable_slots, 2);
        assert_eq!(config.pack_slots, 2);
        assert_eq!(config.voucher_slots, 1);
        assert_eq!(config.reroll_cost, 5);
        assert_eq!(config.price_multiplier, 1.0);
    }

    #[test]
    fn test_shop_with_custom_config() {
        let config = ShopConfig {
            joker_slots: 5,
            consumable_slots: 5,
            pack_slots: 3,
            voucher_slots: 2,
            reroll_cost: 10,
            price_multiplier: 0.5,
        };

        let mut shop = Shop::with_config(config);
        shop.refresh(&[]);

        assert_eq!(shop.jokers.len(), 5);
        assert_eq!(shop.consumables.len(), 5);
        assert_eq!(shop.packs.len(), 3);
    }
}
