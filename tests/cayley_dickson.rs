//! Der Cayley-Dickson-Turm über ℤ/5: Körper → 2-, 4-, 8-, 16-dimensionale Algebra.
//!
//! Über ℤ/5 ist −1 ein Quadrat, die Algebren sind deshalb "gespalten" (mit Nullteilern), aber
//! die Reihenfolge der verlorenen Eigenschaften ist dieselbe wie bei ℝ → ℂ → ℍ → 𝕆 → 𝕊.

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

type C1 = CayleyDickson<Z5, Z5>; // 2-dimensional  (ℂ)
type C2 = CayleyDickson<C1, Z5>; // 4-dimensional  (ℍ)
type C3 = CayleyDickson<C2, Z5>; // 8-dimensional  (𝕆)
type C4 = CayleyDickson<C3, Z5>; // 16-dimensional (𝕊)

// --- Was der Compiler weiß ----------------------------------------------------------------

fn composition<A: CompositionAlgebra<K>, K: Field>() {}
fn associative<A: AssociativeAlgebra<K>, K: CommutativeRing>() {}
fn alternative<A: AlternativeAlgebra<K>, K: CommutativeRing>() {}
fn commutative<A: Commutative<Multiplicative>>() {}
fn involutive<A: AlgebraWithInvolution<K> + UnitalAlgebra<K>, K: CommutativeRing>() {}

#[test]
fn type_level_properties() {
    composition::<Z5, Z5>();
    composition::<C1, Z5>();
    composition::<C2, Z5>();
    composition::<C3, Z5>();
    associative::<Z5, Z5>();
    associative::<C1, Z5>();
    associative::<C2, Z5>();
    alternative::<C1, Z5>();
    alternative::<C2, Z5>();
    alternative::<C3, Z5>();
    commutative::<Z5>();
    commutative::<C1>();
    involutive::<C4, Z5>(); // das 16-dimensionale hat Einselement, Involution und Norm-Form, aber keine Kompositionseigenschaft
}

// --- Stichproben --------------------------------------------------------------------------

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
impl<A: Rand, R> Rand for CayleyDickson<A, R> {
    fn rand(r: &mut Rng) -> Self {
        let a = A::rand(r);
        CayleyDickson::new(a, A::rand(r))
    }
}

fn mul<T: Magma<Multiplicative>>(a: &T, b: &T) -> T {
    <T as Magma<Multiplicative>>::op(a, b)
}
fn add<T: Magma<Additive>>(a: &T, b: &T) -> T {
    <T as Magma<Additive>>::op(a, b)
}
fn conj<T: Involutive>(a: &T) -> T {
    <T as Involutive>::conjugate(a)
}

const N: usize = 60;

fn samples<T: Rand>(seed: u32, n: usize) -> Vec<T> {
    let mut r = Rng(seed);
    (0..n).map(|_| T::rand(&mut r)).collect()
}

/// Zählt die Tripel, bei denen das Gesetz verletzt ist.
fn violations<T: Rand>(law: impl Fn(&T, &T, &T) -> bool) -> usize {
    let (xs, ys, zs) = (samples::<T>(1, N), samples::<T>(2, N), samples::<T>(3, N));
    (0..N).filter(|&i| !law(&xs[i], &ys[i], &zs[i])).count()
}

fn assoc<T>(x: &T, y: &T, z: &T) -> bool
where
    T: Magma<Multiplicative> + PartialEq,
{
    mul(&mul(x, y), z) == mul(x, &mul(y, z))
}
fn commut<T>(x: &T, y: &T, _z: &T) -> bool
where
    T: Magma<Multiplicative> + PartialEq,
{
    mul(x, y) == mul(y, x)
}
/// `(x x) y = x (x y)` und `(y x) x = y (x x)`
fn alt<T>(x: &T, y: &T, _z: &T) -> bool
where
    T: Magma<Multiplicative> + PartialEq,
{
    mul(&mul(x, x), y) == mul(x, &mul(x, y)) && mul(&mul(y, x), x) == mul(y, &mul(x, x))
}
fn distrib<T>(x: &T, y: &T, z: &T) -> bool
where
    T: Magma<Multiplicative> + Magma<Additive> + PartialEq,
{
    mul(x, &add(y, z)) == add(&mul(x, y), &mul(x, z))
        && mul(&add(y, z), x) == add(&mul(y, x), &mul(z, x))
}
fn conj_anti<T>(x: &T, y: &T, _z: &T) -> bool
where
    T: Magma<Multiplicative> + Involutive + PartialEq,
{
    conj(&conj(x)) == *x && conj(&mul(x, y)) == mul(&conj(y), &conj(x))
}
fn norm_mult<T>(x: &T, y: &T, _z: &T) -> bool
where
    T: Magma<Multiplicative> + QuadraticForm<Z5, Norm>,
{
    <T as QuadraticForm<Z5, Norm>>::value(&mul(x, y))
        == <Z5 as Magma<Multiplicative>>::op(
            &<T as QuadraticForm<Z5, Norm>>::value(x),
            &<T as QuadraticForm<Z5, Norm>>::value(y),
        )
}

#[test]
fn associativity_is_lost_at_dimension_eight() {
    assert_eq!(violations::<Z5>(assoc), 0);
    assert_eq!(violations::<C1>(assoc), 0);
    assert_eq!(violations::<C2>(assoc), 0);
    assert!(violations::<C3>(assoc) > 0);
    assert!(violations::<C4>(assoc) > 0);
}

