# Guidelines for Teal

The following list of [guidelines](https://developer.algorand.org/docs/get-details/dapps/avm/teal/guidelines/) should be followed when working with TEAL.

| No. | Category |  Description | RuleID |
| ------ | ------ | ------ | ------ |
| 1 | Txn | TEAL code should validate as many txn fields as possible. Things not checked could become set to anything. | |
| 2 | RekeyTo | Always verify that the RekeyTo property of any transaction is set to the ZeroAddress unless the contract is specifically involved in a rekeying operation. | algorand-rekey-zero |
| 3 | RekeyTo | Version 2 of smart contracts must have the first line of code specified with #pragma version 2. If this is not in the code, the contract will be interpreted as a version 1 contract. Also, when upgrading version 1 contracts make sure to check for rekeying transactions as this could cause the account to be easily rekeyed to another account. | |
| 4 |  CloseRemainderTo  | CloseRemainderTo or AssetCloseTo should be the intended recipient or equal to global ZeroAddress. An unchecked address could steal all the value! | algorand-closeremainderto-zero |
| 5 | Amount   | When not closing out a contract account, Amount or AssetAmount should be checked to be equal to the intended transfer amount. | algorand-amount-check |
| 6 | Fee | Fee should be less than some reasonable amount (in micro-Algos). An unchecked Fee could burn the entire value of the contract account! | algorand-fee-limit, algorand-fee-set |
| 7 | Type   | Type or TypeEnum can be checked to ensure something is a Payment or AssetXfer or another transaction type.| algorand-type-enum | 
| 8 | Sender | Sender doesn’t need to be checked. It gets checked automatically.| |
| 9 | Limit   | Using a contract account limits your exposure as compared to a delegated account. (Mistakes and bugs can only lose as much as you put into that account. If you can close out that account it closes out any exposure.) | |
| 10 |  Counterpoint  | Using a regular private/public key account with delegated logic allows you to always use the private key to sign any transaction and not be locked into only what the logic allows.| |
| 11 | FirstValid   | Contract accounts should have an expiration in the future (txn FirstValid > NNNN, or txn FirstValidTime > TTTT) after which the originator can reclaim their assets.| algorand-first-valid |
| 12 |  Expiration  | Delegated LogicSigs may wish to specify an expiration round in the future after which they will no longer function.| |  
| 13 |  Group  | Transaction groups must be designed with ‘slots’ such that each transaction will know what position it is in. A transaction’s logic can check (txn GroupIndex == K && global GroupSize == M) and then also check specific other transactions by their index through gtxn N field |  |

## Examples

- txn[0] sends N Algos from A to B; program checks that (M/N >= desired exchange rate) and (gtxn 1 Receiver == A)|  
- txn[1] sends M AltCoin from B to A; program checks that (N/M >= desired exchange rate) and (gtxn 0 Receiver == B)
- Corollary to checking as many txn fields as possible: be strict with other transaction properties in a group.
- Verify the GroupSize to make sure the size corresponds to the number of transactions the logic is expecting.

## Tips and Patterns

In addition to the guidelines, the following tips and patterns may also be useful.

| No. | Category |  Description |
| ------ | ------ | ------ |
| 1 | CloseRemainderTo | A contract account that requires CloseRemainderTo or AssetCloseTo being equal to the intended recipient is good for a conditional all-or-nothing payment. |
| 2 | Payment | A recurring payment contract should combine (FirstValid % N == 0) and (Lease == xxxxx), where N is the period of the recurring payment in rounds, at least 1000. |
| 3 | Panic | Some operations panic and exit the transaction validation with an immediate fail. For example a divide-by-zero. It’s possible to test for these and skip panicking instructions with a bnz branch-if-not-zero instruction.|


## Most common pitfalls ([runtime verification](https://www.youtube.com/watch?v=FkU_P0r15Qs))

| No. | Category |  Description | RuleID |
| ------ | ------ | ------ | ------ |
| 1 | Unchecked transaction fields | execution path where Rekey/CloseRemainderTo/AssetCloseTo is not checked (also GroupSize), a fund-moving transaction where source/target accounts are not checked, modifying the wrong local state due to accounts/app Ids no checked against foreign argument | algorand-group-size, algorand-closeremainder-zero, algorand-rekeyto-zero |
| 2 | Faulty logic (Design errors)|design errors, incorrect arithmetc formulas, incorrect boolean checks, zombie code | |
| 3 | Fixed-point arithmetic errors | rounding errors vs overflow, intra-block vs inter-block calculations | |
| 4 | Authorization errors | unprotected privileged operations, e.g. global data manipulation, falsely assuming that contract addresses cannot be pre-computed => DoS attacks | |


## Confidential Report([KS])

| No. | Category |  Description | RuleID |
| ------ | ------ | ------ | ------ |
| 1 | AppArgs | Check that application_args length is equal to what supposed to be. Do not accept any other value | algorand-application-args |
| 2 | governance | The smart contract should be protected against unexpected upgrade. It is recommended that the smart contract should only accept contract upgrade with Algorand, not all chain. |  |
| 3 | Swap | Algorand smart contract should be aligned with Ethereum smart contract. |  |




## Tinyman Post-mortem Report([runtime verification](https://github.com/runtimeverification/publications/blob/main/reports/smart-contracts/Tinyman-security-review.pdf))

| No. | Category |  Description | RuleID |
| ------ | ------ | ------ | ------ |
| 1 | Unchecked asset ID | When a pay transaction or a asset transfer is required, its asset ID should be checked. If the asset ID is not checked, more valuable asset could be transfered instead of an intended asset! | algorand-asset-id |


## Algofi Assessment ([runtime verification](https://github.com/runtimeverification/publications/blob/main/reports/smart-contracts/Algofi.pdf))
| No. | Category |  Description | RuleID |
| ------ | ------ | ------ | ------ |
| 1 | Critical | Flipped price scales in liquidation reward calculation => custom business logic mistake potentially exploitable by hackers. | |
| 2 | High | Missing exchange logic in liquidation reward calculation => custom business logic mistake potentially exploitable by hackers | |
| 3 | High | Incorrect access of repaid amount in liquidation reward calculation => ALGO token can only be sent via "Payment" transaction while other tokens are sent by via "AssetTransfert" transaction, this can cause potentially exploitable bugs if a wrapper is not systematically used.| |
| 4 | High | Incorrect check for expected transaction type in some operations => Similar issue about ALGO being only useable in "Payment" transactions while some operation expects "AssetTransfert" transacation type | |
| 5 | Medium | Impact of rounding on interest calulation => potential exploit of integer division to decrease the interest value.| |
| 6 |Informative| Missing check on user collateral cap during liquidation => custom business logic mistake| |
| 7 |Critical| Incomplete checks for transaction group validity => Algorand transactions can be grouped (atomic), and each transactions of a group can potentially target different application Id. A Dapp can be composed of different application Id, some of them that should not be called by end users directly. More generally, all transaction fields should be checked before executing each transaction.| |
| 8 |Medium| Incorrect check for non-zero denominator in issued borrow shares => custom borrow logic might produce zero denominator in certain cases due to integer rounding logic.| |
| 9 |Informative| Possible acelerated growth of bank asset exchange ratio => custom busines logic mistake potentially exploitable by hackers| |
| 10 |Informative| Possible deflaction of borrow share value => custom busines logic mistake potentially exploitable by hackers with transactions minting or burning 0 amounts| |
| 11 |Informative| Extraneous market supply cap check while adding collateral => custom busines logic mistake| |

## Algofi DEX + Nanoswap Assessment ([runtime verification](https://github.com/runtimeverification/publications/blob/main/reports/smart-contracts/Algofi-dex-nanoswap.pdf))

| No. | Category |  Description | RuleID
| ------ | ------ | ------ | ------ |
| 1 |  | Possible inflation of LP-token value (Severity: Medium, Difficulty: Medium) => Generic business/security issue in CPMM, basically each CPMM should be seeded with some number of tokens and these tokens should be locked in the contract.| |
| 2 | | Possible off-by-one calculation in price statistics (Informative) => integer overflow | |
| 3 | | Recommended redundancy in checking OnCompletion fields (Informative) =>  Transaction contains an OnCompletion field that defines what action should occur with the transaction, this field should be checked in all cases and especially for admin | |
| 4 | | Imperfect invarian for allowed asset ratio (Informative) => Generic business/security issue in CPMM, corner cases around burn operations can break the invariant logic | |
| 5 | | Unfavorable rounding in computation of stableswap invariant (Informative) =>  similar to previous | |


# wAlgo Assessment ( [Trail of Bits](https://github.com/trailofbits/publications/blob/master/reviews/wALGO.pdf) )

## General

| No. | Category |  Description |
| ------ | ------ | ------ |
| 1 | OnCompletion | the user transactions are of NoOp type if they are not an Optin or a CloseOut. This will prevent an attacker from updating or deleting the applications. |
| 2 | OnCompletion | OnCompletion in the first transaction of the group is either NoOp or CloseOut. This will prevent an attacker from bypassing all the code’s logic by executing the clear state program. |
| 3 | RekeyTo | there is no rekeyto operation in the transaction. This will prevent an attacker from rekeying the vault. |
| 4 | mint | Prevent the minter from self-transferring during the burning operation. This will prevent attackers from burning without paying back the wAlgo.  |
| 5 | bytecode | Set the bytecode during the creation of the app. Alternatively Use a template to generate the base64 of the vault bytecode in app-vault or check if it matches the latest version. This will ensure the app-vault is up to date with the vault code. |
| 6 | Fee | that the transaction fee in the minter stateless contract is equal to MinTxnFee. This will ensure that all the transactions from the minter account have a limited fee.  |
| 7 | burn | Ensure the system locks enough Algo with respect to the wAlgo minted during the burn operation.|





## Stateless

| No. | Category |  Description |
| ------ | ------ | ------ |
| 1 | Fee | Check the transaction fee is below a threshold or equal to MinTxnFee. |
| 2 | RekeyTo | Check that RekeyTo operation is disabled unless explicitly approved.|
| 3 | CloseReminderTo | Check that CloseRemainderTo is disabled unless explicitly approved.| 
| 4 | AssetCloseTo | Check that AssetCloseTo is disabled unless explicitly approved. |
| 5 | Tx type | Disable all the transactions types unless explicitly approved. |


## Stateful

| No. | Category |  Description |
| ------ | ------ | ------ |
| 1 | Path | Ensure that all the paths check for ApplicationUpdate/ApplicationDelete. |
|   |      |  | 


## Transaction group

| No. | Category |  Description | RuleID |
| ------ | ------ | ------ | ------ |
| 1 | GroupSize  | Check that the GroupSize is equal to the expected number of transactions. | |
| 2 | GroupIndex | Check that the GroupIndex is equal to the expected index. |  |
| 3 | Sender | Check the sender of each external transaction. | |
| 4 | AppArgs | Consider checking the ApplicationArgs of stateful transactions in stateless transactions | algorand-application-args |



# HumbleSwap Assessment

### REACH
- Uses verifyArthimatic to prevent int overflow.
- Uses declassification to make the local information public.
- Uses Assume feature for local steps correctly.
- Uses Assert and Require functions correctly when making checks.
- Uses RequireIfVerify properly.
- Uses .only feature when needed.
- Uses Commit() and exit() correctly.
- Uses MulDiv when handling multiplication and division.

### TEAL
- Sets Pragma version
- Handles assertions correctly
- The general flow of application states are correct
- Checks for int overflows
- Handles group transactions correctly
- Checks transactions in some fields (AssetAmount, AssetReceiver, etc.)
- Checks AssetAmount
- Uses Type and TypeEnum to check if something is a payment or asset
- Multiplies integers before division, reducing the chance of over-frequent 0 value




