# List (unexhaustive) of crypto algorithms to avoid in cryptographic situation

*This list is unexhaustive, and obviously not using these algos is not sufficent*

## Dual EC DRBG
It's a PRNG algorithm, but broken if one specific ECDLP instance is solved (seems to be a backdoor of NSA to break easily others algorithms). 

Reference: https://blog.cryptographyengineering.com/2013/09/18/the-many-flaws-of-dualecdrbg/

## LCG

LCG (linear congruential generator) must not be used as PRNG:
- a leak of bits of LSB permits to leak the LSB of future outputs
- if one output leaks, all future outputs leak

Note that rand() is a specific LCG.

LCG is useful as RNG, but not for cryptography.

TODO: add others? DES? MD5? SHA1?