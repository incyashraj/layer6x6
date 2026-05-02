# Vision

## The first-principles insight

Every scalable computing abstraction has solved fragmentation the same way:
insert a universal intermediate layer.

| Fragmentation | Intermediate layer | Result |
|---|---|---|
| Multiple CPUs | LLVM IR | Any backend |
| Multiple OSes (server) | JVM bytecode / .NET CLR | Any OS |
| Multiple devices (web) | HTML / JS / CSS | Any browser |

Each time, the N² problem became 2N.

Layer36 applies the same transformation to native apps: one portable bytecode,
one standard library, one permission model at the center; a thin adapter per
host on the outside.

## Why now

Four forces are converging in 2026:

1. **WebAssembly is production-ready.** The bytecode is stable. The Component
   Model has shipped. WASI Preview 2 exists. Tooling for Rust, Go, C++, JS, and
   Python is mature.

2. **Device fragmentation is worse than ever.** Laptops (x86/ARM), phones
   (iOS/Android), tablets, watches, cars, TVs, XR headsets — each with a
   different SDK. No dev wants to ship seven codebases.

3. **Native cross-platform solutions are incomplete.** Flutter is UI-only.
   React Native is JS-only. Electron is bloated. None deliver true native
   capability + performance + cross-platform in one package.

4. **Hardware converges on ARM.** When every device runs the same ISA family,
   the primary reason to target different CPU backends evaporates. Only the OS
   layer differs, which is exactly what we abstract.

## Our wedge

We do not compete with Flutter, React Native, or Electron. We **complete
WASM + WASI** for native app scenarios.

Specifically:

1. We ship the UI, GPU, and hardware UAPIs that WASI doesn't have yet.
2. We ship a productized runtime + SDK developers install in five minutes.
3. We have an anchor tenant (ParkSure) that forces us to dogfood
   production-quality from day one.

## Success criteria at v1.0

At v1.0 (end of month 24), Layer36 must be able to:

| # | Criterion | Target |
|---|-----------|--------|
| 1 | Run the same `.l36app` binary on | Windows 11+, macOS 13+, Ubuntu 22.04+, iOS 16+, Android 12+, browsers |
| 2 | Hello-world startup | < 100 ms cold, < 20 ms warm |
| 3 | GUI frame budget | 16.7 ms (60 fps) on M1 / Snapdragon 8 Gen 2 |
| 4 | Binary size overhead vs native | < 3× for a typical productivity app |
| 5 | Source languages | Rust, Go, TypeScript (first-class); C/C++, Python, Swift (compatible) |
| 6 | Anchor tenant | ParkSure migrated end-to-end |
| 7 | Developer docs | 100% UAPI coverage with examples |
| 8 | CI pass | Nightly green on all target hosts for ≥ 7 consecutive days |
