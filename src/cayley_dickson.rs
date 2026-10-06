//! Die Cayley-Dickson-Konstruktion: aus einer Algebra mit Involution wird die doppelt so große.

use core::fmt;
use core::marker::PhantomData;

use crate::{
    Additive, Algebra, AlgebraWithInvolution, Alternative, AlternativeAlgebra, AntiAutomorphism,
    AssociativeAlgebra, Automorphism, Bilinear, Commutative, CommutativeRing, CompositionAlgebra,
    Conjugation, Distributive, Field, Flexible, Group, Involutive, LeftAction, LeftDistributive,
    Magma, Module, Multiplicative, Norm, PowerAssociative, QuadraticForm, Quasigroup,
    RightDistributive, Semigroup, Semigroupoid, TrivialInvolution, UnitalAlgebra, UnitalMagma,
};

/// Die Verdopplung der Algebra `A` über `R`: Paare `(a, b)` mit
///
/// ```text
/// (a, b) · (c, d) = (a·c − d*·b,  d·a + b·c*)
/// (a, b)*         = (a*, −b)
/// ```
///
/// Addition und Skalarmultiplikation wirken komponentenweise. Aus ℝ entsteht so ℂ, daraus ℍ,
/// 𝕆 (Oktonionen), 𝕊 (Sedenionen) und so fort.
///
/// Was die Verdopplung erbt, hängt von `A` ab (jeweils unter der genannten Bedingung):
///
/// | Eigenschaft von `CayleyDickson<A, R>` | Bedingung an `A` |
/// |---|---|
/// | Algebra, Einselement, Involution | immer (unitale Algebra mit Involution) |
/// | Norm-Form, `CompositionAlgebra` | Kompositionsalgebra und assoziativ |
/// | alternativ (auch flexibel, potenz-assoziativ) | Kompositionsalgebra und assoziativ |
/// | assoziativ | zusätzlich kommutativ |
/// | kommutativ | zusätzlich triviale Involution |
///
/// Das ist die bekannte Reihe: ℝ → ℂ bleibt kommutativ, ℍ ist assoziativ, aber nicht
/// kommutativ, 𝕆 ist nur noch alternativ, und die Sedenionen verlieren auch das und die
/// Kompositionseigenschaft (es entstehen Nullteiler).
pub struct CayleyDickson<A, R> {
    first: A,
    second: A,
    _scalars: PhantomData<fn() -> R>,
}

impl<A, R> CayleyDickson<A, R> {
    /// Das Paar `(first, second)`.
    pub fn new(first: A, second: A) -> Self {
        CayleyDickson {
            first,
            second,
            _scalars: PhantomData,
        }
    }

    /// Die erste Komponente.
    pub fn first(&self) -> &A {
        &self.first
    }

    /// Die zweite Komponente.
    pub fn second(&self) -> &A {
        &self.second
    }

    /// Zerlegt das Paar in seine Komponenten.
    pub fn into_parts(self) -> (A, A) {
        (self.first, self.second)
    }
}

impl<A: Clone, R> Clone for CayleyDickson<A, R> {
    fn clone(&self) -> Self {
        Self::new(self.first.clone(), self.second.clone())
    }
}

impl<A: Copy, R> Copy for CayleyDickson<A, R> {}

impl<A: PartialEq, R> PartialEq for CayleyDickson<A, R> {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

impl<A: fmt::Debug, R> fmt::Debug for CayleyDickson<A, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CayleyDickson")
            .field(&self.first)
            .field(&self.second)
            .finish()
    }
}

// --- Hilfsfunktionen in `A` ------------------------------------------------------------------

fn add<A: Magma<Additive>>(x: &A, y: &A) -> A {
    <A as Magma<Additive>>::op(x, y)
}
fn neg<A: Group<Additive>>(x: &A) -> A {
    <A as Group<Additive>>::inverse(x)
}
fn sub<A: Group<Additive>>(x: &A, y: &A) -> A {
    add(x, &neg(y))
}
fn mul<A: Magma<Multiplicative>>(x: &A, y: &A) -> A {
    <A as Magma<Multiplicative>>::op(x, y)
}
fn conj<A: Involutive<Conjugation>>(x: &A) -> A {
    <A as Involutive<Conjugation>>::conjugate(x)
}

/// Implementiert Marker-Traits für `CayleyDickson<A, R>` unter der Grundbedingung.
macro_rules! forward_markers {
    ($op:ty: $($tr:ident),+ $(,)?) => {
        $( impl<A, R> $crate::$tr<$op> for CayleyDickson<A, R>
            where A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing {} )+
    };
}

// --- Die Addition: komponentenweise ------------------------------------------------------------

forward_markers!(Additive: PartialMagma, Semigroupoid, Alternative, Flexible, PowerAssociative,
    Semigroup, Commutative, Trimedial, Medial, CommutativeSemigroup, UnitalPartialMagma,
    SmallCategory, Monoid, CommutativeMonoid, LeftCancellative, RightCancellative, Cancellative,
    Groupoid, Loop, AssociativeQuasigroup, AbelianGroup);

impl<A, R> Magma<Additive> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
    fn op(&self, rhs: &Self) -> Self {
        Self::new(add(&self.first, &rhs.first), add(&self.second, &rhs.second))
    }
}

