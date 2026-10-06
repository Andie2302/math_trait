//! # math_trait
//!
//! Ein Trait-System für Mathematik, **unabhängig von konkreten Basisdatentypen**.
//! Dieses Crate enthält keinen einzigen `i32`, `f64` & Co.
//!
//! ## Ein Kern mit Methoden, alles andere sind Marker
//!
//! Nur wenige Traits haben Methoden, alle anderen sind leere Marker, deren Gesetz in der
//! Doc-Zeile steht:
//!
//! | Trait | Methode |
//! |---|---|
//! | [`Magma`] | `op` |
//! | [`UnitalMagma`] | `identity` |
//! | [`Quasigroup`] | `ldiv`, `rdiv` |
//! | [`Group`] | `inverse` |
//! | [`LeftAction`] | `act` |
//! | [`BilinearForm`] | `form` |
//! | [`QuadraticForm`] | `value` |
//! | [`Involutive`] | `conjugate` |
//!
//! ## Die Verknüpfung ist ein Typparameter
//!
//! `Magma<Op>` statt `Magma`: Ein Typ kann mehrere Rollen haben (`Magma<Additive>` *und*
//! `Magma<Multiplicative>`). Das Etikett `Op` ist ein beliebiger Typ. Bei mehreren Rollen
//! ruft man `<T as Magma<Additive>>::op(&a, &b)` auf.
//!
//! ## Hierarchie
//!
//! ```text
//! PartialMagma ─ UnitalPartialMagma ─┐
//!      │                             ├─ SmallCategory ─ Groupoid
//!      └─ Semigroupoid ─────────────┘
//!
//! Magma ┬─ Quasigroup ─┐
//!       │              ├─ Loop ──────────────┐
//!       ├─ UnitalMagma ┘                     │
//!       └─ Semigroup ─ Monoid ───────────────┴─ Group ─ AbelianGroup
//!
//! Ring ⊂ CommutativeRing ⊂ Field        (mit zwei Verknüpfungen: Add, Mul)
//! Module ⊂ VectorSpace;  Algebra ⊂ UnitalAlgebra ⊂ DivisionAlgebra ⊂ CompositionAlgebra
//! Algebra ⊂ LieAlgebra                  (nicht assoziativ: [x, y], alternierend, Jacobi)
//! CayleyDickson<A, R>                  (verdoppelt eine Algebra mit Involution: ℝ → ℂ → ℍ → 𝕆 → 𝕊 …)
//! Commutator<A, R>                      (jede assoziative Algebra A wird mit [x,y] = xy − yx eine LieAlgebra)
//! ```
//!
//! Zahlenartige Traits (`Number`, `Integer`, `Float`, …) stehen getrennt im Modul [`numeric`].
//!
//! ## Implementieren mit Makros
//!
//! Eine Gruppe verlangt 15 Impls, eine Lie-Algebra weit mehr. Die Makros `impl_magma!`,
//! `impl_unital_magma!`, `impl_semigroup!`, `impl_monoid!`, `impl_group!`, `impl_abelian_group!`,
//! `impl_ring!`, `impl_field!`, `impl_module!`, `impl_algebra!` (mit Zusätzen wie
//! `impl_unital_algebra!`; für Ringe `impl_ring_algebra!`) und `impl_lie_algebra!` erzeugen sie aus kurzen Rümpfen. Alle nehmen
//! optional Typparameter mit Bedingungen (`for [A: Bound] Typ<A>`), siehe das Modul `macros`
//! im Quelltext. Die Gesetze prüfen die Makros nicht: Wer eins benutzt, behauptet sie.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod cayley_dickson;
mod commutator;
mod macros;
pub mod numeric;

pub use cayley_dickson::CayleyDickson;
pub use commutator::Commutator;

// --- Etiketten für Verknüpfungen ---

/// Etikett für die als „Addition“ geschriebene Verknüpfung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Additive;

/// Etikett für die als „Multiplikation“ geschriebene Verknüpfung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Multiplicative;

/// Etikett für die Skalarmultiplikation `Skalar × Vektor → Vektor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScalarMultiplication;

/// Etikett für die Konjugation, die Standard-Involution `x ↦ x*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Conjugation;

