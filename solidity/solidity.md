## Solidity Checklist

- [BabelBlock Checker-openzeppelin](https://github.com/OpenZeppelin/BabelBlock-Checker)
- [Style guide](https://docs.soliditylang.org/en/v0.8.16/style-guide.html)
- [Security consideration](https://docs.soliditylang.org/en/v0.8.16/security-considerations.html)
- [Consensys](https://consensys.github.io/smart-contract-best-practices/attacks/)
- [SWC Registry](https://swcregistry.io/)
- [CVE](https://www.cvedetails.com/vulnerability-list/vendor_id-17524/Ethereum.html)
- [Sigma Prime](https://github.com/sigp/solidity-security-blog)
- [Securing](https://github.com/securing/SCSVS)
- [cryptofinlabs](https://github.com/cryptofinlabs/audit-checklist)
- [NCCgroup](https://dasp.co/)
- [Slither](https://github.com/crytic/slither)
- [Guy Lando](https://github.com/guylando/KnowledgeLists/blob/master/EthereumSmartContracts.md)
- [Ethereum Smart Contract Auditor's 2022 Rewind](https://ventral.digital/posts/2022/12/15/ethereum-smart-contract-auditors-2022-rewind)

## Smart Contract Weakness Classification Registry
- [SWC-registry](https://github.com/SmartContractSecurity/SWC-registry/) (MythX): SWC definitions and a large repository of crafted and real-world samples of vulnerable smart contracts. https://swcregistry.io/
- Ethereum Smart Contract Best Practices https://consensys.github.io/smart-contract-best-practices/


## Static analyzer for Solidity

- [Building Secure Smart Contracts -trail of bits](https://github.com/crytic/building-secure-contracts)
    - [Echidna-devcon-video](https://www.youtube.com/watch?v=yYnQ9g_j_2c)
- [Ethereum Security Toolbox](https://github.com/trailofbits/eth-security-toolbox/)
    + [Echidna](https://github.com/crytic/echidna) property-based fuzz tester: Echidna is a weird creature that eats bugs and is highly electrosensitive (with apologies to Jacob Stanley) The only available fuzzer for Ethereum software. Uses property testing to generate malicious inputs that break smart contracts.
    + [Medusa](https://github.com/crytic/medusa) fuzz tester based on go-ethereum
    + [Slither](https://github.com/crytic/slither) static analysis tool: Slither is a Solidity static analysis framework written in Python 3. It runs a suite of vulnerability detectors, prints visual information about contract details, and provides an API to easily write custom analyses. Static analysis framework with detectors for many common Solidity issues. It has taint and value tracking capabilities and is written in Python.
    + solc-select to quickly switch between Solidity compiler versions 
    + Building secure contracts repository
- [Mythril](https://github.com/ConsenSys/mythril): Security analysis tool for EVM bytecode. Supports smart contracts built for Ethereum, Hedera, Quorum, Vechain, Roostock, Tron and other EVM-compatible blockchains. The Swiss army knife for smart contract security.
    + myth analyze src/[target].sol --solc-json remappings.json
    + Example: remappings.json
```
{
"remappings": [
    "@openzeppelin/contracts/=lib/openzeppelin-contracts-upgradeable/lib/openzeppelin-contracts/contracts/",
    "@openzeppelin/contracts-upgradeable/=lib/openzeppelin-contracts-upgradeable/contracts/",
    "forge-std/=lib/forge-std/src/"
  ]
}
```
- [MythX](https://mythx.io/) (consensys): MythX is a professional-grade cloud service that uses symbolic analysis and input fuzzing to detect common security bugs and verify the correctness of smart contract code. Using MythX requires an API key from mythx.io.
- [Securify](https://github.com/eth-sri/securify2) (ETH foundation) - Fully automated online static analyzer for smart contracts, providing a security report based on vulnerability patterns.
- [Manticore](https://github.com/trailofbits/manticore) (trailofbits) - Dynamic binary analysis tool with EVM support.
- [Octopus](https://github.com/FuzzingLabs/octopus): Security Analysis tool for WebAssembly module (wasm) and Blockchain Smart Contracts (BTC/ETH/NEO/EOS) Security Analysis tool for Blockchain Smart Contracts with support of EVM and (e)WASM.
- [MadMax](https://github.com/nevillegrech/MadMax) - Static analysis tool for gas DoS vulnerabilities  in Ethereum smart contracts.
- [Gigahorse](https://github.com/nevillegrech/gigahorse-toolchain) - Fast binary lifter and program analysis framework written in Datalog.
- [Oyente](https://github.com/enzymefinance/oyente) - Analyze Ethereum code to find common vulnerabilities, based on this paper.
- [SmartCheck](https://smartdec.net/) - Static analysis of Solidity source code for security vulnerabilities and best practices.
- [Vertigo](https://github.com/JoranHonig/vertigo) - Mutation Testing for Ethereum Smart Contracts.
- Contract-Library - Decompiler and security analysis tool for all deployed contracts.
- [sFuzz](https://contract.guardstrike.com/#/scan) - Efficient fuzzer inspired from AFL to find common vulnerabilities.
- [Fuzzland](https://github.com/fuzzland/ityfuzz) - a blazing-fast EVM and MoveVM smart contract hybrid fuzzer that combines symbolic execution and fuzzing to find bugs in smart contracts offchain and onchain.
- [Wake framework](https://github.com/Ackee-Blockchain/wake)


## Semgrep for Ethereum Solidity
Semgrep is a static code analyzer using an internal set of rules. It produces false positives because the rules are not 100% accurate but it can help detect hotspots for further manual investigation.

+ (local rules) semgrep --config ~/semgrep/solidity-semgrep/solidity-decurity/ --emacs --output audit_logs/semgrep-result-local.emacs . --verbose
+ (global rules) semgrep --config "p/smart-contracts" --emacs --output audit_logs/semgrep-result-global.emacs  ./ --verbose

+ To upgrade semgrep, ```python3 -m pip install --upgrade semgrep```

- https://semgrep.dev/docs/
- https://github.com/Decurity/semgrep-smart-contracts
- https://semgrep.dev/p/smart-contracts



## Learning

- [Solidity by Example](https://solidity-by-example.org/)
- [Awesome Solidity](https://github.com/bkrem/awesome-solidity?tab=readme-ov-file#tutorials)



