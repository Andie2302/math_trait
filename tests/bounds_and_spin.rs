//! Bereichsprüfungen der Basis-Konstruktoren, die Spin-Gruppe ab N = 6, Gruppendivision auf
//! einer nichtabelschen Gruppe und die gewichtete quadratische Form auf `Vector`.

use math_trait::*;

mod common;
use common::*;

type C2 = Clifford<Z5, 4, Ones>;
type C3 = Clifford<Z5, 8, Ones>;
type C6 = Clifford<Z5, 64, Six>;
type C6Ones = Clifford<Z5, 64, Ones>;

// --- Bereichsprüfungen --------------------------------------------------------------------------

#[test]
fn basis_constructors_accept_every_valid_index() {
    for i in 0..3 {
        assert_eq!(Vector::<Z5, 3>::basis(i).coords()[i], Z5(1));
    }
    for i in 0..2 {
        assert_eq!(C2::generator(i), C2::blade(1 << i));
    }
    for mask in 0..4 {
        assert_eq!(C2::blade(mask).coefficients()[mask], Z5(1));
    }
}

#[test]
#[should_panic(expected = "außerhalb")]
fn vector_basis_beyond_n_panics() {
    let _ = Vector::<Z5, 3>::basis(3);
}

#[test]
#[should_panic(expected = "außerhalb")]
fn clifford_blade_beyond_d_panics() {
    let _ = C3::blade(8);
}

#[test]
#[should_panic(expected = "außerhalb")]
fn clifford_generator_beyond_n_panics() {
    let _ = C3::generator(3);
}

/// Ab Index 64 wäre `1 << i` im Release-Build still `1 << (i % 64)`; die Prüfung kommt zuerst.
#[test]
#[should_panic(expected = "außerhalb")]
fn clifford_generator_far_beyond_n_panics_in_every_profile() {
    let i = std::hint::black_box(64usize);
    let _ = C2::generator(i);
}

#[test]
#[should_panic(expected = "zwei verschiedene")]
fn bivector_basis_needs_two_different_generators() {
    let _ = Bivector::<Z5, 8, Ones>::basis(1, 1);
}

#[test]
#[should_panic(expected = "außerhalb")]
fn bivector_basis_beyond_n_panics() {
    let _ = Bivector::<Z5, 4, Ones>::basis(0, 2);
}

#[test]
fn bivector_basis_changes_sign_when_swapped() {
    type B = Bivector<Z5, 8, Ones>;
    let e01 = B::basis(0, 1);
    let e10 = B::basis(1, 0);
    assert_eq!(e10, <B as Group<Additive>>::inverse(&e01));
    assert_ne!(e01, e10);
}

// --- Die Spin-Gruppe: die Vektor-Bedingung ab N = 6 --------------------------------------------

/// Das Pseudoskalar `I = e₀e₁e₂e₃e₄e₅` (Maske 63).
fn pseudoscalar() -> C6 {
    C6::blade(63)
}

#[test]
fn a_pseudoscalar_part_satisfies_the_norm_condition_but_is_no_rotor() {
    // s = 2 + I mit I² = −q₀⋯q₅ = −2 und Ĩ = −I: s·s̃ = 4 − I² = 6 = 1 in ℤ/5
    let s = cadd(&C6::scalar(Z5(2)), &pseudoscalar());
    assert!(s.is_even());
    assert_eq!(cmul(&s, &s.reverse()), C6::scalar(Z5(1)));
    // trotzdem bildet s die Vektoren nicht auf Vektoren ab: kein Rotor
    assert!(Rotor::new(s).is_none());
    let moved = s.sandwich(&C6::generator(0));
    assert_ne!(moved.grade_part(1), moved);
}

#[test]
fn a_sum_of_blades_with_unit_norm_is_no_rotor_either() {
    // s = 3 (1 + e₀₁₂₃ + e₀₁₄₅ + e₂₃₄₅), alle q = 1
    let blades = [0b001111, 0b110011, 0b111100];
    let mut s = C6Ones::scalar(Z5(1));
    for mask in blades {
        s = cadd(&s, &C6Ones::blade(mask));
    }
    let s = <C6Ones as LeftAction<Z5>>::act(&Z5(3), &s);
    assert!(s.is_even());
    assert_eq!(cmul(&s, &s.reverse()), C6Ones::scalar(Z5(1)));
    assert!(Rotor::new(s).is_none());
}

#[test]
fn genuine_rotors_in_six_dimensions_are_accepted_and_form_a_group() {
    // e₀₁ ist ein Rotor (eine Drehung um π in der e₀e₁-Ebene), ebenso e₂₃
    let a = Rotor::new(C6::blade(0b000011)).expect("e01 ist ein Rotor");
    let b = Rotor::new(C6::blade(0b001100)).expect("e23 ist ein Rotor");
    let ab = <Rotor<Z5, 64, Six> as Magma<Multiplicative>>::op(&a, &b);
    assert!(Rotor::new(*ab.get()).is_some(), "das Produkt ist ein Rotor");
    let inv = <Rotor<Z5, 64, Six> as Group<Multiplicative>>::inverse(&a);
    assert!(
        Rotor::new(*inv.get()).is_some(),
        "das Inverse ist ein Rotor"
    );
    // und jeder Rotor lässt die Vektoren Vektoren sein
    for i in 0..6 {
        let w = ab.rotate(&C6::generator(i));
        assert_eq!(w.grade_part(1), w);
    }
}

// --- Gruppendivision auf einer nichtabelschen Gruppe -------------------------------------------

type R3 = Rotor<Z5, 8, Ones>;

