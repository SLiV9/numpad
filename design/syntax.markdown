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

(The first time you run numpad, it will create a history file)

(to run the repl after passing in a source file, use the --repl flag)
numpad hello.num --repl

(to get large amounts of debug information us the --verbose flag)
numpad --verbose

(--verbose and --repl do work together)
numpad hello.num --repl --verbose

(End repl session by typing 4 minus characters)
----
````