//! Das Magma: die Basis der algebraischen Hierarchie.

use crate::{BinaryOp, Operation};

/// Ein Magma: eine Menge (der implementierende Typ) mit einer
/// **abgeschlossenen** binären Verknüpfung `op: Self × Self → Self`
/// bezüglich der Operation `Op`.
///
/// Mehr wird nicht verlangt – insbesondere *keine* Assoziativität,
/// Kommutativität oder neutrales Element. Diese Eigenschaften kommen als
/// eigene Traits obendrauf.
///
/// Jedes `Magma<Op>` ist automatisch ein [`BinaryOp<Op, Self>`] mit
/// `Output = Self`.
///
/// # Beispiel
///
/// ```
/// use math_trait::{Additive, Magma};
///
/// #[derive(Debug, PartialEq)]
/// struct Mod5(u8);
///
/// impl Magma<Additive> for Mod5 {
///     fn op(&self, rhs: &Self) -> Self {
///         Mod5((self.0 + rhs.0) % 5)
///     }
/// }
///
/// assert_eq!(Magma::<Additive>::op(&Mod5(3), &Mod5(4)), Mod5(2));
/// ```
pub trait Magma<Op: Operation>: Sized {
    /// Verknüpft `self` mit `rhs`.
    fn op(&self, rhs: &Self) -> Self;

    /// Wie [`op`](Magma::op), aber verbraucht beide Operanden.
    fn op_owned(self, rhs: Self) -> Self {
        self.op(&rhs)
    }
}

impl<Op: Operation, T: Magma<Op>> BinaryOp<Op, T> for T {
    type Output = T;

    fn apply(&self, rhs: &T) -> T {
        Magma::<Op>::op(self, rhs)
    }
}
