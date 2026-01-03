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

