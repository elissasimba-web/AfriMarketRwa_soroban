#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Map, Symbol,
};
use escrow::EscrowContractClient;
use core::{ProductStatus};

const LIVESTOCK_KEY: Symbol = symbol_short!("LIVESTOCK");
const ESCROW_KEY: Symbol = symbol_short!("ESCROW");
const XLM_TOKEN_KEY: Symbol = symbol_short!("XLM");

#[contracttype]
#[derive(Clone)]
pub struct Livestock {
    pub id: u32,
    pub kind: Symbol,        // Inka, ihene, intama
    pub owner: Address,
    pub price: i128,
    pub status: ProductStatus,
}

#[contract]
pub struct LivestockContract;

#[contractimpl]
impl LivestockContract {

    /// initialize
    pub fn init(env: Env, escrow: Address, xlm_token: Address) {
        env.storage().instance().set(&ESCROW_KEY, &escrow);
        env.storage().instance().set(&XLM_TOKEN_KEY, &xlm_token);

        let items: Map<u32, Livestock> = Map::new(&env);
        env.storage().instance().set(&LIVESTOCK_KEY, &items);
    }

    /// umworozi ashyira itungo ku isoko
    pub fn add_livestock(
        env: Env,
        owner: Address,
        id: u32,
        kind: Symbol,
        price: i128,
    ) {
        owner.require_auth();

        let mut items: Map<u32, Livestock> = env
            .storage()
            .instance()
            .get(&LIVESTOCK_KEY)
            .unwrap();

        let animal = Livestock {
            id,
            kind,
            owner,
            price,
            status: ProductStatus::Available,
        };

        items.set(id, animal);
        env.storage().instance().set(&LIVESTOCK_KEY, &items);
    }

    /// kugura itungo (payment via escrow)
    pub fn buy_livestock(env: Env, id: u32, buyer: Address) {
        buyer.require_auth();

        let escrow_addr: Address =
            env.storage().instance().get(&ESCROW_KEY).unwrap();
        let xlm_token: Address =
            env.storage().instance().get(&XLM_TOKEN_KEY).unwrap();

        let escrow = EscrowContractClient::new(&env, &escrow_addr);

        let mut items: Map<u32, Livestock> =
            env.storage().instance().get(&LIVESTOCK_KEY).unwrap();

        let mut animal = items.get(id).unwrap();
        if animal.status != ProductStatus::Available {
            panic!("Livestock not available");
        }

        // buyer pays
        escrow.deposit(&xlm_token, &buyer, &animal.price);

        // release to seller
        escrow.settle_return_with_xlm(
            &xlm_token,
            &buyer,
            &animal.owner,
            &animal.price,
            &0,
        );

        animal.owner = buyer;
        animal.status = ProductStatus::Sold;

        items.set(id, animal);
        env.storage().instance().set(&LIVESTOCK_KEY, &items);
    }

    /// kureba amatungo yose
    pub fn list(env: Env) -> Map<u32, Livestock> {
        env.storage()
            .instance()
            .get(&LIVESTOCK_KEY)
            .unwrap()
    }
}
