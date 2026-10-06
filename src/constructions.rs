//! Konkrete Konstruktionen mit fester Dimension: freie Moduln, das Tensorprodukt `V ⊗ W` und die
//! freie Clifford-Algebra.
//!
//! Alle drei sind Koeffizienten-Felder über einem Ring `R` mit Const-Generics. Sie brauchen keine
//! Allokation und keinen Basisdatentyp: `R` kann alles sein, was die Ring-Traits erfüllt.

use core::array::from_fn;
use core::fmt;
use core::marker::PhantomData;

use crate::{
    Additive, AntiAutomorphism, Automorphism, CliffordAlgebra, CommutativeRing, Field,
    GradeInvolution, GradedAlgebra, Group, Involutive, LeftAction, Magma, Module, Multiplicative,
    QuadraticForm, Reversion, Ring, TensorProduct, UnitalAlgebra, UnitalMagma, impl_abelian_group,
    impl_algebra, impl_algebra_with_involution, impl_associative_algebra, impl_group, impl_module,
    impl_monoid, impl_unital_algebra,
};

// --- Hilfsfunktionen in `R` ------------------------------------------------------------------

fn add<R: Magma<Additive>>(a: &R, b: &R) -> R {
    <R as Magma<Additive>>::op(a, b)
}
fn neg<R: Group<Additive>>(a: &R) -> R {
    <R as Group<Additive>>::inverse(a)
}
fn mul<R: Magma<Multiplicative>>(a: &R, b: &R) -> R {
    <R as Magma<Multiplicative>>::op(a, b)
}
fn zero<R: UnitalMagma<Additive>>() -> R {
    <R as UnitalMagma<Additive>>::identity()
}
fn one<R: UnitalMagma<Multiplicative>>() -> R {
    <R as UnitalMagma<Multiplicative>>::identity()
}

// =================================================================================================
// Freier Modul `R^N`
// =================================================================================================

/// Der freie Modul `R^N`: Vektoren mit `N` Koordinaten aus `R`. Über einem Körper ist das der
/// `N`-dimensionale Vektorraum.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector<R, const N: usize>([R; N]);

impl<R, const N: usize> Vector<R, N> {
    /// Der Vektor mit den Koordinaten `coords`.
    pub fn new(coords: [R; N]) -> Self {
        Vector(coords)
    }

    /// Die Koordinaten.
    pub fn coords(&self) -> &[R; N] {
        &self.0
    }

    /// Gibt die Koordinaten zurück.
    pub fn into_coords(self) -> [R; N] {
        self.0
    }
}

impl<R: Ring, const N: usize> Vector<R, N> {
    /// Der `i`-te Basisvektor `eᵢ`: Koordinate `i` ist eins, alle anderen null.
    pub fn basis(i: usize) -> Self {
        Vector(from_fn(|j| if j == i { one() } else { zero() }))
    }
}

impl_abelian_group!(for [R: Ring, const N: usize] Vector<R, N>, Additive;
    op(a, b) { Vector(from_fn(|i| add(&a.0[i], &b.0[i]))) }
    identity() { Vector(from_fn(|_| zero())) }
    inverse(a) { Vector(from_fn(|i| neg(&a.0[i]))) }
);

impl_module!(for [R: CommutativeRing, const N: usize] Vector<R, N>, R;
    act(s, x) { Vector(from_fn(|i| mul(s, &x.0[i]))) }
);

// =================================================================================================
// Tensorprodukt `R^M ⊗ R^N`
// =================================================================================================

/// Das Tensorprodukt `Vector<R, M> ⊗ Vector<R, N>`: eine `M×N`-Koeffizientenmatrix.
///
/// Der reine Tensor `v ⊗ w` hat die Koeffizienten `vᵢ ⋅ wⱼ` ([`TensorProduct::tensor`]). Die Summe
/// reiner Tensoren ist im Allgemeinen nicht mehr rein: das sind die *verschränkten* Zustände.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tensor<R, const M: usize, const N: usize>([[R; N]; M]);

