# Project: Asynchrony

**Task**: Create a multi-threaded, persistent key/value store server and client
with _asynchronous_ networking over a custom protocol.

**Goals**:

- Understand the patterns used when writing Rust futures
- Understand error handling with futures
- Learn to debug the type system
- Perform asynchronous networking with the tokio runtime
- Use boxed futures to handle difficult type-system problems
- Use `impl Trait` to create anonymous `Future` types

**Topics**: asynchrony, futures, tokio, "impl Trait".

**Extensions**: tokio-fs.

- [Introduction](#user-content-introduction)
- [Project spec](#user-content-project-spec)
- [Project setup](#user-content-project-setup)
- [Background: The state of async Rust](#user-content-background-the-state-of-async-rust)
- [Background: Thinking in futures, in Rust](#user-content-background-thinking-in-futures-in-rust)
- [Part 1: Introducing tokio to the client](#user-content-part-1-introducing-tokio-to-the-client)
- [Part 2: Converting `KvsClient` to boxed futures](#user-content-part-2-converting-kvsclient-to-boxed-futures)
- [Part 3: `KvsClient` with explicit future types](#user-content-part-3-kvsclient-with-explicit-future-types)
- [Part 4: `KvsClient` with anonymous future types](#user-content-part-4-kvsclient-with-anonymous-future-types)
- [Part 5: Making `ThreadPool` sharable](#user-content-part-5-making-threadpool-sharable)
- [Part 6: Converting `KvsEngine` to futures](#user-content-part-6-converting-kvsengine-to-futures)
- [Part 7: Converting `KvsServer` to tokio](#user-content-part-7-converting-kvsserver-to-tokio)
- [Extension 1: Converting to tokio-fs](#user-content-extension-1-converting-to-tokio-fs)


## Introduction

_Note: this project is only outlined, not written. If you are at this point in
the course email brian@pingcap.com and let me know and I will finish writing it
ASAP._

In this project you will create a simple key/value server and client that
communicate over a custom protocol. The server will use asynchronous networking,
built on the tokio runtime. The key/value engine that reads and writes to files
will remain synchronous, scheduling work on an underlying thread pool, while
presenting an asynchronous interface. Along the way you will experiment
with multiple ways of defining and working with future types.

Because learning to program with Rust futures is especially challenging, and
existing documentation on the subject is limited, the scope of this project is
relatively modest, and it contains more direct explanation than past projects.

This project will focus on learning to program with futures directly. Later
projects will introduce [async / await], a language feature that makes writing
asynchronous code more naturally. Learning how to write futures first will make
understanding async / await easier.

Be sure to read the background readings on this project. And if you get
frustrated, then forgive yourself, take a break, and try again with a fresh
mind. Writing asynchronous Rust is difficult for everybody.

**Note: this and future projects temporarily require the nightly compiler. See
["Part 1: Using the nightly toolchain"][part-1] for rationale and instructions.**

[part-1]: #user-content-part-1-using-the-nightly-toolchain

## Project spec

The cargo project, `kvs`, builds a command-line key-value store client called
`kvs-client`, and a key-value store server called `kvs-server`, both of which in
turn call into a library called `kvs`. The client speaks to the server over
a custom protocol.

The interface to the CLI is the same as in the [previous project]. The engine
implementation is largely the same, distributing synchronous file I/O over
a thread pool.

The difference this time is that all the networking is performed asynchronously.

As part of the conversion to asynchrony, the `KvsClient` will present a
futures-based API, and the `KvsEngine` trait will also present a futures-based
API, even while it remains implemented with blocking (synchronous) I/O via a
thread pool (for now).

Up until this point the API for KvsClient has been unspecified. In this
project the API will be explicitly specified.

Your `KvsServer` will be based on the tokio runtime, which handles the
distribution of asynchronous work to multiple threads on its own (tokio itself
contains a thread pool). This means that your architecture will actually have
two layers of thread pools: the first handling with the networking,
asynchronously, one thread per core; the second handling the file I/O,
synchronously, with enough threads to keep the networking threads as busy as
possible.

As a result if this architectural change, where your jobs will be spawned into
your thread pool from multiple threads, your `ThreadPool` trait and its
implementations will become shared types implementing `Clone + Send + 'sync`, as
your `KvsEngine` is.

To gain experience you will experimenting with multiple definitions of the
futures returned by these types. You will work with function
signatures like all the following:

- `Client::get(&mut self, key: String) -> Box<dyn Future<Item = Option<String>, Error = Error>`

- `Client::get(&mut self, key: String) -> future::SomeExplicitCombinator<...>`

- `Client::get(&mut self, key: String) -> impl Future<Item = Option<String>, Error = Error>`

Ultimately, `KvsClient` will have the following signature:

```rust
impl KvsClient {
    // TODO
}
```

`KvsEngine` will have the following signature:

```rust
pub trait KvsEngine: Clone + Send + 'static {
    // TODO
}
```


## Project setup

Continuing from your previous project, delete your privous `tests` directory and
copy this project's `tests` directory into its place. This project should
contain a library named `kvs`, and two executables, `kvs-server` and
`kvs-client`.

You need the following dev-dependencies in your `Cargo.toml`:

```toml
[dev-dependencies]
assert_cmd = "0.11"
criterion = "0.2.11"
crossbeam-utils = "0.6.5"
predicates = "1.0.0"
rand = "0.6.5"
tempfile = "3.0.7"
walkdir = "2.2.7"
panic-control = "0.1.4"
```

Unlike with previous projects, don't bother to fill in enough type definitions
to make the test suite compile. Doing so would require jumping a number of steps
ahead at once. The text will indicate when to start working with the test suite.


## Background: The state of Rust futures

Asynchronous Rust has a long history, and at the time of this writing (September
2019) neither the language features or ecosystem are mature. The world of Rust
async is in a transitional period, moving from futures 0.1, to std futures
(A.K.A. "futures 0.3", and [async / await].

This course is going to focus on std futures, tokio 0.2, and async / await. As
such, it will be depending on crates that are still in an alpha state, and that
depend on the nightly compiler. As such, *this project will depend on the nightly
compiler until async / await is released with Rust 1.39, in November 2019,
after which the course will be updated to use the stable compiler again*.

Since Rust 1.0 the best practice for writing futures-based asynchronous code in
Rust has been to use Tokio 0.1. Tokio was the first production-ready
asynchronous programming library for Rust and pioneered many of the libraries
and techniques used in asynchronous programming.

Tokio 0.1 is built around futures 0.1, a standalone crate.

As of Rust 1.36, released in July 2019, the `Futures` trait is defined in the standard
library, as `std::future::Future`. This definition of `Future` though is _different_
from futures 0.1, and is designed to work more seamlessly with [async / await].

`futures::Future` as of 0.1:

```rust
pub trait Future {
    type Item;
    type Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;

	<... a variety of built-in future "combinators", like ...>

    fn map<F, U>(self, f: F) -> Map<Self, F>
    where
        F: FnOnce(Self::Item) -> U,
        Self: Sized { ... }
}
```

`std::future::Future`:

```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```

There are 3 differences between the 0.1 `Future` and std `Future`:

1) There is no longer an `Error` associated type. Instead the `Output` type
   should be a `Result` whenever an error is possible. This in particular makes
   async function signatures look more like non-async function signatures.

2): std `Future` does not define any future combinators. Those are defined
   outside of std, notably in the [`futures-preview`] crate.

3): The signature of `poll` has changed significantly. `poll` is a low-level
   method written by implementors of `Future`. It does not need to be understood
   to write futures-based code, and won't be discussed in this chapter.

The [`futures-preview`] crate is version 0.3, and as such std futures are often
also refered to as futures 0.3. The `Future` defined within is simply a reexport
of `std::future::Future`. Presumably, `futures-preview` will someday be renamed
to simply `futures`. As of now though, using std futures will generally also
entail using the `futures-preview` crate.

[tokio 0.1]: https://docs.rs/tokio/0.1
[futures 0.1]: docs.rs/futures/0.1
[async / await]: https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
[`std::future::Future`]: https://doc.rust-lang.org/std/future/trait.Future.html
[`futures-preview`]: https://docs.rs/futures-preview/


## Background: Thinking in futures, in Rust

- why futures? networking vs file/io, blocking vs non-blocking, sync vs async
- futures from a user persective (not a poll-centric implementation perspective)
- don't think too hard about executors and runtimes
- method chaining and how it transforms the future type
- debugging Rust types
- Result vs Future vs FutureResult
- error handling with futures
- concrete futures vs boxed futures vs anonymous futures
- note about futures 0.1 and futures 0.3 (we'll use futures 0.1)
- note about async / await


## Part 1: Using the nightly toolchain

TODO


## Part 2: Converting `KvsClient` to boxed futures

the path of least resistence for future types


## Part 3: `KvsClient` with explicit future types

just to have the experience of seeing how untenable it is


## Part 4: `KvsClient` with anonymous future types

the final solution


## Part 5: Making `ThreadPool` sharable


## Part 6: Converting `KvsEngine` to futures

for the server we're going to do the opposite of what we did in the client, and
give `KvsEngine` an async interface. this will show that futures and the
underlying runtime are independent, and just general provide a spectrum of
experience.


## Part 7: Converting `KvsServer` to tokio

note that even though we have ourselves have written very little asynchronous
code, that tokio itself is distributing asynchronous work across num_cpus
threads. think about the tradeoffs of putting cpu-intensive work directly on the
network threads or the file threads, e.g. where does the serialization go?

TODO

Nice coding, friend. Enjoy a nice break.


---


## Extension 1: Converting to tokio-fs

not sure if this should be required or an extension


<!--

TODO:
- can we find an excuse to write a future by hand?

- background readings
  - something on associated types

via @sticnarf:

> As there is only the outline of project 5, I write the code mostly according to
my own thoughts. Hope this will be a reference while you're writing the text.
@brson

> I change the concurrent_get/set tests to use async. Students should change their
SledKvsEngine and KvStore to adapt to the KvsEngine trait with new async APIs.
The engines have a ThreadPool type parameter and the constructor has a
concurrency argument (maybe we should remove it). Students need to follow the
design so that the test will work.

> I don't test the client. Implementors can choose the API design of the client
themselves (unless we work out a perfect design so we can just give instructions
to students).

-->
