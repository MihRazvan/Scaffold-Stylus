// tests/counter_tests.rs

use scaffold_stylus_contracts::{StylusContract, contracts::Counter};
use stylus_sdk::prelude::*;

#[test]
fn test_counter_basic_functionality() {
    // Test the standalone Counter contract
    let mut counter = Counter::default();
    
    // Test initial state
    assert_eq!(counter.get_count(), 0, "Initial count should be 0");
    
    // Test increment
    counter.increment();
    assert_eq!(counter.get_count(), 1, "Count should be 1 after first increment");
    
    counter.increment();
    assert_eq!(counter.get_count(), 2, "Count should be 2 after second increment");
}

#[test]
fn test_stylus_contract_counter() {
    // Test the counter through main contract
    let mut contract = StylusContract::new();
    
    // Test initial state
    assert_eq!(contract.get_count(), 0, "Initial count should be 0");
    
    // Test increment
    contract.increment();
    assert_eq!(contract.get_count(), 1, "Count should be 1 after increment");
}