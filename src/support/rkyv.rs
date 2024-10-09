use core::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::{ArchivedUint, Uint};

impl<const BITS: usize, const LIMBS: usize> From<&ArchivedUint<BITS, LIMBS>> for Uint<BITS, LIMBS> {
    fn from(archived: &ArchivedUint<BITS, LIMBS>) -> Self {
        Uint::from_limbs(archived.limbs.map(|x| x.into()))
    }
}

impl<const BITS: usize, const LIMBS: usize> From<ArchivedUint<BITS, LIMBS>> for Uint<BITS, LIMBS> {
    fn from(archived: ArchivedUint<BITS, LIMBS>) -> Self {
        Uint::from_limbs(archived.limbs.map(|x| x.into()))
    }
}


impl<const BITS: usize, const LIMBS: usize> Debug for ArchivedUint<BITS, LIMBS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(
            &<Uint<BITS, LIMBS> as From<&ArchivedUint<BITS, LIMBS>>>:: from(self),
            f
        )
    }
}

impl<const BITS: usize, const LIMBS: usize> Copy for ArchivedUint<BITS, LIMBS> {}

impl<const BITS: usize, const LIMBS: usize> Clone for ArchivedUint<BITS, LIMBS> {
    fn clone(&self) -> Self {
        ArchivedUint {
            limbs: self.limbs,
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> PartialEq for ArchivedUint<BITS, LIMBS> {
    fn eq(&self, other: &Self) -> bool {
        self.limbs == other.limbs
    }
}

impl<const BITS: usize, const LIMBS: usize> Eq for ArchivedUint<BITS, LIMBS> {}


impl<const BITS: usize, const LIMBS: usize> Hash for ArchivedUint<BITS, LIMBS> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.limbs.hash(state)
    }
}