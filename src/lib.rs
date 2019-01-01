//
// Copyright 2018 Red Hat, Inc.
//
// Author: Nathaniel McCallum <npmccallum@redhat.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Implements endianness encodings using the `codicon` traits.
//!
//! See the `codicon` crate for details.
//!
//! # Examples
//!
//! ```rust
//! extern crate codicon;
//! extern crate endicon;
//!
//! use endicon::Endianness;
//! use codicon::Encoder;
//!
//! let mut bytes = [0u8; 2];
//! let little = [1u8, 0u8];
//! let big = [0u8, 1u8];
//!
//! 1u16.encode(&mut bytes.as_mut(), Endianness::Little).unwrap();
//! assert_eq!(bytes, little);
//!
//! 1u16.encode(&mut bytes.as_mut(), Endianness::Big).unwrap();
//! assert_eq!(bytes, big);
//! ```

extern crate codicon;

use codicon::{Decoder, Encoder};

use std::mem::{size_of, transmute};
use std::io;

/// Endianness to use during encoding/decoding.
pub enum Endianness {
    /// Encode/decode using the CPU's native endianness.
    Native,
    
    /// Encode/decode using little endianness.
    Little,
    
    /// Encode/decode using big endianness.
    Big
}

trait FloatEndian<T> {
    fn to_le(self) -> T;
    fn to_be(self) -> T;
}

macro_rules! end_impl {
    () => ();

    ($t:ident:$i:ident $($rest:tt)*) => (
        impl Decoder<Endianness> for $t {
            fn decode<R: io::Read>(reader: &mut R, params: Endianness) -> io::Result<Self> {
                Ok($t::from_bits($i::decode(reader, params)?))
            }
        }

        impl Encoder<Endianness> for $t {
            fn encode<W: io::Write>(&self, writer: &mut W, params: Endianness) -> io::Result<()> {
                self.to_bits().encode(writer, params)
            }
        }

        impl FloatEndian<$i> for $t {
            fn to_le(self) -> $i {
                self.to_bits().to_le()
            }

            fn to_be(self) -> $i {
                self.to_bits().to_be()
            }
        }

        end_impl!(!$t);
        end_impl!($($rest)*);
    );

    ($t:ident $($rest:tt)*) => (
        impl Decoder<Endianness> for $t {
            fn decode<R: io::Read>(reader: &mut R, params: Endianness) -> io::Result<Self> {
                let mut bytes = [0u8; size_of::<$t>()];
                reader.read_exact(&mut bytes)?;

                let value = unsafe { transmute(bytes) };

                Ok(match params {
                    Endianness::Native => value,
                    Endianness::Little => $t::from_le(value),
                    Endianness::Big => $t::from_be(value),
                })
            }
        }

        impl Encoder<Endianness> for $t {
            fn encode<W: io::Write>(&self, writer: &mut W, params: Endianness) -> io::Result<()> {
                let v = match params {
                    Endianness::Native => *self,
                    Endianness::Little => self.to_le(),
                    Endianness::Big => self.to_be(),
                };

                let bytes: [u8; size_of::<Self>()] = unsafe { transmute(v) };
                writer.write_all(&bytes)?;
                Ok(())
            }
        }

        end_impl!(!$t);
        end_impl!($($rest)*);
    );

    (!$t:ident) => (
        #[cfg(test)]
        mod $t {
            mod ne {
                use std::mem::{size_of, transmute};
                use codicon::{Decoder, Encoder};
                use super::super::Endianness::*;

                const S: usize = size_of::<$t>();
                const V: $t = 1 as $t;

                #[test]
                fn enc() {
                    let e: [u8; S] = unsafe { transmute(V) };
                    let mut x = [0u8; S];
                    V.encode(&mut x.as_mut(), Native).unwrap();
                    assert_eq!(x, e);
                }

                #[test]
                fn dec() {
                    let e: [u8; S] = unsafe { transmute(V) };
                    let x = $t::decode(&mut e.as_ref(), Native).unwrap();
                    assert_eq!(x, V);
                }
            }

            mod le {
                use std::mem::{size_of, transmute};
                use codicon::{Decoder, Encoder};
                use super::super::*;

                const S: usize = size_of::<$t>();
                const V: $t = 1 as $t;

                #[test]
                fn enc() {
                    let e: [u8; S] = unsafe { transmute(V.to_le()) };
                    let mut x = [0u8; S];
                    V.encode(&mut x.as_mut(), Endianness::Little).unwrap();
                    assert_eq!(x, e);
                }

                #[test]
                fn dec() {
                    let e: [u8; S] = unsafe { transmute(V.to_le()) };
                    let x = $t::decode(&mut e.as_ref(), Endianness::Little).unwrap();
                    assert_eq!(x, V);
                }
            }

            mod be {
                use std::mem::{size_of, transmute};
                use codicon::{Decoder, Encoder};
                use super::super::*;

                const S: usize = size_of::<$t>();
                const V: $t = 1 as $t;

                #[test]
                fn enc() {
                    let e: [u8; S] = unsafe { transmute(V.to_be()) };
                    let mut x = [0u8; S];
                    V.encode(&mut x.as_mut(), Endianness::Big).unwrap();
                    assert_eq!(x, e);
                }

                #[test]
                fn dec() {
                    let e: [u8; S] = unsafe { transmute(V.to_be()) };
                    let x = $t::decode(&mut e.as_ref(), Endianness::Big).unwrap();
                    assert_eq!(x, V);
                }
            }
        }
    );
}

end_impl! {
    usize u128 u64 u32 u16 u8
    isize i128 i64 i32 i16 i8
    f64:u64 f32:u32
}
