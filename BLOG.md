## An argument against the Rust compiler's amazing error messages

It's been a minute since the last commit to this repo, but because
the Rust compiler's error messages is a common selling point.  I thought
I'd take the time to say that they suck.  In specfic scenerios, like the one
I encountered when writing the Rust version of the retry function.  The Rust
compiler's error messages were entirely unhelpful, and I spent an unknown
quantity of my life listening to the compiler who was sending me off 
on the wildest of goose chases.

Granted, this exercise was my first real attempt at not writing a "Hello world"
application in Rust; and unfortunately, since I did not commit every 
thought/recommendation from the compiler to fix the problems encountered.
There isn't really a good way to point to the specific examples.

Working from the volatile memory in my smooth human brain and all caveats considered.
The compiler kept suggesting to add different traits(? I don't remember if they
were traits, but it definately kept asking me to add things) to the generic parameters.

So the secret to the Rust compiler's error messages?  Throw stuff at the wall
and see what sticks.

It wasn't until I threw the compiler out the window and really thought
about the problem; that I managed to write something that resembled a solution.  

So it turns out, sometimes you have still have to understand the problem to come
up with a reasonable resolution.

## Lost in translation

An attempt to replicate the problem

```rust
fn func<T, E>(f: fn() -> Result<T,E>) -> Result<T,E> {
    f()
}

fn main() {
    let x = 10;
    fn fun() -> Result<T, E> {
        Ok(x)
    }

    let r = func(fun);
    println!("Result: {:?}", r.ok());
}
```

The errors

```
   Compiling rust-errs v0.1.0 (/home/cpalv/workspace/experiments/rust-errs)
error[E0434]: can't capture dynamic environment in a fn item
 --> src/main.rs:8:12
  |
8 |         Ok(x)
  |            ^
  |
  = help: use the `|| { ... }` closure form instead

error[E0412]: cannot find type `T` in this scope
 --> src/main.rs:7:24
  |
7 |     fn fun() -> Result<T, E> {
  |                        ^ not found in this scope
  |
help: you might be missing a type parameter
  |
7 |     fn fun<T>() -> Result<T, E> {
  |           +++

error[E0412]: cannot find type `E` in this scope
   --> src/main.rs:7:27
    |
7   |     fn fun() -> Result<T, E> {
    |                           ^
    |
   ::: /home/cpalv/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:285:1
    |
285 | pub trait Eq: PartialEq<Self> {
    | ----------------------------- similarly named trait `Eq` defined here
    |
help: a trait with a similar name exists
    |
7   |     fn fun() -> Result<T, Eq> {
    |                           ~~
help: you might be missing a type parameter
    |
7   |     fn fun<E>() -> Result<T, E> {
    |           +++

Some errors have detailed explanations: E0412, E0434.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `rust-errs` due to 3 previous errors
```

For whatever reason, maybe the compile errors scrolled past the terminal window,
I tunnel visioned on `help: a trait with a similar name exists`
and `help: you might be missing a type parameter`.
I took this to mean that the trait needed to be added.

So what gives for the other errors? `E0412 cannot find type in this scope`
The function `fun` is defined exactly as the function `func` expects.

Semantically, the function `fun` returns a `Result` that contains
any type T XOR E.  However, T and E are not the same as Rust's [any type](https://doc.rust-lang.org/std/any/index.html).
These are type parameters that need concrete types defined later or
may be inferred by the compiler.

So because we flipped a coin and decided to work from the bottom up;
let's try the suggested changes and see if the compiler can help us.

new code

```rust
fn main() {
    let x = 10;
    fn fun<T,Eq>() -> Result<T, Eq> {
        Ok(x)
    }

    let r: Result<T,Eq> = func(fun);
    println!("Result: {:?}", r.ok());
}
```

new errors

```
   Compiling rust-errs v0.1.0 (/home/cpalv/workspace/experiments/rust-errs)
error[E0434]: can't capture dynamic environment in a fn item
 --> src/main.rs:8:12
  |
8 |         Ok(x)
  |            ^
  |
  = help: use the `|| { ... }` closure form instead

error[E0412]: cannot find type `T` in this scope
  --> src/main.rs:15:19
   |
15 |     let r: Result<T,Eq> = func(fun);
   |                   ^ not found in this scope

error[E0782]: trait objects must include the `dyn` keyword
  --> src/main.rs:15:21
   |
15 |     let r: Result<T,Eq> = func(fun);
   |                     ^^
   |
help: add `dyn` keyword before this trait
   |
15 |     let r: Result<T,dyn Eq> = func(fun);
   |                     +++

error[E0038]: the trait `Eq` cannot be made into an object
   --> src/main.rs:15:21
    |
15  |     let r: Result<T,Eq> = func(fun);
    |                     ^^ `Eq` cannot be made into an object
    |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   --> /home/cpalv/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:285:15
    |
285 | pub trait Eq: PartialEq<Self> {
    |               ^^^^^^^^^^^^^^^ the trait cannot be made into an object because it uses `Self` as a type parameter

Some errors have detailed explanations: E0038, E0412, E0434, E0782.
For more information about an error, try `rustc --explain E0038`.
error: could not compile `rust-errs` due to 4 previous errors
```

So, we can spend some time coping.  Telling ourselves that
we have new error messages and therefore have made some sort
of progress. Or we can ignore ~80% of the error text
and try using the closure syntax instead.

new code

```rust
fn main() {
    let x = 10;

    let fun = || Ok(x);
    let r = func(fun);
    println!("Result: {:?}", r.ok());
}
```

new errors

```
   Compiling rust-errs v0.1.0 (/home/cpalv/workspace/experiments/rust-errs)
error[E0308]: mismatched types
  --> src/main.rs:15:18
   |
14 |     let fun = || Ok(x);
   |               -- the found closure
15 |     let r = func(fun);
   |             ---- ^^^ expected fn pointer, found closure
   |             |
   |             arguments to this function are incorrect
   |
   = note: expected fn pointer `fn() -> Result<_, _>`
                 found closure `[closure@src/main.rs:14:15: 14:17]`
note: closures can only be coerced to `fn` types if they do not capture any variables
  --> src/main.rs:14:21
   |
14 |     let fun = || Ok(x);
   |                     ^ `x` captured here
note: function defined here
  --> src/main.rs:1:4
   |
1  | fn func<T, E>(f: fn() -> Result<T,E>) -> Result<T,E> {
   |    ^^^^       ----------------------

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rust-errs` due to previous error
```

You're aware of type coercion from dynamically typed language horror stories,
but you need to be able to capture variables.  Since functions generally aren't very
useful if they don't have stuff to work with.  You might be able to use [macros](https://doc.rust-lang.org/rust-by-example/macros/variadics.html) to solve this problem, but have decided you don't want to start chopping down
that part of the forest yet.  After some research, you discover function traits: [Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html), [FnMut](https://doc.rust-lang.org/std/ops/trait.FnMut.html), [FnOnce](https://doc.rust-lang.org/std/ops/trait.FnOnce.html)
and finally get something that works as you'd expect.

![rust meme](https://github.com/cpalv/RetryFunctions/blob/main/imgs/rustc.jpg)

## Suggested changes

1. closure err.help() to suggest usage of Fn, FnMut, or FnOnce

patch coming soon<sup>tm</sup>

2. More involed, if in a { } scope, check that type parameters are concrete types.
if not suggest usage of primatives or other defined types.

patch coming soon<sup>tm</sup>
