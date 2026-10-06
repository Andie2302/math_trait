//! Ein vollständiger Körper und eine vollständige Lie-Algebra aus wenigen Zeilen.

use math_trait::*;

// --- ℤ/5 als Körper ----------------------------------------------------------

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

fn add(a: Z5, b: Z5) -> Z5 {
    <Z5 as Magma<Additive>>::op(&a, &b)
}
fn mul(a: Z5, b: Z5) -> Z5 {
    <Z5 as Magma<Multiplicative>>::op(&a, &b)
}

fn takes_field<K: Field>() {}
fn takes_comm_ring<K: CommutativeRing>() {}
fn takes_abelian<K: AbelianGroup<Additive>>() {}

#[test]
fn z5_is_a_field() {
    takes_field::<Z5>();
    takes_comm_ring::<Z5>();
    takes_abelian::<Z5>();
}

#[test]
fn z5_distributes_over_all_triples() {
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                let (a, b, c) = (Z5(a), Z5(b), Z5(c));
                assert_eq!(mul(a, add(b, c)), add(mul(a, b), mul(a, c)));
                assert_eq!(mul(add(b, c), a), add(mul(b, a), mul(c, a)));
                assert_eq!(add(add(a, b), c), add(a, add(b, c)));
                assert_eq!(mul(mul(a, b), c), mul(a, mul(b, c)));
            }
        }
    }
}

// --- ℤ/5³ mit dem Kreuzprodukt als Lie-Algebra über ℤ/5 ----------------------

#[derive(Debug, Clone, Copy, PartialEq)]
struct V3(Z5, Z5, Z5);

impl_abelian_group!(V3, Additive;
    op(a, b) { V3(add(a.0, b.0), add(a.1, b.1), add(a.2, b.2)) }
    identity() { V3(Z5(0), Z5(0), Z5(0)) }
    inverse(a) {
        let n = |x: Z5| <Z5 as Group<Additive>>::inverse(&x);
        V3(n(a.0), n(a.1), n(a.2))
    }
);

impl_module!(V3, Z5; act(s, x) { V3(mul(*s, x.0), mul(*s, x.1), mul(*s, x.2)) });

fn sub(a: Z5, b: Z5) -> Z5 {
    add(a, <Z5 as Group<Additive>>::inverse(&b))
}

impl_magma!(V3, Bracket; op(a, b) {
    V3(
        sub(mul(a.1, b.2), mul(a.2, b.1)),
        sub(mul(a.2, b.0), mul(a.0, b.2)),
        sub(mul(a.0, b.1), mul(a.1, b.0)),
    )
});

impl_lie_algebra!(V3, Z5);

fn takes_lie<L: LieAlgebra<K>, K: CommutativeRing>() {}
fn takes_vector_space<V: VectorSpace<K>, K: Field>() {}

#[test]
fn v3_is_a_lie_algebra_and_a_vector_space() {
    takes_lie::<V3, Z5>();
    takes_vector_space::<V3, Z5>();
}

#[test]
fn jacobi_holds_for_all_basis_triples_and_samples() {
    let br = |a: &V3, b: &V3| <V3 as Magma<Bracket>>::op(a, b);
    let ad = |a: &V3, b: &V3| <V3 as Magma<Additive>>::op(a, b);
    let zero = <V3 as UnitalMagma<Additive>>::identity();
    let vs: Vec<V3> = (0..5u8)
        .flat_map(|i| (0..5u8).map(move |j| V3(Z5(i), Z5(j), Z5((i * 2 + j) % 5))))
        .collect();
    for x in &vs {
        assert_eq!(br(x, x), zero);
        for y in &vs {
            for z in &vs {
                let sum = ad(&ad(&br(x, &br(y, z)), &br(y, &br(z, x))), &br(z, &br(x, y)));
                assert_eq!(sum, zero);
            }
        }
    }
}

#[test]
fn scalar_action_and_group_division() {
    let v = V3(Z5(1), Z5(2), Z5(3));
    let w = <V3 as LeftAction<Z5>>::act(&Z5(2), &v);
    assert_eq!(w, V3(Z5(2), Z5(4), Z5(1)));
    // ldiv ist aus op und inverse abgeleitet: v + x = w  =>  x = -v + w
    let x = <V3 as Quasigroup<Additive>>::ldiv(&v, &w);
    assert_eq!(<V3 as Magma<Additive>>::op(&v, &x), w);
}

#[test]
fn reciprocal_exists_exactly_for_nonzero() {
    assert_eq!(<Z5 as DivisionRing>::recip(&Z5(0)), None);
    for a in 1..5 {
        let a = Z5(a);
        let r = <Z5 as DivisionRing>::recip(&a).expect("nonzero has reciprocal");
        assert_eq!(mul(a, r), Z5(1));
        assert_eq!(mul(r, a), Z5(1));
    }
}
