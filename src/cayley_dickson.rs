//! Die Cayley-Dickson-Konstruktion: aus einer Algebra mit Involution wird die doppelt so große.

use core::fmt;
use core::marker::PhantomData;

use crate::{
    Additive, AlgebraWithInvolution, Alternative, AlternativeAlgebra, AssociativeAlgebra,
    Commutative, CommutativeRing, CompositionAlgebra, Conjugation, Field, Flexible, Group,
    Involutive, LeftAction, Magma, Multiplicative, Norm, PowerAssociative, QuadraticForm,
    Semigroup, Semigroupoid, TrivialInvolution, UnitalAlgebra, UnitalMagma, impl_abelian_group,
    impl_algebra, impl_algebra_with_involution, impl_module, impl_unital_algebra,
    impl_unital_magma,
};

/// Der Parameter `γ` der Verdopplung, ein Element von `R`, das dem Typ `Self` als Etikett
/// zugeordnet ist.
///
/// `γ` muss invertierbar sein, sonst artet die Verdopplung aus. Das prüft der Compiler nicht.
/// Mit `γ = −1` ([`MinusOne`]) entstehen ℂ, ℍ, 𝕆 über ℝ. Andere Werte liefern andere Formen,
/// etwa gespaltene Algebren (Nullteiler) oder, über endlichen Körpern, Körper.
pub trait Gamma<R> {
    /// Der Wert von `γ`.
    fn gamma() -> R;
}

/// Das Standard-`γ = −1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinusOne;

impl<R: crate::Ring> Gamma<R> for MinusOne {
    fn gamma() -> R {
        <R as Group<Additive>>::inverse(&<R as UnitalMagma<Multiplicative>>::identity())
    }
}

/// Die Verdopplung der Algebra `A` über `R`: Paare `(a, b)` mit
///
/// ```text
/// (a, b) · (c, d) = (a·c + γ·d*·b,  d·a + b·c*)
/// (a, b)*         = (a*, −b)
/// ```
///
/// `γ` kommt aus `G` (Standard: `−1`, siehe [`Gamma`]). Addition und Skalarmultiplikation wirken
/// komponentenweise. Aus ℝ entsteht so ℂ, daraus ℍ,
/// 𝕆 (Oktonionen), 𝕊 (Sedenionen) und so fort.
///
/// Was die Verdopplung erbt, hängt von `A` ab (jeweils unter der genannten Bedingung):
///
/// | Eigenschaft von `CayleyDickson<A, R, G>` | Bedingung an `A` |
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
pub struct CayleyDickson<A, R, G = MinusOne> {
    first: A,
    second: A,
    _scalars: PhantomData<fn() -> (R, G)>,
}

impl<A, R, G> CayleyDickson<A, R, G> {
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

impl<A: Clone, R, G> Clone for CayleyDickson<A, R, G> {
    fn clone(&self) -> Self {
        Self::new(self.first.clone(), self.second.clone())
    }
}

impl<A: Copy, R, G> Copy for CayleyDickson<A, R, G> {}

impl<A: PartialEq, R, G> PartialEq for CayleyDickson<A, R, G> {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

impl<A: fmt::Debug, R, G> fmt::Debug for CayleyDickson<A, R, G> {
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
fn mul<A: Magma<Multiplicative>>(x: &A, y: &A) -> A {
    <A as Magma<Multiplicative>>::op(x, y)
}
fn conj<A: Involutive<Conjugation>>(x: &A) -> A {
    <A as Involutive<Conjugation>>::conjugate(x)
}

// Die Grundbedingung für alle Impls: `A` ist eine unitale Algebra mit Involution über `R`.

// --- Die Addition und Skalarwirkung: komponentenweise ------------------------------------------

impl_abelian_group!(
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing, G: Gamma<R>]
    CayleyDickson<A, R, G>, Additive;
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
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing, G: Gamma<R>]
    CayleyDickson<A, R, G>, R;
    act(s, x) {
        CayleyDickson::new(
            <A as LeftAction<R>>::act(s, &x.first),
            <A as LeftAction<R>>::act(s, &x.second),
        )
    }
);

// --- Das Produkt: (a,b)(c,d) = (ac + γd*b, da + bc*), Einselement (1, 0) -------------------------

impl_unital_magma!(
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing, G: Gamma<R>]
    CayleyDickson<A, R, G>, Multiplicative;
    op(x, y) {
        let (a, b) = (&x.first, &x.second);
        let (c, d) = (&y.first, &y.second);
        let gamma = <G as Gamma<R>>::gamma();
        let scaled = <A as LeftAction<R>>::act(&gamma, &mul(&conj(d), b));
        CayleyDickson::new(add(&mul(a, c), &scaled), add(&mul(d, a), &mul(b, &conj(c))))
    }
    identity() {
        CayleyDickson::new(
            <A as UnitalMagma<Multiplicative>>::identity(),
            <A as UnitalMagma<Additive>>::identity(),
        )
    }
);

