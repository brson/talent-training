# PNA Rust &mdash; Building Blocks 5

Let's learn some building blocks!

Put your other projects and concerns aside. Take a breath and relax. Here
are some fun resources for you to explore.

Read all the readings and perform all the exercises.

- **[Reading: Zero-cost futures in Rust][zcf]**. The blog post announcing
  the original Rust futures design, and a good general overview of what
  futures are for. Note the mentions of "impl syntax" (A.K.A "impl trait"),
  and "async/await", two language features necessary for ergonomic futures
  in Rust.

- **[Reading: Returning types that implement traits][impl]**. The Rust Book's
  explanation of `impl Trait`. `impl Trait` is a language feature that makes
  it possible to return implementations of futures without naming the
  implementation, but also without [boxing] the implementation as a
  `Box<Future<...>>`.

- **[Reading: RFC 1522: Conservative `impl Trait`][rfc]**. The original
  motivation for `impl Trait`. In Rust, reading the RFCs can be the best source
  of documentation, though note that its common for the final design to have
  differences from the initial RFC specification.

- **Exercise: Create an asyncronous HTTP server with Hyper**.

  Following the [Hyper guide][hg], write the described "Hello, World!" server.

- **Exercise: Write an HTTP file server with Hyper**.

  In the previous exercise, the server ignored the [`Request`] completely, and
  always responded with "Hello, World!". This time, serve the local file that
  corresponds to the path of the requested [URI]. Replace the `hello_world`
  function with a `serve_file` function that looks like:

  ```
  fn hello_world(req: Request<Body>) -> impl Future<Response<Body>> {
      ...
  }  
  ```

  Note that the return type is no longer a `Response<Body>` but an
  implementation of `Future<Response<Body>>`. This is because your server now
  must do I/O (file I/O) in order to create the response. That is, it can no
  longer serve the response immediately, it must wait for the file to load from
  disk, then serve the response. This is the reason futures exist &mdash; if we
  "blocked" the thread while doing I/O then the server could only serve one file
  per thread at a time; by immediately returning a future the server can
  continue processing requests while the file is loaded from disk.

  For this exercise the URI path requested can correspond to the same path
  relative to the local current working directory ([`std::env::current_dir`]).

  To perform asynchronous file I/O you can't use the standard library, but instead
  a futures-based file I/O library. For this exercise use [`tokio::fs`].

  Note that Tokio is compatible with Hyper, as the default Hyper runtime
  ([`hyper::rt`]), _is_ the tokio runtime. Be sure though that you are using a
  version of Tokio that corresponds to the version of Hyper you are using. This
  can be determined by examining Hyper's dependencies.

[zcf]: https://aturon.github.io/blog/2016/08/11/futures/
[impl]: https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits
[rfc]: https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md
[hg]: https://hyper.rs/guides/server/hello-world/
[`Request`]: https://docs.rs/hyper/0.12.33/hyper/struct.Request.html
[URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
[`tokio::fs`]: https://docs.rs/tokio/0.1.22/tokio/fs/index.html
[`hyper::rt`]: https://docs.rs/hyper/0.12.33/hyper/rt/index.html
[`std::env::current_dir`]: https://doc.rust-lang.org/std/env/fn.current_dir.html

<!--

https://blog.stephencleary.com/2012/02/async-and-await.html
https://docs.microsoft.com/en-us/dotnet/csharp/async

- async cheat sheet
- Add https://www.youtube.com/watch?reload=9&v=9_3krAQtD2k&list=WL&index=6&t=848s
  as reading for async section?
- https://tmandry.gitlab.io/blog/posts/optimizing-await-1/
- https://aturon.github.io/tech/2018/04/24/async-borrowing/
- https://areweasyncyet.rs/ - contains async blog posts
- Async I/O
  - [Getting Acquainted with `mio`](https://hoverbear.org/2015/03/03/getting-acquainted-with-mio/)
  - [My Basic Understanding of `mio` and Async I/O](http://hermanradtke.com/2015/07/12/my-basic-understanding-of-mio-and-async-io.html)
  - [Creating a Simple Protocol With `mio`](http://hermanradtke.com/2015/09/12/creating-a-simple-protocol-when-using-rust-and-mio.html)
  - [Managing Connection State With `mio`](http://hermanradtke.com/2015/10/23/managing-connection-state-with-mio-rust.html)
  - [Zero-cost Futures in Rust](http://aturon.github.io/blog/2016/08/11/futures/)
  - [Designing Futures for Rust](http://aturon.github.io/blog/2016/09/07/futures-design/)
  - [Asynchronous Rust for Fun and Profit](http://xion.io/post/programming/rust-async-closer-look.html)
  - [Understanding the Tokio Reactor Core](https://www.coredump.ch/2017/07/05/understanding-the-tokio-reactor-core/)
- https://rust-lang.github.io/async-book/
- https://cheats.rs/#async-await-101

-->