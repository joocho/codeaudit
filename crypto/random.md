Never use directly random from random source. 

The seed used to initialize the DRG must have entropy of at least 128.

A RNG should be refreshed periodically.

## Choose a number in [0, q-1] randomly

Assume q is l bits long.

The guides of ANSSI recommend two methods to generate random numbers.

### Reject method

The reject method consists to ask for l bits, and if number made from the bits asked is in [0, q-1], we take it, else we retry. 

q is random if the bits asked are random.

### Low bias method

This method consists to ask for l + 64 bits and compute modulo q, so the bias exists but is negligeable.


### Method to avoid

If we asks only l bits and do a mod q, there will be a bias, so q will not be random. 

If there is a sufficent bias, by repeating requests, it's possible to guess one number supposed random. It's a big problem for example with ECDSA: by asking many signatures, it's possible to retrieve the nonce and the private key.