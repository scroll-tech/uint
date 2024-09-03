//! Support for the [`rkyv`](https://crates.io/crates/rkyv) crate.

#![cfg(feature = "rkyv")]
#![cfg_attr(docsrs, doc(cfg(feature = "rkyv")))]

use std::convert::Infallible;
use crate::Uint;
use rkyv::{Archive, Archived, CheckBytes, Deserialize, Fallible, Serialize};

impl<const BITS: usize, const LIMBS: usize> Archive for Uint<BITS, LIMBS> {
    type Archived = Uint<BITS, LIMBS>;
    type Resolver = <[u64; LIMBS] as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.limbs.resolve(pos, resolver, out as *mut [u64; LIMBS]);
    }
}

impl<C: ?Sized, const BITS: usize, const LIMBS: usize> CheckBytes<C> for Uint<BITS, LIMBS>
{
    type Error = Infallible;
    unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
        Ok(&*value)
    }
}

impl<S: Fallible + ?Sized, const BITS: usize, const LIMBS: usize> Serialize<S> for Uint<BITS, LIMBS>
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        Serialize::<S>::serialize(&self.limbs, serializer)
    }
}

impl<D: Fallible + ?Sized, const BITS: usize, const LIMBS: usize>
    Deserialize<Uint<BITS, LIMBS>, D> for Archived<Uint<BITS, LIMBS>>
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Uint<BITS, LIMBS>, D::Error> {
        Ok(Uint {
            limbs: Deserialize::<[u64; LIMBS], D>::deserialize(&self.limbs, deserializer)?,
        })
    }
}
