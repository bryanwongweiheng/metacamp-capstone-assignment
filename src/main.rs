mod location;
mod transaction;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use transaction::Transaction;

fn main() {
    let file = File::open("./transactions.csv").unwrap();
    let reader = BufReader::new(file);
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut skipped_lines = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        if idx == 0 {
            continue;
        } else {
            let line_str = line.unwrap();
            let parsed_transaction = Transaction::from_csv_line(line_str.as_str());
            match parsed_transaction {
                Ok(transaction) => transactions.push(transaction),
                Err(skipped_line) => skipped_lines.push((idx, skipped_line, line_str)),
            }
        }
    }

    for transaction in transactions {
        println!("{:?}", transaction);
    }
    for (idx, skipped_line, line_str) in skipped_lines {
        println!("Skipped {:?} {:?} {:?}", idx, skipped_line, line_str);
    }
}
