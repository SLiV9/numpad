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
(note this means you cannot nest statements inside of expressions)
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
Fetching a list actually gets a pointer, but pointers behave basically the same as a list

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
1..4+5 (this will immediatly output 9 in the repl)
```
When using the repl, it is reccomended to have address `1` call your main expression

This will let you use address `1` to select how to start your evaluation
```
| 2..98*/4
| 3..97*/4
| 1..*2
| 
Output: (24.5)
| 1..*3
| 
Output: (24.25)
| 
   
```

### Fetch and Call
use unary `*` to evaluate an expression at an address
```
| 1..*2  (fetch address 2)
| 2..5+7
|
Output: (12)
```
Use unary `*`on a list to get the first element
```
1..*/.3..4./ (3)

(for other elements increment the pointer)

1..*1+/.3..4./ (4)
```
Binary `/`  does a call
```
| 1..2/5 (call 2 with 5)
| 2..*2  (2 gets it's argument which will happen to be 5)
| 
Output: (5)
```

### Tangent

The following should treated as undefined behavior, but will describe the current implementation.
Be careful when mixing fetch `*`, `\` call, and assign `-`
Using fetch on a address before it is called will run it, but self referencial fetch will return undefined
```
1..*2
2..*2 (here *2 returns undefined)
```
After Call `\` is run, the functon obtains state
```
2..1+*2    (this is 6)
1..2/5..*2 (this will be 5 because we query the state of 2 after call)
```
Assign `-` will completely erase the code of the adress with a value
```
2..*2
1..2-20..2/5 (this is 20, it will ignore the argument)
```
### End of Tangent

Naturally calls can take lists
```
| 1..2//.100..200./
| 2..*1+*2
| 
Output: (200)
```
Arguments using lazy lists can make control statements
```
| 3..*.72..*.10   (this would print 'H' and Linefeed)
| 4..*.69..*.10   (  "       "      'E' and Linefeed)
| 1..2//.*3..*4./ (if the list was eager, both 'H' and 'E' would appear)
| 2..**2
| 
H
Output: (10)
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

### Input
Although you can use the whole keyboard.. what fun is that?
Remember to use `NumLock` to toggle between characters and actions.

Characters
  - Numbers : `0 1 2 3 4 5 6 7 8 9`
  - Dot : `.`
  - Operators : `+ - * /`
Actions
  - Cursor movement : `← → Home End`
  - History : `↑ ↓`
  - Editing : `Del`
  - Input   : `Enter`
  - Unimplemented/Terminal dependant : `Insert PgDn PgUp`


Press `Enter` twice to evaluate from the entry point
```
numpad
| 1..*2
| 2..5
| 
Output: (5)
| 
```
End repl session by typing 4 `-` characters at the start of the line
```
numpad
| ----
````

The first time you run `numpad`, it will create a history file.
The history is stored in the location you called numpad from in a `history.txt` file

You can use the arrow keys to browse your repl input history while the repl is running.


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

