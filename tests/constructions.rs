//! Die Konstruktionen `Vector`, `Tensor` (V ⊗ W) und `Clifford`, über ℤ/5.

use math_trait::*;

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
fn add(a: Z5, b: Z5) -> Z5 {
    <Z5 as Magma<Additive>>::op(&a, &b)
}
fn mul(a: Z5, b: Z5) -> Z5 {
    <Z5 as Magma<Multiplicative>>::op(&a, &b)
}

// --- Vector ------------------------------------------------------------------------------------

type V2 = Vector<Z5, 2>;
type V3 = Vector<Z5, 3>;

fn v2(a: usize, b: usize) -> V2 {
    Vector::new([z(a), z(b)])
}
fn all_v2() -> Vec<V2> {
    (0..25).map(|i| v2(i, i / 5)).collect()
}

#[test]
fn vector_is_a_vector_space_with_a_basis() {
    fn takes_vs<V: VectorSpace<K>, K: Field>() {}
    takes_vs::<V3, Z5>();
    let e1 = V3::basis(1);
    assert_eq!(e1.coords(), &[Z5(0), Z5(1), Z5(0)]);
    let x = Vector::new([Z5(2), Z5(3), Z5(4)]);
    // x = 2 e0 + 3 e1 + 4 e2
    let mut sum = <V3 as UnitalMagma<Additive>>::identity();
    for (i, c) in x.coords().iter().enumerate() {
        let term = <V3 as LeftAction<Z5>>::act(c, &V3::basis(i));
        sum = <V3 as Magma<Additive>>::op(&sum, &term);
    }
    assert_eq!(sum, x);
}

// --- Tensorprodukt -----------------------------------------------------------------------------

type T22 = Tensor<Z5, 2, 2>;

fn t(v: &V2, w: &V2) -> T22 {
    <T22 as TensorProduct<V2, V2, Z5>>::tensor(v, w)
}

#[test]
fn tensor_of_unequal_dimensions() {
    let v = V3::basis(2);
    let w = Vector::<Z5, 2>::basis(1);
    let x = <Tensor<Z5, 3, 2> as TensorProduct<V3, Vector<Z5, 2>, Z5>>::tensor(&v, &w);
    assert_eq!(
        x.coefficients(),
        &[[Z5(0), Z5(0)], [Z5(0), Z5(0)], [Z5(0), Z5(1)]]
    );
}

#[test]
fn lift_is_the_universal_property() {
    // f(v, w) = v₀w₀ + v₁w₁ ist bilinear; lift(f) ist die Spur
    let dot = |v: &V2, w: &V2| {
        add(
            mul(v.coords()[0], w.coords()[0]),
            mul(v.coords()[1], w.coords()[1]),
        )
    };
    for v in all_v2() {
        for w in all_v2() {
            assert_eq!(t(&v, &w).lift(dot), dot(&v, &w), "lift(v ⊗ w) = f(v, w)");
        }
    }
    // lift ist linear: lift(x + y) = lift(x) + lift(y)
    let a = Tensor::new([[z(1), z(2)], [z(3), z(4)]]);
    let b = Tensor::new([[z(2), z(2)], [z(0), z(3)]]);
    let sum = <T22 as Magma<Additive>>::op(&a, &b);
    assert_eq!(sum.lift(dot), add(a.lift(dot), b.lift(dot)));
}

#[test]
fn entanglement_counts() {
    // Zählt, wie viele der 625 Tensoren rein sind: die 2×2-Matrizen mit Determinante 0
    let mut pure = 0;
    let mut images = std::collections::HashSet::new();
    for i in 0..625usize {
        let x = Tensor::new([[z(i), z(i / 5)], [z(i / 25), z(i / 125)]]);
        if x.is_pure() {
            pure += 1;
        }
    }
    for v in all_v2() {
        for w in all_v2() {
            let x = t(&v, &w);
            assert!(x.is_pure());
            images.insert(*x.coefficients());
        }
    }
    // q³ + q² − q = 145 für q = 5
    assert_eq!(pure, 145);
    assert_eq!(images.len(), 145, "jeder reine Tensor ist ein v ⊗ w");
    // Der Bell-Zustand |00⟩ + |11⟩ gehört zu den 480 verschränkten
    let bell = <T22 as Magma<Additive>>::op(
        &t(&V2::basis(0), &V2::basis(0)),
        &t(&V2::basis(1), &V2::basis(1)),
    );
    assert!(!bell.is_pure());
}

