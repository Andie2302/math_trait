//! Makros, die sonst kein Test benutzt: `impl_commutative_magma!`, `impl_semigroup!`,
//! `impl_commutative_semigroup!`, `impl_division_ring!`, `impl_alternative_algebra!`,
//! `impl_division_algebra!` und `impl_field_algebra!` mit Typparametern.

use math_trait::*;

mod common;
use common::*;

// --- impl_commutative_magma!: kommutativ, sonst nichts -------------------------------------------

struct Mean;
#[derive(Debug, Clone, Copy, PartialEq)]
struct M5(u8);
// op(a, b) = 3 (a + b): kommutativ, aber nicht assoziativ
impl_commutative_magma!(M5, Mean; op(a, b) { M5((3 * (a.0 + b.0)) % 5) });

#[test]
fn commutative_magma_is_commutative_and_not_more() {
    fn takes<T: Commutative<O>, O>() {}
    takes::<M5, Mean>();
    let op = |a: &M5, b: &M5| <M5 as Magma<Mean>>::op(a, b);
    let mut non_associative = false;
    for a in 0..5u8 {
        for b in 0..5u8 {
            assert_eq!(op(&M5(a), &M5(b)), op(&M5(b), &M5(a)));
            for c in 0..5u8 {
                non_associative |=
                    op(&op(&M5(a), &M5(b)), &M5(c)) != op(&M5(a), &op(&M5(b), &M5(c)));
            }
        }
    }
    assert!(non_associative, "das Beispiel ist bewusst nicht assoziativ");
}

// --- impl_semigroup!: assoziativ, nicht kommutativ ------------------------------------------------

struct Cat;
#[derive(Debug, Clone, PartialEq)]
struct Word(String);
impl_semigroup!(Word, Cat; op(a, b) { Word(format!("{}{}", a.0, b.0)) });

#[test]
fn semigroup_is_associative_but_may_be_noncommutative() {
    fn takes<T: Semigroup<O>, O>() {}
    takes::<Word, Cat>();
    let op = |a: &Word, b: &Word| <Word as Magma<Cat>>::op(a, b);
    let (x, y, w) = (Word("a".into()), Word("b".into()), Word("c".into()));
    assert_eq!(op(&op(&x, &y), &w), op(&x, &op(&y, &w)));
    assert_ne!(op(&x, &y), op(&y, &x));
}

// --- impl_commutative_semigroup!: medial und semimedial ------------------------------------------

struct Times;
#[derive(Debug, Clone, Copy, PartialEq)]
struct T5(u8);
impl_commutative_semigroup!(T5, Times; op(a, b) { T5((a.0 * b.0) % 5) });

#[test]
fn commutative_semigroup_is_medial_and_semimedial() {
    fn takes<T: CommutativeSemigroup<O> + Medial<O> + Trimedial<O> + Semimedial<O>, O>() {}
    takes::<T5, Times>();
    let op = |a: &T5, b: &T5| <T5 as Magma<Times>>::op(a, b);
    let all: Vec<T5> = (0..5).map(T5).collect();
    for x in &all {
        for y in &all {
            for u in &all {
                for z in &all {
                    // medial: (xy)(uz) = (xu)(yz)
                    assert_eq!(op(&op(x, y), &op(u, z)), op(&op(x, u), &op(y, z)));
                    // linkssemimedial: (xx)(yz) = (xy)(xz)
                    assert_eq!(op(&op(x, x), &op(y, z)), op(&op(x, y), &op(x, z)));
                    // rechtssemimedial: (yz)(xx) = (yx)(zx)
                    assert_eq!(op(&op(y, z), &op(x, x)), op(&op(y, x), &op(z, x)));
                }
            }
        }
    }
}

// --- impl_division_ring!: div und div_left unterscheiden sich ------------------------------------