fn rotors() -> Vec<R3> {
    (0..625usize)
        .filter_map(|i| {
            let mut c = [Z5(0); 8];
            c[0b000] = z(i);
            c[0b011] = z(i / 5);
            c[0b101] = z(i / 25);
            c[0b110] = z(i / 125);
            Rotor::new(C3::new(c))
        })
        .collect()
}

#[test]
fn ldiv_and_rdiv_are_the_two_different_divisions_of_a_nonabelian_group() {
    let rs = rotors();
    assert_eq!(rs.len(), 120);
    let op = |a: &R3, b: &R3| <R3 as Magma<Multiplicative>>::op(a, b);
    let ldiv = |a: &R3, b: &R3| <R3 as Quasigroup<Multiplicative>>::ldiv(a, b);
    let rdiv = |a: &R3, b: &R3| <R3 as Quasigroup<Multiplicative>>::rdiv(a, b);
    let mut differ = false;
    for a in rs.iter().step_by(7) {
        for b in rs.iter().step_by(11) {
            assert_eq!(op(a, &ldiv(a, b)), *b, "a ∘ (a \\ b) = b");
            assert_eq!(op(&rdiv(a, b), a), *b, "(b / a) ∘ a = b");
            differ |= ldiv(a, b) != rdiv(a, b);
        }
    }
    assert!(
        differ,
        "in einer nichtabelschen Gruppe sind ldiv und rdiv verschieden"
    );
}

// --- Die gewichtete quadratische Form auf Vector ------------------------------------------------

#[test]
fn quadratic_form_of_a_vector_uses_the_weights() {
    type V3 = Vector<Z5, 3>;
    // Q(x) = 1·x₀² + 2·x₁² + 3·x₂², hier x = (1, 1, 1): 1 + 2 + 3 = 6 = 1
    let x = V3::new([Z5(1), Z5(1), Z5(1)]);
    assert_eq!(<V3 as QuadraticForm<Z5, Weights>>::value(&x), Z5(1));
    // x = (2, 3, 1): 1·4 + 2·9 + 3·1 = 25 = 0
    let y = V3::new([Z5(2), Z5(3), Z5(1)]);
    assert_eq!(<V3 as QuadraticForm<Z5, Weights>>::value(&y), Z5(0));
    // mit allen Gewichten 1 ist es die Summe der Quadrate: 4 + 9 + 1 = 14 = 4
    assert_eq!(<V3 as QuadraticForm<Z5, Ones>>::value(&y), Z5(4));
}

#[test]
fn quadratic_form_is_homogeneous_of_degree_two_and_matches_the_embedding() {
    type V3 = Vector<Z5, 3>;
    type C = Clifford<Z5, 8, Weights>;
    for i in 0..125usize {
        let x = V3::new([z(i), z(i / 5), z(i / 25)]);
        let q = <V3 as QuadraticForm<Z5, Weights>>::value(&x);
        for s in 0..5usize {
            let sx = <V3 as LeftAction<Z5>>::act(&z(s), &x);
            let expected = <Z5 as Magma<Multiplicative>>::op(&z(s * s), &q);
            assert_eq!(<V3 as QuadraticForm<Z5, Weights>>::value(&sx), expected);
        }
        let e = <C as CliffordAlgebra<V3, Z5, Weights>>::embed(&x);
        assert_eq!(cmul(&e, &e), C::scalar(q), "embed(x)² = Q(x)");
    }
}

// --- Das Bild der Spin-Gruppe über ℤ/5 ----------------------------------------------------------

/// Die Matrix der Drehung `v ↦ s v s̃` auf den Vektoren, als Bilder von `e₀, e₁, e₂`.
fn rotation_of(r: &R3) -> [[u8; 3]; 3] {
    std::array::from_fn(|i| {
        let w = r.rotate(&C3::generator(i));
        std::array::from_fn(|j| w.coefficients()[1 << j].0)
    })
}

#[test]
fn the_rotors_reach_only_half_of_the_rotation_group_over_z5() {
    let dot = |a: &[u8; 3], b: &[u8; 3]| (0..3).map(|i| u32::from(a[i] * b[i])).sum::<u32>() % 5;
    // |SO(3, ℤ/5)| = Anzahl der Paare orthonormaler erster zwei Zeilen (die dritte ist das
    // Kreuzprodukt, hat dann ebenfalls Norm 1 und Determinante 1)
    let vectors: Vec<[u8; 3]> = (0..125u8).map(|i| [i % 5, (i / 5) % 5, i / 25]).collect();
    let so3 = vectors
        .iter()
        .flat_map(|a| vectors.iter().map(move |b| (a, b)))
        .filter(|(a, b)| dot(a, a) == 1 && dot(b, b) == 1 && dot(a, b) == 0)
        .count();
    assert_eq!(so3, 120);

    let images: std::collections::HashSet<[[u8; 3]; 3]> =
        rotors().iter().map(rotation_of).collect();
    assert_eq!(
        images.len() * 2,
        so3,
        "zwei Rotoren je Drehung, aber nur die Hälfte aller Drehungen"
    );

    // die 90°-Drehung (e₀ ↦ e₁, e₁ ↦ −e₀, e₂ ↦ e₂) liegt in SO(3, ℤ/5), wird aber von keinem
    // Rotor erreicht: Sie bräuchte s = (1 + e₀e₁)/√2, und 2 ist in ℤ/5 kein Quadrat.
    let quarter_turn = [[0, 1, 0], [4, 0, 0], [0, 0, 1]];
    assert!(!images.contains(&quarter_turn));
}
