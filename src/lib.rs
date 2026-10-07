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
//! | Trait | Methoden |
//! |---|---|
//! | [`Magma`] | `op` |
//! | [`UnitalMagma`] | `identity` |
//! | [`Quasigroup`] | `ldiv`, `rdiv` |
//! | [`Group`] | `inverse` |
//! | [`DivisionRing`] | `recip`, `div`, `div_left` |
//! | [`DivisionAlgebra`] | `recip` |
//! | [`LeftAction`] | `act` |
//! | [`BilinearForm`] | `form` |
//! | [`QuadraticForm`] | `value` |
//! | [`BilinearMap`] | `bilinear` |
//! | [`Involutive`] | `conjugate` |
//! | [`TensorProduct`] | `tensor` |
//! | [`CliffordAlgebra`] | `embed` |
//! | [`LieModule`] | `lie_act` |
//! | [`GradedAlgebra`] | `even_part`, `odd_part`, `grade_involution` |
//! | [`Gamma`], [`DiagonalForm`] | `gamma`, `square` (Parameter der Konstruktionen) |
//! | [`numeric::Number`] | `zero`, `one` |
//!
//! ## Die Verknüpfung ist ein Typparameter
//!
//! `Magma<Op>` statt `Magma`: Ein Typ kann mehrere Rollen haben (`Magma<Additive>` *und*
//! `Magma<Multiplicative>`). Das Etikett `Op` ist ein beliebiger Typ. Bei mehreren Rollen
//! ruft man `<T as Magma<Additive>>::op(&a, &b)` auf.
//!
//! ## Etiketten und die Orphan-Regel
//!
//! Die Etiketten [`Additive`], [`Multiplicative`], [`ScalarMultiplication`], [`Conjugation`],
//! [`Norm`], [`Bracket`] … sind gewöhnliche Strukturen. Wer eine eigene Verknüpfung braucht
//! (`struct Min;`, `struct Konkatenation;`), definiert ein eigenes Etikett.
//!
//! Wegen der Orphan-Regel kann ein *anderes* Crate `Magma<Additive>` nicht für fremde Typen wie
//! `i32` implementieren: Weder das Trait noch der Typ gehören dort dazu. Mit eigenen Typen
//! (auch Newtypes um `i32`) oder mit einem eigenen Etikett geht es. Die höheren Schichten
//! (`Module`, `Algebra`, die Konstruktionen) kennen aber nur die Standard-Etiketten
//! `Additive`/`Multiplicative`. Impls für die Basisdatentypen sollen deshalb später in diesem
//! Crate stehen, hinter Feature-Flags (siehe `TODO.md`).
//!
//! **Marker-Impls sind Zusicherungen.** Der Compiler prüft die Gesetze (Assoziativität, Inverse,
//! Distributivität, …) nie, auch nicht bei handgeschriebenen Impls und nicht bei den Makros.
//!
//! ## Hierarchie
//!
//! `A → B` heißt: `B` hat `A` als Supertrait.
//!
//! ```text
//! Partielle Seite:  PartialMagma → UnitalPartialMagma ┐
//!                   PartialMagma → Semigroupoid ──────┴→ SmallCategory → Groupoid
//!
//! Eine Verknüpfung: Magma → Quasigroup ┐
//!                   Magma → UnitalMagma┴→ Loop ──┐
//!                   Magma → Semigroup → Monoid ──┴→ Group → AbelianGroup
//!                   Semigroup + Quasigroup → AssociativeQuasigroup  (Obertrait von Group)
//!                   Semigroup + Commutative → CommutativeSemigroup → CommutativeMonoid
//!                   Semigroup + Idempotent → Band;  Band + CommutativeSemigroup → Semilattice
//!                   Marker: Cancellative, Alternative, Flexible, PowerAssociative, Medial-,
//!                   Selbstdistributiv-, Unar- und Potenz-Familie
//!
//! Zwei Verknüpfungen (Add, Mul):
//!                   CommutativeMonoid<Add> + Monoid<Mul> + Distributive → Semiring
//!                   AbelianGroup<Add> + Semigroup<Mul> + Distributive → Rng
//!                   Rng + Semiring → Ring → CommutativeRing
//!                   Ring → DivisionRing;  DivisionRing + CommutativeRing → Field
//!
//! Dritte Verknüpfung (Skalare R wirken):
//!                   LeftAction + AbelianGroup → Module → VectorSpace   (automatisch über Körpern)
//!                   Module + Distributive + Bilinear → Algebra → Unital-/Associative-/
//!                   AlternativeAlgebra;  UnitalAlgebra → DivisionAlgebra, GradedAlgebra
//!                   UnitalAlgebra + AlgebraWithInvolution + QuadraticForm → CompositionAlgebra
//!                   Algebra + Alternating + Jacobi → LieAlgebra → LieModule (adjungierte Darstellung)
//!                   Involutive → Automorphism, AntiAutomorphism, TrivialInvolution → StarRing
//!                   TensorProduct, CliffordAlgebra, BilinearMap, BilinearForm, QuadraticForm
//! ```
//!
//! Dazu kommen konkrete Konstruktionen, die nur aus diesen Traits gebaut sind:
//!
//! ```text
//! Commutator<A, R>        jede assoziative Algebra wird mit [x, y] = xy − yx eine LieAlgebra
//! CayleyDickson<A, R, G>  verdoppelt eine Algebra mit Involution: ℝ → ℂ → ℍ → 𝕆 → 𝕊 …
//!                         (G: Gamma<R> legt den Parameter γ fest, Standard MinusOne)
//! Units<K>                die Einheitengruppe K× eines Schiefkörpers
//! Vector<R, N>            der freie Modul R^N
//! Tensor<R, M, N>         das Tensorprodukt V ⊗ W
//! Clifford<R, D, Q>       die freie Clifford-Algebra zur DiagonalForm Q, graduiert (GradedAlgebra)
//! EvenSubalgebra          die gerade Unteralgebra der Clifford-Algebra
//! Rotor                   die Spin-Gruppe: gerade s mit s·s̃ = 1, die Vektoren erhalten
//! Bivector                so(N): Grad-2-Elemente mit dem Kommutator; Vektor- und Spindarstellung
//! ```
//!
//! Zahlenartige Traits (`Number`, `Integer`, `Float`, …) stehen getrennt im Modul [`numeric`].
//!
//! ## Implementieren mit Makros
//!
//! Eine Gruppe verlangt 19 Impls, eine Lie-Algebra weit mehr. Die Makros `impl_*!` erzeugen sie
//! aus kurzen Rümpfen. Alle nehmen optional Typparameter mit Bedingungen vorweg
//! (`impl_x!(for [A: Bound, const N: usize] Typ<A>, Etikett; …)`). Die Gesetze prüfen die Makros
//! nicht: Wer eins benutzt, behauptet sie. Für einen Typ und ein Etikett nimmt man genau **ein**
//! Makro der jeweiligen Reihe:
//!
//! | Reihe | Makros |
//! |---|---|
//! | eine Verknüpfung | `impl_magma!`, `impl_commutative_magma!`, `impl_unital_magma!`, `impl_quasigroup!`, `impl_loop!`, `impl_semigroup!`, `impl_commutative_semigroup!`, `impl_band!`, `impl_semilattice!`, `impl_monoid!`, `impl_commutative_monoid!`, `impl_group!`, `impl_abelian_group!` |
//! | zwei Verknüpfungen | `impl_semiring!`, `impl_commutative_semiring!`, `impl_rng!`, `impl_ring!`, `impl_commutative_ring!`, `impl_division_ring!`, `impl_field!`, `impl_star_ring!` |
//! | Moduln und Algebren | `impl_module!`, `impl_algebra!` mit den Zusätzen `impl_unital_algebra!`, `impl_associative_algebra!`, `impl_alternative_algebra!`, `impl_division_algebra!`; für Ringe `impl_ring_algebra!`, für Körper über sich selbst `impl_field_algebra!` |
//! | Involution, Norm, Lie | `impl_algebra_with_involution!`, `impl_composition_algebra!`, `impl_lie_algebra!` |
//!
//! Die ausführliche Anleitung (Reihen, Kombinationsregeln, `for [..]`-Syntax) steht in der
//! Dokumentation der einzelnen Makros und im Quelltext von `macros.rs`.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod cayley_dickson;
mod commutator;
mod constructions;
mod macros;
pub mod numeric;
mod units;

