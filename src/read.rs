use std::io::{Read, Result};
use std::mem::size_of;

use crate::Endianness;

macro_rules! read_method {
    ($name:ident, $Ty:ty, $value:expr, $be_slice:expr, $le_slice:expr) => {
        doc!(
            concat!("Reads a [`", stringify!($Ty), "`] value from the")
            "underlying reader."
            ""
            "# Errors"
            ""
            "This method returns the same errors as [`Read::read_exact`]."
            ""
            "# Examples"
            ""
            "```"
            "use byte_order::{Endianness, EndianReader};"
            ""
            concat!("let be_bytes: &[u8] = ", stringify!($be_slice), ";")
            "let mut be_reader = EndianReader::with_order(Endianness::BE, be_bytes);"
            concat!("let value = be_reader.", stringify!($name), "().unwrap();")
            ""
            concat!("assert_eq!(value, ", stringify!($value), stringify!($Ty), ");")
            "```"
            ""
            "```"
            "use byte_order::{Endianness, EndianReader};"
            ""
            concat!("let le_bytes: &[u8] = ", stringify!($le_slice), ";")
            "let mut le_reader = EndianReader::with_order(Endianness::LE, le_bytes);"
            concat!("let value = le_reader.", stringify!($name), "().unwrap();")
            ""
            concat!("assert_eq!(value, ", stringify!($value), stringify!($Ty), ");")
            "```";
            pub fn $name(&mut self) -> Result<$Ty> {
                let mut bytes = [0; size_of::<$Ty>()];
                self.inner.read_exact(&mut bytes)?;
                Ok(match self.order {
                    Endianness::BE => <$Ty>::from_be_bytes(bytes),
                    Endianness::LE => <$Ty>::from_le_bytes(bytes),
                })
            }
        );
    };
}

macro_rules! read_method_pair {
    (
        $unsigned_name:ident,
        $U:ty,
        $signed_name:ident,
        $S:ty,
        $value:expr,
        $be_vec: expr,
        $le_vec:expr
    ) => {
        read_method!($unsigned_name, $U, $value, $be_vec, $le_vec);
        read_method!($signed_name, $S, $value, $be_vec, $le_vec);
    };
}

pub struct EndianReader<R: Read> {
    inner: R,
    order: Endianness,
}

impl<R: Read> EndianReader<R> {
    /// Creates a new byte reader wrapping the provided reader. Endianness is
    /// derived from native byte order.
    ///
    /// Since the target platform's endianness is used, portable code should use
    /// [`EndianReader::with_order`], as appropriate.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use byte_order::EndianReader;
    ///
    /// let rdr = EndianReader::new(Cursor::new(vec![]));
    /// ```
    #[inline]
    pub fn new(reader: R) -> EndianReader<R> {
        let order = if cfg!(target_endian = "big") {
            Endianness::BE
        } else {
            Endianness::LE
        };
        EndianReader::with_order(order, reader)
    }

    /// Creates a new endian reader wrapping the provided reader. An order is
    /// also required, either [`Endianness::BE`] for big-endian or
    /// [`Endianness::LE`] for little-endian byte ordering.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use byte_order::{Endianness, EndianReader};
    ///
    /// let be_reader = EndianReader::with_order(Endianness::BE, Cursor::new(vec![]));
    /// let le_reader = EndianReader::with_order(Endianness::LE, Cursor::new(vec![]));
    /// ```
    #[inline]
    pub fn with_order(order: Endianness, reader: R) -> EndianReader<R> {
        EndianReader {
            inner: reader,
            order,
        }
    }

    /// Consumes this byte reader, returning the underlying reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use byte_order::EndianReader;
    ///
    /// let reader = EndianReader::new(Cursor::new(vec![]));
    ///
    /// let cursor = reader.into_inner();
    /// ```
    #[inline]
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Gets a reference to the underlying reader of this byte reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use byte_order::EndianReader;
    ///
    /// let reader = EndianReader::new(Cursor::new(vec![]));
    ///
    /// let cursor_reference = reader.get_ref();
    /// ```
    #[inline]
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader of this byte reader.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying reader as it may corrupt reading and writing numbers.
    ///
    /// ```
    /// use std::io::Cursor;
    /// use byte_order::EndianReader;
    ///
    /// let mut reader = EndianReader::new(Cursor::new(vec![]));
    ///
    /// let cursor_reference = reader.get_mut();
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    read_method_pair!(read_u8, u8, read_i8, i8, 0x12, &[0x12], &[0x12]);

    read_method_pair!(
        read_u16,
        u16,
        read_i16,
        i16,
        0x1234,
        &[0x12, 0x34],
        &[0x34, 0x12]
    );

    read_method_pair!(
        read_u32,
        u32,
        read_i32,
        i32,
        0x12345678,
        &[0x12, 0x34, 0x56, 0x78],
        &[0x78, 0x56, 0x34, 0x12]
    );

    read_method_pair!(
        read_u64,
        u64,
        read_i64,
        i64,
        0x1234567890123456,
        &[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56],
        &[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]
    );

    read_method_pair!(
        read_u128,
        u128,
        read_i128,
        i128,
        0x12345678901234567890123456789012,
        &[
            0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78,
            0x90, 0x12
        ],
        &[
            0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56,
            0x34, 0x12
        ]
    );
}
