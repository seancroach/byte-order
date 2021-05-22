use std::io::{Result, Write};

use crate::order::ByteOrder;

/// A `NumberWriter` wraps a [writer] and provides methods for writing numbers.
///
/// Unlike many libraries, which take byte order as a parameter per operation, a
/// `NumberWriter` takes byte order as a parameter upon initialization.
///
/// # Examples
///
/// We can create a new `NumberWriter` using the target endianness using
/// [`NumberWriter::new`]:
///
/// ```
/// use byte_order::NumberWriter;
///
/// let mut writer = NumberWriter::new(vec![]);
/// ```
///
/// Or, to write numbers with a certain endianness, we can create `NumberWriter`
/// structures using [`NumberWriter::with_order`]:
///
/// ```
/// use byte_order::{ByteOrder, NumberWriter};
///
/// let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
/// let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
/// let mut ne_writer = NumberWriter::with_order(ByteOrder::NE, vec![]);
/// ```
///
/// To write numbers from the underlying [writer], use any of the `write_*`
/// methods that are provided:
///
/// ```
/// use std::io;
/// use byte_order::{ByteOrder, NumberWriter};
///
/// fn main() -> io::Result<()> {
///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
///     be_writer.write_u16(0xA1B2)?;
///     assert_eq!(be_writer.into_inner(), vec![0xA1, 0xB2]);
///
///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
///     le_writer.write_u16(0xA1B2)?;
///     assert_eq!(le_writer.into_inner(), vec![0xB2, 0xA1]);
///
///     let mut ne_writer = NumberWriter::with_order(ByteOrder::NE, vec![]);
///     ne_writer.write_u16(0xA1B2)?;
///     assert_eq!(
///         ne_writer.into_inner(),
///         if cfg!(target_endian = "big") {
///             vec![0xA1, 0xB2]
///         } else {
///             vec![0xB2, 0xA1]
///         }
///     );
///
///     Ok(())
/// }
/// ```
///
/// [writer]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`NumberWriter::new`]: NumberWriter::new
/// [`NumberWriter::with_order`]: NumberWriter::with_order
pub struct NumberWriter<W: Write> {
    inner: W,
    order: ByteOrder,
}

impl<W: Write> NumberWriter<W> {
    #[inline]
    pub fn new(w: W) -> NumberWriter<W> {
        NumberWriter::with_order(ByteOrder::NE, w)
    }

    #[inline]
    pub fn with_order(order: ByteOrder, w: W) -> NumberWriter<W> {
        NumberWriter { inner: w, order }
    }

    /// Consumes this `NumberReader`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use byte_order::NumberReader;
    ///
    /// let reader = NumberReader::new(Cursor::new(vec![]));
    ///
    /// let cursor = reader.into_inner();
    /// ```
    pub fn into_inner(self) -> W {
        self.inner
    }

