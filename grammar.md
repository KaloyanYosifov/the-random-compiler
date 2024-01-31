```
P -> S
S -> KV=E
   | V(E)
   | Q 
Q -> K(E){S}
E -> I+E
   | I*E
   | I/E
   | I-E
   | I==E 
   | I>=E 
   | I<=E 
   | I!=E 
   | I>E 
   | I<E 
   | (E)
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
