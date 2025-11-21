---
layout: home

hero:
  name: 'HypnoScript'
  text: 'The hypnotic programming language'
  tagline: Modern scripting with hypnotic syntax and a solid Rust foundation
  image:
    src: /img/logo.svg
    alt: HypnoScript Logo
  actions:
    - theme: brand
      text: Quick Start
      link: /getting-started/quick-start
    - theme: alt
      text: Documentation
      link: /intro
    - theme: alt
      text: GitHub
      link: https://github.com/Kink-Development-Group/hyp-runtime

features:
  - icon: 🎯
    title: Hypnotic Syntax
    details: Keywords like Focus, Relax, induce, observe or deepFocus bring hypnotic metaphors directly into your code.

  - icon: 🦀
    title: Fully Implemented in Rust
    details: Lexer, Parser, static type checker, interpreter and WASM codegen run natively on Windows, macOS and Linux.

  - icon: 🧠
    title: Static Type System
    details: The type checker understands numbers, strings, booleans, arrays, functions and sessions including visibility modifiers.

  - icon: 📦
    title: Standard Library Included
    details: Math, strings, arrays, files, statistics, system information, time & date as well as validation functions are available out of the box.

  - icon: 🛠️
    title: Lean CLI
    details: A single binary provides run, lex, parse, check, compile-wasm, builtins and version – that's all you need.

  - icon: 🧩
    title: Sessions with Visibility
    details: Define sessions with `expose`/`conceal`, constructors and static (`dominant`) members.

  - icon: 🌐
    title: WebAssembly Export
    details: Optionally generate WebAssembly text files (.wat) and use HypnoScript in the browser.
---

## Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime

# Build HypnoScript CLI in release mode
cargo build -p hypnoscript-cli --release

# Optionally install globally (binary is called hypnoscript)
cargo install --path hypnoscript-cli
```

Pre-built artifacts (Windows, macOS, Linux) can also be found in the `release/` folder and under [GitHub Releases](https://github.com/Kink-Development-Group/hyp-runtime/releases).

### Your First HypnoScript Program

```hyp
Focus {
    entrance {
        observe "Welcome to HypnoScript!";
    }

    induce name: string = "Developer";
    observe "Hello, " + name + "!";

    induce numbers: number[] = [1, 2, 3, 4, 5];
    induce sum: number = ArraySum(numbers);
    observe "Sum: " + ToString(sum);

    if (sum lookAtTheWatch 10) deepFocus {
      observe "The memory is now becoming more intense.";
    }
}
```

### Running

```bash
hypnoscript run my_script.hyp
```

## Why HypnoScript?

HypnoScript combines the elegance of modern programming languages with a unique, hypnotically inspired syntax. The Rust implementation provides you with:

- **🎯 Unique Syntax** – Focus/Relax blocks, hypnotic operators like `youAreFeelingVerySleepy` (`==`) or `underMyControl` (`&&`).
- **🦾 Rust Performance** – No external runtime dependencies, fast binaries and optional WASM export.
- **🔒 Static Safety** – The type checker validates variables, functions, sessions as well as accesses to static and private members.
- **🧰 Standard Library** – Math, strings, arrays, files, statistics, validation, system and time functions are directly integrated.
- **🧪 Development Workflow** – The CLI supports lexing, parsing, type checking and program execution in the same tool.
- **📄 Examples & Tests** – `.hyp` examples and regression tests in the repository demonstrate real language features.

## Community & Support

- **GitHub**: [Kink-Development-Group/hyp-runtime](https://github.com/Kink-Development-Group/hyp-runtime)
- **Documentation**: This site
- **Issues**: [GitHub Issues](https://github.com/Kink-Development-Group/hyp-runtime/issues)
- **Community Updates**: Follow the progress in the [GitHub Repository](https://github.com/Kink-Development-Group/hyp-runtime)

## License

HypnoScript is open source and available under the MIT License.
