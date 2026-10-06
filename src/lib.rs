// --- Partielle Seite: die Verknüpfung gilt nicht für jedes Paar ---

/// Menge mit Verknüpfung, die nicht für alle Paare definiert sein muss.
pub trait PartialMagma {}

/// Partielles Magma mit neutralem Element.
pub trait Multiplicative: PartialMagma {}

/// Partielles Magma mit assoziativer Verknüpfung.
pub trait Semigroupoid: PartialMagma {}

/// Semigroupoid mit neutralem Element (Kategorie mit Menge von Objekten).
pub trait SmallCategory: Semigroupoid + Multiplicative {}

/// Kleine Kategorie, in der jede Verknüpfung umkehrbar ist.
pub trait Groupoid: SmallCategory {}

// --- Totale Seite: die Verknüpfung gilt für alle Paare ---

/// Menge mit abgeschlossener Verknüpfung.
pub trait Magma: PartialMagma {}

/// Magma mit Teilbarkeit: `a ∘ x = b` und `y ∘ a = b` sind stets eindeutig lösbar.
pub trait Quasigroup: Magma {}

/// Magma mit neutralem Element.
pub trait UnitalMagma: Magma + Multiplicative {}

/// Quasigruppe mit neutralem Element. Nicht notwendig assoziativ.
pub trait Loop: Quasigroup + UnitalMagma {}

/// Magma, dessen Verknüpfung assoziativ ist.
pub trait Semigroup: Magma + Semigroupoid {}

/// Assoziative Quasigruppe. Ein neutrales Element folgt daraus zwangsläufig.
pub trait AssociativeQuasigroup: Semigroup + Quasigroup {}

/// Halbgruppe mit neutralem Element.
pub trait Monoid: Semigroup + UnitalMagma + SmallCategory {}

/// Monoid, in dem jedes Element ein Inverses hat. Zugleich ein assoziativer Loop.
pub trait Group: Monoid + Loop + AssociativeQuasigroup + Groupoid {}

// --- Zusatzeigenschaften ---

/// Magma, dessen Verknüpfung kommutativ ist: `a ∘ b = b ∘ a`.
pub trait Commutative: Magma {}

/// Magma, in dem jedes Element mit sich selbst verknüpft sich selbst ergibt: `a ∘ a = a`.
pub trait Idempotent: Magma {}

/// Monoid mit kommutativer Verknüpfung.
pub trait CommutativeMonoid: Monoid + Commutative {}

/// Gruppe mit kommutativer Verknüpfung.
pub trait AbelianGroup: Group + Commutative {}
