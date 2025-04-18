![Build Status](https://gitlab.com/pages/plain-html/badges/master/build.svg)

---
## Semgrep for Ethereum Solidity
Semgrep is a static code analyzer using an internal set of rules. It produces false positives because the rules are not 100% accurate but it can help detect hotspots for further manual investigation.

### External Resources 

- https://semgrep.dev/docs/


###	Internal resources

- [Solana semgrep](https://gitlab.kudelski.com/KS-Innovation/solana-semgrep)
- [Algorand semgrep](https://gitlab.kudelski.com/emea-blockchain/algorand-semgrep)
- [Internal workshop by Nils](https://nagragrp.sharepoint.com/:f:/s/KSResearch/EjdZe5Rt_SFNiUAwypdpbIEBQb-YnHfkKy5D1iUOIryQug?e=qP8J4u)

### Usage examples

- Solidity: semgrep --config ./solidity-semgrep <path_to_codebase>
- Rust: semgrep --config ./solana-semgrep/rust <path_to_codebase>
- auto: semgrep --config=auto <path_to_codebase>
- python: semgrep –config “p/python” <path_to_codebase>


### Checklist for Solidity
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

### Rules example

```
rules:
 -
    id: erc777-reentrancy
    message: ERC777 tokensReceived() reentrancy
    metadata:
        references:
        - https://mirror.xyz/baconcoin.eth/LHaPiX38mnx8eJ2RVKNXHttHfweQMKNGmEnX4KUksk0
        - https://etherscan.io/address/0xf53f00f844b381963a47fde3325011566870b31f
        category: reentrancy
        tags:
        - erc777
        - bacon-protocol
    patterns:
    - pattern: $X.tokensReceived(...);
    languages:
    - solidity
    severity: WARNING     
```

