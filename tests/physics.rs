//! Tensorprodukt, Clifford-Algebra und Lie-Darstellung, mit den Beispielen der Quantenmechanik
//! (hier über ℤ/5 statt ℂ): Verschränkung, Pauli-Matrizen, Spin ½.

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
fn z(a: u8) -> Z5 {
    Z5(a % 5)
}

// --- Vektoren (2-dimensional) und Matrizen (2×2) über ℤ/5 -----------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
struct V2(Z5, Z5);

impl_abelian_group!(V2, Additive;
    op(a, b) { V2(add(a.0, b.0), add(a.1, b.1)) }
    identity() { V2(Z5(0), Z5(0)) }
    inverse(a) { V2(neg(a.0), neg(a.1)) }
);
impl_module!(V2, Z5; act(s, x) { V2(mul(*s, x.0), mul(*s, x.1)) });

#[derive(Debug, Clone, Copy, PartialEq)]
struct Mat2(Z5, Z5, Z5, Z5);

fn m(a: u8, b: u8, c: u8, d: u8) -> Mat2 {
    Mat2(z(a), z(b), z(c), z(d))
}

impl_abelian_group!(Mat2, Additive;
    op(x, y) { Mat2(add(x.0, y.0), add(x.1, y.1), add(x.2, y.2), add(x.3, y.3)) }
    identity() { m(0, 0, 0, 0) }
    inverse(x) { Mat2(neg(x.0), neg(x.1), neg(x.2), neg(x.3)) }
);
impl_module!(Mat2, Z5; act(s, x) { Mat2(mul(*s, x.0), mul(*s, x.1), mul(*s, x.2), mul(*s, x.3)) });
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
impl_unital_algebra!(Mat2, Z5, Multiplicative);
impl_associative_algebra!(Mat2, Z5, Multiplicative);

fn det(x: &Mat2) -> Z5 {
    add(mul(x.0, x.3), neg(mul(x.1, x.2)))
}
fn mmul(x: &Mat2, y: &Mat2) -> Mat2 {
    <Mat2 as Magma<Multiplicative>>::op(x, y)
}
fn madd(x: &Mat2, y: &Mat2) -> Mat2 {
    <Mat2 as Magma<Additive>>::op(x, y)
}
fn all_vectors() -> Vec<V2> {
    (0..25).map(|i| V2(z(i), z(i / 5))).collect()
}
fn all_matrices() -> Vec<Mat2> {
    (0..625usize)
        .map(|i| {
            m(
                (i % 5) as u8,
                ((i / 5) % 5) as u8,
                ((i / 25) % 5) as u8,
                ((i / 125) % 5) as u8,
            )
        })
        .collect()
}

// --- Tensorprodukt: V2 ⊗ V2 = 2×2-Matrizen ----------------------------------------------------

/// `v ⊗ w = v · wᵀ`
impl TensorProduct<V2, V2, Z5> for Mat2 {
    fn tensor(v: &V2, w: &V2) -> Self {
        Mat2(mul(v.0, w.0), mul(v.0, w.1), mul(v.1, w.0), mul(v.1, w.1))
    }
}

fn t(v: &V2, w: &V2) -> Mat2 {
    <Mat2 as TensorProduct<V2, V2, Z5>>::tensor(v, w)
}

#[test]
fn tensor_is_bilinear() {
    let vs = all_vectors();
    let va = |a: &V2, b: &V2| <V2 as Magma<Additive>>::op(a, b);
    let act = |s: u8, v: &V2| <V2 as LeftAction<Z5>>::act(&z(s), v);
    let macts = |s: u8, x: &Mat2| <Mat2 as LeftAction<Z5>>::act(&z(s), x);
    for v in &vs[..10] {
        for w in &vs[..10] {
            for u in &vs[..10] {
                assert_eq!(t(&va(v, u), w), madd(&t(v, w), &t(u, w)));
                assert_eq!(t(v, &va(w, u)), madd(&t(v, w), &t(v, u)));
            }
            for s in 0..5 {
                assert_eq!(t(&act(s, v), w), macts(s, &t(v, w)));
                assert_eq!(t(v, &act(s, w)), macts(s, &t(v, w)));
            }
        }
    }
}

#[test]
fn pure_tensors_have_determinant_zero_but_not_every_tensor_is_pure() {
    let vs = all_vectors();
    for v in &vs {
        for w in &vs {
            assert_eq!(
                det(&t(v, w)),
                Z5(0),
                "ein reiner Tensor hat Rang höchstens 1"
            );
        }
    }
    // |00⟩ + |11⟩ ist nicht rein, also verschränkt: Determinante 1 ≠ 0
    let (e0, e1) = (V2(Z5(1), Z5(0)), V2(Z5(0), Z5(1)));
    let bell = madd(&t(&e0, &e0), &t(&e1, &e1));
    assert_eq!(bell, m(1, 0, 0, 1));
    assert_ne!(det(&bell), Z5(0));
}

// --- bilineare Abbildung mit Methode: dasselbe, als `BilinearMap` ----------------------------------

struct Outer;
impl Bilinear<V2, Mat2, Z5, Outer> for V2 {}
impl BilinearMap<V2, Mat2, Z5, Outer> for V2 {
    fn bilinear(&self, rhs: &V2) -> Mat2 {
        t(self, rhs)
    }
}

