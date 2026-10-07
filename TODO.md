# TODO

Offene Themen und Ideen, grob nach Priorität.

## Geplant

- **Weiter bei Lie-Algebren**: Ideale und Quotienten, Killing-Form (eine `BilinearForm` aus der Spur von `ad ∘ ad`, braucht endliche Dimension), einhüllende Algebra, Lie-Gruppen, die Exponentialabbildung `exp: so(N) → Spin(N)` (braucht Charakteristik 0 oder Nilpotenz), Wurzeln und Gewichte, Darstellungstheorie (irreduzible Darstellungen). `so(N)` aus den Bivektoren (`Bivector`) mit Vektor- und Spindarstellung gibt es.
- **Clifford-Algebren**: `Clifford<R, D, Q>` ist die freie Clifford-Algebra zu **Diagonalformen**, mit `lift`, Graduierung (`GradedAlgebra`), den drei Involutionen, der geraden Unteralgebra (`EvenSubalgebra`) und der Rotorgruppe (`Rotor`). Offen: nicht diagonale Formen (Basiswechsel), die Pin-Gruppe (Spiegelungen), die ℤ-Graduierung als eigenes Trait (nur ℤ/2 ist ein Trait, die Grade stehen als `grade_part`), Hodge-Dualität, die Zerlegung in Weyl-Spinoren (gerader und ungerader Teil) und Majorana-Bedingungen. Vorhanden ist die Spindarstellung als `LieModule` auf `Clifford`.
- **Rotor und Spin**: `Rotor::new` prüft die Vektor-Bedingung (`s v s̃` ist wieder ein Vektor), ohne die ab `N = 6` mehr als die Spin-Gruppe herauskäme. Offen: die Spinornorm als eigene Funktion, damit sich das Bild in der Drehgruppe (Index zwei über endlichen Körpern) beschreiben lässt.
- **Tensorrechnung**: `Tensor<R, M, N>` ist `R^M ⊗ R^N` mit `lift` und `is_pure`. Offen: mehr als zwei Faktoren, Tensorprodukt von Algebren (komponentenweises Produkt), Kronecker-Produkt linearer Abbildungen, symmetrische und äußere Potenzen, Kontraktion.

## Lücken im bisherigen Entwurf

- **Bekannte Einschränkungen der Geometrie**: Über Körpern der Charakteristik 2 und bei ausgearteten Formen sind `Clifford`, `Rotor` und `Bivector` formal erlaubt, ihre geometrischen Aussagen (Drehungen, `so(N)`, treue Vektordarstellung) gelten dort nicht. Das steht in den Doku-Kommentaren, das Crate prüft es nicht.
- **Ring-Reihe für `CayleyDickson`**: `Clifford` und `EvenSubalgebra` sind `Ring`, die assoziative `CayleyDickson`-Verdopplung ist es noch nicht (Monoid, Annihilating, Semiring, Rng, Ring unter den Bedingungen der Semigroup-Impls). Dasselbe gilt für `CommutativeSemigroup`/`Medial` im kommutativen Fall.
- **Operatoren und Hilfsfunktionen**: Es gibt keine `+`, `-`, `*` für die Konstruktionen und keine öffentlichen Hilfsfunktionen (`add`, `mul`, …). Jede Rechnung braucht die ausführliche Schreibweise `<T as Magma<Additive>>::op(&a, &b)`. Bei eigenen Typen wären `core::ops`-Impls orphan-konform möglich.
- **`recip` in `DivisionRing` und `DivisionAlgebra`**: Für einen Körper mit `impl_field!` *und* `impl_field_algebra!` ist `x.recip()` mehrdeutig (E0034); die ausführliche Schreibweise löst es. Eine Umbenennung ist noch offen.
- **Eigene Etiketten**: Sie funktionieren bis zu `Magma`, `Group`, `Ring` usw. Ab `Module`/`Algebra` und in allen Konstruktionen sind aber nur `Additive`/`Multiplicative` möglich.
- **Marker-Familien ohne Impl**: `Semimedial`, `SelfDistributive`, `Unipotent`, `Zeropotent`, `LeftUnar`, `RightUnar`, `NullSemigroup`, `LeftZero/RightZeroSemigroup`, `Central`, `Entropic` und `BilinearForm` haben kein Makro und keine Beispiel-Impl.
- **Partielle Seite ohne Methoden**: `PartialMagma` und Verwandte brauchen `Option<Self>` als Ergebnis, das passt nicht in die Supertrait-Kette von `Magma`.
- **Cayley-Dickson**: `CayleyDickson<A, R, G>` verdoppelt mit dem Parameter `γ` aus `G` (Standard `−1`). Offen: `DivisionAlgebra` (braucht geordnete Körper bzw. eine anisotrope Norm) und die Bedingung "assoziativ" für Verdopplungen ohne Kompositions-Voraussetzung. Die bedingten Impls (alternativ, assoziativ, kommutativ) stützen sich auf bekannte Sätze und die Tests, der Compiler beweist sie nicht.
- **Makros für bedingte Impls**: Die bedingten Impls von `CayleyDickson` (Eigenschaften, die nur unter Voraussetzungen an die Ausgangsalgebra gelten) sind noch von Hand geschrieben.
- **Analytische Norm** (`‖x‖ ≥ 0`, Dreiecksungleichung): braucht geordnete Körper bzw. Beträge. Bisher gibt es nur die algebraische Norm-Form (`CompositionAlgebra`).
- **Rechtswirkung / Bimoduln**: bisher nur `LeftAction`. Nötig für Moduln über nicht-kommutativen Ringen.
- **Getrennte Etiketten für Skalare und Vektoren** in `Module`: aktuell teilen sie sich `Add`/`Mul`.
- **Kein Default für `Op`** bei den Einzel-Verknüpfungs-Traits. Bei Bedarf ein Standard-Etikett einführen.

## Später

- **Basisdatentypen und externe Crates** (`i32`, `f64`, `num-complex`, …): Wegen der Orphan-Regel können die Impls nicht in einem Aufsatz-Crate stehen, solange die Etiketten (`Additive`, …) aus `math_trait` kommen. Geplant: Feature-Flags in `math_trait` selbst. Die Zahlen-Traits (`numeric::Number`, `Integer`, `Float`, `Signed`, `Unsigned`) sind dafür vorbereitet.
- **Testfixtures zusammenführen**: `tests/common/mod.rs` enthält ℤ/5 und die Diagonalformen für neue Tests. Die älteren Testdateien (rund 13) haben noch eigene Kopien von `Z5`, `Mat2`, den Formen und den Gesetzesprüfern.
- Prüffunktionen für die Gesetze (Assoziativität, Distributivität usw.) zum Testen von Datentypen. Mit den Methoden jetzt möglich.
