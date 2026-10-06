//! Graduierte Struktur der Clifford-Algebra: Grade, Involutionen, gerade Unteralgebra, Rotoren.

use math_trait::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Z5(u8);

impl_field!(Z5, Additive, Multiplicative;
    add(a, b) { Z5((a.0 + b.0) % 5) }
    zero() { Z5(0) }
    neg(a) { Z5((5 - a.0) % 5) }
    mul(a, b) { Z5((a.0 * b.0) % 5) }
    one() { Z5(1) }
    recip(a) { [None, Some(Z5(1)), Some(Z5(3)), Some(Z5(2)), Some(Z5(4))][a.0 as usize] }
);
impl_field_algebra!(Z5);

fn z(a: usize) -> Z5 {
    Z5((a % 5) as u8)
}

struct Ones;
struct MinusOnes;
impl DiagonalForm<Z5> for Ones {
    fn square(_: usize) -> Z5 {
        Z5(1)
    }
}
impl DiagonalForm<Z5> for MinusOnes {
    fn square(_: usize) -> Z5 {
        Z5(4)
    }
}

type C2 = Clifford<Z5, 4, Ones>;
type C3 = Clifford<Z5, 8, Ones>;

fn mul<const D: usize, Q: DiagonalForm<Z5>>(
    a: &Clifford<Z5, D, Q>,
    b: &Clifford<Z5, D, Q>,
) -> Clifford<Z5, D, Q> {
    <Clifford<Z5, D, Q> as Magma<Multiplicative>>::op(a, b)
}
fn add<const D: usize, Q: DiagonalForm<Z5>>(
    a: &Clifford<Z5, D, Q>,
    b: &Clifford<Z5, D, Q>,
) -> Clifford<Z5, D, Q> {
    <Clifford<Z5, D, Q> as Magma<Additive>>::op(a, b)
}

fn all<const D: usize, Q: DiagonalForm<Z5>>() -> Vec<Clifford<Z5, D, Q>> {
    let total = 5usize.pow(D as u32);
    (0..total)
        .map(|i| Clifford::new(std::array::from_fn(|k| z(i / 5usize.pow(k as u32)))))
        .collect()
}

#[test]
fn clifford_is_a_graded_algebra() {
    fn takes<A: GradedAlgebra<K>, K: CommutativeRing>() {}
    takes::<C2, Z5>();
    takes::<C3, Z5>();
}

// --- Zerlegung in Grade -------------------------------------------------------------------------

#[test]
fn elements_split_into_even_and_odd_parts() {
    for x in all::<4, Ones>() {
        assert_eq!(add(&x.even_part(), &x.odd_part()), x);
        // die Grade 0, 1, 2 summieren sich ebenfalls zum Element
        let sum = add(&add(&x.grade_part(0), &x.grade_part(1)), &x.grade_part(2));
        assert_eq!(sum, x);
        assert!(x.even_part().is_even());
        assert!(x.odd_part().is_odd());
    }
}

#[test]
fn degrees_add_modulo_two() {
    let xs = all::<8, Ones>();
    let mut checked = 0;
    for x in xs.iter().step_by(1009) {
        for y in xs.iter().step_by(1013) {
            let (xe, xo) = (x.even_part(), x.odd_part());
            let (ye, yo) = (y.even_part(), y.odd_part());
            assert!(mul(&xe, &ye).is_even());
            assert!(mul(&xo, &yo).is_even());
            assert!(mul(&xe, &yo).is_odd());
            assert!(mul(&xo, &ye).is_odd());
            checked += 1;
        }
    }
    assert!(checked > 25);
}

// --- Die drei Involutionen ---------------------------------------------------------------------

#[test]
fn reversion_reverses_the_product() {
    let e = |i: usize| C2::generator(i);
    // (e₀ e₁)~ = e₁ e₀ = −e₀ e₁
    let b = mul(&e(0), &e(1));
    assert_eq!(b.reverse(), <C2 as Group<Additive>>::inverse(&b));
    let ys = all::<4, Ones>();
    for x in ys.iter().step_by(7) {
        assert_eq!(x.reverse().reverse(), *x);
        for y in ys.iter().cloned() {
            assert_eq!(mul(x, &y).reverse(), mul(&y.reverse(), &x.reverse()));
        }
    }
}

