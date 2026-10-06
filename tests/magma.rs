//! Beweist, dass das Design trägt – ohne ein einziges Primitiv im Crate.

use math_trait::{Additive, BinaryOp, Magma, Multiplicative, Operation};

// --- Ein Typ mit zwei Verknüpfungen (Ring-artig) -------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
struct Z5(u8);

impl Magma<Additive> for Z5 {
    fn op(&self, rhs: &Self) -> Self {
        Z5((self.0 + rhs.0) % 5)
    }
}
impl Magma<Multiplicative> for Z5 {
    fn op(&self, rhs: &Self) -> Self {
        Z5((self.0 * rhs.0) % 5)
    }
}

#[test]
fn one_type_two_operations() {
    let (a, b) = (Z5(3), Z5(4));
    assert_eq!(Magma::<Additive>::op(&a, &b), Z5(2));
    assert_eq!(Magma::<Multiplicative>::op(&a, &b), Z5(2));
    assert_eq!(Magma::<Additive>::op(&a, &Z5(1)), Z5(4));
}

// --- Ein Magma ohne Assoziativität: Schere-Stein-Papier ------------------

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

struct Beats;
impl Operation for Beats {}

impl Magma<Beats> for Rps {
    fn op(&self, rhs: &Self) -> Self {
        use Rps::*;
        match (self, rhs) {
            (Rock, Scissors) | (Scissors, Rock) => Rock,
            (Paper, Rock) | (Rock, Paper) => Paper,
            (Scissors, Paper) | (Paper, Scissors) => Scissors,
            (x, _) => *x,
        }
    }
}

#[test]
fn magma_needs_no_associativity() {
    use Rps::*;
    let (a, b, c) = (Rock, Paper, Scissors);
    let left = a.op(&b).op(&c);
    let right = a.op(&b.op(&c));
    assert_ne!(left, right);
}

// --- Nicht-mathematisch, nicht-Copy, nicht-Clone -------------------------

struct Concat;
impl Operation for Concat {}

#[derive(Debug, PartialEq)]
struct Text(String);

impl Magma<Concat> for Text {
    fn op(&self, rhs: &Self) -> Self {
        Text(format!("{}{}", self.0, rhs.0))
    }
}

#[test]
fn non_math_magma_and_owned_variant() {
    let t = Magma::<Concat>::op_owned(Text("ab".into()), Text("cd".into()));
    assert_eq!(t, Text("abcd".into()));
}

// --- Magma ist automatisch BinaryOp; BinaryOp bleibt heterogen möglich ---

fn apply_generic<Op: Operation, A: BinaryOp<Op, B>, B>(a: &A, b: &B) -> A::Output {
    a.apply(b)
}

#[test]
fn magma_is_binary_op() {
    assert_eq!(apply_generic::<Additive, _, _>(&Z5(2), &Z5(4)), Z5(1));
}

struct Scale;
impl Operation for Scale {}

#[derive(Debug, PartialEq)]
struct Vec2(i32, i32);

impl BinaryOp<Scale, i32> for Vec2 {
    type Output = Vec2;
    fn apply(&self, k: &i32) -> Vec2 {
        Vec2(self.0 * k, self.1 * k)
    }
}

#[test]
fn heterogeneous_binary_op_coexists() {
    assert_eq!(apply_generic::<Scale, _, _>(&Vec2(1, 2), &3), Vec2(3, 6));
}

// --- Darauf aufbauen: ein eigenes Trait über Magma -----------------------

/// So würde z. B. `Semigroup` aussehen: Marker + Gesetz in der Doku.
trait Semigroup<Op: Operation>: Magma<Op> {}
impl Semigroup<Additive> for Z5 {}

fn fold_all<Op: Operation, S: Semigroup<Op> + Clone>(items: &[S]) -> Option<S> {
    let (first, rest) = items.split_first()?;
    Some(
        rest.iter()
            .fold(first.clone(), |acc, x| Magma::<Op>::op(&acc, x)),
    )
}

#[test]
fn build_on_top_of_magma() {
    let r = fold_all::<Additive, _>(&[Z5(1), Z5(2), Z5(3), Z5(4)]);
    assert_eq!(r, Some(Z5(0)));
}