impl<R, const M: usize, const N: usize> Tensor<R, M, N> {
    /// Der Tensor mit den Koeffizienten `rows[i][j]` zur Basis `eᵢ ⊗ eⱼ`.
    pub fn new(rows: [[R; N]; M]) -> Self {
        Tensor(rows)
    }

    /// Die Koeffizienten.
    pub fn coefficients(&self) -> &[[R; N]; M] {
        &self.0
    }
}

impl<R: CommutativeRing, const M: usize, const N: usize> Tensor<R, M, N> {
    /// Die universelle Eigenschaft: Jede bilineare Abbildung `f: V × W → U` setzt sich zu einer
    /// linearen Abbildung `Tensor → U` fort, nämlich `Σᵢⱼ tᵢⱼ ⋅ f(eᵢ, eⱼ)`. Es gilt
    /// `lift(v ⊗ w, f) = f(v, w)`.
    pub fn lift<U: Module<R>>(&self, f: impl Fn(&Vector<R, M>, &Vector<R, N>) -> U) -> U {
        let mut acc = <U as UnitalMagma<Additive>>::identity();
        for i in 0..M {
            for j in 0..N {
                let term = <U as LeftAction<R>>::act(
                    &self.0[i][j],
                    &f(&Vector::basis(i), &Vector::basis(j)),
                );
                acc = <U as Magma<Additive>>::op(&acc, &term);
            }
        }
        acc
    }
}

