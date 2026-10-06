//! Die Cayley-Dickson-Konstruktion: aus einer Algebra mit Involution wird die doppelt so große.

use core::fmt;
use core::marker::PhantomData;

use crate::{
    Additive, AlgebraWithInvolution, Alternative, AlternativeAlgebra, AntiAutomorphism,
    AssociativeAlgebra, Automorphism, Commutative, CommutativeRing, CompositionAlgebra,
    Conjugation, Field, Flexible, Group, Involutive, LeftAction, Magma, Multiplicative, Norm,
    PowerAssociative, QuadraticForm, Semigroup, Semigroupoid, TrivialInvolution, UnitalAlgebra,
    UnitalMagma, impl_abelian_group, impl_algebra, impl_module, impl_unital_algebra,
    impl_unital_magma,
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

// Die Grundbedingung für alle Impls: `A` ist eine unitale Algebra mit Involution über `R`.

// --- Die Addition und Skalarwirkung: komponentenweise ------------------------------------------

impl_abelian_group!(
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing]
    CayleyDickson<A, R>, Additive;
    op(x, y) { CayleyDickson::new(add(&x.first, &y.first), add(&x.second, &y.second)) }
    identity() {
        CayleyDickson::new(
            <A as UnitalMagma<Additive>>::identity(),
            <A as UnitalMagma<Additive>>::identity(),
        )
    }
    inverse(x) { CayleyDickson::new(neg(&x.first), neg(&x.second)) }
);

impl_module!(
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing]
    CayleyDickson<A, R>, R;
    act(s, x) {
        CayleyDickson::new(
            <A as LeftAction<R>>::act(s, &x.first),
            <A as LeftAction<R>>::act(s, &x.second),
        )
    }
);

// --- Das Produkt: (a,b)(c,d) = (ac − d*b, da + bc*), Einselement (1, 0) -------------------------

impl_unital_magma!(
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing]
    CayleyDickson<A, R>, Multiplicative;
    op(x, y) {
        let (a, b) = (&x.first, &x.second);
        let (c, d) = (&y.first, &y.second);
        CayleyDickson::new(
            sub(&mul(a, c), &mul(&conj(d), b)),
            add(&mul(d, a), &mul(b, &conj(c))),
        )
    }
    identity() {
        CayleyDickson::new(
            <A as UnitalMagma<Multiplicative>>::identity(),
            <A as UnitalMagma<Additive>>::identity(),
        )
    }
);

impl_algebra!(
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing]
    CayleyDickson<A, R>, R, Multiplicative
);
impl_unital_algebra!(
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing]
    CayleyDickson<A, R>, R, Multiplicative
);

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
