use crate::location::{Country, Continent};

struct Transaction {
    transaction_id: u32,
    client_id: u32,
    asset_name: String,
    country: Country,
    continent: Continent,
    amount: f64,
    days_under_management: i64,
}