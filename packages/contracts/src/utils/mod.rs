pub mod convert {
    use alloy_primitives::{Address, U256};

    pub fn to_u256(value: u64) -> U256 {
        U256::from(value)
    }

    pub fn to_address(hex: &str) -> Address {
        Address::from_slice(&hex::decode(hex).unwrap())
    }
}