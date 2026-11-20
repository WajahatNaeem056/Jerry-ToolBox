# Duck-ToolBox (RKP device only)

Trimmed fork of Duck-ToolBox for **Jerry Manager**: it keeps only what the
"Generate RKP Keybox" screen needs and builds a single `duckd` binary that
exposes `duckd rkp-device status | provision`.

## What is in here

| Crate | Why it is kept |
| --- | --- |
| `duckd` | The binary. Registers only the `rkp-device` feature. |
| `duck-rkp-device` | Asks this device's KeyMint for the RKP CSR and builds the keybox. |
| `duck-rkp` | CBOR/COSE/HTTP and keybox XML code that `duck-rkp-device` is built on. |
| `duck-core`, `duck-platform` | Shared runtime (paths, JSON envelope, props, root, SoC). |

Everything else (Tricky Store, Play Integrity, device faker, zram, kernel modules,
linux-env, module/ and ui/ folders, release workflows) was removed.

## What changed for Jerry Manager

* **No module root needed.** `duckd` used to fail with `... is not a valid Duck ToolBox
  module root` unless `DUCK_TOOLBOX_ROOT` held a `module.prop`. The root is now
  `DUCK_TOOLBOX_ROOT` as given, else a real module root, else `DUCK_TOOLBOX_DATA_ROOT`.
* **Only the keybox is written.** `rkp-device provision --output <file>` writes that one
  file. The `var/outputs/<run>/` directory (CSR, certificate chain, separate private key)
  is only produced when no `--output` is given.
* **No log file** unless `DUCK_TOOLBOX_LOG=1`.
* **Scratch space follows `TMPDIR`** instead of `<data>/var/tmp`.

A normal Jerry run therefore leaves `rkp_device/keybox.xml` and nothing else.
Tests: `crates/duckd/tests/jerry_root.rs`, `service_tests.rs`.

## Build

GitHub: run the **Build duckd (rkp-device only)** workflow, then put the
`duckd` artifact at `Jerry/dex/duckd` in Jerry.

Local (needs Rust, the `aarch64-linux-android` target, Android NDK and SDK
build-tools for the V1 bridge): see `.github/workflows/rkp-device-only.yml`.

    cargo test --locked --workspace
