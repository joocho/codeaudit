![Mode GCM](images_mode/mode_gcm.svg)

## MAC truncature
MAC shall not be truncated in case of AES in GCM mode. If it is the case, the authentication tag can be forged

See N. Ferguson. Authentication weaknesses in gcm. 
https://csrc.nist.gov/CSRC/media/Projects/Block-Cipher-Techniques/documents/BCM/Comments/CWC-GCM/Ferguson2.pdf, May 2005

## IV shall not be reused
In this case, the ciphering sequence is the same and the attacker can know the Xor of two plaintexts (equal to the Xor of the two ciphertexts).

Moreover, using the Xor of several pairs of authentication tags, the attacker can find the value of H which will be the root of many polynomials. To reduce the computations, the attacker can compute the GCD of the polynomials to get a polynomial of small degree with H as a root. In this case, with the knowledge of H, he can forge an authentication tag, which means the authentication will be lost .

See chapter 8 and Annex A of https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38d.pdf

For more details on how to find H value, see original paper of  A. Joux, Authentication Failures in NIST version of GCM https://csrc.nist.gov/csrc/media/projects/block-cipher-techniques/documents/bcm/joux_comments.pdf

## Clear length should not be too long

Clear length should not exceed $`2^{32} - 2`$ blocks, so $`2^{39} - 256`$ bits