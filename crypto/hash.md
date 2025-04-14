# Length extension attack
Algorithms MD5, SHA1 and SHA2 are vulnerable to [Length extension attack](https://en.wikipedia.org/wiki/Length_extension_attack). A hash with an attacker controlled value can be forged from a previous hash of an unknown value. Truncated versions of SHA2, SHA3 and BLAKE2 algorithms are immune to this attack.

# Domain Separation
When hashing objects of mixed types, a proper [domain separation](https://crypto.junod.info/posts/recursive-hash/)  and serialization must be implemented.

# MD5
MD5 must not be used, it's possible to find collisions (inputs with the same hash)

# SHA1
It's possible to generate collisions with SHA1, so it's also a hash function to avoid.
