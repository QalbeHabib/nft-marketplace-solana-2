use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

// FIXED: Safe fee calculation helper function
pub fn calculate_marketplace_fee(total_price: u64, fee_bps: u16) -> Result<u64> {
    // Validate fee basis points (max 100% = 10,000 bps)
    require!(fee_bps <= 10000, ErrorCode::InvalidFeeBasisPoints);

    // Use u128 for intermediate calculation to prevent overflow
    let fee_u128 = (total_price as u128)
        .checked_mul(fee_bps as u128)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Ensure the result fits in u64
    if fee_u128 > u64::MAX as u128 {
        return Err(ErrorCode::ArithmeticOverflow.into());
    }

    Ok(fee_u128 as u64)
}

// Validate and convert royalty percentage to basis points
pub fn validate_royalty_percent(royalty_percent: u16) -> Result<u16> {
    // 50% = 5000 basis points
    require!(royalty_percent <= 50, ErrorCode::RoyaltyTooHigh);

    // Convert percentage to basis points (multiply by 100)
    let basis_points = royalty_percent
        .checked_mul(100)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    Ok(basis_points)
}
