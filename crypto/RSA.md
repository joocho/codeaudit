# RSA

## e must not be too small
In the case e is too small, it's possible to find d in the convergents of the continued fraction e/N.

## d must not be small
Coppersmith

## p and q must not be too close
Fermat factorization method

... TODO

Reference: https://crypto.stanford.edu/~dabo/pubs/papers/RSA-survey.pdf

## Padding

Padding must be random, and comes from a recommended RNG.

RSA-PKCS#1v1.5 can lead to padding oracle.
Bad implementation of OEAP can lead to padding oracle.

Reference: look for Bleichenbacher attacks.

## Primality test
Miller Rabin test is recommended, the number of iterations depends on the length of the modulus. This is a probabilistic test, so we are not sure, but by doing it multiple times, the probability to have a false positive is negligeable.

## Message to multiple persons

The same message should not be sent to multiple persons because of CRT

## RSA PKCS1 v1.5 deprecated.

[The Marvin Attack: potential key recovery through timing sidechannels](https://rustsec.org/advisories/RUSTSEC-2023-0071)
