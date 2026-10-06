//! Der Parameter γ: mit `γ = 2` (kein Quadrat in ℤ/5) wird die Verdopplung von ℤ/5 zu einem
//! Körper mit 25 Elementen, mit `γ = 1` bleibt sie gespalten (Nullteiler).

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
impl_field_algebra!(Z5);

macro_rules! gamma {
    ($name:ident = $v:expr) => {
        struct $name;
        impl Gamma<Z5> for $name {
            fn gamma() -> Z5 {
                Z5($v)
            }
        }
    };
}
gamma!(One = 1);
gamma!(Two = 2);
gamma!(Three = 3);

fn mul<T: Magma<Multiplicative>>(a: &T, b: &T) -> T {
    <T as Magma<Multiplicative>>::op(a, b)
}
fn add<T: Magma<Additive>>(a: &T, b: &T) -> T {
    <T as Magma<Additive>>::op(a, b)
}
fn conj<T: Involutive>(a: &T) -> T {
    <T as Involutive>::conjugate(a)
}

#[test]
fn j_squared_is_gamma() {
    fn j<G: Gamma<Z5>>() -> CayleyDickson<Z5, Z5, G> {
        CayleyDickson::new(Z5(0), Z5(1))
    }
    let g = |a: Z5| CayleyDickson::<Z5, Z5, MinusOne>::new(a, Z5(0));
    assert_eq!(mul(&j::<MinusOne>(), &j::<MinusOne>()), g(Z5(4)));
    let h = |a: Z5| CayleyDickson::<Z5, Z5, One>::new(a, Z5(0));
    assert_eq!(mul(&j::<One>(), &j::<One>()), h(Z5(1)));
    let t = |a: Z5| CayleyDickson::<Z5, Z5, Two>::new(a, Z5(0));
    assert_eq!(mul(&j::<Two>(), &j::<Two>()), t(Z5(2)));
}

fn nonzero_pairs() -> Vec<(u8, u8)> {
    (0..5)
        .flat_map(|a| (0..5).map(move |b| (a, b)))
        .filter(|&p| p != (0, 0))
        .collect()
}

#[test]
fn gamma_two_gives_a_field_with_25_elements() {
    type F = CayleyDickson<Z5, Z5, Two>;
    let zero = <F as UnitalMagma<Additive>>::identity();
    for &(a, b) in &nonzero_pairs() {
        for &(c, d) in &nonzero_pairs() {
            let p = mul(&F::new(Z5(a), Z5(b)), &F::new(Z5(c), Z5(d)));
            assert_ne!(p, zero, "kein Nullteiler: ({a},{b})·({c},{d})");
        }
    }
    // Die Norm hat keine nichttrivialen Nullstellen: a² − 2b² ≠ 0
    for &(a, b) in &nonzero_pairs() {
        let n = <F as QuadraticForm<Z5, Norm>>::value(&F::new(Z5(a), Z5(b)));
        assert_ne!(n, Z5(0));
    }
}

#[test]
fn gamma_one_stays_split() {
    type S = CayleyDickson<Z5, Z5, One>;
    // (1 + j)(1 − j) = 1 − j² = 1 − 1 = 0
    let p = S::new(Z5(1), Z5(1));
    let q = S::new(Z5(1), Z5(4));
    assert_eq!(mul(&p, &q), <S as UnitalMagma<Additive>>::identity());
}

// --- Gemischte γ auf jeder Stufe -----------------------------------------------------------------

type D1 = CayleyDickson<Z5, Z5, Two>;
type D2 = CayleyDickson<D1, Z5, Three>;
type D3 = CayleyDickson<D2, Z5, MinusOne>;
type D4 = CayleyDickson<D3, Z5, One>;

struct Rng(u32);
impl Rng {
    fn next(&mut self) -> u8 {
        self.0 = self.0.wrapping_mul(1_103_515_245).wrapping_add(12345);
        ((self.0 >> 16) % 5) as u8
    }
}
trait Rand {
    fn rand(r: &mut Rng) -> Self;
}
impl Rand for Z5 {
    fn rand(r: &mut Rng) -> Self {
        Z5(r.next())
    }
}
impl<A: Rand, R, G> Rand for CayleyDickson<A, R, G> {
    fn rand(r: &mut Rng) -> Self {
        let a = A::rand(r);
        CayleyDickson::new(a, A::rand(r))
    }
}

const N: usize = 60;

fn violations<T: Rand>(law: impl Fn(&T, &T, &T) -> bool) -> usize {
    let mut rs = [Rng(1), Rng(2), Rng(3)];
    let xs: Vec<T> = (0..N).map(|_| T::rand(&mut rs[0])).collect();
    let ys: Vec<T> = (0..N).map(|_| T::rand(&mut rs[1])).collect();
    let zs: Vec<T> = (0..N).map(|_| T::rand(&mut rs[2])).collect();
    (0..N).filter(|&i| !law(&xs[i], &ys[i], &zs[i])).count()
}

fn assoc<T: Magma<Multiplicative> + PartialEq>(x: &T, y: &T, z: &T) -> bool {
    mul(&mul(x, y), z) == mul(x, &mul(y, z))
}
fn alt<T: Magma<Multiplicative> + PartialEq>(x: &T, y: &T, _z: &T) -> bool {
    mul(&mul(x, x), y) == mul(x, &mul(x, y)) && mul(&mul(y, x), x) == mul(y, &mul(x, x))
}
fn distrib<T: Magma<Multiplicative> + Magma<Additive> + PartialEq>(x: &T, y: &T, z: &T) -> bool {
    mul(x, &add(y, z)) == add(&mul(x, y), &mul(x, z))
}
fn conj_anti<T: Magma<Multiplicative> + Involutive + PartialEq>(x: &T, y: &T, _z: &T) -> bool {
    conj(&conj(x)) == *x && conj(&mul(x, y)) == mul(&conj(y), &conj(x))
}
fn norm_mult<T: Magma<Multiplicative> + QuadraticForm<Z5, Norm>>(x: &T, y: &T, _z: &T) -> bool {
    <T as QuadraticForm<Z5, Norm>>::value(&mul(x, y))
        == mul(
            &<T as QuadraticForm<Z5, Norm>>::value(x),
            &<T as QuadraticForm<Z5, Norm>>::value(y),
        )
}

#[test]
fn the_same_series_of_lost_properties_for_any_gamma() {
    assert_eq!(violations::<D2>(assoc), 0);
    assert!(violations::<D3>(assoc) > 0);
    assert_eq!(violations::<D3>(alt), 0);
    assert!(violations::<D4>(alt) > 0);
    assert_eq!(violations::<D1>(norm_mult), 0);
    assert_eq!(violations::<D2>(norm_mult), 0);
    assert_eq!(violations::<D3>(norm_mult), 0);
}

#[test]
fn bilinear_and_conjugation_for_mixed_gammas() {
    assert_eq!(violations::<D4>(distrib), 0);
    assert_eq!(violations::<D4>(conj_anti), 0);
    assert_eq!(violations::<D2>(conj_anti), 0);
}

#[test]
fn type_level_properties_are_independent_of_gamma() {
    fn composition<A: CompositionAlgebra<K>, K: Field>() {}
    fn associative<A: AssociativeAlgebra<K>, K: CommutativeRing>() {}
    fn alternative<A: AlternativeAlgebra<K>, K: CommutativeRing>() {}
    composition::<D1, Z5>();
    composition::<D2, Z5>();
    composition::<D3, Z5>();
    associative::<D2, Z5>();
    alternative::<D3, Z5>();
}
