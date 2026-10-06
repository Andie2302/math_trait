/// Menge mit abgeschlossener Verknüpfung.
pub trait Magma {}

/// Magma, dessen Verknüpfung assoziativ ist.
pub trait Semigroup: Magma {}

/// Halbgruppe mit neutralem Element.
pub trait Monoid: Semigroup {}

/// Monoid, in dem jedes Element ein Inverses hat.
pub trait Group: Monoid {}
