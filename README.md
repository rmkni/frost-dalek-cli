# FROST-CLI

## About

### What are Threshold Signature Schemes

Threshold signature schemes are cryptographic protocols that enable a group of participants to collectively share ownership of a private key. In such schemes, a predefined threshold determines the minimum number of participants who must work together in order to perform operations such as signing a piece of data. This approach enhances both security and distributed authority.

### What is Schnorr Signature

Schnorr signatures are a type of digital signature scheme that relies on the mathematical principles of discrete logarithms. They offer security features such as resistance to forgery and support for efficient signature verification. Additionally, Schnorr signatures can be utilized in zero-knowledge protocols, allowing one party to prove knowledge of a secret without revealing the secret itself.

### What is FROST

FROST, or Flexible Round-Optimized Schnorr Threshold Signatures, is a cryptographic protocol that aims to improve the efficiency of **threshold signatures** using **Schnorr signatures**. A common challenge with traditional threshold signature schemes is the need for participants to exchange data and perform operations over multiple rounds in a pre-defined order. FROST addresses this limitation by separating the distributed key generation phase from the signature phase, enabling signature creation to occur in a single network round. This optimization reduces latency and enhances overall performance.

See the [paper](https://eprint.iacr.org/2020/852) by **Chelsea Komlo** and **Ian Goldberg**.

### What is FROST-Dalek

FROST-Dalek is a Rust implementation of the **FROST protocol**, utilizing the [Dalek](https://github.com/dalek-cryptography/curve25519-dalek) library for elliptic curve cryptography. This project provides a secure and efficient implementation of FROST, enabling developers to leverage the benefits of threshold signatures in their applications.

### What is FORST-CLI

FORST-CLI is a Rust Command Line Application that allows you to interact with the **FROST-Dalek library** and perform key pair generation and signature management for threshold signatures. This application provides a user-friendly interface for executing the FROST protocol, enabling you to easily generate key pairs, initiate signature rounds, and verify signatures.

## Features (not implemented)

### Phase 1: Key Pair Generation

- Generate key pairs for participants within a specified group.
- Generate a corresponding key pair for the group itself.
- Serialize and dump the keys in ASN format for other exchange or applications to use.

*Note: Phase 1 must be completed before proceeding to Phase 2.*

### Phase 2: Signature Management

- Initiate a signature round using the previously generated key pairs.
- Allow one participant to sign a message
- Close a signature round
- Verify a signature

*Note: Once Phase 1 is complete, Phase 2 can be executed an arbitrary number of times.*

### Limitations

The subset of participants required to meet the threshold must be predetermined before the signature round commences.
Keys dump on disk are not protected with passwords.

## Installation

Clone the repository and ensure you have Cargo (Rust package manager) installed. If not, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

```bash
cargo build
env RUST_LOG=info cargo run 3 5
```

## Usage (not implemented)

```bash
Usage:

# Common Parameters
-g, --group <group_path>       Specify the input/output path for keys
-f, --format <keys_format>     Specify the format of the keys
-c, --context <context>        Provide the context for signature creation or verification
-m, --message <message>        Provide the message for signature creation or verification

create group <threshold> <number_of_participants>
        # Create a new group with the specified threshold and number of participants

create signature -o <output_path>
        # Create a signature for the given participant and context

sign -o <output_path> <participant>
        # Sign a message for the specified group and participant

close signature -o <output_path>
        # Close the signature round and generate the final signature

verify -s <signature_path>
        # Verify the provided signature using the group key
```

Folders must follow file structure:

```bash
/path/to/keys/
    ├── group
    ├── group.pubkey
    ├── participant_i
    ├── participant_i.pubkey
    └── signers/
        └── participant_j
        └── participant_j.pubkey    
```

Move participants in `signers` folder to include them in a signature.

## Contributing

This project is under development in very early stage thus not open for pull requests.

The repository owner is available to engage in discussions and collaborate on projets, feel free to make contact by mail from github profile.