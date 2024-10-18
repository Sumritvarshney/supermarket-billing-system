use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductInfo {
    pub name: String,
    pub rate: u64,
}

impl ProductInfo {
    pub fn new() -> ProductInfo {
        ProductInfo {
            name: String::from(""),
            rate: 0,
        }
    }
}

impl fmt::Display for ProductInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} per unit", self.name, self.rate)
    }
}
