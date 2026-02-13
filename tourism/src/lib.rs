#![no_std]

use platform_core::ProductStatus;
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol};

const PACKAGES_KEY: Symbol = symbol_short!("TOUR");

#[contracttype]
#[derive(Clone)]
pub struct TourPackage {
    pub id: u32,
    pub host: Address,
    pub title: Symbol,
    pub price: i128,
    pub status: ProductStatus,
}

#[contract]
pub struct TourismContract;

#[contractimpl]
impl TourismContract {
    pub fn init(env: Env) {
        env.storage()
            .instance()
            .set(&PACKAGES_KEY, &Map::<u32, TourPackage>::new(&env));
    }

    pub fn add_package(env: Env, host: Address, id: u32, title: Symbol, price: i128) {
        host.require_auth();
        let mut packages: Map<u32, TourPackage> = env.storage().instance().get(&PACKAGES_KEY).unwrap();

        packages.set(
            id,
            TourPackage {
                id,
                host,
                title,
                price,
                status: ProductStatus::Available,
            },
        );

        env.storage().instance().set(&PACKAGES_KEY, &packages);
    }

    pub fn book(env: Env, tourist: Address, id: u32) {
        tourist.require_auth();
        let mut packages: Map<u32, TourPackage> = env.storage().instance().get(&PACKAGES_KEY).unwrap();
        let mut package = packages.get(id).expect("Package not found");

        if package.status != ProductStatus::Available {
            panic!("Package unavailable");
        }

        package.status = ProductStatus::Reserved;
        packages.set(id, package);
        env.storage().instance().set(&PACKAGES_KEY, &packages);
    }
}