pub use cayley_dickson::{CayleyDickson, Gamma, MinusOne};
pub use commutator::Commutator;
pub use constructions::{Bivector, Clifford, DiagonalForm, EvenSubalgebra, Rotor, Tensor, Vector};
pub use units::Units;

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

/// Etikett für die Umkehrung (*reversion*) `x ↦ x̃`: kehrt die Reihenfolge der Faktoren um.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reversion;

/// Etikett für die Gradinvolution `x ↦ x̂`: das Vorzeichen der ungeraden Anteile wird umgekehrt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GradeInvolution;

/// Etikett für die Norm-Form `N(x) = x ∘ x*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Norm;

/// Etikett für die Lie-Klammer `[x, y]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bracket;

// --- Partielle Seite: die Verknüpfung gilt nicht für jedes Paar ---

/// Menge mit Verknüpfung, die nicht für alle Paare definiert sein muss.
pub trait PartialMagma<Op> {}

/// Partielles Magma mit Einselementen: Jedes Element hat ein linkes und ein rechtes Einselement
/// (`e ∘ x = x`, `x ∘ f = x`), die im Allgemeinen verschieden sind und nur für die Elemente gelten,
/// mit denen sie verknüpfbar sind. Ein einziges Einselement für alle Paare liefert erst
/// [`UnitalMagma`].
pub trait UnitalPartialMagma<Op>: PartialMagma<Op> {}

