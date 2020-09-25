2 3 + .s
dup * .s
dup * .s
.

3 4 + .

decimal

: fib dup 2 < if drop 1 else dup 2 - recurse swap 1 - recurse + then ;
: fibs for i fib u. next ;

14 fib .

15 fibs .