/// Etikett für die Norm-Form `N(x) = x ∘ x*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Norm;

/// Etikett für die Lie-Klammer `[x, y]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bracket;

// --- Partielle Seite: die Verknüpfung gilt nicht für jedes Paar ---

/// Menge mit Verknüpfung, die nicht für alle Paare definiert sein muss.
pub trait PartialMagma<Op> {}

/// Partielles Magma mit neutralem Element.
pub trait UnitalPartialMagma<Op>: PartialMagma<Op> {}

/// Partielles Magma mit assoziativer Verknüpfung.
pub trait Semigroupoid<Op>: PartialMagma<Op> {}

/// Semigroupoid mit neutralem Element (Kategorie mit Menge von Objekten).
pub trait SmallCategory<Op>: Semigroupoid<Op> + UnitalPartialMagma<Op> {}

/// Kleine Kategorie, in der jede Verknüpfung umkehrbar ist.
pub trait Groupoid<Op>: SmallCategory<Op> {}

// --- Totale Seite: die Verknüpfung gilt für alle Paare ---

/// Menge mit abgeschlossener Verknüpfung.
pub trait Magma<Op>: PartialMagma<Op> + Sized {
    /// Verknüpft `self` mit `rhs`.
    fn op(&self, rhs: &Self) -> Self;
}

/// Magma mit Teilbarkeit: `a ∘ x = b` und `y ∘ a = b` sind stets eindeutig lösbar.
pub trait Quasigroup<Op>: Magma<Op> + Cancellative<Op> {
    /// Löst `self ∘ x = b` nach `x`.
    fn ldiv(&self, b: &Self) -> Self;

    /// Löst `y ∘ self = b` nach `y`.
    fn rdiv(&self, b: &Self) -> Self;
}

/// Magma mit neutralem Element.
pub trait UnitalMagma<Op>: Magma<Op> + UnitalPartialMagma<Op> {
    /// Das neutrale Element (`0` bzw. `1`).
    fn identity() -> Self;
}

/// Quasigruppe mit neutralem Element. Nicht notwendig assoziativ.
pub trait Loop<Op>: Quasigroup<Op> + UnitalMagma<Op> {}

/// Magma, dessen Verknüpfung assoziativ ist.
pub trait Semigroup<Op>:
    Magma<Op> + Semigroupoid<Op> + Alternative<Op> + Flexible<Op> + PowerAssociative<Op>
{
}

/// Assoziative Quasigruppe. Ein neutrales Element folgt daraus zwangsläufig.
pub trait AssociativeQuasigroup<Op>: Semigroup<Op> + Quasigroup<Op> {}

/// Halbgruppe mit neutralem Element.
pub trait Monoid<Op>: Semigroup<Op> + UnitalMagma<Op> + SmallCategory<Op> {}

/// Monoid, in dem jedes Element ein Inverses hat. Zugleich ein assoziativer Loop.
pub trait Group<Op>: Monoid<Op> + Loop<Op> + AssociativeQuasigroup<Op> + Groupoid<Op> {
    /// Das Inverse von `self` (`-a` bzw. `a⁻¹`).
    fn inverse(&self) -> Self;
}

// --- Zusatzeigenschaften ---

/// Magma, dessen Verknüpfung kommutativ ist: `a ∘ b = b ∘ a`.
pub trait Commutative<Op>: Magma<Op> + Flexible<Op> {}

/// Magma, in dem jedes Element mit sich selbst verknüpft sich selbst ergibt: `a ∘ a = a`.
pub trait Idempotent<Op>: Magma<Op> {}

/// Halbgruppe mit kommutativer Verknüpfung. Solche Halbgruppen sind stets medial.
pub trait CommutativeSemigroup<Op>: Semigroup<Op> + Commutative<Op> + Medial<Op> {}

/// Monoid mit kommutativer Verknüpfung.
pub trait CommutativeMonoid<Op>: Monoid<Op> + CommutativeSemigroup<Op> {}

/// Gruppe mit kommutativer Verknüpfung.
pub trait AbelianGroup<Op>: Group<Op> + CommutativeMonoid<Op> {}

// --- Kürzbarkeit ---

