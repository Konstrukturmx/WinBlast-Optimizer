# WinBlast0 Agent Guidelines & Rules

Welcome to the WinBlast0 repository. All autonomous agents, swarm workers, and contributors must adhere to the following rules:

## 1. UI Layout & Collision Prevention Rule
- Refer to [ui-layout-collision.md](file:///c:/Hive/Projects/WinBlast0/.agents/rules/ui-layout-collision.md) for mandatory Rune scroll wrapper ordering and responsive styling invariants.
- **Rule:** Always attach `.child(element)` **BEFORE** `.winblast_scroll(p)`. Never chain `.child(...)` after `.winblast_scroll(p)`.
- Ensure all scrollable lists and cards specify explicit height bounds and responsive `.flex_wrap()` reflow.

## 2. Zero Fabricated Hardware Telemetry
- WinBlast0 reports authentic physical hardware diagnostics.
- Fallbacks must honestly indicate sensor requirements or probe non-privileged Windows APIs (WMI root\cimv2, user-mode NVML/nvidia-smi, IOCTLs, Get-PhysicalDisk) without generating fake random or static dummy values.

## 3. Strict Quality & Compiler Standards
- 100% test pass rate across all suites.
- Zero warnings under `cargo check --all-targets` and `cargo clippy --all-targets -- -D warnings`.
- Code changes must be synchronized with `/graphify` for persistent memory.

## 4. Dual-Engine Coding & Algorithmic Delegation Protocol
- Refer to [dual-engine-coding-protocol.md](file:///c:/Hive/Projects/WinBlast0/.agents/rules/dual-engine-coding-protocol.md) for role assignment and model specialization.
- **Tier 1 (Antigravity Gemini 3.8 Flash)** handles orchestration, basic coding, UI wiring, Win32 API calls, compiler checks, and tests.
- **Tier 2 (Browser Gemini Web Engine at gemini.google.com)** handles complex algorithms, deep optimizations, and mathematical formulations.
- Results from Tier 2 are copied and integrated into the local workspace by Tier 1, followed by 100% test verification and `/graphify` memory synchronization.

