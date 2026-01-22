---
title: Hypnotic Functions
---

# Hypnotic Functions

HypnoScript provides specialized functions for hypnotic applications and trance induction.

## Overview

Hypnotic functions are the core of HypnoScript and enable you to program hypnotic sessions, trance inductions, and therapeutic applications.

## Basic Trance Functions

### HypnoticBreathing

Executes a hypnotic breathing exercise.

```hyp
// Simple breathing exercise
HypnoticBreathing();

// Breathing exercise with a specific number of cycles
HypnoticBreathing(10);
```

**Parameters:**

- `cycles` (optional): Number of breathing cycles (default: 5)

### HypnoticAnchoring

Creates or activates a hypnotic anchor.

```hyp
// Create an anchor
HypnoticAnchoring("Relaxation");

// Anchor with specific feeling
HypnoticAnchoring("Safety", "Warmth");
```

**Parameters:**

- `anchorName`: Name of the anchor
- `feeling` (optional): Associated feeling

### HypnoticRegression

Executes a hypnotic regression.

```hyp
// Standard regression
HypnoticRegression();

// Regression to a specific age
HypnoticRegression(7);
```

**Parameters:**

- `targetAge` (optional): Target age for regression

### HypnoticFutureProgression

Executes a hypnotic future progression.

```hyp
// Standard future progression
HypnoticFutureProgression();

// Vision for specific year
HypnoticFutureProgression(5); // 5 years in the future
```

**Parameters:**

- `yearsAhead` (optional): Years ahead

## Advanced Hypnotic Functions

### ProgressiveRelaxation

Executes progressive muscle relaxation.

```hyp
// Standard relaxation
ProgressiveRelaxation();

// Relaxation with a specific duration per muscle group
ProgressiveRelaxation(3); // 3 seconds per group
```

**Parameters:**

- `durationPerGroup` (optional): Duration per muscle group in seconds

### HypnoticVisualization

Executes a hypnotic visualization.

```hyp
// Simple visualization
HypnoticVisualization("a peaceful garden");

// Detailed visualization
HypnoticVisualization("a sunny beach with gentle waves", 30);
```

**Parameters:**

- `scene`: Scene to visualize
- `duration` (optional): Duration in seconds

### HypnoticSuggestion

Returns a hypnotic suggestion.

```hyp
// Positive suggestion
HypnoticSuggestion("You feel increasingly relaxed and safe");

// Suggestion with reinforcement
HypnoticSuggestion("With each breath you relax more deeply", 3);
```

**Parameters:**

- `suggestion`: The hypnotic suggestion
- `repetitions` (optional): Number of repetitions

### TranceDeepening

Deepens the hypnotic trance state.

```hyp
// Standard trance deepening
TranceDeepening();

// Deepening with a specific level
TranceDeepening(3); // Level 3 (deep)
```

**Parameters:**

- `level` (optional): Trance level (1-5, 5 = deepest)

## Specialized Hypnotic Functions

### EgoStateTherapy

Executes ego-state therapy.

```hyp
// Ego-state identification
induce egoState = EgoStateTherapy("identify");

// Ego-state integration
EgoStateTherapy("integrate", egoState);
```

**Parameters:**

- `action`: Action ("identify", "integrate", "communicate")
- `state` (optional): Ego state for integration

### PartsWork

Works with inner parts.

```hyp
// Identify an inner part
induce part = PartsWork("find", "Fear");

// Communicate with a part
PartsWork("communicate", part, "What do you need?");
```

**Parameters:**

- `action`: Action ("find", "communicate", "integrate")
- `partName`: Name of the part
- `message` (optional): Message to the part

### TimelineTherapy

Executes timeline therapy.

```hyp
// Create a timeline
induce timeline = TimelineTherapy("create");

// Navigate the timeline
TimelineTherapy("navigate", timeline, "Past");
```

**Parameters:**

- `action`: Action ("create", "navigate", "heal")
- `timeline` (optional): Timeline object
- `location` (optional): Position on the timeline

### HypnoticPacing

Executes hypnotic pacing and leading.

```hyp
// Pacing - mirror the current experience
HypnoticPacing("You are sitting here and breathing");

// Leading - guide in desired direction
HypnoticLeading("And with each breath you relax more");
```

**Parameters:**

- `statement`: The pacing or leading statement

## Therapeutic Functions

### PainManagement

Hypnotic pain management.

```hyp
// Pain reduction
PainManagement("reduce", "Headache");

// Pain transformation
PainManagement("transform", "Back pain", "Warmth");
```

**Parameters:**

- `action`: Action ("reduce", "transform", "eliminate")
- `painType`: Type of pain
- `transformation` (optional): Pain transformation

### AnxietyReduction

Reduces anxiety and tension.

```hyp
// Anxiety reduction
AnxietyReduction("general");

// Treat specific anxiety
AnxietyReduction("social", 0.8); // 80% reduction
```

**Parameters:**

- `type`: Type of anxiety ("general", "social", "performance")
- `reductionLevel` (optional): Reduction level (0.0-1.0)

### ConfidenceBuilding

Builds confidence.

```hyp
// General confidence
ConfidenceBuilding();

// Specific confidence
ConfidenceBuilding("public-speaking", 0.9);
```

**Parameters:**

- `area` (optional): Confidence area
- `level` (optional): Desired level (0.0-1.0)

### HabitChange

Supports habit change.

```hyp
// Identify habit
induce habit = HabitChange("identify", "Smoking");

// Change habit
HabitChange("modify", habit, "healthy breathing exercises");
```

**Parameters:**

