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
   | I&&E
   | I||E
   | (E)
   | (E)+E
   | (E)*E
   | (E)/E
   | (E)-E
   | (E)==E
   | (E)>=E
   | (E)<=E
   | (E)!=E
   | (E)>E
   | (E)<E
   | (E)&&E
   | (E)||E
   | I
V -> id(.id)*
L -> ".*"
B -> true
   | false
I -> V
   | N
   | L
   | B
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