/// Linkskürzbar: `x ∘ y = x ∘ z` impliziert `y = z`.
pub trait LeftCancellative<Op>: Magma<Op> {}

/// Rechtskürzbar: `y ∘ x = z ∘ x` impliziert `y = z`.
pub trait RightCancellative<Op>: Magma<Op> {}

/// Links- und rechtskürzbar.
pub trait Cancellative<Op>: LeftCancellative<Op> + RightCancellative<Op> {}

// --- Schwächere Formen der Assoziativität ---

/// Alternativ: `(x∘x)∘y = x∘(x∘y)` und `x∘(y∘y) = (x∘y)∘y`.
pub trait Alternative<Op>: Magma<Op> {}

/// Flexibel: `(x∘y)∘x = x∘(y∘x)`.
pub trait Flexible<Op>: Magma<Op> {}

/// Potenz-assoziativ: Das von jedem Element erzeugte Untermagma ist assoziativ.
pub trait PowerAssociative<Op>: Magma<Op> {}

// --- Mediale Familie ---

/// Trimedial: Je drei (nicht notwendig verschiedene) Elemente erzeugen ein mediales Untermagma.
pub trait Trimedial<Op>: Magma<Op> {}

/// Medial: `(x∘y)∘(u∘z) = (x∘u)∘(y∘z)`. Jedes Untermagma ist dann ebenfalls medial.
pub trait Medial<Op>: Trimedial<Op> {}

/// Entropisch: homomorphes Bild eines medialen, kürzbaren Magmas. Insbesondere medial.
pub trait Entropic<Op>: Medial<Op> {}

/// Linkssemimedial: `(x∘x)∘(y∘z) = (x∘y)∘(x∘z)`.
pub trait LeftSemimedial<Op>: Magma<Op> {}

/// Rechtssemimedial: `(y∘z)∘(x∘x) = (y∘x)∘(z∘x)`.
pub trait RightSemimedial<Op>: Magma<Op> {}

/// Semimedial: links- und rechtssemimedial.
pub trait Semimedial<Op>: LeftSemimedial<Op> + RightSemimedial<Op> {}

// --- Selbstdistributivität (die Verknüpfung distribuiert über sich selbst) ---

/// Linksdistributiv: `x∘(y∘z) = (x∘y)∘(x∘z)`.
pub trait LeftSelfDistributive<Op>: Magma<Op> {}

/// Rechtsdistributiv: `(y∘z)∘x = (y∘x)∘(z∘x)`.
pub trait RightSelfDistributive<Op>: Magma<Op> {}

/// Autodistributiv: links- und rechtsdistributiv.
pub trait SelfDistributive<Op>: LeftSelfDistributive<Op> + RightSelfDistributive<Op> {}

// --- Potenz-Eigenschaften ---

/// Unipotent: `x∘x = y∘y`. Alle Quadrate sind gleich.
pub trait Unipotent<Op>: Magma<Op> {}

/// Nullpotent: `(x∘x)∘y = x∘x = y∘(x∘x)`. Quadrate absorbieren.
pub trait Zeropotent<Op>: Magma<Op> {}

// --- Konstante Verknüpfungen ---

/// Links-unar: `x∘y = x∘z`. Das Ergebnis hängt nur vom linken Operanden ab.
pub trait LeftUnar<Op>: Magma<Op> {}

/// Rechts-unar: `y∘x = z∘x`. Das Ergebnis hängt nur vom rechten Operanden ab.
pub trait RightUnar<Op>: Magma<Op> {}

/// Nullhalbgruppe: `x∘y = u∘v`. Alle Produkte sind gleich.
pub trait NullSemigroup<Op>: Semigroup<Op> + LeftUnar<Op> + RightUnar<Op> {}

/// Halbgruppe mit Linksnullen: `x∘y = x`.
pub trait LeftZeroSemigroup<Op>: Semigroup<Op> {}

/// Halbgruppe mit Rechtsnullen: `y∘x = x`.
pub trait RightZeroSemigroup<Op>: Semigroup<Op> {}

// --- Sonstiges ---

/// Zentral: `(x∘y)∘(y∘z) = y`.
pub trait Central<Op>: Magma<Op> {}

// --- Zwei Verknüpfungen: Distributivität ---

