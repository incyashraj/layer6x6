# Introduction

**Layer36** is a universal application platform: a portable runtime, a universal
standard library (UAPI), a capability-based permission system (UCap), and a
package format that together let developers write an app **once** and have it
run natively — with access to each platform's hardware and performance — on
Windows, macOS, Linux, iOS, Android, and the web.

## What problem does it solve?

An application today is coupled to six independent things: its CPU architecture,
its kernel's syscall table, its system libraries, its framework APIs, its bundle
format, and its security model. Supporting N operating systems requires N
implementations of each layer — an O(N²) cost that compounds with every new
device category.

The result: Android apps don't run on macOS, iOS apps don't run on Android,
native desktop apps don't run on phones. Users pay for it in device lock-in.
Developers pay for it in duplicated work.

## How does Layer36 solve it?

By inserting a **universal intermediate layer** — the same architectural move
that solved fragmentation every previous time:

- Multiple CPUs → LLVM IR → any backend
- Multiple OSes (server) → JVM / .NET CLR → any OS
- Multiple devices (web) → HTML/JS/CSS → any browser

Layer36 applies the same transformation to native apps: **one portable bytecode,
one standard library, one permission model** at the center; a thin adapter per
host on the outside.

## What is Layer36 built on?

- **WebAssembly + Component Model** — the portable bytecode
- **Wasmtime** — the embedded WASM runtime engine
- **WIT (WebAssembly Interface Types)** — the interface definition language for UAPI
- **Rust** — the runtime implementation language

## Status

**Pre-alpha. Phase 0 — Foundation.** See the [roadmap](roadmap.md) for
the full 24-month plan.
