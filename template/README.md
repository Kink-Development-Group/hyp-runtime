# HypnoScript CLI Template

This directory ships a complete starter kit for a HypnoScript-powered CLI application. It includes a clean folder structure, a themed manifest (`trance.json`), and initial scripts that showcase the default commands (`help`, `status`, `pulse`).

> **Heads-up:** The files under `src/commands/` already contain `MindLink` statements that will be used once the module loader is available. For now, all logic is wired through `src/main.hyp`; as soon as imports are enabled you can connect those modules directly.

## Directory layout

```text
template/
├── README.md
├── trance.json
├── scripts/
│   ├── watch.ps1
│   └── watch.sh
├── src/
│   ├── main.hyp
│   ├── commands/
│   │   ├── help.hyp
│   │   ├── pulse_report.hyp
│   │   └── session_status.hyp
│   └── shared/
│       └── runtime_state.hyp
└── tests/
    └── smoke.hyp
```

## `trance.json` manifest

The manifest mirrors `package.json`, but embraces HypnoScript terminology. It will become the contract for the future HypnoScript package manager.

| Field         | Type   | Meaning                                                               |
| ------------- | ------ | --------------------------------------------------------------------- |
| `ritualName`  | string | Package / project name.                                               |
| `mantra`      | string | Semantic version (for example `0.1.0`).                               |
| `intent`      | string | Project type (`cli`, `service`, `library`, …).                        |
| `induction`   | object | Metadata such as `description`, `entryScript`, `keywords`, `license`. |
| `hypnotists`  | array  | Maintainers (`name`, `role`, `contact`).                              |
| `auras`       | object | Repository, homepage, and documentation links.                        |
| `suggestions` | object | Script hooks similar to npm "scripts".                                |
| `anchors`     | object | Runtime dependencies (`name: versionRange`).                          |
| `deepAnchors` | object | Dev / test dependencies.                                              |
| `channels`    | object | CLI metadata (`binary`, `entry`, `targets`, telemetry settings).      |
| `triggers`    | object | Lifecycle hooks (for example `preFocus`, `postRelax`).                |

### Manifest workflow

1. Update `ritualName`, `mantra`, and `induction.description` for your project.
2. Add maintainers to `hypnotists` and point `auras` to the right URLs.
3. Extend `suggestions` with your build / test commands.
4. Keep `anchors` and `deepAnchors` in sync with your actual dependencies.
5. Use `channels.targets` to describe the platforms you intend to ship.

## Included scripts

| File                              | Purpose                                                               |
| --------------------------------- | --------------------------------------------------------------------- |
| `src/main.hyp`                    | Entry point that reads CLI input and dispatches commands.             |
| `src/commands/help.hyp`           | Full help page plus command listing.                                  |
| `src/commands/session_status.hyp` | Simulates the state of an active hypnosis session ("status").         |
| `src/commands/pulse_report.hyp`   | Emits health metrics for the runtime ("pulse").                       |
| `src/shared/runtime_state.hyp`    | Shared `tranceify` structures and helper suggestions.                 |
| `tests/smoke.hyp`                 | Minimal smoke test for the shared building blocks.                    |
| `scripts/watch.ps1/.sh`           | Convenience wrappers for Windows, Linux, and macOS development loops. |

## Quickstart

1. Copy the contents of `template/` into a new project folder.
2. Customize `trance.json` to match your scenario.
3. Use the wrapper scripts to run commands:

```pwsh
# Windows / PowerShell
pwsh scripts/watch.ps1 help
pwsh scripts/watch.ps1 status
pwsh scripts/watch.ps1 pulse
```

```bash
# Linux / macOS
./scripts/watch.sh help
./scripts/watch.sh status
./scripts/watch.sh pulse
```

1. Prefer environment variables when you want to call the CLI without wrappers:

```pwsh
$env:HYPNO_COMMAND = "help"
$env:HYPNO_PAYLOAD = ""
pwsh scripts/watch.ps1
```

Or call HypnoScript directly:

```pwsh
hypnoscript run src/main.hyp -- help
```

## Next steps

- Add more commands under `src/commands/` and register them inside `DispatchCommand` (`src/main.hyp`).
- Map additional lifecycle hooks from the manifest to HypnoScript scripts or shell commands as needed.
- Create more tests under `tests/` and wire them into `suggestions.test` when the runner is connected.

Treat `trance.json` as a contract for upcoming automation. Keeping it tidy now will make future tooling effortless.
