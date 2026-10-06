//! Schreibt die Hierarchie als Kompilier-Test fest: Wer die Beziehungen zwischen den
//! Traits versehentlich ändert, bekommt hier einen Fehler.

#![allow(dead_code)]

use math_trait::*;

/// `implies!(name: Sub => Sup1, Sup2, ...)` prüft bei der Compilierung, dass jedes
/// `Sub<Op>` auch `Sup<Op>` ist.
macro_rules! implies {
    ($name:ident: $sub:ident => $($sup:ident),+ $(,)?) => {
        #[allow(dead_code)]
        fn $name<T: $sub<O>, O>() {
            $( { fn need<U: $sup<O2>, O2>() {} need::<T, O>(); } )+
        }
    };
}

// Tabelle "Group-like structures" (Wikipedia)
implies!(magma_is_partial: Magma => PartialMagma);
implies!(quasigroup: Quasigroup => Magma, Cancellative);
implies!(unital_magma: UnitalMagma => Magma, UnitalPartialMagma);
implies!(loop_: Loop => Quasigroup, UnitalMagma);
implies!(semigroup: Semigroup => Magma, Semigroupoid, Alternative, Flexible, PowerAssociative);
implies!(assoc_quasigroup: AssociativeQuasigroup => Semigroup, Quasigroup);
implies!(monoid: Monoid => Semigroup, UnitalMagma, SmallCategory);
implies!(group: Group => Monoid, Loop, AssociativeQuasigroup, Groupoid);
implies!(small_category: SmallCategory => Semigroupoid, UnitalPartialMagma);
implies!(groupoid: Groupoid => SmallCategory);

// Zusatzeigenschaften
implies!(commutative: Commutative => Magma, Flexible);
implies!(commutative_semigroup: CommutativeSemigroup => Semigroup, Commutative, Medial);
implies!(commutative_monoid: CommutativeMonoid => Monoid, CommutativeSemigroup);
implies!(abelian_group: AbelianGroup => Group, CommutativeMonoid, Commutative);
implies!(cancellative: Cancellative => LeftCancellative, RightCancellative);
implies!(entropic: Entropic => Medial, Trimedial);
implies!(semimedial: Semimedial => LeftSemimedial, RightSemimedial);
implies!(self_distributive: SelfDistributive => LeftSelfDistributive, RightSelfDistributive);
implies!(null_semigroup: NullSemigroup => Semigroup, LeftUnar, RightUnar);

// Ringe und Körper (zwei Verknüpfungen)
fn ring<T: Ring<A, M>, A, M>() {
    fn need<U: AbelianGroup<A2> + Monoid<M2> + Distributive<M2, A2>, A2, M2>() {}
    need::<T, A, M>();
}
fn field<T: Field<A, M>, A, M>() {
    fn need<U: DivisionRing<A2, M2> + CommutativeRing<A2, M2> + Ring<A2, M2>, A2, M2>() {}
    need::<T, A, M>();
}
fn defaults<T: Field>() {
    fn need<U: Ring<Additive, Multiplicative>>() {}
    need::<T>();
}

// Vektorräume und Algebren
fn vector_space<V: VectorSpace<K>, K: Field>() {
    fn need<U: Module<K2>, K2: Ring>() {}
    need::<V, K>();
}
fn division_algebra<A: DivisionAlgebra<K>, K: Field>() {
    fn need<U: UnitalAlgebra<K2> + Algebra<K2> + VectorSpace<K2>, K2: Field>() {}
    need::<A, K>();
}
fn composition_algebra<A: CompositionAlgebra<K>, K: Field>() {
    fn need<
        U: UnitalAlgebra<K2> + AlgebraWithInvolution<K2> + QuadraticForm<K2, Norm>,
        K2: Field,
    >() {
    }
    need::<A, K>();
}
fn star_ring<T: StarRing>() {
    fn need<U: Ring + Automorphism<Additive> + AntiAutomorphism<Multiplicative>>() {}
    need::<T>();
}

#[test]
fn hierarchy_compiles() {}