    /// Gets a reference to the underlying value in this `NumberReader`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use byte_order::NumberReader;
    ///
    /// let reader = NumberReader::new(Cursor::new(vec![]));
    ///
    /// let reference = reader.get_ref();
    /// ```
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Gets a mutable reference to the underlying value in this `NumberReader`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use byte_order::NumberReader;
    ///
    /// let mut reader = NumberReader::new(Cursor::new(vec![]));
    ///
    /// let reference = reader.get_mut();
    /// ```
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Writes an unsigned 8-bit integer to the underlying writer.
    ///
    /// **Note:** Since this method reads a single byte, no byte order
    /// conversions are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use byte_order::NumberWriter;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut writer = NumberWriter::new(vec![]);
    ///     writer.write_u8(0x12u8)?;
    ///     assert_eq!(writer.into_inner(), vec![0x12]);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_u8(&mut self, n: u8) -> Result<()> {
        self.inner.write_all(&[n])
    }

    /// Writes an signed 8-bit integer to the underlying writer.
    ///
    /// **Note:** Since this method reads a single byte, no byte order
    /// conversions are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use byte_order::NumberWriter;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut writer = NumberWriter::new(vec![]);
    ///     writer.write_u8(0x12u8)?;
    ///     assert_eq!(writer.into_inner(), vec![0x12]);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_i8(&mut self, n: i8) -> Result<()> {
        self.write_u8(n as u8)
    }

    /// Writes an unsigned 16-bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use byte_order::{ByteOrder, NumberWriter};
    ///
    /// fn main() -> io::Result<()> {
    ///     let n = 0x1234u16;
    ///
    ///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
    ///     be_writer.write_u16(n)?;
    ///     assert_eq!(be_writer.into_inner(), vec![0x12, 0x34]);
    ///
    ///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
    ///     le_writer.write_u16(n)?;
    ///     assert_eq!(le_writer.into_inner(), vec![0x34, 0x12]);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_u16(&mut self, n: u16) -> Result<()> {
        let bytes = match self.order {
            ByteOrder::BE => n.to_be_bytes(),
            ByteOrder::LE => n.to_le_bytes(),
        };
        self.inner.write_all(&bytes)
    }

    /// Writes a signed 16-bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use byte_order::{ByteOrder, NumberWriter};
    ///
    /// fn main() -> io::Result<()> {
    ///     let n = 0x1234i16;
    ///
    ///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
    ///     be_writer.write_i16(n)?;
    ///     assert_eq!(be_writer.into_inner(), vec![0x12, 0x34]);
    ///
    ///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
    ///     le_writer.write_i16(n)?;
    ///     assert_eq!(le_writer.into_inner(), vec![0x34, 0x12]);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_i16(&mut self, n: i16) -> Result<()> {
        self.write_u16(n as u16)
    }

    /// Writes an unsigned 32-bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use byte_order::{ByteOrder, NumberWriter};
    ///
    /// fn main() -> io::Result<()> {
    ///     let n = 0x12345678u32;
    ///
    ///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
    ///     be_writer.write_u32(n)?;
    ///     assert_eq!(be_writer.into_inner(), vec![0x12, 0x34, 0x56, 0x78]);
    ///
    ///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
    ///     le_writer.write_u32(n)?;
    ///     assert_eq!(le_writer.into_inner(), vec![0x78, 0x56, 0x34, 0x12]);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_u32(&mut self, n: u32) -> Result<()> {
        let bytes = match self.order {
            ByteOrder::BE => n.to_be_bytes(),
            ByteOrder::LE => n.to_le_bytes(),
        };
        self.inner.write_all(&bytes)
    }

    /// Writes a signed 32-bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use byte_order::{ByteOrder, NumberWriter};
    ///
    /// fn main() -> io::Result<()> {
    ///     let n = 0x12345678i32;
    ///
    ///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
    ///     be_writer.write_i32(n)?;
    ///     assert_eq!(be_writer.into_inner(), vec![0x12, 0x34, 0x56, 0x78]);
    ///
    ///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
    ///     le_writer.write_i32(n)?;
    ///     assert_eq!(le_writer.into_inner(), vec![0x78, 0x56, 0x34, 0x12]);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_i32(&mut self, n: i32) -> Result<()> {
        self.write_u32(n as u32)
    }

    /// Writes an unsigned 64-bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use byte_order::{ByteOrder, NumberWriter};
    ///
    /// fn main() -> io::Result<()> {
    ///     let n = 0x1234567890123456u64;
    ///
    ///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
    ///     be_writer.write_u64(n)?;
    ///     assert_eq!(be_writer.into_inner(), vec![0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]);
    ///
    ///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
    ///     le_writer.write_u64(n)?;
    ///     assert_eq!(le_writer.into_inner(), vec![0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_u64(&mut self, n: u64) -> Result<()> {
        let bytes = match self.order {
            ByteOrder::BE => n.to_be_bytes(),
            ByteOrder::LE => n.to_le_bytes(),
        };
        self.inner.write_all(&bytes)
    }

    /// Writes a signed 64-bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use byte_order::{ByteOrder, NumberWriter};
    ///
    /// fn main() -> io::Result<()> {
    ///     let n = 0x1234567890123456i64;
    ///
    ///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
    ///     be_writer.write_i64(n)?;
    ///     assert_eq!(be_writer.into_inner(), vec![0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]);
    ///
    ///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
    ///     le_writer.write_i64(n)?;
    ///     assert_eq!(le_writer.into_inner(), vec![0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_i64(&mut self, n: i64) -> Result<()> {
        self.write_u64(n as u64)
    }

    /// Writes an unsigned 128-bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use byte_order::{ByteOrder, NumberWriter};
    ///
    /// fn main() -> io::Result<()> {
    ///     let n = 0x12345678901234567890123456789012u128;
    ///
    ///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
    ///     be_writer.write_u128(n)?;
    ///     assert_eq!(
    ///         be_writer.into_inner(),
    ///         vec![
    ///             0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56,
    ///             0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12,
    ///         ]
    ///     );
    ///
    ///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
    ///     le_writer.write_u128(n)?;
    ///     assert_eq!(
    ///         le_writer.into_inner(),
    ///         vec![
    ///             0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78,
    ///             0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12
    ///         ]
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_u128(&mut self, n: u128) -> Result<()> {
        let bytes = match self.order {
            ByteOrder::BE => n.to_be_bytes(),
            ByteOrder::LE => n.to_le_bytes(),
        };
        self.inner.write_all(&bytes)
    }

    /// Writes a signed 128-bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use byte_order::{ByteOrder, NumberWriter};
    ///
    /// fn main() -> io::Result<()> {
    ///     let n = 0x12345678901234567890123456789012i128;
    ///
    ///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
    ///     be_writer.write_i128(n)?;
    ///     assert_eq!(
    ///         be_writer.into_inner(),
    ///         vec![
    ///             0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56,
    ///             0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12,
    ///         ]
    ///     );
    ///
    ///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
    ///     le_writer.write_i128(n)?;
    ///     assert_eq!(
    ///         le_writer.into_inner(),
    ///         vec![
    ///             0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78,
    ///             0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12
    ///         ]
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_i128(&mut self, n: i128) -> Result<()> {
        self.write_u128(n as u128)
    }

    /// Writes a IEEE754 single-precision floating point number to the
    /// underlying writer.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberWriter};
    ///
    /// fn main() -> io::Result<()> {
    ///     let n = 12.5f32;
    ///
    ///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
    ///     be_writer.write_f32(n)?;
    ///     assert_eq!(be_writer.into_inner(), vec![0x41, 0x48, 0x00, 0x00]);
    ///
    ///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
    ///     le_writer.write_f32(n)?;
    ///     assert_eq!(le_writer.into_inner(), vec![0x00, 0x00, 0x48, 0x41]);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_f32(&mut self, n: f32) -> Result<()> {
        let bytes = match self.order {
            ByteOrder::BE => n.to_be_bytes(),
            ByteOrder::LE => n.to_le_bytes(),
        };
        self.inner.write_all(&bytes)
    }

    /// Writes a IEEE754 double-precision floating point number to the
    /// underlying writer.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberWriter};
    ///
    /// fn main() -> io::Result<()> {
    ///     let n = 12.5f64;
    ///
    ///     let mut be_writer = NumberWriter::with_order(ByteOrder::BE, vec![]);
    ///     be_writer.write_f64(n)?;
    ///     assert_eq!(be_writer.into_inner(), vec![0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    ///
    ///     let mut le_writer = NumberWriter::with_order(ByteOrder::LE, vec![]);
    ///     le_writer.write_f64(n)?;
    ///     assert_eq!(le_writer.into_inner(), vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40]);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Write::write_all`]: Write::write_all
    #[inline]
    pub fn write_f64(&mut self, n: f64) -> Result<()> {
        let bytes = match self.order {
            ByteOrder::BE => n.to_be_bytes(),
            ByteOrder::LE => n.to_le_bytes(),
        };
        self.inner.write_all(&bytes)
    }
}