/// Partielles Magma mit assoziativer Verknüpfung.
pub trait Semigroupoid<Op>: PartialMagma<Op> {}

/// Semigroupoid mit Einselementen: eine kleine Kategorie. Jedes Objekt hat seine Identität, die
/// Verknüpfung ist nur bei passender Quelle und passendem Ziel definiert. Ein Monoid ist eine
/// Kategorie mit einem Objekt.
pub trait SmallCategory<Op>: Semigroupoid<Op> + UnitalPartialMagma<Op> {}

/// Kleine Kategorie, in der jeder Morphismus invertierbar ist.
pub trait Groupoid<Op>: SmallCategory<Op> {}

// --- Totale Seite: die Verknüpfung gilt für alle Paare ---

/// Menge mit abgeschlossener Verknüpfung.
pub trait Magma<Op>: PartialMagma<Op> + Sized {
    /// Verknüpft `self` mit `rhs`.
    #[must_use]
    fn op(&self, rhs: &Self) -> Self;
}

/// Magma mit Teilbarkeit: `a ∘ x = b` und `y ∘ a = b` sind stets eindeutig lösbar.
pub trait Quasigroup<Op>: Magma<Op> + Cancellative<Op> {
    /// Löst `self ∘ x = b` nach `x`.
    #[must_use]
    fn ldiv(&self, b: &Self) -> Self;

    /// Löst `y ∘ self = b` nach `y`.
    #[must_use]
    fn rdiv(&self, b: &Self) -> Self;
}

/// Magma mit einem neutralen Element für alle Paare.
pub trait UnitalMagma<Op>: Magma<Op> + UnitalPartialMagma<Op> {
    /// Das neutrale Element (`0` bzw. `1`).
    #[must_use]
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
    #[must_use]
    fn inverse(&self) -> Self;
}

// --- Zusatzeigenschaften ---

/// Magma, dessen Verknüpfung kommutativ ist: `a ∘ b = b ∘ a`.
pub trait Commutative<Op>: Magma<Op> + Flexible<Op> {}

/// Magma, in dem jedes Element mit sich selbst verknüpft sich selbst ergibt: `a ∘ a = a`. Dann
/// erzeugt jedes Element nur sich selbst, also ist das Magma potenz-assoziativ.
pub trait Idempotent<Op>: Magma<Op> + PowerAssociative<Op> {}

/// Halbgruppe mit kommutativer Verknüpfung. Solche Halbgruppen sind stets medial.
pub trait CommutativeSemigroup<Op>: Semigroup<Op> + Commutative<Op> + Medial<Op> {}

/// Monoid mit kommutativer Verknüpfung.
pub trait CommutativeMonoid<Op>: Monoid<Op> + CommutativeSemigroup<Op> {}

/// Gruppe mit kommutativer Verknüpfung.
pub trait AbelianGroup<Op>: Group<Op> + CommutativeMonoid<Op> {}

/// Band: idempotente Halbgruppe, `x ∘ x = x`.
pub trait Band<Op>: Semigroup<Op> + Idempotent<Op> {}

/// Halbverband: kommutatives Band, z. B. `max`, `min`, Mengenvereinigung, `ggT`, logisches Oder.
pub trait Semilattice<Op>: Band<Op> + CommutativeSemigroup<Op> {}

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
/// Insbesondere semimedial.
pub trait Trimedial<Op>: Semimedial<Op> {}

