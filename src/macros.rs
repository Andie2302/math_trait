//! Makros, die alle nötigen Trait-Impls für eine Struktur auf einmal erzeugen.
//!
//! Jede algebraische Struktur verlangt viele Marker-Impls (eine Gruppe z. B. 15). Die Makros
//! erzeugen sie und die Methoden aus kurzen Rümpfen. Die *Gesetze* (Assoziativität, Inverse, …)
//! prüft kein Makro: Wer ein Makro verwendet, behauptet, dass sie gelten.
//!
//! # Reihen
//!
//! Für einen Typ und ein Etikett nimmt man genau **ein** Makro der Reihe `magma` → `semigroup` →
//! `monoid` → `group`, in der kommutativen Variante `commutative_…` bzw. `abelian_group`. Daneben
//! gibt es `unital_magma` (Magma mit Eins, ohne Assoziativität).
//!
//! Bei Algebren gilt ein anderes Muster: `impl_algebra!` ist die Basis, danach kommen
//! Zusatz-Makros, die jeweils nur *ein* Marker-Trait hinzufügen (`impl_unital_algebra!`,
//! `impl_associative_algebra!`, `impl_alternative_algebra!`, `impl_division_algebra!`).
//!
//! # Generische Typen
//!
//! Jedes Makro nimmt optional eine Liste von Typparametern mit Bedingungen vorweg:
//!
//! ```text
//! impl_abelian_group!(for [T: AbelianGroup<Additive>] Pair<T>, Additive; op(a, b) { … } … );
//! ```
//!
//! Die Liste steht in `[ ]` und enthält genau das, was in `impl<…>` stehen würde.

