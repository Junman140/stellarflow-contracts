use soroban_sdk::{contracttype, Address};

/// Canonical storage format for a price entry.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PriceData {
    /// The price value stored as a scaled integer.
    pub price: i128,
    /// Ledger timestamp when this price was written.
    pub timestamp: u64,
    /// Address that provided the price update.
    pub provider: Address,
}
