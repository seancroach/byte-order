/// An enumeration used in the creation of [`NumberReader`] and [`NumberWriter`]
/// structures to specify what endianness their operations should be performed
/// with.
///
/// # Examples
///
/// Constructing a [`NumberWriter`] which writes numbers with big-endian byte
/// order:
///
/// ```
/// use byte_order::{ByteOrder, NumberWriter};
///
/// let be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
/// ```
///
/// Likewise, constructing another [`NumberWriter`] which now writes numbers
/// with little-endian byte order:
///
/// ```
/// # use byte_order::{ByteOrder, NumberWriter};
/// let le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
/// ```
///
/// [`NumberReader`]: crate::NumberReader
/// [`NumberWriter`]: crate::NumberWriter
#[derive(Debug, Eq, PartialEq)]
pub enum ByteOrder {
    BE,
    LE,
}

impl ByteOrder {
    /// The native-endian serialization of the target platform. This value will
    /// be equal to [`ByteOrder::BE`] or [`ByteOrder::LE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use byte_order::ByteOrder;
    ///
    /// assert_eq!(
    ///     ByteOrder::NE,
    ///     if cfg!(target_endian = "big") { ByteOrder::BE } else { ByteOrder::LE }
    /// );
    /// ```
    ///
    /// [`ByteOrder::BE`]: ByteOrder::BE
    /// [`ByteOrder::LE`]: ByteOrder::LE
    pub const NE: ByteOrder = if cfg!(target_endian = "big") {
        ByteOrder::BE
    } else {
        ByteOrder::LE
    };
}
