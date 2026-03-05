

#[derive(Clone)]
pub enum ItemStatus {
    Active,
    SoldOut,
}
#[derive(Clone)]
pub enum OrderStatus {
    Pending,
    Completed,
    Cancelled,
}
#[derive(Clone)]
pub enum AgricultureCategory {
    Crops,
    Livestock,
    Tools,
}
