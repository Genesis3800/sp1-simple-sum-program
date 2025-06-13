#![warn(clippy::all)]

use anyhow::Result;
use bincode;
use clap::{Parser, Subcommand};
use sp1_sdk::{
    include_elf,  // Embed the program ELF by name :contentReference[oaicite:5]{index=5}
    HashableKey,  // Extension trait for bytes32_raw() :contentReference[oaicite:6]{index=6}
    ProverClient, // The client for prove/execute/verify
    SP1ProofWithPublicValues, // Proof + public-values bundle
    SP1Stdin,     // To write inputs
    SP1VerifyingKey, // For deserializing raw vkey bytes
};
use std::time::Instant; 

/// Embed the ELF for the `sum` program (built automatically by build.rs).
const ELF: &[u8] = include_elf!("sum");

/// CLI definition
#[derive(Parser, Debug)]
#[clap(author, version, about = "SP1 addition zkVM demo")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute without proof
    Execute {
        #[clap(short, long)]
        a: u32,
        #[clap(short, long)]
        b: u32,
    },
    /// Generate a zk-proof
    Prove {
        #[clap(short, long)]
        a: u32,
        #[clap(short, long)]
        b: u32,
    },
    /// Verify a proof (`proof.bin` + `public_values.bin` + `vkey.bin`)
    Verify,
    /// Export only the verifying key (`vkey.bin`)
    Vkey,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = ProverClient::from_env();

    match cli.command {
        Commands::Execute { a, b } => {
            let mut stdin = SP1Stdin::new();
            stdin.write(&a);
            stdin.write(&b);

            let (output, report) = client.execute(ELF, &stdin).run()?;
            println!("₿ Execution output: {:?}", output);
            println!("⌛ Cycles count: {:?}", report.total_instruction_count());
        }

        Commands::Prove { a, b } => {
            // 1. Marshal the two inputs exactly as the guest will read them
            let mut stdin = SP1Stdin::new();
            stdin.write(&a);
            stdin.write(&b);

            // 2. Derive the proving key (pk) and verifying key (vk) for this ELF
            let (pk, vk) = client.setup(ELF);

            // extra: Measure wall-clock time
            let t0 = Instant::now();
            // 3. Run the prover to get a full proof bundle (proof + public values)
            let proof_bundle = client.prove(&pk, &stdin).run()?;

            // extra: Measure wall-clock time
            let elapsed = t0.elapsed();

            // 4. Save the entire bundle in one shot.
            //    Bundled file contains both the zero-knowledge proof and the public outputs.
            proof_bundle.save("proof_with_public_values.bin")?;

            // 5. Serialize the verifying key into 32-byte format and write it out
            let vk_bytes = bincode::serialize(&vk)?;
            std::fs::write("vkey.bin", vk_bytes)?;

            println!("✅ Wrote proof_with_public_values.bin and vkey.bin");
            println!("⏱️  Proving time: {:.2?}", elapsed);
        }

        Commands::Verify => {
            // Print the current directory for debugging
            println!("Current directory: {:?}", std::env::current_dir()?);

            // 1. Load the bundled proof + public values in one shot
            let proof_file_path = "proof_with_public_values.bin";
            println!(
                "Looking for proof file at: {:?}",
                std::path::Path::new(proof_file_path).canonicalize()
            );

            let proof_bundle = match SP1ProofWithPublicValues::load(proof_file_path) {
                Ok(bundle) => bundle,
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to load proof bundle: {}", e));
                }
            };

            // 2. Read & reconstruct the verifying key (From<Vec<u8>> conversion)
            let vk_bytes = std::fs::read("vkey.bin")?;
            let vk: SP1VerifyingKey = bincode::deserialize(&vk_bytes)?;

            // 3. Run the succinct verification
            client.verify(&proof_bundle, &vk)?;
            println!("🔑 Proof is valid!");
        }

        Commands::Vkey => {
            let (_, vk) = client.setup(ELF);
            std::fs::write("vkey.bin", vk.clone().bytes32_raw().to_vec())?;
            println!("🔑 vkey.bin written.");
        }
    }

    Ok(())
}
