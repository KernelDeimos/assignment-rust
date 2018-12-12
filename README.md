# Rust

## Overview of Rust
(todo: this)

## Summary
(todo: this)

## Notes

1. The book titled "The Rust Programming Language" will be referred to as
  "the Rust book" throughout this document to prevent the author from
  developing RSI symptoms.
2. I have never used or learned about Rust prior to this project, but I have
  used Go, which is frequently compared to Rust. Because of this, I will comment
  on some of the differences and similarities between Rust and Go.

## Language Syntax

- The syntax of Rust looks similar to C, using curly braces to enclose blocks
  of code and semicolons to mark the ends of statements.
- Like in Go, brackets aren't required around conditions when using `if`, `for`,
  or `while`.
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

The Rust book explains macros using the built in `vec!` macro as an example.
This macro creates a vector, and makes multiple calls to the `push` method to
add each element. An advantage this has is that there does not need to be
separate logic for adding items when constructing a vector vs adding items after
a vector has been constructed.

Rust has a two different types of macros, and the one mentioned above is called
a **declarative macro**. These use a special syntax, separate from other Rust
code, to define macros.

The other type is a procedural macro. These are more similar to macros in
Clojure because Rust source code is used to modify other Rust source code.

Macros are one of the features which separate Rust from Go. However, Go has
tools available to encourage the use of custom code generators.

## Scoping and Ownership

Rust uses lexical scoping with block scope, which is typical in most modern
languages. However, Rust has very specific rules about when variables are valid
to ensure any compiled code can't make invalid accesses to memory or cause
race conditions. They claim that any code which can cause a data race simply
won't compile. (although this is different from a general race condition which
can be caused by logic errors)

Here's a simplified take on Rust's ownership rules:
- Once the scope owning a value is closed, data on the heap is cleaned up
- Ownership can be transferred to another function (or a closure; I tested it)
- Once ownership is transferred, the original variable becomes invalid
- Ownership can be "borrowed" using references
- Specific rules prevent borrowing in situations where a data race could occur

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

Rust uses static typing. Rust also has support for generics using a syntax
similar to that seen in C++ or Java. This is one of the features which set Rust
apart from Go, which does not support generics, but has an "empty interface"
type which can store any object (similar to Object in Java).

Like most modern statically typed languages, Rust will infer the type of data
when enough information is available for the compiler to figure it out, which
saves extra typing for the programmer.

Another interesting feature is that a function can be generic to its return
type, or a subset of the possible return types. For example, the parse function
on strings will behave differently depending on whether the return value is an
integer (will only accept whole number strings) or a float (will accept a
decimal point in the string).

The following is an example from the Rust book.
```
let guess: u32 = guess.trim().parse()
	.expect("Please type a number!");
```

It can be seen in this example that parse is called with no parameters, and
cast to the `u32` type (unsigned integer). When entering a value such as `5.1`,
the "Please type a number!" error is displayed, which shows that the parse
method knows its return value is being cast to an integer.

## Strengths and Weaknesses

### Some Strengths
- Invalid memory access not possible
- Syntax is borrowed from existing languages
- Concurrency is less scary

### Some Weaknesses
- Ownership rules can be difficult to remember, which makes code more likely
  not to compile
  - To be honest I'm not sure if this is a weakness, because if the compiler
    didn't catch this a user might
- Macro syntax is hard to read (the Rust book admits this)
- If you want to write a program that performs invalid access to memory,
  this might not be possible to do in Rust (for instance, to test robustness
  of an operating system). This is kind of beyond the scope of Rust but I wanted
  to include at least three weaknesses and I'm sure this technically counts.

## Webserver

I've looked at an HTTP library called hyper and placed the example code from
their documentation under `/webserver` of this repo. Unfortunately I wasn't able
to install the dependancy due to issue #2078 of Cargo
(https://github.com/rust-lang/cargo/issues/2078), so instead I'll discuss some
interesting things about the library.
