#![no_std]

use platform_core::ProductStatus;
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol};

const RIDES_KEY: Symbol = symbol_short!("RIDE");

#[contracttype]
#[derive(Clone)]
pub struct RideOffer {
    pub id: u32,
    pub driver: Address,
    pub route: Symbol,
    pub price: i128,
    pub status: ProductStatus,
}

#[contract]
pub struct TransportContract;

#[contractimpl]
impl TransportContract {
    pub fn init(env: Env) {
        env.storage()
            .instance()
            .set(&RIDES_KEY, &Map::<u32, RideOffer>::new(&env));
    }

    pub fn add_offer(env: Env, driver: Address, id: u32, route: Symbol, price: i128) {
        driver.require_auth();
        let mut rides: Map<u32, RideOffer> = env.storage().instance().get(&RIDES_KEY).unwrap();

        rides.set(
            id,
            RideOffer {
                id,
                driver,
                route,
                price,
                status: ProductStatus::Available,
            },
        );

        env.storage().instance().set(&RIDES_KEY, &rides);
    }

    pub fn reserve(env: Env, passenger: Address, id: u32) {
        passenger.require_auth();
        let mut rides: Map<u32, RideOffer> = env.storage().instance().get(&RIDES_KEY).unwrap();
        let mut ride = rides.get(id).expect("Ride not found");

        if ride.status != ProductStatus::Available {
            panic!("Ride unavailable");
        }

        ride.status = ProductStatus::Reserved;
        rides.set(id, ride);
        env.storage().instance().set(&RIDES_KEY, &rides);
    }
}
