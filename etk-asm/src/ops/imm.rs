use hex::ToHex;

use snafu::{Backtrace, Snafu};

use std::convert::TryFrom;
use std::fmt::{self, Debug};

/// An error that arises when converting an integer into an immediate.
#[derive(Snafu, Debug)]
#[snafu(visibility = "pub(super)")]
pub struct TryFromIntError {
    backtrace: Backtrace,
}

/// An error that arises when converting a slice into an immediate.
#[derive(Snafu, Debug)]
pub struct TryFromSliceError {
    backtrace: Backtrace,
}

impl From<std::convert::Infallible> for TryFromIntError {
    fn from(e: std::convert::Infallible) -> Self {
        match e {}
    }
}

/// An immediate value for push instructions.
#[derive(Clone, Eq, PartialEq)]
pub enum Imm<T> {
    /// A label argument.
    Label(String),

    /// A constant argument.
    Constant(T),
}

impl<T> From<&str> for Imm<T> {
    fn from(label: &str) -> Self {
        Imm::Label(label.to_owned())
    }
}

impl<T> From<String> for Imm<T> {
    fn from(label: String) -> Self {
        Imm::Label(label)
    }
}

macro_rules! impl_from {
    ($ii:literal;) => {
        impl From<[u8; $ii]> for Imm<[u8; $ii]> {
            fn from(konst: [u8; $ii]) -> Self {
                Imm::Constant(konst)
            }
        }
    };

    ($ii:literal; $ty:ty $(, $rest:ty)* $(,)*) => {
        impl From<$ty> for Imm<[u8; $ii]> {
            fn from(x: $ty) -> Self {
                let mut output = [0u8; $ii];
                let bytes = x.to_be_bytes();
                let start = $ii - bytes.len();
                (&mut output[start..$ii]).copy_from_slice(&bytes);
                Imm::Constant(output)
            }
        }

        impl_from!($ii; $($rest,)*);
    }
}

macro_rules! impl_try_from {
    ($ii:literal;) => {
    };

    ($ii:literal; $ty:ty $(, $rest:ty)* $(,)*) => {
        impl TryFrom<$ty> for Imm<[u8; $ii]> {
            type Error = TryFromIntError;

            fn try_from(x: $ty) -> Result<Self, Self::Error> {
                let max = <$ty>::pow(2, 8 * $ii);

                if x >= max {
                    return TryFromIntContext.fail();
                }

                let mut output = [0u8; $ii];
                let bytes = x.to_be_bytes();
                let start = std::mem::size_of::<$ty>() - $ii;
                output.copy_from_slice(&bytes[start..]);
                Ok(Imm::Constant(output))
            }
        }

        impl_try_from!($ii; $($rest,)*);
    }
}

macro_rules! impl_try_from_slice {
    ($ii:literal) => {
        impl TryFrom<&[u8]> for Imm<[u8; $ii]> {
            type Error = TryFromSliceError;

            fn try_from(x: &[u8]) -> Result<Self, Self::Error> {
                if x.len() > $ii {
                    return TryFromSliceContext.fail();
                }

                let mut output = [0u8; $ii];
                output.copy_from_slice(x);
                Ok(Imm::Constant(output))
            }
        }
    };
}

