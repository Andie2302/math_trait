//! Makros, die alle nötigen Trait-Impls für eine Struktur auf einmal erzeugen.
//!
//! Jede algebraische Struktur verlangt viele Marker-Impls (eine Gruppe z. B. 15). Die Makros
//! erzeugen sie und die Methoden aus kurzen Rümpfen. Die *Gesetze* (Assoziativität, Inverse, …)
//! prüft kein Makro: Wer ein Makro verwendet, behauptet, dass sie gelten.
//!
//! Die Makros sind nach Strukturen gestaffelt und schließen sich gegenseitig aus: Für einen Typ
//! und ein Etikett nimmt man genau **ein** Makro der Reihe `magma` → `semigroup` → `monoid` →
//! `group`, in der kommutativen Variante `commutative_…` bzw. `abelian_group`.

/// Implementiert eine Liste von Marker-Traits für einen Typ und ein Etikett.
#[doc(hidden)]
#[macro_export]
macro_rules! __markers {
    ($t:ty, $op:ty: $($tr:ident),* $(,)?) => { $( impl $crate::$tr<$op> for $t {} )* };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_op {
    ($t:ty, $op:ty, $a:ident, $b:ident, $body:block) => {
        impl $crate::Magma<$op> for $t {
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
    ($t:ty, $op:ty, $body:block) => {
        impl $crate::UnitalMagma<$op> for $t {
            fn identity() -> Self $body
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_group_methods {
    ($t:ty, $op:ty, $x:ident, $body:block) => {
        impl $crate::Quasigroup<$op> for $t {
            fn ldiv(&self, b: &Self) -> Self {
                <$t as $crate::Magma<$op>>::op(&<$t as $crate::Group<$op>>::inverse(self), b)
            }
            fn rdiv(&self, b: &Self) -> Self {
                <$t as $crate::Magma<$op>>::op(b, &<$t as $crate::Group<$op>>::inverse(self))
            }
        }
        impl $crate::Group<$op> for $t {
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
    ($t:ty, $op:ty) => {
        $crate::__markers!($t, $op: PartialMagma, Semigroupoid, Alternative, Flexible,
            PowerAssociative, Semigroup);
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __commutative_markers {
    ($t:ty, $op:ty) => {
        $crate::__markers!($t, $op: Commutative, Trimedial, Medial, CommutativeSemigroup);
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __monoid_markers {
    ($t:ty, $op:ty) => {
        $crate::__markers!($t, $op: UnitalPartialMagma, SmallCategory, Monoid);
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __group_markers {
    ($t:ty, $op:ty) => {
        $crate::__markers!($t, $op: LeftCancellative, RightCancellative, Cancellative,
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
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::__markers!($t, $op: PartialMagma);
        $crate::__impl_op!($t, $op, $a, $b, $body);
    };
}

/// Kommutatives Magma: `op` ist kommutativ.
#[macro_export]
macro_rules! impl_commutative_magma {
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::__markers!($t, $op: PartialMagma, Commutative, Flexible);
        $crate::__impl_op!($t, $op, $a, $b, $body);
    };
}

/// Halbgruppe: `op` ist assoziativ.
#[macro_export]
macro_rules! impl_semigroup {
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::__semigroup_markers!($t, $op);
        $crate::__impl_op!($t, $op, $a, $b, $body);
    };
}

/// Kommutative Halbgruppe.
#[macro_export]
macro_rules! impl_commutative_semigroup {
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block) => {
        $crate::__semigroup_markers!($t, $op);
        $crate::__commutative_markers!($t, $op);
        $crate::__impl_op!($t, $op, $a, $b, $body);
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
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block) => {
        $crate::__semigroup_markers!($t, $op);
        $crate::__monoid_markers!($t, $op);
        $crate::__impl_op!($t, $op, $a, $b, $body);
        $crate::__impl_identity!($t, $op, $id);
    };
}

/// Kommutatives Monoid.
#[macro_export]
macro_rules! impl_commutative_monoid {
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block) => {
        $crate::__semigroup_markers!($t, $op);
        $crate::__commutative_markers!($t, $op);
        $crate::__monoid_markers!($t, $op);
        $crate::__markers!($t, $op: CommutativeMonoid);
        $crate::__impl_op!($t, $op, $a, $b, $body);
        $crate::__impl_identity!($t, $op, $id);
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
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block
        inverse($x:ident) $inv:block) => {
        $crate::__semigroup_markers!($t, $op);
        $crate::__monoid_markers!($t, $op);
        $crate::__group_markers!($t, $op);
        $crate::__impl_op!($t, $op, $a, $b, $body);
        $crate::__impl_identity!($t, $op, $id);
        $crate::__impl_group_methods!($t, $op, $x, $inv);
    };
}

/// Abelsche Gruppe: kommutative Gruppe.
#[macro_export]
macro_rules! impl_abelian_group {
    ($t:ty, $op:ty; op($a:ident, $b:ident) $body:block identity() $id:block
        inverse($x:ident) $inv:block) => {
        $crate::__semigroup_markers!($t, $op);
        $crate::__commutative_markers!($t, $op);
        $crate::__monoid_markers!($t, $op);
        $crate::__markers!($t, $op: CommutativeMonoid);
        $crate::__group_markers!($t, $op);
        $crate::__markers!($t, $op: AbelianGroup);
        $crate::__impl_op!($t, $op, $a, $b, $body);
        $crate::__impl_identity!($t, $op, $id);
        $crate::__impl_group_methods!($t, $op, $x, $inv);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ring_markers {
    ($t:ty, $add:ty, $mul:ty) => {
        impl $crate::LeftDistributive<$mul, $add> for $t {}
        impl $crate::RightDistributive<$mul, $add> for $t {}
        impl $crate::Distributive<$mul, $add> for $t {}
        impl $crate::Ring<$add, $mul> for $t {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_recip {
    ($t:ty, $add:ty, $mul:ty, $r:ident, $body:block) => {
        impl $crate::DivisionRing<$add, $mul> for $t {
            fn recip(&self) -> Option<Self> {
                let $r = self;
                $body
            }
        }
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
    ($t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block) => {
        $crate::impl_abelian_group!($t, $add; op($a, $b) $addb identity() $zero inverse($n) $negb);
        $crate::impl_monoid!($t, $mul; op($c, $d) $mulb identity() $one);
        $crate::__ring_markers!($t, $add, $mul);
    };
}

/// Kommutativer Ring: `mul` ist kommutativ.
#[macro_export]
macro_rules! impl_commutative_ring {
    ($t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block) => {
        $crate::impl_abelian_group!($t, $add; op($a, $b) $addb identity() $zero inverse($n) $negb);
        $crate::impl_commutative_monoid!($t, $mul; op($c, $d) $mulb identity() $one);
        $crate::__ring_markers!($t, $add, $mul);
        impl $crate::CommutativeRing<$add, $mul> for $t {}
    };
}

/// Schiefkörper: Ring, in dem jedes Element außer `zero` ein Inverses bezüglich `mul` hat.
///
/// `recip` liefert `Some(Kehrwert)`, und `None` genau für `zero`.
#[macro_export]
macro_rules! impl_division_ring {
    ($t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block recip($r:ident) $recip:block) => {
        $crate::impl_ring!($t, $add, $mul;
            add($a, $b) $addb zero() $zero neg($n) $negb mul($c, $d) $mulb one() $one);
        $crate::__impl_recip!($t, $add, $mul, $r, $recip);
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
    ($t:ty, $add:ty, $mul:ty;
        add($a:ident, $b:ident) $addb:block zero() $zero:block neg($n:ident) $negb:block
        mul($c:ident, $d:ident) $mulb:block one() $one:block recip($r:ident) $recip:block) => {
        $crate::impl_commutative_ring!($t, $add, $mul;
            add($a, $b) $addb zero() $zero neg($n) $negb mul($c, $d) $mulb one() $one);
        $crate::__impl_recip!($t, $add, $mul, $r, $recip);
        impl $crate::Field<$add, $mul> for $t {}
    };
}

/// Modul über dem Ring `R` (mit den Standard-Etiketten). `V` muss schon eine abelsche
/// Gruppe bezüglich `Additive` sein, z. B. über [`impl_abelian_group!`].
#[macro_export]
macro_rules! impl_module {
    ($v:ty, $r:ty; act($s:ident, $x:ident) $body:block) => {
        impl $crate::LeftAction<$r> for $v {
            fn act(scalar: &$r, x: &Self) -> Self {
                let $s = scalar;
                let $x = x;
                $body
            }
        }
        impl $crate::Module<$r> for $v {}
    };
}

/// Algebra über `R` mit dem Produkt-Etikett `Prod`. `A` muss schon ein Modul sein und
/// `Magma<Prod>` implementieren (z. B. über [`impl_magma!`]).
#[macro_export]
macro_rules! impl_algebra {
    ($a:ty, $r:ty, $prod:ty) => {
        impl $crate::LeftDistributive<$prod, $crate::Additive> for $a {}
        impl $crate::RightDistributive<$prod, $crate::Additive> for $a {}
        impl $crate::Distributive<$prod, $crate::Additive> for $a {}
        impl $crate::Bilinear<$a, $a, $r, $prod> for $a {}
        impl
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
}

/// Lie-Algebra über `R`: Algebra mit dem Produkt `Bracket`, das alternierend ist und die
/// Jacobi-Identität erfüllt. `L` muss schon ein Modul sein und `Magma<Bracket>` implementieren.
#[macro_export]
macro_rules! impl_lie_algebra {
    ($l:ty, $r:ty) => {
        $crate::impl_algebra!($l, $r, $crate::Bracket);
        impl $crate::Anticommutative<$crate::Bracket> for $l {}
        impl $crate::Alternating<$crate::Bracket> for $l {}
        impl $crate::Jacobi<$crate::Bracket> for $l {}
        impl $crate::LieAlgebra<$r> for $l {}
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
        impl $crate::Bilinear<$k, $k, $k, $crate::Multiplicative> for $k {}
        impl
            $crate::Algebra<
                $k,
                $crate::Additive,
                $crate::Multiplicative,
                $crate::ScalarMultiplication,
                $crate::Multiplicative,
            > for $k
        {
        }
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
