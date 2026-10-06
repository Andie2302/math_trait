//! Zahlenartige Traits.
//!
//! Diese Traits sind bewusst von der strikten Mathematik getrennt: Basisdatentypen wie
//! `i32` oder `f64` erfüllen die algebraischen Gesetze nur *näherungsweise* (Überlauf,
//! Rundung, `NaN`). Das Modul sagt nur, welche Rolle ein Typ spielt, nicht, ob er ein exakter
//! Ring oder Körper ist. Exakte Typen können zusätzlich `CommutativeRing` o. Ä. implementieren.

use crate::{Additive, Commutative, Multiplicative, UnitalMagma};

/// Zahl: Addition und Multiplikation, beide kommutativ und mit neutralem Element.
///
/// Das ist, was ganze Zahlen und Gleitkommazahlen gemeinsam haben. Assoziativität und
/// Distributivität werden hier nicht verlangt, denn Gleitkommazahlen erfüllen sie nicht exakt.
pub trait Number:
    UnitalMagma<Additive>
    + UnitalMagma<Multiplicative>
    + Commutative<Additive>
    + Commutative<Multiplicative>
{
    /// Das Nullelement (neutral bezüglich `Additive`).
    fn zero() -> Self {
        <Self as UnitalMagma<Additive>>::identity()
    }

    /// Das Einselement (neutral bezüglich `Multiplicative`).
    fn one() -> Self {
        <Self as UnitalMagma<Multiplicative>>::identity()
    }
}

/// Ganzzahlig und diskret: Zahlen mit Division mit Rest (ℤ-artig).
pub trait Integer: Number {}

/// Näherung der reellen Zahlen: Zahlen mit Rundung, nicht assoziativ, mit Sonderwerten.
pub trait Float: Number {}

/// Vorzeichenbehaftet: zu jeder Zahl gibt es eine Negation (`-x`).
pub trait Signed: Number {}

/// Vorzeichenlos: keine Negation, ℕ-artig.
pub trait Unsigned: Number {}
