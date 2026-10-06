//! Rechnen ohne Basistypen im Crate: Die Funktionen sind generisch über die Struktur,
//! die Beispieltypen sind lokale Testobjekte.

use math_trait::numeric::*;
use math_trait::*;

// Hilfsmakro: implementiert alle Marker-Traits einer Liste für einen Typ und ein Etikett.
macro_rules! markers {
    ($t:ty, $op:ty: $($tr:ident),+ $(,)?) => { $( impl $tr<$op> for $t {} )+ };
}

// --- Beispiel 1: Text unter Verkettung (Monoid, nicht mathematisch) ---------

struct Concat;
#[derive(Debug, Clone, PartialEq)]
struct Text(String);

impl Magma<Concat> for Text {
    fn op(&self, rhs: &Self) -> Self {
        Text(format!("{}{}", self.0, rhs.0))
    }
}
impl UnitalMagma<Concat> for Text {
    fn identity() -> Self {
        Text(String::new())
    }
}
markers!(Text, Concat: PartialMagma, Semigroupoid, UnitalPartialMagma, SmallCategory,
    Alternative, Flexible, PowerAssociative, Semigroup, Monoid);

// --- Beispiel 2: ℤ/5 unter Addition (abelsche Gruppe) -----------------------

#[derive(Debug, Clone, Copy, PartialEq)]
struct Z5(u8);

impl Magma<Additive> for Z5 {
    fn op(&self, rhs: &Self) -> Self {
        Z5((self.0 + rhs.0) % 5)
    }
}
impl UnitalMagma<Additive> for Z5 {
    fn identity() -> Self {
        Z5(0)
    }
}
impl Quasigroup<Additive> for Z5 {
    fn ldiv(&self, b: &Self) -> Self {
        Z5((b.0 + 5 - self.0) % 5)
    }
    fn rdiv(&self, b: &Self) -> Self {
        Z5((b.0 + 5 - self.0) % 5)
    }
}
impl Group<Additive> for Z5 {
    fn inverse(&self) -> Self {
        Z5((5 - self.0) % 5)
    }
}
markers!(Z5, Additive: PartialMagma, Semigroupoid, UnitalPartialMagma, SmallCategory, Groupoid,
    LeftCancellative, RightCancellative, Cancellative, Alternative, Flexible,
    PowerAssociative, Semigroup, Monoid, Loop, AssociativeQuasigroup, Commutative,
    Trimedial, Medial, CommutativeSemigroup, CommutativeMonoid, AbelianGroup);

// --- generische Rechnungen ---------------------------------------------------

/// Faltet eine Liste mit der Verknüpfung, beginnend beim neutralen Element.
fn sum<T: Monoid<Op>, Op>(xs: &[T]) -> T {
    xs.iter()
        .fold(<T as UnitalMagma<Op>>::identity(), |acc, x| {
            <T as Magma<Op>>::op(&acc, x)
        })
}

/// `x` mit sich selbst verknüpft, `n`-mal.
fn power<T: Monoid<Op>, Op>(x: &T, n: u32) -> T {
    (0..n).fold(<T as UnitalMagma<Op>>::identity(), |acc, _| {
        <T as Magma<Op>>::op(&acc, x)
    })
}

#[test]
fn works_for_non_mathematical_monoid() {
    let words = [Text("ab".into()), Text("cd".into()), Text("ef".into())];
    assert_eq!(sum::<_, Concat>(&words), Text("abcdef".into()));
    assert_eq!(power::<_, Concat>(&Text("x".into()), 3), Text("xxx".into()));
}

#[test]
fn works_for_group() {
    assert_eq!(sum::<_, Additive>(&[Z5(1), Z5(2), Z5(3), Z5(4)]), Z5(0));
    assert_eq!(power::<_, Additive>(&Z5(2), 4), Z5(3));
    let a = Z5(3);
    assert_eq!(
        <Z5 as Magma<Additive>>::op(&a, &<Z5 as Group<Additive>>::inverse(&a)),
        <Z5 as UnitalMagma<Additive>>::identity()
    );
    assert_eq!(<Z5 as Quasigroup<Additive>>::ldiv(&Z5(2), &Z5(1)), Z5(4));
}

// --- Zahlen-Traits -----------------------------------------------------------

#[derive(Debug, PartialEq)]
struct Q(u8);

impl Magma<Additive> for Q {
    fn op(&self, rhs: &Self) -> Self {
        Q(self.0.saturating_add(rhs.0))
    }
}
impl Magma<Multiplicative> for Q {
    fn op(&self, rhs: &Self) -> Self {
        Q(self.0.saturating_mul(rhs.0))
    }
}
impl UnitalMagma<Additive> for Q {
    fn identity() -> Self {
        Q(0)
    }
}
impl UnitalMagma<Multiplicative> for Q {
    fn identity() -> Self {
        Q(1)
    }
}
markers!(Q, Additive: PartialMagma, UnitalPartialMagma, Flexible, Commutative);
markers!(Q, Multiplicative: PartialMagma, UnitalPartialMagma, Flexible, Commutative);
impl Number for Q {}
impl Integer for Q {}
impl Unsigned for Q {}

#[test]
fn number_traits() {
    assert_eq!(Q::zero(), Q(0));
    assert_eq!(Q::one(), Q(1));
}
