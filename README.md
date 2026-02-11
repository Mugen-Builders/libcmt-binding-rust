<br>
<p align="center">
    <img src="https://github.com/user-attachments/assets/080bb0be-060c-4813-85b4-6d9bf25af01f" align="center" width="20%">
</p>
<br>
<div align="center">
	<i>Cartesi Rollups LIBCMT Binding for Rust</i>
</div>
<div align="center">
	<!-- <b>Any Code. Ethereum's Security.</b> -->
</div>
<br>
<p align="center">
	<img src="https://img.shields.io/github/license/Mugen-Builders/libcmt-binding-rust?style=default&logo=opensourceinitiative&logoColor=white&color=008DA5" alt="license">
	<img src="https://img.shields.io/github/last-commit/Mugen-Builders/libcmt-binding-rust?style=default&logo=git&logoColor=white&color=000000" alt="last-commit">
</p>

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Requirements](#requirements)
- [Usage](#usage)
- [Building and Running the Sample application](#Building-and-running-the-Sample-echo-app)
- [Testing](#testing)
- [License](#license)

## Overview

This repository provides **Rust bindings** for the [Cartesi machine guest library (libcmt)](https://github.com/cartesi/machine-guest-tools/tree/main/sys-utils/libcmt). The bindings expose libcmt’s C API (rollup I/O, ABI, Merkle trees, buffers, etc.) for utilization in rust applications. This would serve as an alternative to the HttpServer and offer methods to manage a Cartesi application instance.

The repo includes:

- **Library**: `libcmt-binding-rust` — the main crate (FFI bindings and Rust wrappers and helpers).
- **Sample apps**: `echo_app` (handles asset deposit and voucher emission) and `app_template` (minimal starter).
- **Cartesi config**: `cartesi.echoApp.toml` and `build/Dockerfile.echoApp` for building and running the echo app in the Cartesi machine.
- **Test Instance**: `/tests` Utilizes cartesapp to test the sample application which uses the libcmt-rust-bindings.

## Project Structure

```
libcmt-binding-rust/
├── src/                    # Library source (bindings + Rust API)
│   ├── lib.rs
│   ├── abi.rs
│   ├── buf.rs
│   ├── io.rs
│   ├── keccak.rs
│   ├── merkle.rs
│   ├── rollup.rs
│   └── util.rs
├── build.rs                # bindgen build script (wrapper.h → bindings)
├── wrapper.h               # C header include for bindgen
├── Cargo.toml              # Library crate
├── cartesi.echoApp.toml    # Cartesi config for echo app
├── build/
│   └── Dockerfile.echoApp   # Multi-stage Dockerfile (echo app)
├── sample_apps/
│   ├── echo_app/           # Echo dApp (emits a voucher for all asset deposit)
│   │   ├── src/main.rs
│   │   ├── Cargo.toml
│   │   ├── cartesi.toml
│   │   └── tests/
│   └── app_template/       # Minimal app template
├── tests/                  # Integration tests (Python, cartesapp)
│   ├── model.py
│   └── test_echo.py
└── third_party/
    └── machine-guest-tools # Cartesi libcmt (submodule)
```

## Requirements

- **Rust** (stable) — to build the library and sample apps.
- **Clang** / **libclang-dev** — for `bindgen` (used in `build.rs`).
- **Git** — managing `third_party/machine-guest-tools` (Submodule containing C headers and libcmt).
- **Docker** — for building the RISC-V image and running the Cartesi machine.
- **Python 3.12+** — for the test suite (cartesapp).

## Usage

### Add the binding to your Cartesi project

Modify your projects `Cargo.toml` file to include:

```toml
[dependencies]
libcmt-binding-rust = { git = "https://github.com/Mugen-Builders/libcmt-binding-rust.git", tag = "v0.1.0" }
```

Then import and use the rollup API in your Rust code.

### Code Snippets

The following snippets show how to use the libcmt Rust bindings. A more detailed use can be found on the [echo app](sample_apps/echo_app/src/main.rs).

**Creating a rollup and main request loop:**

```rust
use libcmt_binding_rust::rollup::{Rollup, Advance, Inspect};
use libcmt_binding_rust::cmt_rollup_finish_t;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut accept_previous_request = true;
    let mut rollup = Rollup::new()?;

    loop {
        let mut finish = cmt_rollup_finish_t {
            accept_previous_request,
            next_request_type: 0,
            next_request_payload_length: 0,
        };
        rollup.finish(&mut finish)?;

        accept_previous_request = match finish.next_request_type {
            0 => handle_advance(&mut rollup).await?,
            1 => handle_inspect(&mut rollup).await?,
            _ => false,
        };
    }
}
```

**Handling an advance request (read input, optionally emit voucher/notice):**

```rust
async fn handle_advance(rollup: &mut Rollup) -> Result<bool, Box<dyn std::error::Error>> {
    let advance = rollup.read_advance_state()?;
    let payload = advance.payload;           // hex string, e.g. "0x..."
    let msg_sender = advance.msg_sender;    // 20-byte address (hex)
    let app_contract = advance.app_contract;

    rollup.emit_notice(&payload)?;          // emit notice (payload in hex)
    // or
    rollup.emit_voucher(&target_address, Some("0x0"), &calldata_hex)?;

    Ok(true)  // accept this advance
}
```

**Handling an inspect request (read-only query):**

```rust
async fn handle_inspect(rollup: &mut Rollup) -> Result<bool, Box<dyn std::error::Error>> {
    let inspect = rollup.read_inspect_state()?;
    let payload = inspect.payload;  // hex string

    // Optional: emit a report (only output allowed for inspect)
    rollup.emit_report(&payload)?;

    Ok(true)
}
```

**Emitting a notice** (data that can be validated on-chain):

```rust
// payload_hex: hex-encoded bytes, e.g. "0x48656c6c6f"
rollup.emit_notice("0x48656c6c6f")?;
```

**Emitting a voucher** (transaction to be executed on L1):

```rust
// address_hex: 20-byte destination (40 hex chars), e.g. "0x..."
// value_hex: optional ETH value, e.g. Some("0xde0b6b3a7640000") or None
// payload_hex: calldata (hex)
rollup.emit_voucher(
    "0xc70076a466789B595b50959cdc261227F0D70051",
    Some("0xde0b6b3a7640000"),
    "0x...",
)?;
```

**Emitting a report** (log; not validated on-chain):

```rust
rollup.emit_report("0x7265706f7274")?;
```

## Building and running the Sample echo app

Config is in `cartesi.echoApp.toml`; the root filesystem is built from `build/Dockerfile.echoApp`.

**Build the machine image (RISC-V):**

```bash
cartesi build -c "./cartesi.echoApp.toml"
```

**Run the Cartesi machine:**

```bash
cartesi run
```

This starts the rollup node with the echo app; you can then send deposits, advances and inspect requests to the application.

## Testing

Tests are written in Python using [cartesapp](https://github.com/prototyp3-dev/cartesapp) and run inside the Cartesi machine.

**Prerequisites:** Python 3.12+, virtualenv with cartesapp installed (see [Installation](#5-python-test-suite-optional-for-testing)).

**Run tests (with Cartesi machine emulator):**

```bash
. .venv/bin/activate
pip3 install cartesapp[dev]@git+https://github.com/prototyp3-dev/cartesapp@v1.2.1
cartesapp test --config-file "./cartesi.echoApp.toml" --log-level debug --cartesi-machine
```

This builds, starts the machine, and runs the test client.

**Run tests (pytest only, if you have a running node):**

Test modules live in `tests/` (e.g. `test_echo.py`, `model.py`) and in `sample_apps/echo_app/tests/`.

## License

See [LICENSE](LICENSE).
