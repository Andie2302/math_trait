//! Quasigruppe, Loop (ohne Assoziativität), *-Ring und Kompositionsalgebra aus Makros.

use math_trait::*;

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

fn add(a: &Z5, b: &Z5) -> Z5 {
    <Z5 as Magma<Additive>>::op(a, b)
}
fn mul(a: &Z5, b: &Z5) -> Z5 {
    <Z5 as Magma<Multiplicative>>::op(a, b)
}

// --- Quasigruppe: a ∘ b = 2a + 4b (mod 5), idempotent und nicht assoziativ --------------------

struct Q;
#[derive(Debug, Clone, Copy, PartialEq)]
struct Qg(u8);

impl_quasigroup!(Qg, Q;
    op(a, b) { Qg((2 * a.0 + 4 * b.0) % 5) }
    ldiv(a, b) { Qg((4 * (b.0 + 5 - (2 * a.0) % 5)) % 5) }
    rdiv(a, b) { Qg((3 * (b.0 + 5 - (4 * a.0) % 5)) % 5) }
);

#[test]
fn quasigroup_division_solves_the_equations() {
    let op = |a: &Qg, b: &Qg| <Qg as Magma<Q>>::op(a, b);
    for a in 0..5 {
        for b in 0..5 {
            let (a, b) = (Qg(a), Qg(b));
            let x = <Qg as Quasigroup<Q>>::ldiv(&a, &b);
            let y = <Qg as Quasigroup<Q>>::rdiv(&a, &b);
            assert_eq!(op(&a, &x), b);
            assert_eq!(op(&y, &a), b);
        }
    }
    // nicht assoziativ
    let (a, b, c) = (Qg(1), Qg(2), Qg(3));
    assert_ne!(op(&op(&a, &b), &c), op(&a, &op(&b, &c)));
}

// --- Loop der Ordnung 5, der keine Gruppe ist ---------------------------------------------------

struct L;
#[derive(Debug, Clone, Copy, PartialEq)]
struct Lp(usize);

const TABLE: [[usize; 5]; 5] = [
    [0, 1, 2, 3, 4],
    [1, 0, 3, 4, 2],
    [2, 4, 0, 1, 3],
    [3, 2, 4, 0, 1],
    [4, 3, 1, 2, 0],
];

impl_loop!(Lp, L;
    op(a, b) { Lp(TABLE[a.0][b.0]) }
    identity() { Lp(0) }
    ldiv(a, b) { Lp((0..5).find(|&x| TABLE[a.0][x] == b.0).unwrap()) }
    rdiv(a, b) { Lp((0..5).find(|&y| TABLE[y][a.0] == b.0).unwrap()) }
);

#[test]
fn a_loop_has_identity_and_division_but_is_not_associative() {
    fn takes_loop<X: Loop<L>>() {}
    takes_loop::<Lp>();
    let op = |a: &Lp, b: &Lp| <Lp as Magma<L>>::op(a, b);
    let id = <Lp as UnitalMagma<L>>::identity();
    for a in 0..5 {
        let a = Lp(a);
        assert_eq!(op(&id, &a), a);
        assert_eq!(op(&a, &id), a);
        for b in 0..5 {
            let b = Lp(b);
            assert_eq!(op(&a, &<Lp as Quasigroup<L>>::ldiv(&a, &b)), b);
            assert_eq!(op(&<Lp as Quasigroup<L>>::rdiv(&a, &b), &a), b);
        }
    }
    // (1∘1)∘2 = 0∘2 = 2,  1∘(1∘2) = 1∘3 = 4
    let (a, b) = (Lp(1), Lp(2));
    assert_ne!(op(&op(&a, &a), &b), op(&a, &op(&a, &b)));
}

// --- *-Ring: Z5 × Z5 mit der Vertauschung als Involution ----------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pair(Z5, Z5);

impl_commutative_ring!(Pair, Additive, Multiplicative;
    add(a, b) { Pair(add(&a.0, &b.0), add(&a.1, &b.1)) }
    zero() { Pair(Z5(0), Z5(0)) }
    neg(a) { Pair(<Z5 as Group<Additive>>::inverse(&a.0), <Z5 as Group<Additive>>::inverse(&a.1)) }
    mul(a, b) { Pair(mul(&a.0, &b.0), mul(&a.1, &b.1)) }
    one() { Pair(Z5(1), Z5(1)) }
);
impl_star_ring!(Pair; conjugate(x) { Pair(x.1, x.0) });

#[test]
fn star_ring_involution_laws() {
    fn takes_star<R: StarRing>() {}
    takes_star::<Pair>();
    let c = |x: &Pair| <Pair as Involutive>::conjugate(x);
    let (m, a) = (
        |x: &Pair, y: &Pair| <Pair as Magma<Multiplicative>>::op(x, y),
        |x: &Pair, y: &Pair| <Pair as Magma<Additive>>::op(x, y),
    );
    for i in 0..25u8 {
        for j in 0..25u8 {
            let (x, y) = (Pair(Z5(i % 5), Z5(i / 5)), Pair(Z5(j % 5), Z5(j / 5)));
            assert_eq!(c(&c(&x)), x);
            assert_eq!(c(&m(&x, &y)), m(&c(&y), &c(&x)));
            assert_eq!(c(&a(&x, &y)), a(&c(&x), &c(&y)));
        }
    }
}

// --- Kompositionsalgebra: Z5[i] mit i² = −1 -----------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cx(Z5, Z5); // a + b·i

fn neg(a: &Z5) -> Z5 {
    <Z5 as Group<Additive>>::inverse(a)
}

impl_abelian_group!(Cx, Additive;
    op(x, y) { Cx(add(&x.0, &y.0), add(&x.1, &y.1)) }
    identity() { Cx(Z5(0), Z5(0)) }
    inverse(x) { Cx(neg(&x.0), neg(&x.1)) }
);
impl_module!(Cx, Z5; act(s, x) { Cx(mul(s, &x.0), mul(s, &x.1)) });
impl_commutative_monoid!(Cx, Multiplicative;
    op(x, y) {
        Cx(
            add(&mul(&x.0, &y.0), &neg(&mul(&x.1, &y.1))),
            add(&mul(&x.0, &y.1), &mul(&x.1, &y.0)),
        )
    }
    identity() { Cx(Z5(1), Z5(0)) }
);
impl_algebra!(Cx, Z5, Multiplicative);
impl_unital_algebra!(Cx, Z5, Multiplicative);
impl_associative_algebra!(Cx, Z5, Multiplicative);
impl_algebra_with_involution!(Cx, Z5, Multiplicative; conjugate(x) { Cx(x.0, neg(&x.1)) });
impl_composition_algebra!(Cx, Z5; norm(x) { add(&mul(&x.0, &x.0), &mul(&x.1, &x.1)) });

#[test]
fn composition_algebra_from_macros_has_a_multiplicative_norm() {
    fn takes_comp<A: CompositionAlgebra<K>, K: Field>() {}
    takes_comp::<Cx, Z5>();
    let n = |x: &Cx| <Cx as QuadraticForm<Z5, Norm>>::value(x);
    let m = |x: &Cx, y: &Cx| <Cx as Magma<Multiplicative>>::op(x, y);
    for i in 0..25u8 {
        for j in 0..25u8 {
            let (x, y) = (Cx(Z5(i % 5), Z5(i / 5)), Cx(Z5(j % 5), Z5(j / 5)));
            assert_eq!(n(&m(&x, &y)), mul(&n(&x), &n(&y)));
        }
    }
}
