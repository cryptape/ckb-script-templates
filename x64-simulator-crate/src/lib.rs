#![cfg_attr(not(test), no_std)]

pub fn foo() -> usize {
    ckb_std::high_level::load_witness(0, ckb_std::ckb_constants::Source::Input)
        .expect("load_witness")
        .len()
}

// Here we provide a native runnable test sample. The test uses ckb-x64-simulator
// to mock CKB syscalls.
#[cfg(all(test, target_arch = "x86_64", unix))]
mod tests {
    use super::*;
    use ckb_testtool::ckb_types::{core::TransactionBuilder, prelude::*};
    use ckb_testtool::context::Context;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use rusty_fork::rusty_fork_test;
    use std::io::Write;

    // TODO: Right now ckb-x64-simulator has no way of resetting the
    // test transaction after initial setup. Hence we have to use this
    // circumvent way of testing. Later we would want to fix ckb-x64-simulator
    // so test data can be properly mutated, after that, we can switch
    // to proptest for testing here.
    rusty_fork_test! {
        #[test]
        fn test_any_data() {
            let seed: u64 = match std::env::var("SEED") {
                Ok(val) => str::parse(&val).expect("parsing number"),
                Err(_) => std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
            };
            println!("Seed: {seed}");

            let mut rng = StdRng::seed_from_u64(seed);
            let length = rng.gen_range(0..614400usize);
            let data = {
                let mut data = vec![0u8; length];
                rng.fill(&mut data[..]);
                data
            };
            let data_length = data.len();

            let file = {
                // Build a tx using data as a cell
                let context = Context::default();
                let tx = TransactionBuilder::default()
                    .witness(data.pack())
                    .build();

                let mock_tx = context.dump_tx(&tx).expect("dump tx");

                // Keep the tx in a temporary file, then set the environment
                // variable for ckb-x64-simulator
                let json = serde_json::to_string_pretty(&mock_tx).expect("json");
                let mut file = tempfile::NamedTempFile::new().expect("tempfile");
                file.write_all(json.as_ref()).expect("write");
                file.flush().expect("flush");
                unsafe { std::env::set_var("CKB_TX_FILE", file.path()); }
                file
            };

            assert_eq!(foo(), data_length);

            drop(file);
        }
    }
}