impl<R: Field + PartialEq, const M: usize, const N: usize> Tensor<R, M, N> {
    /// Ist der Tensor *rein* (von der Form `v ⊗ w`)? Das gilt genau dann, wenn alle
    /// `2×2`-Minoren verschwinden, d. h. der Rang höchstens eins ist. Nicht reine Tensoren heißen
    /// in der Quantenmechanik *verschränkt*.
    pub fn is_pure(&self) -> bool {
        let zero: R = zero();
        for i in 0..M {
            for k in (i + 1)..M {
                for j in 0..N {
                    for l in (j + 1)..N {
                        let minor = add(
                            &mul(&self.0[i][j], &self.0[k][l]),
                            &neg(&mul(&self.0[i][l], &self.0[k][j])),
                        );
                        if minor != zero {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

impl_abelian_group!(for [R: Ring, const M: usize, const N: usize] Tensor<R, M, N>, Additive;
    op(a, b) { Tensor(from_fn(|i| from_fn(|j| add(&a.0[i][j], &b.0[i][j])))) }
    identity() { Tensor(from_fn(|_| from_fn(|_| zero()))) }
    inverse(a) { Tensor(from_fn(|i| from_fn(|j| neg(&a.0[i][j])))) }
);

impl_module!(for [R: CommutativeRing, const M: usize, const N: usize] Tensor<R, M, N>, R;
    act(s, x) { Tensor(from_fn(|i| from_fn(|j| mul(s, &x.0[i][j])))) }
);

impl<R: CommutativeRing, const M: usize, const N: usize>
    TensorProduct<Vector<R, M>, Vector<R, N>, R> for Tensor<R, M, N>
{
    fn tensor(v: &Vector<R, M>, w: &Vector<R, N>) -> Self {
        Tensor(from_fn(|i| from_fn(|j| mul(&v.0[i], &w.0[j]))))
    }
}

// =================================================================================================
// Freie Clifford-Algebra
// =================================================================================================

/// Eine Diagonalform `Q(Σ xᵢ eᵢ) = Σ qᵢ xᵢ²`, gegeben durch die Quadrate `qᵢ = eᵢ²` der Erzeuger.
///
/// Über einem Körper mit Charakteristik ungleich zwei lässt sich jede quadratische Form auf diese
/// Gestalt bringen. Das implementierende Etikett legt die `qᵢ` fest.
pub trait DiagonalForm<R> {
    /// Das Quadrat `qᵢ` des `i`-ten Erzeugers.
    fn square(i: usize) -> R;
}

/// Die freie Clifford-Algebra `Cl(R^N, Q)` zur Diagonalform `Q`, mit `D = 2^N` Basiselementen.
///
/// Die Basis sind die geordneten Produkte `e_S = e_{i₁} ⋅ e_{i₂} ⋯` der Erzeuger zu den
/// Teilmengen `S ⊆ {0, …, N−1}` (als Bitmaske `S` indiziert, `e_∅ = 1`). Es gilt
///
/// ```text
/// eᵢ ⋅ eᵢ = qᵢ     und     eᵢ ⋅ eⱼ = − eⱼ ⋅ eᵢ    (i ≠ j)
/// ```
///
/// und sonst nichts, daher *frei*. Das Produkt ist assoziativ. Beispiele über `ℝ`: mit `N = 1`,
/// `q = −1` die komplexen Zahlen, mit `N = 2`, `q = (−1, −1)` die Quaternionen, mit `q = (1, 1)`
/// die 2×2-Matrizen.
///
/// `D` muss eine Potenz von zwei sein, sonst gibt es beim ersten Gebrauch einen
/// Compile-Fehler.
pub struct Clifford<R, const D: usize, Q> {
    c: [R; D],
    _form: PhantomData<fn() -> Q>,
}

impl<R: Clone, const D: usize, Q> Clone for Clifford<R, D, Q> {
    fn clone(&self) -> Self {
        Clifford {
            c: self.c.clone(),
            _form: PhantomData,
        }
    }
}
impl<R: Copy, const D: usize, Q> Copy for Clifford<R, D, Q> {}
impl<R: PartialEq, const D: usize, Q> PartialEq for Clifford<R, D, Q> {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c
    }
}
impl<R: fmt::Debug, const D: usize, Q> fmt::Debug for Clifford<R, D, Q> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Clifford").field(&self.c).finish()
    }
}

impl<R, const D: usize, Q> Clifford<R, D, Q> {
    /// Die Anzahl `N` der Erzeuger: `D = 2^N`.
    pub const GENERATORS: usize = {
        assert!(D.is_power_of_two(), "D muss eine Potenz von zwei sein");
        D.trailing_zeros() as usize
    };

    /// Das Element mit den Koeffizienten `coeffs`; `coeffs[S]` gehört zur Basis `e_S`
    /// (`S` als Bitmaske der Erzeuger).
    pub fn new(coeffs: [R; D]) -> Self {
        let _ = Self::GENERATORS;
        Clifford {
            c: coeffs,
            _form: PhantomData,
        }
    }

    /// Die Koeffizienten.
    pub fn coefficients(&self) -> &[R; D] {
        &self.c
    }
}

impl<R: Ring, const D: usize, Q> Clifford<R, D, Q> {
    /// Das Basiselement `e_S` zur Bitmaske `mask`.
    pub fn blade(mask: usize) -> Self {
        Self::new(from_fn(|s| if s == mask { one() } else { zero() }))
    }

    /// Der `i`-te Erzeuger `eᵢ`.
    pub fn generator(i: usize) -> Self {
        Self::blade(1 << i)
    }

    /// Das Skalar `r`, als `r ⋅ e_∅`.
    pub fn scalar(r: R) -> Self {
        let mut r = Some(r);
        Self::new(from_fn(|s| {
            if s == 0 {
                r.take().expect("nur einmal")
            } else {
                zero()
            }
        }))
    }
}

/// Das Vorzeichen (`true` = negativ), das beim Umordnen von `e_S ⋅ e_T` in aufsteigende
/// Reihenfolge entsteht: für jedes `i ∈ T` zählt jedes `j ∈ S` mit `j > i` eine Vertauschung.
fn reorder_is_negative(s: usize, t: usize) -> bool {
    let mut swaps = 0u32;
    let mut rest = t;
    while rest != 0 {
        let i = rest.trailing_zeros();
        swaps += (s >> (i + 1)).count_ones();
        rest &= rest - 1;
    }
    swaps % 2 == 1
}

fn clifford_product<R, const D: usize, Q>(
    a: &Clifford<R, D, Q>,
    b: &Clifford<R, D, Q>,
) -> Clifford<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
    Clifford::new(from_fn(|k| {
        let mut acc: R = zero();
        for s in 0..D {
            let t = s ^ k;
            // gemeinsame Erzeuger ergeben je ihr Quadrat qᵢ
            let mut factor: R = one();
            let mut common = s & t;
            while common != 0 {
                let i = common.trailing_zeros() as usize;
                factor = mul(&factor, &Q::square(i));
                common &= common - 1;
            }
            let mut term = mul(&mul(&a.c[s], &b.c[t]), &factor);
            if reorder_is_negative(s, t) {
                term = neg(&term);
            }
            acc = add(&acc, &term);
        }
        acc
    }))
}

impl_abelian_group!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] Clifford<R, D, Q>, Additive;
    op(a, b) { Clifford::new(from_fn(|i| add(&a.c[i], &b.c[i]))) }
    identity() { Clifford::new(from_fn(|_| zero())) }
    inverse(a) { Clifford::new(from_fn(|i| neg(&a.c[i]))) }
);

impl_module!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] Clifford<R, D, Q>, R;
    act(s, x) { Clifford::new(from_fn(|i| mul(s, &x.c[i]))) }
);

