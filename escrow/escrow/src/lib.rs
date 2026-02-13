#![no_std]

use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol};

const ESCROWS_KEY: Symbol = symbol_short!("ESCROWS");
const BALANCES_KEY: Symbol = symbol_short!("BALANCE");

#[contracttype]
#[derive(Clone)]
pub struct EscrowDeal {
    pub payer: Address,
    pub receiver: Address,
    pub amount: i128,
    pub released: bool,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn deposit(env: Env, xlm_token: Address, payer: Address, amount: i128) {
        payer.require_auth();
        let token = TokenClient::new(&env, &xlm_token);
        token.transfer(&payer, &env.current_contract_address(), &amount);

        let mut balances: Map<Address, i128> = env
            .storage()
            .persistent()
            .get(&BALANCES_KEY)
            .unwrap_or(Map::new(&env));
        let current = balances.get(payer.clone()).unwrap_or(0);
        balances.set(payer, current + amount);
        env.storage().persistent().set(&BALANCES_KEY, &balances);
    }

    pub fn settle_rl(
        env: Env,
        xlm_token: Address,
        renter: Address,
        owner: Address,
        refund_to_renter: i128,
        pay_to_owner: i128,
    ) {
        let mut balances: Map<Address, i128> = env
            .storage()
            .persistent()
            .get(&BALANCES_KEY)
            .unwrap_or(Map::new(&env));

        let renter_balance = balances.get(renter.clone()).unwrap_or(0);
        if refund_to_renter + pay_to_owner > renter_balance {
            panic!("Insufficient escrowed balance");
        }

        let token = TokenClient::new(&env, &xlm_token);
        if refund_to_renter > 0 {
            token.transfer(&env.current_contract_address(), &renter, &refund_to_renter);
        }
        if pay_to_owner > 0 {
            token.transfer(&env.current_contract_address(), &owner, &pay_to_owner);
        }

        balances.set(renter, renter_balance - refund_to_renter - pay_to_owner);
        env.storage().persistent().set(&BALANCES_KEY, &balances);
    }

    pub fn balance(env: Env, user: Address) -> i128 {
        let balances: Map<Address, i128> = env
            .storage()
            .persistent()
            .get(&BALANCES_KEY)
            .unwrap_or(Map::new(&env));
        balances.get(user).unwrap_or(0)
    }

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
        token.transfer(&payer, &env.current_contract_address(), &amount);

        let mut escrows: Map<u64, EscrowDeal> = env
            .storage()
            .persistent()
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

    pub fn release(
        env: Env,
        escrow_id: u64,
        xlm_token: Address,
        refund_to_payer: i128,
        pay_to_receiver: i128,
    ) {
        let token = TokenClient::new(&env, &xlm_token);

        let mut escrows: Map<u64, EscrowDeal> = env.storage().persistent().get(&ESCROWS_KEY).unwrap();

        let mut deal = escrows.get(escrow_id).expect("Escrow not found");

        if deal.released {
            panic!("Escrow already settled");
        }

        if refund_to_payer + pay_to_receiver > deal.amount {
            panic!("Invalid escrow settlement");
        }

        if refund_to_payer > 0 {
            token.transfer(&env.current_contract_address(), &deal.payer, &refund_to_payer);
        }

        if pay_to_receiver > 0 {
            token.transfer(&env.current_contract_address(), &deal.receiver, &pay_to_receiver);
        }

        deal.released = true;
        escrows.set(escrow_id, deal);

        env.storage().persistent().set(&ESCROWS_KEY, &escrows);
    }

    pub fn get_escrow(env: Env, escrow_id: u64) -> EscrowDeal {
        let escrows: Map<u64, EscrowDeal> = env.storage().persistent().get(&ESCROWS_KEY).unwrap();
        escrows.get(escrow_id).expect("Escrow not found")
    }
}
