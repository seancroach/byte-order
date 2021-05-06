use std::io::{Result, Write};

use crate::Endianness;

macro_rules! write_method {
    ($name:ident, $Ty:ty, $value:expr, $be_vec:expr, $le_vec:expr) => {
        doc!(
            concat!("Writes a [`", stringify!($Ty), "`] value to the")
            "underlying writer."
            ""
            "# Errors"
            ""
            "This method returns the same errors as [`Write::write_all`]."
            ""
            "# Examples"
            ""
            "```"
            "use byte_order::{Endianness, EndianWriter};"
            ""
            "let mut be_writer = EndianWriter::with_order(Endianness::BE, Vec::new());"
            concat!("be_writer.", stringify!($name), "(", stringify!($value), stringify!($Ty), ").unwrap();")
            ""
            concat!("assert_eq!(be_writer.into_inner(), ", stringify!($be_vec), ");")
            "```"
            ""
            "```"
            "use byte_order::{Endianness, EndianWriter};"
            ""
            "let mut le_writer = EndianWriter::with_order(Endianness::LE, Vec::new());"
            concat!("le_writer.", stringify!($name), "(", stringify!($value), stringify!($Ty), ").unwrap();")
            ""
            concat!("assert_eq!(le_writer.into_inner(), ", stringify!($le_vec), ");")
            "```";
            #[inline]
            pub fn $name(&mut self, value: $Ty) -> Result<()> {
                let buf = match self.order {
                    Endianness::BE => value.to_be_bytes(),
                    Endianness::LE => value.to_le_bytes(),
                };
                self.inner.write_all(&buf)
            }
        );
    };
}

macro_rules! write_method_pair {
    (
        $unsigned_name:ident,
        $U:ty,
        $signed_name:ident,
        $S:ty,
        $value:expr,
        $be_vec: expr,
        $le_vec:expr
    ) => {
        write_method!($unsigned_name, $U, $value, $be_vec, $le_vec);
        write_method!($signed_name, $S, $value, $be_vec, $le_vec);
    };
}

/// An `EndianWriter` wraps some [writer](Write) and provides it methods to
/// write integers in some defined endianness.
pub struct EndianWriter<W: Write> {
    inner: W,
    order: Endianness,
}

impl<W: Write> EndianWriter<W> {
    /// Creates a new byte writer wrapping the provided writer. Endianness is
    /// derived from native byte order.
    ///
    /// Since the target platform's endianness is used, portable code should use
    /// [`EndianWriter::with_order`], as appropriate.
    ///
    /// # Examples
    ///
    /// ```
    /// use byte_order::EndianWriter;
    ///
    /// let writer = EndianWriter::new(Vec::new());
    /// ```
    #[inline]
    pub fn new(writer: W) -> EndianWriter<W> {
        let order = if cfg!(target_endian = "big") {
            Endianness::BE
        } else {
            Endianness::LE
        };
        EndianWriter::with_order(order, writer)
    }

    /// Creates a new endian writer wrapping the provided writer. An order is
    /// also required, either [`Endianness::BE`] for big-endian or
    /// [`Endianness::LE`] for little-endian byte ordering.
    ///
    /// # Examples
    ///
    /// ```
    /// use byte_order::{Endianness, EndianWriter};
    ///
    /// let be_writer = EndianWriter::with_order(Endianness::BE, Vec::new());
    /// let le_writer = EndianWriter::with_order(Endianness::LE, Vec::new());
    /// ```
    #[inline]
    pub fn with_order(order: Endianness, writer: W) -> EndianWriter<W> {
        EndianWriter {
            inner: writer,
            order,
        }
    }

    /// Consumes this byte writer, returning the underlying writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use byte_order::EndianWriter;
    ///
    /// let writer = EndianWriter::new(Vec::new());
    ///
    /// let vec = writer.into_inner();
    /// ```
    #[inline]
    pub fn into_inner(self) -> W {
        self.inner
    }

    /// Gets a reference to the underlying writer of this byte writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use byte_order::EndianWriter;
    ///
    /// let writer = EndianWriter::new(Vec::new());
    ///
    /// let vec_reference = writer.get_ref();
    /// ```
    #[inline]
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Gets a mutable reference to the underlying writer of this byte writer.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying writer as it may corrupt reading and writing numbers.
    ///
    /// ```
    /// use byte_order::EndianWriter;
    ///
    /// let mut writer = EndianWriter::new(Vec::new());
    ///
    /// let vec_reference = writer.get_mut();
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    write_method_pair!(write_u8, u8, write_i8, i8, 0x12, vec![0x12], vec![0x12]);

    write_method_pair!(
        write_u16,
        u16,
        write_i16,
        i16,
        0x1234,
        vec![0x12, 0x34],
        vec![0x34, 0x12]
    );

    write_method_pair!(
        write_u32,
        u32,
        write_i32,
        i32,
        0x12345678,
        vec![0x12, 0x34, 0x56, 0x78],
        vec![0x78, 0x56, 0x34, 0x12]
    );

    write_method_pair!(
        write_u64,
        u64,
        write_i64,
        i64,
        0x1234567890123456,
        vec![0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56],
        vec![0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]
    );

    write_method_pair!(
        write_u128,
        u128,
        write_i128,
        i128,
        0x12345678901234567890123456789012,
        vec![
            0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78,
            0x90, 0x12
        ],
        vec![
            0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56,
            0x34, 0x12
        ]
    );
}

impl<W: Write> Write for EndianWriter<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