impl_monoid!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] Clifford<R, D, Q>, Multiplicative;
    op(a, b) { clifford_product(a, b) }
    identity() { Clifford::scalar(one()) }
);

impl_algebra!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] Clifford<R, D, Q>, R, Multiplicative
);
impl_unital_algebra!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] Clifford<R, D, Q>, R, Multiplicative
);
impl_associative_algebra!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] Clifford<R, D, Q>, R, Multiplicative
);

impl<R, const D: usize, Q> Clifford<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
    /// Die universelle Eigenschaft: Zu Bildern `gens[i]` der Erzeuger in einer unitalen Algebra
    /// `A` (mit `gens[i]² = qᵢ` und `gens[i] ⋅ gens[j] = −gens[j] ⋅ gens[i]`) gibt es genau einen
    /// Algebra-Homomorphismus `Cl → A`. Diese Funktion berechnet ihn: Sie ersetzt jedes
    /// `e_S` durch das entsprechende Produkt der Bilder.
    ///
    /// Gelten die Relationen nicht, ist das Ergebnis kein Homomorphismus. Das prüft der Compiler
    /// nicht.
    pub fn lift<A, const N: usize>(&self, gens: &[A; N]) -> A
    where
        A: UnitalAlgebra<R>,
    {
        const { assert!(D == 1 << N, "D muss 2^N sein") };
        let mut acc = <A as UnitalMagma<Additive>>::identity();
        for s in 0..D {
            let mut blade = <A as UnitalMagma<Multiplicative>>::identity();
            for (i, g) in gens.iter().enumerate() {
                if (s >> i) & 1 == 1 {
                    blade = <A as Magma<Multiplicative>>::op(&blade, g);
                }
            }
            let term = <A as LeftAction<R>>::act(&self.c[s], &blade);
            acc = <A as Magma<Additive>>::op(&acc, &term);
        }
        acc
    }
}

/// `Q(x) = Σ qᵢ xᵢ²` auf `Vector<R, N>`.
impl<R, const N: usize, Q> QuadraticForm<R, Q> for Vector<R, N>
where
    R: Field,
    Q: DiagonalForm<R>,
{
    fn value(&self) -> R {
        let mut acc: R = zero();
        for i in 0..N {
            let sq = mul(&self.0[i], &self.0[i]);
            acc = add(&acc, &mul(&Q::square(i), &sq));
        }
        acc
    }
}

