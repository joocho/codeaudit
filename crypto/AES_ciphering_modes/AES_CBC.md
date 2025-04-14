![Mode CBC](images_mode/mode_cbc.svg)

## The IV of CBC ciphering mode must not be predictable
Not predictable includes not fix and not the last ciphered block !
The reason is explained hereafter.

If the attacker can choose the plaintext and knows two previous ciphered blocks $`C_i`$ and $`C_{i-1}`$, he can make a guess on the plaintext $`P_i`$ (if the entropy of $`P_i`$ is low). So he guesses $`P_i=X`$. 

Then, as he also knows the next $`IV`$ (as it is predictable), he can ask for the AES_CBC encryption of its own plaintext $`MyP = X \oplus C_{i-1} \oplus IV`$ 


The output will be only one block equal to

```math
MyC = AES(MyP \oplus IV)
```
```math
MyC = AES(X \oplus C_{i-1} \oplus IV \oplus IV)
```
```math
MyC = AES(X \oplus C_{i-1})
```
```math
MyC = C_i
```
So if $`MyC = C_i`$, it means that $`P_i=X`$ is a good guess and the attacker succeeded to retrieve the plaintext !
