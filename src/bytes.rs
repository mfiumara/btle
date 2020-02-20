use core::convert::TryInto;
use core::ops;
use std::ops::RangeFull;

#[derive(Copy, Clone)]
pub enum Endian {
    Big,
    Little,
}
impl Endian {
    #[cfg(target_endian = "big")]
    const fn _native() -> Endian {
        Endian::Big
    }

    #[cfg(target_endian = "little")]
    const fn _native() -> Endian {
        Endian::Little
    }
    /// Returns the target platform `Endian`. This is a NOP const fn.
    pub const fn native() -> Endian {
        Self::_native()
    }
    pub const NATIVE: Endian = Self::native();
}
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum BufError {
    /// The given index/`usize` is out of range. (overflow, underflow)
    OutOfRange(usize),
    /// The given index/`usize` is invalid. (misaligned, non-sensible)
    InvalidIndex(usize),
    /// The bytes at positive/`usize` are invalid. Used if its possible to pass a 'bad' sequence of
    /// bytes values. (Ex: Trying to turn a byte that isn't a 0 or 1 into a `bool`).
    BadBytes(usize),
    /// Input is completely invalid. Used when unable to pinpoint an index where the bad bytes are.
    InvalidInput,
}
pub trait ToFromBytesEndian: Sized {
    type AsBytesType: AsRef<[u8]>;

    #[must_use]
    fn byte_size() -> usize {
        core::mem::size_of::<Self::AsBytesType>()
    }

    #[must_use]
    fn to_bytes_le(&self) -> Self::AsBytesType;

    #[must_use]
    fn to_bytes_be(&self) -> Self::AsBytesType;

    #[must_use]
    fn to_bytes_ne(&self) -> Self::AsBytesType {
        if cfg!(target_endian = "big") {
            self.to_bytes_be()
        } else {
            self.to_bytes_le()
        }
    }
    #[must_use]
    fn from_bytes_le(bytes: &[u8]) -> Option<Self>;

    #[must_use]
    fn from_bytes_be(bytes: &[u8]) -> Option<Self>;

    #[must_use]
    fn from_bytes_ne(bytes: &[u8]) -> Option<Self> {
        if cfg!(target_endian = "big") {
            Self::from_bytes_be(bytes)
        } else {
            Self::from_bytes_le(bytes)
        }
    }
    #[must_use]
    fn to_bytes_endian(&self, endian: Option<Endian>) -> Self::AsBytesType {
        match endian {
            Some(Endian::Big) => self.to_bytes_be(),
            Some(Endian::Little) => self.to_bytes_le(),
            None => self.to_bytes_ne(),
        }
    }
    #[must_use]
    fn from_bytes_endian(bytes: &[u8], endian: Option<Endian>) -> Option<Self> {
        match endian {
            Some(Endian::Big) => Self::from_bytes_be(bytes),
            Some(Endian::Little) => Self::from_bytes_le(bytes),
            None => Self::from_bytes_ne(bytes),
        }
    }
}
/// Implement ToFromEndian for all primitive types (see beneath)
macro_rules! implement_to_from_bytes {
    ( $( $t:ty ), *) => {
        $(
            impl ToFromBytesEndian for $t {
    type AsBytesType = [u8; core::mem::size_of::<Self>()];

    #[inline]
    #[must_use]
    fn byte_size() -> usize {
        core::mem::size_of::<Self>()
    }

    #[inline]
    #[must_use]
    fn to_bytes_le(&self) -> Self::AsBytesType {
        self.to_le_bytes()
    }

    #[inline]
    #[must_use]
    fn to_bytes_be(&self) -> Self::AsBytesType {
        self.to_be_bytes()
    }

    #[inline]
    #[must_use]
    fn to_bytes_ne(&self) -> Self::AsBytesType {
        self.to_ne_bytes()
    }

    #[inline]
    #[must_use]
    fn from_bytes_le(bytes: &[u8]) -> Option<Self> {
        Some(Self::from_le_bytes(bytes.try_into().ok()?))
    }

    #[inline]
    #[must_use]
    fn from_bytes_be(bytes: &[u8]) -> Option<Self> {
        Some(Self::from_be_bytes(bytes.try_into().ok()?))
    }

    #[inline]
    #[must_use]
    fn from_bytes_ne(bytes: &[u8]) -> Option<Self> {
        Some(Self::from_ne_bytes(bytes.try_into().ok()?))
    }
}
        )*
    }
}
implement_to_from_bytes!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
impl ToFromBytesEndian for bool {
    type AsBytesType = [u8; 1];

    #[inline]
    fn to_bytes_le(&self) -> Self::AsBytesType {
        self.to_bytes_ne()
    }

    #[inline]
    fn to_bytes_be(&self) -> Self::AsBytesType {
        self.to_bytes_ne()
    }

    #[inline]
    fn to_bytes_ne(&self) -> Self::AsBytesType {
        [u8::from(*self)]
    }

