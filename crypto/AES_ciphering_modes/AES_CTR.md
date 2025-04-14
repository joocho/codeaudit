![Mode CTR](images_mode/mode_ctr.png)

## two different messages must never use the same pair (Key,IV)
If the same key and IV are used, the generated mask will be the same. So the xor of the plaintexts leaks. Depending on the plaintexts possibilities, it can reveal important information.

The attacker knows C1 and C2, so he can compute C1 xor C2 \
C1 xor C2 = (P1 xor mask1) xor (P2 xor mask2) \
as mask1 = mask2, \
C1 xor C2 = P1 xor P2
