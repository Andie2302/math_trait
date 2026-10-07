# math_trait

Ein Trait-System für Mathematik in Rust, **unabhängig von konkreten Basisdatentypen**: Das Crate
enthält weder `i32` noch `f64` noch eine externe Zahlenbibliothek. Es beschreibt zuerst, *was* eine
Gruppe, ein Ring, eine Algebra ist, und lässt dich danach entscheiden, welche Typen das erfüllen.

`#![no_std]`, `#![forbid(unsafe_code)]`, keine Abhängigkeiten. Mindestversion: Rust 1.85
(Edition 2024).

## Die Idee in drei Sätzen

1. **Die Verknüpfung ist ein Typparameter.** Es heißt `Magma<Op>` statt `Magma`. Ein Typ kann so
   mehrere Rollen haben (`Magma<Additive>` *und* `Magma<Multiplicative>`), und `Op` ist ein
   beliebiger Etikett-Typ, auch einer aus deinem eigenen Crate.
2. **Fast alles ist ein Marker.** Nur wenige Traits haben Methoden (`op`, `identity`, `ldiv`/`rdiv`,
   `inverse`, `recip`, `act`, `form`, `value`, `conjugate`, `tensor`, `embed`, `lie_act`,
   `even_part`/`odd_part`). Das Gesetz steht in der Doc-Zeile.
3. **Die Gesetze prüft der Compiler nicht.** Ein Marker-Impl ist eine Zusicherung des Autors. Die
   Makros `impl_*!` erzeugen alle nötigen Impls aus kurzen Rümpfen, behaupten aber nur.

## Hierarchie

```text
Magma ┬─ Quasigroup ─┐
      │              ├─ Loop ─────────────┐
      ├─ UnitalMagma ┘                    │
      └─ Semigroup ─ Monoid ──────────────┴─ Group ─ AbelianGroup

Semiring ─ Rng/Ring ─ CommutativeRing ─ Field         (zwei Verknüpfungen: Add, Mul)
Module ─ VectorSpace;  Algebra ─ Unital/Associative/Alternative/Division/CompositionAlgebra
LieAlgebra (Antikommutativität, Alternieren, Jacobi); Involutionen; TensorProduct, CliffordAlgebra
```

Dazu kommen die Zweige für Halbverbände, Medial-/Distributiv-Familien und die partielle Seite
(Semigroupoid, Kategorie, Groupoid). Die vollständige Übersicht steht in der Crate-Dokumentation.

## Konstruktionen

Das Crate enthält auch konkrete Bausteine, die *nur* aus den Traits gebaut sind:

| Typ | Was es ist |
|---|---|
| `Commutator<A>` | jede assoziative Algebra wird mit `[x, y] = xy − yx` eine Lie-Algebra |
| `CayleyDickson<A, R, G>` | Verdopplung: ℝ → ℂ → ℍ → 𝕆 → 𝕊 …, mit einstellbarem Parameter γ |
| `Units<K>` | die Einheitengruppe K× eines Körpers |
| `Vector<R, N>`, `Tensor<R, M, N>` | R^N und V ⊗ W |
| `Clifford<R, D, Q>` | freie Clifford-Algebra zu einer Diagonalform, mit Graduierung |
| `EvenSubalgebra`, `Rotor`, `Bivector` | gerade Teilalgebra, Spin-Gruppe, Lie-Algebra so(N) |

## Beispiel: eine Gruppe mit einem Makro

```rust
use math_trait::{impl_group, Additive, Group, Magma};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Z5(u8);

impl_group!(Z5, Additive;
    op(a, b) { Z5((a.0 + b.0) % 5) }
    identity() { Z5(0) }
    inverse(x) { Z5((5 - x.0) % 5) }
);

fn double<G: Group<Additive>>(g: &G) -> G {
    <G as Magma<Additive>>::op(g, g)
}

fn main() {
    assert_eq!(double(&Z5(4)), Z5(3));
    assert_eq!(<Z5 as Group<Additive>>::inverse(&Z5(2)), Z5(3));
}
```

Strukturen mit zwei Verknüpfungen und Module bekommt man mit `impl_ring!`, `impl_field!`,
`impl_module!`, `impl_algebra!` usw. Alle Makros nehmen optional Typparameter mit Bedingungen vorweg
(`impl_x!(for [A: Bound, const N: usize] Typ<A>, Op; …)`).

## Grenzen, die man kennen sollte

- **Orphan-Regel:** Ein anderes Crate kann `Magma<Additive>` nicht für fremde Typen wie `i32`
  implementieren, wenn weder Trait noch Typ lokal sind. Impls für Basisdatentypen gehören daher
  später in dieses Crate (geplant: Feature-Flags). Mit eigenen Etiketten und eigenen Typen
  funktioniert alles heute schon.
- **Gesetze werden nicht geprüft.** Wer `impl_group!` benutzt, behauptet Assoziativität, Neutralität
  und Inverse. Tests über endlichen Strukturen (hier ℤ/5) prüfen sie stichprobenartig.
- Die Konstruktionen sind für **Körper** gedacht. Über endlichen Körpern der Charakteristik 2 (und
  bei ausgearteten Formen) gelten einige Aussagen nicht mehr, die über ℝ oder ℂ stimmen. Die
  Dokumentation der einzelnen Typen nennt die Bedingungen.

## Entwicklung

```sh
cargo test                                  # Tests und Doctests
cargo clippy --all-targets
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

Offene Punkte und Ideen stehen in [TODO.md](TODO.md).

## Lizenz

Wahlweise unter [MIT](LICENSE-MIT) oder [Apache-2.0](LICENSE-APACHE).
