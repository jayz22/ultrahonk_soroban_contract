use soroban_sdk::{Bytes, Env};

const CONTRACT_WASM: &[u8] = include_bytes!("../target/wasm32v1-none/release/ultrahonk_soroban_contract.wasm");

mod ultrahonk_contract {
    soroban_sdk::contractimport!(file = "target/wasm32v1-none/release/ultrahonk_soroban_contract.wasm");
}

#[test]
fn basic_verify_budget_test() {
    let vk_bin: &[u8] = include_bytes!("simple_circuit/target/vk");
    let proof_bin: &[u8] = include_bytes!("simple_circuit/target/proof");
    let pub_inputs_bin: &[u8] = include_bytes!("simple_circuit/target/public_inputs");

    let env = Env::default();
    env.cost_estimate().budget().reset_unlimited();

    let contract_id = env.register(CONTRACT_WASM, ());
    let client = ultrahonk_contract::Client::new(&env, &contract_id);

    let vk_bytes = Bytes::from_slice(&env, vk_bin);
    let pub_inputs_bytes = Bytes::from_slice(&env, pub_inputs_bin);
    let proof_bytes = Bytes::from_slice(&env, proof_bin);

    client.verify_proof(&vk_bytes, &pub_inputs_bytes, &proof_bytes);
    env.cost_estimate().budget().print();
}

#[test]
fn fib_chain_verify_budget_test() {
    let vk_bin: &[u8] = include_bytes!("fib_chain/target/vk");
    let proof_bin: &[u8] = include_bytes!("fib_chain/target/proof");
    let pub_inputs_bin: &[u8] = include_bytes!("fib_chain/target/public_inputs");

    let env = Env::default();
    env.cost_estimate().budget().reset_unlimited();

    let contract_id = env.register(CONTRACT_WASM, ());
    let client = ultrahonk_contract::Client::new(&env, &contract_id);

    let vk_bytes = Bytes::from_slice(&env, vk_bin);
    let pub_inputs_bytes = Bytes::from_slice(&env, pub_inputs_bin);
    let proof_bytes = Bytes::from_slice(&env, proof_bin);

    client.verify_proof(&vk_bytes, &pub_inputs_bytes, &proof_bytes);
    env.cost_estimate().budget().print();
}
