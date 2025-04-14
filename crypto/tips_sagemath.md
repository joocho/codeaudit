# Tips in sagemath

For linux (or in WSL):
```py
%colors Linux
```
(default blue on black -> just unreadable)

Factor a big number:
```py
N=big number
factor(N)
```
Use sagemath in python:

```py
from sage.all import *
```

To convert the x coordinate of a point on curve to python integer:
```py
int(str(point[0].lift()))
```

To define a curve in sagemath: go to https://neuromancer.sk and search for the curve

To approximate a fraction:
```py
(1/3).n()
```

<!-- 
```py
```
```py
```
```py
``` -->

