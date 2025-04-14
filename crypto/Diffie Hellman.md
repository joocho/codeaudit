# Authenticate the identities before
Diffie Hellman (DH) allows to create a private secure channel between two persons Alice and Bob. It doesn't authenticate the identities. So Alice and Bob must check (with another algorithm) that the other person is really who she/he pretends to be. If it's not done, it's possible to do man in the middle!

There are different types of DH: DH, ECDH, ECDHE, ...
We can use elliptic curves (EC) or not (work in finite field, so mod p), use ephemeral/static keys from one side/the two sides, ...

In ECDH, the things to verify on what we receive:
- the point Q is not the infinity point
- the point Q has coordinates in the good ranges ([1, p-1])
- the point Q is on curve
- nQ must be equal to 0

Reference: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Ar3.pdf , part 5.6.2.3

With DH, if the order of the generator is not prime, you can leak bits of the secret key by giving chosen points (see [elliptic_curve.md](elliptic_curve.md) for that).