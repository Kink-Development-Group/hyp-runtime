---
title: Hypnotic Functions
---

# Hypnotic Functions

HypnoScript bietet spezielle Funktionen für hypnotische Anwendungen und Trance-Induktion.

## Übersicht

Hypnotische Funktionen sind das Herzstück von HypnoScript und ermöglichen es Ihnen, hypnotische Sitzungen, Trance-Induktionen und therapeutische Anwendungen zu programmieren.

## Grundlegende Trance-Funktionen

### HypnoticBreathing

Führt eine hypnotische Atemübung durch.

```hyp
// Einfache Atemübung
HypnoticBreathing();

// Atemübung mit spezifischer Anzahl von Zyklen
HypnoticBreathing(10);
```

**Parameter:**

- `cycles` (optional): Anzahl der Atemzyklen (Standard: 5)

### HypnoticAnchoring

Erstellt oder aktiviert einen hypnotischen Anker.

```hyp
// Anker erstellen
HypnoticAnchoring("Entspannung");

// Anker mit spezifischem Gefühl
HypnoticAnchoring("Sicherheit", "Wärme");
```

**Parameter:**

- `anchorName`: Name des Ankers
- `feeling` (optional): Assoziiertes Gefühl

### HypnoticRegression

Führt eine hypnotische Regression durch.

```hyp
// Standard-Regression
HypnoticRegression();

// Regression zu spezifischem Alter
HypnoticRegression(7);
```

**Parameter:**

- `targetAge` (optional): Zielalter für Regression

### HypnoticFutureProgression

Führt eine hypnotische Zukunftsvision durch.

```hyp
// Standard-Zukunftsvision
HypnoticFutureProgression();

// Vision für spezifisches Jahr
HypnoticFutureProgression(5); // 5 Jahre in der Zukunft
```

**Parameter:**

- `yearsAhead` (optional): Jahre in die Zukunft

## Erweiterte hypnotische Funktionen

### ProgressiveRelaxation

Führt eine progressive Muskelentspannung durch.

```hyp
// Standard-Entspannung
ProgressiveRelaxation();

// Entspannung mit spezifischer Dauer pro Muskelgruppe
ProgressiveRelaxation(3); // 3 Sekunden pro Gruppe
```

**Parameter:**

- `durationPerGroup` (optional): Dauer pro Muskelgruppe in Sekunden

### HypnoticVisualization

Führt eine hypnotische Visualisierung durch.

```hyp
// Einfache Visualisierung
HypnoticVisualization("ein friedlicher Garten");

// Detaillierte Visualisierung
HypnoticVisualization("ein sonniger Strand mit sanften Wellen", 30);
```

**Parameter:**

- `scene`: Die zu visualisierende Szene
- `duration` (optional): Dauer in Sekunden

### HypnoticSuggestion

Gibt eine hypnotische Suggestion.

```hyp
// Positive Suggestion
HypnoticSuggestion("Du fühlst dich zunehmend entspannt und sicher");

// Suggestion mit Verstärkung
HypnoticSuggestion("Mit jedem Atemzug wirst du tiefer entspannt", 3);
```

**Parameter:**

- `suggestion`: Die hypnotische Suggestion
- `repetitions` (optional): Anzahl der Wiederholungen

### TranceDeepening

Vertieft den hypnotischen Trance-Zustand.

```hyp
// Standard-Trancevertiefung
TranceDeepening();

// Vertiefung mit spezifischem Level
TranceDeepening(3); // Level 3 (tief)
```

**Parameter:**

- `level` (optional): Trance-Level (1-5, 5 = am tiefsten)

## Spezialisierte hypnotische Funktionen

### EgoStateTherapy

Führt eine Ego-State-Therapie durch.

```hyp
// Ego-State-Identifikation
induce egoState = EgoStateTherapy("identify");

// Ego-State-Integration
EgoStateTherapy("integrate", egoState);
```

**Parameter:**

- `action`: Aktion ("identify", "integrate", "communicate")
- `state` (optional): Ego-State für Integration

### PartsWork

Arbeitet mit inneren Anteilen.

```hyp
// Inneren Anteil identifizieren
induce part = PartsWork("find", "Angst");

// Mit Anteil kommunizieren
PartsWork("communicate", part, "Was brauchst du?");
```

**Parameter:**

- `action`: Aktion ("find", "communicate", "integrate")
- `partName`: Name des Anteils
- `message` (optional): Nachricht an den Anteil

### TimelineTherapy

Führt eine Timeline-Therapie durch.

