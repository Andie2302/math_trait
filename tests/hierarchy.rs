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
implies!(band: Band => Semigroup, Idempotent);
implies!(semilattice: Semilattice => Band, CommutativeSemigroup);
implies!(commutative: Commutative => Magma, Flexible);
implies!(commutative_semigroup: CommutativeSemigroup => Semigroup, Commutative, Medial);
implies!(commutative_monoid: CommutativeMonoid => Monoid, CommutativeSemigroup);
implies!(abelian_group: AbelianGroup => Group, CommutativeMonoid, Commutative);
implies!(cancellative: Cancellative => LeftCancellative, RightCancellative);
implies!(entropic: Entropic => Medial, Trimedial);
implies!(semimedial: Semimedial => LeftSemimedial, RightSemimedial);
implies!(self_distributive: SelfDistributive => LeftSelfDistributive, RightSelfDistributive);
implies!(null_semigroup: NullSemigroup => Semigroup, LeftUnar, RightUnar, Zeropotent, Unipotent);
implies!(zeropotent: Zeropotent => Unipotent, Magma);
implies!(left_zero: LeftZeroSemigroup => Band, Semigroup, Idempotent, LeftUnar, RightCancellative);
implies!(right_zero: RightZeroSemigroup => Band, Semigroup, Idempotent, RightUnar, LeftCancellative);
implies!(idempotent: Idempotent => Magma, PowerAssociative);
implies!(trimedial: Trimedial => Magma, Semimedial, LeftSemimedial, RightSemimedial);
implies!(medial: Medial => Trimedial, Semimedial);

// Marker, die nur ein Magma voraussetzen
implies!(unital_partial_magma: UnitalPartialMagma => PartialMagma);
implies!(semigroupoid: Semigroupoid => PartialMagma);
implies!(left_cancellative: LeftCancellative => Magma);
implies!(right_cancellative: RightCancellative => Magma);
implies!(alternative: Alternative => Magma);
implies!(flexible: Flexible => Magma);
implies!(power_associative: PowerAssociative => Magma);
implies!(left_semimedial: LeftSemimedial => Magma);
implies!(right_semimedial: RightSemimedial => Magma);
implies!(left_self_distributive: LeftSelfDistributive => Magma);
implies!(right_self_distributive: RightSelfDistributive => Magma);
implies!(unipotent: Unipotent => Magma);
implies!(left_unar: LeftUnar => Magma);
implies!(right_unar: RightUnar => Magma);
implies!(central: Central => Magma);

// Halbringe, Ringe und Körper (zwei Verknüpfungen)
fn semiring<T: Semiring<A, M>, A, M>() {
    fn need<
        U: CommutativeMonoid<A2> + Monoid<M2> + Distributive<M2, A2> + Annihilating<M2, A2>,
        A2,
        M2,
    >() {
    }
    need::<T, A, M>();
}
fn left_distributive<T: LeftDistributive<M, A>, M, A>() {
    fn need<U: Magma<M2> + Magma<A2>, M2, A2>() {}
    need::<T, M, A>();
}
fn right_distributive<T: RightDistributive<M, A>, M, A>() {
    fn need<U: Magma<M2> + Magma<A2>, M2, A2>() {}
    need::<T, M, A>();
}
fn distributive<T: Distributive<M, A>, M, A>() {
    fn need<U: LeftDistributive<M2, A2> + RightDistributive<M2, A2>, M2, A2>() {}
    need::<T, M, A>();
}
fn annihilating<T: Annihilating<M, A>, M, A>() {
    fn need<U: Magma<M2> + UnitalMagma<A2>, M2, A2>() {}
    need::<T, M, A>();
}
fn commutative_semiring<T: CommutativeSemiring<A, M>, A, M>() {
    fn need<U: Semiring<A2, M2> + Commutative<M2>, A2, M2>() {}
    need::<T, A, M>();
}
fn rng<T: Rng<A, M>, A, M>() {
    fn need<U: AbelianGroup<A2> + Semigroup<M2> + Distributive<M2, A2>, A2, M2>() {}
    need::<T, A, M>();
}
fn ring_is_rng<T: Ring<A, M>, A, M>() {
    fn need<U: Rng<A2, M2>, A2, M2>() {}
    need::<T, A, M>();
}
fn ring_is_semiring<T: Ring<A, M>, A, M>() {
    fn need<U: Semiring<A2, M2>, A2, M2>() {}
    need::<T, A, M>();
}
fn commutative_ring_is_commutative_semiring<T: CommutativeRing<A, M>, A, M>() {
    fn need<U: CommutativeSemiring<A2, M2>, A2, M2>() {}
    need::<T, A, M>();
}
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
fn algebra<T: Algebra<R>, R: CommutativeRing>() {
    fn need<U: Module<R2> + Distributive<Multiplicative, Additive>, R2: CommutativeRing>() {}
    need::<T, R>();
}
fn unital_algebra<T: UnitalAlgebra<R>, R: CommutativeRing>() {
    fn need<U: Algebra<R2> + UnitalMagma<Multiplicative>, R2: CommutativeRing>() {}
    need::<T, R>();
}
fn associative_algebra<T: AssociativeAlgebra<R>, R: CommutativeRing>() {
    fn need<U: Algebra<R2> + Semigroup<Multiplicative>, R2: CommutativeRing>() {}
    need::<T, R>();
}
fn alternative_algebra<T: AlternativeAlgebra<R>, R: CommutativeRing>() {
    fn need<U: Algebra<R2> + Alternative<Multiplicative>, R2: CommutativeRing>() {}
    need::<T, R>();
}
fn algebra_with_involution<T: AlgebraWithInvolution<R>, R: CommutativeRing>() {
    fn need<
        U: Algebra<R2> + Automorphism<Additive> + AntiAutomorphism<Multiplicative>,
        R2: CommutativeRing,
    >() {
    }
    need::<T, R>();
}
fn trivial_involution<T: TrivialInvolution>() {
    fn need<U: Involutive>() {}
    need::<T>();
}
fn automorphism<T: Automorphism<Op>, Op>() {
    fn need<U: Magma<Op2> + Involutive, Op2>() {}
    need::<T, Op>();
}
fn anti_automorphism<T: AntiAutomorphism<Op>, Op>() {
    fn need<U: Magma<Op2> + Involutive, Op2>() {}
    need::<T, Op>();
}
fn bilinear_form<T: BilinearForm<K, M>, K: Field, M>() {
    fn need<U: VectorSpace<K2>, K2: Field>() {}
    need::<T, K>();
}
fn quadratic_form<T: QuadraticForm<K, Q>, K: Field, Q>() {
    fn need<U: VectorSpace<K2>, K2: Field>() {}
    need::<T, K>();
}
fn jacobi<T: Jacobi<Op, A>, Op, A>() {
    fn need<U: Magma<Op2> + UnitalMagma<A2>, Op2, A2>() {}
    need::<T, Op, A>();
}
fn star_ring<T: StarRing>() {
    fn need<U: Ring + Automorphism<Additive> + AntiAutomorphism<Multiplicative>>() {}
    need::<T>();
}

