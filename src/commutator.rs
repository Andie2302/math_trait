//! Die Kommutator-Konstruktion: Jede assoziative Algebra ist mit `[x, y] = x ∘ y − y ∘ x`
//! eine Lie-Algebra.

use core::fmt;
use core::marker::PhantomData;

use crate::{
    Additive, AssociativeAlgebra, Bracket, CommutativeRing, Group, LeftAction, Magma,
    Multiplicative, UnitalMagma, impl_abelian_group, impl_lie_algebra, impl_magma, impl_module,
};

/// Die zur assoziativen Algebra `A` über `R` gehörende Lie-Algebra mit der Klammer
/// `[x, y] = x ∘ y − y ∘ x` (dem *Kommutator*).
///
/// Addition und Skalarmultiplikation sind die von `A`; nur die Klammer ersetzt das Produkt.
/// So entstehen die Matrix-Lie-Algebren und in der Quantenmechanik die Drehimpuls-Operatoren
/// mit `[Lx, Ly] = iħ Lz`.
///
/// `R` kommt nur als Typparameter vor, damit die Impls eindeutig bleiben.
pub struct Commutator<A, R> {
    inner: A,
    _scalars: PhantomData<fn() -> R>,
}

impl<A, R> Commutator<A, R> {
    /// Fasst `inner` als Element der Kommutator-Lie-Algebra auf.
    pub fn new(inner: A) -> Self {
        Commutator {
            inner,
            _scalars: PhantomData,
        }
    }

    /// Gibt das zugrunde liegende Element der Algebra zurück.
    pub fn into_inner(self) -> A {
        self.inner
    }

    /// Das zugrunde liegende Element der Algebra.
    pub fn as_inner(&self) -> &A {
        &self.inner
    }
}

impl<A: Clone, R> Clone for Commutator<A, R> {
    fn clone(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

impl<A: Copy, R> Copy for Commutator<A, R> {}

impl<A: PartialEq, R> PartialEq for Commutator<A, R> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<A: fmt::Debug, R> fmt::Debug for Commutator<A, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Commutator").field(&self.inner).finish()
    }
}

// Die Bedingung für alle Impls: `A` ist eine assoziative Algebra über dem kommutativen Ring `R`.

// --- Die Addition von A, unverändert --------------------------------------------------------

impl_abelian_group!(for [A: AssociativeAlgebra<R>, R: CommutativeRing] Commutator<A, R>, Additive;
    op(x, y) {
        Commutator::new(<A as Magma<Additive>>::op(x.as_inner(), y.as_inner()))
    }
    identity() { Commutator::new(<A as UnitalMagma<Additive>>::identity()) }
    inverse(x) { Commutator::new(<A as Group<Additive>>::inverse(x.as_inner())) }
);

// --- Die Skalarwirkung von A, unverändert ---------------------------------------------------

impl_module!(for [A: AssociativeAlgebra<R>, R: CommutativeRing] Commutator<A, R>, R;
    act(s, x) { Commutator::new(<A as LeftAction<R>>::act(s, x.as_inner())) }
);

// --- Die Klammer: [x, y] = x∘y − y∘x --------------------------------------------------------

impl_magma!(for [A: AssociativeAlgebra<R>, R: CommutativeRing] Commutator<A, R>, Bracket;
    op(x, y) {
        let xy = <A as Magma<Multiplicative>>::op(x.as_inner(), y.as_inner());
        let yx = <A as Magma<Multiplicative>>::op(y.as_inner(), x.as_inner());
        Commutator::new(<A as Magma<Additive>>::op(
            &xy,
            &<A as Group<Additive>>::inverse(&yx),
        ))
    }
);

impl_lie_algebra!(for [A: AssociativeAlgebra<R>, R: CommutativeRing] Commutator<A, R>, R);
