// --- Etiketten für Verknüpfungen ---

/// Etikett für die als „Addition“ geschriebene Verknüpfung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Additive;

/// Etikett für die als „Multiplikation“ geschriebene Verknüpfung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Multiplicative;

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
pub trait Magma<Op>: PartialMagma<Op> {}

/// Magma mit Teilbarkeit: `a ∘ x = b` und `y ∘ a = b` sind stets eindeutig lösbar.
pub trait Quasigroup<Op>: Magma<Op> + Cancellative<Op> {}

/// Magma mit neutralem Element.
pub trait UnitalMagma<Op>: Magma<Op> + UnitalPartialMagma<Op> {}

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
pub trait Group<Op>: Monoid<Op> + Loop<Op> + AssociativeQuasigroup<Op> + Groupoid<Op> {}

// --- Zusatzeigenschaften ---

/// Magma, dessen Verknüpfung kommutativ ist: `a ∘ b = b ∘ a`.
pub trait Commutative<Op>: Magma<Op> + Flexible<Op> {}

/// Magma, in dem jedes Element mit sich selbst verknüpft sich selbst ergibt: `a ∘ a = a`.
pub trait Idempotent<Op>: Magma<Op> {}

/// Monoid mit kommutativer Verknüpfung.
pub trait CommutativeMonoid<Op>: Monoid<Op> + Commutative<Op> {}

/// Gruppe mit kommutativer Verknüpfung.
pub trait AbelianGroup<Op>: Group<Op> + Commutative<Op> {}

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
pub trait DivisionRing<Add = Additive, Mul = Multiplicative>: Ring<Add, Mul> {}

/// Körper: kommutativer Schiefkörper.
pub trait Field<Add = Additive, Mul = Multiplicative>:
    DivisionRing<Add, Mul> + CommutativeRing<Add, Mul>
{
}
