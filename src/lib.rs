/// Menge mit abgeschlossener Verknüpfung.
pub trait Magma {}

/// Magma, dessen Verknüpfung assoziativ ist.
pub trait Semigroup: Magma {}

/// Halbgruppe mit neutralem Element.
pub trait Monoid: Semigroup {}

/// Monoid, in dem jedes Element ein Inverses hat.
pub trait Group: Monoid {}

/// Magma, dessen Verknüpfung kommutativ ist: `a ∘ b = b ∘ a`.
pub trait Commutative: Magma {}

/// Gruppe mit kommutativer Verknüpfung.
pub trait AbelianGroup: Group + Commutative {}
