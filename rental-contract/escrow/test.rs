#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_escrow_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let renter = Address::generate(&env);
    let owner = Address::generate(&env);

    EscrowContract::pay_with_xlm(
        env.clone(),
        renter.clone(),
        owner.clone(),
        300,
        200,
    );

    EscrowContract::settle_return(
        env,
        renter,
        200,
        0,
    );
}
