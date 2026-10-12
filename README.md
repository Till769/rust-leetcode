# LeetCode in Rust – Lernrepo

Dieses Repo ist mein persönliches Übungsfeld. Ich nutze es für zwei Dinge:

- **Rust wieder auffrischen:** Mein letzter Kontakt mit Rust ist eine Weile her.
- **Algorithmen und Datenstrukturen neu aufbauen:** Das Uni-Wissen dazu ist größtenteils verschüttet.

Ich löse LeetCode-Aufgaben in Rust und dokumentiere dabei meinen Lernweg.

## Aufbau

```
src/
  lib.rs                         # bindet die Aufgaben als Module ein
  pXXXX_<aufgabenname>.rs        # eine Datei pro Aufgabe: Lösung + Tests
lerntagebuch/
  JJJJ-MM-TT.md                  # Lerntagebuch, eine Datei pro Übungstag
```

Tests laufen lokal mit `cargo test` oder gezielt für eine Aufgabe, zum Beispiel mit `cargo test p2073`.

## Wie ich arbeite und welche Rolle KI dabei spielt

Ich übe zusammen mit Claude (Anthropic). Claude arbeitet dabei bewusst als **sokratischer Tutor** und gibt mir keine fertigen Lösungen. Den Lösungsweg erarbeite ich mir über Rückfragen, Gegenbeispiele und gestufte Hinweise selbst. Wie viel Hilfe ich bei einer Aufgabe gebraucht habe, steht im jeweiligen Tagebucheintrag unter „Hinweisstufe“.

### Von mir

- **Die Lösungen:** der Code innerhalb der `impl Solution`-Blöcke, also Algorithmus und Rust-Umsetzung.
- **Die Kernidee jeder Aufgabe** im Lerntagebuch, unter „Kernidee (eigene Worte)“, wörtlich übernommen.
- Komplexitätsanalyse und Korrekturen nach dem Code-Review.

### KI-generiert

- **Datei-Gerüste:** Funktionssignatur mit `todo!()` und die Testfälle (`#[cfg(test)]`), also die Beispiele aus der Aufgabe plus Randfälle.
- **Das Lerntagebuch:** Claude schreibt die Einträge in `lerntagebuch/` auf Basis unseres Gesprächs. Mein Code und meine Kernidee stehen darin unverändert. Zusammenfassungen, „Schärfung“, „Weg zur Lösung“, „Wo ich hing“ und „Rust-Learnings“ formuliert Claude.
- Diese README, inhaltlich nach meinen Vorgaben.

## Hinweis

Die Aufgabenstellungen stammen von [LeetCode](https://leetcode.com/). Die Lösungen sind Lernstände und nicht immer die eleganteste Fassung. Offene Verbesserungsideen stehen jeweils im Tagebuch.
