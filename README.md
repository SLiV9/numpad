# Numpad

A dynamically typed expression language that can be programmed using just one hand on the numpad.


## Syntax
### Unary Operators
```
*     fetch
+     signum / copy list
-     negate / get length of list
/     reciprocal

+.    Ceiling
-.    Floor
*.    Print unicode-scalar values
```

## Binary Operators
```
+ plus
* multiply
- assign
/ call with
```

### literals
There are no negative literals
```
0   integer
0.0 floating point/fixed point
```

### Evaluation Order
Evaluation of expressions is right to left,
```
-2    (-2)
-2+6  (-8)
```

### Division and subtraction
Make use of combining unary and binary operators to
do division and subtraction
```
100+-5 (100-5)
100*/5 (100/5)
```


Evaluation of statements is left to right, top to bottom
```
(EntryPoint..H..E..L..L..O..LineFeed)
1..*.72..*.69..*.76..*.76..*.79..*.10

1       (EntryPoint)
..*.72  (H)
..*.69  (E)
..*.76  (L)
..*.76  (L)
..*.79  (O)
..*.10  (LineFeed)
```

### Nested expression
```
/.0./         (with no separator this is like parenthesis)
/.-2./ * 6      (-2 * 6)
```

### Lists
Make a list by using the `..` separator
(note this means you cannot nest statements inside of expressions syntacticaly, but you can semantically using `*`)
```
/../         (empty list)
/.0.../      (single, trailing .. separator okay)
/.0....2./   (you can even have multiple separators)
             (this has legth 2)
/.0..1+1./   (two element array)
```

Lists are lazy, you may need to evaluate them eagerly with `*`
```
| 1../.75+32.../
| 
Output: list [Plus((75) (32)), ]
| 1..*/.75+32.../
| 
Output: (107)
```

Get the length of a list with unary `-`

in the repl
```
| 1..-/.1..2..3./
| 
Output: (3)

```

### Declaration
If a line starts with a number, it is a fetchable
```
100 .. 5    (*100 => 5)
200 .. *100 (*200 => *100 => 5)
```
Fetching a list actually gets a pointer, but pointer behave basically the same a list

In the repl
```
| 2../.10..20..30./
| 3..*2
| 1..-*3
| 
Output: (3)
```

### Entry point
Entry point is address `1`
```
(leading zeros is okay)
01 .. *10 (*10 here could be any expression)
10 .. 20
(in quick testing in the repl it can be convienient to just use 1)
1..4+5 (this will immediatly output 9 in the repl)
```

### Variables
assign to a variable with -
```
10 .. 100 - 5 (using *10 will assign 5 to address 100)
```

### Comments in source code
```
(comments use paired parenthesis, and should only be used in source code)
(they must be on the same line)
```

## Run source code
By convention the extension is `.num`
```
numpad source.num
```

## Start a repl 

running numpad with no arguments starts the repl
```
numpad
```
A '| ' prompt will appear
```
numpad
| 
```

Press `Enter` twice to evaluate from the entry point
```
numpad
| 1..*2
| 2..5
| 
Output: (5)
| 
```
End repl session by typing 4 minus characters
```
numpad
| ----
````

The first time you run `numpad`, it will create a history file.

To run the repl after passing in a source file, use the --repl flag
```
numpad hello.num --repl
```
Get large amounts of debug information us the --verbose flag
```
numpad --verbose
| 1..*2
| 2..5
| 
TRACE - "1"	| Number 
TRACE - ".."	| Separator 
TRACE - "*"	| Star 
TRACE - "2"	| Number 
TRACE - ""	| Enter 
TRACE - "2"	| Number 
TRACE - ".."	| Separator 
TRACE - "5"	| Number 
TRACE - ""	| Enter 
TRACE - 
TRACE - Label : 
TRACE - 	Int(1)
TRACE - 	Sep
TRACE - 	Unary(Fetch)
TRACE - 	Int(2)
TRACE - Label : 
TRACE - 	Int(2)
TRACE - 	Sep
TRACE - 	Int(5)
TRACE - 
TRACE - 1:	Fetch((2))
TRACE - 2:	(5)
TRACE - 
TRACE - Evaluating 1: Fetch((2))
TRACE - 
TRACE - Eval :: Fetch((2))
TRACE - Access register 2: (5)
Output: (5)
|
```
`--verbose` and `--repl` do work together
```
numpad hello.num --repl --verbose
```
You might be interested specific debug information.

Use `--log-module=numpad::<module>`
 with `--verbose`

where module is one of the projects modules.

currently
  - lexer
  - parser
  - machine

For example with the lexer
```
numpad --verbose --log-module=numpad::lexer
| 1..*2
| 2..5
| 
TRACE - "1"	| Number 
TRACE - ".."	| Separator 
TRACE - "*"	| Star 
TRACE - "2"	| Number 
TRACE - ""	| Enter 
TRACE - "2"	| Number 
TRACE - ".."	| Separator 
TRACE - "5"	| Number 
TRACE - ""	| Enter 
TRACE - 
TRACE - Label : 
TRACE - 	Int(1)
TRACE - 	Sep
TRACE - 	Unary(Fetch)
TRACE - 	Int(2)
TRACE - Label : 
TRACE - 	Int(2)
TRACE - 	Sep
TRACE - 	Int(5)
Output: (5)
| 

```