impl_algebra!(
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing, G: Gamma<R>]
    CayleyDickson<A, R, G>, R, Multiplicative
);
impl_unital_algebra!(
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing, G: Gamma<R>]
    CayleyDickson<A, R, G>, R, Multiplicative
);

// --- Die Involution: (a, b)* = (a*, −b) -------------------------------------------------------

impl_algebra_with_involution!(
    for [A: UnitalAlgebra<R> + AlgebraWithInvolution<R>, R: CommutativeRing, G: Gamma<R>]
    CayleyDickson<A, R, G>, R, Multiplicative;
    conjugate(x) { CayleyDickson::new(conj(&x.first), neg(&x.second)) }
);

// --- Die Norm: N(a, b) = N(a) − γ·N(b) -----------------------------------------------------------

impl<A, R, G> QuadraticForm<R, Norm> for CayleyDickson<A, R, G>
where
    A: CompositionAlgebra<R>,
    R: Field,
    G: Gamma<R>,
{
    fn value(&self) -> R {
        let na = <A as QuadraticForm<R, Norm>>::value(&self.first);
        let nb = <A as QuadraticForm<R, Norm>>::value(&self.second);
        let gamma_nb = <R as Magma<Multiplicative>>::op(&<G as Gamma<R>>::gamma(), &nb);
        <R as Magma<Additive>>::op(&na, &<R as Group<Additive>>::inverse(&gamma_nb))
    }
}

// --- Bedingte Eigenschaften -------------------------------------------------------------------
//
// Bedingung "Kompositionsalgebra und assoziativ": Satz von Hurwitz, ℍ → 𝕆.

impl<A, R, G> Alternative<Multiplicative> for CayleyDickson<A, R, G>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative>,
    R: Field,
    G: Gamma<R>,
{
}
impl<A, R, G> Flexible<Multiplicative> for CayleyDickson<A, R, G>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative>,
    R: Field,
    G: Gamma<R>,
{
}
impl<A, R, G> PowerAssociative<Multiplicative> for CayleyDickson<A, R, G>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative>,
    R: Field,
    G: Gamma<R>,
{
}
impl<A, R, G> AlternativeAlgebra<R> for CayleyDickson<A, R, G>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative>,
    R: Field,
    G: Gamma<R>,
{
}
impl<A, R, G> CompositionAlgebra<R> for CayleyDickson<A, R, G>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative>,
    R: Field,
    G: Gamma<R>,
{
}

// Zusätzlich kommutativ: die Verdopplung ist assoziativ (ℂ → ℍ).

impl<A, R, G> Semigroupoid<Multiplicative> for CayleyDickson<A, R, G>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative> + Commutative<Multiplicative>,
    R: Field,
    G: Gamma<R>,
{
}
impl<A, R, G> Semigroup<Multiplicative> for CayleyDickson<A, R, G>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative> + Commutative<Multiplicative>,
    R: Field,
    G: Gamma<R>,
{
}
impl<A, R, G> AssociativeAlgebra<R> for CayleyDickson<A, R, G>
where
    A: CompositionAlgebra<R> + Semigroup<Multiplicative> + Commutative<Multiplicative>,
    R: Field,
    G: Gamma<R>,
{
}

// Zusätzlich triviale Involution: die Verdopplung ist kommutativ (ℝ → ℂ).

impl<A, R, G> Commutative<Multiplicative> for CayleyDickson<A, R, G>
where
    A: CompositionAlgebra<R>
        + Semigroup<Multiplicative>
        + Commutative<Multiplicative>
        + TrivialInvolution,
    R: Field,
    G: Gamma<R>,
{
}
