use crate::utils::ErrorCode;
use anchor_lang::prelude::*;

pub fn get_sqrt_price_from_tick(tick: i32) -> Result<u128> {
    // This is a simplification; real math is logarithmic.
    let base_sqrt_price = 1u128 << 96;
    let adjustment_factor = 1_000_000_000 / 1000;
    let adjusted_price = base_sqrt_price
        .checked_add_signed((tick as i128) * (adjustment_factor as i128))
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    Ok(adjusted_price)
}


//“Given the current price of the pool, which price bucket (tick) are we in?”
pub fn get_tick_at_sqrt_price(sqrt_price_x96:u128)->Result<i32>{
    let base_sqrt_price = 1u128 << 96;  //represent sqrt(price = 1) × 2^96
    let adjustment_factor = 1_000_000_000 / 1000;
    let diff = sqrt_price_x96 as i128 - base_sqrt_price as i128;

    let tick = diff
        .checked_div(adjustment_factor as i128)
        .ok_or(ErrorCode::ArithmeticOverflow)? as i32;
    Ok(tick)
    
}

// Fix the Reference Point
// let base_sqrt_price = 1u128 << 96;
// This is:sqrt(price = 1) × 2^96
// This is your zero point
// So:

// If sqrt_price_x96 == base_sqrt_price → tick = 0
// If larger → positive tick
// If smaller → negative tick

//what adjustment_factor REALLY Means : the diffrence between 2 ticks
//so the tick formula is : tick = (sqrt_price_x96 − base_sqrt_price) / adjustment_factor
//tick spacing means which of those ticks are allowd to hold liquidity


// Adjustment factor defines the numeric distance between consecutive ticks in price space.
// Tick spacing defines which of those ticks are allowed to add or remove liquidity.
// Price moves across all ticks, but liquidity only changes at ticks aligned with tick spacing.


pub fn get_amount_for_liquidity(
    current_sqrt_price_x96:u128,
    lower_sqrt_price_x96 : u128,
    upper_sqrt_price_x96 :u128,
    liquidity : u128
)->Result<(u64,u64)>{
    let amount0 : u64;
    let amount1 : u64;

    if current_sqrt_price_x96 >= lower_sqrt_price_x96 && current_sqrt_price_x96 < upper_sqrt_price_x96 {
        amount0 = (liquidity / 2) as u64;
        amount1 = (liquidity / 2) as u64;
    }else if current_sqrt_price_x96 < lower_sqrt_price_x96{
        amount0 = liquidity as u64;
        amount1 = 0;

    }else {
        amount0 = 0 ;
        amount1 = liquidity as u64
    }
    Ok((amount0,amount1))
}

// When an LP opens a position, they say:
// “I want to provide liquidity between lower_tick and upper_tick.”
// But the current price may be:
// below that range
// inside that range
// above that range
// Depending on where the price is right now, the LP must deposit different tokens.
// This function answers: “How much token0 and token1 does the LP need to deposit right now?”

//CASE :1 => price is inside the range in this case liq is active both token is in used
//CASE :2 => price is bleow the ragne in this case if When price eventually moves up into the range, swaps will consume token0 first
//so we need only token 0 in this case
//CASE :3 => price is above the range in this case When price eventually moves down into the range, swaps will consume token1 first