# Announcing Layer36

**Status:** Draft
**Target:** Week 4 of Phase 0

Layer36 is an experiment in making native application development portable again.
The goal is direct: write an app once, package it once, and run it natively on
the devices people actually use.

The platform is built around WebAssembly and the Component Model. Applications
compile to a portable component, call a Universal API defined in WIT, and run
inside a host runtime that enforces capability-based permissions before mapping
those calls onto native operating-system APIs.

This is not a new kernel and it is not an emulator. It is a meta-platform: a
runtime, standard library, permission model, bundle format, and distribution
layer that sit above existing operating systems.

Phase 0 is quiet on purpose. We are setting up the repository, documentation,
CI, contribution path, licensing, and first architecture decision before writing
runtime code. A platform that wants trust has to practice it in the repo before
it asks for it at runtime.

The next milestone is Phase 1: a proof-of-concept runtime that executes the same
small WebAssembly component on Linux, macOS, and Windows.

Follow the roadmap in the docs, read the build plan, and open an issue if there
is a small piece you want to help with.