/// Die Einbettung `V = R^N → Cl`, `x ↦ Σ xᵢ eᵢ`; es gilt `embed(x)² = Q(x) ⋅ 1`.
impl<R, const N: usize, const D: usize, Q> CliffordAlgebra<Vector<R, N>, R, Q> for Clifford<R, D, Q>
where
    R: Field,
    Q: DiagonalForm<R>,
{
    fn embed(v: &Vector<R, N>) -> Self {
        const { assert!(D == 1 << N, "D muss 2^N sein") };
        Clifford::new(from_fn(|s| {
            if s.count_ones() == 1 {
                v.0[s.trailing_zeros() as usize].clone_via_add()
            } else {
                zero()
            }
        }))
    }
}

/// Kopiert ein Ringelement ohne `Clone`-Bedingung: `x + 0`.
trait CloneViaAdd {
    fn clone_via_add(&self) -> Self;
}
impl<R: Magma<Additive> + UnitalMagma<Additive>> CloneViaAdd for R {
    fn clone_via_add(&self) -> Self {
        add(self, &zero())
    }
}

// =================================================================================================
// Graduierung der Clifford-Algebra
// =================================================================================================

/// Kopiert ein Ringelement ohne `Clone`-Bedingung: `x + 0`.
fn copy<R: Magma<Additive> + UnitalMagma<Additive>>(x: &R) -> R {
    add(x, &zero())
}

impl<R, const D: usize, Q> Clifford<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
    /// Der Anteil vom Grad `k`: nur die Basiselemente `e_S` mit `|S| = k` bleiben stehen.
    ///
    /// Grad 0 sind die Skalare, Grad 1 die Vektoren, Grad 2 die *Bivektoren* und so fort.
    pub fn grade_part(&self, k: usize) -> Self {
        Self::new(from_fn(|s| {
            if s.count_ones() as usize == k {
                copy(&self.c[s])
            } else {
                zero()
            }
        }))
    }

    /// Wendet ein Vorzeichen `(-1)^f(k)` auf den Grad-`k`-Anteil an.
    fn signed_by_grade(&self, negate: impl Fn(u32) -> bool) -> Self {
        Self::new(from_fn(|s| {
            if negate(s.count_ones()) {
                neg(&self.c[s])
            } else {
                copy(&self.c[s])
            }
        }))
    }

    /// Die Umkehrung `x̃`: kehrt die Reihenfolge der Faktoren um, `(x ⋅ y)~ = ỹ ⋅ x̃`. Auf dem
    /// Grad-`k`-Anteil ist sie das Vorzeichen `(−1)^{k(k−1)/2}`.
    pub fn reverse(&self) -> Self {
        self.signed_by_grade(|k| (k * k.wrapping_sub(1) / 2) % 2 == 1)
    }

    /// Die Clifford-Konjugation `x̄ = ̂x̃` (Gradinvolution nach Umkehrung): Auf dem Grad-`k`-Anteil
    /// das Vorzeichen `(−1)^{k(k+1)/2}`. Sie verallgemeinert die komplexe und die
    /// Quaternionen-Konjugation.
    pub fn clifford_conjugate(&self) -> Self {
        self.signed_by_grade(|k| (k * (k + 1) / 2) % 2 == 1)
    }

    /// Das „Sandwich“ `s ⋅ v ⋅ s̃`. Für einen [`Rotor`] `s` ist das die Drehung von `v`.
    pub fn sandwich(&self, v: &Self) -> Self {
        clifford_product(&clifford_product(self, v), &self.reverse())
    }
}

impl<R, const D: usize, Q> Clifford<R, D, Q>
where
    R: CommutativeRing + PartialEq,
    Q: DiagonalForm<R>,
{
    /// Ist das Element gerade (nur gerade Grade)?
    pub fn is_even(&self) -> bool {
        self.odd_part() == Self::new(from_fn(|_| zero()))
    }

    /// Ist das Element ungerade (nur ungerade Grade)?
    pub fn is_odd(&self) -> bool {
        self.even_part() == Self::new(from_fn(|_| zero()))
    }
}

