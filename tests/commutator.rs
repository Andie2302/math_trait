//! Kommutator-Konstruktion: Aus 2×2-Matrizen über ℤ/5 (assoziativ, nicht kommutativ)
//! wird mit `[A, B] = AB − BA` eine Lie-Algebra, hier mit den Relationen von sl(2).

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

fn add(a: Z5, b: Z5) -> Z5 {
    <Z5 as Magma<Additive>>::op(&a, &b)
}
fn mul(a: Z5, b: Z5) -> Z5 {
    <Z5 as Magma<Multiplicative>>::op(&a, &b)
}
fn neg(a: Z5) -> Z5 {
    <Z5 as Group<Additive>>::inverse(&a)
}

// 2×2-Matrizen [[a, b], [c, d]]
#[derive(Debug, Clone, Copy, PartialEq)]
struct Mat2(Z5, Z5, Z5, Z5);

fn m(a: u8, b: u8, c: u8, d: u8) -> Mat2 {
    Mat2(Z5(a), Z5(b), Z5(c), Z5(d))
}

impl_abelian_group!(Mat2, Additive;
    op(x, y) { Mat2(add(x.0, y.0), add(x.1, y.1), add(x.2, y.2), add(x.3, y.3)) }
    identity() { m(0, 0, 0, 0) }
    inverse(x) { Mat2(neg(x.0), neg(x.1), neg(x.2), neg(x.3)) }
);

impl_module!(Mat2, Z5; act(s, x) {
    Mat2(mul(*s, x.0), mul(*s, x.1), mul(*s, x.2), mul(*s, x.3))
});

impl_monoid!(Mat2, Multiplicative;
    op(x, y) {
        Mat2(
            add(mul(x.0, y.0), mul(x.1, y.2)),
            add(mul(x.0, y.1), mul(x.1, y.3)),
            add(mul(x.2, y.0), mul(x.3, y.2)),
            add(mul(x.2, y.1), mul(x.3, y.3)),
        )
    }
    identity() { m(1, 0, 0, 1) }
);

impl_algebra!(Mat2, Z5, Multiplicative);
impl AssociativeAlgebra<Z5> for Mat2 {}

type L = Commutator<Mat2, Z5>;

fn br(x: &L, y: &L) -> L {
    <L as Magma<Bracket>>::op(x, y)
}
fn ladd(x: &L, y: &L) -> L {
    <L as Magma<Additive>>::op(x, y)
}
fn lie(a: u8, b: u8, c: u8, d: u8) -> L {
    L::new(m(a, b, c, d))
}

fn takes_lie<X: LieAlgebra<K>, K: CommutativeRing>() {}
fn takes_vector_space<V: VectorSpace<K>, K: Field>() {}

#[test]
fn commutator_is_a_lie_algebra_over_z5() {
    takes_lie::<L, Z5>();
    takes_vector_space::<L, Z5>();
}

#[test]
fn matrix_product_is_not_commutative_but_bracket_measures_it() {
    let (e12, e21) = (m(0, 1, 0, 0), m(0, 0, 1, 0));
    let mulm = |a: &Mat2, b: &Mat2| <Mat2 as Magma<Multiplicative>>::op(a, b);
    assert_ne!(mulm(&e12, &e21), mulm(&e21, &e12));
}

#[test]
fn sl2_relations() {
    let (e, f, h) = (lie(0, 1, 0, 0), lie(0, 0, 1, 0), lie(1, 0, 0, 4));
    // [e, f] = h,  [h, e] = 2e,  [h, f] = −2f
    assert_eq!(br(&e, &f), h);
    assert_eq!(br(&h, &e), lie(0, 2, 0, 0));
    assert_eq!(br(&h, &f), lie(0, 0, 3, 0));
}

fn sample() -> Vec<L> {
    let mut v = Vec::new();
    for i in 0..12u8 {
        v.push(lie(i % 5, (i * 2 + 1) % 5, (i * 3 + 2) % 5, (i / 2) % 5));
    }
    v.push(lie(1, 0, 0, 1)); // Einheitsmatrix: liegt im Zentrum
    v
}

#[test]
fn bracket_is_alternating_and_satisfies_jacobi() {
    let zero = <L as UnitalMagma<Additive>>::identity();
    let s = sample();
    for x in &s {
        assert_eq!(br(x, x), zero);
        for y in &s {
            for z in &s {
                let sum = ladd(
                    &ladd(&br(x, &br(y, z)), &br(y, &br(z, x))),
                    &br(z, &br(x, y)),
                );
                assert_eq!(sum, zero);
            }
        }
    }
}

#[test]
fn identity_matrix_is_central_and_bracket_is_not_associative() {
    let zero = <L as UnitalMagma<Additive>>::identity();
    let id = lie(1, 0, 0, 1);
    for x in sample() {
        assert_eq!(br(&id, &x), zero);
    }
    let (e, f) = (lie(0, 1, 0, 0), lie(0, 0, 1, 0));
    assert_ne!(br(&br(&e, &e), &f), br(&e, &br(&e, &f)));
}

#[test]
fn addition_and_scalars_are_those_of_the_algebra() {
    let x = lie(1, 2, 3, 4);
    let y = <L as LeftAction<Z5>>::act(&Z5(2), &x);
    assert_eq!(y, lie(2, 4, 1, 3));
    assert_eq!(
        ladd(&x, &<L as Group<Additive>>::inverse(&x)),
        lie(0, 0, 0, 0)
    );
    assert_eq!(x.as_inner(), &m(1, 2, 3, 4));
}
