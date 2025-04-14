# General Guidline for Metamask Snaps Audit 

- Typescript / Javascript
- Target on a small piece of code under `packages/snap/src`
- Usually audit in 2-5 labor-days
- Potential multiple clients 
- KS is one of the approved Auditors listed in Consensys 


## Audit Requirements 

### From Consensys 

- The audit should cover the Snap code itself that will run in MetaMask. 
- The commit hash that was audited and the commit hash of any fixes, if applicable, should be stated clearly in the audit report. 
- Any audit findings that are medium/major or greater severity should be addressed. 
- Once the audit is complete and these issues have been addressed, share the audit report with the MetaMask Snaps team for final review.
- Updated snaps do not require additional security audits, but they must be manually reviewed by the MetaMask Snaps team. 

### Others

- [Snaps security guidelines](https://docs.metamask.io/snaps/concepts/security-guidelines/)
- [Guidelines for Account Snaps](https://github.com/MetaMask/keyring-api/blob/main/docs/security.md#security)

## Setup Environment

- npm, yarn, pnpm (may require specific versions.)
    - open `package.json` and check `packageManager`. For example, if "packageManager": "yarn@3.2.1", then yarn should be used.

- [Metamask Flask](https://metamask.io/flask/)

## Scope

- The source code are usually located under packages/snap/src. Sometimes, other folders under packages needs to be audited, e.g. site or utils.

- cloc --exclude-dir=tests,images packages/

## Test

- Integration test
    - Run on the Metamask Flask (local server). e.g.
        - yarn install 
        - yarn start
        
    - For more info, see https://docs.metamask.io/snaps/get-started/quickstart/

- TODO [Test a Snap](https://docs.metamask.io/snaps/how-to/test-a-snap/)
- npm test / yarn test / pnpm run test


## Static Analysis

- typescript-eslint

    - [getting-started](https://typescript-eslint.io/getting-started)
    - e.g. yarn eslint . > audit_logs/eslint.log 2>&1

- Semgrep

```
semgrep --config auto --output audit_logs/semgrep-result.emacs --emacs . --verbose
```

- npm audit / yarn npm audit 

- Horusec
    - https://github.com/ZupIT/horusec
    - need Docker
    - horusec start -p . > audit_logs/horusec.log 2>&1

- Bearer
    - https://github.com/bearer/bearer
    - bearer scan .
    - but not very useful


For more tools, see https://owasp.org/www-community/Source_Code_Analysis_Tools.



## Manual Analysis


Typically focus on 

- input validation
    - Snap API should strictly validate inputs.
    - Setting HTML from code is risky because it’s easy to inadvertently expose users to a cross-site scripting (XSS) attack.

- dependencies 
    - Some dependencies are only used for testing or development purposes and should therefore be listed as devDependencies in package.json.

- return check
- user confirmation
- ...