#[test]
fn grade_involution_preserves_the_product() {
    let ys = all::<4, Ones>();
    for x in ys.iter().step_by(7) {
        assert_eq!(x.grade_involution().grade_involution(), *x);
        for y in ys.iter().cloned() {
            assert_eq!(
                mul(x, &y).grade_involution(),
                mul(&x.grade_involution(), &y.grade_involution())
            );
        }
    }
}

#[test]
fn clifford_conjugation_is_grade_involution_after_reversion_and_an_anti_automorphism() {
    fn takes<A: AlgebraWithInvolution<K>, K: CommutativeRing>() {}
    takes::<C3, Z5>();
    let ys = all::<4, Ones>();
    for x in ys.iter().step_by(7) {
        assert_eq!(x.clifford_conjugate(), x.reverse().grade_involution());
        for y in ys.iter().cloned() {
            assert_eq!(
                mul(x, &y).clifford_conjugate(),
                mul(&y.clifford_conjugate(), &x.clifford_conjugate())
            );
        }
    }
}

/// Bei zwei Erzeugern mit `q = (−1, −1)` ist die Clifford-Konjugation die Konjugation der
/// Quaternionen aus Cayley-Dickson.
#[test]
fn clifford_conjugation_is_quaternion_conjugation() {
    type C = Clifford<Z5, 4, MinusOnes>;
    type H = CayleyDickson<CayleyDickson<Z5, Z5>, Z5>;
    let to_h = |x: &C| {
        let c = x.coefficients();
        H::new(
            CayleyDickson::new(c[0], c[1]),
            CayleyDickson::new(c[2], c[3]),
        )
    };
    for x in all::<4, MinusOnes>() {
        assert_eq!(
            to_h(&x.clifford_conjugate()),
            <H as Involutive>::conjugate(&to_h(&x))
        );
    }
}

// --- Gerade Unteralgebra ----------------------------------------------------------------------

#[test]
fn even_subalgebra_membership() {
    assert!(EvenSubalgebra::new(C3::generator(0)).is_none()); // ungerade
    assert!(EvenSubalgebra::new(mul(&C3::generator(0), &C3::generator(1))).is_some());
    let mixed = add(&C3::generator(0), &C3::scalar(Z5(1)));
    assert!(EvenSubalgebra::new(mixed).is_none());
    // der gerade Anteil von allem ist gerade
    assert!(EvenSubalgebra::new(mixed.even_part()).is_some());
    assert_eq!(
        *EvenSubalgebra::from_even_part(&mixed).get(),
        mixed.even_part()
    );
}

/// `Cl⁰(−1, −1, −1) ≅ Cl(−1, −1)`: `f₀ = e₀e₂` und `f₁ = e₁e₂` sind die Erzeuger der
/// kleineren Algebra, mit `f₀f₁ = e₀e₁`.
#[test]
fn even_subalgebra_of_three_generators_is_the_quaternions() {
    type Big = Clifford<Z5, 8, MinusOnes>;
    type Small = Clifford<Z5, 4, MinusOnes>;
    type E = EvenSubalgebra<Z5, 8, MinusOnes>;
    let phi = |x: &E| {
        let c = x.get().coefficients();
        Small::new([c[0b000], c[0b101], c[0b110], c[0b011]])
    };
    let evens: Vec<E> = (0..625usize)
        .map(|i| {
            let mut c = [Z5(0); 8];
            c[0b000] = z(i);
            c[0b011] = z(i / 5);
            c[0b101] = z(i / 25);
            c[0b110] = z(i / 125);
            EvenSubalgebra::new(Big::new(c)).expect("gerade")
        })
        .collect();
    assert_eq!(evens.len(), 625, "Cl⁰ hat 4 Dimensionen, also 5⁴ Elemente");
    for x in evens.iter().step_by(5) {
        for y in &evens {
            let lhs = phi(&<E as Magma<Multiplicative>>::op(x, y));
            assert_eq!(lhs, mul(&phi(x), &phi(y)));
        }
    }
}

