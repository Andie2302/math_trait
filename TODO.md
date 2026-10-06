# TODO

Offene Themen und Ideen, grob nach Priorität.

## Geplant

- **Weiter bei Lie-Algebren**: Ideale und Quotienten, Killing-Form (eine `BilinearForm` aus der Spur von `ad ∘ ad`, braucht endliche Dimension), einhüllende Algebra, Lie-Gruppen. Darstellungen (`LieModule`) gibt es.
- **Clifford-Algebren**: die Konstruktion selbst (aus `(V, Q)` die freie Algebra bilden) fehlt. Das Trait `CliffordAlgebra` beschreibt nur Algebren, die schon eine Einbettung haben. Dazu: Spin-Gruppen, graduierte Struktur.
- **Tensorrechnung**: Das Trait `TensorProduct` gibt es. Offen: Tensorprodukt von Algebren (komponentenweises Produkt), symmetrische/äußere Potenzen, Kontraktion, die Konstruktion `V ⊗ W` selbst als Typ.

## Lücken im bisherigen Entwurf

- **Partielle Seite ohne Methoden**: `PartialMagma` und Verwandte brauchen `Option<Self>` als Ergebnis, das passt nicht in die Supertrait-Kette von `Magma`.
- **Cayley-Dickson**: `CayleyDickson<A, R, G>` verdoppelt mit dem Parameter `γ` aus `G` (Standard `−1`). Offen: `DivisionAlgebra` (braucht geordnete Körper bzw. eine anisotrope Norm) und die Bedingung "assoziativ" für Verdopplungen ohne Kompositions-Voraussetzung. Die bedingten Impls (alternativ, assoziativ, kommutativ) stützen sich auf bekannte Sätze und die Tests, der Compiler beweist sie nicht.
- **Makros für bedingte Impls**: Die bedingten Impls von `CayleyDickson` (Eigenschaften, die nur unter Voraussetzungen an die Ausgangsalgebra gelten) sind noch von Hand geschrieben.
- **Analytische Norm** (`‖x‖ ≥ 0`, Dreiecksungleichung): braucht geordnete Körper bzw. Beträge. Bisher gibt es nur die algebraische Norm-Form (`CompositionAlgebra`).
- **Rechtswirkung / Bimoduln**: bisher nur `LeftAction`. Nötig für Moduln über nicht-kommutativen Ringen.
- **Getrennte Etiketten für Skalare und Vektoren** in `Module`: aktuell teilen sie sich `Add`/`Mul`.
- **Kein Default für `Op`** bei den Einzel-Verknüpfungs-Traits. Bei Bedarf ein Standard-Etikett einführen.

## Später

- **Basisdatentypen und externe Crates** (`i32`, `f64`, `num-complex`, …): Wegen der Orphan-Regel können die Impls nicht in einem Aufsatz-Crate stehen, solange die Etiketten (`Additive`, …) aus `math_trait` kommen. Geplant: Feature-Flags in `math_trait` selbst. Die Zahlen-Traits (`numeric::Number`, `Integer`, `Float`, `Signed`, `Unsigned`) sind dafür vorbereitet.
- Prüffunktionen für die Gesetze (Assoziativität, Distributivität usw.) zum Testen von Datentypen. Mit den Methoden jetzt möglich.
