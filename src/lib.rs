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
pub trait Quasigroup: Magma + Cancellative {}

/// Magma mit neutralem Element.
pub trait UnitalMagma: Magma + Multiplicative {}

/// Quasigruppe mit neutralem Element. Nicht notwendig assoziativ.
pub trait Loop: Quasigroup + UnitalMagma {}

/// Magma, dessen Verknüpfung assoziativ ist.
pub trait Semigroup: Magma + Semigroupoid + Alternative + Flexible + PowerAssociative {}

/// Assoziative Quasigruppe. Ein neutrales Element folgt daraus zwangsläufig.
pub trait AssociativeQuasigroup: Semigroup + Quasigroup {}

/// Halbgruppe mit neutralem Element.
pub trait Monoid: Semigroup + UnitalMagma + SmallCategory {}

/// Monoid, in dem jedes Element ein Inverses hat. Zugleich ein assoziativer Loop.
pub trait Group: Monoid + Loop + AssociativeQuasigroup + Groupoid {}

// --- Zusatzeigenschaften ---

/// Magma, dessen Verknüpfung kommutativ ist: `a ∘ b = b ∘ a`.
pub trait Commutative: Magma + Flexible {}

/// Magma, in dem jedes Element mit sich selbst verknüpft sich selbst ergibt: `a ∘ a = a`.
pub trait Idempotent: Magma {}

/// Monoid mit kommutativer Verknüpfung.
pub trait CommutativeMonoid: Monoid + Commutative {}

/// Gruppe mit kommutativer Verknüpfung.
pub trait AbelianGroup: Group + Commutative {}

// --- Kürzbarkeit ---

/// Linkskürzbar: `x ∘ y = x ∘ z` impliziert `y = z`.
pub trait LeftCancellative: Magma {}

/// Rechtskürzbar: `y ∘ x = z ∘ x` impliziert `y = z`.
pub trait RightCancellative: Magma {}

/// Links- und rechtskürzbar.
pub trait Cancellative: LeftCancellative + RightCancellative {}

// --- Schwächere Formen der Assoziativität ---

/// Alternativ: `(x∘x)∘y = x∘(x∘y)` und `x∘(y∘y) = (x∘y)∘y`.
pub trait Alternative: Magma {}

/// Flexibel: `(x∘y)∘x = x∘(y∘x)`.
pub trait Flexible: Magma {}

/// Potenz-assoziativ: Das von jedem Element erzeugte Untermagma ist assoziativ.
pub trait PowerAssociative: Magma {}

// --- Mediale Familie ---

/// Trimedial: Je drei (nicht notwendig verschiedene) Elemente erzeugen ein mediales Untermagma.
pub trait Trimedial: Magma {}

/// Medial: `(x∘y)∘(u∘z) = (x∘u)∘(y∘z)`. Jedes Untermagma ist dann ebenfalls medial.
pub trait Medial: Trimedial {}

/// Entropisch: homomorphes Bild eines medialen, kürzbaren Magmas. Insbesondere medial.
pub trait Entropic: Medial {}

/// Linkssemimedial: `(x∘x)∘(y∘z) = (x∘y)∘(x∘z)`.
pub trait LeftSemimedial: Magma {}

/// Rechtssemimedial: `(y∘z)∘(x∘x) = (y∘x)∘(z∘x)`.
pub trait RightSemimedial: Magma {}

/// Semimedial: links- und rechtssemimedial.
pub trait Semimedial: LeftSemimedial + RightSemimedial {}

// --- Selbstdistributivität (die Verknüpfung distribuiert über sich selbst) ---

/// Linksdistributiv: `x∘(y∘z) = (x∘y)∘(x∘z)`.
pub trait LeftDistributive: Magma {}

/// Rechtsdistributiv: `(y∘z)∘x = (y∘x)∘(z∘x)`.
pub trait RightDistributive: Magma {}

/// Autodistributiv: links- und rechtsdistributiv.
pub trait Autodistributive: LeftDistributive + RightDistributive {}

// --- Potenz-Eigenschaften ---

/// Unipotent: `x∘x = y∘y`. Alle Quadrate sind gleich.
pub trait Unipotent: Magma {}

/// Nullpotent: `(x∘x)∘y = x∘x = y∘(x∘x)`. Quadrate absorbieren.
pub trait Zeropotent: Magma {}

// --- Konstante Verknüpfungen ---

/// Links-unar: `x∘y = x∘z`. Das Ergebnis hängt nur vom linken Operanden ab.
pub trait LeftUnar: Magma {}

/// Rechts-unar: `y∘x = z∘x`. Das Ergebnis hängt nur vom rechten Operanden ab.
pub trait RightUnar: Magma {}

/// Nullhalbgruppe: `x∘y = u∘v`. Alle Produkte sind gleich.
pub trait NullSemigroup: Semigroup + LeftUnar + RightUnar {}

/// Halbgruppe mit Linksnullen: `x∘y = x`.
pub trait LeftZeroSemigroup: Semigroup {}

/// Halbgruppe mit Rechtsnullen: `y∘x = x`.
pub trait RightZeroSemigroup: Semigroup {}

// --- Sonstiges ---

/// Zentral: `(x∘y)∘(y∘z) = y`.
pub trait Central: Magma {}
