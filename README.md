# Perception Kernel

**Perception Kernel** is an experimental research fork of the **seL4 microkernel**, developed by the **Deepcomet Organization**. It is a dedicated sandbox for exploring AI‑native microkernel concepts, capability system evolution, and hybrid verification techniques. Perception is intentionally experimental: features are prototyped here, evaluated rigorously, and only the most promising ideas are migrated into the production **Zenith Kernel**.

---

## Overview

**Perception Kernel** provides a compact, verifiable microkernel base for rapid research and reproducible experiments. It preserves seL4’s minimal trusted computing base while enabling controlled deviations for:
- **AI‑aware scheduling and resource management**.
- **Dynamic capability models** for distributed agents.
- **Rust and safe language integration** alongside verified C proofs.
- **Hardware acceleration orchestration** for NPUs and GPUs.

This repository is organized to keep experimental work isolated from verified baselines and to make it straightforward to reproduce results, run verification tasks, and iterate on designs.

---

## Vision

- Build a research platform where **formal methods** and **practical AI systems** meet.
- Enable experiments that push microkernel design toward **agent‑native operating systems**.
- Maintain a clear path for **proven research** to be integrated into Zenith.
- Keep the codebase small and auditable so verification and reproducibility remain tractable.

---

## Goals

**Primary goals**
- Provide a reproducible environment for microkernel research.
- Maintain compatibility with seL4 verification artifacts where feasible.
- Rapidly prototype and evaluate AI‑centric kernel mechanisms.

**Secondary goals**
- Demonstrate safe Rust integration patterns for microkernel components.
- Explore capability delegation and revocation models suited for distributed agents.
- Produce benchmark suites for microkernel performance on ML workloads.

---

## Design Principles

- **Minimal trusted computing base**: Keep kernel code minimal and auditable.
- **Separation of concerns**: Experimental features live in `feature/*` branches and are clearly labeled as non‑verified.
- **Proof first where possible**: Preserve seL4 proof artifacts; when experiments break proofs, document the divergence and provide a verification plan.
- **Reproducibility**: CI and test harnesses must reproduce builds, tests, and verification steps deterministically.
- **Incremental migration**: Only migrate features into Zenith after performance, safety, and verification criteria are met.

---

## Supported Architectures

Perception follows seL4’s focused architecture support to keep verification tractable and maintenance manageable.

- **RISC‑V** — research friendly and open ISA.
- **ARM** — dominant in embedded and mobile.
- **x86** — mainstream desktop and server.

---

## Key Research Areas

#### AI Aware Scheduling
- **Goal**: Prioritize and schedule ML/NPU workloads with latency and throughput constraints.
- **Approach**: Extend scheduler metadata to include workload hints, QoS tokens, and energy budgets.
- **Deliverables**: Prototype scheduler, microbenchmarks, and evaluation reports.

#### Capability System Evolution
- **Goal**: Support dynamic capability delegation for distributed agents and multi‑tenant AI services.
- **Approach**: Prototype revocation semantics, capability provenance tracking, and delegation policies.
- **Deliverables**: Capability extension RFCs, test suites, and formal models.

#### IPC and Communication Models
- **Goal**: Experiment with richer IPC semantics for agent communication and trust metadata.
- **Approach**: Add optional message metadata fields, priority lanes, and secure channel primitives.
- **Deliverables**: IPC extensions, compatibility shims, and performance analysis.

#### Rust Integration and Safety
- **Goal**: Integrate Rust components safely while preserving verifiability of critical paths.
- **Approach**: Define FFI boundaries, memory safety wrappers, and verification strategies for mixed language modules.
- **Deliverables**: Rust module examples, safety guidelines, and integration tests.

#### Hardware Acceleration Orchestration
- **Goal**: Coordinate NPUs and GPUs from microkernel space with predictable isolation.
- **Approach**: Prototype device capability models, scheduling hooks, and secure driver patterns.
- **Deliverables**: Device orchestration prototypes and benchmark results.

---

## Tri Kernel Ecosystem Comparison

| Kernel | Supported Architectures | Purpose | Verification Focus |
|--------|-------------------------|---------|--------------------|
| **Zenith** | RISC‑V; ARM; x86 | Production microkernel; flagship | High; production proofs |
| **Perception** | RISC‑V; ARM; x86 | Research fork of seL4; experimental features | Mixed; baseline proofs retained, experiments non‑verified |
| **Mono‑Zenith** | x86‑64; ARM64 | Monolithic Linux fork for compatibility and performance | Low; pragmatic engineering |

---

## Roadmap

### Phase 0 Establishment
- Fork seL4 into `perception-1.0`.
- Set up repository structure, CI, and baseline builds for x86 and ARM.
- Import seL4 proof artifacts and document verification baseline.

