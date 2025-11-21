# Keywords Reference

Complete reference of all keywords in HypnoScript based on the Rust implementation.

## Program Structure

| Keyword   | Description                             | Example                         |
| --------- | --------------------------------------- | ------------------------------- |
| `Focus`   | Program start (required)                | `Focus { ... } Relax`           |
| `Relax`   | Program end (required)                  | `Focus { ... } Relax`           |
| `entrance`| Initialization block (optional)         | `entrance { observe "Start"; }` |
| `finale`  | Cleanup/destructor block (optional)     | `finale { observe "End"; }`     |
| `deepFocus`| Extended if block with deeper scope    | `if (x > 5) deepFocus { ... }`  |

## Variable Declarations

| Keyword   | Description                      | Mutability  | Example                              |
| --------- | -------------------------------- | ----------- | ------------------------------------ |
| `induce`  | Standard variable declaration    | Mutable     | `induce x: number = 42;`             |
| `implant` | Alternative variable declaration | Mutable     | `implant y: string = "text";`        |
| `freeze`  | Constant declaration             | Immutable   | `freeze PI: number = 3.14159;`       |
| `anchor`  | Create state snapshot/backup     | Immutable   | `anchor saved = currentValue;`       |
| `from`    | Input source specification       | -           | `induce x: number from external;`    |
| `external`| External input source            | -           | `induce name: string from external;` |

## Control Structures

| Keyword   | Description              | Equivalent | Example                                      |
| --------- | ------------------------ | ---------- | -------------------------------------------- |
| `if`      | Conditional statement    | if         | `if (x > 5) { ... }`                         |
| `else`    | Alternative branch       | else       | `if (x > 5) { ... } else { ... }`            |
| `while`   | While loop               | while      | `while (x > 0) { x = x - 1; }`               |
| `loop`    | For-like loop            | for        | `loop (induce i = 0; i < 10; i = i + 1) { ... }` |
| `snap`    | Break loop               | break      | `while (true) { snap; }`                     |
| `sink`    | Continue to next iteration| continue  | `while (x < 10) { sink; }`                   |
| `sinkTo`  | Goto (jump to label)     | goto       | `sinkTo myLabel;`                            |
| `oscillate`| Toggle boolean variable | -          | `oscillate isActive;`                        |

**Note:** `break` and `continue` are also accepted as synonyms for `snap` and `sink`.

## Functions

| Keyword            | Description                     | Example                                                |
| ------------------ | ------------------------------- | ------------------------------------------------------ |
| `suggestion`       | Function declaration            | `suggestion add(a: number, b: number): number { ... }` |
| `trigger`          | Event handler/callback function | `trigger onClick = suggestion() { ... };`              |
| `imperativeSuggestion` | Imperative function (modifier) | `imperativeSuggestion doSomething() { ... }`      |
| `dominantSuggestion`   | Static function (modifier)     | `dominantSuggestion helperFunc() { ... }`          |
| `awaken`           | Return statement                | `awaken x + y;`                                        |
| `call`             | Explicit function call          | `call myFunction();`                                   |

**Note:** `return` is also accepted as a synonym for `awaken`.

## Object-Orientation

### Sessions (Classes)

| Keyword       | Description          | Example                                        |
| ------------- | -------------------- | ---------------------------------------------- |
| `session`     | Class declaration    | `session Person { ... }`                       |
| `constructor` | Constructor method   | `suggestion constructor(name: string) { ... }` |
| `expose`      | Public visibility    | `expose name: string;`                         |
| `conceal`     | Private visibility   | `conceal age: number;`                         |
| `dominant`    | Static member        | `dominant counter: number = 0;`                |

### Structures

| Keyword     | Description               | Example                                     |
| ----------- | ------------------------- | ------------------------------------------- |
| `tranceify` | Record/struct declaration | `tranceify Point { x: number; y: number; }` |

## Input/Output

| Keyword   | Description          | Behavior                    | Example                             |
| --------- | -------------------- | --------------------------- | ----------------------------------- |
| `observe` | Standard output      | With newline                | `observe "Hello World";`            |
| `whisper` | Output without newline| Without newline            | `whisper "Part1"; whisper "Part2";` |
| `command` | Imperative output    | Uppercase, with newline     | `command "Important!";`             |
| `drift`   | Pause/sleep          | Delay in ms                 | `drift(2000);`                      |

## Modules and Globals

| Keyword        | Description       | Example                                   |
| -------------- | ----------------- | ----------------------------------------- |
| `mindLink`     | Import/include    | `mindLink "utilities.hyp";`               |
| `sharedTrance` | Global variable   | `sharedTrance config: string = "global";` |
| `label`        | Label declaration | `label myLabel;`                          |

## Data Types

| Keyword   | Description       | Example                          |
| --------- | ----------------- | -------------------------------- |
| `number`  | Numeric type      | `induce x: number = 42;`         |
| `string`  | String type       | `induce text: string = "hello";` |
| `boolean` | Boolean type      | `induce flag: boolean = true;`   |
| `trance`  | Special trance type| `induce state: trance;`         |

## Literals

| Keyword | Description      | Example                             |
| ------- | ---------------- | ----------------------------------- |
| `true`  | Boolean literal  | `induce isActive: boolean = true;`  |
| `false` | Boolean literal  | `induce isActive: boolean = false;` |

## Testing/Debugging

| Keyword  | Description | Example         |
| -------- | ----------- | --------------- |
| `assert` | Assertion   | `assert x > 0;` |

## Hypnotic Operators

### Comparison Operators

| Hypnotic                  | Standard | Meaning         |
| ------------------------- | -------- | --------------- |
| `youAreFeelingVerySleepy` | `==`     | Equal           |
| `youCannotResist`         | `!=`     | Not equal       |
| `lookAtTheWatch`          | `>`      | Greater         |
| `fallUnderMySpell`        | `<`      | Less            |
| `yourEyesAreGettingHeavy` | `>=`     | Greater-or-equal|
| `goingDeeper`             | `<=`     | Less-or-equal   |

### Legacy Operators (deprecated but supported)

| Hypnotic        | Standard | Note                               |
| --------------- | -------- | ---------------------------------- |
| `notSoDeep`     | `!=`     | Use `youCannotResist`              |
| `deeplyGreater` | `>=`     | Use `yourEyesAreGettingHeavy`      |
| `deeplyLess`    | `<=`     | Use `goingDeeper`                  |

### Logical Operators

| Hypnotic             | Standard | Meaning     |
| -------------------- | -------- | ----------- |
| `underMyControl`     | `&&`     | Logical AND |
| `resistanceIsFutile` | `\|\|`   | Logical OR  |

## Usage Notes

### Case-Insensitivity

All keywords are **case-insensitive** during lexing but are normalized to their canonical form:

```hyp
// All of the following are equivalent:
Focus { ... } Relax
focus { ... } relax
FOCUS { ... } RELAX
```

### Standard Synonyms

For better readability, HypnoScript supports standard synonyms:

- `return` → `awaken`
- `break` → `snap`
- `continue` → `sink`

### Recommendations

1. **Use canonical forms** for better readability
2. **Use hypnotic operators** for thematic consistency
3. **Avoid legacy operators** (`notSoDeep`, `deeplyGreater`, `deeplyLess`)
4. **Prefer `induce`** over `implant` for standard variables
5. **Use `freeze`** for immutable values instead of `induce`
