use crate::display::MyDisplay;
use crate::prelude::*;
use alloy::primitives::Address;
use colored::Colorize;
use serde::Serialize;
use serde_json::to_string;
use std::fmt::{self, Write};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Serialize)]
pub struct Token {
    address: Address,
    symbol: String,
}

pub struct Tokens {
    pub by_symbols: HashMap<String, Token>,
    pub by_address: HashMap<Address, Token>,
}

impl Tokens {
    pub fn new() -> Self {
        let by_symbols = HashMap::new();
        let by_address = HashMap::new();

        Self {
            by_symbols,
            by_address,
        }
    }

    pub fn insert(&mut self, symbol: String, address: Address) {
        let token = Token {
            address: address.clone(),
            symbol: symbol.clone(),
        };

        self.by_symbols
            .entry(symbol.clone())
            .insert_entry(token.clone());
        self.by_address.entry(address.clone()).or_insert(token);
    }
}

impl MyDisplay for Tokens {
    fn to_json(&self) -> Result<String> {
        // Create a vector or symbol, address.
        let mut v: Vec<_> = (&self.by_symbols)
            .into_iter()
            .map(|p| (p.1.symbol.clone(), p.1.address))
            .collect();

        // Sort the hashmap by key.
        v.sort_by(|x, y| x.0.cmp(&y.0));

        // Export to JSON string.
        Ok(serde_json::to_string(&v)?)
    }

    fn to_text(&self) -> Result<String> {
        let mut buf = String::new();

        // Create a vector of symbol, token.
        let mut v: Vec<_> = (&self.by_symbols).into_iter().collect();

        // Sort the hashmap by key.
        v.sort_by(|x, y| x.0.cmp(&y.0));

        // // Display all tokens.
        for (symbol, token) in v {
            buf.write_str(
                format!(
                    "{} - {}\n",
                    token.address.to_string().blue(),
                    symbol.green()
                )
                .as_str(),
            )?;
        }

        Ok(buf)
    }
}
