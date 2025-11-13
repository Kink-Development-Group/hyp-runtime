---
title: Hello World
sidebar_position: 3
---

# Hello World

Dein erstes HypnoScript-Programm!

## Einfaches Hello World

Erstelle eine Datei `hello.hyp` mit folgendem Inhalt:

```hyp
Focus {
    observe "Hallo Welt!";
} Relax
```

Führe das Programm aus:

```bash
hypnoscript hello.hyp
```

Ausgabe:

```
Hallo Welt!
```

## Mit Entrance-Block

Der `entrance`-Block wird beim Programmstart ausgeführt:

```hyp
Focus {
    entrance {
        observe "Willkommen in HypnoScript!";
        observe "Dies ist dein erstes Programm.";
    }
} Relax
```

## Mit Variablen

```hyp
Focus {
    entrance {
        induce name: string = "Entwickler";
        observe "Hallo, " + name + "!";
        observe "Willkommen bei HypnoScript.";
    }
} Relax
```

## Interaktives Hello World

```hyp
Focus {
    entrance {
        observe "=== HypnoScript Willkommens-Programm ===";

        induce name: string = "Welt";
        induce version: number = 1.0;

        observe "Hallo, " + name + "!";
        observe "HypnoScript Version " + version;
        observe "Bereit für hypnotische Programmierung!";
    }
} Relax
```

## Mit Funktionen

```hyp
Focus {
    suggestion greet(name: string) {
        observe "Hallo, " + name + "!";
        observe "Schön, dich kennenzulernen.";
    }

    entrance {
        greet("HypnoScript-Entwickler");
    }
} Relax
```

## Nächste Schritte

- Lerne über [Variablen und Datentypen](../language-reference/variables.md)
- Verstehe [Kontrollstrukturen](../language-reference/control-flow.md)
- Entdecke [Builtin-Funktionen](../builtins/overview.md)
