//! This crate provides convenience methods for encoding and decoding numbers in
//! either big-endian or little-endian byte order.
//!
//! The organization of the crate is simple. A struct, [`NumberReader`], wraps
//! some [reader] with convenience methods to read each type of number in Rust
//! except for numbers that have platform-dependent sizes (`usize` and
//! `isize`). Likewise, a struct, [`NumberWriter`], does the same for
//! [writers]. Finally, an enum named [`ByteOrder`] is used to differentiate
//! the endianness that [`NumberReader`] and [`NumberWriter`] structures perform
//! their operations.
//!
//! # Examples
//!
//! Read unsigned 16-bit big-endian integers from a reader:
//!
//! ```
//! use std::io::{self, Cursor};
//! use byte_order::{ByteOrder, NumberReader};
//!
//! fn main() -> io::Result<()> {
//!     let src = Cursor::new(vec![0x12, 0x34, 0x56, 0x78]);
//!
//!     let mut be_reader = NumberReader::with_order(ByteOrder::BE, src);
//!     assert_eq!(0x1234u16, be_reader.read_u16()?);
//!     assert_eq!(0x5678u16, be_reader.read_u16()?);
//!
//!     Ok(())
//! }
//! ```
//!
//! Write unsigned 16-bit little-endian integers to a writer:
//!
//! ```
//! use std::io;
//! use byte_order::{ByteOrder, NumberWriter};
//!
//! fn main() -> io::Result<()> {
//!     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
//!     le_writer.write_u16(0x1234)?;
//!     le_writer.write_u16(0x5678)?;
//!     assert_eq!(le_writer.into_inner(), vec![0x34, 0x12, 0x78, 0x56]);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Alternatives
//!
//! This crate is an alternative to `byteorder`. What makes `byte_order`
//! different at the design-level is the exclusion of extension traits and the
//! ability to decide endianness once at the creation of a [`NumberReader`] or
//! [`NumberWriter`] instead of once per operation.
//!
//! The performance between `byteorder` and `byte_order` is comparable. It is
//! strongly advised you compare these two crates and use what best suits your
//! specific use case.
//!
//! Likewise, as of Rust 1.32, the standard numeric types provide built-in
//! methods like `to_le_bytes` and `from_be_bytes`, which support many of the
//! same use cases.
//!
//! [`NumberReader`]: crate::NumberReader
//! [`NumberWriter`]: crate::NumberWriter
//! [reader]: https://doc.rust-lang.org/std/io/trait.Read.html
//! [writers]: https://doc.rust-lang.org/std/io/trait.Write.html
//! [`ByteOrder`]: crate::ByteOrder

mod order;
mod read;
mod write;

pub use order::ByteOrder;
pub use read::NumberReader;
pub use write::NumberWriter;