/// `Mul` distribuiert von links über `Add`: `a ⋅ (b + c) = a ⋅ b + a ⋅ c`.
pub trait LeftDistributive<Mul, Add>: Magma<Mul> + Magma<Add> {}

/// `Mul` distribuiert von rechts über `Add`: `(b + c) ⋅ a = b ⋅ a + c ⋅ a`.
pub trait RightDistributive<Mul, Add>: Magma<Mul> + Magma<Add> {}

/// `Mul` distribuiert von beiden Seiten über `Add`.
pub trait Distributive<Mul, Add>: LeftDistributive<Mul, Add> + RightDistributive<Mul, Add> {}

// --- Ringe und Körper ---

/// Ring: `(R, Add)` ist eine abelsche Gruppe, `(R, Mul)` ein Monoid, und `Mul` distribuiert über `Add`.
pub trait Ring<Add = Additive, Mul = Multiplicative>:
    AbelianGroup<Add> + Monoid<Mul> + Distributive<Mul, Add>
{
}

/// Ring mit kommutativer Multiplikation.
pub trait CommutativeRing<Add = Additive, Mul = Multiplicative>:
    Ring<Add, Mul> + Commutative<Mul>
{
}

/// Schiefkörper: Ring, in dem jedes Element außer dem Nullelement bezüglich `Mul` ein Inverses hat (z. B. die Quaternionen).
pub trait DivisionRing<Add = Additive, Mul = Multiplicative>: Ring<Add, Mul> {
    /// Der Kehrwert bezüglich `Mul`: `Some(x⁻¹)`, und `None` genau für das Nullelement.
    fn recip(&self) -> Option<Self>;
}

/// Körper: kommutativer Schiefkörper.
pub trait Field<Add = Additive, Mul = Multiplicative>:
    DivisionRing<Add, Mul> + CommutativeRing<Add, Mul>
{
}

// --- Dritte Verknüpfung: Skalare wirken auf Elemente ---

/// Die Menge `S` wirkt von links auf `Self`: `S × Self → Self`.
///
/// Anders als bei `Magma` stehen links und rechts verschiedene Typen.
pub trait LeftAction<S, Act = ScalarMultiplication>: Sized {
    /// Lässt `scalar` auf `x` wirken.
    fn act(scalar: &S, x: &Self) -> Self;
}

/// Modul über dem Ring `R`: `Self` ist eine abelsche Gruppe, `R` wirkt von links, und es gilt
/// `a(x + y) = ax + ay`, `(a + b)x = ax + bx`, `(ab)x = a(bx)` und `1x = x`.
///
/// `Add` und `Mul` benennen die Verknüpfungen von `Self` bzw. `R` (je Typ ein eigenes Etikett-Paar).
pub trait Module<R, Add = Additive, Mul = Multiplicative, Act = ScalarMultiplication>:
    AbelianGroup<Add> + LeftAction<R, Act>
where
    R: Ring<Add, Mul>,
{
}

/// Vektorraum: Modul über einem Körper. Gilt automatisch für jedes solche Modul.
pub trait VectorSpace<K, Add = Additive, Mul = Multiplicative, Act = ScalarMultiplication>:
    Module<K, Add, Mul, Act>
where
    K: Field<Add, Mul>,
{
}

impl<V, K, Add, Mul, Act> VectorSpace<K, Add, Mul, Act> for V
where
    K: Field<Add, Mul>,
    V: Module<K, Add, Mul, Act>,
{
}

/// Bilineare Abbildung `Self × B → C` über dem kommutativen Ring `R`, benannt durch das Etikett `Map`.
///
/// Linear in jedem Argument: `f(x + x', y) = f(x, y) + f(x', y)`, `f(x, y + y') = f(x, y) + f(x, y')`
/// und `f(ax, y) = a f(x, y) = f(x, ay)`. `Self`, `B` und `C` sind Moduln über `R`.
pub trait Bilinear<B, C, R, Map, Add = Additive, Mul = Multiplicative, Act = ScalarMultiplication>:
    Module<R, Add, Mul, Act>
where
    R: CommutativeRing<Add, Mul>,
    B: Module<R, Add, Mul, Act>,
    C: Module<R, Add, Mul, Act>,
{
}

