//! Die Lie-Algebra so(N) aus den Bivektoren einer Clifford-Algebra, über ℤ/5.

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
struct Q123;
impl DiagonalForm<Z5> for Ones {
    fn square(_: usize) -> Z5 {
        Z5(1)
    }
}
impl DiagonalForm<Z5> for Q123 {
    fn square(i: usize) -> Z5 {
        z(i + 1)
    }
}

type C3 = Clifford<Z5, 8, Ones>;
type B3 = Bivector<Z5, 8, Ones>;
type V3 = Vector<Z5, 3>;

fn br<const D: usize, Q: DiagonalForm<Z5>>(
    a: &Bivector<Z5, D, Q>,
    b: &Bivector<Z5, D, Q>,
) -> Bivector<Z5, D, Q> {
    <Bivector<Z5, D, Q> as Magma<Bracket>>::op(a, b)
}
fn badd<const D: usize, Q: DiagonalForm<Z5>>(
    a: &Bivector<Z5, D, Q>,
    b: &Bivector<Z5, D, Q>,
) -> Bivector<Z5, D, Q> {
    <Bivector<Z5, D, Q> as Magma<Additive>>::op(a, b)
}

/// Alle Bivektoren der Algebra mit drei Erzeugern: `c₀·e₀e₁ + c₁·e₀e₂ + c₂·e₁e₂`.
fn bivectors3<Q: DiagonalForm<Z5>>() -> Vec<Bivector<Z5, 8, Q>> {
    (0..125usize)
        .map(|i| {
            let mut c = [Z5(0); 8];
            c[0b011] = z(i);
            c[0b101] = z(i / 5);
            c[0b110] = z(i / 25);
            Bivector::new(Clifford::new(c)).expect("Grad 2")
        })
        .collect()
}

fn vectors3() -> Vec<V3> {
    (0..125usize)
        .map(|i| Vector::new([z(i), z(i / 5), z(i / 25)]))
        .collect()
}

#[test]
fn bivectors_form_a_lie_algebra() {
    fn takes_lie<L: LieAlgebra<K>, K: CommutativeRing>() {}
    takes_lie::<B3, Z5>();
    takes_lie::<Bivector<Z5, 16, Ones>, Z5>(); // vier Erzeuger: so(4), sechs Dimensionen
}

#[test]
fn the_commutator_of_bivectors_is_again_a_bivector() {
    let bs = bivectors3::<Ones>();
    for a in &bs {
        for b in &bs {
            let c = br(a, b);
            assert!(Bivector::new(*c.get()).is_some(), "nur Grad 2");
        }
    }
    // vier Erzeuger (sechs Dimensionen): Stichprobe
    let b4: Vec<Bivector<Z5, 16, Ones>> = (0..15625usize)
        .step_by(97)
        .map(|i| {
            let mut c = [Z5(0); 16];
            for (k, mask) in [0b0011, 0b0101, 0b0110, 0b1001, 0b1010, 0b1100]
                .iter()
                .enumerate()
            {
                c[*mask] = z(i / 5usize.pow(k as u32));
            }
            Bivector::new(Clifford::new(c)).expect("Grad 2")
        })
        .collect();
    for a in &b4 {
        for b in &b4 {
            assert!(Bivector::new(*br(a, b).get()).is_some());
        }
    }
}

#[test]
fn bracket_is_alternating_and_satisfies_jacobi() {
    let zero = <B3 as UnitalMagma<Additive>>::identity();
    let bs = bivectors3::<Ones>();
    for x in &bs {
        assert_eq!(br(x, x), zero);
    }
    for x in bs.iter().step_by(7) {
        for y in bs.iter().step_by(9) {
            for w in bs.iter().step_by(11) {
                let sum = badd(
                    &badd(&br(x, &br(y, w)), &br(y, &br(w, x))),
                    &br(w, &br(x, y)),
                );
                assert_eq!(sum, zero);
            }
        }
    }
}

#[test]
fn two_generators_give_the_one_dimensional_abelian_so_2() {
    type B2 = Bivector<Z5, 4, Ones>;
    let zero = <B2 as UnitalMagma<Additive>>::identity();
    let e01 = B2::basis(0, 1);
    for k in 0..5 {
        let x = <B2 as LeftAction<Z5>>::act(&z(k), &e01);
        for l in 0..5 {
            let y = <B2 as LeftAction<Z5>>::act(&z(l), &e01);
            assert_eq!(br(&x, &y), zero);
        }
    }
}

