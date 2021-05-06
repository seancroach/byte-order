#[macro_use]
extern crate docgen;

mod read;
mod write;

/// The two possible endianness operations can be performed with.
// TODO: Create meaningful examples in how this enum is used.
pub enum Endianness {
    BE,
    LE,
}

// TODO: Evaluate the necessity of these aliases and document examples:

/// An alias for `Endianness::BE` in contexts where verbosity may be required,
/// such as when collaborators may not know what "BE" quite means.
pub const BIG_ENDIAN: Endianness = Endianness::BE;

/// An alias for `Endianness::LE` in contexts where verbosity may be required,
/// such as when collaborators may not know what "LE" quite means.
pub const LITTLE_ENDIAN: Endianness = Endianness::LE;

/// An alias for `Endianness::BE` in contexts where verbosity may be required,
/// such as when collaborators may not know what "BE" quite means.
///
/// Particularly helpful when implementing a protocol that calls for "network
/// order" and prevents possible confusion when referring to big endian and
/// network order when possible collaborators are unaware of these topics.
pub const NETWORK_ORDER: Endianness = Endianness::BE;

pub use read::EndianReader;
pub use write::EndianWriter;
