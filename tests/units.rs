//! Die Einheitengruppe `K×` und die abgeleitete Division.

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

type U = Units<Z5>;

fn u(k: u8) -> U {
    Units::new(Z5(k)).expect("ungleich null")
}
fn mul(a: &U, b: &U) -> U {
    <U as Magma<Multiplicative>>::op(a, b)
}

#[test]
fn zero_is_not_a_unit() {
    assert!(Units::new(Z5(0)).is_none());
    assert_eq!(Units::new(Z5(3)).unwrap().get(), &Z5(3));
}

#[test]
fn units_form_an_abelian_group() {
    fn takes_group<G: Group<Multiplicative>>() {}
    fn takes_abelian<G: AbelianGroup<Multiplicative>>() {}
    takes_group::<U>();
    takes_abelian::<U>();
    let all: Vec<U> = (1..5).map(u).collect();
    let one = <U as UnitalMagma<Multiplicative>>::identity();
    for x in &all {
        let inv = <U as Group<Multiplicative>>::inverse(x);
        assert_eq!(mul(x, &inv), one);
        assert_eq!(mul(&inv, x), one);
        for y in &all {
            assert_eq!(mul(x, y), mul(y, x));
            // abgeschlossen: kein Produkt ist null
            assert!(Units::new(*mul(x, y).get()).is_some());
            // ldiv: x ∘ z = y  =>  z = x⁻¹ ∘ y
            let z = <U as Quasigroup<Multiplicative>>::ldiv(x, y);
            assert_eq!(mul(x, &z), *y);
        }
    }
}

#[test]
fn two_is_a_generator_so_the_group_is_cyclic_of_order_four() {
    let one = <U as UnitalMagma<Multiplicative>>::identity();
    let g = u(2);
    let mut p = g;
    let mut order = 1;
    while p != one {
        p = mul(&p, &g);
        order += 1;
    }
    assert_eq!(order, 4);
}

#[test]
fn division_is_derived_from_the_reciprocal() {
    // 3 / 2 = 3 · 2⁻¹ = 3 · 3 = 9 = 4
    assert_eq!(<Z5 as DivisionRing>::div(&Z5(3), &Z5(2)), Some(Z5(4)));
    assert_eq!(<Z5 as DivisionRing>::div_left(&Z5(3), &Z5(2)), Some(Z5(4)));
    assert_eq!(<Z5 as DivisionRing>::div(&Z5(3), &Z5(0)), None);
}