impl_from!(1; u8);
impl_from!(2; u8, u16);
impl_from!(3; u8, u16);
impl_from!(4; u8, u16, u32);
impl_from!(5; u8, u16, u32);
impl_from!(6; u8, u16, u32);
impl_from!(7; u8, u16, u32);
impl_from!(8; u8, u16, u32, u64);
impl_from!(9; u8, u16, u32, u64);
impl_from!(10; u8, u16, u32, u64);
impl_from!(11; u8, u16, u32, u64);
impl_from!(12; u8, u16, u32, u64);
impl_from!(13; u8, u16, u32, u64);
impl_from!(14; u8, u16, u32, u64);
impl_from!(15; u8, u16, u32, u64);
impl_from!(16; u8, u16, u32, u64, u128);
impl_from!(17; u8, u16, u32, u64, u128);
impl_from!(18; u8, u16, u32, u64, u128);
impl_from!(19; u8, u16, u32, u64, u128);
impl_from!(20; u8, u16, u32, u64, u128);
impl_from!(21; u8, u16, u32, u64, u128);
impl_from!(22; u8, u16, u32, u64, u128);
impl_from!(23; u8, u16, u32, u64, u128);
impl_from!(24; u8, u16, u32, u64, u128);
impl_from!(25; u8, u16, u32, u64, u128);
impl_from!(26; u8, u16, u32, u64, u128);
impl_from!(27; u8, u16, u32, u64, u128);
impl_from!(28; u8, u16, u32, u64, u128);
impl_from!(29; u8, u16, u32, u64, u128);
impl_from!(30; u8, u16, u32, u64, u128);
impl_from!(31; u8, u16, u32, u64, u128);
impl_from!(32; u8, u16, u32, u64, u128);

impl_try_from_slice!(1);
impl_try_from_slice!(2);
impl_try_from_slice!(3);
impl_try_from_slice!(4);
impl_try_from_slice!(5);
impl_try_from_slice!(6);
impl_try_from_slice!(7);
impl_try_from_slice!(8);
impl_try_from_slice!(9);
impl_try_from_slice!(10);
impl_try_from_slice!(11);
impl_try_from_slice!(12);
impl_try_from_slice!(13);
impl_try_from_slice!(14);
impl_try_from_slice!(15);
impl_try_from_slice!(16);
impl_try_from_slice!(17);
impl_try_from_slice!(18);
impl_try_from_slice!(19);
impl_try_from_slice!(20);
impl_try_from_slice!(21);
impl_try_from_slice!(22);
impl_try_from_slice!(23);
impl_try_from_slice!(24);
impl_try_from_slice!(25);
impl_try_from_slice!(26);
impl_try_from_slice!(27);
impl_try_from_slice!(28);
impl_try_from_slice!(29);
impl_try_from_slice!(30);
impl_try_from_slice!(31);
impl_try_from_slice!(32);

impl_try_from!(1; u16, u32, u64, u128);
impl_try_from!(2; u32, u64, u128);
impl_try_from!(3; u32, u64, u128);
impl_try_from!(4; u64, u128);
impl_try_from!(5; u64, u128);
impl_try_from!(6; u64, u128);
impl_try_from!(7; u64, u128);
impl_try_from!(8; u128);
impl_try_from!(9; u128);
impl_try_from!(10; u128);
impl_try_from!(11; u128);
impl_try_from!(12; u128);
impl_try_from!(13; u128);
impl_try_from!(14; u128);
impl_try_from!(15; u128);

impl<T> Debug for Imm<T>
where
    T: ToHex,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Imm::Label(s) => write!(f, r#"Imm::Label("{}")"#, s),
            Imm::Constant(c) => write!(f, "Imm::Constant(0x{})", c.encode_hex::<String>()),
        }
    }
}

impl<T> fmt::Display for Imm<T>
where
    T: ToHex,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Imm::Label(s) => write!(f, ":{}", s),
            Imm::Constant(c) => write!(f, "0x{}", c.encode_hex::<String>()),
        }
    }
}

#[doc(hidden)]
pub trait Immediate<const N: usize>: Debug + Clone + Eq + PartialEq {
    fn extra_len() -> usize {
        N
    }
}

impl<T, const N: usize> Immediate<N> for [T; N] where T: Debug + Clone + Eq + PartialEq {}
impl<const N: usize> Immediate<N> for Imm<[u8; N]> {}
impl<const N: usize> Immediate<N> for () {}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use hex_literal::hex;

    use super::*;

    #[test]
    fn imm4_from_array() {
        let imm = Imm::from(hex!("95ea7b30"));
        assert_matches!(imm, Imm::Constant([0x95, 0xea, 0x7b, 0x30]));
    }
}
