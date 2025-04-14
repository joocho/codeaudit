MAC is Message Authentication Code

## Check there is no oracle of padding or of format
You should verify that there is no padding oracle. You can go to [padding_oracle.md](padding_oracle.md).

In particulary, in a case of a MAC and ciphered data, the MAC must be computed after the encryption of the data at the sender side (Encrypt-then-MAC mode). In this way, at the receiver side, the MAC can be verified before the deciphering process. If the MAC is wrong, it won't give any information about the clear text and it cannot be used as a padding or format oracle.

## Check if MAC is truncated that it's not too much truncated
See "Guide de sélection d'algorithmes cryptographiques" from ANSSI or alternative (BSI, ...)
If a MAC is too much truncated, the probability to find to generate a good MAC increases.

## CBC_MAC : check the IV is not variable and predictable
The IV must be constant for any message. If it is not the case, the attacker can forge another message with the knowledge of a previous message and of its MAC. See [Allowing the initialization vector to vary in value](https://en.wikipedia.org/wiki/CBC-MAC#Using_the_same_key_for_encryption_and_authentication)  and [Using predictable initialization vector sections](https://en.wikipedia.org/wiki/CBC-MAC#Using_predictable_initialization_vector)  in CBC MAC on Wikipedia

TODO : wikipedia says IV 0???

## CBC_MAC : the length of the messages MUST NOT be variable
If the length is variable, it is possible to compute MAC(M1||M2) if the attacker knows MAC(M1) and MAC(M2). See [Security with fixed and variable-length messages](https://en.wikipedia.org/wiki/CBC-MAC#Security_with_fixed_and_variable-length_messages) section of CBC MAC on Wikipedia


## CMAC
TODO : what is the tweak? https://en.wikipedia.org/wiki/One-key_MAC

## HMAC
HMAC-SHA1 is tolerated by ANSSI for now, but keysize >= 128 should be used (so other hash function should be used)

## GMAC