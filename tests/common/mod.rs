//! Gemeinsame Bausteine für neue Tests: der Körper ℤ/5 und ein paar Diagonalformen.
//!
//! Ältere Testdateien haben noch eigene Kopien davon (siehe TODO.md).
#![allow(dead_code)]

use math_trait::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Z5(pub u8);

impl_field!(Z5, Additive, Multiplicative;
    add(a, b) { Z5((a.0 + b.0) % 5) }
    zero() { Z5(0) }
    neg(a) { Z5((5 - a.0) % 5) }
    mul(a, b) { Z5((a.0 * b.0) % 5) }
    one() { Z5(1) }
    recip(a) { [None, Some(Z5(1)), Some(Z5(3)), Some(Z5(2)), Some(Z5(4))][a.0 as usize] }
);
impl_field_algebra!(Z5);

/// `a mod 5` als Element von ℤ/5.
pub fn z(a: usize) -> Z5 {
    Z5((a % 5) as u8)
}

/// Alle Quadrate gleich 1.
pub struct Ones;
impl DiagonalForm<Z5> for Ones {
    fn square(_: usize) -> Z5 {
        Z5(1)
    }
}

/// Alle Quadrate gleich −1.
pub struct MinusOnes;
impl DiagonalForm<Z5> for MinusOnes {
    fn square(_: usize) -> Z5 {
        Z5(4)
    }
}

/// `q = (1, 2, 3, 4, 0, 1, 2, …)`: jede Koordinate ein anderes Gewicht.
pub struct Weights;
impl DiagonalForm<Z5> for Weights {
    fn square(i: usize) -> Z5 {
        z(i + 1)
    }
}

/// `q = (1, 1, 1, 1, 1, 2)` für den Fall `N = 6`.
pub struct Six;
impl DiagonalForm<Z5> for Six {
    fn square(i: usize) -> Z5 {
        if i == 5 { Z5(2) } else { Z5(1) }
    }
}

pub fn cmul<const D: usize, Q: DiagonalForm<Z5>>(
    a: &Clifford<Z5, D, Q>,
    b: &Clifford<Z5, D, Q>,
) -> Clifford<Z5, D, Q> {
    <Clifford<Z5, D, Q> as Magma<Multiplicative>>::op(a, b)
}

pub fn cadd<const D: usize, Q: DiagonalForm<Z5>>(
    a: &Clifford<Z5, D, Q>,
    b: &Clifford<Z5, D, Q>,
) -> Clifford<Z5, D, Q> {
    <Clifford<Z5, D, Q> as Magma<Additive>>::op(a, b)
}

pub fn cneg<const D: usize, Q: DiagonalForm<Z5>>(a: &Clifford<Z5, D, Q>) -> Clifford<Z5, D, Q> {
    <Clifford<Z5, D, Q> as Group<Additive>>::inverse(a)
}
