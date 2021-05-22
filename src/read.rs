use std::io::{self, Read};
use std::mem;

use crate::order::ByteOrder;

/// A `NumberReader` wraps a [reader] and provides methods for reading numbers.
///
/// Unlike many libraries, which take byte order as a parameter per operation, a
/// `NumberReader` takes byte order as a parameter upon initialization.
///
/// # Examples
///
/// We can create a new `NumberReader` using the target endianness using
/// [`NumberReader::new`]:
///
/// ```
/// use std::io::Cursor;
/// use byte_order::NumberReader;
///
/// let src = Cursor::new(vec![]);
/// let mut reader = NumberReader::new(src);
/// ```
///
/// Or, to read numbers with a certain endianness, we can create `NumberReader`
/// structures using [`NumberReader::with_order`]:
///
/// ```
/// use std::io::Cursor;
/// use byte_order::{ByteOrder, NumberReader};
///
/// let src = Cursor::new(vec![]);
/// let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
/// let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
/// let mut ne_reader = NumberReader::with_order(ByteOrder::NE, src.clone());
/// ```
///
/// To read numbers from the underlying [reader], use any of the `read_*`
/// methods that are provided:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::io::Cursor;
/// use byte_order::{ByteOrder, NumberReader};
///
/// let src = Cursor::new(vec![0xA1, 0xB2]);
///
/// let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
/// assert_eq!(0xA1B2, be_reader.read_u16()?);
///
/// let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
/// assert_eq!(0xB2A1, le_reader.read_u16()?);
///
/// let mut ne_reader = NumberReader::with_order(ByteOrder::NE, src.clone());
/// assert_eq!(
///     if cfg!(target_endian = "big") {
///         0xA1B2
///     } else {
///         0xB2A1
///     },
///     ne_reader.read_u16()?
/// );
/// # Ok(())
/// # }
/// ```
///
/// [reader]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`NumberReader::new`]: NumberReader::new
/// [`NumberReader::with_order`]: NumberReader::with_order
pub struct NumberReader<R: Read> {
    inner: R,
    order: ByteOrder,
}

