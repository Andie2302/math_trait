//! Die Makros mit Typparametern: das direkte Produkt `K × K` eines Körpers `K`
//! ist ein kommutativer Ring und eine assoziative, unitale Algebra über `K`.

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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pair<K>(K, K);

fn add<K: Magma<Additive>>(a: &K, b: &K) -> K {
    <K as Magma<Additive>>::op(a, b)
}
fn mul<K: Magma<Multiplicative>>(a: &K, b: &K) -> K {
    <K as Magma<Multiplicative>>::op(a, b)
}
fn neg<K: Group<Additive>>(a: &K) -> K {
    <K as Group<Additive>>::inverse(a)
}

impl_commutative_ring!(for [K: CommutativeRing] Pair<K>, Additive, Multiplicative;
    add(a, b) { Pair(add(&a.0, &b.0), add(&a.1, &b.1)) }
    zero() { Pair(<K as UnitalMagma<Additive>>::identity(), <K as UnitalMagma<Additive>>::identity()) }
    neg(a) { Pair(neg(&a.0), neg(&a.1)) }
    mul(a, b) { Pair(mul(&a.0, &b.0), mul(&a.1, &b.1)) }
    one() {
        Pair(
            <K as UnitalMagma<Multiplicative>>::identity(),
            <K as UnitalMagma<Multiplicative>>::identity(),
        )
    }
);

impl_module!(for [K: CommutativeRing] Pair<K>, K;
    act(s, x) { Pair(mul(s, &x.0), mul(s, &x.1)) }
);
impl_ring_algebra!(for [K: CommutativeRing] Pair<K>, K);
impl_unital_algebra!(for [K: CommutativeRing] Pair<K>, K, Multiplicative);
impl_associative_algebra!(for [K: CommutativeRing] Pair<K>, K, Multiplicative);

fn takes_comm_ring<R: CommutativeRing>() {}
fn takes_assoc_algebra<A: AssociativeAlgebra<K> + UnitalAlgebra<K>, K: CommutativeRing>() {}

#[test]
fn pair_has_all_the_structure() {
    takes_comm_ring::<Pair<Z5>>();
    takes_assoc_algebra::<Pair<Z5>, Z5>();
    // Das direkte Produkt zweier Körper ist kein Körper: (1,0)·(0,1) = 0 mit Nullteilern.
    let (e1, e2) = (Pair(Z5(1), Z5(0)), Pair(Z5(0), Z5(1)));
    assert_eq!(mul(&e1, &e2), Pair(Z5(0), Z5(0)));
}

#[test]
fn pair_obeys_the_ring_laws_on_all_triples() {
    let all: Vec<Pair<Z5>> = (0..5)
        .flat_map(|a| (0..5).map(move |b| Pair(Z5(a), Z5(b))))
        .collect();
    for x in &all {
        for y in &all {
            assert_eq!(mul(x, y), mul(y, x));
            for z in &all {
                assert_eq!(mul(&mul(x, y), z), mul(x, &mul(y, z)));
                assert_eq!(mul(x, &add(y, z)), add(&mul(x, y), &mul(x, z)));
            }
        }
    }
}

#[test]
fn derived_division_and_scalars() {
    let v = Pair(Z5(1), Z5(2));
    let w = <Pair<Z5> as LeftAction<Z5>>::act(&Z5(3), &v);
    assert_eq!(w, Pair(Z5(3), Z5(1)));
    // ldiv ist aus op und inverse abgeleitet: v + x = w
    let x = <Pair<Z5> as Quasigroup<Additive>>::ldiv(&v, &w);
    assert_eq!(add(&v, &x), w);
}

#[test]
fn commutator_of_a_commutative_algebra_is_abelian() {
    // Ein weiterer Beweis, dass die Konstruktionen mit den Makro-Typen zusammenspielen:
    // in einer kommutativen Algebra ist der Kommutator immer null.
    type L = Commutator<Pair<Z5>, Z5>;
    fn takes_lie<X: LieAlgebra<K>, K: CommutativeRing>() {}
    takes_lie::<L, Z5>();
    let (x, y) = (L::new(Pair(Z5(2), Z5(3))), L::new(Pair(Z5(4), Z5(1))));
    let zero = <L as UnitalMagma<Additive>>::identity();
    assert_eq!(<L as Magma<Bracket>>::op(&x, &y), zero);
}
