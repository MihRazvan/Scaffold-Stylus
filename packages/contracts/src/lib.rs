// packages/contracts/src/lib.rs
use stylus_sdk::{prelude::*, Contract};

#[contract]
pub struct Counter {
    count: StorageU64,
}

#[external]
impl Counter {
    pub fn increment(&mut self) {
        self.count.set(self.count.get() + 1);
    }

    pub fn get_count(&self) -> u64 {
        self.count.get()
    }
}