    #[inline]
    fn from_bytes_le(bytes: &[u8]) -> Option<Self> {
        Self::from_bytes_ne(bytes)
    }
    #[inline]
    fn from_bytes_be(bytes: &[u8]) -> Option<Self> {
        Self::from_bytes_ne(bytes)
    }
    /// # Example
    /// ```
    /// use btle::bytes::ToFromBytesEndian;
    /// assert_eq!(bool::from_bytes_ne(&[0]), Some(false));
    /// assert_eq!(bool::from_bytes_ne(&[1]), Some(true));
    /// assert_eq!(bool::from_bytes_ne(&[2]), None);
    /// ```
    #[inline]
    fn from_bytes_ne(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == 1 {
            match bytes[0] {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        } else {
            None
        }
    }
}

/// Static byte buffer. `StaticBuf<[u8; 16]>` can store a `[u8]` array from 0-16 bytes for example.
/// Unlike other static buffers, this does NOT reallocate if you out grow the internal buffer. If
/// you try to request more bytes than its able to store, it will panic.  
#[derive(Copy, Clone, Default)]
pub struct StaticBuf<ArrayBuf: AsRef<[u8]> + Default + Copy> {
    buf: ArrayBuf,
    len: usize,
}
impl<ArrayBuf: AsRef<[u8]> + Default + Copy> StaticBuf<ArrayBuf> {
    /// Returns the maximum size the `StaticBuf` can hold.
    /// # Examples
    /// ```
    /// use btle::bytes::StaticBuf;
    /// assert_eq!(StaticBuf::<[u8; 10]>::max_size(), 10);
    /// assert_eq!(StaticBuf::<[u8; 23]>::max_size(), 23);
    /// ```
    pub fn max_size() -> usize {
        ArrayBuf::default().as_ref().len()
    }

    /// Resizes the `StaticBuf` by settings `self.len` to `new_size` if `new_size <= Self::max_size()`.
    /// This is only a single variable change and WILL NOT zero or change any of the buffers bytes.
    /// # Examples
    /// ```
    /// use btle::bytes::{StaticBuf, Storage};
    /// let mut buf = StaticBuf::<[u8; 10]>::with_size(10);
    /// assert_eq!(buf.len(), 10);
    /// assert_eq!(buf[9], 0);
    /// buf[9] = 0xFF;
    /// buf.resize(1);
    /// assert_eq!(buf.len(), 1);
    /// buf.resize(10);
    /// assert_eq!(buf[9], 0xFF);
    /// ```
    pub fn resize(&mut self, new_size: usize) {
        assert!(
            new_size <= Self::max_size(),
            "requested size {} bigger than static buf size {}",
            new_size,
            Self::max_size()
        );
        self.len = new_size;
    }
}
impl<ArrayBuf: AsRef<[u8]> + Default + Copy> AsRef<[u8]> for StaticBuf<ArrayBuf> {
    fn as_ref(&self) -> &[u8] {
        &self.buf.as_ref()[..self.len]
    }
}
impl<ArrayBuf: AsRef<[u8]> + AsMut<[u8]> + Default + Copy> AsMut<[u8]> for StaticBuf<ArrayBuf> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.buf.as_mut()[..self.len]
    }
}
impl<ArrayBuf: AsRef<[u8]> + Default + Copy> ops::Index<ops::RangeFull> for StaticBuf<ArrayBuf> {
    type Output = [u8];

    fn index(&self, _index: RangeFull) -> &Self::Output {
        self.as_ref()
    }
}
impl<ArrayBuf: AsRef<[u8]> + AsMut<[u8]> + Default + Copy> ops::IndexMut<ops::RangeFull>
    for StaticBuf<ArrayBuf>
{
    fn index_mut(&mut self, _index: RangeFull) -> &mut Self::Output {
        self.as_mut()
    }
}
impl<ArrayBuf: AsRef<[u8]> + Default + Copy> ops::Index<usize> for StaticBuf<ArrayBuf> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl<ArrayBuf: AsRef<[u8]> + AsMut<[u8]> + Default + Copy> ops::IndexMut<usize>
    for StaticBuf<ArrayBuf>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}
impl<ArrayBuf: AsRef<[u8]> + Default + Copy> Storage for StaticBuf<ArrayBuf> {
    fn with_size(size: usize) -> Self
    where
        Self: Sized,
    {
        assert!(
            size <= Self::max_size(),
            "requested size {} bigger than static buf size {}",
            size,
            Self::max_size()
        );
        Self {
            buf: ArrayBuf::default(),
            len: size,
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

/// Objects that store and own bytes (`Box<[u8]>`, `Vec<u8>`, `StaticBuf<[u8; 32]>`, etc).
/// This allows for generic byte storage types for byte buffers.
pub trait Storage: AsRef<[u8]> {
    fn with_size(size: usize) -> Self
    where
        Self: Sized;
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}
impl Storage for Vec<u8> {
    fn with_size(size: usize) -> Self
    where
        Self: Sized,
    {
        vec![0; size]
    }
    fn len(&self) -> usize {
        <Vec<u8>>::len(self)
    }
}
impl Storage for Box<[u8]> {
    fn with_size(size: usize) -> Self
    where
        Self: Sized,
    {
        Vec::with_size(size).into_boxed_slice()
    }
}
