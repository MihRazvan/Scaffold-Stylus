// scripts/deploy.rs

use std::env;
use std::process::Command;

const ARBITRUM_RPC: &str = "https://sepolia-rollup.arbitrum.io/rpc";

fn main() {
    println!("Starting deployment process...");

    // Build the contract
    build_contract();

    // Deploy the contract
    deploy_contract();
}

fn build_contract() {
    println!("Building contract...");
    
    let output = Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .expect("Failed to build contract");

    if !output.status.success() {
        panic!("Build failed: {:?}", String::from_utf8_lossy(&output.stderr));
    }

    println!("Contract built successfully!");
}

fn deploy_contract() {
    println!("Deploying contract...");

    // Get private key from environment variable
    let private_key = env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY environment variable not set");

    println!("Deploying to Arbitrum Sepolia...");
    
    let output = Command::new("stylus")
        .args([
            "deploy",
            "--private-key", &private_key,
            "--rpc", ARBITRUM_RPC,
            "target/wasm32-unknown-unknown/release/scaffold_stylus_contracts.wasm"
        ])
        .output()
        .expect("Failed to deploy contract");

    if !output.status.success() {
        panic!("Deployment failed: {:?}", String::from_utf8_lossy(&output.stderr));
    }

    println!("Contract deployed successfully!");
}