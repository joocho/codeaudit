# Unicity of nonce and ephemeral keys
In many algorithms or cryptographic mechanisms, some *nonce* may be used. This *nonce* is supposed to be used only once per *session* (or call to the algorithm). If the same *nonce* can be at least twice for the same algorithm or mechanism, it can create a weakness which may be result in an exploit.

The same rule apply to *ephemeral keys* which must remain *ephemeral* of course.

* a typical example is the ephemeral key *k* in ECDSA. If the same *k* is used twice to sign two messages, the knowledge of the two signatures results in the knowledge of the secret private key *d*. This vulnerability has been exploited on the PlayStation.

# Nonce and ephemeral keys must be random
Due to previous rule, the nonces and ephemeral keys must be randomly chosen. The randomization algorithm must be uniform for every single bit.

In some cases, the knowledge of only some bits can be considered as a critical vulnerability. Again, ECDSA suffers from the knowledge of only 3 bits of *k* (with a 256 bits elliptic curve) : a lattice attack can be mounted with some known signatures to reveal the secret private key *d*.
