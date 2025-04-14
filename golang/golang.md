# cloc
Do not count test in number of lines estimation:
```bash
cloc internal/fptower --not-match-f '_test\.go$'
```
# Checklist

- [KS Go for Audit](https://gitlab.kudelski.com/iotlabs/code-audit/-/blob/master/golang/golang.md)
- [KS TSS for Audit](https://gitlab.kudelski.com/emea-blockchain/tss-zkp-fhe/-/blob/main/tss-audit.md)
- [Golang Audit Checklist](https://www.dylanlott.com/golang-security-audit-checklist/)
- [Checklist to evaluate MPC deployments](https://www.reparaz.net/oscar/misc/evaluating-mpc)


# Static analysis

- https://github.com/avelino/awesome-go#code-analysis
  
## semgrep
Default semgrep rules:
```bash
semgrep --config auto --output audit_logs/semgrep-result.emacs --emacs ./pkg/ --verbose
```
Other interesting rules:
* https://github.com/dgryski/semgrep-go
* https://github.com/trailofbits/semgrep-rules
* https://gitlab.kudelski.com/KS-Innovation/go-semgrep

```bash
semgrep --config ~/software/semgrep-go/ .
```

# Other tools
```bash
gosec -tests ./...
go vet ./...
staticcheck ./...
ineffassign ./...
errcheck ./...
```

## codeQL for go
```
codeql database create codeqldb --language=go
codeql database analyze --format=csv --output=audit_logs/codeql-results.txt   codeqldb 'codeql/go-queries:codeql-suites/go-security-and-quality.qls'
```

- https://codeql.github.com/docs/codeql-language-guides/codeql-for-go/
- https://github.com/github/codeql/tree/main/go/ql/src/codeql-suites

## golangci-lint
- https://golangci-lint.run/


## Code coverage
```bash
go test -coverprofile=cover.out
go tool cover -html=cover.out
```

## Fuzzing

- [Tutorial: Getting started with fuzzing](https://go.dev/doc/tutorial/fuzz)
- [Go-Ethereum Fuzzers](https://github.com/ethereum/go-ethereum/tree/master/tests/fuzzers)


# Cryptography libraries:
* Coinbase Kryptology: https://github.com/coinbase/kryptology
* Taurus: https://github.com/taurusgroup/multi-party-sig
* Consensys: https://github.com/ConsenSys/gnark-crypto
* [Awesome Go](https://github.com/avelino/awesome-go)
* [Awesome Go Security](https://github.com/Binject/awesome-go-security)
* [Awesome Goland Security](https://github.com/guardrailsio/awesome-golang-security)
* [Two-Round Threshold Schnorr Signatures with FROST](https://datatracker.ietf.org/doc/draft-irtf-cfrg-frost/)
* [not go-ing anywhere](https://github.com/trailofbits/not-going-anywhere)

A list of open-source software designed for MPC as well as introductory material.
* [awesome-mpc](https://github.com/rdragos/awesome-mpc)
