#![no_std]

use platform_core::ProductStatus;
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol};

const SERVICES_KEY: Symbol = symbol_short!("HEALTH");

#[contracttype]
#[derive(Clone)]
pub struct HealthService {
    pub id: u32,
    pub provider: Address,
    pub name: Symbol,
    pub fee: i128,
    pub status: ProductStatus,
}

#[contract]
pub struct HealthContract;

#[contractimpl]
impl HealthContract {
    pub fn init(env: Env) {
        env.storage()
            .instance()
            .set(&SERVICES_KEY, &Map::<u32, HealthService>::new(&env));
    }

    pub fn add_service(env: Env, provider: Address, id: u32, name: Symbol, fee: i128) {
        provider.require_auth();
        let mut services: Map<u32, HealthService> = env.storage().instance().get(&SERVICES_KEY).unwrap();

        services.set(
            id,
            HealthService {
                id,
                provider,
                name,
                fee,
                status: ProductStatus::Available,
            },
        );

        env.storage().instance().set(&SERVICES_KEY, &services);
    }

    pub fn get_service(env: Env, id: u32) -> HealthService {
        let services: Map<u32, HealthService> = env.storage().instance().get(&SERVICES_KEY).unwrap();
        services.get(id).expect("Service not found")
    }
}
