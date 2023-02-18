unary
operators
```
*     fetch
+     signum
-     negate
/     reciprocal

Up    Cieling
Down  Floor
End   prints
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
```

Declaration
```
(if a line starts with a number, it is a fetchable)
100 5    (*100 => 5)
200 *100 (*200 => *100 => 5)
```

Variables
```
(assign to a variable with -)
100 - 5 (this declares 100 and assigns it)
```