//! Die Makros expandieren in `#![no_std]`-Code, in dem der Prelude von `std` fehlt: Sie dürfen
//! nicht auf `Vec`, `String`, `format!` o. Ä. angewiesen sein und müssen `Option` und `Some`
//! selbst qualifizieren.
//!
//! Der Test-Läufer braucht selbst `std`, deshalb wird es nur als Crate eingebunden. Einen echten
//! Bare-Metal-Build der Bibliothek prüft die CI (`thumbv7em-none-eabi`).

#![no_std]

extern crate std;

use math_trait::*;

struct Sum;
#[derive(Clone, Copy, PartialEq, Debug)]
struct Z3(u8);

impl_abelian_group!(Z3, Sum;
    op(a, b) { Z3((a.0 + b.0) % 3) }
    identity() { Z3(0) }
    inverse(a) { Z3((3 - a.0) % 3) }
);

#[derive(Clone, Copy, PartialEq, Debug)]
struct F3(u8);
impl_field!(F3, Additive, Multiplicative;
    add(a, b) { F3((a.0 + b.0) % 3) }
    zero() { F3(0) }
    neg(a) { F3((3 - a.0) % 3) }
    mul(a, b) { F3((a.0 * b.0) % 3) }
    one() { F3(1) }
    recip(a) { match a.0 { 1 => Some(F3(1)), 2 => Some(F3(2)), _ => None } }
);
impl_field_algebra!(F3);

#[test]
fn macros_work_without_the_standard_library_prelude() {
    assert_eq!(<Z3 as Magma<Sum>>::op(&Z3(2), &Z3(2)), Z3(1));
    assert_eq!(<Z3 as Group<Sum>>::inverse(&Z3(1)), Z3(2));
    assert_eq!(<F3 as DivisionRing>::recip(&F3(2)), Some(F3(2)));
    assert_eq!(<F3 as DivisionRing>::recip(&F3(0)), None);
    // eine Konstruktion über dem Körper
    let v = Vector::<F3, 2>::basis(1);
    assert_eq!(v.coords(), &[F3(0), F3(1)]);
}