- `action`: Action ("identify", "modify", "eliminate")
- `habitName`: Habit name
- `replacement` (optional): Replacement behavior

## Monitoring and Feedback

### TranceDepth

Measures the current trance depth.

```hyp
induce depth = TranceDepth();
observe "Current trance depth: " + depth + "/10";
```

**Return value:** Trance depth from 1-10

### HypnoticResponsiveness

Measures hypnotic responsiveness.

```hyp
induce responsiveness = HypnoticResponsiveness();
observe "Hypnotic responsiveness: " + responsiveness + "%";
```

**Return value:** Responsiveness in percent

### SuggestionAcceptance

Checks suggestion acceptance.

```hyp
induce acceptance = SuggestionAcceptance("You feel relaxed");
observe "Suggestion acceptance: " + acceptance + "%";
```

**Parameters:**

- `suggestion`: Suggestion to test

**Return value:** Acceptance in percent

## Safety Functions

### SafetyCheck

Executes a safety check.

```hyp
induce safetyStatus = SafetyCheck();
if (safetyStatus.isSafe) {
    observe "Session is safe";
} else {
    observe "Safety warning: " + safetyStatus.warning;
}
```

**Return value:** Safety status object

### EmergencyExit

Emergency exit from trance.

```hyp
// Immediate exit
EmergencyExit();

// Gentle exit
EmergencyExit("gentle");
```

**Parameters:**

- `mode` (optional): Exit mode ("immediate", "gentle")

### Grounding

Grounds the client after the session.

```hyp
// Standard grounding
Grounding();

// Extended grounding
Grounding("visual", 60); // Visual grounding for 60 seconds
```

**Parameters:**

- `method` (optional): Grounding method ("visual", "physical", "mental")
- `duration` (optional): Duration in seconds

## Best Practices

### Complete Hypnotic Session

```hyp
Focus {
    entrance {
        // Safety check
        induce safety = SafetyCheck();
        if (!safety.isSafe) {
            observe "Session not safe - aborting";
            return;
        }

        // Introduction
        observe "Welcome to your hypnotic session";
        drift(2000);

        // Trance induction
        HypnoticBreathing(5);
        ProgressiveRelaxation(3);

        // Deepen trance
        TranceDeepening(3);

        // Main work
        HypnoticSuggestion("You feel increasingly relaxed and safe", 3);
        HypnoticVisualization("a peaceful garden", 30);

        // Grounding
        Grounding("visual", 60);

        observe "Session completed successfully";
    }
} Relax;
```

### Therapeutic Application

```hyp
Focus {
    entrance {
        // Intake
        induce clientName = InputProvider("Client name: ");
        induce issue = InputProvider("Primary issue: ");

        // Safety check
        if (!SafetyCheck().isSafe) {
            observe "Client is not suitable for hypnosis";
            return;
        }

        // Individual session
        if (issue == "Anxiety") {
            AnxietyReduction("general", 0.8);
        } else if (issue == "Pain") {
            PainManagement("reduce", "chronic pain");
        } else if (issue == "Habit") {
            induce habit = HabitChange("identify", "Smoking");
            HabitChange("modify", habit, "deep breathing");
        }

        // Follow-up
        observe "Therapeutic session completed";
        observe "Next appointment recommended in one week";
    }
} Relax;
```

### Group Hypnosis

```hyp
Focus {
    entrance {
        // Group alignment
        induce groupSize = InputProvider("Number of participants: ");
        observe "Welcome to the group hypnosis session";

        // Collective trance induction
        HypnoticBreathing(3);
        ProgressiveRelaxation(2);

        // Group suggestion
        HypnoticSuggestion("All of you feel increasingly relaxed", 2);

        // Individual work (simulated)
        for (induce i = 0; i < groupSize; induce i = i + 1) {
            induce individualDepth = TranceDepth();
            observe "Participant " + (i + 1) + " trance depth: " + individualDepth;
        }

        // Group grounding
        Grounding("visual", 45);

        observe "Group session completed successfully";
    }
} Relax;
```

## Safety Guidelines

### Important Safety Considerations

1. **Always run SafetyCheck** before every hypnotic session
2. **Keep an emergency exit ready** with EmergencyExit()
3. **Gentle induction** with HypnoticBreathing and ProgressiveRelaxation
4. **Customize sessions** to the client
5. **Adequate grounding** after every session

### Contraindications

```hyp
// Check contraindications
induce contraindications = CheckContraindications();
if (contraindications.hasPsychosis) {
    observe "WARNING: Psychosis - hypnosis contraindicated";
    return;
}
if (contraindications.hasEpilepsy) {
    observe "CAUTION: Epilepsy - gentle hypnosis only under supervision";
}
```

## Error Handling

Hypnotic functions can throw errors on unexpected reactions:

```hyp
Focus {
    entrance {
        try {
            HypnoticBreathing(5);
            observe "Breathing exercise successful";
        } catch (error) {
            observe "Error during breathing exercise: " + error;
            EmergencyExit("gentle");
        }

        try {
            induce depth = TranceDepth();
            if (depth < 3) {
                observe "Trance too shallow - deepening";
                TranceDeepening(2);
            }
        } catch (error) {
            observe "Error during trance monitoring: " + error;
        }
    }
} Relax;
```

## Next Steps

- [System Functions](./system-functions) - System-specific functions
- [Time & Date Functions](./time-date-functions) - Time and date functions
- [Therapeutic Applications](../examples/therapeutic-examples) - Therapeutic applications

---

**Mastered hypnotic functions? Then explore [System Functions](./system-functions)!** ✅