impl<A, R> UnitalMagma<Additive> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
    fn identity() -> Self {
        Self::new(
            <A as UnitalMagma<Additive>>::identity(),
            <A as UnitalMagma<Additive>>::identity(),
        )
    }
}

impl<A, R> Quasigroup<Additive> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
    fn ldiv(&self, b: &Self) -> Self {
        Self::new(
            <A as Quasigroup<Additive>>::ldiv(&self.first, &b.first),
            <A as Quasigroup<Additive>>::ldiv(&self.second, &b.second),
        )
    }

    fn rdiv(&self, b: &Self) -> Self {
        Self::new(
            <A as Quasigroup<Additive>>::rdiv(&self.first, &b.first),
            <A as Quasigroup<Additive>>::rdiv(&self.second, &b.second),
        )
    }
}

impl<A, R> Group<Additive> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
    fn inverse(&self) -> Self {
        Self::new(neg(&self.first), neg(&self.second))
    }
}

// --- Die Skalarwirkung: komponentenweise -----------------------------------------------------

impl<A, R> LeftAction<R> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
    fn act(scalar: &R, x: &Self) -> Self {
        Self::new(
            <A as LeftAction<R>>::act(scalar, &x.first),
            <A as LeftAction<R>>::act(scalar, &x.second),
        )
    }
}

impl<A, R> Module<R> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
}

// --- Das Produkt: (a,b)(c,d) = (ac − d*b, da + bc*) --------------------------------------------

forward_markers!(Multiplicative: PartialMagma, UnitalPartialMagma);

impl<A, R> Magma<Multiplicative> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
    fn op(&self, rhs: &Self) -> Self {
        let (a, b) = (&self.first, &self.second);
        let (c, d) = (&rhs.first, &rhs.second);
        Self::new(
            sub(&mul(a, c), &mul(&conj(d), b)),
            add(&mul(d, a), &mul(b, &conj(c))),
        )
    }
}

impl<A, R> UnitalMagma<Multiplicative> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
    /// Das Einselement ist `(1, 0)`.
    fn identity() -> Self {
        Self::new(
            <A as UnitalMagma<Multiplicative>>::identity(),
            <A as UnitalMagma<Additive>>::identity(),
        )
    }
}

impl<A, R> LeftDistributive<Multiplicative, Additive> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
}
impl<A, R> RightDistributive<Multiplicative, Additive> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
}
impl<A, R> Distributive<Multiplicative, Additive> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
}
impl<A, R> Bilinear<CayleyDickson<A, R>, CayleyDickson<A, R>, R, Multiplicative>
    for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
}
impl<A, R> Algebra<R> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
}
impl<A, R> UnitalAlgebra<R> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
}

// --- Die Involution: (a, b)* = (a*, −b) -------------------------------------------------------

impl<A, R> Involutive for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
    fn conjugate(&self) -> Self {
        Self::new(conj(&self.first), neg(&self.second))
    }
}

impl<A, R> Automorphism<Additive> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
}
impl<A, R> AntiAutomorphism<Multiplicative> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
}
impl<A, R> AlgebraWithInvolution<R> for CayleyDickson<A, R>
where
    A: UnitalAlgebra<R> + AlgebraWithInvolution<R>,
    R: CommutativeRing,
{
}

// --- Die Norm: N(a, b) = N(a) + N(b) -----------------------------------------------------------

impl<A, R> QuadraticForm<R, Norm> for CayleyDickson<A, R>
where
    A: CompositionAlgebra<R>,
    R: Field,
{
    fn value(&self) -> R {
        <R as Magma<Additive>>::op(
            &<A as QuadraticForm<R, Norm>>::value(&self.first),
            &<A as QuadraticForm<R, Norm>>::value(&self.second),
        )
    }
}

// --- Bedingte Eigenschaften -------------------------------------------------------------------
//
// Bedingung "Kompositionsalgebra und assoziativ": Satz von Hurwitz, ℍ → 𝕆.

impl<A, R> Alternative<Multiplicative> for CayleyDickson<A, R>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative>,
    R: Field,
{
}
impl<A, R> Flexible<Multiplicative> for CayleyDickson<A, R>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative>,
    R: Field,
{
}
impl<A, R> PowerAssociative<Multiplicative> for CayleyDickson<A, R>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative>,
    R: Field,
{
}
impl<A, R> AlternativeAlgebra<R> for CayleyDickson<A, R>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative>,
    R: Field,
{
}
impl<A, R> CompositionAlgebra<R> for CayleyDickson<A, R>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative>,
    R: Field,
{
}

// Zusätzlich kommutativ: die Verdopplung ist assoziativ (ℂ → ℍ).

impl<A, R> Semigroupoid<Multiplicative> for CayleyDickson<A, R>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative> + Commutative<Multiplicative>,
    R: Field,
{
}
impl<A, R> Semigroup<Multiplicative> for CayleyDickson<A, R>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative> + Commutative<Multiplicative>,
    R: Field,
{
}
impl<A, R> AssociativeAlgebra<R> for CayleyDickson<A, R>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative> + Commutative<Multiplicative>,
    R: Field,
{
}

// Zusätzlich triviale Involution: die Verdopplung ist kommutativ (ℝ → ℂ).

impl<A, R> Commutative<Multiplicative> for CayleyDickson<A, R>
where
    A: CompositionAlgebra<R>
        + Semigroup<Multiplicative>
        + Commutative<Multiplicative>
        + TrivialInvolution,
    R: Field,
{
}