/// Bilinearform `Self × Self → K` über dem Körper `K` (Sonderfall von [`Bilinear`] mit `C = K`).
pub trait BilinearForm<K, Map, Add = Additive, Mul = Multiplicative, Act = ScalarMultiplication>:
    VectorSpace<K, Add, Mul, Act>
where
    K: Field<Add, Mul>,
{
    /// Wertet die Form an `(self, rhs)` aus.
    fn form(&self, rhs: &Self) -> K;
}

/// Quadratische Form `Q: Self → K` über dem Körper `K`, benannt durch das Etikett `Q`.
///
/// Es gilt `Q(ax) = a² Q(x)`, und `Q(x + y) − Q(x) − Q(y)` ist eine [`BilinearForm`].
pub trait QuadraticForm<K, Q, Add = Additive, Mul = Multiplicative, Act = ScalarMultiplication>:
    VectorSpace<K, Add, Mul, Act>
where
    K: Field<Add, Mul>,
{
    /// Wertet die Form an `self` aus.
    fn value(&self) -> K;
}

/// Algebra über dem kommutativen Ring `R`: ein Modul mit einer weiteren Verknüpfung `Prod`
/// (`Self × Self → Self`), die über `Add` distribuiert und mit den Skalaren verträglich ist:
/// `(ax)y = a(xy) = x(ay)`. Assoziativität wird *nicht* verlangt.
pub trait Algebra<
    R,
    Add = Additive,
    Mul = Multiplicative,
    Act = ScalarMultiplication,
    Prod = Multiplicative,
>:
    Module<R, Add, Mul, Act> + Distributive<Prod, Add> + Bilinear<Self, Self, R, Prod, Add, Mul, Act> where
    R: CommutativeRing<Add, Mul>,
{
}

/// Algebra mit Einselement bezüglich `Prod`.
pub trait UnitalAlgebra<
    R,
    Add = Additive,
    Mul = Multiplicative,
    Act = ScalarMultiplication,
    Prod = Multiplicative,
>: Algebra<R, Add, Mul, Act, Prod> + UnitalMagma<Prod> where
    R: CommutativeRing<Add, Mul>,
{
}

/// Algebra mit assoziativem Produkt.
pub trait AssociativeAlgebra<
    R,
    Add = Additive,
    Mul = Multiplicative,
    Act = ScalarMultiplication,
    Prod = Multiplicative,
>: Algebra<R, Add, Mul, Act, Prod> + Semigroup<Prod> where
    R: CommutativeRing<Add, Mul>,
{
}

/// Algebra mit alternativem Produkt (z. B. die Oktonionen).
pub trait AlternativeAlgebra<
    R,
    Add = Additive,
    Mul = Multiplicative,
    Act = ScalarMultiplication,
    Prod = Multiplicative,
>: Algebra<R, Add, Mul, Act, Prod> + Alternative<Prod> where
    R: CommutativeRing<Add, Mul>,
{
}

/// Divisionsalgebra: Algebra mit Einselement, in der jedes Element außer dem Nullvektor
/// bezüglich `Prod` teilbar ist (z. B. ℝ, ℂ, ℍ, 𝕆).
pub trait DivisionAlgebra<
    R,
    Add = Additive,
    Mul = Multiplicative,
    Act = ScalarMultiplication,
    Prod = Multiplicative,
>: UnitalAlgebra<R, Add, Mul, Act, Prod> where
    R: CommutativeRing<Add, Mul>,
{
    /// Der Kehrwert bezüglich `Prod`: `Some(x⁻¹)`, und `None` genau für den Nullvektor.
    fn recip(&self) -> Option<Self>;
}

// --- Involution ---

/// Auf `Self` gibt es eine Involution `x ↦ x*`, benannt durch `Inv`: `(x*)* = x`.
pub trait Involutive<Inv = Conjugation>: Sized {
    /// Wendet die Involution an: `x ↦ x*`.
    fn conjugate(&self) -> Self;
}

/// Die Involution ist die Identität: `x* = x`.
pub trait TrivialInvolution<Inv = Conjugation>: Involutive<Inv> {}

