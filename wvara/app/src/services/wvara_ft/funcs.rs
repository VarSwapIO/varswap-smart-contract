use sails_rs::prelude::*;
use vft_service::{
    funcs,
    utils::{Error, Result, *},
};

pub fn deposit(
    balances: &mut BalancesMap,
    total_supply: &mut U256,
    from: ActorId,
    value: U256,
) -> Result<bool> {
    if value.is_zero() {
        return Ok(false);
    }

    let new_total_supply = total_supply
        .checked_add(value)
        .ok_or(Error::NumericOverflow)?;

    let new_to = funcs::balance_of(balances, from)
        .checked_add(value)
        .ok_or(Error::NumericOverflow)?;

    balances.insert(from, new_to);
    *total_supply = new_total_supply;

    Ok(true)
}

pub fn withdraw(
    balances: &mut BalancesMap,
    total_supply: &mut U256,
    to: ActorId,
    value: U256,
) -> Result<bool> {
    if value.is_zero() {
        return Ok(false);
    }

    let new_total_supply = total_supply.checked_sub(value).ok_or(Error::Underflow)?;

    let new_to = funcs::balance_of(balances, to)
        .checked_sub(value)
        .ok_or(Error::InsufficientBalance)?;

    if !new_to.is_zero() {
        balances.insert(to, new_to);
    } else {
        balances.remove(&to);
    }

    *total_supply = new_total_supply;
    Ok(true)
}