// --- Clifford ----------------------------------------------------------------------------------

struct Ones; // q = (1, 1, …)
struct MinusOnes; // q = (−1, −1, …)
struct Q123; // q = (1, 2, 3, …)
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
impl DiagonalForm<Z5> for Q123 {
    fn square(i: usize) -> Z5 {
        z(i + 1)
    }
}

fn cmul<const D: usize, Q: DiagonalForm<Z5>>(
    a: &Clifford<Z5, D, Q>,
    b: &Clifford<Z5, D, Q>,
) -> Clifford<Z5, D, Q> {
    <Clifford<Z5, D, Q> as Magma<Multiplicative>>::op(a, b)
}
fn cneg<const D: usize, Q: DiagonalForm<Z5>>(a: &Clifford<Z5, D, Q>) -> Clifford<Z5, D, Q> {
    <Clifford<Z5, D, Q> as Group<Additive>>::inverse(a)
}

#[test]
fn generators_square_to_q_and_anticommute() {
    type C = Clifford<Z5, 8, Q123>; // drei Erzeuger mit Quadraten 1, 2, 3
    let e = [C::generator(0), C::generator(1), C::generator(2)];
    for i in 0..3 {
        assert_eq!(cmul(&e[i], &e[i]), C::scalar(z(i + 1)), "e{i}² = q{i}");
        for j in 0..3 {
            if i != j {
                assert_eq!(
                    cmul(&e[i], &e[j]),
                    cneg(&cmul(&e[j], &e[i])),
                    "e{i}e{j} = −e{j}e{i}"
                );
            }
        }
    }
}

#[test]
fn clifford_product_is_associative() {
    type C = Clifford<Z5, 16, Q123>;
    let mut s = 7u32;
    let mut next = || {
        s = s.wrapping_mul(1_103_515_245).wrapping_add(12345);
        z(((s >> 16) % 5) as usize)
    };
    let mut rand = || C::new(std::array::from_fn(|_| next()));
    for _ in 0..40 {
        let (x, y, w) = (rand(), rand(), rand());
        assert_eq!(cmul(&cmul(&x, &y), &w), cmul(&x, &cmul(&y, &w)));
    }
}

#[test]
fn embedding_squares_to_the_quadratic_form() {
    type C = Clifford<Z5, 4, Ones>;
    for v in all_v2() {
        let q = <V2 as QuadraticForm<Z5, Ones>>::value(&v);
        let e = <C as CliffordAlgebra<V2, Z5, Ones>>::embed(&v);
        assert_eq!(cmul(&e, &e), C::scalar(q), "embed(v)² = Q(v)");
    }
}

/// Ordnet die Koeffizienten der Clifford-Algebra mit einem Erzeuger den komplexen Zahlen
/// aus Cayley-Dickson zu: `a + b·e₀ ↦ (a, b)`.
#[test]
fn one_generator_with_q_minus_one_is_the_complex_numbers() {
    type C = Clifford<Z5, 2, MinusOnes>;
    type K = CayleyDickson<Z5, Z5>;
    let to_k = |x: &C| K::new(x.coefficients()[0], x.coefficients()[1]);
    for i in 0..25 {
        for j in 0..25 {
            let (x, y) = (C::new([z(i), z(i / 5)]), C::new([z(j), z(j / 5)]));
            let lhs = to_k(&cmul(&x, &y));
            let rhs = <K as Magma<Multiplicative>>::op(&to_k(&x), &to_k(&y));
            assert_eq!(lhs, rhs);
        }
    }
}

