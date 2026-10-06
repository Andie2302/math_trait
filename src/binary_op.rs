//! Die allgemeinste Form einer binären Verknüpfung.

use crate::Operation;

/// Eine binäre Verknüpfung `Self × Rhs → Output` bezüglich der Operation `Op`.
///
/// Weder Abgeschlossenheit (`Output == Self`) noch Homogenität
/// (`Rhs == Self`) werden verlangt. Das ist die Stufe unterhalb des
/// [`Magma`](crate::Magma) und die Basis für alles, was keine reine
/// „Menge mit Verknüpfung“ ist (Skalarmultiplikation, Torsoren, ...).
///
/// Argumente werden per Referenz übergeben, damit auch nicht-`Copy`- und
/// nicht-`Clone`-Typen verknüpft werden können.
pub trait BinaryOp<Op: Operation, Rhs: ?Sized = Self> {
    /// Ergebnistyp der Verknüpfung.
    type Output;

    /// Verknüpft `self` mit `rhs`.
    fn apply(&self, rhs: &Rhs) -> Self::Output;
}