/// 2×2-Matrizen über ℤ/5 mit `recip = Inverse` (und `None` für singuläre). Das ist *kein* echter
/// Schiefkörper (es gibt Nullteiler), aber ein nichtkommutatives Beispiel für die Standardmethoden
/// `div` und `div_left`: dort sind `a ⋅ b⁻¹` und `b⁻¹ ⋅ a` verschieden.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Mat2([Z5; 4]);

fn zadd(a: Z5, b: Z5) -> Z5 {
    <Z5 as Magma<Additive>>::op(&a, &b)
}
fn zmul(a: Z5, b: Z5) -> Z5 {
    <Z5 as Magma<Multiplicative>>::op(&a, &b)
}
fn zneg(a: Z5) -> Z5 {
    <Z5 as Group<Additive>>::inverse(&a)
}
fn det(m: &Mat2) -> Z5 {
    zadd(zmul(m.0[0], m.0[3]), zneg(zmul(m.0[1], m.0[2])))
}
fn mat_inverse(m: &Mat2) -> Option<Mat2> {
    let r = <Z5 as DivisionRing>::recip(&det(m))?;
    Some(Mat2([
        zmul(r, m.0[3]),
        zneg(zmul(r, m.0[1])),
        zneg(zmul(r, m.0[2])),
        zmul(r, m.0[0]),
    ]))
}

impl_division_ring!(Mat2, Additive, Multiplicative;
    add(a, b) { Mat2(std::array::from_fn(|i| zadd(a.0[i], b.0[i]))) }
    zero() { Mat2([Z5(0); 4]) }
    neg(a) { Mat2(std::array::from_fn(|i| zneg(a.0[i]))) }
    mul(a, b) {
        Mat2([
            zadd(zmul(a.0[0], b.0[0]), zmul(a.0[1], b.0[2])),
            zadd(zmul(a.0[0], b.0[1]), zmul(a.0[1], b.0[3])),
            zadd(zmul(a.0[2], b.0[0]), zmul(a.0[3], b.0[2])),
            zadd(zmul(a.0[2], b.0[1]), zmul(a.0[3], b.0[3])),
        ])
    }
    one() { Mat2([Z5(1), Z5(0), Z5(0), Z5(1)]) }
    recip(a) { mat_inverse(a) }
);

#[test]
fn division_from_the_right_and_from_the_left_differ_without_commutativity() {
    fn takes<T: DivisionRing>() {}
    takes::<Mat2>();
    let mmul = |a: &Mat2, b: &Mat2| <Mat2 as Magma<Multiplicative>>::op(a, b);
    let a = Mat2([Z5(1), Z5(1), Z5(0), Z5(1)]);
    let b = Mat2([Z5(1), Z5(0), Z5(1), Z5(1)]);
    let b_inv = <Mat2 as DivisionRing>::recip(&b).expect("b ist invertierbar");
    let right = <Mat2 as DivisionRing>::div(&a, &b).expect("Some");
    let left = <Mat2 as DivisionRing>::div_left(&a, &b).expect("Some");
    assert_eq!(right, mmul(&a, &b_inv), "div = a ⋅ b⁻¹");
    assert_eq!(left, mmul(&b_inv, &a), "div_left = b⁻¹ ⋅ a");
    assert_ne!(right, left, "die Matrizen kommutieren nicht");
    // durch eine singuläre Matrix gibt es keine Division
    let singular = Mat2([Z5(1), Z5(1), Z5(1), Z5(1)]);
    assert_eq!(<Mat2 as DivisionRing>::div(&a, &singular), None);
    assert_eq!(<Mat2 as DivisionRing>::div_left(&a, &singular), None);
}

// --- impl_alternative_algebra! und impl_division_algebra! mit eigenem Produkt-Etikett ------------

/// Etikett für das Produkt der Algebra `W` (nicht `Multiplicative`).
struct Dot;

/// `ℤ/5` als Algebra über sich selbst, aber mit dem Produkt-Etikett `Dot`.
#[derive(Debug, Clone, Copy, PartialEq)]
struct W(Z5);