#[test]
fn hierarchy_compiles() {}

// Lie-Algebren
fn alternating<T: Alternating<Op, A>, Op, A>() {
    fn need<U: Anticommutative<Op2, A2> + Distributive<Op2, A2>, Op2, A2>() {}
    need::<T, Op, A>();
}
fn anticommutative<T: Anticommutative<Op, A>, Op, A>() {
    fn need<U: Magma<Op2> + Group<A2>, Op2, A2>() {}
    need::<T, Op, A>();
}
fn lie_algebra<L: LieAlgebra<R>, R: CommutativeRing>() {
    fn need<
        U: Algebra<R2, Additive, Multiplicative, ScalarMultiplication, Bracket>
            + Alternating<Bracket>
            + Jacobi<Bracket>
            + Anticommutative<Bracket>
            + Module<R2>,
        R2: CommutativeRing,
    >() {
    }
    need::<L, R>();
}

// Kommutator-Konstruktion: jede assoziative Algebra liefert eine Lie-Algebra
fn commutator<A: AssociativeAlgebra<R>, R: CommutativeRing>() {
    fn need<L: LieAlgebra<R2>, R2: CommutativeRing>() {}
    need::<Commutator<A, R>, R>();
}

// Tensorprodukt, Clifford-Algebra, Darstellungen
fn bilinear_map<S: BilinearMap<B, C, R, M>, B: Module<R>, C: Module<R>, R: CommutativeRing, M>() {
    fn need<
        U: Bilinear<B2, C2, R2, M2>,
        B2: Module<R2>,
        C2: Module<R2>,
        R2: CommutativeRing,
        M2,
    >() {
    }
    need::<S, B, C, R, M>();
}
fn tensor_product<T: TensorProduct<V, W, R>, V: Module<R>, W: Module<R>, R: CommutativeRing>() {
    fn need<U: Module<R2>, R2: CommutativeRing>() {}
    need::<T, R>();
}
fn clifford<A: CliffordAlgebra<V, K, Q>, V: VectorSpace<K> + QuadraticForm<K, Q>, K: Field, Q>() {
    fn need<U: UnitalAlgebra<K2>, K2: Field>() {}
    need::<A, K>();
}
fn representation<V: LieModule<L, R>, L: LieAlgebra<R>, R: CommutativeRing>() {
    fn need<U: Module<R2>, R2: CommutativeRing>() {}
    need::<V, R>();
}
fn adjoint<L: LieAlgebra<R>, R: CommutativeRing>() {
    fn need<U: LieAlgebra<R2> + LieModule<U, R2>, R2: CommutativeRing>() {}
    need::<L, R>();
}

// Graduierte Algebren
fn graded<A: GradedAlgebra<R>, R: CommutativeRing>() {
    fn need<U: UnitalAlgebra<R2>, R2: CommutativeRing>() {}
    need::<A, R>();
}

// so(N) aus Bivektoren
fn bivector_is_lie<const D: usize, Q: DiagonalForm<R>, R: CommutativeRing>() {
    fn need<L: LieAlgebra<R2>, R2: CommutativeRing>() {}
    need::<Bivector<R, D, Q>, R>();
}
