//! Tensorprodukt: `lift` mit unsymmetrischer Form und Zielmodul ≠ `R`, `is_pure` für größere
//! Formate.

use math_trait::*;

mod common;
use common::*;

type V2 = Vector<Z5, 2>;
type V3 = Vector<Z5, 3>;

fn vec3(i: usize) -> V3 {
    Vector::new([z(i), z(i / 5), z(i / 25)])
}

#[test]
fn lift_follows_an_asymmetric_bilinear_form() {
    // f(v, w) = v₀ ⋅ w₁ ist nicht symmetrisch: lift(v ⊗ w) muss f(v, w) ergeben, nicht f(w, v)
    let f = |v: &V3, w: &V2| <Z5 as Magma<Multiplicative>>::op(&v.coords()[0], &w.coords()[1]);
    for i in 0..125 {
        for j in (0..25).step_by(3) {
            let (v, w) = (vec3(i), Vector::new([z(j), z(j / 5)]));
            let t = <Tensor<Z5, 3, 2> as TensorProduct<V3, V2, Z5>>::tensor(&v, &w);
            assert_eq!(t.lift(f), f(&v, &w));
        }
    }
    // und die Koeffizienten werden mit f(eᵢ, eⱼ) gewichtet: nur e₀ ⊗ e₁ zählt
    let t = Tensor::new([[Z5(1), Z5(2)], [Z5(3), Z5(4)], [Z5(1), Z5(1)]]);
    assert_eq!(t.lift(f), Z5(2));
}

#[test]
fn lift_into_another_module_is_linear_there() {
    // f(v, w) = (v₀ w₀, v₁ w₁) in V2: das Ziel ist kein Skalar
    let f = |v: &V2, w: &V2| {
        Vector::new([
            <Z5 as Magma<Multiplicative>>::op(&v.coords()[0], &w.coords()[0]),
            <Z5 as Magma<Multiplicative>>::op(&v.coords()[1], &w.coords()[1]),
        ])
    };
    let t = Tensor::new([[Z5(2), Z5(3)], [Z5(4), Z5(1)]]);
    // nur die Diagonale überlebt: 2 ⋅ (1, 0) + 1 ⋅ (0, 1)
    assert_eq!(t.lift(f), Vector::new([Z5(2), Z5(1)]));
}

#[test]
fn is_pure_counts_the_rank_at_most_one_matrices() {
    // über 𝔽₅ gibt es 1 + (5³ − 1)(5² − 1)/(5 − 1) = 745 Matrizen 3×2 vom Rang höchstens eins
    let pure = (0..15625usize)
        .filter(|&i| {
            let rows: [[Z5; 2]; 3] = std::array::from_fn(|r| {
                std::array::from_fn(|c| z(i / 5usize.pow((2 * r + c) as u32)))
            });
            Tensor::new(rows).is_pure()
        })
        .count();
    assert_eq!(pure, 745);
}

#[test]
fn is_pure_looks_at_every_minor_of_a_three_by_three_tensor() {
    // Rang 2, aber nur der Minor in den letzten Zeilen und Spalten ist ungleich null
    let t = Tensor::new([
        [Z5(0), Z5(0), Z5(0)],
        [Z5(0), Z5(1), Z5(0)],
        [Z5(0), Z5(0), Z5(1)],
    ]);
    assert!(!t.is_pure());
    // ein reiner Tensor v ⊗ w ist rein
    let v = vec3(37);
    let w = vec3(91);
    let pure = <Tensor<Z5, 3, 3> as TensorProduct<V3, V3, Z5>>::tensor(&v, &w);
    assert!(pure.is_pure());
    // die Summe zweier reiner Tensoren mit unabhängigen Faktoren ist es nicht
    let a = <Tensor<Z5, 3, 3> as TensorProduct<V3, V3, Z5>>::tensor(&V3::basis(0), &V3::basis(0));
    let b = <Tensor<Z5, 3, 3> as TensorProduct<V3, V3, Z5>>::tensor(&V3::basis(2), &V3::basis(1));
    assert!(!<Tensor<Z5, 3, 3> as Magma<Additive>>::op(&a, &b).is_pure());
}
