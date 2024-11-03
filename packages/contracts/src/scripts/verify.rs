// scripts/verify.rs

use std::env;
use std::process::Command;

fn main() {
    println!("Starting verification process...");

    // Get contract address from environment variable
    let contract_address = env::var("CONTRACT_ADDRESS")
        .expect("CONTRACT_ADDRESS environment variable not set");

    verify_contract(&contract_address);
}

fn verify_contract(address: &str) {
    println!("Verifying contract at address: {}", address);

    let output = Command::new("stylus")
        .args([
            "verify",
            "--address", address,
            "--rpc", "https://sepolia-rollup.arbitrum.io/rpc"
        ])
        .output()
        .expect("Failed to verify contract");

    if !output.status.success() {
        panic!("Verification failed: {:?}", String::from_utf8_lossy(&output.stderr));
    }

    println!("Contract verified successfully!");
}