/// Menge mit abgeschlossener Verknüpfung.
pub trait Magma {}

/// Magma, in dem Gleichungen `a ∘ x = b` und `y ∘ a = b` stets eindeutig lösbar sind (Teilbarkeit).
pub trait Quasigroup: Magma {}

/// Quasigruppe mit neutralem Element. Nicht notwendig assoziativ.
pub trait Loop: Quasigroup {}

/// Magma, dessen Verknüpfung assoziativ ist.
pub trait Semigroup: Magma {}

/// Halbgruppe mit neutralem Element.
pub trait Monoid: Semigroup {}

/// Monoid, in dem jedes Element ein Inverses hat. Zugleich ein assoziativer Loop.
pub trait Group: Monoid + Loop {}

/// Magma, dessen Verknüpfung kommutativ ist: `a ∘ b = b ∘ a`.
pub trait Commutative: Magma {}

/// Gruppe mit kommutativer Verknüpfung.
pub trait AbelianGroup: Group + Commutative {}
