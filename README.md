# ApexForge TenaryArch

> **A ternary CPU virtual machine built in Rust — the foundation for a future custom OS kernel.**

ApexForge TenaryArch is a ternary (base-3) virtual CPU and virtual machine implemented entirely in Rust. The CPU operates logically in a `{0, 1, 2}` trit world, while host memory remains binary — a clean encoding/decoding layer bridges the two. The long-term goal is to evolve this VM into a real kernel and eventually target bare-metal hardware.

---

## What is a Ternary CPU?

Most computers today are **binary** — every signal is either `0` or `1`.  
A **ternary** system uses three states: `0`, `1`, and `2`.

This means:
- 1 **trit** (ternary digit) carries `log₂(3) ≈ 1.585` bits of information
- 16 trits can represent values from `0` to `3¹⁶ − 1 = 43,046,720`
- Ternary arithmetic has natural properties that simplify certain operations (e.g. balanced ternary for signed arithmetic)

The CPU in this project is a **software emulation** of a ternary CPU running on standard x86_64 binary hardware.

---

## Architecture Overview

```
ApexForge TenaryArch

              ┌──────────────────────────────┐
              │         Ternary CPU           │
              │  ┌────────┬────────────────┐  │
              │  │  Trit  │  TernaryWord   │  │
              │  └────────┴────────────────┘  │
              │  ┌──────────────────────────┐ │
              │  │      Registers           │ │
              │  │  R0–R7 | PC | SP | FP   │ │
              │  └──────────────────────────┘ │
              └──────────────┬───────────────┘
                             │
                     ┌───────┴───────┐
                     │      ISA      │
                     │  Instruction  │
                     └───────┬───────┘
                             │
                     ┌───────┴───────┐
                     │      VM       │
                     │ Fetch→Decode  │
                     │   →Execute    │
                     └───────┬───────┘
                             │
                     ┌───────┴────────┐
                     │    Memory      │
                     │ Address Trans. │
                     └───────┬────────┘
                             │
                     ┌───────┴────────┐
                     │  Host Backend  │
                     │  mmap / RAM    │
                     └────────────────┘
```

---

## Key Design Principles

### 1. Logical vs. Physical Separation
The CPU is **logically ternary** — programs, registers, and addresses all think in base-3. The host machine is **physically binary** — RAM stores bytes. A dedicated encoding layer handles the translation transparently.

```
Logical Trit  →  2-bit host encoding
     0        →  00
     1        →  01
     2        →  10
  (invalid)   →  11
```

### 2. Modular, Kernel-Ready Architecture
Every subsystem is isolated:
- `cpu/` — trit primitives, word arithmetic, registers
- `encoding/` — ternary ↔ binary conversion
- `memory/` — address translation, host memory backend
- `isa/` — instruction set and opcode definitions
- `vm/` — fetch/decode/execute pipeline
- `assembler/` — text assembly → machine code

This separation means migrating from VM to kernel later requires **no full rewrite** — only the memory backend and execution context need to change.

### 3. mmap() is not physical RAM
The host memory backend uses `mmap()` for large ternary address spaces. This gives the Linux kernel's virtual memory system, **not** direct physical memory access. Real hardware support comes in a later phase via bootloader + MMU + physical memory manager.

---

## Project Structure

```
ApexForge_TenaryArch/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
├── ROADMAP.md
│
├── docs/
│   ├── architecture.md
│   ├── instruction-set.md
│   ├── ternary-math.md
│   ├── memory-model.md
│   └── roadmap.md
│
├── src/
│   ├── main.rs
│   ├── cpu/
│   │   ├── mod.rs
│   │   ├── trit.rs       ← Trit enum {Zero, One, Two}
│   │   ├── word.rs       ← TernaryWord<16>
│   │   ├── registers.rs  ← R0–R7, PC, SP, FP
│   │   ├── flags.rs      ← CPU status flags
│   │   └── cpu.rs        ← CPU state machine
│   ├── encoding/
│   │   ├── mod.rs
│   │   ├── ternary.rs    ← ternary arithmetic
│   │   └── binary.rs     ← trit ↔ 2-bit encoding
│   ├── memory/
│   │   ├── mod.rs
│   │   ├── host.rs       ← mmap-backed memory
│   │   ├── address.rs    ← ternary address → binary offset
│   │   └── layout.rs     ← memory region layout
│   ├── isa/
│   │   ├── mod.rs
│   │   ├── opcode.rs     ← instruction opcodes
│   │   └── instruction.rs
│   ├── vm/
│   │   ├── mod.rs
│   │   ├── machine.rs    ← VM state
│   │   ├── fetch.rs      ← fetch instruction
│   │   └── execute.rs    ← execute instruction
│   ├── assembler/
│   │   ├── mod.rs
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   └── encoder.rs
│   └── debugger/
│       ├── mod.rs
│       └── dump.rs
│
└── tests/
    ├── trit.rs
    ├── word.rs
    ├── registers.rs
    ├── encoding.rs
    └── memory.rs
```

---

## Quick Start

### Prerequisites
- Rust (stable, 2021 edition or later)
- Cargo

### Build

```bash
git clone https://github.com/ApexForge/TenaryArch
cd ApexForge_TenaryArch
cargo build
```

### Run

```bash
cargo run
```

### Test

```bash
cargo test
```

---

## Assembly Language (TRASM)

ApexForge TenaryArch uses its own assembly syntax (`.trasm` files):

```asm
; Load two values, add them, store result
PUT   R0, 42
PUT   R1, 10
ADD   R0, R1
SAVE  R0, 100
STOP
```

Assembled programs are stored in the `.atx` (ApexForge Ternary Executable) binary format.

### ATX Executable Format

```
┌──────────────┐
│    HEADER    │  magic, version, entry point
├──────────────┤
│     CODE     │  ternary machine instructions
├──────────────┤
│     DATA     │  static data
├──────────────┤
│   SYMBOLS    │  debug/linker symbol table
└──────────────┘
```

---

## Ternary Math Quick Reference

| Operation | Example | Result |
|-----------|---------|--------|
| Add       | 2 + 2   | 11₃ (carry=1, digit=1) |
| Max value (16 trit) | — | 3¹⁶ − 1 = 43,046,720 |
| 1 trit info | — | ≈ 1.585 binary bits |

---

## Registers

| Register | Purpose |
|----------|---------|
| R0 – R7  | General-purpose (8 registers) |
| PC       | Program Counter |
| SP       | Stack Pointer |
| FP       | Frame Pointer |
| FLAGS    | CPU status flags (zero, carry, overflow, …) |

---

## Long-Term Vision

This VM is the **first layer** of a larger system:

```
TenaryArch VM
    ↓
Assembler + Linker
    ↓
Runtime (heap, stack, syscalls)
    ↓
Kernel (scheduler, virtual memory, drivers)
    ↓
Bootloader
    ↓
Real Hardware (FPGA / ASIC ternary CPU)
```

The ultimate goal is **NeutronOS** — a fully custom operating system by ApexForge, built from the ground up on this ternary architecture.

---

## License

ApexForge © 2026. All rights reserved.