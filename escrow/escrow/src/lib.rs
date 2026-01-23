#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Map, Symbol,
};
use soroban_sdk::symbol_short;
use soroban_sdk::token::Client as TokenClient;

const ESCROWS_KEY: Symbol = symbol_short!("ESCROWS");

#[contracttype]
#[derive(Clone)]
pub struct EscrowDeal {
    pub payer: Address,     // renter / buyer
    pub receiver: Address,  // owner / seller
    pub amount: i128,
    pub released: bool,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {

    /// 🟡 CREATE ESCROW (deposit XLM)
    pub fn create_escrow(
        env: Env,
        escrow_id: u64,
        xlm_token: Address,
        payer: Address,
        receiver: Address,
        amount: i128,
    ) {
        payer.require_auth();

        let token = TokenClient::new(&env, &xlm_token);

        token.transfer(
            &payer,
            &env.current_contract_address(),
            &amount,
        );

        let mut escrows: Map<u64, EscrowDeal> =
            env.storage().persistent()
                .get(&ESCROWS_KEY)
                .unwrap_or(Map::new(&env));

        if escrows.contains_key(escrow_id) {
            panic!("Escrow already exists");
        }

        escrows.set(
            escrow_id,
            EscrowDeal {
                payer,
                receiver,
                amount,
                released: false,
            },
        );

        env.storage().persistent().set(&ESCROWS_KEY, &escrows);
    }

    /// 🔓 RELEASE FUNDS (refund / pay)
    pub fn release(
        env: Env,
        escrow_id: u64,
        xlm_token: Address,
        refund_to_payer: i128,
        pay_to_receiver: i128,
    ) {
        let token = TokenClient::new(&env, &xlm_token);

        let mut escrows: Map<u64, EscrowDeal> =
            env.storage().persistent()
                .get(&ESCROWS_KEY)
                .unwrap();

        let mut deal = escrows.get(escrow_id).expect("Escrow not found");

        if deal.released {
            panic!("Escrow already settled");
        }

        if refund_to_payer + pay_to_receiver > deal.amount {
            panic!("Invalid escrow settlement");
        }

        if refund_to_payer > 0 {
            token.transfer(
                &env.current_contract_address(),
                &deal.payer,
                &refund_to_payer,
            );
        }

        if pay_to_receiver > 0 {
            token.transfer(
                &env.current_contract_address(),
                &deal.receiver,
                &pay_to_receiver,
            );
        }

        deal.released = true;
        escrows.set(escrow_id, deal);

        env.storage().persistent().set(&ESCROWS_KEY, &escrows);
    }

    /// 📊 VIEW ESCROW
    pub fn get_escrow(env: Env, escrow_id: u64) -> EscrowDeal {
        let escrows: Map<u64, EscrowDeal> =
            env.storage().persistent()
                .get(&ESCROWS_KEY)
                .unwrap();

        escrows.get(escrow_id).expect("Escrow not found")
    }
}
