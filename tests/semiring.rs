//! Halbringe: Ringe ohne additive Inverse. Drei Beispiele, zwei davon keine Ringe.

use math_trait::*;

/// Prüft alle Halbring-Gesetze auf allen Tripeln der Stichprobe.
fn semiring_laws<S, A, M>(samples: &[S])
where
    S: Semiring<A, M> + PartialEq + core::fmt::Debug,
{
    let add = |a: &S, b: &S| <S as Magma<A>>::op(a, b);
    let mul = |a: &S, b: &S| <S as Magma<M>>::op(a, b);
    let zero = <S as UnitalMagma<A>>::identity();
    let one = <S as UnitalMagma<M>>::identity();
    for x in samples {
        assert_eq!(add(x, &zero), *x);
        assert_eq!(mul(x, &one), *x);
        assert_eq!(mul(&one, x), *x);
        assert_eq!(mul(x, &zero), zero, "Null absorbiert");
        assert_eq!(mul(&zero, x), zero, "Null absorbiert");
        for y in samples {
            assert_eq!(add(x, y), add(y, x), "Addition kommutativ");
            for z in samples {
                assert_eq!(add(&add(x, y), z), add(x, &add(y, z)));
                assert_eq!(mul(&mul(x, y), z), mul(x, &mul(y, z)));
                assert_eq!(mul(x, &add(y, z)), add(&mul(x, y), &mul(x, z)));
                assert_eq!(mul(&add(y, z), x), add(&mul(y, x), &mul(z, x)));
            }
        }
    }
}

// --- Wahrheitswerte: ∨ und ∧ (kein Ring: a ∨ b = 0 hat nur die Lösung a = b = 0) -------------

#[derive(Debug, Clone, Copy, PartialEq)]
struct Bool(bool);

impl_commutative_semiring!(Bool, Additive, Multiplicative;
    add(a, b) { Bool(a.0 || b.0) }
    zero() { Bool(false) }
    mul(a, b) { Bool(a.0 && b.0) }
    one() { Bool(true) }
);

#[test]
fn boolean_semiring() {
    semiring_laws::<Bool, Additive, Multiplicative>(&[Bool(false), Bool(true)]);
}

// --- Tropischer Halbring: min und + mit eigenen Etiketten ----------------------------------------

struct Min;
struct Plus;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Trop(Option<u8>); // None steht für ∞

fn trop_min(a: &Trop, b: &Trop) -> Trop {
    match (a.0, b.0) {
        (None, x) | (x, None) => Trop(x),
        (Some(x), Some(y)) => Trop(Some(x.min(y))),
    }
}
fn trop_plus(a: &Trop, b: &Trop) -> Trop {
    match (a.0, b.0) {
        (Some(x), Some(y)) => Trop(Some(x + y)),
        _ => Trop(None),
    }
}

impl_commutative_semiring!(Trop, Min, Plus;
    add(a, b) { trop_min(a, b) }
    zero() { Trop(None) }
    mul(a, b) { trop_plus(a, b) }
    one() { Trop(Some(0)) }
);

#[test]
fn tropical_semiring_with_custom_labels() {
    let samples: Vec<Trop> = std::iter::once(Trop(None))
        .chain((0..5).map(|i| Trop(Some(i))))
        .collect();
    semiring_laws::<Trop, Min, Plus>(&samples);
    // der kürzeste Weg über zwei Kanten: min(3 + 4, 2 + 6) = 7
    let (a, b, c, d) = (Trop(Some(3)), Trop(Some(4)), Trop(Some(2)), Trop(Some(6)));
    let path = trop_min(&trop_plus(&a, &b), &trop_plus(&c, &d));
    assert_eq!(path, Trop(Some(7)));
}

// --- Ringe sind Halbringe ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
struct Z5(u8);

impl_field!(Z5, Additive, Multiplicative;
    add(a, b) { Z5((a.0 + b.0) % 5) }
    zero() { Z5(0) }
    neg(a) { Z5((5 - a.0) % 5) }
    mul(a, b) { Z5((a.0 * b.0) % 5) }
    one() { Z5(1) }
    recip(a) { [None, Some(Z5(1)), Some(Z5(3)), Some(Z5(2)), Some(Z5(4))][a.0 as usize] }
);

#[test]
fn a_ring_is_a_semiring_and_so_is_a_field() {
    fn takes_semiring<S: Semiring>() {}
    fn takes_comm_semiring<S: CommutativeSemiring>() {}
    takes_semiring::<Z5>();
    takes_comm_semiring::<Z5>();
    let all: Vec<Z5> = (0..5).map(Z5).collect();
    semiring_laws::<Z5, Additive, Multiplicative>(&all);
}