/// Zwei Erzeuger mit `q = (−1, −1)` ergeben die Quaternionen. Auf der Cayley-Dickson-Seite
/// entspricht `c₀ + c₁e₀ + c₂e₁ + c₃e₀e₁` dem Paar `((c₀, c₁), (c₂, c₃))`.
#[test]
fn two_generators_with_q_minus_one_are_the_quaternions() {
    type C = Clifford<Z5, 4, MinusOnes>;
    type H = CayleyDickson<CayleyDickson<Z5, Z5>, Z5>;
    let to_h = |x: &C| {
        let c = x.coefficients();
        H::new(
            CayleyDickson::new(c[0], c[1]),
            CayleyDickson::new(c[2], c[3]),
        )
    };
    let all: Vec<C> = (0..625usize)
        .map(|i| C::new([z(i), z(i / 5), z(i / 25), z(i / 125)]))
        .collect();
    for x in &all {
        for y in &all {
            let lhs = to_h(&cmul(x, y));
            let rhs = <H as Magma<Multiplicative>>::op(&to_h(x), &to_h(y));
            assert_eq!(lhs, rhs);
        }
    }
}

// --- Universelle Eigenschaft: Cl(ℤ/5², x² + y²) ≅ M₂(ℤ/5) -----------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
struct Mat2([[Z5; 2]; 2]);

fn mneg(a: Z5) -> Z5 {
    <Z5 as Group<Additive>>::inverse(&a)
}

impl_abelian_group!(Mat2, Additive;
    op(x, y) { Mat2([[add(x.0[0][0], y.0[0][0]), add(x.0[0][1], y.0[0][1])],
                     [add(x.0[1][0], y.0[1][0]), add(x.0[1][1], y.0[1][1])]]) }
    identity() { Mat2([[Z5(0); 2]; 2]) }
    inverse(x) { Mat2([[mneg(x.0[0][0]), mneg(x.0[0][1])], [mneg(x.0[1][0]), mneg(x.0[1][1])]]) }
);
impl_module!(Mat2, Z5; act(s, x) {
    Mat2([[mul(*s, x.0[0][0]), mul(*s, x.0[0][1])], [mul(*s, x.0[1][0]), mul(*s, x.0[1][1])]])
});
impl_monoid!(Mat2, Multiplicative;
    op(x, y) {
        let e = |i: usize, j: usize| add(mul(x.0[i][0], y.0[0][j]), mul(x.0[i][1], y.0[1][j]));
        Mat2([[e(0, 0), e(0, 1)], [e(1, 0), e(1, 1)]])
    }
    identity() { Mat2([[Z5(1), Z5(0)], [Z5(0), Z5(1)]]) }
);
impl_algebra!(Mat2, Z5, Multiplicative);
impl_unital_algebra!(Mat2, Z5, Multiplicative);
impl_associative_algebra!(Mat2, Z5, Multiplicative);

#[test]
fn lift_to_the_pauli_matrices_is_an_isomorphism() {
    type C = Clifford<Z5, 4, Ones>;
    // σz = diag(1, −1), σx = [[0, 1], [1, 0]]: σ² = 1 und σzσx = −σxσz
    let gens = [
        Mat2([[Z5(1), Z5(0)], [Z5(0), Z5(4)]]),
        Mat2([[Z5(0), Z5(1)], [Z5(1), Z5(0)]]),
    ];
    let h = |x: &C| x.lift(&gens);
    let all: Vec<C> = (0..625usize)
        .map(|i| C::new([z(i), z(i / 5), z(i / 25), z(i / 125)]))
        .collect();
    // Homomorphismus: h(x·y) = h(x)·h(y)
    for x in all.iter().step_by(7) {
        for y in &all {
            let lhs = h(&cmul(x, y));
            let rhs = <Mat2 as Magma<Multiplicative>>::op(&h(x), &h(y));
            assert_eq!(lhs, rhs);
        }
    }
    // bijektiv: alle 625 Matrizen werden getroffen
    let images: std::collections::HashSet<[[u8; 2]; 2]> = all
        .iter()
        .map(|x| {
            let m = h(x).0;
            [[m[0][0].0, m[0][1].0], [m[1][0].0, m[1][1].0]]
        })
        .collect();
    assert_eq!(images.len(), 625);
}
