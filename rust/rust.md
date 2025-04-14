# Solana/Rust Security

## Resources

- [Solana Simplified: Master Solana Core Concepts in One Read](https://blocksec.com/blog/solana-simplified-master-solana-core-concepts-in-one-read)


- Neodyme
[Solana Smart Contracts: Common Pitfalls and How to Avoid Them](https://blog.neodyme.io/posts/solana_common_pitfalls/)


- [KS Blockchain Heroes](https://gitlab.kudelski.com/KS-Innovation/blockchain-kb/-/blob/main/Solana/Educational%20Resources/Blockchain%20Heroes.md)

 
- [Rustsec](https://rustsec.org/): 
The RustSec Advisory Database is a repository of security advisories filed against Rust crates published via https://crates.io maintained by the Rust Secure Code Working Group.

- [Solana Auditing and Security Resources](https://github.com/0xsanny/solsec):
A collection of resources to study Solana smart contract security, auditing, and exploits.

- [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html): 
Rust’s memory safety guarantees enforced at compile time. However, Rust has a second language hidden inside it that doesn’t enforce these memory safety guarantees: it’s called unsafe Rust and works just like regular Rust, but gives us extra superpowers.

- [Sealevel attacks](https://github.com/coral-xyz/sealevel-attacks): 
Solana Vulnerabilities and Exposures (SVE)

- [Rust Secure Code Working Group](https://github.com/rust-secure-code):
Make it easy to write secure Rust code. Working group for supply chain security tools including @rustsec and `cargo-geiger`

- ANSSI Checklist
[Secure Rust Guidelines](https://anssi-fr.github.io/rust-guide/checklist.html)

## Static analyzer 

###	Semgrep 
[Semgrep](https://github.com/returntocorp/semgrep) is a fast, open-source, static analysis tool for finding bugs and enforcing code standards at editor, commit, and CI time.

- [Internal workshop by Nils](https://nagragrp.sharepoint.com/:f:/s/KSResearch/EjdZe5Rt_SFNiUAwypdpbIEBQb-YnHfkKy5D1iUOIryQug?e=qP8J4u)

- Using global ruleset: semgrep --config auto --output ./semgrep-result.emacs --emacs . --verbose

- Using local ruleset: semgrep --config ./solana-semgrep/rust --output ./semgrep-result.emacs --emacs . --verbose

- Internal ruleset
    - [Ken Toler ](https://gitlab.kudelski.com/toler/solana-semgrep)
    - [Nils ](https://gitlab.kudelski.com/KS-Innovation/solana-semgrep)
    - [trail of bits ](https://github.com/trailofbits/semgrep-rules)


### Cargo tools

- cargo audit: to monitor unmaintained dependencies. https://crates.io/crates/cargo-audit 

- cargo clippy: https://github.com/rust-lang/rust-clippy

- cargo edit: https://crates.io/crates/cargo-edit
    - install: cargo install cargo-edit
    - run: cargo upgrade --incompatible --dry-run

- Cargo miri test
    + rustup +nightly component add miri
    + https://nexte.st/book/miri.html
    + cargo clean
    + cargo miri test
    + https://github.com/rust-lang/miri
    + if too slow, try [nexttest](https://nexte.st/docs/integrations/miri/).

- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
    + cargo llvm-cov --html

- cargo outdated: https://github.com/kbknapp/cargo-outdated

- cargo geiger: to monitor unsafe calls. https://github.com/rust-secure-code/cargo-geiger  

- cargo-deny: https://github.com/EmbarkStudios/cargo-deny

- cargo supply-chain publishers: https://github.com/rust-secure-code/cargo-supply-chain

- cargo crev: a system for verifying security and reliability of dependencies based on collaborative code reviews. 
    - https://github.com/crev-dev/cargo-crev/blob/main/cargo-crev/src/doc/getting_started.md
    - cargo crev verify --show-all


- cargo-fmt
    + rustup component add rustfmt
    + cargo fmt
    + https://github.com/rust-lang/rustfmt

### Others

- Rudra: https://github.com/sslab-gatech/Rudra
- [dylint](https://github.com/trailofbits/dylint)



## Solana audit reports

- [BlockSec](https://github.com/blocksecteam/audit-reports/tree/main/rust)



## Constant-time code
Avoid to use function with execution time depending on secret input like:
* powm


## Learning Rust

- The book: https://doc.rust-lang.org/book/

- Rust by Example: https://doc.rust-lang.org/rust-by-example/

- Rustlings:  https://github.com/rust-lang/rustlings

- Explore the Rust exercises on Exercism: https://exercism.org/tracks/rust/exercises

- The udemy tutorial: https://www.udemy.com/course/rust-fundamentals/ 

- Reference cheat sheet for obscure syntax https://cheats.rs/ 

- Youtube: https://www.youtube.com/playlist?list=PLZaoyhMXgBzoM9bfb5pyUOT3zjnaDdSEP


### Fuzzing

+ cargo-test-fuzz: https://github.com/trailofbits/test-fuzz
    + install: cargo install cargo-test-fuzz cargo-afl
    + add target function "foo"
    + cargo test
    + cargo test-fuzz foo 

+ List of security-related projects - Fuzzers
    - https://github.com/rust-secure-code/projects#fuzzers

+ honggfuzz-rs
    - https://github.com/rust-fuzz/honggfuzz-rs
    
+ cargo-fuzz using libfuzzer behind 
    - https://rust-fuzz.github.io/book/introduction.html

+ Fuzzing forum 
    - https://github.com/google/fuzzing

+ fuzzing Solana
    + https://secret.club/2022/05/11/fuzzing-solana.html
    + https://secret.club/2022/05/11/fuzzing-solana-2.html


+ libFuzzer Tutorial 
    - https://github.com/google/fuzzing/blob/master/tutorial/libFuzzerTutorial.md

+ Training from fuzzing labs 
    + https://academy.fuzzinglabs.com/rust-security-audit-and-fuzzing-training

+ Fuzzers for Solidity
    + Scribble (Consensys) https://github.com/Consensys/scribble
    + Echidna (Trail of bits) https://github.com/crytic/echidna
    + Foundry / Forge https://github.com/foundry-rs/foundry