```hyp
// Timeline erstellen
induce timeline = TimelineTherapy("create");

// Auf Timeline navigieren
TimelineTherapy("navigate", timeline, "Vergangenheit");
```

**Parameter:**

- `action`: Aktion ("create", "navigate", "heal")
- `timeline` (optional): Timeline-Objekt
- `location` (optional): Position auf der Timeline

### HypnoticPacing

Führt hypnotisches Pacing und Leading durch.

```hyp
// Pacing - aktuelle Erfahrung spiegeln
HypnoticPacing("Du sitzt hier und atmest");

// Leading - in gewünschte Richtung führen
HypnoticLeading("Und mit jedem Atemzug entspannst du dich mehr");
```

**Parameter:**

- `statement`: Die Pacing- oder Leading-Aussage

## Therapeutische Funktionen

### PainManagement

Hypnotische Schmerzbehandlung.

```hyp
// Schmerzreduktion
PainManagement("reduce", "Kopfschmerzen");

// Schmerztransformation
PainManagement("transform", "Rückenschmerzen", "Wärme");
```

**Parameter:**

- `action`: Aktion ("reduce", "transform", "eliminate")
- `painType`: Art des Schmerzes
- `transformation` (optional): Transformation des Schmerzes

### AnxietyReduction

Reduziert Angst und Anspannung.

```hyp
// Angstreduktion
AnxietyReduction("general");

// Spezifische Angst behandeln
AnxietyReduction("social", 0.8); // 80% Reduktion
```

**Parameter:**

- `type`: Art der Angst ("general", "social", "performance")
- `reductionLevel` (optional): Reduktionslevel (0.0-1.0)

### ConfidenceBuilding

Baut Selbstvertrauen auf.

```hyp
// Allgemeines Selbstvertrauen
ConfidenceBuilding();

// Spezifisches Selbstvertrauen
ConfidenceBuilding("public-speaking", 0.9);
```

**Parameter:**

- `area` (optional): Bereich des Selbstvertrauens
- `level` (optional): Gewünschtes Level (0.0-1.0)

### HabitChange

Unterstützt Gewohnheitsänderungen.

```hyp
// Gewohnheit identifizieren
induce habit = HabitChange("identify", "Rauchen");

// Gewohnheit ändern
HabitChange("modify", habit, "gesunde Atemübungen");
```

**Parameter:**

- `action`: Aktion ("identify", "modify", "eliminate")
- `habitName`: Name der Gewohnheit
- `replacement` (optional): Ersatzverhalten

## Monitoring und Feedback

### TranceDepth

Misst die aktuelle Trance-Tiefe.

```hyp
induce depth = TranceDepth();
observe "Aktuelle Trance-Tiefe: " + depth + "/10";
```

**Rückgabewert:** Trance-Tiefe von 1-10

### HypnoticResponsiveness

Misst die hypnotische Reaktionsfähigkeit.

```hyp
induce responsiveness = HypnoticResponsiveness();
observe "Hypnotische Reaktionsfähigkeit: " + responsiveness + "%";
```

**Rückgabewert:** Reaktionsfähigkeit in Prozent

### SuggestionAcceptance

Überprüft die Akzeptanz von Suggestionen.

```hyp
induce acceptance = SuggestionAcceptance("Du fühlst dich entspannt");
observe "Suggestion-Akzeptanz: " + acceptance + "%";
```

**Parameter:**

- `suggestion`: Die zu testende Suggestion

**Rückgabewert:** Akzeptanz in Prozent

## Sicherheitsfunktionen

### SafetyCheck

Führt eine Sicherheitsüberprüfung durch.

```hyp
induce safetyStatus = SafetyCheck();
if (safetyStatus.isSafe) {
    observe "Sitzung ist sicher";
} else {
    observe "Sicherheitswarnung: " + safetyStatus.warning;
}
```

**Rückgabewert:** Sicherheitsstatus-Objekt

### EmergencyExit

Notfall-Ausstieg aus Trance.

```hyp
// Sofortiger Ausstieg
EmergencyExit();

// Sanfter Ausstieg
EmergencyExit("gentle");
```

**Parameter:**

- `mode` (optional): Ausstiegsmodus ("immediate", "gentle")

### Grounding

Erdet den Klienten nach der Sitzung.

```hyp
// Standard-Erdung
Grounding();

// Erweiterte Erdung
Grounding("visual", 60); // Visuelle Erdung für 60 Sekunden
```

**Parameter:**

- `method` (optional): Erdungsmethode ("visual", "physical", "mental")
- `duration` (optional): Dauer in Sekunden

## Best Practices

### Vollständige hypnotische Sitzung