### Phase 1 Prototyping
- Implement AI‑aware scheduler prototype.
- Add Rust integration scaffolding and example modules.
- Create microbenchmarks for IPC and scheduling.

### Phase 2 Formalization
- Formalize capability extensions in a model amenable to verification.
- Attempt partial proofs for critical invariants affected by experiments.
- Harden CI to run verification tasks where applicable.

### Phase 3 Evaluation
- Run cross‑architecture benchmarks including NPU/GPU workloads.
- Produce reproducible experiment reports and publish datasets.
- Select candidate features for migration to Zenith.

### Phase 4 Migration and Maintenance
- Migrate proven features into Zenith via documented RFCs.
- Maintain Perception as a rolling research branch for new ideas.

---

## Repository Layout

```
perception-kernel/
 ├── docs/                # Design docs, RFCs, experiment reports
 ├── kernel/              # seL4 fork and kernel sources
 ├── proofs/              # Verification artifacts and proof scripts
 ├── experiments/         # Feature prototypes and benchmarks
 ├── tools/               # Build, test, and verification tooling
 ├── ci/                  # CI workflows and reproducible pipelines
 ├── examples/            # Example userland components and Rust modules
 └── README.md            # This document
```

---

## Getting Started

#### Prerequisites
- Linux host for builds.
- Toolchains for target architectures (GCC/Clang, RISC‑V toolchain, ARM toolchain).
- Python 3 for build scripts and verification tooling.
- Docker optional for reproducible CI environments.

#### Quick Build
1. Clone the repository.
2. Run the bootstrap script in `tools/` to install toolchain helpers.
3. Build the kernel for a target:
   - `make ARCH=x86_64` or `make ARCH=arm64` depending on target.
4. Run unit tests and microbenchmarks via `tools/test.sh`.

Detailed build instructions and reproducible container images live in `docs/build.md`.

---

## Verification Strategy

- **Preserve seL4 proofs** for the baseline kernel where possible.
- **Document divergences**: every experimental change that invalidates proofs must include a verification impact statement.
- **Isolate non‑verified code**: experimental modules should be clearly separated and flagged in CI.
- **Incremental proofs**: attempt to re‑establish proofs for critical invariants after stabilization.

Proof artifacts and verification scripts are located in `proofs/` with step‑by‑step reproduction instructions.

---

## CI and Reproducibility

- CI runs:
  - Cross‑architecture builds.
  - Unit tests and microbenchmarks.
  - Reproducible verification steps for baseline artifacts.
- All CI runs must be deterministic and produce artifacts that can be re‑run locally.
- Use pinned container images and toolchain versions to avoid drift.

CI configuration is in `ci/` and includes templates for GitHub Actions and self‑hosted runners.

---

## Contribution Guidelines

- **Propose first**: Open an Issue describing the experiment, goals, and verification impact before opening a PR.
- **Branching**: Use `feature/<short-name>` for experiments. Keep `perception-1.0` stable for reproducible baselines.
- **Documentation**: Every PR must include a short design note and test plan.
- **Verification note**: If a change affects proofs, include a `proof-impact.md` describing what breaks and a plan to address it.
- **Code style**: Follow the repository linting rules. Rust code follows Rustfmt; C code follows the kernel style guide.

---

## Security and Responsible Disclosure

- Treat Perception as a research kernel; do not deploy experimental builds in production.
- Report security issues privately to the Deepcomet security contact listed in `SECURITY.md`.
- Experimental features that affect isolation or capability semantics must include threat models and mitigations.

---

## License

Perception Kernel inherits the license of seL4. All contributions are expected to be compatible with that license. See `LICENSE` for full details.

---

## Governance and Maintainers

- **Maintainers**: Deepcomet Kernel Team.
- **Decision process**: RFCs for major design changes; maintainers review and approve migrations to Zenith.
- **Community**: Research collaborators are welcome; maintainers will triage issues and prioritize reproducible experiments.

---

## Contact and Community

- **Repository Issues**: Use GitHub Issues for proposals and bug reports.
- **Research Papers and Reports**: Place experiment reports in `docs/reports/` and link to external publications where applicable.
- **Mailing List**: See `docs/community.md` for mailing list and meeting cadence.

---

## Appendix

#### Example RFC Template
- Problem statement
- Motivation and use cases
- Design overview
- Verification impact
- Test plan and benchmarks
- Migration criteria

#### Example Experiment Checklist
- Baseline reproducible build
- Unit tests passing
- Microbenchmarks recorded
- Verification impact documented
- Security threat model included

---

> _**Perception Kernel** is intended to be the place where bold microkernel ideas are tried, measured, and either refined for production or documented as valuable negative results. It is a research artifact first and a product second. Welcome to the lab._

---