// --- so(3) ≅ Kreuzprodukt -------------------------------------------------------------------

fn cross(a: [Z5; 3], b: [Z5; 3]) -> [Z5; 3] {
    let m = |x: Z5, y: Z5| <Z5 as Magma<Multiplicative>>::op(&x, &y);
    let s = |x: Z5, y: Z5| <Z5 as Magma<Additive>>::op(&x, &<Z5 as Group<Additive>>::inverse(&y));
    [
        s(m(a[1], b[2]), m(a[2], b[1])),
        s(m(a[2], b[0]), m(a[0], b[2])),
        s(m(a[0], b[1]), m(a[1], b[0])),
    ]
}

/// Mit `X = e₁e₂`, `Y = e₂e₀`, `Z = e₀e₁` ist `[X, Y] = −2Z`. Die Zuordnung
/// `aX + bY + cZ ↦ −2·(a, b, c)` macht den Kommutator zum Kreuzprodukt.
#[test]
fn so3_is_the_cross_product_lie_algebra() {
    let minus_two = z(3);
    let psi = |b: &B3| -> [Z5; 3] {
        let c = b.get().coefficients();
        // X = e₁e₂ (Maske 0b110), Y = e₂e₀ = −e₀e₂ (Maske 0b101), Z = e₀e₁ (Maske 0b011)
        let mul = |x: Z5, y: Z5| <Z5 as Magma<Multiplicative>>::op(&x, &y);
        [
            mul(minus_two, c[0b110]),
            mul(minus_two, <Z5 as Group<Additive>>::inverse(&c[0b101])),
            mul(minus_two, c[0b011]),
        ]
    };
    let bs = bivectors3::<Ones>();
    for a in &bs {
        for b in &bs {
            assert_eq!(psi(&br(a, b)), cross(psi(a), psi(b)));
        }
    }
}

// --- Vektordarstellung -------------------------------------------------------------------------

fn matrix_of<Q: DiagonalForm<Z5>>(b: &Bivector<Z5, 8, Q>) -> [[Z5; 3]; 3] {
    let col = |j: usize| <V3 as LieModule<Bivector<Z5, 8, Q>, Z5>>::lie_act(b, &V3::basis(j));
    let cols = [col(0), col(1), col(2)];
    std::array::from_fn(|i| std::array::from_fn(|j| cols[j].coords()[i]))
}

#[test]
fn vectors_stay_vectors() {
    let embed = |v: &V3| <C3 as CliffordAlgebra<V3, Z5, Ones>>::embed(v);
    for b in bivectors3::<Ones>() {
        for v in vectors3() {
            let c = b.commutator_with(&embed(&v));
            assert_eq!(c.grade_part(1), c, "[B, v] ist ein Vektor");
        }
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn vector_representation_is_skew_faithful_and_surjective() {
    let q = |i: usize| <Q123 as DiagonalForm<Z5>>::square(i);
    let m = |x: Z5, y: Z5| <Z5 as Magma<Multiplicative>>::op(&x, &y);
    let a = |x: Z5, y: Z5| <Z5 as Magma<Additive>>::op(&x, &y);
    // schiefsymmetrisch bezüglich B(u, v) = Σ qᵢ uᵢ vᵢ:  qᵢ Mᵢⱼ + qⱼ Mⱼᵢ = 0
    let mut images = HashSet::new();
    for b in bivectors3::<Q123>() {
        let mat = matrix_of(&b);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(a(m(q(i), mat[i][j]), m(q(j), mat[j][i])), Z5(0));
            }
        }
        images.insert(mat.map(|r| r.map(|e| e.0)));
    }
    assert_eq!(
        images.len(),
        125,
        "treu: verschiedene Bivektoren, verschiedene Matrizen"
    );
    // Es gibt genau 5³ = 125 solche Matrizen: Das Bild ist ganz so(Q).
    let mut skew = 0usize;
    for n in 0..5usize.pow(9) {
        let e = |k: usize| z(n / 5usize.pow(k as u32));
        let mat = [[e(0), e(1), e(2)], [e(3), e(4), e(5)], [e(6), e(7), e(8)]];
        if (0..3).all(|i| (0..3).all(|j| a(m(q(i), mat[i][j]), m(q(j), mat[j][i])) == Z5(0))) {
            skew += 1;
        }
    }
    assert_eq!(skew, 125);
}

