//! Die Kommutator-Konstruktion: Jede assoziative Algebra ist mit `[x, y] = x ∘ y − y ∘ x`
//! eine Lie-Algebra.

use core::fmt;
use core::marker::PhantomData;

use crate::{
    Additive, Algebra, Alternating, Anticommutative, AssociativeAlgebra, Bilinear, Bracket,
    CommutativeRing, Distributive, Group, Jacobi, LeftAction, LeftDistributive, LieAlgebra, Magma,
    Module, Multiplicative, Quasigroup, RightDistributive, ScalarMultiplication, UnitalMagma,
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

/// Implementiert Marker-Traits für `Commutator<A, R>` mit der Bedingung "assoziative Algebra".
macro_rules! forward_markers {
    ($op:ty: $($tr:ident),+ $(,)?) => {
        $( impl<A, R> $crate::$tr<$op> for Commutator<A, R>
            where A: AssociativeAlgebra<R>, R: CommutativeRing {} )+
    };
}

// --- Die Addition von A, unverändert --------------------------------------------------------

forward_markers!(Additive: PartialMagma, Semigroupoid, Alternative, Flexible, PowerAssociative,
    Semigroup, Commutative, Trimedial, Medial, CommutativeSemigroup, UnitalPartialMagma,
    SmallCategory, Monoid, CommutativeMonoid, LeftCancellative, RightCancellative, Cancellative,
    Groupoid, Loop, AssociativeQuasigroup, AbelianGroup);

impl<A, R> Magma<Additive> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
    fn op(&self, rhs: &Self) -> Self {
        Self::new(<A as Magma<Additive>>::op(&self.inner, &rhs.inner))
    }
}

impl<A, R> UnitalMagma<Additive> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
    fn identity() -> Self {
        Self::new(<A as UnitalMagma<Additive>>::identity())
    }
}

impl<A, R> Quasigroup<Additive> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
    fn ldiv(&self, b: &Self) -> Self {
        Self::new(<A as Quasigroup<Additive>>::ldiv(&self.inner, &b.inner))
    }

    fn rdiv(&self, b: &Self) -> Self {
        Self::new(<A as Quasigroup<Additive>>::rdiv(&self.inner, &b.inner))
    }
}

impl<A, R> Group<Additive> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
    fn inverse(&self) -> Self {
        Self::new(<A as Group<Additive>>::inverse(&self.inner))
    }
}

// --- Die Skalarwirkung von A, unverändert ---------------------------------------------------

impl<A, R> LeftAction<R> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
    fn act(scalar: &R, x: &Self) -> Self {
        Self::new(<A as LeftAction<R>>::act(scalar, &x.inner))
    }
}

impl<A, R> Module<R> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
}

// --- Die Klammer: [x, y] = x∘y − y∘x --------------------------------------------------------

forward_markers!(Bracket: PartialMagma);

impl<A, R> Magma<Bracket> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
    fn op(&self, rhs: &Self) -> Self {
        let xy = <A as Magma<Multiplicative>>::op(&self.inner, &rhs.inner);
        let yx = <A as Magma<Multiplicative>>::op(&rhs.inner, &self.inner);
        Self::new(<A as Magma<Additive>>::op(
            &xy,
            &<A as Group<Additive>>::inverse(&yx),
        ))
    }
}

impl<A, R> LeftDistributive<Bracket, Additive> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
}
impl<A, R> RightDistributive<Bracket, Additive> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
}
impl<A, R> Distributive<Bracket, Additive> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
}
impl<A, R> Bilinear<Commutator<A, R>, Commutator<A, R>, R, Bracket> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
}
impl<A, R> Algebra<R, Additive, Multiplicative, ScalarMultiplication, Bracket> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
}

impl<A, R> Anticommutative<Bracket> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
}
impl<A, R> Alternating<Bracket> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
}
impl<A, R> Jacobi<Bracket> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
}
impl<A, R> LieAlgebra<R> for Commutator<A, R>
where
    A: AssociativeAlgebra<R>,
    R: CommutativeRing,
{
}