impl<R, const D: usize, Q> GradedAlgebra<R> for Clifford<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
    fn even_part(&self) -> Self {
        Self::new(from_fn(|s| {
            if s.count_ones() % 2 == 0 {
                copy(&self.c[s])
            } else {
                zero()
            }
        }))
    }

    fn odd_part(&self) -> Self {
        Self::new(from_fn(|s| {
            if s.count_ones() % 2 == 1 {
                copy(&self.c[s])
            } else {
                zero()
            }
        }))
    }
}

// Die Clifford-Konjugation ist die Standard-Involution der Algebra.
impl_algebra_with_involution!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] Clifford<R, D, Q>, R, Multiplicative;
    conjugate(x) { x.clifford_conjugate() }
);

// Umkehrung: kehrt das Produkt um.
impl<R, const D: usize, Q> Involutive<Reversion> for Clifford<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
    fn conjugate(&self) -> Self {
        self.reverse()
    }
}
impl<R, const D: usize, Q> Automorphism<Additive, Reversion> for Clifford<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
}
impl<R, const D: usize, Q> AntiAutomorphism<Multiplicative, Reversion> for Clifford<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
}

// Gradinvolution: erhält das Produkt.
impl<R, const D: usize, Q> Involutive<GradeInvolution> for Clifford<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
    fn conjugate(&self) -> Self {
        self.grade_involution()
    }
}
impl<R, const D: usize, Q> Automorphism<Additive, GradeInvolution> for Clifford<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
}
impl<R, const D: usize, Q> Automorphism<Multiplicative, GradeInvolution> for Clifford<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
}

// =================================================================================================
// Die gerade Unteralgebra
// =================================================================================================

/// Die gerade Unteralgebra `Cl⁰`: die Elemente, die nur aus Basiselementen geraden Grades
/// bestehen. Sie ist unter Summe und Produkt abgeschlossen und enthält die Eins, also selbst
/// eine assoziative Algebra mit `D/2` Basiselementen.
///
/// Für `N = 3` und `q = (−1, −1, −1)` ist sie isomorph zu den Quaternionen.
pub struct EvenSubalgebra<R, const D: usize, Q>(Clifford<R, D, Q>);

impl<R, const D: usize, Q> EvenSubalgebra<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
    /// Der gerade Anteil von `x`.
    pub fn from_even_part(x: &Clifford<R, D, Q>) -> Self {
        EvenSubalgebra(x.even_part())
    }

    /// Das Element als Element der gesamten Algebra.
    pub fn get(&self) -> &Clifford<R, D, Q> {
        &self.0
    }

    /// Gibt das Element der gesamten Algebra zurück.
    pub fn into_inner(self) -> Clifford<R, D, Q> {
        self.0
    }
}

impl<R, const D: usize, Q> EvenSubalgebra<R, D, Q>
where
    R: CommutativeRing + PartialEq,
    Q: DiagonalForm<R>,
{
    /// `Some`, wenn `x` gerade ist, sonst `None`.
    pub fn new(x: Clifford<R, D, Q>) -> Option<Self> {
        x.is_even().then_some(EvenSubalgebra(x))
    }
}

impl<R: Clone, const D: usize, Q> Clone for EvenSubalgebra<R, D, Q> {
    fn clone(&self) -> Self {
        EvenSubalgebra(self.0.clone())
    }
}
impl<R: Copy, const D: usize, Q> Copy for EvenSubalgebra<R, D, Q> {}
impl<R: PartialEq, const D: usize, Q> PartialEq for EvenSubalgebra<R, D, Q> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<R: fmt::Debug, const D: usize, Q> fmt::Debug for EvenSubalgebra<R, D, Q> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EvenSubalgebra").field(&self.0).finish()
    }
}

impl_abelian_group!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] EvenSubalgebra<R, D, Q>, Additive;
    op(a, b) {
        EvenSubalgebra(<Clifford<R, D, Q> as Magma<Additive>>::op(&a.0, &b.0))
    }
    identity() { EvenSubalgebra(<Clifford<R, D, Q> as UnitalMagma<Additive>>::identity()) }
    inverse(a) { EvenSubalgebra(<Clifford<R, D, Q> as Group<Additive>>::inverse(&a.0)) }
);

