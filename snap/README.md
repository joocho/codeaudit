# Metamask Snap

MetaMask Snaps is an open source system that allows anyone to safely extend the functionality of MetaMask, creating new web3 end user experiences.

The MetaMask Snaps team maintains a list of approved auditors that can audit Snaps code. For a list of approved auditors or to submit an audit report, please reach out to Manish Tomer manish.tomer@consensys.io.



## Tutorial

- [Extend the functionality of MetaMask using Snaps](https://docs.metamask.io/snaps/)
- [MetaMask 🛠 Extend MetaMask with Snaps (video)](https://www.youtube.com/watch?v=GLI4iQvfrcs)
- [MetaMask 🛠 Snaps Workshop (video)](https://www.youtube.com/watch?v=qkFKrLtJ3eY&t=540s)
- [metamask snap](https://docs.metamask.io/snaps/tutorials/)
- [Transaction Insights workshop](https://github.com/ziad-saab/metamask-snaps-transaction-insights-workshop)

## Test

- [MetaMask Test Dapp](https://metamask.github.io/test-dapp/)

## Security Guideline

- [Snaps security guidelines](https://docs.metamask.io/snaps/concepts/security-guidelines/)

### Others

- [metamask Guidelines for Account Snaps](https://github.com/MetaMask/keyring-api/blob/main/docs/security.md)
- [Open Beta Readiness Guide](https://github.com/MetaMask/snaps/discussions/1411)
- [Requirement document from web3auth](https://docs.google.com/document/d/1k1bYIgFPB8gSm02-lsxOJUV9CidBd0yQGfo23CfCT5A/edit?usp=sharing)
- [Sayfer Essential Guidelines from an Approved Auditor](https://sayfer.io/blog/auditing-metamask-snaps/)

### Misc

- dApp should be untrusted. 
- Snap API should strictly validate inputs.
- Setting HTML from code is risky because it’s easy to inadvertently expose users to a cross-site scripting (XSS) attack.
- Some dependencies are only used for testing or development purposes and should therefore be listed as devDependencies in package.json.


## Static analysis  and checkpoints

- [KS typescript](https://gitlab.kudelski.com/iotlabs/code-audit/-/blob/master/typescript/typescript.md) 


## Public audit report

- [consensys](https://consensys.io/diligence/audits/)
- [least authority](https://leastauthority.com/security-consulting/published-audits/#2023)
- [cure53 Pentest](https://cure53.de/#publications)
- [sayfer](https://sayfer.io/audits/metamask-snap-audit-report-for-demystify/)


## Analysis of other audit reports

| Client | Snap | Days | Loc | Findings | Auditor   |
| ------ | ------ | ------ | ------ | ------ | ------ |
| Solflare Wallet | aptos-snap (Rise Wallet) <br />  solana-snap (Solflare) <br />  sui-snap (Elli Wallet)   |  9      | 242<br /> 234<br /> 247    |   | consensys |   
| Protocol Labs  |  FilSnap  |  5   |  2751  |    | consensys   | 
|   ShapeShift  | MetaMask Snap  |   2x5     | 2162  |     | consensys |
| PushProtocol  |  Snap v1 |   2     | 249  |     |  consensys |      
| ConsenSys  |  MetaMask Starknet Snap |   4     | 2485  | input validation, user confirmation, return check  |  consensys |        
| MetaMask   |  MobyMask MVP Snaps |   5     | 163 | Dependencies vuln.    |  consensys       |
| XMTP | MetaMask Snap  | July 20 - 24  |  690  |    |  Least Authority |        
| Blockfence   | Snap  |   June 19 - 21   |  81 |    |  Least Authority |        
| Blockchain Lab  | Masca MetaMask Snap |  Aug 14 - 23   |  3380  |    |  Least Authority |        
| Staging Labs  | Saferoot Snap |  Aug 15 - 22  |    |    |  Least Authority |        
| Generative Labs | Web3MQ Snap | Aug 16 - 18   | 1920   |    |  Least Authority |        
| Consensys  | MetaMask Snap |   April 4 - June 8   |    |    |  Least Authority |        
| Consensys  | MetaMask Snaps-Extension | June 13 - July 17 |   |     |  Least Authority |        


### MetaMask Starknet Snap Audit

- contractCallData is a user data and may contain Markdown renderable strings or strings containing Control Characters that break the context of the message displayed to the user. 
- address length is not validated.  
- The signing request dialogue does not display the user account that is being used to sign the message.
- Unnecessary Use of dangerouslySetInnerHTML 
- The RPC method upserts ERC20 tokens received via RPC without asking the user for confirmation. 
- getKeysFromAddress() may return null but most callers of the function do not check for the null condition. 
- Potentially untrusted inputs, e.g. addresses received via RPC calls, are not always checked to conform to the StarkNet address format. 
- Debug Log is not disabled for production build.

### Solflare/Sui/Aptos MetaMask Snap Audit

- Allowing the dapp to control if the data to be signed is displayed to the user is dangerous as the dapp may silently ask for a signature to sign data the user did not intend to sign.
- the text() component can render Markdown or allow for control character injections. request.params is considered untrusted. For example, request.params.simulationResult[] may contain Markdown renderable strings or Control Characters that can disrupt the context of the user-displayed message. 
- deriveKeyPair is not checked for valid key derivation path format which may lead to unexpected outcomes or unhandled exceptions.
- The snaps RPC access is restricted to certain origins only. However, there is no logic that disables development/test domains from origin checks in production builds.



### Web3MQ MetaMask Snap Audit