/// Implementiert eine Liste von Marker-Traits für einen Typ und ein Etikett.
#[doc(hidden)]
#[macro_export]
macro_rules! __markers {
    ([$($g:tt)*] $t:ty, $op:ty:) => {};
    ([$($g:tt)*] $t:ty, $op:ty: $tr:ident $(, $rest:ident)* $(,)?) => {
        impl<$($g)*> $crate::$tr<$op> for $t {}
        $crate::__markers!([$($g)*] $t, $op: $($rest),*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_op {
    ([$($g:tt)*] $t:ty, $op:ty, $a:ident, $b:ident, $body:block) => {
        impl<$($g)*> $crate::Magma<$op> for $t {
            fn op(&self, rhs: &Self) -> Self {
                let $a = self;
                let $b = rhs;
                $body
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_identity {
    ([$($g:tt)*] $t:ty, $op:ty, $body:block) => {
        impl<$($g)*> $crate::UnitalMagma<$op> for $t {
            fn identity() -> Self $body
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_group_methods {
    ([$($g:tt)*] $t:ty, $op:ty, $x:ident, $body:block) => {
        impl<$($g)*> $crate::Quasigroup<$op> for $t {
            fn ldiv(&self, b: &Self) -> Self {
                <$t as $crate::Magma<$op>>::op(&<$t as $crate::Group<$op>>::inverse(self), b)
            }
            fn rdiv(&self, b: &Self) -> Self {
                <$t as $crate::Magma<$op>>::op(b, &<$t as $crate::Group<$op>>::inverse(self))
            }
        }
        impl<$($g)*> $crate::Group<$op> for $t {
            fn inverse(&self) -> Self {
                let $x = self;
                $body
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __semigroup_markers {
    ($g:tt $t:ty, $op:ty) => {
        $crate::__markers!($g $t, $op: PartialMagma, Semigroupoid, Alternative, Flexible,
            PowerAssociative, Semigroup);
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __commutative_markers {
    ($g:tt $t:ty, $op:ty) => {
        $crate::__markers!($g $t, $op: Commutative, Trimedial, Medial, CommutativeSemigroup);
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __monoid_markers {
    ($g:tt $t:ty, $op:ty) => {
        $crate::__markers!($g $t, $op: UnitalPartialMagma, SmallCategory, Monoid);
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __group_markers {
    ($g:tt $t:ty, $op:ty) => {
        $crate::__markers!($g $t, $op: LeftCancellative, RightCancellative, Cancellative,
            Groupoid, Loop, AssociativeQuasigroup);
    };
}

/// Magma: nur die Verknüpfung `op`. Es werden keine Gesetze behauptet.
///
/// ```
/// use math_trait::{impl_magma, Magma, Additive};
/// struct Avg(f64);
/// impl_magma!(Avg, Additive; op(a, b) { Avg((a.0 + b.0) / 2.0) });
/// let m = <Avg as Magma<Additive>>::op(&Avg(1.0), &Avg(3.0));
/// assert_eq!(m.0, 2.0);
/// ```
#[macro_export]
macro_rules! impl_magma {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::__markers!([$($g)*] $t, $op: PartialMagma);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::impl_magma!(for [] $t, $op; op($a, $b) $body);
    };
}

/// Kommutatives Magma: `op` ist kommutativ.
#[macro_export]
macro_rules! impl_commutative_magma {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::__markers!([$($g)*] $t, $op: PartialMagma, Commutative, Flexible);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::impl_commutative_magma!(for [] $t, $op; op($a, $b) $body);
    };
}

/// Magma mit neutralem Element `identity`, ohne Assoziativität.
#[macro_export]
macro_rules! impl_unital_magma {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block) => {
        $crate::__markers!([$($g)*] $t, $op: PartialMagma, UnitalPartialMagma);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
        $crate::__impl_identity!([$($g)*] $t, $op, $id);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block) => {
        $crate::impl_unital_magma!(for [] $t, $op; op($a, $b) $body identity() $id);
    };
}

/// Quasigruppe: Verknüpfung `op` mit Teilbarkeit. `ldiv(a, b)` löst `a ∘ x = b`,
/// `rdiv(a, b)` löst `y ∘ a = b`.
///
/// ```
/// use math_trait::{impl_quasigroup, Magma, Quasigroup};
/// struct Label;
/// struct Z5(u8);
/// // a ∘ b = 2a + 4b (mod 5): eindeutig lösbar, aber nicht assoziativ
/// impl_quasigroup!(Z5, Label;
///     op(a, b) { Z5((2 * a.0 + 4 * b.0) % 5) }
///     ldiv(a, b) { Z5((4 * (b.0 + 5 - (2 * a.0) % 5)) % 5) }
///     rdiv(a, b) { Z5((3 * (b.0 + 5 - (4 * a.0) % 5)) % 5) }
/// );
/// let (a, b) = (Z5(1), Z5(3));
/// let x = <Z5 as Quasigroup<Label>>::ldiv(&a, &b);
/// assert_eq!(<Z5 as Magma<Label>>::op(&a, &x).0, b.0);
/// ```
#[macro_export]
macro_rules! impl_quasigroup {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block
        ldiv($la:ident, $lb:ident) $ldiv:block rdiv($ra:ident, $rb:ident) $rdiv:block) => {
        $crate::__markers!([$($g)*] $t, $op:
            PartialMagma, LeftCancellative, RightCancellative, Cancellative);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
        $crate::__impl_quasigroup_methods!([$($g)*] $t, $op, $la, $lb, $ldiv, $ra, $rb, $rdiv);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block
        ldiv($la:ident, $lb:ident) $ldiv:block rdiv($ra:ident, $rb:ident) $rdiv:block) => {
        $crate::impl_quasigroup!(for [] $t, $op; op($a, $b) $body
            ldiv($la, $lb) $ldiv rdiv($ra, $rb) $rdiv);
    };
}

/// Loop: Quasigruppe mit neutralem Element `identity`, nicht notwendig assoziativ.
#[macro_export]
macro_rules! impl_loop {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block
        ldiv($la:ident, $lb:ident) $ldiv:block rdiv($ra:ident, $rb:ident) $rdiv:block) => {
        $crate::__markers!([$($g)*] $t, $op:
            PartialMagma, LeftCancellative, RightCancellative, Cancellative,
            UnitalPartialMagma, Loop);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
        $crate::__impl_identity!([$($g)*] $t, $op, $id);
        $crate::__impl_quasigroup_methods!([$($g)*] $t, $op, $la, $lb, $ldiv, $ra, $rb, $rdiv);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block
        ldiv($la:ident, $lb:ident) $ldiv:block rdiv($ra:ident, $rb:ident) $rdiv:block) => {
        $crate::impl_loop!(for [] $t, $op; op($a, $b) $body identity() $id
            ldiv($la, $lb) $ldiv rdiv($ra, $rb) $rdiv);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_quasigroup_methods {
    ([$($g:tt)*] $t:ty, $op:ty, $la:ident, $lb:ident, $ldiv:block,
        $ra:ident, $rb:ident, $rdiv:block) => {
        impl<$($g)*> $crate::Quasigroup<$op> for $t {
            fn ldiv(&self, b: &Self) -> Self {
                let $la = self;
                let $lb = b;
                $ldiv
            }
            fn rdiv(&self, b: &Self) -> Self {
                let $ra = self;
                let $rb = b;
                $rdiv
            }
        }
    };
}

/// Halbgruppe: `op` ist assoziativ.
#[macro_export]
macro_rules! impl_semigroup {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::__semigroup_markers!([$($g)*] $t, $op);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::impl_semigroup!(for [] $t, $op; op($a, $b) $body);
    };
}

/// Kommutative Halbgruppe.
#[macro_export]
macro_rules! impl_commutative_semigroup {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::__semigroup_markers!([$($g)*] $t, $op);
        $crate::__commutative_markers!([$($g)*] $t, $op);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::impl_commutative_semigroup!(for [] $t, $op; op($a, $b) $body);
    };
}

/// Monoid: assoziatives `op` mit neutralem Element `identity`.
///
/// ```
/// use math_trait::{impl_monoid, Magma, UnitalMagma, Monoid};
/// struct Concat;
/// #[derive(Debug, PartialEq)]
/// struct Text(String);
/// impl_monoid!(Text, Concat;
///     op(a, b) { Text(format!("{}{}", a.0, b.0)) }
///     identity() { Text(String::new()) }
/// );
/// fn sum<T: Monoid<O>, O>(xs: &[T]) -> T {
///     xs.iter().fold(<T as UnitalMagma<O>>::identity(), |acc, x| <T as Magma<O>>::op(&acc, x))
/// }
/// assert_eq!(sum::<_, Concat>(&[Text("a".into()), Text("b".into())]), Text("ab".into()));
/// ```
#[macro_export]
macro_rules! impl_monoid {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block) => {
        $crate::__semigroup_markers!([$($g)*] $t, $op);
        $crate::__monoid_markers!([$($g)*] $t, $op);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
        $crate::__impl_identity!([$($g)*] $t, $op, $id);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block) => {
        $crate::impl_monoid!(for [] $t, $op; op($a, $b) $body identity() $id);
    };
}

/// Kommutatives Monoid.
#[macro_export]
macro_rules! impl_commutative_monoid {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block) => {
        $crate::__semigroup_markers!([$($g)*] $t, $op);
        $crate::__commutative_markers!([$($g)*] $t, $op);
        $crate::__monoid_markers!([$($g)*] $t, $op);
        $crate::__markers!([$($g)*] $t, $op: CommutativeMonoid);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
        $crate::__impl_identity!([$($g)*] $t, $op, $id);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block) => {
        $crate::impl_commutative_monoid!(for [] $t, $op; op($a, $b) $body identity() $id);
    };
}

/// Gruppe: Monoid mit `inverse`. `ldiv` und `rdiv` werden daraus abgeleitet.
///
/// ```
/// use math_trait::{impl_group, Additive, Group, Magma};
/// struct Z5(u8);
/// impl_group!(Z5, Additive;
///     op(a, b) { Z5((a.0 + b.0) % 5) }
///     identity() { Z5(0) }
///     inverse(a) { Z5((5 - a.0) % 5) }
/// );
/// let x = Z5(2);
/// let y = <Z5 as Group<Additive>>::inverse(&x);
/// assert_eq!(<Z5 as Magma<Additive>>::op(&x, &y).0, 0);
/// ```
#[macro_export]
macro_rules! impl_group {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block
        inverse($x:ident) $inv:block) => {
        $crate::__semigroup_markers!([$($g)*] $t, $op);
        $crate::__monoid_markers!([$($g)*] $t, $op);
        $crate::__group_markers!([$($g)*] $t, $op);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
        $crate::__impl_identity!([$($g)*] $t, $op, $id);
        $crate::__impl_group_methods!([$($g)*] $t, $op, $x, $inv);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block
        inverse($x:ident) $inv:block) => {
        $crate::impl_group!(for [] $t, $op; op($a, $b) $body identity() $id inverse($x) $inv);
    };
}

/// Abelsche Gruppe: kommutative Gruppe.
///
/// Mit Generics, hier das direkte Produkt `T × T`:
///
/// ```
/// use math_trait::{impl_abelian_group, AbelianGroup, Additive, Group, Magma, UnitalMagma};
/// struct Pair<T>(T, T);
/// # struct Z5(u8);
/// # math_trait::impl_abelian_group!(Z5, Additive;
/// #     op(a, b) { Z5((a.0 + b.0) % 5) } identity() { Z5(0) } inverse(a) { Z5((5 - a.0) % 5) });
/// impl_abelian_group!(for [T: AbelianGroup<Additive>] Pair<T>, Additive;
///     op(a, b) { Pair(
///         <T as Magma<Additive>>::op(&a.0, &b.0),
///         <T as Magma<Additive>>::op(&a.1, &b.1)) }
///     identity() { Pair(
///         <T as UnitalMagma<Additive>>::identity(),
///         <T as UnitalMagma<Additive>>::identity()) }
///     inverse(a) { Pair(
///         <T as Group<Additive>>::inverse(&a.0),
///         <T as Group<Additive>>::inverse(&a.1)) }
/// );
/// fn takes<G: AbelianGroup<Additive>>() {}
/// takes::<Pair<Z5>>();
/// ```
#[macro_export]
macro_rules! impl_abelian_group {
    (for [$($g:tt)*] $t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block
        inverse($x:ident) $inv:block) => {
        $crate::__semigroup_markers!([$($g)*] $t, $op);
        $crate::__commutative_markers!([$($g)*] $t, $op);
        $crate::__monoid_markers!([$($g)*] $t, $op);
        $crate::__markers!([$($g)*] $t, $op: CommutativeMonoid);
        $crate::__group_markers!([$($g)*] $t, $op);
        $crate::__markers!([$($g)*] $t, $op: AbelianGroup);
        $crate::__impl_op!([$($g)*] $t, $op, $a, $b, $body);
        $crate::__impl_identity!([$($g)*] $t, $op, $id);
        $crate::__impl_group_methods!([$($g)*] $t, $op, $x, $inv);
    };
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block
        inverse($x:ident) $inv:block) => {
        $crate::impl_abelian_group!(for [] $t, $op; op($a, $b) $body identity() $id
            inverse($x) $inv);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __semiring_markers {
    ([$($g:tt)*] $t:ty, $add:ty, $mul:ty) => {
        impl<$($g)*> $crate::LeftDistributive<$mul, $add> for $t {}
        impl<$($g)*> $crate::RightDistributive<$mul, $add> for $t {}
        impl<$($g)*> $crate::Distributive<$mul, $add> for $t {}
        impl<$($g)*> $crate::Annihilating<$mul, $add> for $t {}
        impl<$($g)*> $crate::Semiring<$add, $mul> for $t {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ring_markers {
    ([$($g:tt)*] $t:ty, $add:ty, $mul:ty) => {
        $crate::__semiring_markers!([$($g)*] $t, $add, $mul);
        impl<$($g)*> $crate::Ring<$add, $mul> for $t {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_recip {
    ([$($g:tt)*] $t:ty, $add:ty, $mul:ty, $r:ident, $body:block) => {
        impl<$($g)*> $crate::DivisionRing<$add, $mul> for $t {
            fn recip(&self) -> Option<Self> {
                let $r = self;
                $body
            }
        }
    };
}

/// Halbring: kommutatives Monoid `add`, Monoid `mul`, Distributivgesetz, `zero` absorbiert.
///
/// ```
/// use math_trait::{impl_commutative_semiring, Additive, Multiplicative};
/// struct Bool(bool);
/// impl_commutative_semiring!(Bool, Additive, Multiplicative;
///     add(a, b) { Bool(a.0 || b.0) }
///     zero() { Bool(false) }
///     mul(a, b) { Bool(a.0 && b.0) }
///     one() { Bool(true) }
/// );
/// ```
#[macro_export]
macro_rules! impl_semiring {
    (for [$($g:tt)*] $t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block) => {
        $crate::impl_commutative_monoid!(for [$($g)*] $t, $add; op($a, $b) $addb identity() $zero);
        $crate::impl_monoid!(for [$($g)*] $t, $mul; op($c, $d) $mulb identity() $one);
        $crate::__semiring_markers!([$($g)*] $t, $add, $mul);
    };
    ($t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block) => {
        $crate::impl_semiring!(for [] $t, $add, $mul;
            add($a, $b) $addb zero() $zero mul($c, $d) $mulb one() $one);
    };
}

/// Kommutativer Halbring: `mul` ist kommutativ.
#[macro_export]
macro_rules! impl_commutative_semiring {
    (for [$($g:tt)*] $t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block) => {
        $crate::impl_commutative_monoid!(for [$($g)*] $t, $add; op($a, $b) $addb identity() $zero);
        $crate::impl_commutative_monoid!(for [$($g)*] $t, $mul; op($c, $d) $mulb identity() $one);
        $crate::__semiring_markers!([$($g)*] $t, $add, $mul);
        impl<$($g)*> $crate::CommutativeSemiring<$add, $mul> for $t {}
    };
    ($t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block) => {
        $crate::impl_commutative_semiring!(for [] $t, $add, $mul;
            add($a, $b) $addb zero() $zero mul($c, $d) $mulb one() $one);
    };
}

/// Ring: abelsche Gruppe `add`, Monoid `mul`, Distributivgesetz.
///
/// ```
/// use math_trait::{impl_ring, Additive, Multiplicative};
/// struct Z4(u8);
/// impl_ring!(Z4, Additive, Multiplicative;
///     add(a, b) { Z4((a.0 + b.0) % 4) }
///     zero() { Z4(0) }
///     neg(a) { Z4((4 - a.0) % 4) }
///     mul(a, b) { Z4((a.0 * b.0) % 4) }
///     one() { Z4(1) }
/// );
/// ```
#[macro_export]
macro_rules! impl_ring {
    (for [$($g:tt)*] $t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block) => {
        $crate::impl_abelian_group!(for [$($g)*] $t, $add;
            op($a, $b) $addb identity() $zero inverse($n) $negb);
        $crate::impl_monoid!(for [$($g)*] $t, $mul; op($c, $d) $mulb identity() $one);
        $crate::__ring_markers!([$($g)*] $t, $add, $mul);
    };
    ($t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block) => {
        $crate::impl_ring!(for [] $t, $add, $mul;
            add($a, $b) $addb zero() $zero neg($n) $negb mul($c, $d) $mulb one() $one);
    };
}

/// Kommutativer Ring: `mul` ist kommutativ.
#[macro_export]
macro_rules! impl_commutative_ring {
    (for [$($g:tt)*] $t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block) => {
        $crate::impl_abelian_group!(for [$($g)*] $t, $add;
            op($a, $b) $addb identity() $zero inverse($n) $negb);
        $crate::impl_commutative_monoid!(for [$($g)*] $t, $mul;
            op($c, $d) $mulb identity() $one);
        $crate::__ring_markers!([$($g)*] $t, $add, $mul);
        impl<$($g)*> $crate::CommutativeSemiring<$add, $mul> for $t {}
        impl<$($g)*> $crate::CommutativeRing<$add, $mul> for $t {}
    };
    ($t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block) => {
        $crate::impl_commutative_ring!(for [] $t, $add, $mul;
            add($a, $b) $addb zero() $zero neg($n) $negb mul($c, $d) $mulb one() $one);
    };
}

/// Schiefkörper: Ring, in dem jedes Element außer `zero` ein Inverses bezüglich `mul` hat.
///
/// `recip` liefert `Some(Kehrwert)`, und `None` genau für `zero`.
#[macro_export]
macro_rules! impl_division_ring {
    (for [$($g:tt)*] $t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block recip($r:ident) $recip:block) => {
        $crate::impl_ring!(for [$($g)*] $t, $add, $mul;
            add($a, $b) $addb zero() $zero neg($n) $negb mul($c, $d) $mulb one() $one);
        $crate::__impl_recip!([$($g)*] $t, $add, $mul, $r, $recip);
    };
    ($t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block recip($r:ident) $recip:block) => {
        $crate::impl_division_ring!(for [] $t, $add, $mul;
            add($a, $b) $addb zero() $zero neg($n) $negb mul($c, $d) $mulb one() $one
            recip($r) $recip);
    };
}

/// Körper: kommutativer Schiefkörper.
///
/// `recip` liefert `Some(Kehrwert)`, und `None` genau für `zero`.
///
/// ```
/// use math_trait::{impl_field, Additive, Multiplicative, Field};
/// struct Z5(u8);
/// impl_field!(Z5, Additive, Multiplicative;
///     add(a, b) { Z5((a.0 + b.0) % 5) }
///     zero() { Z5(0) }
///     neg(a) { Z5((5 - a.0) % 5) }
///     mul(a, b) { Z5((a.0 * b.0) % 5) }
///     one() { Z5(1) }
///     recip(a) { match a.0 { 1 => Some(Z5(1)), 2 => Some(Z5(3)), 3 => Some(Z5(2)), 4 => Some(Z5(4)), _ => None } }
/// );
/// fn takes_field<K: Field>() {}
/// takes_field::<Z5>();
/// ```
#[macro_export]
macro_rules! impl_field {
    (for [$($g:tt)*] $t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block recip($r:ident) $recip:block) => {
        $crate::impl_commutative_ring!(for [$($g)*] $t, $add, $mul;
            add($a, $b) $addb zero() $zero neg($n) $negb mul($c, $d) $mulb one() $one);
        $crate::__impl_recip!([$($g)*] $t, $add, $mul, $r, $recip);
        impl<$($g)*> $crate::Field<$add, $mul> for $t {}
    };
    ($t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block recip($r:ident) $recip:block) => {
        $crate::impl_field!(for [] $t, $add, $mul;
            add($a, $b) $addb zero() $zero neg($n) $negb mul($c, $d) $mulb one() $one
            recip($r) $recip);
    };
}

/// Modul über dem Ring `R` (mit den Standard-Etiketten). `V` muss schon eine abelsche
/// Gruppe bezüglich `Additive` sein, z. B. über [`impl_abelian_group!`].
#[macro_export]
macro_rules! impl_module {
    (for [$($g:tt)*] $v:ty, $r:ty; act($s:ident, $x:ident) $body:block) => {
        impl<$($g)*> $crate::LeftAction<$r> for $v {
            fn act(scalar: &$r, x: &Self) -> Self {
                let $s = scalar;
                let $x = x;
                $body
            }
        }
        impl<$($g)*> $crate::Module<$r> for $v {}
    };
    ($v:ty, $r:ty; act($s:ident, $x:ident) $body:block) => {
        $crate::impl_module!(for [] $v, $r; act($s, $x) $body);
    };
}

/// Algebra über `R` mit dem Produkt-Etikett `Prod`. `A` muss schon ein Modul sein und
/// `Magma<Prod>` implementieren (z. B. über [`impl_magma!`]).
#[macro_export]
macro_rules! impl_algebra {
    (for [$($g:tt)*] $a:ty, $r:ty, $prod:ty) => {
        impl<$($g)*> $crate::LeftDistributive<$prod, $crate::Additive> for $a {}
        impl<$($g)*> $crate::RightDistributive<$prod, $crate::Additive> for $a {}
        impl<$($g)*> $crate::Distributive<$prod, $crate::Additive> for $a {}
        impl<$($g)*> $crate::Bilinear<$a, $a, $r, $prod> for $a {}
        impl<$($g)*>
            $crate::Algebra<
                $r,
                $crate::Additive,
                $crate::Multiplicative,
                $crate::ScalarMultiplication,
                $prod,
            > for $a
        {
        }
    };
    ($a:ty, $r:ty, $prod:ty) => {
        $crate::impl_algebra!(for [] $a, $r, $prod);
    };
}

/// Wie [`impl_algebra!`], aber für einen Typ, der schon über [`impl_ring!`] (oder eine
/// Variante davon) ein Ring ist und dessen Ring-Multiplikation das Produkt der Algebra ist.
/// Das Distributivgesetz gibt es dann schon, deshalb wird es hier nicht noch einmal erzeugt.
///
/// So ist z. B. ein Körper `K` eine Algebra über sich selbst oder `K × K` eine Algebra über `K`.
#[macro_export]
macro_rules! impl_ring_algebra {
    (for [$($g:tt)*] $a:ty, $r:ty) => {
        impl<$($g)*> $crate::Bilinear<$a, $a, $r, $crate::Multiplicative> for $a {}
        impl<$($g)*>
            $crate::Algebra<
                $r,
                $crate::Additive,
                $crate::Multiplicative,
                $crate::ScalarMultiplication,
                $crate::Multiplicative,
            > for $a
        {
        }
    };
    ($a:ty, $r:ty) => {
        $crate::impl_ring_algebra!(for [] $a, $r);
    };
}

/// Zusatz zu [`impl_algebra!`]: die Algebra hat ein Einselement bezüglich `Prod`
/// (`UnitalMagma<Prod>` muss schon implementiert sein).
#[macro_export]
macro_rules! impl_unital_algebra {
    (for [$($g:tt)*] $a:ty, $r:ty, $prod:ty) => {
        impl<$($g)*> $crate::UnitalAlgebra<
            $r, $crate::Additive, $crate::Multiplicative, $crate::ScalarMultiplication, $prod
        > for $a {}
    };
    ($a:ty, $r:ty, $prod:ty) => {
        $crate::impl_unital_algebra!(for [] $a, $r, $prod);
    };
}

/// Zusatz zu [`impl_algebra!`]: das Produkt ist assoziativ (`Semigroup<Prod>` muss schon
/// implementiert sein).
#[macro_export]
macro_rules! impl_associative_algebra {
    (for [$($g:tt)*] $a:ty, $r:ty, $prod:ty) => {
        impl<$($g)*> $crate::AssociativeAlgebra<
            $r, $crate::Additive, $crate::Multiplicative, $crate::ScalarMultiplication, $prod
        > for $a {}
    };
    ($a:ty, $r:ty, $prod:ty) => {
        $crate::impl_associative_algebra!(for [] $a, $r, $prod);
    };
}

/// Zusatz zu [`impl_algebra!`]: das Produkt ist alternativ (`Alternative<Prod>` muss schon
/// implementiert sein).
#[macro_export]
macro_rules! impl_alternative_algebra {
    (for [$($g:tt)*] $a:ty, $r:ty, $prod:ty) => {
        impl<$($g)*> $crate::AlternativeAlgebra<
            $r, $crate::Additive, $crate::Multiplicative, $crate::ScalarMultiplication, $prod
        > for $a {}
    };
    ($a:ty, $r:ty, $prod:ty) => {
        $crate::impl_alternative_algebra!(for [] $a, $r, $prod);
    };
}

/// Zusatz zu [`impl_unital_algebra!`]: Divisionsalgebra mit `recip`, `None` genau für den
/// Nullvektor.
#[macro_export]
macro_rules! impl_division_algebra {
    (for [$($g:tt)*] $a:ty, $r:ty, $prod:ty; recip($x:ident) $body:block) => {
        impl<$($g)*> $crate::DivisionAlgebra<
            $r, $crate::Additive, $crate::Multiplicative, $crate::ScalarMultiplication, $prod
        > for $a {
            fn recip(&self) -> Option<Self> {
                let $x = self;
                $body
            }
        }
    };
    ($a:ty, $r:ty, $prod:ty; recip($x:ident) $body:block) => {
        $crate::impl_division_algebra!(for [] $a, $r, $prod; recip($x) $body);
    };
}

/// Lie-Algebra über `R`: Algebra mit dem Produkt `Bracket`, das alternierend ist und die
/// Jacobi-Identität erfüllt. `L` muss schon ein Modul sein und `Magma<Bracket>` implementieren.
#[macro_export]
macro_rules! impl_lie_algebra {
    (for [$($g:tt)*] $l:ty, $r:ty) => {
        $crate::impl_algebra!(for [$($g)*] $l, $r, $crate::Bracket);
        impl<$($g)*> $crate::Anticommutative<$crate::Bracket> for $l {}
        impl<$($g)*> $crate::Alternating<$crate::Bracket> for $l {}
        impl<$($g)*> $crate::Jacobi<$crate::Bracket> for $l {}
        impl<$($g)*> $crate::LieAlgebra<$r> for $l {}
    };
    ($l:ty, $r:ty) => {
        $crate::impl_lie_algebra!(for [] $l, $r);
    };
}

/// Macht den Körper `K` zur Algebra über sich selbst: assoziativ, kommutativ, mit Einselement,
/// trivialer Involution und der Norm `N(x) = x²`. Das ist der Anfang der Cayley-Dickson-Reihe.
///
/// `K` muss schon über [`impl_field!`] ein Körper sein.
#[macro_export]
macro_rules! impl_field_algebra {
    ($k:ty) => {
        impl $crate::LeftAction<$k> for $k {
            fn act(scalar: &$k, x: &Self) -> Self {
                <$k as $crate::Magma<$crate::Multiplicative>>::op(scalar, x)
            }
        }
        impl $crate::Module<$k> for $k {}
        $crate::impl_ring_algebra!($k, $k);
        impl $crate::UnitalAlgebra<$k> for $k {}
        impl $crate::AssociativeAlgebra<$k> for $k {}
        impl $crate::DivisionAlgebra<$k> for $k {
            fn recip(&self) -> Option<Self> {
                <$k as $crate::DivisionRing>::recip(self)
            }
        }
        impl $crate::Involutive for $k {
            fn conjugate(&self) -> Self {
                <$k as $crate::Magma<$crate::Additive>>::op(
                    self,
                    &<$k as $crate::UnitalMagma<$crate::Additive>>::identity(),
                )
            }
        }
        impl $crate::TrivialInvolution for $k {}
        impl $crate::Automorphism<$crate::Additive> for $k {}
        impl $crate::AntiAutomorphism<$crate::Multiplicative> for $k {}
        impl $crate::AlgebraWithInvolution<$k> for $k {}
        impl $crate::QuadraticForm<$k, $crate::Norm> for $k {
            fn value(&self) -> $k {
                <$k as $crate::Magma<$crate::Multiplicative>>::op(self, self)
            }
        }
        impl $crate::CompositionAlgebra<$k> for $k {}
    };
}

/// *-Ring: ein Ring mit Involution `conjugate`, die die Addition erhält und die Multiplikation
/// umkehrt. `T` muss schon ein Ring sein (z. B. über [`impl_ring!`]).
#[macro_export]
macro_rules! impl_star_ring {
    (for [$($g:tt)*] $t:ty; conjugate($x:ident) $body:block) => {
        impl<$($g)*> $crate::Involutive for $t {
            fn conjugate(&self) -> Self {
                let $x = self;
                $body
            }
        }
        impl<$($g)*> $crate::Automorphism<$crate::Additive> for $t {}
        impl<$($g)*> $crate::AntiAutomorphism<$crate::Multiplicative> for $t {}
        impl<$($g)*> $crate::StarRing for $t {}
    };
    ($t:ty; conjugate($x:ident) $body:block) => {
        $crate::impl_star_ring!(for [] $t; conjugate($x) $body);
    };
}

/// Algebra mit Involution `conjugate` über `R`, die die Addition erhält und `Prod` umkehrt.
/// `T` muss schon eine Algebra sein (z. B. über [`impl_algebra!`]).
#[macro_export]
macro_rules! impl_algebra_with_involution {
    (for [$($g:tt)*] $t:ty, $r:ty, $prod:ty; conjugate($x:ident) $body:block) => {
        impl<$($g)*> $crate::Involutive for $t {
            fn conjugate(&self) -> Self {
                let $x = self;
                $body
            }
        }
        impl<$($g)*> $crate::Automorphism<$crate::Additive> for $t {}
        impl<$($g)*> $crate::AntiAutomorphism<$prod> for $t {}
        impl<$($g)*> $crate::AlgebraWithInvolution<
            $r, $crate::Additive, $crate::Multiplicative, $crate::ScalarMultiplication, $prod
        > for $t {}
    };
    ($t:ty, $r:ty, $prod:ty; conjugate($x:ident) $body:block) => {
        $crate::impl_algebra_with_involution!(for [] $t, $r, $prod; conjugate($x) $body);
    };
}

/// Kompositionsalgebra über dem Körper `K` mit der Norm-Form `norm`. `T` muss schon eine
/// unitale Algebra mit Involution sein (über die Makros `impl_unital_algebra!` und
/// `impl_algebra_with_involution!`), mit dem Produkt `Multiplicative`.
#[macro_export]
macro_rules! impl_composition_algebra {
    (for [$($g:tt)*] $t:ty, $k:ty; norm($x:ident) $body:block) => {
        impl<$($g)*> $crate::QuadraticForm<$k, $crate::Norm> for $t {
            fn value(&self) -> $k {
                let $x = self;
                $body
            }
        }
        impl<$($g)*> $crate::CompositionAlgebra<$k> for $t {}
    };
    ($t:ty, $k:ty; norm($x:ident) $body:block) => {
        $crate::impl_composition_algebra!(for [] $t, $k; norm($x) $body);
    };
}
