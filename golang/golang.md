# cloc
Do not count test in number of lines estimation:
```bash
cloc internal/fptower --not-match-f '_test\.go$'
```

# Static analysis
## semgrep
Default semgrep rules:
```bash
semgrep --config=auto
```

Other interesting rules:
* https://github.com/dgryski/semgrep-go
* https://github.com/trailofbits/semgrep-rules

```bash
semgrep --config ~/software/semgrep-go/ .
```

Kudelski Security rules:
* https://gitlab.kudelski.com/KS-Innovation/go-semgrep

# Other tools
```bash
gosec ./...
go vet ./...
staticcheck ./...
ineffassign ./...
errcheck ./...
```

# golangci-lint
- https://golangci-lint.run/


# Code coverage
```bash
go test -coverprofile=cover.out
go tool cover -html=cover.out
```

# Cryptography libraries:
* Coinbase Kryptology: https://github.com/coinbase/kryptology
* Taurus: https://github.com/taurusgroup/multi-party-sig
* Consensys: https://github.com/ConsenSys/gnark-crypto