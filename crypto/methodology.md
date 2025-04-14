# Documentation analysis
A document which describes the used cryptographic primitives and mechanisms must be available. It is the basis to make the theoretical cryptographic analysis. 

## Make the theoretical cryptographic analysis
Only this documentation must be analyzed to look for possible vulnerabilities. The vulnerabilities are dependent on the algorithms themselves. Look at the pages in [crypto directory](crypto/)

# Implementation analysis
## Check the conformity of the implementation
- Every cryptographic primitive (such as AES or SHA3) must be tested with known patterns and compared to reference implementation
- Every cryptographic mechanism (such as AES-CBC, AES-CTR, ECDSA, HMAC-SHA3...) must also be tested and compared to reference implementation

## Search for implementation vulnerabilities

