// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use risc0_zkvm::{ExecutorEnv, ExecutorImpl, VerifierContext};
use risc0_zkvm::sha::Digestible;

#[test]
fn test_resumed_profiling() -> anyhow::Result<()> {
    // For example:
    let input: u32 = 15 * u32::pow(2, 27) + 1;

    // Setup environment
    let mut env_builder = ExecutorEnv::builder();

    env_builder
        .write(&input)?;

    // Save guest profiling information in file
    // FIXME: Profiling does not work with resumed sessions
    env_builder.enable_profiler("./profiling.pprof");

    let env = env_builder.build()?;
    // Setup executor using environment
    let mut executor = ExecutorImpl::from_elf(env, methods::MY_RESUMABLE_ZK_PROGRAM_ELF)?;

    // Run the executor until the first pause and prove that
    let first_session = executor.run()?;
    let prove_info = first_session.prove()?;
    assert!(prove_info.receipt.journal.bytes.is_empty());
    let first_stark_receipt = prove_info.receipt;
    // Ensures the claim and journal are attested to by the seal
    first_stark_receipt.verify_integrity_with_context(&VerifierContext::default())?;
    // Manually check the image ID.
    let first_receipt_claim = first_stark_receipt.claim()?.value()?;
    assert_eq!(
        first_receipt_claim.pre.digest(),
        methods::MY_RESUMABLE_ZK_PROGRAM_ID.into()
    );

    // Run the executor until termination and prove that
    let second_session = executor.run()?;
    let prove_info = second_session.prove()?;
    // extract the receipt.
    let second_stark_receipt = prove_info.receipt;
    // verify output
    let output: u32 = second_stark_receipt.journal.decode()?;
    assert_eq!(input, output);


    second_stark_receipt.verify(first_session.post_state.digest())?;

    Ok(())
}
