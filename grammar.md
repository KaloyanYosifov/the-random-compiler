```
P -> S
S -> A
   | V(E)
   | Q 
Q -> K(E){S}
A -> KV=E
F -> for(A;E;E){S} 
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
   | I++
   | I--
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
