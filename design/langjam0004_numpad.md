Types of "things":
- Numeric values (integers or decimals, the regex `[0-9]+(\.[0-9]+)?`)
- Lists of heterogeneous values, passed by pointer
- Expressions (are not values)
- Null?

Ascii order: `*+-./`

Tokens:
- `*` (evaluation operator, "get")
- `*+` (signum operator, bikesheddable)
- `+` (add operator)
- `+-` (subtraction operator)
- ??? (multiplication operator)
- ??? (float division operator)
- ??? (integer division or rounding operator)
- `-*` (?, some TBD operator)
- `..` (list separator, "comma")
- `./` (list closing bracket)
- `/.` (list opening bracket)
- ??? (more to be added as needed)
- `INSERT` (keyword)
- `DELETE` (keyword)
- `HOME` (keyword)
- `END` (keyword)
- `PGUP` (keyword)
- `PGDN` (keyword)
- `ENTER` (keyword)
- \n (newline, by pressing Enter)
- ` `, \t, \r (whitespace is ignored and *not* used to separate tokens)
- the regex `([^)]*)` is a comment, *not* used to separate tokens

Possible tokenizer rules of thumb:
- Tokens starting with `*` are unary prefix operators?
- Tokens starting with `+` and `-` are binary operators?
- Periods are either surrounded by digits or part of a two-character token
- Tokens with slashes are structural

Syntax of an expression (where X, Y, ... are expressions):
- Numeric literal
- `/.` X `./`
- `/.` X (`..` Y)+ `./`
- `INSERT` X Y                    (can be chained `INSERT` X `INSERT` Y Z)
- `DELETE` X
- `ENTER` X
- `*` X
- `*+` X
- X o Y                           (where o is a binary operator)

Semantics:
- `INSERT` `/.` X `..` Y `./` Z   inserts the value X into the line/register numbered Y, then returns Z
- `INSERT` X Z                    where X is not a list: inserts X on the stack (at the end of program data?), then returns Z
- `DELETE` X                      pops a value from the top of the stack, then returns it
- `ENTER` X                       prints the Unicode codepoints in the list X to stdout, then gets a value (numeric or string as list of Unicode codepoints) from stdin
- `*` A                           where A is a numeric value: evaluates the expression at floor(A) and returns it
- `*` LIST                        returns the first element of LIST
- LIST `+` A                      where A is a numeric value: returns the "list" (that is, pointer) obtained from skipping the first floor(A) elements, similar to C where `A[3]` is `A + 3`

An "if statement" dissected:
106 `**/.102..107./ + *+ /.*103./ +- *101`
- `*101` is the result of evaluating the expression at address 101. If the expression at 101 is a numeric value, this is the same as dereferencing a pointer.
- `/.*103./` is a parenthesized expression that does the same thing for address 103.
- `/.*103./ +- *101` subtracts these two numbers.
- `*+ /.*103./ +- *101` performs a signum, thus returns either -1, 0 or 1.
- `/.102..107./` is a list with two elements, the value 102 and the value 107.
- `/.102..107./ + *+ /.*103./ +- *101` skips either 0 or 1 (or -1?!) elements, so returns a list that starts with either 102 or 107.
- `*/.102..107./ + *+ /.*103./ +- *101` returns this first element, so is either 102 or 107.
- `**/.102..107./ + *+ /.*103./ +- *101` is the result of evaluating either 102 or 107.
- This expression corresponds to an C expression of the shape `(*var103 == *var101) ? func102() : func107()`.

REPL/editor controls:
- NumLock is actual numlock, needed to access arrow keys, insert and del
- Arrow left and right moves the cursor for typing
- Arrow up and down selects different lines
- Numlocked Insert, Del, Home, End, PageUp and PageDown acts as in text editor
- Enter inserts a line feed, thus starting a new line
- Holding down Enter acts as AltGr, so Enter+0 inserts the keyword `INSERT`
- Enter+5 inserts the keyword `ENTER`?

Line numbers are written explicitly, followed by `..`.
(Maybe line numbers and newlines are semantically significant? But then the editor has to make sure lines don't shift down when you type new lines in the middle of a program.)
Line 1 is the entry point / main function, and evaluting a program just means evaluating line 1 (calling `*1`).
Evaluating a value or list just returns that value/list.
Evaluating an expression means evaluating it until you are left with either a numeric value or a list, then returning that value/list.

Open questions:
- How closely does * bind? (`*10+*20` versus `/.*10./+*20`)
- Is `/.X./` a list or a parenthesized expression? Maybe `/.X.../` for a list of length 1?
