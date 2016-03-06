# nickel-diesel

Nickel middleware providing a [diesel ORM](diesel.rs) with
[r2d2](https://github.com/sfackler/r2d2) connection pooling

Code originally by Robert Yokota as part of the [Yeoman generator for AngularJS +
Nickel](https://github.com/rayokota/generator-angular-nickel/) project. Packaged
by Andrew Straw.

Due to the use of `#![feature(reflect_marker)]`, this package currently requires
rust nightly.
