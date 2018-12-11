# Rust

## Overview of Rust
(todo: this)

## Summary
(todo: this)

## Notes

1. The book titled "The Rust Programming Language" will be referred to as
  "the Rust book" throughout this document to prevent the author from
  developing RSI symptoms.

## Language Syntax

- The syntax of Rust looks similar to C, using curly braces to enclose blocks
  of code and semicolons to mark the ends of statements.
- Variable types in Rust are written after the identifier,
  or implied by their first assignment.
- Like in Clojure, control flow constructs can be used as expressions.
- "arms" are used for any conditional syntax

### Example: Control Flow as Expressions
The following snippet of code is an example of a loop being used as an
expression. This loop reports the 6th fibonacci number. Additionally, this loop
produces a side effect, causing a and b to be the 4th and 5th fibonacci numbers
respectively.
```
    let mut a = 0; let mut b = 1;
    let mut i = 3;

    let fib5 = loop {
        let c = a + b;
        if i == 6 {
            break c;
        }
        a = b; b = c;
        i += 1
    };
```

## Metaprogramming

Yes.

(todo: elaborate using Rust's `vec!` example)

## Scoping

Lexical scoping with block scope

(todo: maybe elaborate)

## Functional Language Features
Rust has limited support for functional programming features. Because recursion
and closures with lexical scoping are supported, it is possible to implement and
compile code in a functional way. However, a problem occurs when implementing
code such as the following:

```
use std::thread;
use std::time::Duration;

fn recur(current: u32) -> u32 {
    if current == 0 {
        // Sleep for 5 seconds so the state of RAM use can be observed
        thread::sleep(Duration::from_secs(5));
    }
    return if current == 0 {
        1
    } else {
        recur(current - 1)
    }
}

fn main() {
    println!("{}", recur(10_000_000));
}
```

When running this code, I was hoping to observe RAM usage, forgetting that the
limit on the stack would catch the problem for me. This was the actual output
when running the program:

```
$ ./recursion 

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
Aborted (core dumped)
```

This leads me to believe that Rust does not optimize tail recursion. The same
result was observed when rewriting the code to not use `if` as an expression.

## Static Typing

(todo: talk about when Rust's compiler can infer types)

## Strengths and Weaknesses

(todo: try to get into the controversy without getting into the controversy)

## Interesting Examples

### Method behaviour based on return type

The following is an example from the Rust book.
```
let guess: u32 = guess.trim().parse()
	.expect("Please type a number!");
```

In this example, the `parse` method produces an error if it fails to
parse an integer. What I think is interesting about this is that the
desired format (i.e. integer or floating point) is not specified as
an argument to the parse method.

At first, I expected that the call to parse wouldn't report any error
if the user types `5.1`, but instead a different error would be
produced after parse returns and before the number is cast to `u32`.

To my surprise, the behaviour of the parse method seems to be
dependent on the type its return value is being returned to. I assume this is
done using method overloading, but I'm not sure yet.
