# A Tick Array is:

An account

That belongs to one pool

That stores tick data for a continuous range of ticks

It stores per-tick metadata, mainly:

how much liquidity starts at that tick

how much liquidity ends at that tick

# What Happens During a Swap? (VERY SIMPLE)

Assume current active liquidity before crossing tick 100:

global_liquidity = 200


Price moves upward and crosses tick 100.

We apply:

global_liquidity += liquidity_net
global_liquidity += 80


So now:

global_liquidity = 280


That is the only purpose of liquidity_net.



Why Do We Need BOTH Fields?
liquidity_net

Used during swaps to:

update active liquidity

handle tick crossing

liquidity_gross

Used to:

know whether a tick is still in use

know when it can be safely cleared

validate removals

They serve different responsibilities.

liquidity_gross tells “how much liquidity touches this tick”,
liquidity_net tells “how liquidity changes when crossing this tick”.














use anchor_lang::prelude::*;
use crate::utils::ErrorCode;

#[account]
pub struct TickInfo {
    pub initialized: bool,
    pub liquidity_gross: u128, //Sum of absolute liquidity amounts that reference this tick
   //(both starting AND ending here)
    pub liquidity_net: i128,  //Net change in active liquidity when price crosses this tick upward
}
// Positive → liquidity is added when crossing up
// Negative → liquidity is removed when crossing up

impl TickInfo {

    pub const SPACE: usize = 
        8 + // discriminator
        16 + // liquidity_gross
        16 + // liquidity_net
        1;   // initialized

    pub fn update_liquidity(&mut self, liquidity_delta: i128, is_lower: bool) -> Result<()> { //This function is called only when a position is opened or closed.
        if !self.initialized {
            self.initialized = true;
        }
        self.liquidity_gross = self
            .liquidity_gross
            .checked_add(liquidity_delta.unsigned_abs())
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        if is_lower {
            self.liquidity_net = self
                .liquidity_net
                .checked_add(liquidity_delta)
                .ok_or(ErrorCode::ArithmeticOverflow)?;
        } else {
            self.liquidity_net = self
                .liquidity_net
                .checked_sub(liquidity_delta)
                .ok_or(ErrorCode::ArithmeticOverflow)?;
        }
        Ok(())
    }
//     lower → +liquidity
//     upper → −liquidity
}

pub const TICKS_PER_ARRAY: usize = 30;

#[account]
pub struct TickArray {
    pub pool: Pubkey,
    pub starting_tick: i32,  //starting_tick is the first tick index covered by this array.
    pub ticks: [TickInfo; TICKS_PER_ARRAY],
    pub bump: u8,
 }
 //THis is acutal storage
// Each element:
// corresponds to one tick
// stores liquidity_gross and liquidity_net for that tick
// ticks[0] does not mean tick 0
// It means:tick = starting_tick + 0 * tick_spacing
//Mapping formula : tick_value = starting_tick + i * tick_spacing
//And reverse:i = (tick - starting_tick) / tick_spacing

//A TickArray is an account that owns a fixed contiguous range of ticks starting at starting_tick
//, and each slot in its ticks array stores liquidity change information for exactly one tick spaced by tick_spacing.

impl TickArray {

    pub const SPACE: usize = 8 + // discriminator
        32 + // pool
        4 +  // starting_tick
        TICKS_PER_ARRAY * 48 + // ticks
        1;   // bump

    pub fn get_starting_tick_index(tick: i32, tick_spacing: i32) -> i32 {
        let ticks_per_array_i32 = TICKS_PER_ARRAY as i32;
        let array_idx = tick
            .checked_div(tick_spacing)
            .expect("Div by zero: tick_spacing")
            .checked_div(ticks_per_array_i32)
            .expect("Div by zero: TICKS_PER_ARRAY");
        array_idx
            .checked_mul(ticks_per_array_i32)
            .expect("Mul overflow")
            .checked_mul(tick_spacing)
            .expect("Mul overflow")
    }
    pub fn get_tick_info_mutable(&mut self, tick: i32, tick_spacing: i32) -> Result<&mut TickInfo> {
        let ticks_per_array_i32 = TICKS_PER_ARRAY as i32;
        let offset = (tick
            .checked_div(tick_spacing)
            .ok_or(ErrorCode::ArithmeticOverflow)?)
        .checked_sub(
            self.starting_tick
                .checked_div(tick_spacing)
                .ok_or(ErrorCode::ArithmeticOverflow)?,
        )
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_rem(ticks_per_array_i32)
        .ok_or(ErrorCode::ArithmeticOverflow)? as usize;
        Ok(&mut self.ticks[offset])
    }
}