#[test]
fn vector_representation_turns_the_bracket_into_a_commutator() {
    let act = |b: &B3, v: &V3| <V3 as LieModule<B3, Z5>>::lie_act(b, v);
    let sub = |x: V3, y: V3| <V3 as Magma<Additive>>::op(&x, &<V3 as Group<Additive>>::inverse(&y));
    let bs = bivectors3::<Ones>();
    for x in bs.iter().step_by(13) {
        for y in bs.iter().step_by(17) {
            for v in vectors3().iter().step_by(5) {
                let lhs = act(&br(x, y), v);
                let rhs = sub(act(x, &act(y, v)), act(y, &act(x, v)));
                assert_eq!(lhs, rhs);
            }
        }
    }
}

// --- Spindarstellung -----------------------------------------------------------------------------

fn cmul(a: &C3, b: &C3) -> C3 {
    <C3 as Magma<Multiplicative>>::op(a, b)
}
fn cadd(a: &C3, b: &C3) -> C3 {
    <C3 as Magma<Additive>>::op(a, b)
}

fn spinors() -> Vec<C3> {
    (0..5usize.pow(8))
        .step_by(1009)
        .map(|n| C3::new(std::array::from_fn(|k| z(n / 5usize.pow(k as u32)))))
        .collect()
}

#[test]
fn spin_representation_turns_the_bracket_into_a_commutator() {
    let act = |b: &B3, s: &C3| <C3 as LieModule<B3, Z5>>::lie_act(b, s);
    let sub = |x: C3, y: C3| cadd(&x, &<C3 as Group<Additive>>::inverse(&y));
    let bs = bivectors3::<Ones>();
    for x in bs.iter().step_by(19) {
        for y in bs.iter().step_by(23) {
            for s in spinors() {
                assert_eq!(
                    act(&br(x, y), &s),
                    sub(act(x, &act(y, &s)), act(y, &act(x, &s)))
                );
            }
        }
    }
}

/// Die Clifford-Multiplikation `V ⊗ S → S` ist verträglich mit der Wirkung von so(3):
/// `B ⋅ (v ⋅ s) = [B, v] ⋅ s + v ⋅ (B ⋅ s)`. Das macht die Algebra zum Spinorraum.
#[test]
fn clifford_multiplication_is_equivariant() {
    let embed = |v: &V3| <C3 as CliffordAlgebra<V3, Z5, Ones>>::embed(v);
    let act_s = |b: &B3, s: &C3| <C3 as LieModule<B3, Z5>>::lie_act(b, s);
    let act_v = |b: &B3, v: &V3| <V3 as LieModule<B3, Z5>>::lie_act(b, v);
    for b in bivectors3::<Ones>().iter().step_by(11) {
        for v in vectors3().iter().step_by(7) {
            for s in spinors() {
                let lhs = act_s(b, &cmul(&embed(v), &s));
                let rhs = cadd(
                    &cmul(&embed(&act_v(b, v)), &s),
                    &cmul(&embed(v), &act_s(b, &s)),
                );
                assert_eq!(lhs, rhs);
            }
        }
    }
}

// --- Gruppe und Algebra -------------------------------------------------------------------------

/// Die Rotorgruppe wirkt durch Konjugation auf ihre Lie-Algebra: `s B s̃` ist wieder ein
/// Bivektor, und die Klammer wird erhalten (`s̃ = s⁻¹`).
#[test]
fn rotors_act_on_so3_by_conjugation_preserving_the_bracket() {
    let evens: Vec<C3> = (0..625usize)
        .map(|i| {
            let mut c = [Z5(0); 8];
            c[0b000] = z(i);
            c[0b011] = z(i / 5);
            c[0b101] = z(i / 25);
            c[0b110] = z(i / 125);
            C3::new(c)
        })
        .collect();
    let rotors: Vec<Rotor<Z5, 8, Ones>> = evens.into_iter().filter_map(Rotor::new).collect();
    assert_eq!(rotors.len(), 120);
    let bs = bivectors3::<Ones>();
    let ad = |r: &Rotor<Z5, 8, Ones>, b: &B3| {
        Bivector::new(r.get().sandwich(b.get())).expect("s B s̃ ist ein Bivektor")
    };
    for r in rotors.iter().step_by(7) {
        for x in bs.iter().step_by(9) {
            for y in bs.iter().step_by(11) {
                assert_eq!(ad(r, &br(x, y)), br(&ad(r, x), &ad(r, y)));
            }
        }
    }
}
