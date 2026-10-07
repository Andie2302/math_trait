//! Vorzeichenregeln der Clifford-Algebra bei höheren Graden (D = 16, 32), die Involutionen über
//! die Traits, die Ring-Struktur und die Zugriffsmethoden der Konstruktionen.

use math_trait::*;

mod common;
use common::*;

type C3 = Clifford<Z5, 8, Ones>;
type C4 = Clifford<Z5, 16, Weights>;
type C5 = Clifford<Z5, 32, Weights>;

fn scaled<const D: usize, Q: DiagonalForm<Z5>>(
    s: Z5,
    x: &Clifford<Z5, D, Q>,
) -> Clifford<Z5, D, Q> {
    <Clifford<Z5, D, Q> as LeftAction<Z5>>::act(&s, x)
}

/// Eine Folge „zufälliger“ Elemente (lineare Kongruenz), ohne Abhängigkeit.
fn samples<const D: usize, Q: DiagonalForm<Z5>>(
    count: usize,
    seed: u64,
) -> Vec<Clifford<Z5, D, Q>> {
    let mut state = seed;
    (0..count)
        .map(|_| {
            Clifford::new(std::array::from_fn(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                z((state >> 33) as usize)
            }))
        })
        .collect()
}

fn minus_one_to(k: u32) -> Z5 {
    if k % 2 == 0 { Z5(1) } else { Z5(4) }
}

// --- Vorzeichen je Grad -------------------------------------------------------------------------

#[test]
fn involutions_act_on_every_grade_by_the_textbook_signs() {
    // bis Grad 5: dort wird jedes der drei Vorzeichen mindestens einmal nichttrivial
    for mask in 0..32usize {
        let k = mask.count_ones();
        let e = C5::blade(mask);
        assert_eq!(
            e.reverse(),
            scaled(minus_one_to(k * k.wrapping_sub(1) / 2), &e)
        );
        assert_eq!(e.grade_involution(), scaled(minus_one_to(k), &e));
        assert_eq!(
            e.clifford_conjugate(),
            scaled(minus_one_to(k * (k + 1) / 2), &e)
        );
    }
}

#[test]
fn reversion_matches_the_reversed_product_of_generators() {
    // e_S = e_i₁ ⋯ e_iₖ, also ist ẽ_S = e_iₖ ⋯ e_i₁
    for mask in 0..32usize {
        let ids: Vec<usize> = (0..5).filter(|i| (mask >> i) & 1 == 1).collect();
        let forward = ids
            .iter()
            .fold(C5::scalar(Z5(1)), |acc, &i| cmul(&acc, &C5::generator(i)));
        let backward = ids
            .iter()
            .rev()
            .fold(C5::scalar(Z5(1)), |acc, &i| cmul(&acc, &C5::generator(i)));
        assert_eq!(
            forward,
            C5::blade(mask),
            "e_S ist das aufsteigende Produkt der Erzeuger"
        );
        assert_eq!(C5::blade(mask).reverse(), backward);
    }
}

// --- Erzeuger-Relationen bei mehr als drei Erzeugern ---------------------------------------------

#[test]
fn generator_relations_hold_for_four_and_five_generators() {
    for i in 0..5 {
        let ei = C5::generator(i);
        assert_eq!(cmul(&ei, &ei), C5::scalar(Weights::square(i)), "eᵢ² = qᵢ");
        for j in 0..5 {
            if i != j {
                let ej = C5::generator(j);
                assert_eq!(cmul(&ei, &ej), cneg(&cmul(&ej, &ei)), "eᵢeⱼ = −eⱼeᵢ");
            }
        }
    }
    for i in 0..4 {
        let ei = C4::generator(i);
        assert_eq!(cmul(&ei, &ei), C4::scalar(Weights::square(i)));
    }
}

// --- Involutionen sind (Anti-)Automorphismen, auch bei D = 16 -----------------------------------