impl_module!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] EvenSubalgebra<R, D, Q>, R;
    act(s, x) { EvenSubalgebra(<Clifford<R, D, Q> as LeftAction<R>>::act(s, &x.0)) }
);

impl_monoid!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] EvenSubalgebra<R, D, Q>,
    Multiplicative;
    op(a, b) { EvenSubalgebra(clifford_product(&a.0, &b.0)) }
    identity() { EvenSubalgebra(Clifford::scalar(one())) }
);

impl_algebra!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] EvenSubalgebra<R, D, Q>,
    R, Multiplicative
);
impl_unital_algebra!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] EvenSubalgebra<R, D, Q>,
    R, Multiplicative
);
impl_associative_algebra!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] EvenSubalgebra<R, D, Q>,
    R, Multiplicative
);

// =================================================================================================
// Die Rotorgruppe (Spin-Gruppe)
// =================================================================================================

/// Ein *Rotor*: ein gerades Element `s` mit `s ⋅ s̃ = 1`. Die Rotoren bilden eine Gruppe, die
/// **Spin-Gruppe**; das Inverse von `s` ist die Umkehrung `s̃`.
///
/// Ein Rotor wirkt durch `v ↦ s ⋅ v ⋅ s̃` auf die Vektoren (Elemente vom Grad 1): Das ist eine
/// lineare Abbildung, die die quadratische Form erhält, also eine Drehung. `s` und `−s` ergeben
/// dieselbe Drehung: Die Spin-Gruppe ist eine *zweifache Überlagerung* der Drehgruppe. Im
/// euklidischen `N = 3` sind die Rotoren die Einheitsquaternionen (`SU(2)`), die Drehungen mit
/// Spin ½ in der Quantenmechanik.
pub struct Rotor<R, const D: usize, Q>(Clifford<R, D, Q>);

impl<R, const D: usize, Q> Rotor<R, D, Q>
where
    R: CommutativeRing + PartialEq,
    Q: DiagonalForm<R>,
{
    /// `Some`, wenn `s` gerade ist und `s ⋅ s̃ = 1` gilt, sonst `None`.
    pub fn new(s: Clifford<R, D, Q>) -> Option<Self> {
        let unit = s.is_even() && clifford_product(&s, &s.reverse()) == Clifford::scalar(one());
        unit.then_some(Rotor(s))
    }
}

impl<R, const D: usize, Q> Rotor<R, D, Q>
where
    R: CommutativeRing,
    Q: DiagonalForm<R>,
{
    /// Das zugrunde liegende gerade Element.
    pub fn get(&self) -> &Clifford<R, D, Q> {
        &self.0
    }

    /// Die Drehung `v ↦ s ⋅ v ⋅ s̃` des Vektors `v`.
    pub fn rotate(&self, v: &Clifford<R, D, Q>) -> Clifford<R, D, Q> {
        self.0.sandwich(v)
    }
}

impl<R: Clone, const D: usize, Q> Clone for Rotor<R, D, Q> {
    fn clone(&self) -> Self {
        Rotor(self.0.clone())
    }
}
impl<R: Copy, const D: usize, Q> Copy for Rotor<R, D, Q> {}
impl<R: PartialEq, const D: usize, Q> PartialEq for Rotor<R, D, Q> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<R: fmt::Debug, const D: usize, Q> fmt::Debug for Rotor<R, D, Q> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Rotor").field(&self.0).finish()
    }
}

// Das Produkt zweier Rotoren ist ein Rotor: (st)(st)~ = s t t̃ s̃ = s s̃ = 1.
impl_group!(
    for [R: CommutativeRing, const D: usize, Q: DiagonalForm<R>] Rotor<R, D, Q>, Multiplicative;
    op(a, b) { Rotor(clifford_product(&a.0, &b.0)) }
    identity() { Rotor(Clifford::scalar(one())) }
    inverse(a) { Rotor(a.0.reverse()) }
);
