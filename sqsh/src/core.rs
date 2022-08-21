//! # Core library
//!
//! This is the core library containing abstractions and traits.
//! While there are specific notes on each implementation, the core principles
//! are described in this module.
//!
//! The basic setup of the library takes a data source, processses the data,
//! produces new data and writes it into a data sink. This is valid for any
//! data processing in computer science. This module provides the necessary
//! abstractions and traits to implement these processes.
//!
//! The data source and data sinks are already provided by the Rust Standard
//! library and its `io` module. A data source should implement the
//! [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) trait and
//! a data sink implement the
//! [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) trait.
//! For the processing of the data the new Trait `Process` is being defined.
//!
//! These three components define the core of the data processing in the
//! library. The interaction of these components are organised by a `Stream`
//! object which coordinates the whole interaction.

/// The `Process` trait allows processing bytes from a source and
/// writing the results to a sink.
///
/// Implementor of the `Process` trait are called `processors`.
pub trait Process {
  fn process(&mut self, source: &[u8], sink: &mut [u8]) -> Option<usize>;
  fn finish(&mut self, sink: &mut [u8]);
}
