use risc0_zkvm::guest::env;

fn main() {
    // read the input
    let input: u32 = env::read();

    // Pause execution here to be continued later
    env::pause(0);

    // write public output to the journal
    env::commit(&input);
}