#[test]
fn reversion_reverses_products_and_grade_involution_preserves_them() {
    let xs = samples::<16, Weights>(12, 1);
    let ys = samples::<16, Weights>(12, 2);
    for (x, y) in xs.iter().zip(&ys) {
        let xy = cmul(x, y);
        assert_eq!(
            xy.reverse(),
            cmul(&y.reverse(), &x.reverse()),
            "(xy)~ = ỹ x̃"
        );
        assert_eq!(
            xy.grade_involution(),
            cmul(&x.grade_involution(), &y.grade_involution()),
            "(xy)^ = x̂ ŷ"
        );
        assert_eq!(
            xy.clifford_conjugate(),
            cmul(&y.clifford_conjugate(), &x.clifford_conjugate()),
            "(xy)‾ = ȳ x̄"
        );
        // alle drei sind Involutionen
        assert_eq!(x.reverse().reverse(), *x);
        assert_eq!(x.grade_involution().grade_involution(), *x);
        assert_eq!(x.clifford_conjugate().clifford_conjugate(), *x);
    }
}

#[test]
fn the_involution_traits_agree_with_the_inherent_methods() {
    for x in samples::<8, Ones>(20, 3) {
        assert_eq!(<C3 as Involutive<Reversion>>::conjugate(&x), x.reverse());
        assert_eq!(
            <C3 as Involutive<GradeInvolution>>::conjugate(&x),
            x.grade_involution()
        );
        assert_eq!(
            <C3 as Involutive<Conjugation>>::conjugate(&x),
            x.clifford_conjugate()
        );
        assert_eq!(
            <C3 as GradedAlgebra<Z5>>::grade_involution(&x),
            x.grade_involution()
        );
    }
    // die Marker: Reversion kehrt das Produkt um, die Gradinvolution erhält es
    fn anti<A: AntiAutomorphism<Multiplicative, Reversion>>() {}
    fn auto<A: Automorphism<Multiplicative, GradeInvolution>>() {}
    anti::<C3>();
    auto::<C3>();
}

// --- Ringstruktur und Zugriffsmethoden -----------------------------------------------------------

#[test]
fn clifford_algebras_are_rings() {
    fn takes<R: Ring>() {}
    takes::<C3>();
    takes::<C4>();
    takes::<EvenSubalgebra<Z5, 8, Ones>>();
    fn takes_semiring<R: Semiring>() {}
    takes_semiring::<C3>();
}

#[test]
fn accessors_return_what_the_constructors_took() {
    // Vector
    let v = Vector::new([Z5(1), Z5(2), Z5(3)]);
    assert_eq!(v.coords(), &[Z5(1), Z5(2), Z5(3)]);
    assert_eq!(v.into_coords(), [Z5(1), Z5(2), Z5(3)]);
    // CayleyDickson
    let c: CayleyDickson<Z5, Z5> = CayleyDickson::new(Z5(2), Z5(3));
    assert_eq!((c.first(), c.second()), (&Z5(2), &Z5(3)));
    assert_eq!(c.into_parts(), (Z5(2), Z5(3)));
    // Units
    let u = Units::new(Z5(3)).expect("3 ist eine Einheit");
    assert_eq!(u.get(), &Z5(3));
    assert_eq!(u.into_inner(), Z5(3));
    assert!(Units::new(Z5(0)).is_none());
    // Commutator
    let k = Commutator::<Z5, Z5>::new(Z5(4));
    assert_eq!(k.as_inner(), &Z5(4));
    assert_eq!(k.into_inner(), Z5(4));
    // EvenSubalgebra und Bivector aus einem Element der Algebra
    let x = cadd(&C3::blade(0b011), &C3::blade(0b001));
    let even = EvenSubalgebra::from_even_part(&x);
    assert_eq!(even.get(), &C3::blade(0b011));
    assert_eq!(even.into_inner(), C3::blade(0b011));
    let biv = Bivector::from_grade_part(&x);
    assert_eq!(biv.get(), &C3::blade(0b011));
    assert_eq!(biv.into_inner(), C3::blade(0b011));
    assert!(
        Bivector::new(C3::blade(0b001)).is_none(),
        "ein Vektor ist kein Bivektor"
    );
}
