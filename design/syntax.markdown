unary
operators
```
*     fetch
+     signum
-     negate
/     reciprocal

+.    Ceiling
-.    Floor
*.    Print unicode-scalar values
```

binary operators
```
+ plus
* multiply
- assign
/ call with
```

Division and subtraction
```
(pair up unary operator with binary operator)
100+-5 (100-5)
100*/5 (100/5)
```

literals
```
0   integer
0.0 floating point/fixed point
```

expression
```
/.0./         (parenthesis)

(C style arrays, ie: just a pointer to first element)
/.0.../      (single array, trailing .. separator okay)
/.0..1+1./   (two element array)
(lists are lazy, you may need to evaluate them eagerly with *)
*/.75+32.../
```

Declaration
```
(if a line starts with a number, it is a fetchable)
100 .. 5    (*100 => 5)
200 .. *100 (*200 => *100 => 5)
```
Entry point
```
(Entry point is address 1)
01 .. *10 (*10 here could be any expression)
10 .. 20
(in quick testing in the repl it can be convienient to just use 1)
1..4+5 (this will immediatly output 9 in the repl)
```

Variables
```
(assign to a variable with -)
100 - 5 (this declares 100 and assigns it)
```

Comments in source code
```
(comments use parenthesis, and should only be used in source code)
(they must be on the same line)
```

Start a repl 
```
(running numpad with no arguments starts the repl)
numpad
(We reccomend you install rlwrap to have input history)
rlwrap numpad

```

End Repl session
```
(type 4 minus characters)
----
````