// --- Rotoren: die Spin-Gruppe von SO(3) -----------------------------------------------------------

fn even_elements() -> Vec<C3> {
    (0..625usize)
        .map(|i| {
            let mut c = [Z5(0); 8];
            c[0b000] = z(i);
            c[0b011] = z(i / 5);
            c[0b101] = z(i / 25);
            c[0b110] = z(i / 125);
            C3::new(c)
        })
        .collect()
}

fn rotors() -> Vec<Rotor<Z5, 8, Ones>> {
    even_elements().into_iter().filter_map(Rotor::new).collect()
}

#[test]
fn there_are_120_rotors_in_three_dimensions() {
    // Einheitsquaternionen über ℤ/5: |SL₂(ℤ/5)| = 120
    assert_eq!(rotors().len(), 120);
    // 2·1 ist kein Rotor (s s̃ = 4), ein Vektor auch nicht
    assert!(Rotor::new(C3::scalar(Z5(2))).is_none());
    assert!(Rotor::new(C3::generator(0)).is_none());
}

#[test]
fn rotors_form_a_group() {
    fn takes_group<G: Group<Multiplicative>>() {}
    takes_group::<Rotor<Z5, 8, Ones>>();
    let rs = rotors();
    let id = <Rotor<Z5, 8, Ones> as UnitalMagma<Multiplicative>>::identity();
    for a in &rs {
        let inv = <Rotor<Z5, 8, Ones> as Group<Multiplicative>>::inverse(a);
        assert_eq!(
            <Rotor<Z5, 8, Ones> as Magma<Multiplicative>>::op(a, &inv),
            id
        );
        for b in rs.iter().step_by(11) {
            let p = <Rotor<Z5, 8, Ones> as Magma<Multiplicative>>::op(a, b);
            assert!(
                Rotor::new(*p.get()).is_some(),
                "Produkt zweier Rotoren ist ein Rotor"
            );
        }
    }
}

#[test]
fn rotation_keeps_vectors_vectors_and_preserves_the_form() {
    let vectors: Vec<C3> = (0..125usize)
        .map(|i| {
            let mut c = [Z5(0); 8];
            c[0b001] = z(i);
            c[0b010] = z(i / 5);
            c[0b100] = z(i / 25);
            C3::new(c)
        })
        .collect();
    for r in rotors() {
        for v in &vectors {
            let w = r.rotate(v);
            assert_eq!(w.grade_part(1), w, "ein Vektor bleibt ein Vektor");
            assert_eq!(mul(&w, &w), mul(v, v), "Q(w) = Q(v)");
        }
    }
}

#[test]
fn the_spin_group_double_covers_the_rotations() {
    let basis = [C3::generator(0), C3::generator(1), C3::generator(2)];
    let key = |r: &Rotor<Z5, 8, Ones>| -> Vec<u8> {
        basis
            .iter()
            .flat_map(|e| r.rotate(e).coefficients().map(|c| c.0))
            .collect()
    };
    let rs = rotors();
    let maps: HashSet<Vec<u8>> = rs.iter().map(key).collect();
    // 120 Rotoren, aber nur 60 verschiedene Drehungen
    assert_eq!(maps.len(), 60);
    // der Kern ist {1, −1}
    let identity_key = key(&<Rotor<Z5, 8, Ones> as UnitalMagma<Multiplicative>>::identity());
    let kernel: Vec<&Rotor<Z5, 8, Ones>> = rs.iter().filter(|r| key(r) == identity_key).collect();
    assert_eq!(kernel.len(), 2);
    // s und −s drehen gleich
    let s = rs[17];
    let minus_s = Rotor::new(<C3 as Group<Additive>>::inverse(s.get())).expect("−s ist ein Rotor");
    assert_eq!(key(&s), key(&minus_s));
    // Hintereinanderausführen entspricht dem Produkt: rotate(s t) = rotate(s) ∘ rotate(t)
    let t = rs[53];
    let st = <Rotor<Z5, 8, Ones> as Magma<Multiplicative>>::op(&s, &t);
    for e in &basis {
        assert_eq!(st.rotate(e), s.rotate(&t.rotate(e)));
    }
}
