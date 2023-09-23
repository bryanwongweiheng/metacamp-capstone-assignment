use std::collections::HashMap;
use crate::location::{Country, Continent};
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Transaction {
    transaction_id: u32,
    client_id: u32,
    asset_name: String,
    country: Country,
    continent: Continent,
    amount: f64,
    days_under_management: i64,
}

impl Transaction {
    /// parses a line from a csv file into a Transaction. needs 7 fields
     pub fn from_csv_line(line: &str) -> Result<Transaction, String> {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 7 {
            return Err("Incorrect number of fields".to_owned());
        }
        let transaction_id: u32 = fields[0].parse().unwrap();
        let client_id: u32 = fields[1].parse().unwrap();
        let asset_name: String = fields[2].to_uppercase();
        let transaction_start_date: NaiveDate = NaiveDate::parse_from_str(fields[3], "%Y-%m-%d").unwrap();
        let transaction_end_date: NaiveDate = NaiveDate::parse_from_str(fields[4], "%Y-%m-%d").unwrap();
        let country: Country = fields[5].parse()?;
        let amount: f64 = fields[6].parse().unwrap();
        let days_under_management = (transaction_end_date - transaction_start_date).num_days();
        let continent = country.country_to_continent();
        let transaction = Transaction{
            transaction_id,
            client_id,
            asset_name,
            country,
            continent,
            amount,
            days_under_management,
        };
        Ok(transaction)
    }
}


/// sums the total transaction amount by continent
pub fn summarise_by_continent(transactions: &Vec<Transaction>) -> HashMap<String, f64> {
    let mut summary: HashMap<String, f64> = HashMap::new();
    for transaction in transactions {
        let continent_string = transaction.continent.to_string();
        *summary.entry(continent_string).or_default() += transaction.amount;
    }
    summary
}

/// prints out transactions for chosen continent
pub fn display_one_continent(transactions: &Vec<Transaction>, continent: &Continent) {
    let continent_transactions = transactions
        .into_iter()
        .filter(|txn| txn.continent == *continent);
    println!("Here are {:?}'s transactions", continent);
    for transaction in continent_transactions {
        println!("{:?}", transaction);
    }
}