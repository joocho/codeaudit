# Check during signature verification
For an ECDSA signature $`(r, s)`$, verify that $`r`$ and $`s`$ are integers in $`[ 1 , q − 1 ]`$. If not, the signature must be considered invalid. Some language like Go return 0 if the modular inverse does not exists thus it may considered valid the signature $`(0,0)`$ for any message and any public key. It was for example reported as (Psychic Signatures)[https://neilmadden.blog/2022/04/19/psychic-signatures-in-java/] in Java.
You should also verify that $`Q_A`$ is on curve, is not the point at infinity, and $`n*Q_A = 0`$

# Check the ephemeral key used during signature cannot be reused
During the signature computation, a random point $`R=kP`$ is computed where $`k`$ is an ephemeral key.  $`k`$ is also used to compute the  $`s`$ part of the signature ( $`s=k^{-1} * (h+rd) mod q`$). By ephemeral is meant that it must be used only once for a signature. If $`k`$ can be reused (without needing to be known by the attacker), the attacker can sign two different messages and have an equation system with two equations and only two unknowns. It leads to secret key recovery.


# Lattices based attacks

## Check the length of the ephemeral key cannot be guessed
If an attacker can know the length of $`k`$, he can mount a lattice based attack with some signatures and very few information about each ephemeral key. For instance, on an ECC with 256 bits in GF(p), if the attacker can identify some signatures with 5 msb equal to 0, he needs only 66 signatures. [Estimating the Effectiveness of Lattice Attacks, https://eprint.iacr.org/2021/1489.pdf]

## Verify that nonce is randomly chosen
The entropy should be of the length of the nonce. For example, if the nonce of 256 bits is composed of 4 blocks of 64 bits each depending on the last block, lattice attacks can be mount.

## Known bits but in different chunks
If some bits of nonce are known but are not contiguous,  you should look at the EHNP problem: "Extended hidden number problem and its cryptanalytic applications", 2006, Hlavac, Rosa (can be applied on EC Schnorr, ECDSA)