#[test]
fn bilinear_map_agrees_with_tensor() {
    let (v, w) = (V2(z(2), z(3)), V2(z(4), z(1)));
    assert_eq!(
        <V2 as BilinearMap<V2, Mat2, Z5, Outer>>::bilinear(&v, &w),
        t(&v, &w)
    );
}

// --- Clifford-Algebra: Pauli-Matrizen ----------------------------------------------------------

struct SumOfSquares;

/// `Q(x, y) = x² + y²`
impl QuadraticForm<Z5, SumOfSquares> for V2 {
    fn value(&self) -> Z5 {
        add(mul(self.0, self.0), mul(self.1, self.1))
    }
}

/// Die Einbettung `(x, y) ↦ x·σz + y·σx` mit `σz = diag(1, −1)` und `σx = [[0,1],[1,0]]`.
impl CliffordAlgebra<V2, Z5, SumOfSquares> for Mat2 {
    fn embed(v: &V2) -> Self {
        Mat2(v.0, v.1, v.1, neg(v.0))
    }
}

fn embed(v: &V2) -> Mat2 {
    <Mat2 as CliffordAlgebra<V2, Z5, SumOfSquares>>::embed(v)
}

#[test]
fn embedding_squares_to_the_quadratic_form() {
    let one = <Mat2 as UnitalMagma<Multiplicative>>::identity();
    for v in all_vectors() {
        let q = <V2 as QuadraticForm<Z5, SumOfSquares>>::value(&v);
        let q_one = <Mat2 as LeftAction<Z5>>::act(&q, &one);
        assert_eq!(mmul(&embed(&v), &embed(&v)), q_one, "embed(v)² = Q(v)·1");
    }
}

#[test]
fn pauli_matrices_anticommute_and_generate_all_matrices() {
    let (ez, ex) = (embed(&V2(Z5(1), Z5(0))), embed(&V2(Z5(0), Z5(1))));
    // σz σx = −σx σz
    assert_eq!(
        mmul(&ez, &ex),
        <Mat2 as Group<Additive>>::inverse(&mmul(&ex, &ez))
    );
    // 1, σz, σx, σzσx sind linear unabhängig: ihre 625 Kombinationen sind alle 625 Matrizen
    let basis = [m(1, 0, 0, 1), ez, ex, mmul(&ez, &ex)];
    let mut seen = std::collections::HashSet::new();
    for c in 0..625u32 {
        let mut sum = m(0, 0, 0, 0);
        for (i, b) in basis.iter().enumerate() {
            let coeff = z(((c / 5u32.pow(i as u32)) % 5) as u8);
            sum = madd(&sum, &<Mat2 as LeftAction<Z5>>::act(&coeff, b));
        }
        seen.insert((sum.0.0, sum.1.0, sum.2.0, sum.3.0));
    }
    assert_eq!(seen.len(), 625);
}

// --- Lie-Darstellung: Spin ½ von gl(2) ---------------------------------------------------------

type L = Commutator<Mat2, Z5>;

/// Die natürliche Darstellung: Matrizen wirken durch Matrix-Vektor-Produkt auf Vektoren.
impl LieModule<L, Z5> for V2 {
    fn lie_act(x: &L, v: &V2) -> V2 {
        let a = x.as_inner();
        V2(
            add(mul(a.0, v.0), mul(a.1, v.1)),
            add(mul(a.2, v.0), mul(a.3, v.1)),
        )
    }
}

fn br(x: &L, y: &L) -> L {
    <L as Magma<Bracket>>::op(x, y)
}

#[test]
fn natural_representation_turns_the_bracket_into_a_commutator() {
    let act = |x: &L, v: &V2| <V2 as LieModule<L, Z5>>::lie_act(x, v);
    let sub = |a: V2, b: V2| <V2 as Magma<Additive>>::op(&a, &<V2 as Group<Additive>>::inverse(&b));
    let xs: Vec<L> = all_matrices().into_iter().step_by(23).map(L::new).collect();
    for x in &xs {
        for y in &xs {
            for v in all_vectors() {
                // ρ([x,y]) v = ρ(x)(ρ(y) v) − ρ(y)(ρ(x) v)
                let lhs = act(&br(x, y), &v);
                let rhs = sub(act(x, &act(y, &v)), act(y, &act(x, &v)));
                assert_eq!(lhs, rhs);
            }
        }
    }
}

#[test]
fn adjoint_representation_is_the_jacobi_identity() {
    let ad = |x: &L, v: &L| <L as LieModule<L, Z5>>::lie_act(x, v);
    let sub = |a: L, b: L| <L as Magma<Additive>>::op(&a, &<L as Group<Additive>>::inverse(&b));
    let xs: Vec<L> = all_matrices().into_iter().step_by(61).map(L::new).collect();
    for x in &xs {
        for y in &xs {
            for v in &xs {
                let lhs = ad(&br(x, y), v);
                let rhs = sub(ad(x, &ad(y, v)), ad(y, &ad(x, v)));
                assert_eq!(lhs, rhs);
            }
        }
    }
}
