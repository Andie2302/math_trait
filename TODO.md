# TODO

Offene Themen und Ideen, grob nach Priorität.

## Geplant

- **Kommutator-Konstruktion**: Jede assoziative Algebra wird mit `[x, y] = xy − yx` zur Lie-Algebra (so entstehen die Matrix-Lie-Algebren und die Drehimpuls-Operatoren der Quantenmechanik). Als Wrapper-Typ `Commutator<A>` denkbar.
- **Weiter bei Lie-Algebren**: Ideale, Darstellungen (adjungierte Darstellung), Killing-Form (eine `BilinearForm`), einhüllende Algebra, Lie-Gruppen.
- **Clifford-Algebren**: Algebra, die von einer quadratischen Form erzeugt wird (`v∘v = Q(v)`). Baut auf `QuadraticForm` auf.
- **Tensorrechnung**: Tensorprodukt von Moduln/Algebren.

## Lücken im bisherigen Entwurf

- **Partielle Seite ohne Methoden**: `PartialMagma` und Verwandte brauchen `Option<Self>` als Ergebnis, das passt nicht in die Supertrait-Kette von `Magma`.
- **Heterogenes `Bilinear`** hat noch keine Methode (die homogene Fassung ist `Magma::op`).
- **Kehrwert als Typ**: `recip` liefert `Option`. Eleganter wäre ein Typ `NonZero<K>`, der `Group<Multiplicative>` erfüllt (die Einheitengruppe `K×`).
- **Division**: `a / b = a ⋅ b⁻¹` (und im Schiefkörper `b⁻¹ ⋅ a`) als abgeleitete Methoden.
- **Makros für weitere Strukturen**: `impl_module!`/`impl_algebra!` kennen nur die Standard-Etiketten; es fehlen Varianten für `UnitalAlgebra`, `AssociativeAlgebra`, `DivisionAlgebra`, `CompositionAlgebra`, Quasigruppen, Loops, `StarRing`.
- **Analytische Norm** (`‖x‖ ≥ 0`, Dreiecksungleichung): braucht geordnete Körper bzw. Beträge. Bisher gibt es nur die algebraische Norm-Form (`CompositionAlgebra`).
- **Rechtswirkung / Bimoduln**: bisher nur `LeftAction`. Nötig für Moduln über nicht-kommutativen Ringen.
- **Getrennte Etiketten für Skalare und Vektoren** in `Module`: aktuell teilen sie sich `Add`/`Mul`.
- **Rng und Semiring**: Ring ohne Eins bzw. mit nur kommutativem Monoid in der Addition.
- **Cayley-Dickson-Konstruktion** selbst (nicht nur die Eigenschaften der Ergebnisse).
- **Zusammenhang Kürzbarkeit/Idempotenz**: `Band`, `Semilattice` (siehe `Idempotent`).
- **Kein Default für `Op`** bei den Einzel-Verknüpfungs-Traits. Bei Bedarf ein Standard-Etikett einführen.

## Später

- **Basisdatentypen und externe Crates** (`i32`, `f64`, `num-complex`, …): Wegen der Orphan-Regel können die Impls nicht in einem Aufsatz-Crate stehen, solange die Etiketten (`Additive`, …) aus `math_trait` kommen. Geplant: Feature-Flags in `math_trait` selbst. Die Zahlen-Traits (`numeric::Number`, `Integer`, `Float`, `Signed`, `Unsigned`) sind dafür vorbereitet.
- **Semiring** und `CommutativeSemiring`, damit `Unsigned` (ℕ-artig) exakt ausgedrückt werden kann.
- Prüffunktionen für die Gesetze (Assoziativität, Distributivität usw.) zum Testen von Datentypen. Mit den Methoden jetzt möglich.
