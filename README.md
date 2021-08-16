# Solaris-poc

Solaris Proof of Concept

## Development

### Environment Setup

1. Install the latest Rust stable from https://rustup.rs/
2. Install Solana v1.6.1 or later from https://docs.solana.com/cli/install-solana-cli-tools
3. Install the `libudev` development package for your distribution (`libudev-dev` on Debian-derived distros, `libudev-devel` on Redhat-derived).

### Build

To build a program for the Solana BPF target:
```
$ cd program
$ cargo build-bpf
```

### Test

Unit tests can be run with:
```bash
$ cargo test-bpf  # <-- runs BPF program tests
```

## Deployment

To deploy a program to devnet:
```
$ solana program deploy ./target/deploy/poc_program.so
```
