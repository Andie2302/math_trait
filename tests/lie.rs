//! Die Lie-Klammer rechnet generisch: `jacobi` und `alternating` prüfen die Gesetze
//! für beliebige Typen. Als Beispiel dient das Kreuzprodukt, die Lie-Algebra so(3).

use math_trait::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3(i32, i32, i32);

impl PartialMagma<Additive> for Vec3 {}
impl UnitalPartialMagma<Additive> for Vec3 {}
impl PartialMagma<Bracket> for Vec3 {}

impl Magma<Additive> for Vec3 {
    fn op(&self, r: &Self) -> Self {
        Vec3(self.0 + r.0, self.1 + r.1, self.2 + r.2)
    }
}
impl UnitalMagma<Additive> for Vec3 {
    fn identity() -> Self {
        Vec3(0, 0, 0)
    }
}
impl Magma<Bracket> for Vec3 {
    fn op(&self, r: &Self) -> Self {
        Vec3(
            self.1 * r.2 - self.2 * r.1,
            self.2 * r.0 - self.0 * r.2,
            self.0 * r.1 - self.1 * r.0,
        )
    }
}

/// `[x,[y,z]] + [y,[z,x]] + [z,[x,y]] = 0`
fn jacobi<T, Br, A>(x: &T, y: &T, z: &T) -> bool
where
    T: Jacobi<Br, A> + PartialEq,
{
    let br = |a: &T, b: &T| <T as Magma<Br>>::op(a, b);
    let add = |a: &T, b: &T| <T as Magma<A>>::op(a, b);
    let t1 = br(x, &br(y, z));
    let t2 = br(y, &br(z, x));
    let t3 = br(z, &br(x, y));
    add(&add(&t1, &t2), &t3) == <T as UnitalMagma<A>>::identity()
}

// Nur die für die Prüfungen nötigen Traits (nicht die ganze Algebra-Kette).
impl Jacobi<Bracket> for Vec3 {}

#[test]
fn cross_product_satisfies_jacobi() {
    let (x, y, z) = (Vec3(1, 2, 3), Vec3(-4, 5, 6), Vec3(7, -8, 9));
    assert!(jacobi::<Vec3, Bracket, Additive>(&x, &y, &z));
}

#[test]
fn cross_product_is_alternating() {
    let x = Vec3(3, -1, 4);
    assert_eq!(
        <Vec3 as Magma<Bracket>>::op(&x, &x),
        <Vec3 as UnitalMagma<Additive>>::identity()
    );
}

#[test]
fn bracket_is_not_associative() {
    let (x, y, z) = (Vec3(1, 0, 0), Vec3(1, 1, 0), Vec3(0, 1, 1));
    let br = |a: &Vec3, b: &Vec3| <Vec3 as Magma<Bracket>>::op(a, b);
    assert_ne!(br(&br(&x, &y), &z), br(&x, &br(&y, &z)));
}
