//! Rng (Ring ohne Eins), Band und Halbverband.

use math_trait::*;

// --- Rng: die geraden Restklassen modulo 20 haben keine Eins -----------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ev(u16); // gerade, 0..20

impl_rng!(Ev, Additive, Multiplicative;
    add(a, b) { Ev((a.0 + b.0) % 20) }
    zero() { Ev(0) }
    neg(a) { Ev((20 - a.0) % 20) }
    mul(a, b) { Ev((a.0 * b.0) % 20) }
);

fn ev() -> Vec<Ev> {
    (0..10).map(|k| Ev(2 * k)).collect()
}
fn emul(a: &Ev, b: &Ev) -> Ev {
    <Ev as Magma<Multiplicative>>::op(a, b)
}
fn eadd(a: &Ev, b: &Ev) -> Ev {
    <Ev as Magma<Additive>>::op(a, b)
}

#[test]
fn rng_laws_and_no_identity() {
    fn takes_rng<R: Rng>() {}
    takes_rng::<Ev>();
    for x in ev() {
        for y in ev() {
            for z in ev() {
                assert_eq!(emul(&emul(&x, &y), &z), emul(&x, &emul(&y, &z)));
                assert_eq!(emul(&x, &eadd(&y, &z)), eadd(&emul(&x, &y), &emul(&x, &z)));
            }
        }
    }
    // keine Eins: zu jedem e gibt es ein x mit e·x ≠ x
    for e in ev() {
        assert!(
            ev().iter().any(|x| emul(&e, x) != *x),
            "e = {e:?} wäre eine Eins"
        );
    }
}

// --- Band und Halbverband ----------------------------------------------------------------------

struct Op;

/// Linksnull-Halbgruppe: x ∘ y = x. Ein Band, aber nicht kommutativ.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Left(u8);
impl_band!(Left, Op; op(a, _b) { Left(a.0) });

/// Maximum: ein Halbverband.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Max(u8);
impl_semilattice!(Max, Op; op(a, b) { Max(a.0.max(b.0)) });

fn laws<T: Band<Op> + PartialEq + Copy>(xs: &[T]) {
    let op = |a: &T, b: &T| <T as Magma<Op>>::op(a, b);
    for x in xs {
        assert!(op(x, x) == *x, "idempotent");
        for y in xs {
            for z in xs {
                assert!(op(&op(x, y), z) == op(x, &op(y, z)), "assoziativ");
            }
        }
    }
}

#[test]
fn a_band_need_not_be_commutative_but_a_semilattice_is() {
    let ls: Vec<Left> = (0..4).map(Left).collect();
    laws(&ls);
    let l = |a: u8, b: u8| <Left as Magma<Op>>::op(&Left(a), &Left(b));
    assert_ne!(l(1, 2), l(2, 1));

    fn takes_semilattice<S: Semilattice<Op>>() {}
    takes_semilattice::<Max>();
    let ms: Vec<Max> = (0..4).map(Max).collect();
    laws(&ms);
    let m = |a: u8, b: u8| <Max as Magma<Op>>::op(&Max(a), &Max(b));
    assert_eq!(m(1, 2), m(2, 1));
}
