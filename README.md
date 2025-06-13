# A super simple SP1 program

This repo contains the code for:

- A piece of Rust code that adds two numbers in a way that a ZK proof can be generated that testifies to the correct execution
of the program against an input of two private integers.

- A script that generates a proof for each execution of the program, and verifies said proof


## Set up dev environment

1. Clone the repo

```bash
git clone https://github.com/Genesis3800/sp1-simple-sum-program.git
```

2. Make sure all Rust and SP1 dependencies are installed.

- Refer to SP1 docs here: [docs.succinct.xyz/docs/sp1/getting-started/install](https://docs.succinct.xyz/docs/sp1/getting-started/install)

3. `cd` into the cloned repo & install the dependencies:

```bash
cargo build
```

## Compile the RISC-ELF

1. `cd` into the `program` directory

```bash
cd program
```

2. Compile the RISC-ELF

```bash
cargo prove build
```


## 'Execute' the program

Execution in thie context means running the program through Succinct's SP1 ZKVM,
but without generating a proof.

Helps in:
- Performing a sanity check on the program
- Getting a sense of how much resources the program will consume when generating a proof

1. `cd` into the `script` directory

```bash
cd script
```

2. Run the following command to execute the program:

```bash
cargo run -- execute --a 3 --b 500
```

![execute](https://github.com/user-attachments/assets/26bdc828-349a-4a05-bb03-72245476a7f8)


## Generate a proof

1. Make sure you're in the `script` directory

2. Run the following command to generate a proof:

```bash
cargo run -- prove --a 3 --b 500
```

![proof](https://github.com/user-attachments/assets/a7011966-04e5-4086-84cf-6b0a21bd5777)


## Verify a proof

1. Make sure you're in the `script` directory

2. Run the following command to verify a proof:

```bash
cargo run -- verify
```

![verify](https://github.com/user-attachments/assets/b08a4d3a-3dd8-4258-a346-b43f69aef1b0)