/// Medial: `(x∘y)∘(u∘z) = (x∘u)∘(y∘z)`. Jedes Untermagma ist dann ebenfalls medial, und das
/// Magma ist trimedial (und damit semimedial).
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
pub trait Zeropotent<Op>: Unipotent<Op> {}

// --- Konstante Verknüpfungen ---

/// Links-unar: `x∘y = x∘z`. Das Ergebnis hängt nur vom linken Operanden ab.
pub trait LeftUnar<Op>: Magma<Op> {}

/// Rechts-unar: `y∘x = z∘x`. Das Ergebnis hängt nur vom rechten Operanden ab.
pub trait RightUnar<Op>: Magma<Op> {}

/// Nullhalbgruppe: `x∘y = u∘v`. Alle Produkte sind gleich.
pub trait NullSemigroup<Op>: Semigroup<Op> + LeftUnar<Op> + RightUnar<Op> + Zeropotent<Op> {}

/// Halbgruppe mit Linksnullen: `x∘y = x`. Das ist ein Band, links-unar und rechtskürzbar.
pub trait LeftZeroSemigroup<Op>: Band<Op> + LeftUnar<Op> + RightCancellative<Op> {}

/// Halbgruppe mit Rechtsnullen: `y∘x = x`. Das ist ein Band, rechts-unar und linkskürzbar.
pub trait RightZeroSemigroup<Op>: Band<Op> + RightUnar<Op> + LeftCancellative<Op> {}

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

/// Annihilierend: das Nullelement von `Add` absorbiert unter `Mul`: `0 ⋅ x = x ⋅ 0 = 0`.
///
/// In einem Ring folgt das aus dem Distributivgesetz. In einem Halbring muss man es verlangen,
/// weil es kein additives Inverses gibt, aus dem man es herleiten könnte.
pub trait Annihilating<Mul, Add = Additive>: Magma<Mul> + UnitalMagma<Add> {}

/// Halbring: `(S, Add)` ist ein kommutatives Monoid, `(S, Mul)` ein Monoid, `Mul` distribuiert
/// über `Add`, und das Nullelement absorbiert.
///
/// Wie ein Ring, aber ohne additive Inverse. Beispiele: die natürlichen Zahlen, die Wahrheitswerte
/// mit `∨` und `∧`, der tropische Halbring mit `min` und `+`.
pub trait Semiring<Add = Additive, Mul = Multiplicative>:
    CommutativeMonoid<Add> + Monoid<Mul> + Distributive<Mul, Add> + Annihilating<Mul, Add>
{
}

/// Halbring mit kommutativer Multiplikation.
pub trait CommutativeSemiring<Add = Additive, Mul = Multiplicative>:
    Semiring<Add, Mul> + Commutative<Mul>
{
}

/// Rng: Ring ohne Eins. `(R, Add)` ist eine abelsche Gruppe, `(R, Mul)` eine Halbgruppe, und
/// `Mul` distribuiert über `Add`. Ein Beispiel sind die geraden ganzen Zahlen.
pub trait Rng<Add = Additive, Mul = Multiplicative>:
    AbelianGroup<Add> + Semigroup<Mul> + Distributive<Mul, Add>
{
}

/// Ring: ein Rng mit Einselement. Zugleich ein Halbring mit additiven Inversen.
pub trait Ring<Add = Additive, Mul = Multiplicative>:
    Rng<Add, Mul> + Semiring<Add, Mul> + AbelianGroup<Add>
{
}

/// Ring mit kommutativer Multiplikation.
pub trait CommutativeRing<Add = Additive, Mul = Multiplicative>:
    Ring<Add, Mul> + CommutativeSemiring<Add, Mul>
{
}

/// Schiefkörper: Ring, in dem jedes Element außer dem Nullelement bezüglich `Mul` ein Inverses hat (z. B. die Quaternionen).
pub trait DivisionRing<Add = Additive, Mul = Multiplicative>: Ring<Add, Mul> {
    /// Der Kehrwert bezüglich `Mul`: `Some(x⁻¹)`, und `None` genau für das Nullelement.
    #[must_use]
    fn recip(&self) -> Option<Self>;

    /// Division von rechts: `self ⋅ rhs⁻¹`. `None`, wenn `rhs` das Nullelement ist.
    fn div(&self, rhs: &Self) -> Option<Self> {
        rhs.recip().map(|r| <Self as Magma<Mul>>::op(self, &r))
    }

