# Core Concepts

Every concept here has a one-sentence canonical definition. Use them in every
RFC, ADR, and PR description.

| Concept | Definition |
|---------|-----------|
| **UIR** (Universal IR) | The portable bytecode every Layer36 app compiles to. Base: WebAssembly Core + Component Model. |
| **UAPI** (Universal API) | The standard library every Layer36 app calls. Defined as WIT interfaces; implemented per host by the runtime. |
| **UCap** (Universal Capabilities) | The capability-based permission model. Apps declare required capabilities; the runtime issues unforgeable handles only to granted capabilities. |
| **Runtime** | The binary installed on each host OS that loads and executes `.l36app` bundles. |
| **Host Adapter** | The per-OS module inside the runtime that translates UAPI calls into native OS calls. |
| **App Bundle** | The `.l36app` distributable package: a zip-structured container holding WASM component, manifest, assets, and signature. |
| **Marketplace** | Distribution channel and user identity layer. |

For full definitions see [`Plan/Build-Plan.md §3`](https://github.com/incyashraj/layer6x6/blob/main/Plan/Build-Plan.md#3-core-concepts).
