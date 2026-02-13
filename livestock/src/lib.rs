#![no_std]

use platform_core::ProductStatus;
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol, Val, Vec,
    IntoVal,
};

const LIVESTOCK_KEY: Symbol = symbol_short!("LIVEST");
const ESCROW_KEY: Symbol = symbol_short!("ESCROW");
const XLM_TOKEN_KEY: Symbol = symbol_short!("XLM");
const DEPOSIT: Symbol = symbol_short!("deposit");
const SETTLE: Symbol = symbol_short!("settle_rl");

#[contracttype]
#[derive(Clone)]
pub struct Livestock {
    pub id: u32,
    pub kind: Symbol,
    pub owner: Address,
    pub price: i128,
    pub status: ProductStatus,
}

#[contract]
pub struct LivestockContract;

#[contractimpl]
impl LivestockContract {
    pub fn init(env: Env, escrow: Address, xlm_token: Address) {
        env.storage().instance().set(&ESCROW_KEY, &escrow);
        env.storage().instance().set(&XLM_TOKEN_KEY, &xlm_token);
        env.storage()
            .instance()
            .set(&LIVESTOCK_KEY, &Map::<u32, Livestock>::new(&env));
    }

    pub fn add_livestock(env: Env, owner: Address, id: u32, kind: Symbol, price: i128) {
        owner.require_auth();
        let mut items: Map<u32, Livestock> = env.storage().instance().get(&LIVESTOCK_KEY).unwrap();

        items.set(
            id,
            Livestock {
                id,
                kind,
                owner,
                price,
                status: ProductStatus::Available,
            },
        );

        env.storage().instance().set(&LIVESTOCK_KEY, &items);
    }

    pub fn buy_livestock(env: Env, id: u32, buyer: Address) {
        buyer.require_auth();

        let escrow: Address = env.storage().instance().get(&ESCROW_KEY).unwrap();
        let xlm_token: Address = env.storage().instance().get(&XLM_TOKEN_KEY).unwrap();
        let mut items: Map<u32, Livestock> = env.storage().instance().get(&LIVESTOCK_KEY).unwrap();

        let mut animal = items.get(id).expect("Livestock not found");
        if animal.status != ProductStatus::Available {
            panic!("Livestock not available");
        }

        let mut deposit_args = Vec::<Val>::new(&env);
        deposit_args.push_back(xlm_token.clone().into_val(&env));
        deposit_args.push_back(buyer.clone().into_val(&env));
        deposit_args.push_back(animal.price.into_val(&env));
        env.invoke_contract::<()>(&escrow, &DEPOSIT, deposit_args);

        let mut settle_args = Vec::<Val>::new(&env);
        settle_args.push_back(xlm_token.into_val(&env));
        settle_args.push_back(buyer.into_val(&env));
        settle_args.push_back(animal.owner.clone().into_val(&env));
        settle_args.push_back(0_i128.into_val(&env));
        settle_args.push_back(animal.price.into_val(&env));
        env.invoke_contract::<()>(&escrow, &SETTLE, settle_args);

        animal.status = ProductStatus::Sold;
        items.set(id, animal);
        env.storage().instance().set(&LIVESTOCK_KEY, &items);
    }
}