impl_abelian_group!(W, Additive;
    op(a, b) { W(zadd(a.0, b.0)) }
    identity() { W(Z5(0)) }
    inverse(a) { W(zneg(a.0)) }
);
impl_module!(W, Z5; act(s, x) { W(zmul(*s, x.0)) });
impl_monoid!(W, Dot;
    op(a, b) { W(zmul(a.0, b.0)) }
    identity() { W(Z5(1)) }
);
impl_algebra!(W, Z5, Dot);
impl_unital_algebra!(W, Z5, Dot);
impl_associative_algebra!(W, Z5, Dot);
impl_alternative_algebra!(W, Z5, Dot);
impl_division_algebra!(W, Z5, Dot; recip(x) { <Z5 as DivisionRing>::recip(&x.0).map(W) });

#[test]
fn division_algebra_with_its_own_product_label() {
    type Act = ScalarMultiplication;
    fn takes<A: DivisionAlgebra<Z5, Additive, Multiplicative, Act, Dot>>() {}
    fn takes_alt<A: AlternativeAlgebra<Z5, Additive, Multiplicative, Act, Dot>>() {}
    takes::<W>();
    takes_alt::<W>();
    let dot = |a: &W, b: &W| <W as Magma<Dot>>::op(a, b);
    assert_eq!(
        <W as DivisionAlgebra<Z5, Additive, Multiplicative, Act, Dot>>::recip(&W(Z5(0))),
        None
    );
    for a in 1..5u8 {
        let x = W(Z5(a));
        let inv = <W as DivisionAlgebra<Z5, Additive, Multiplicative, Act, Dot>>::recip(&x)
            .expect("nichts außer 0 hat keinen Kehrwert");
        assert_eq!(dot(&x, &inv), <W as UnitalMagma<Dot>>::identity());
    }
}

// --- impl_field_algebra! mit Typparametern ----------------------------------------------------

/// `ℤ/P` für eine Primzahl `P`, als generischer Körper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Zp<const P: u32>(u32);

fn pow_mod(mut base: u64, mut exp: u32, p: u64) -> u64 {
    let mut acc = 1;
    base %= p;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = acc * base % p;
        }
        base = base * base % p;
        exp >>= 1;
    }
    acc
}

impl_field!(for [const P: u32] Zp<P>, Additive, Multiplicative;
    add(a, b) { Zp((a.0 + b.0) % P) }
    zero() { Zp(0) }
    neg(a) { Zp((P - a.0) % P) }
    mul(a, b) { Zp((u64::from(a.0) * u64::from(b.0) % u64::from(P)) as u32) }
    one() { Zp(1 % P) }
    recip(a) {
        if a.0 == 0 { None } else { Some(Zp(pow_mod(u64::from(a.0), P - 2, u64::from(P)) as u32)) }
    }
);
impl_field_algebra!(for [const P: u32] Zp<P>);

#[test]
fn a_generic_field_becomes_an_algebra_over_itself() {
    fn takes<A: CompositionAlgebra<K> + DivisionAlgebra<K> + AssociativeAlgebra<K>, K: Field>() {}
    takes::<Zp<7>, Zp<7>>();
    takes::<Zp<11>, Zp<11>>();
    // und damit beginnt der Cayley-Dickson-Turm über Zp<7>
    let c: CayleyDickson<Zp<7>, Zp<7>> = CayleyDickson::new(Zp(2), Zp(3));
    let conj = <CayleyDickson<Zp<7>, Zp<7>> as Involutive>::conjugate(&c);
    assert_eq!(conj, CayleyDickson::new(Zp(2), Zp(4)), "(a, b)* = (a, −b)");
    // der Kehrwert in Zp<7>: 3 ⋅ 5 = 15 = 1
    assert_eq!(<Zp<7> as DivisionRing>::recip(&Zp(3)), Some(Zp(5)));
}