impl<R: Read> NumberReader<R> {
    /// Creates a new `NumberReader` by wrapping the given [reader].
    ///
    /// Since the target platform's native endianness is used, portable code
    /// should use [`with_order`], as appropriate, instead.
    ///
    /// **Note:** If you want to explicitly mark in your code that the target
    /// platform's endianness is desired, consider creating the `NumberReader`
    /// using [`with_order`] and [`ByteOrder::NE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::NumberReader;
    ///
    /// fn main() -> io::Result<()> {
    ///     let src = Cursor::new(vec![0xA1, 0xB2]);
    ///     let mut reader = NumberReader::new(src);
    ///
    ///     assert_eq!(
    ///         reader.read_u16()?,
    ///         if cfg!(target_endian = "big") {
    ///             0xA1B2
    ///         } else {
    ///             0xB2A1
    ///         }
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [reader]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`with_order`]: NumberReader::with_order
    /// [`ByteOrder::NE`]: ByteOrder::NE
    #[inline]
    pub fn new(src: R) -> NumberReader<R> {
        NumberReader::with_order(ByteOrder::NE, src)
    }

    /// Creates a new `NumberReader` by wrapping the given [reader] with the
    /// specified byte order.
    ///
    /// Use either [`ByteOrder::BE`] for big-endian byte ordering,
    /// [`ByteOrder::LE`] for little-endian byte ordering, or [`ByteOrder::NE`]
    /// to explicitly use the target platform's endianness.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let src = Cursor::new(vec![0xA1, 0xB2]);
    ///
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
    ///     assert_eq!(0xA1B2, be_reader.read_u16()?);
    ///
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
    ///     assert_eq!(0xB2A1, le_reader.read_u16()?);
    ///
    ///     let mut ne_reader = NumberReader::with_order(ByteOrder::NE, src.clone());
    ///     assert_eq!(
    ///         if cfg!(target_endian = "big") {
    ///             0xA1B2
    ///         } else {
    ///             0xB2A1
    ///         },
    ///         ne_reader.read_u16()?
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [reader]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`ByteOrder::BE`]: ByteOrder::BE
    /// [`ByteOrder::LE`]: ByteOrder::LE
    /// [`ByteOrder::NE`]: ByteOrder::NE
    #[inline]
    pub fn with_order(order: ByteOrder, src: R) -> NumberReader<R> {
        NumberReader { inner: src, order }
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
    pub fn into_inner(self) -> R {
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
    pub fn get_ref(&self) -> &R {
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
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Reads an unsigned 8-bit integer from the underlying reader.
    ///
    /// **Note:** Since this method reads a single byte, no byte order
    /// conversions are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::NumberReader;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut reader = NumberReader::new(Cursor::new(vec![0x12]));
    ///     assert_eq!(0x12u8, reader.read_u8()?);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; mem::size_of::<u8>()];
        self.inner.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Reads a signed 8-bit integer from the underlying reader.
    ///
    /// **Note:** Since this method reads a single byte, no byte order
    /// conversions are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::NumberReader;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut reader = NumberReader::new(Cursor::new(vec![0x12]));
    ///     assert_eq!(0x12i8, reader.read_i8()?);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_i8(&mut self) -> io::Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    /// Reads an unsigned 16-bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let src = Cursor::new(vec![0x12, 0x34]);
    ///
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
    ///     assert_eq!(0x1234u16, be_reader.read_u16()?);
    ///
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
    ///     assert_eq!(0x3412u16, le_reader.read_u16()?);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0; mem::size_of::<u16>()];
        self.inner.read_exact(&mut buf)?;
        Ok(if let ByteOrder::LE = self.order {
            u16::from_le_bytes(buf)
        } else {
            u16::from_be_bytes(buf)
        })
    }

    /// Reads a signed 16-bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let src = Cursor::new(vec![0x12, 0x34]);
    ///
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
    ///     assert_eq!(0x1234i16, be_reader.read_i16()?);
    ///
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
    ///     assert_eq!(0x3412i16, le_reader.read_i16()?);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_i16(&mut self) -> io::Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    /// Reads an unsigned 32-bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let src = Cursor::new(vec![0x12, 0x34, 0x56, 0x78]);
    ///
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
    ///     assert_eq!(0x12345678u32, be_reader.read_u32()?);
    ///
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
    ///     assert_eq!(0x78563412u32, le_reader.read_u32()?);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0; mem::size_of::<u32>()];
        self.inner.read_exact(&mut buf)?;
        Ok(if let ByteOrder::LE = self.order {
            u32::from_le_bytes(buf)
        } else {
            u32::from_be_bytes(buf)
        })
    }

    /// Reads a signed 32-bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let src = Cursor::new(vec![0x12, 0x34, 0x56, 0x78]);
    ///
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
    ///     assert_eq!(0x12345678i32, be_reader.read_i32()?);
    ///
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
    ///     assert_eq!(0x78563412i32, le_reader.read_i32()?);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_i32(&mut self) -> io::Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    /// Reads an unsigned 64-bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let src = Cursor::new(vec![0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]);
    ///
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
    ///     assert_eq!(0x1234567890123456u64, be_reader.read_u64()?);
    ///
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
    ///     assert_eq!(0x5634129078563412u64, le_reader.read_u64()?);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0; mem::size_of::<u64>()];
        self.inner.read_exact(&mut buf)?;
        Ok(if let ByteOrder::LE = self.order {
            u64::from_le_bytes(buf)
        } else {
            u64::from_be_bytes(buf)
        })
    }

    /// Reads a signed 64-bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let src = Cursor::new(vec![0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]);
    ///
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
    ///     assert_eq!(0x1234567890123456i64, be_reader.read_i64()?);
    ///
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
    ///     assert_eq!(0x5634129078563412i64, le_reader.read_i64()?);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_i64(&mut self) -> io::Result<i64> {
        Ok(self.read_u64()? as i64)
    }

    /// Reads an unsigned 128-bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let src = Cursor::new(vec![
    ///         0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56,
    ///         0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12,
    ///     ]);
    ///
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
    ///     assert_eq!(0x12345678901234567890123456789012u128, be_reader.read_u128()?);
    ///
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
    ///     assert_eq!(0x12907856341290785634129078563412u128, le_reader.read_u128()?);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_u128(&mut self) -> io::Result<u128> {
        let mut buf = [0; mem::size_of::<u128>()];
        self.inner.read_exact(&mut buf)?;
        Ok(if let ByteOrder::LE = self.order {
            u128::from_le_bytes(buf)
        } else {
            u128::from_be_bytes(buf)
        })
    }

    /// Reads a signed 128-bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let src = Cursor::new(vec![
    ///         0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56,
    ///         0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12,
    ///     ]);
    ///
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, src.clone());
    ///     assert_eq!(0x12345678901234567890123456789012u128, be_reader.read_u128()?);
    ///
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, src.clone());
    ///     assert_eq!(0x12907856341290785634129078563412u128, le_reader.read_u128()?);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_i128(&mut self) -> io::Result<i128> {
        Ok(self.read_u128()? as i128)
    }

    /// Reads a IEEE754 single-precision floating point number from the
    /// underlying reader.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let be_src = Cursor::new(vec![0x41, 0x48, 0x00, 0x00]);
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, be_src);
    ///     assert_eq!(12.5f32, be_reader.read_f32()?);
    ///
    ///     let le_src = Cursor::new(vec![0x00, 0x00, 0x48, 0x41]);
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, le_src);
    ///     assert_eq!(12.5f32, le_reader.read_f32()?);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_f32(&mut self) -> io::Result<f32> {
        let mut buf = [0; mem::size_of::<f32>()];
        self.inner.read_exact(&mut buf)?;
        Ok(if let ByteOrder::LE = self.order {
            f32::from_le_bytes(buf)
        } else {
            f32::from_be_bytes(buf)
        })
    }

    /// Reads a IEEE754 double-precision floating point number from the
    /// underlying reader.
    ///
    /// # Errors
    ///
    /// This method propagates any error recieved from the internal call to
    /// [`Read::read_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Cursor};
    /// use byte_order::{ByteOrder, NumberReader};
    ///
    /// fn main() -> io::Result<()> {
    ///     let be_src = Cursor::new(vec![0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    ///     let mut be_reader = NumberReader::with_order(ByteOrder::BE, be_src);
    ///     assert_eq!(12.5f64, be_reader.read_f64()?);
    ///
    ///     let le_src = Cursor::new(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40]);
    ///     let mut le_reader = NumberReader::with_order(ByteOrder::LE, le_src);
    ///     assert_eq!(12.5f64, le_reader.read_f64()?);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Read::read_exact`]: Read::read_exact
    #[inline]
    pub fn read_f64(&mut self) -> io::Result<f64> {
        let mut buf = [0; mem::size_of::<f64>()];
        self.inner.read_exact(&mut buf)?;
        Ok(if let ByteOrder::LE = self.order {
            f64::from_le_bytes(buf)
        } else {
            f64::from_be_bytes(buf)
        })
    }
}

impl<R: Read> Read for NumberReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}
