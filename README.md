# nickel-diesel [![Version][version-img]][version-url] [![Status][status-img]][status-url]

Nickel middleware providing a [diesel ORM](diesel.rs) with
[r2d2](https://github.com/sfackler/r2d2) connection pooling

Code originally by Robert Yokota as part of the [Yeoman generator for AngularJS +
Nickel](https://github.com/rayokota/generator-angular-nickel/) project. Packaged
by Andrew Straw.

[version-img]: https://img.shields.io/crates/v/nickel-diesel.svg
[version-url]: https://crates.io/crates/nickel-diesel
[status-img]: https://travis-ci.org/strawlab/nickel-diesel.svg?branch=master
[status-url]: https://travis-ci.org/strawlab/nickel-diesel

## example

Run an example server:

    cargo run --example simple_sqlite_memory

Running the server will show

    Listening on http://127.0.0.1:9001
    Ctrl-C to shutdown server

The server can be tested with

    curl http://localhost:9001/one
