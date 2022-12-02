## About

This repository is dedicated to exploring the article by Amber Sprenkles [LLVM provides no side-channel resistance]

[LLVM provides no side-channel resistance]: https://electricdusk.com/cmov-conversion.html


## Findings
My preliminary findings dispute the notion that compiler optimizations change the runtime from constant to non-constant time. That is to say, using some simple time measurements, I'm able to show that the constant time lookup remains constant, while the non constant time varies. The non-constant time function has a significantly higher standard deviation and varies based on where in the array the lookup is performed.

```
constant time:
ellapsed - not found: 5.13µs
ellapsed - found: 2.754µs
ellapsed - found: 2.74µs
ellapsed - found: 2.65µs

non-constant time:
ellapsed - not found: 2.351µs
ellapsed - found: 552ns
ellapsed - found: 1.409µs
ellapsed - found: 2.025µs

standard deviation:

constant time
0.044969125210774
    
non-constant time
0.60402667343605
```
