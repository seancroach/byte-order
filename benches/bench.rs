#![feature(test)]

extern crate test;

use byte_order::{ByteOrder, NumberReader, NumberWriter};
use std::io::{repeat, sink};
use test::{black_box, Bencher};

const N_ITER: usize = 100_000;

macro_rules! bench {
    ($name:ident, $ty:ty, $read:ident, $write:ident) => {
        mod $name {
            use super::*;

            #[bench]
            fn read_big_endian(b: &mut Bencher) {
                let mut reader = NumberReader::with_order(ByteOrder::BE, repeat(0xFF));
                b.iter(|| {
                    for _ in 0..N_ITER {
                        black_box(reader.$read().unwrap());
                    }
                })
            }

            #[bench]
            fn read_little_endian(b: &mut Bencher) {
                let mut reader = NumberReader::with_order(ByteOrder::LE, repeat(0xFF));
                b.iter(|| {
                    for _ in 0..N_ITER {
                        black_box(reader.$read().unwrap());
                    }
                })
            }

            #[bench]
            fn write_big_endian(b: &mut Bencher) {
                let mut writer = NumberWriter::with_order(ByteOrder::BE, sink());
                let n = <$ty>::MAX;
                b.iter(|| {
                    for _ in 0..N_ITER {
                        black_box(writer.$write(n).unwrap());
                    }
                })
            }

            #[bench]
            fn write_little_endian(b: &mut Bencher) {
                let mut writer = NumberWriter::with_order(ByteOrder::LE, sink());
                let n = <$ty>::MAX;
                b.iter(|| {
                    for _ in 0..N_ITER {
                        black_box(writer.$write(n).unwrap());
                    }
                })
            }
        }
    };
}

bench!(u8, u8, read_u8, write_u8);
bench!(i8, i8, read_i8, write_i8);
bench!(u16, u16, read_u16, write_u16);
bench!(i16, i16, read_i16, write_i16);
bench!(u32, u32, read_u32, write_u32);
bench!(i32, i32, read_i32, write_i32);
bench!(u64, u64, read_u64, write_u64);
bench!(i64, i64, read_i64, write_i64);
bench!(u128, u128, read_u128, write_u128);
bench!(i128, i128, read_i128, write_i128);

bench!(f32, f32, read_f32, write_f32);
bench!(f64, f64, read_f64, write_f64);
