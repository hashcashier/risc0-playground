use risc0_zkvm::guest::env;

fn main() {
    // read the input
    let input: u32 = env::read();

    // Read the input label
    let label = env::input_digest();
    dbg!(label);

    // write public output to the journal
    env::commit(&input);
}
