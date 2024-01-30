```
P -> S
S -> KV=E
   | V(E)
   | Q 
Q -> K(C){S}
C -> E==C 
  -> E>=C
  -> E<=C
  -> E!=C
  -> E>C
  -> E<C
E -> I+E
   | I*E
   | I/E
   | I-E
   | (E)
   | C
   | I
V -> id(.id)*
I -> V
   | N
N -> digit+
   | digit+.digit+
K -> int
   | char
   | if
   | elif
   | for
   | continue
   | string
   | bool
   | float
```
