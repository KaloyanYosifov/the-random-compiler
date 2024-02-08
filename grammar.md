```
P -> S
S -> A S'
   | V(E) S'
   | Q S' 
   | F S'
   | D S'
S' -> S 
   | ε
Q -> K(E){S}
A -> KV=E;
F -> for(A E;E){S} 
D -> fn V(KV){S}
R -> K V R' 
   | ε 
R' -> , R
   | ε 
E -> IE'
   | I++
   | I--
E' -> +E
   | *E
   | /E
   | -E
   | ==E 
   | >=E 
   | <=E 
   | !=E 
   | >E 
   | <E 
   | &&E
   | ||E
   | (E)
   | ε
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
   | else
   | continue
   | string
   | bool
   | float
```

## Parser with PDA (Push down automata)

Where keyword = int, char, if, elif, continue, else, string, bool, float 
Where digit = 0-9 

First(P) = keyword, id 
First(S) = keyword, id
First(Q) = keyword
First(A) = keyword
First(F) = for
First(D) = fn
First(E) = id, digit, true, false, ", (
First(V) = id
First(L) = "
First(B) = true, false
First(I) = id, digit, true, false, "
First(N) = digit
First(K) = keyword

Follow(P) = $
Follow(S) = $, }, keyword, id, fn, for
Follow(Q) = $, }, keyword, id, fn, for
Follow(A) = $, }, keyword, id, fn, for
Follow(F) = $, }, keyword, id, fn, for
Follow(D) = $, }, keyword, id, fn, for
Follow(E) = ), ;
Follow(V) = =, (, ), +, *, /, -, ==, >=, <=, !=, >, <, &&, ||, ++, --
Follow(L) = +, *, /, -, ==, >=, <=, !=, >, <, &&, ||, ++, --
Follow(B) = +, *, /, -, ==, >=, <=, !=, >, <, &&, ||, ++, --
Follow(I) = +, *, /, -, ==, >=, <=, !=, >, <, &&, ||, ++, --
Follow(N) = +, *, /, -, ==, >=, <=, !=, >, <, &&, ||, ++, --
Follow(K) = (, id
