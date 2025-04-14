## PBKDF2

PBKDF2 should have a big number of iterations (the goal of iterations is to slow down the computations, and limit bruteforce attacks)

Salt with 128 bits should be used to avoid precomputation tables.

Good PRF (pseudo random function) should be used.