//! Die in der Doku genannten Grenzen der Geometrie: in Charakteristik zwei und bei ausgearteten
//! Formen verschwindet die Vektordarstellung von `so(N)`, und in Charakteristik drei bleibt sie
//! treu.

use math_trait::*;

mod common;
use common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct F2(u8);
impl_field!(F2, Additive, Multiplicative;
    add(a, b) { F2((a.0 + b.0) % 2) }
    zero() { F2(0) }
    neg(a) { F2(a.0) }
    mul(a, b) { F2((a.0 * b.0) % 2) }
    one() { F2(1) }
    recip(a) { if a.0 == 0 { None } else { Some(F2(1)) } }
);
impl_field_algebra!(F2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct F3(u8);
impl_field!(F3, Additive, Multiplicative;
    add(a, b) { F3((a.0 + b.0) % 3) }
    zero() { F3(0) }
    neg(a) { F3((3 - a.0) % 3) }
    mul(a, b) { F3((a.0 * b.0) % 3) }
    one() { F3(1) }
    recip(a) { match a.0 { 1 => Some(F3(1)), 2 => Some(F3(2)), _ => None } }
);
impl_field_algebra!(F3);

struct OnesF2;
impl DiagonalForm<F2> for OnesF2 {
    fn square(_: usize) -> F2 {
        F2(1)
    }
}
struct OnesF3;
impl DiagonalForm<F3> for OnesF3 {
    fn square(_: usize) -> F3 {
        F3(1)
    }
}
/// `q = (1, 1, 0)`: genau ein ausgearteter Erzeuger.
struct OneZero;
impl DiagonalForm<Z5> for OneZero {
    fn square(i: usize) -> Z5 {
        if i == 2 { Z5(0) } else { Z5(1) }
    }
}
struct Zeros;
impl DiagonalForm<Z5> for Zeros {
    fn square(_: usize) -> Z5 {
        Z5(0)
    }
}

/// Alle Bivektoren von `N = 3` (D = 8) über einem Körper mit `p` Elementen, aus den Koeffizienten
/// von `e₀e₁`, `e₀e₂`, `e₁e₂`.
fn bivectors<R, Q>(p: usize, elem: impl Fn(usize) -> R) -> Vec<Bivector<R, 8, Q>>
where
    R: CommutativeRing + PartialEq + Copy,
    Q: DiagonalForm<R>,
{
    (0..p * p * p)
        .map(|i| {
            let mut c = [elem(0); 8];
            c[0b011] = elem(i);
            c[0b101] = elem(i / p);
            c[0b110] = elem(i / (p * p));
            Bivector::new(Clifford::new(c)).expect("Grad 2")
        })
        .collect()
}

/// Wie viele verschiedene Matrizen die Vektordarstellung von `so(3)` annimmt.
fn distinct_vector_actions<R, Q>(bs: &[Bivector<R, 8, Q>]) -> usize
where
    R: CommutativeRing + PartialEq + Copy + std::hash::Hash + Eq,
    Q: DiagonalForm<R>,
{
    let basis: [Vector<R, 3>; 3] = std::array::from_fn(Vector::basis);
    let key = |b: &Bivector<R, 8, Q>| -> Vec<R> {
        basis
            .iter()
            .flat_map(|e| {
                <Vector<R, 3> as LieModule<Bivector<R, 8, Q>, R>>::lie_act(b, e).into_coords()
            })
            .collect()
    };
    bs.iter()
        .map(key)
        .collect::<std::collections::HashSet<_>>()
        .len()
}

#[test]
fn in_characteristic_two_so_n_is_abelian_and_acts_trivially() {
    let bs = bivectors::<F2, OnesF2>(2, |i| F2((i % 2) as u8));
    assert_eq!(bs.len(), 8);
    assert_eq!(distinct_vector_actions(&bs), 1, "nur die Nulldarstellung");
    let e01 = Bivector::<F2, 8, OnesF2>::basis(0, 1);
    let e12 = Bivector::<F2, 8, OnesF2>::basis(1, 2);
    let bracket = <Bivector<F2, 8, OnesF2> as Magma<Bracket>>::op(&e01, &e12);
    assert_eq!(
        bracket,
        <Bivector<F2, 8, OnesF2> as UnitalMagma<Additive>>::identity()
    );
}

#[test]
fn in_characteristic_three_the_vector_representation_is_faithful() {
    let bs = bivectors::<F3, OnesF3>(3, |i| F3((i % 3) as u8));
    assert_eq!(bs.len(), 27);
    assert_eq!(distinct_vector_actions(&bs), 27);
    let e01 = Bivector::<F3, 8, OnesF3>::basis(0, 1);
    let e12 = Bivector::<F3, 8, OnesF3>::basis(1, 2);
    let bracket = <Bivector<F3, 8, OnesF3> as Magma<Bracket>>::op(&e01, &e12);
    assert_ne!(
        bracket,
        <Bivector<F3, 8, OnesF3> as UnitalMagma<Additive>>::identity()
    );
}

#[test]
fn a_degenerate_form_makes_the_vector_representation_unfaithful() {
    // alle qᵢ = 0: [B, v] = 0 für alle Bivektoren
    let bs = bivectors::<Z5, Zeros>(5, z);
    assert_eq!(bs.len(), 125);
    assert_eq!(distinct_vector_actions(&bs), 1);
    // bei nur einem verschwindenden qᵢ (hier q = (1, 1, 0)) und bei q = (1, 1, 1) bleibt sie treu
    let bs = bivectors::<Z5, OneZero>(5, z);
    assert_eq!(distinct_vector_actions(&bs), 125);
    let bs = bivectors::<Z5, Ones>(5, z);
    assert_eq!(distinct_vector_actions(&bs), 125);
}
