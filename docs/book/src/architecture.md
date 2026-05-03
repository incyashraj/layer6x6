# Architecture

Layer36 has one job: keep app code above the platform line.

An app should not need to know whether it is running on Linux, Windows, macOS,
Android, or iOS for common work. It should call Layer36. The host adapter should
do the platform work.

## Full Shape Of The System

```mermaid
flowchart LR
    SRC["App source<br/>Rust, Go, TS, C, etc."]
    WASM["WASM component<br/>portable app code"]
    MAN["Manifest<br/>name, version, permissions"]
    BUNDLE[".l36app bundle<br/>component, assets, signature"]
    RT["Layer36 runtime"]
    UAPI["UAPI<br/>files, net, UI, sensors"]
    UCAP["UCap<br/>permission grants"]
    ADAPT["Host adapter"]
    OS["Native OS"]
    HW["Hardware"]

    SRC --> WASM
    WASM --> BUNDLE
    MAN --> BUNDLE
    BUNDLE --> RT
    RT --> UAPI
    RT --> UCAP
    UAPI --> ADAPT
    UCAP --> ADAPT
    ADAPT --> OS
    OS --> HW

    classDef done fill:#d9fbe3,stroke:#16833a,color:#102a17,stroke-width:2px;
    classDef current fill:#fff3bf,stroke:#b7791f,color:#2d2100,stroke-width:2px;
    classDef pending fill:#eeeeee,stroke:#999999,color:#777777,stroke-width:1px;

    class SRC,WASM,RT current;
    class MAN,BUNDLE,UAPI,UCAP,ADAPT,OS,HW pending;
```

Phase 1 has only the yellow part: a WASM component, the runtime, and a temporary
host interface. The real UAPI, app bundle, permissions, and host adapters start
in later phases.

## Runtime Flow Today

```mermaid
sequenceDiagram
    participant User
    participant CLI as layer36 CLI
    participant Runtime
    participant Wasmtime
    participant App as WASM component

    User->>CLI: layer36 run hello.wasm
    CLI->>Runtime: run_file(path, config)
    Runtime->>Wasmtime: load component
    Runtime->>Wasmtime: link print and exit
    Wasmtime->>App: call run()
    App->>Wasmtime: print("Hello, Layer36!")
    Wasmtime->>Runtime: host print call
    Runtime->>CLI: stdout and exit code
```

## What Phase 1 Proves

Phase 1 proves that the loader works. The CI pipeline builds one hello-world
component, stores its SHA-256 hash, and runs those exact bytes on:

- Linux
- macOS
- Windows

That matters because the promise is not """three hosts can build similar source."""
The promise is """one app artifact can run on different hosts."""

## Crates Today

```mermaid
flowchart TD
    CLI["crates/cli"]
    RT["crates/runtime"]
    WIT["wit/layer36"]
    TEST["test/integration"]

    CLI --> RT
    RT --> WIT
    TEST --> WIT
```

`crates/runtime` owns loading, Wasmtime setup, host imports, fuel, memory
limits, and runtime errors.

`crates/cli` owns arguments, output, exit codes, and developer diagnostics.

## Trust Boundary

The WASM component is untrusted. The runtime and host imports are trusted
project code. The operating system is outside the project boundary.

Phase 1 is not a hardened security sandbox. It avoids filesystem, network, env,
and process access, but it is still a developer proof. Real permission work
starts with UCap in Phase 2.

## Later Phases

Phase 2 replaces the temporary Phase 1 WIT file with real UAPI modules. Phase 3
adds desktop UI and graphics. Phase 4 adds mobile hosts. Later phases add the
SDK, app bundles, signing, identity, updates, and release hardening.