#[test]
fn commutativity_is_lost_at_dimension_four() {
    assert_eq!(violations::<Z5>(commut), 0);
    assert_eq!(violations::<C1>(commut), 0);
    assert!(violations::<C2>(commut) > 0);
}

#[test]
fn alternativity_is_lost_at_dimension_sixteen() {
    assert_eq!(violations::<C1>(alt), 0);
    assert_eq!(violations::<C2>(alt), 0);
    assert_eq!(violations::<C3>(alt), 0);
    assert!(violations::<C4>(alt) > 0);
}

#[test]
fn norm_is_multiplicative_up_to_dimension_eight() {
    assert_eq!(violations::<Z5>(norm_mult), 0);
    assert_eq!(violations::<C1>(norm_mult), 0);
    assert_eq!(violations::<C2>(norm_mult), 0);
    assert_eq!(violations::<C3>(norm_mult), 0);
}

/// Die Norm-Form ist `N(x) = x ⋅ x*` (als Skalar mal Eins), auf jeder Stufe.
fn norm_is_x_times_conjugate<T>(x: &T, _y: &T, _z: &T) -> bool
where
    T: UnitalAlgebra<Z5> + AlgebraWithInvolution<Z5> + QuadraticForm<Z5, Norm> + PartialEq,
{
    let n = <T as QuadraticForm<Z5, Norm>>::value(x);
    let one = <T as UnitalMagma<Multiplicative>>::identity();
    mul(x, &conj(x)) == <T as LeftAction<Z5>>::act(&n, &one)
}

/// `N(s ⋅ x) = s² ⋅ N(x)`.
fn norm_is_homogeneous<T>(x: &T, _y: &T, _z: &T) -> bool
where
    T: UnitalAlgebra<Z5> + QuadraticForm<Z5, Norm>,
{
    let nx = <T as QuadraticForm<Z5, Norm>>::value(x);
    (0..5u8).all(|s| {
        let s = Z5(s);
        let sx = <T as LeftAction<Z5>>::act(&s, x);
        let s2 = <Z5 as Magma<Multiplicative>>::op(&s, &s);
        <T as QuadraticForm<Z5, Norm>>::value(&sx) == <Z5 as Magma<Multiplicative>>::op(&s2, &nx)
    })
}

#[test]
fn norm_form_is_x_times_conjugate_and_homogeneous_at_every_level() {
    assert_eq!(violations::<Z5>(norm_is_x_times_conjugate), 0);
    assert_eq!(violations::<C1>(norm_is_x_times_conjugate), 0);
    assert_eq!(violations::<C2>(norm_is_x_times_conjugate), 0);
    assert_eq!(violations::<C3>(norm_is_x_times_conjugate), 0);
    assert_eq!(violations::<C4>(norm_is_x_times_conjugate), 0);
    assert_eq!(violations::<C3>(norm_is_homogeneous), 0);
}

#[test]
fn product_is_bilinear_and_conjugation_is_an_involution_at_every_level() {
    assert_eq!(violations::<C1>(distrib), 0);
    assert_eq!(violations::<C2>(distrib), 0);
    assert_eq!(violations::<C3>(distrib), 0);
    assert_eq!(violations::<C4>(distrib), 0);
    assert_eq!(violations::<C1>(conj_anti), 0);
    assert_eq!(violations::<C2>(conj_anti), 0);
    assert_eq!(violations::<C3>(conj_anti), 0);
    assert_eq!(violations::<C4>(conj_anti), 0);
}

#[test]
fn one_is_the_identity_at_every_level() {
    fn check<T: Rand + Magma<Multiplicative> + UnitalMagma<Multiplicative> + PartialEq>() {
        let one = <T as UnitalMagma<Multiplicative>>::identity();
        for x in samples::<T>(7, 20) {
            assert!(mul(&one, &x) == x && mul(&x, &one) == x);
        }
    }
    check::<C1>();
    check::<C2>();
    check::<C3>();
    check::<C4>();
}

// --- Die Quaternionen-Relationen i² = j² = k² = ijk = −1 -----------------------------------

#[test]
fn quaternion_relations() {
    let zero = Z5(0);
    let one = Z5(1);
    let c1 = |a, b| C1::new(a, b);
    let (c_zero, c_one) = (c1(zero, zero), c1(one, zero));
    let i = C2::new(c1(zero, one), c_zero); // i = (j₁, 0)
    let j = C2::new(c_zero, c_one); //          j = (0, 1)
    let k = mul(&i, &j);
    let minus_one =
        <C2 as Group<Additive>>::inverse(&<C2 as UnitalMagma<Multiplicative>>::identity());
    assert_eq!(mul(&i, &i), minus_one);
    assert_eq!(mul(&j, &j), minus_one);
    assert_eq!(mul(&k, &k), minus_one);
    assert_eq!(mul(&mul(&i, &j), &k), minus_one);
    // nicht kommutativ: ij = −ji
    assert_eq!(mul(&i, &j), <C2 as Group<Additive>>::inverse(&mul(&j, &i)));
}

#[test]
fn over_z5_the_two_dimensional_algebra_has_zero_divisors() {
    // (2 + e)(2 − e) = 4 − e² = 4 + 1 = 0, obwohl keiner der Faktoren null ist
    let p = C1::new(Z5(2), Z5(1));
    let q = C1::new(Z5(2), Z5(4));
    let zero = <C1 as UnitalMagma<Additive>>::identity();
    assert_eq!(mul(&p, &q), zero);
    assert_ne!(p, zero);
    assert_ne!(q, zero);
}
