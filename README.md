
# Minigrep

My implementation of the project in [Chapter 12](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html) of the [Rust Book](https://doc.rust-lang.org/stable/book/).

I have been going through the book for the past few days and after this chapter I hope I'll finally be able to write a few of my own projects in Rust.

*Minigrep* is a command line tool similar to *grep*.

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

My "main" language is C#, so a lot of my learning is going to relate to differences and simularities between the languages.

### Modules, files, paths

It's weird having declairations, methods, and implementations spread around. I prefer objects over C-like structs.

### Vecs and Strings

Strings are weird, I wish there was a way to work with pure ascii strings other than using a `Vec<u8>` or something.

I don't work with strings (other than string literals) often, so at least I don't have an entrenched mental model I'd have to fight.

`Vec` is similar to `std::vector` in C++, or `List` in C#, or ArrayList I implemented 100 times in Java for uni courses.
I like the functional style Rust supports, obviously it's pretty similar to .NET LINQ which I also like.

### Errors

I wish C# had native support for errors instead of exceptions. Errors feel much easier to work with, especially for things that are expected to go wrong like io.

### Lifetimes



### Traits

It's nice to have an almost interface to work with.

### Testing

> Anthony Cieri [anthonycieri.com](https://anthonycieri.com)
