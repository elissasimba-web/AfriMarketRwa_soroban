use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProductStatus {
    Available,
    Reserved,
    Sold,
    Unavailable,
}

#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Completed,
    Cancelled,
}
