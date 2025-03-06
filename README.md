
# Minigrep

My implementation of the project in [Chapter 12](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html) of the [Rust Book](https://doc.rust-lang.org/stable/book/).

I have been going through the book for the past few days and after this chapter I hope I'll finally be able to write a few of my own projects in Rust.

*Minigrep* is a command line tool similar to *grep*.

***

## Download, Build, Run, etc.

### Downloading

***TODO***

```bash
git clone <repo>
```

### Building and Running

***TODO***

```bash
cd <repo - minigrep probably>

cargo run
```

## Process, Thoughts, and Learning

My "main" language is C#, so a lot of my learning is going to relate to differences and similarities between the languages.

***

### Modules, files, paths

These files are roots for *crates* inside a *package*
- `src/main.rs`
- `src/bin/**.rs`
- `src/lib.rs` Note: This makes a library crate with the same name as the package.

A *module* is declared inside crate root files with `mod <name>`
The code for a module is in
1. A block following the declaration
2. `src/<name>.rs`
3. `src/<name>/mod.rs` (deprecated)

*Submodules* are declared inside any other files, and the code is placed in the same places just nested in directories or `mod` blocks.

So a package could look like this:

```bash
$ ls -R package

> package/src:
> main.rs lib.rs mod.rs module/
>
> package/src/module:
> submodule.rs

$ cat package/src/lib.rs

> pub mod module;
> /* library crate code */

$ cat package/src/module.rs

> pub mod submodule;
> /* module code */

$ cat package/src/module/submodule.rs

> /* code for submodule */

$ cat package/src/main.rs

> use package::{self, module, module::submodule}
> /* binary crate code */
```

It's weird having declarations, methods, and implementations spread around. I prefer objects over C-like structs.

The Rust module and file organization system is super strict.

Modules are different from namespaces in C# because they cannot span multiple files (there are ways to get around that on the API level).
They are similar to Python modules.

I like having each struct in its own file, especially with implementations and traits outside of the struct block.

The "rustic" way to do this (I'm not entirely sure it's not a cult) is to use private modules and then expose types with `pub use`.

***

### Vecs and Strings

Strings are weird, I wish there was a way to work with pure ascii strings other than using a `Vec<u8>` or something.

I don't work with strings (other than string literals) often, so at least I don't have an entrenched mental model I'd have to fight.

`Vec` is similar to `std::vector` in C++, or `List` in C#, or ArrayList I implemented 100 times in Java for uni courses.
I like the functional style Rust supports, obviously it's pretty similar to .NET LINQ which I also like.

***

### Errors

I like errors. It would be cool if C# had native support for errors as well as exceptions. Errors feel much easier to work with for things that are expected to go wrong like io.

***

### Lifetimes

In order to guarantee safety without GC, the Rust compiler needs to know how long data is going to "live" (be referenced).

If data is freed before all references are invalidated, then use after frees and double frees are possible.

Rust requires that data "outlives" all references.

Lifetimes are how Rust keeps track of references through stack frames.

```rust
/*
    This function takes in two refs and returns a ref
    The returned ref must be one of the refs passed in
    because anything created in the function would be
    local to the function and could not be returned

    'a is a lifetime annotation.

    The reference this function returns will be valid
    for at least as long as *both* num1 and num2 are valid
    because we don't know which one it returns
 */
fn func_1<'a>(num1: &'a i32, num2: &'a i32) -> &'a i32 { /* ... */ }

/*
    The annotation is only needed on refs that are returned
 */
fn func_2<'a>(to_ret: &'a mut i32, not_ret: &i32) -> &'a i32 {
   *to_ret += not_ret;
   to_ret
}

/*
    Usage with structs, etc.

    Structs with refs need lifetimes as well
 */
struct MyStruct<'a> {
   ref1: &'a i32,
   ref2: &'a mut i32
}

fn func_2<'a>(to_ret: &'a mut MyStruct, not_ret: &i32) -> &'a i32 {
   *to_ret.ref2 += to_ret.ref1 + not_ret;
   to_ret.ref2
}
```

***

### Traits

It's nice to have an almost interface to work with.

***

### Testing

> Anthony Cieri [anthonycieri.com](https://anthonycieri.com)
