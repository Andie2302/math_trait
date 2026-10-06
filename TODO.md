# TODO

Offene Themen und Ideen, grob nach Priorität.

## Geplant

- **Lie-Algebren**: Algebra mit bilinearer, alternierender Klammer `[x, y]`, die die Jacobi-Identität erfüllt (`[x,[y,z]] + [y,[z,x]] + [z,[x,y]] = 0`). Baut auf `Algebra` und `Bilinear` auf, ist aber in der Regel *nicht* assoziativ. Richtung Quantenphysik (Drehimpuls, Symmetriegruppen, Lie-Gruppen).
- **Clifford-Algebren**: Algebra, die von einer quadratischen Form erzeugt wird (`v∘v = Q(v)`). Baut auf `QuadraticForm` auf.
- **Tensorrechnung**: Tensorprodukt von Moduln/Algebren.

## Lücken im bisherigen Entwurf

- **Unäre Operationen**: Inverse (`Neg`, Kehrwert), Konjugation und Norm sind Funktionen `T → T`. Als leere Marker-Traits sind sie nur beschrieben, nicht ausgedrückt. Ebenso neutrale Elemente (0, 1) als Werte.
- **Analytische Norm** (`‖x‖ ≥ 0`, Dreiecksungleichung): braucht geordnete Körper bzw. Beträge. Bisher gibt es nur die algebraische Norm-Form (`CompositionAlgebra`).
- **Rechtswirkung / Bimoduln**: bisher nur `LeftAction`. Nötig für Moduln über nicht-kommutativen Ringen.
- **Getrennte Etiketten für Skalare und Vektoren** in `Module`: aktuell teilen sie sich `Add`/`Mul`.
- **Rng und Semiring**: Ring ohne Eins bzw. mit nur kommutativem Monoid in der Addition.
- **Cayley-Dickson-Konstruktion** selbst (nicht nur die Eigenschaften der Ergebnisse).
- **Zusammenhang Kürzbarkeit/Idempotenz**: `Band`, `Semilattice` (siehe `Idempotent`).
- **Kein Default für `Op`** bei den Einzel-Verknüpfungs-Traits. Bei Bedarf ein Standard-Etikett einführen.

## Später (Aufsatz-Crate)

- Implementierungen für Basisdatentypen und externe Crates.
- Prüffunktionen für die Gesetze (Assoziativität, Distributivität usw.) zum Testen von Datentypen.