    /// Division von links: `rhs⁻¹ ⋅ self`. `None`, wenn `rhs` das Nullelement ist.
    /// Im Körper gleich [`div`](DivisionRing::div).
    fn div_left(&self, rhs: &Self) -> Option<Self> {
        rhs.recip().map(|r| <Self as Magma<Mul>>::op(&r, self))
    }
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
/// `Add` benennt die Addition von `Self` *und* von `R`, `Mul` die Multiplikation von `R`. Beide
/// Typen teilen sich also das Etikett der Addition (siehe die offenen Punkte in `TODO.md`).
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
    /// Der Kehrwert bezüglich `Prod`: `Some(x⁻¹)` mit `x ∘ x⁻¹ = x⁻¹ ∘ x = 1`, und `None` genau
    /// für den Nullvektor. Ein zweiseitiges Inverses gibt es in den alternativen Divisionsalgebren
    /// (ℝ, ℂ, ℍ, 𝕆); in einer beliebigen nicht assoziativen Divisionsalgebra ist nur die Teilbarkeit
    /// (`ldiv`/`rdiv` von [`Quasigroup`]) garantiert.
    #[must_use]
    fn recip(&self) -> Option<Self>;
}

// --- Involution ---

/// Auf `Self` gibt es eine Involution `x ↦ x*`, benannt durch `Inv`: `(x*)* = x`.
pub trait Involutive<Inv = Conjugation>: Sized {
    /// Wendet die Involution an: `x ↦ x*`.
    #[must_use]
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
/// Nach dem Satz von Hurwitz haben Kompositionsalgebren die Dimension 1, 2, 4 oder 8. Über ℝ sind
/// die Divisionsalgebren darunter genau ℝ, ℂ, ℍ und 𝕆; dazu kommen die *gespaltenen* Formen mit
/// Nullteilern (z. B. die 2×2-Matrizen als gespaltene Quaternionen). Die Sedenionen sind keine
/// Kompositionsalgebra.
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
/// Zusammen mit der Distributivität von `Op` über `Add` folgt die Antikommutativität:
/// `0 = (x + y) ∘ (x + y) = x ∘ y + y ∘ x`. Ohne Distributivität wäre das kein Satz, deshalb
/// verlangt das Trait sie.
pub trait Alternating<Op, Add = Additive>:
    Anticommutative<Op, Add> + Distributive<Op, Add>
{
}

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

// --- Bilineare Abbildungen und Tensorprodukt ---

/// Bilineare Abbildung `Self × B → C` mit der Methode `bilinear`. Das ist [`Bilinear`] zusammen
/// mit der Funktion selbst, für Abbildungen zwischen *verschiedenen* Moduln.
pub trait BilinearMap<
    B,
    C,
    R,
    Map,
    Add = Additive,
    Mul = Multiplicative,
    Act = ScalarMultiplication,
>: Bilinear<B, C, R, Map, Add, Mul, Act> where
    R: CommutativeRing<Add, Mul>,
    B: Module<R, Add, Mul, Act>,
    C: Module<R, Add, Mul, Act>,
{
    /// Wertet die Abbildung an `(self, rhs)` aus.
    fn bilinear(&self, rhs: &B) -> C;
}

/// Das Tensorprodukt `V ⊗ W` über `R`: `Self` ist ein Modul mit einer bilinearen Abbildung
/// `tensor: V × W → Self`.
///
/// Die Elemente der Form `v ⊗ w` heißen *reine* Tensoren. Nicht jedes Element von `V ⊗ W` ist
/// rein: In der Quantenmechanik sind genau die nicht reinen Tensoren *verschränkte* Zustände.
///
/// Dazu gehört die universelle Eigenschaft: Jede bilineare Abbildung `V × W → U` faktorisiert
/// eindeutig über eine lineare Abbildung `Self → U`. Der Compiler prüft das nicht.
pub trait TensorProduct<V, W, R, Add = Additive, Mul = Multiplicative, Act = ScalarMultiplication>:
    Module<R, Add, Mul, Act>
where
    R: CommutativeRing<Add, Mul>,
    V: Module<R, Add, Mul, Act>,
    W: Module<R, Add, Mul, Act>,
{
    /// Das reine Tensorprodukt `v ⊗ w`, bilinear in `(v, w)`.
    fn tensor(v: &V, w: &W) -> Self;
}

// --- Clifford-Algebren ---

/// Clifford-Algebra von `(V, Q)` über dem Körper `K`: eine unitale Algebra `Self` mit einer
/// linearen Einbettung `embed: V → Self`, für die `embed(v)² = Q(v) ⋅ 1` gilt.
///
/// Die *universelle* Eigenschaft (die Clifford-Algebra ist die „freieste“ solche Algebra, jede
/// lineare Abbildung `f: V → A` mit `f(v)² = Q(v)` setzt sich eindeutig zu einem
/// Algebra-Homomorphismus fort) kann der Compiler nicht prüfen.
///
/// Beispiele: ℂ, ℍ (über ℝ), die Pauli-Algebra der Spin-½-Teilchen und die Dirac-Algebra.
pub trait CliffordAlgebra<V, K, Q, Add = Additive, Mul = Multiplicative, Act = ScalarMultiplication>:
    UnitalAlgebra<K, Add, Mul, Act>
where
    K: Field<Add, Mul>,
    V: VectorSpace<K, Add, Mul, Act> + QuadraticForm<K, Q, Add, Mul, Act>,
{
    /// Die lineare Einbettung `V → Self`.
    fn embed(v: &V) -> Self;
}

// --- Darstellungen von Lie-Algebren ---

/// Darstellung der Lie-Algebra `L` auf dem Modul `Self` über `R`: Jedes `x ∈ L` wirkt linear
/// auf `Self`, und die Klammer wird zum Kommutator der Wirkungen:
///
/// ```text
/// ρ([x, y]) = ρ(x) ∘ ρ(y) − ρ(y) ∘ ρ(x)
/// ```
///
/// Jede Lie-Algebra wirkt auf sich selbst durch die Klammer, die *adjungierte Darstellung*
/// (dafür gibt es ein Blanket-Impl). Dass das eine Darstellung ist, ist genau die
/// Jacobi-Identität. In der Quantenmechanik sind die Zustandsräume Darstellungen der
/// Symmetrie-Lie-Algebra, z. B. Spin ½ für `sl(2)` bzw. `su(2)`.
pub trait LieModule<L, R>: Module<R>
where
    R: CommutativeRing,
    L: LieAlgebra<R>,
{
    /// Die Wirkung `ρ(x)(v)`.
    fn lie_act(x: &L, v: &Self) -> Self;
}

impl<L, R> LieModule<L, R> for L
where
    L: LieAlgebra<R>,
    R: CommutativeRing,
{
    /// Die adjungierte Darstellung: `ad(x)(v) = [x, v]`.
    fn lie_act(x: &L, v: &Self) -> Self {
        <L as Magma<Bracket>>::op(x, v)
    }
}

// --- Graduierte Algebren ---

/// Eine ℤ/2-graduierte Algebra (*Superalgebra*): `A = A₀ ⊕ A₁` mit `Aᵢ ⋅ Aⱼ ⊆ A_{i+j}`. Das
/// Produkt zweier gerader oder zweier ungerader Elemente ist gerade, das Produkt eines geraden
/// und eines ungeraden ist ungerade.
///
/// Die Teile summieren sich zum Element: `x = even_part(x) + odd_part(x)`. Der Compiler prüft
/// die Verträglichkeit mit dem Produkt nicht.
///
/// Beispiele: Clifford-Algebren (gerade und ungerade Grade), das Matrix-Beispiel der
/// Fermionen und Bosonen in der Quantenmechanik.
pub trait GradedAlgebra<
    R,
    Add = Additive,
    Mul = Multiplicative,
    Act = ScalarMultiplication,
    Prod = Multiplicative,
>: UnitalAlgebra<R, Add, Mul, Act, Prod> where
    R: CommutativeRing<Add, Mul>,
{
    /// Der gerade Anteil `A₀`.
    #[must_use]
    fn even_part(&self) -> Self;

    /// Der ungerade Anteil `A₁`.
    #[must_use]
    fn odd_part(&self) -> Self;

    /// Die Gradinvolution `x̂ = gerade − ungerade`. Sie ist ein Algebra-Automorphismus.
    #[must_use]
    fn grade_involution(&self) -> Self {
        <Self as Magma<Add>>::op(
            &self.even_part(),
            &<Self as Group<Add>>::inverse(&self.odd_part()),
        )
    }
}
