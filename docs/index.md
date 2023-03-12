---
layout: default
---

# Numpad

A dynamically typed expression language that can be programmed using just one hand on the numpad.
Created by Remy Pierre Bushnell Clarke and Sander in 't Veld.

[![Demo on YouTube](http://img.youtube.com/vi/0ftXhx-7Ffs/0.jpg)](http://www.youtube.com/watch?v=0ftXhx-7Ffs "Numpad demo: printing A-Z")

[Watch Sander demo the language using the REPL.](http://www.youtube.com/watch?v=0ftXhx-7Ffs "Numpad demo: printing A-Z")

## Hello world

The following program prints a message and then calculates 1+1:

```
1
..*.72..*.101..*.108..*.108..*.111..*.32
..*.119..*.111..*.114..*.108..*.100..*.33
..*.10
..1+1
```

It should output the following:

```
Hello world!
Output: (2)
```

Easy, right? Next we'll try sorting a list with 5 elements.

```
1 .. 100 / /.*2..*3./
2 .. 5
3 .. /.20..40..11..1..16./

100
.. 101 - **100
.. 102 - +*/.*100./ + 1
.. 103 - 0
.. 104 - 0
.. 105 - 0
.. *106

106 .. **/.102..107./ + +/.*101./ +- *103
107 .. **/.111..108./ + +/.*101./ +- *104
108 .. **/.109..110..110./ + 1 + +/.*/.*102./+*103./ +- */.*102./+*104
109
.. 105 - */.*102./+*103
.. /./.*102./+*103./ - */.*102./+*104
.. /./.*102./+*104./ - *105
.. *110
110
.. 104 - 1+*104
.. *107
111
.. 103 - 1+*103
.. 104 - 0
.. *106
```

Uhm, ok, maybe not *that* easy. But sure enough, this outputs:

```
Output: list [(1), (11), (16), (20), (40), ]
```

## Syntax

Although not part of the language proper because they cannot be typed on a numpad, comments can be useful to clarify things.

```
(Comments use paired parentheses on the same line.)
(They should only be used to annotate source code.)
```

We will use them below to explain the syntax.

### Literals

Numpad is a number-based language.

```
0     (Integer)
0.0   (Floating point)
```

There are no negative literals, but there are unary operators.

### Unary Operators

Because the numpad only has five symbols on it, the number of operators is limited. We repurpose the four mathematical operators (`+-*/`) and use a dot to create three additional operators.

```
*     (Fetch)
+     (Signum / copy list)
-     (Negate / get length of list)
/     (Reciprocal)

+.    (Ceiling)
-.    (Floor)
*.    (Print unicode-scalar values)
```

The dot (`.`) is not an operator and always appears next to something else.

### Binary Operators

Each of the main unary operators doubles as a binary operator.

```
+     (Addition / skip list elements)
*     (Multiplication)
-     (Assign to address / assign to list element)
/     (Call function with argument)
```

### Evaluation Order

There is no operator precedence. Evaluation of expressions happens right to left.

```
-2    (-2)
-2+6  (-8)
```

### Division and subtraction

You can combine unary and binary operators to do division and subtraction:

```
100+-5 (95)
100*/5 (20)
```

### Nested expression

Expressions can be wrapped with brackets, which are written `/.` and `./`.

```
/.0./      (0)
/.-2./*6   (-12)
```

### Instructions

All code consists of a list of instructions. Each instruction is a number (its address) followed by a separator `..` and then an expression.

```
100..500
200..1+1
300..5.3
```

### Entry point

The entry point of a Numpad program is address **1**. When using the [REPL](#start-a-repl), the expression at address **1** is evaluated every time you submit code.

```
| 1..4+5
|
Output: (9)
```

### Fetching

Expressions at other addresses are fetchable with unary `*`. You can think of this as referencing a variable. Alternatively you can think of each number as a pointer.

```
| 1..*2
| 2..5
|
Output: (5)
```

Expressions are evaluated as soon as their are fetched.

```
| 1..*2
| 2..*3
| 3..*4
| 4..100
|
Output: (100)
| 1..*2
| 2..23+27
|
Output: (50)
```

Note that this language is Turing complete, thus there is no protection against infinite loops.

```
(execute at your own peril)
1 .. *2
2 .. *1
```

### Lists

In addition to numbers, Numpad also supports lists.
A list consists of zero or more expressions separated by `..`, surrounded by brackets.

```
/../         (empty list)
/.0..1+1./   (list with two elements)
```

For a list of length 1, use an explicit `..` to avoid creating a nested expression.

```
/.0.../      (list of length 1 with a trailing .. separator)
             (you can even have multiple separators between values)
/.0....2./   (list of length 2)
```

You can get the length of a list with unary `-`. In the REPL:

```
| 1..-/.10..20..30./
|
Output: (3)
```

### Lazy evaluation

Lists are lazy. Consider the following REPL example:

```
| 1../.75+32.../
|
Output: list [Plus((75) (32)), ]
```

Although the expression stored at address **1** is evaluated, it is a list containing another expression, which is not evaluated.
List elements are only evaluated when you retrieve them from the head of the list using unary `*`.

```
| 1..*/.75+32.../
|
Output: (107)
```

To retrieve other elements, use binary `+` to skip elements first.

```
| 1..**2                             (fetch the list, then the head)
| 2..3+/.0..10..20..30..40..50./     (this becomes /.30..40..50./)
|
Output: (30)
```

In addition to being lazy, lists are passed by reference when fetched.
The following sample does not copy any lists:

```
| 1..-*2
| 2..*3
| 3../.10..20..30..40..50./
|
Output: (5)
```

### Statements

An instruction may contain multiple "statements", separated by `..`, that are evaluated before its expression.
The main use for this is to assign values to addresses using binary `-`:

```
| 1 .. 100 - 5 .. *100
|
Output: (5)
| 1 .. *100
|
Output: (5)
```


You can think of this as writing to "variables", but essentially you are writing directly to memory.
Note that assigning a value to an address already containing an expression will overwrite it.

```
| 1 .. 2 - 5 .. *2
| 2 .. 485+293
|
Output: (5)
```

Lists are fetched by reference and can be mutated by assigning to them.

```
| 1 .. /./.*2./+1./ - 55 .. *2
| 2 .. /.10..20..30./
|
Output: list [(10), (55), (30), ]
```

It is also possible to append to a list by assigning to the *one past the end* position.

```
| 1 .. /./.*2./+3./ - 55 .. *2
| 2 .. /.10..20..30./
|
Output: list [(10), (20), (30), (55), ]
```

Another useful statement is printing a Unicode character using unary `*.`:

```
| 1 .. *. 72 .. *2      (print ascii character 72: 'H')
| 2 .. *. 10 .. 5       (print ascii character 10: line feed)
|
H
Output: (5)
```

Unlike expressions, evaluation of statements is left to right.

```
(EntryPoint..H..E..L..L..O..LineFeed)
1..*.72..*.69..*.76..*.76..*.79..*.10
```

Newlines and other whitespace may be added anywhere in the source code.
Instructions spread out over multiple lines are evaluated top to bottom.

```
1       (EntryPoint)
..*.72  (H)
..*.69  (E)
..*.76  (L)
..*.76  (L)
..*.79  (O)
..*.10  (LineFeed)
```

### Algorithms

Lazy evaluation allows you to do conditional computation:

```
| 1 .. */.*2..*3./ + 1   (the list contains two expressions)
|                        (2 is undefined, but never fetched)
| 3 .. 100
|
Output: (100)
```

By combining this trick with the careful placement of values at certain addresses, you can implement any algorithm you want. Here's one that prints the alphabet:

```
1
.. 2 - 0               (set a "variable", let's call it "i", to 0)
.. *3                  (continue by evaluating 3, the main loop)
3
.. 4 - 26 +- *2        (calculate 26 - i and store it in "x")
.. */.*9..*5./ + +*4   (evaluate either 9 or 5, depending on x)
5
.. *. 65 + *2          (print an ascii character from 'A' to 'Z')
.. 2 - 1+*2            (i = i + 1)
.. *3                  (loop back to 3)
9
.. *. 10               (print a newline)
.. /../                (we are done, return an empty list)
```

The example above doesn't declare the instructions at addresses 2 and 4, but instead uses the binary `-` operator to assign to them directly.
By convention, algorithms are stored at multiples of ten so that they can use the following addresses (11, 12, 13, etcetera for the algorithm stored at 10) to store temporary values.

In fact, you could use this convention to supply "arguments" to an algorithm.

```
1
.. 31 - 4
.. 32 - 5
.. *30

30 .. /.*31./ + *32      (this algorithm calculates a sum)
```

### Functions

To make life a little easier, you can instead declare "functions". In Numpad, a function is an expression that tries to retrieve its own argument(s) by fetching *itself*. Behold!

```
9000 .. 50 + *9000
```

In order to evaluate this strange expression, it must be called using binary `/`:

```
| 1 .. 9000/3
| 9000 .. 50 + *9000
|
Output: (53)
```

Powerful as this magic may be, it has its limits. Before using a function argument in further computations, it is best to store it in a local variable.

```
9000
.. 9001 - *9000
.. 9002 - 50
.. /.*9001./ + *9002
```

You can even pass multiple arguments by using a list.

```
| 1 .. 30//.4..5./
| 30
| .. 31 - **30       (get the first argument and store it)
| .. 32 - *1+*30     (get the second argument and store it)
| .. /.*31./ + *32   (calculate the sum)
Output: (9)
```

Using the function call operator on a normal expression is fine but useless. It does the same thing as a fetch.

```
| 1 .. 2/123132123
| 2 .. 3+4
|
Output: (7)
```

However beware that trying to fetch a function is undefined behavior.

```
1 .. *9000
9000 .. 50 + *9000
```

This may result in an infinite loop, return `undefined`, cause nasal demons to erupt or expose something about the weird internals of our interpreter. Ahem.

```
| 1 .. *2
| 2 .. *2
|
Output: undefined
| 2 .. 1+*2
| 1 .. 2/5 .. *2
|
Output: (5)
```

Don't worry about it.

## Run source code

By convention the extension for Numpad programs is `.num`.

```
numpad examples/hello.num
```

## Start a REPL

Running numpad with no arguments starts the REPL. A prompt will appear, starting with `| `, where you can type in your code:

```
numpad
|
```

### Input

Although you can use both hands and an entire keyboard to program... what fun is that?

Using only the numpad, the following keys are available to you:

  - Numbers : `0 1 2 3 4 5 6 7 8 9`
  - Dot : `.`
  - Operators : `+ - * /`
  - Cursor movement : `← → Home End`
  - History : `↑ ↓`
  - Editing : `Del`
  - Input   : `Enter`
  - Terminal dependent : `Insert PgDn PgUp`

Remember to use `NumLock` to toggle between characters and actions.

Press `Enter` twice to evaluate from the entry point:

```
numpad
| 1..*2
| 2..5
|
Output: (5)
|
```

End the REPL session by typing 4 dashes at the start of the line:

```
numpad
| ----
````

### Advanced REPL usage

When using the REPL, it is recommended to define your main expression at a different address, and have instruction **1** call your main expression.
This will let you use address **1** to select how to start your evaluation on the fly:

```
| 2..98*/4
| 3..97*/4
| 1..*2
|
Output: (24.5)
| 1..*3
|
Output: (24.25)
| 4..96*/4
| 1..*4
|
Output: (24)
```

The first time you run the REPL, it will create a `history.txt` file in the current directory.
You can use the arrow keys to browse your REPL input history while the REPL is running.

### Command line interface

To run the REPL after passing in a source file, use the `--repl` flag:

```
numpad hello.num --repl
```

To get large amounts of debug information, use the `--verbose` flag:

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

You can combine the `--verbose` and `--repl` flags:

```
numpad hello.num --repl --verbose
```

You can fine-tune the level of verbosity using `--log-level=<module>`
in combination with `--verbose`,
where `<module>` is one of:
  - error
  - warn
  - info
  - debug
  - trace

You might also be interested in debug information for a specific module.
Use `--log-module=numpad::<module>`
(in combination with `--verbose`)
where `<module>` is one of the project modules:
  - lexer
  - parser
  - machine

For example, to get only lexer output:

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
