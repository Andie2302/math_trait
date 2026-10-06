//! Die Einheitengruppe `K×` eines Schiefkörpers: die Elemente ungleich null.

use core::fmt;

use crate::{DivisionRing, Field, Magma, Multiplicative, UnitalMagma, impl_group};

/// Die Einheitengruppe `K×`: die Elemente von `K` ohne das Nullelement, mit der Multiplikation.
///
/// Anders als `K` selbst ist `K×` eine **Gruppe** bezüglich `Multiplicative`: Jedes Element hat
/// einen Kehrwert, und das Produkt zweier Elemente ungleich null ist ungleich null. Wenn `K`
/// ein Körper ist, ist `K×` sogar abelsch.
///
/// Ein Wert dieses Typs ist garantiert ungleich null (`new` gibt sonst `None` zurück).
pub struct Units<K> {
    inner: K,
}

impl<K: DivisionRing> Units<K> {
    /// `Some`, wenn `k` nicht das Nullelement ist, sonst `None`.
    pub fn new(k: K) -> Option<Self> {
        k.recip().map(|_| Units { inner: k })
    }
}

impl<K> Units<K> {
    /// Das zugrunde liegende Element von `K`.
    pub fn get(&self) -> &K {
        &self.inner
    }

    /// Gibt das zugrunde liegende Element zurück.
    pub fn into_inner(self) -> K {
        self.inner
    }
}

impl<K: Clone> Clone for Units<K> {
    fn clone(&self) -> Self {
        Units {
            inner: self.inner.clone(),
        }
    }
}

impl<K: Copy> Copy for Units<K> {}

impl<K: PartialEq> PartialEq for Units<K> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<K: fmt::Debug> fmt::Debug for Units<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Units").field(&self.inner).finish()
    }
}

impl_group!(for [K: DivisionRing] Units<K>, Multiplicative;
    op(a, b) { Units { inner: <K as Magma<Multiplicative>>::op(&a.inner, &b.inner) } }
    identity() { Units { inner: <K as UnitalMagma<Multiplicative>>::identity() } }
    inverse(a) {
        Units {
            inner: a.inner.recip().expect("ein Element von K× hat einen Kehrwert"),
        }
    }
);

// Über einem Körper ist K× abelsch.
crate::__markers!([K: Field] Units<K>, Multiplicative:
    Commutative, Trimedial, Medial, CommutativeSemigroup, CommutativeMonoid, AbelianGroup);
