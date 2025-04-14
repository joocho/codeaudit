A padding oracle is when it's possible to know if the padding is correct or not during decryption of a modified encrypted message. 

Real problem: before decryption, authentification and integrity should be checked.

That's why MAC-then-Encrypt and Encrypt-and-MAC are obsolecents.
Encrypt then MAC is recommended.

## Example of attack
Suppose that AES-CBC is used and we don't verify authentication. Suppose that padding is PKCS7, so the format

01

0202

030303

...

00000000000000000000000000000000


An attacker can change the last byte of second last encrypted, ask to decrypt each ciphertext to know when there is a padding error. There are multiple possibilities:
- 254 padding error, 2 OK: should be 01 and another one due to the rest of padding
- 255 padding error, 1 OK: the byte targeted is probably 01
...

Then you've found one byte, you can iterate on the rest of the block to retrieve the decrypted block without knowing the key. 

When you retrieved the cleartext of the block, you can put it away and continue on others blocks.

Implementation: https://github.com/mpgn/Padding-oracle-attack (TODO: not tested, seems to be well-known)