# What is this?

This is based on Ken's repository at https://gitlab.kudelski.com/toler/solana-semgrep
This repository contains Semgrep rules that should match common Solana program pitfalls.

# Installation

```
pip install semgrep
```

# Usage

Run: 

```
semgrep --config ./rust/ <path_to_codebase>
```

This will run all semgrep rules in the `rust` directory on the target directory.

# Vulnerabilities checked

This is a list of vulnerabilities being checked by the rules in this repository

* use of static mut variables
* potentially unnecessary use of unwrap() and expect()
* raw solana-specific known issues
  * account type confusion
  * arbitrary cross program invocation
  * missing ownership/signer checks
  * unchecked cast
* Anchor-specific known issues
  * mostly in the table below
* and everything that Soteria premium claims to detect (see table below)

## Soteria premium

See https://www.soteria.dev/post/announcing-soteria-premium-auto-auditor-for-solana-smart-contracts

* Issues marked with [x] are implemented as Semgrep rules in this repository
* Issues marked with [-] will not be implemented because it is believed that it makes no sense to implement those as a Semgrep rule (see git commit messages for details)
* Issues marked with [ ] are not implemented yet

| Implemented? | SVE     | 	Checker                | Description                                                                                          | Explanation |
|--------------|---------|-------------------------|------------------------------------------------------------------------------------------------------|-------------|
| [x]          | SVE1001 | MissingSignerCheck      | The account is missing signer check                                                                  | Example     |
| [x]          | SVE1002 | MissingOwnerCheck       | The account is missing owner check                                                                   | Example     |
| [x]          | SVE1003 | IntegerAddOverflow      | The add operation may result in overflows                                                            | Example     |
| [x]          | SVE1004 | IntegerUnderflow        | The sub operation may result in underflows                                                           | Example     |
| [x]          | SVE1005 | IntegerMulOverflow      | The mul operation may result in overflows                                                            | Example     |
| [x]          | SVE1006 | IntegerDivOverflow      | The div operation may result in overflows                                                            | Example     |
| [x]          | SVE1007 | UnverifiedParsedAccount | The account is not validated before parsing its data                                                 | Example     |
| [x]          | SVE1008 | DuplicateMutableAccount | These two accounts are both mutable and may be the same account                                      | Example     |
| [x]          | SVE1009 | InsecureAccountClosing  | The account is not securely closed                                                                   | Example     |
| [x]          | SVE1010 | TypeFullCosplay         | These two account data types are fully compatible and can be used to launch type confusion attacks   | Example     |
| [x]          | SVE1011 | TypePartialCosplay      | These two account data types are partially compatible and may be exploited by type confusion attacks | Example     |
| [x]          | SVE1012 | DivideByZero            | The arithmetic operation may result in a div-by-zero error                                           | Example     |
| [x]          | SVE1013 | AccountReInitialization | The account is vulnerable to program re-initialization                                               | Example     |
| [x]          | SVE1014 | BumpSeedNotValidated    | The account's bump seed is not validated and may be vulnerable to seed canonicalization attacks,     | Example     |
| [x]          | SVE1015 | InsecurePDASharing      | The PDA sharing with these seeds may be insecure                                                     | Example     |
| [x]          | SVE1016 | ArbitraryCPI            | The spl_token account may be arbitrary                                                               | Example     |
| [x]          | SVE1017 | MaliciousSimulation     | The program may contain malicious simulation                                                         | Example     |
| [x]          | SVE1018 | UnsafeSysVarAPI         | The sysvar instructions API is unsafe and deprecated (wormhole exploit)                              | Example     |
| [-]          | SVE1019 | UnvalidatedAccount      | The account is not properly validated and may be untrustful                                          | Example     |
| [-]          | SVE1020 | OutdatedDependency      | The program has outdated and vulnerable dependencies                                                 | Example     |
| [x]          | SVE1021 | UnsafeRust              | The program contains unsafe Rust code                                                                | Example     |
| [-]          | SVE1022 | OverPayment             | The code misses checking to prevent over payment                                                     | Example     |
| [-]          | SVE1023 | StalePriceFeed          | The code may use a stale price feed (solend loss)                                                    | Example     |
| [-]          | SVE1024 | MissInitTokenMint       | The init instruction misses minting pool tokens                                                      | Example     |
| [ ]          | SVE1025 | MissRentExempt          | The account misses rent exempt check                                                                 | Example     |
| [x]          | SVE1026 | MissFreezeAuthority     | The account misses checking for freeze authority                                                     | Example     |
| [x]          | SVE1027 | FlashLoanRisk           | The instruction may suffer from flash loan attacks                                                   | Example     |
| [x]          | SVE1028 | InconsistentRounding    | The arithmetics here have inconsistent rounding                                                      | Example     |
| [x]          | SVE1029 | CastTruncation          | The cast operation here may lose precision due to truncation                                         | Example     |
| [-]          | SVE2001 | IncorrectLogic          | Loop break instead of continue (jet-v1 exploit)                                                      | Example     |
| [-]          | SVE2002 | IncorrectCalculation    | Liquidation condition should be > instead of >=                                                      | Example     |
| [-]          | SVE2003 | ExponentialCalculation  | The calculation has exponential complexity                                                           | Example     |
| [ ]          | SVE3001 | BestSecurityPractice    | The code does not follow best security practices                                                     | Example     |
| [-]          | SVE3002 | RedundantCode           | The code is redundant or unused                                                                      | Example     |
| [ ]          | SVE3003 | InconsistentAnchor      | The program uses Anchor inconsistently across different instructions                                 | Example     |
| [ ]          | SVE3004 | InconsistentConfig      | The configuration and initialization data are inconsistent                                           | Example     |


# References

* https://blog.neodyme.io/posts/solana_common_pitfalls
* https://medium.com/coinmonks/from-ethereum-smart-contracts-to-solana-programs-two-common-security-pitfalls-and-beyond-ea5b919ade1c
* https://blog.supercompiler.xyz/soteria-a-vulnerability-scanner-for-solana-smart-contracts-cc202cf17c99

Note: Soteria looks suspicious to me. Don't run it blindly :)

# Articles about programming on Solana

These are not directly security related, but interesting nonetheless.

* https://solongwallet.medium.com/solana-development-tutorial-things-you-should-know-before-structuring-your-code-807f0e2ee43
* https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/
* https://brson.github.io/2021/06/08/rust-on-solana

