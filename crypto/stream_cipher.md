## ChaCha20

Ok to use, but (key, IV) should not be used twice (leak of XOR of cleartexts, so a lose of confidentiality).

Prefer to use stream ciphers based on block cipher, like AES_CFB, AES_OFB, AES_CTR, ...