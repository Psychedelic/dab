use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../node/*/**");
    println!("cargo:rerun-if-changed=../tree/*/**");

    println!("Building node.wasm");
    Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .arg("--package")
        .arg("node")
        .status()
        .unwrap();

    println!("Running ic-cdk-optimizer on node.wasm");
    Command::new("ic-cdk-optimizer")
        .current_dir("../..")
        .arg("target/wasm32-unknown-unknown/release/node.wasm")
        .arg("-o")
        .arg("target/wasm32-unknown-unknown/release/node-opt.wasm")
        .status()
        .unwrap();
}