```hyp
Focus {
    entrance {
        // Sicherheitscheck
        induce safety = SafetyCheck();
        if (!safety.isSafe) {
            observe "Sitzung nicht sicher - Abbruch";
            return;
        }

        // Einleitung
        observe "Willkommen zu Ihrer hypnotischen Sitzung";
        drift(2000);

        // Trance-Induktion
        HypnoticBreathing(5);
        ProgressiveRelaxation(3);

        // Trance vertiefen
        TranceDeepening(3);

        // Hauptarbeit
        HypnoticSuggestion("Du fühlst dich zunehmend entspannt und sicher", 3);
        HypnoticVisualization("ein friedlicher Garten", 30);

        // Erdung
        Grounding("visual", 60);

        observe "Sitzung erfolgreich abgeschlossen";
    }
} Relax;
```

### Therapeutische Anwendung

```hyp
Focus {
    entrance {
        // Anamnese
        induce clientName = InputProvider("Name des Klienten: ");
        induce issue = InputProvider("Hauptproblem: ");

        // Sicherheitscheck
        if (!SafetyCheck().isSafe) {
            observe "Klient ist nicht für Hypnose geeignet";
            return;
        }

        // Individuelle Sitzung
        if (issue == "Angst") {
            AnxietyReduction("general", 0.8);
        } else if (issue == "Schmerzen") {
            PainManagement("reduce", "chronische Schmerzen");
        } else if (issue == "Gewohnheit") {
            induce habit = HabitChange("identify", "Rauchen");
            HabitChange("modify", habit, "tiefe Atemzüge");
        }

        // Nachsorge
        observe "Therapeutische Sitzung abgeschlossen";
        observe "Nächster Termin in einer Woche empfohlen";
    }
} Relax;
```

### Gruppen-Hypnose

```hyp
Focus {
    entrance {
        // Gruppeneinstimmung
        induce groupSize = InputProvider("Anzahl Teilnehmer: ");
        observe "Willkommen zur Gruppen-Hypnose-Sitzung";

        // Kollektive Trance-Induktion
        HypnoticBreathing(3);
        ProgressiveRelaxation(2);

        // Gruppen-Suggestion
        HypnoticSuggestion("Ihr alle fühlt euch zunehmend entspannt", 2);

        // Individuelle Arbeit (simuliert)
        for (induce i = 0; i < groupSize; induce i = i + 1) {
            induce individualDepth = TranceDepth();
            observe "Teilnehmer " + (i + 1) + " Trance-Tiefe: " + individualDepth;
        }

        // Gruppen-Erdung
        Grounding("visual", 45);

        observe "Gruppen-Sitzung erfolgreich abgeschlossen";
    }
} Relax;
```

## Sicherheitsrichtlinien

### Wichtige Sicherheitsaspekte

1. **Immer SafetyCheck durchführen** vor jeder hypnotischen Sitzung
2. **Notfall-Ausstieg bereithalten** mit EmergencyExit()
3. **Sanfte Einleitung** mit HypnoticBreathing und ProgressiveRelaxation
4. **Individuelle Anpassung** der Sitzung an den Klienten
5. **Ausreichende Erdung** nach jeder Sitzung

### Kontraindikationen

```hyp
// Prüfe Kontraindikationen
induce contraindications = CheckContraindications();
if (contraindications.hasPsychosis) {
    observe "WARNUNG: Psychose - Hypnose kontraindiziert";
    return;
}
if (contraindications.hasEpilepsy) {
    observe "VORSICHT: Epilepsie - Sanfte Hypnose nur unter Aufsicht";
}
```

## Fehlerbehandlung

Hypnotische Funktionen können bei unerwarteten Reaktionen Fehler werfen:

```hyp
Focus {
    entrance {
        try {
            HypnoticBreathing(5);
            observe "Atemübung erfolgreich";
        } catch (error) {
            observe "Fehler bei Atemübung: " + error;
            EmergencyExit("gentle");
        }

        try {
            induce depth = TranceDepth();
            if (depth < 3) {
                observe "Trance zu flach - vertiefen";
                TranceDeepening(2);
            }
        } catch (error) {
            observe "Fehler bei Trance-Monitoring: " + error;
        }
    }
} Relax;
```

## Nächste Schritte

- [System Functions](./system-functions) - System-spezifische Funktionen
- [Time & Date Functions](./time-date-functions) - Zeit- und Datumsfunktionen
- [Therapeutic Applications](../examples/therapeutic-examples) - Therapeutische Anwendungen

---

**Hypnotische Funktionen gemeistert? Dann lerne [System Functions](./system-functions) kennen!** ✅