/// Die Involution `Inv` ist verträglich mit `Op`: `(x ∘ y)* = x* ∘ y*`.
pub trait Automorphism<Op, Inv = Conjugation>: Magma<Op> + Involutive<Inv> {}

/// Die Involution `Inv` kehrt `Op` um: `(x ∘ y)* = y* ∘ x*`.
pub trait AntiAutomorphism<Op, Inv = Conjugation>: Magma<Op> + Involutive<Inv> {}

/// *-Ring: Ring mit Involution, die `Add` erhält und `Mul` umkehrt.
pub trait StarRing<Add = Additive, Mul = Multiplicative, Inv = Conjugation>:
    Ring<Add, Mul> + Automorphism<Add, Inv> + AntiAutomorphism<Mul, Inv>
{
}

/// Algebra mit `R`-linearer Involution, die `Add` erhält und `Prod` umkehrt (z. B. Konjugation in ℂ, ℍ, 𝕆).
pub trait AlgebraWithInvolution<
    R,
    Add = Additive,
    Mul = Multiplicative,
    Act = ScalarMultiplication,
    Prod = Multiplicative,
    Inv = Conjugation,
>: Algebra<R, Add, Mul, Act, Prod> + Automorphism<Add, Inv> + AntiAutomorphism<Prod, Inv> where
    R: CommutativeRing<Add, Mul>,
{
}

// --- Norm ---

/// Kompositionsalgebra über dem Körper `K`: unitale Algebra mit Involution und
/// nicht ausgearteter quadratischer Norm-Form `Nm(x) = x ∘ x*`, die multiplikativ ist:
/// `Nm(x ∘ y) = Nm(x) ⋅ Nm(y)`.
///
/// Nach dem Satz von Hurwitz sind das (über ℝ) genau ℝ, ℂ, ℍ und 𝕆. Die Sedenionen sind keine.
pub trait CompositionAlgebra<
    K,
    Add = Additive,
    Mul = Multiplicative,
    Act = ScalarMultiplication,
    Prod = Multiplicative,
    Inv = Conjugation,
    Nm = Norm,
>:
    UnitalAlgebra<K, Add, Mul, Act, Prod>
    + AlgebraWithInvolution<K, Add, Mul, Act, Prod, Inv>
    + QuadraticForm<K, Nm, Add, Mul, Act> where
    K: Field<Add, Mul>,
{
}

// --- Lie-Algebren ---

/// Antikommutativ bezüglich `Op`: `x ∘ y = −(y ∘ x)`, wobei das Negative bezüglich `Add` gebildet wird.
pub trait Anticommutative<Op, Add = Additive>: Magma<Op> + Group<Add> {}

/// Alternierend bezüglich `Op`: `x ∘ x = 0` (das Nullelement von `Add`).
///
/// Daraus folgt aus der Bilinearität die Antikommutativität.
pub trait Alternating<Op, Add = Additive>: Anticommutative<Op, Add> + UnitalMagma<Add> {}

/// Jacobi-Identität für `Op`: `x ∘ (y ∘ z) + y ∘ (z ∘ x) + z ∘ (x ∘ y) = 0`.
///
/// Sie ersetzt die Assoziativität und sagt, dass `Op` „fast“ assoziativ ist.
pub trait Jacobi<Op, Add = Additive>: Magma<Op> + UnitalMagma<Add> {}

/// Lie-Algebra über dem kommutativen Ring `R`: eine Algebra, deren Produkt `Br` (die Lie-Klammer
/// `[x, y]`) alternierend ist und die Jacobi-Identität erfüllt. Sie ist in der Regel nicht
/// assoziativ und hat kein Einselement.
///
/// Beispiele: Vektoren im ℝ³ mit dem Kreuzprodukt, Matrizen mit dem Kommutator `[A, B] = AB − BA`
/// und in der Quantenmechanik die Drehimpuls-Operatoren mit `[Lx, Ly] = iħ Lz`.
pub trait LieAlgebra<
    R,
    Add = Additive,
    Mul = Multiplicative,
    Act = ScalarMultiplication,
    Br = Bracket,
>: Algebra<R, Add, Mul, Act, Br> + Alternating<Br, Add> + Jacobi<Br, Add> where
    R: CommutativeRing<Add, Mul>,
{
}
