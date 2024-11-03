// src/lib.rs

// Module declarations
pub mod contracts;
pub mod utils;

// Re-exports for easier access
pub use contracts::*;
pub use utils::*;

// Import necessary dependencies
use stylus_sdk::{prelude::*, Contract};
use alloy_primitives::{Address, U256};

// Contract modules
pub mod contracts {
    pub mod counter;
    // Add more contract modules as needed
    pub use counter::Counter;
}

// Export main contract implementation
// This is your default contract that will be deployed
#[contract]
pub struct StylusContract {
    counter: contracts::counter::Counter,
}

#[external]
impl StylusContract {
    // Constructor
    pub fn new() -> Self {
        Self {
            counter: contracts::counter::Counter::default(),
        }
    }

    // Counter methods
    pub fn increment(&mut self) {
        self.counter.increment();
    }

    pub fn get_count(&self) -> u64 {
        self.counter.get_count()
    }

    // Token methods
    pub fn balance_of(&self, account: Address) -> U256 {
        self.token.balance_of(account)
    }

    pub fn transfer(&mut self, to: Address, amount: U256) -> bool {
        self.token.transfer(to, amount)
    }
}

// Tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut contract = StylusContract::new();
        assert_eq!(contract.get_count(), 0);
        
        contract.increment();
        assert_eq!(contract.get_count(), 1);
    }
}

// Initialize function for WASM
#[no_mangle]
pub extern "C" fn init() {
    let contract = StylusContract::new();
    stylus_sdk::init::init_contract(contract);
}