# ApexForge TenaryArch — Roadmap

> From a ternary math foundation to a real kernel and bare-metal hardware.

This roadmap is intentionally layered — each phase produces a working, testable artifact before the next begins. Nothing is thrown away; every layer becomes the foundation for the next.

---

## Phase Overview

```
Phase 0   Architecture Design          ← decisions, no code
Phase 1   Ternary Math Foundation      ← Trit type + arithmetic
Phase 2   Ternary Word                 ← 16-trit CPU word
Phase 3   Registers                    ← R0–R7, PC, SP, FP
Phase 4   Encoding Layer               ← trit ↔ 2-bit binary
Phase 5   Memory Abstraction           ← address translation + mmap backend
Phase 6   ISA Design                   ← instruction set specification
Phase 7   Instruction Encoding         ← opcode → binary representation
Phase 8   VM (Fetch/Decode/Execute)    ← the execution engine
Phase 9   Assembler (TASM)             ← text assembly → machine code
Phase 10  Executable Format (ATX)      ← .atx binary format
Phase 11  Runtime                      ← heap, stack, syscalls, I/O
Phase 12  Kernel                       ← scheduler, virtual memory, drivers
Phase 13  Bootloader                   ← bare-metal init
Phase 14  Real Hardware                ← FPGA / ASIC ternary CPU
```

---

## Phase 0 — Architecture Design

**Goal:** Establish all core decisions before writing any code.

### Decisions

- [ ] Word width: **16 trits** (can represent 0 → 43,046,720)
- [ ] Register file: **R0–R7** (general), **PC, SP, FP, FLAGS** (special)
- [ ] Host encoding: **2 bits per trit** (00=0, 01=1, 10=2, 11=invalid)
- [ ] Memory model: **ternary address space** → translated to binary host offset
- [ ] Host backend: **mmap()** — Linux virtual memory, not physical RAM
- [ ] Assembly syntax: **TASM** (ApexForge Ternary Assembly)
- [ ] Executable format: **ATX** (ApexForge Ternary Executable)
- [ ] Language: **Rust** throughout the VM layer

### Deliverable
`docs/architecture.md` — full architecture decisions documented.

---

## Phase 1 — Ternary Mathematical Foundation

**Goal:** A correct, well-tested `Trit` primitive.

### Files
- `src/cpu/trit.rs`

### Features
- [ ] `Trit` enum: `{Zero, One, Two}` with `#[repr(u8)]`
- [ ] `value()` → u8
- [ ] `from_u8()` → `Option<Trit>`
- [ ] `is_zero()`, `is_nonzero()`
- [ ] Ternary ADD with carry (single trit)
- [ ] Ternary SUB with borrow (single trit)
- [ ] Ternary MUL (single trit)

### Tests
- [ ] All arithmetic combinations for ADD/SUB/MUL
- [ ] Carry/borrow propagation correctness
- [ ] `from_u8` rejects values ≥ 3

### Deliverable
`tests/trit.rs` — full coverage, all passing.

---

## Phase 2 — Ternary Word

**Goal:** A 16-trit CPU word that wraps the `Trit` primitive.

### Files
- `src/cpu/word.rs`

### Features
- [ ] `TernaryWord` — fixed array of 16 `Trit`s
- [ ] `TernaryWord::zero()` — const constructor
- [ ] `from_u64(value: u64) -> TernaryWord` — decimal to ternary
- [ ] `to_u64(&self) -> u64` — ternary to decimal
- [ ] `get(index)` / `set(index, trit)` — trit-level access
- [ ] `dump() -> String` — human-readable ternary string (MSB first)
- [ ] `is_zero()` — check if all trits are zero
- [ ] Word-level ADD (with carry out)
- [ ] Word-level SUB (with borrow out)
- [ ] Bitwise ternary ops: NOT, MIN, MAX (trit-wise)

### Tests
- [ ] Round-trip: `from_u64(x).to_u64() == x` for many values
- [ ] `dump()` output matches expected ternary strings
- [ ] Overflow handling at word boundary

### Deliverable
`tests/word.rs` — full coverage, all passing.

---

## Phase 3 — Registers

**Goal:** A CPU register file matching the architecture spec.

### Files
- `src/cpu/registers.rs`
- `src/cpu/flags.rs`

### Features
- [ ] General registers: `R0`–`R7` (each a `TernaryWord`)
- [ ] Special registers: `PC`, `SP`, `FP` (each a `TernaryWord`)
- [ ] `FLAGS` register with named flag bits: Zero, Carry, Overflow, Negative
- [ ] `read_general(index)` / `write_general(index, value)`
- [ ] `read_pc()` / `write_pc()` / `increment_pc()`
- [ ] `read_sp()` / `push_sp()` / `pop_sp()`
- [ ] Flag read/write helpers: `set_flag()`, `get_flag()`, `clear_flags()`

### Tests
- [ ] Register read/write round-trips
- [ ] Out-of-range register index returns `None`/`Err`
- [ ] Flag set/clear/read correctness

### Deliverable
`tests/registers.rs` — full coverage, all passing.

---

## Phase 4 — Encoding Layer

**Goal:** Lossless, fast conversion between ternary trits and binary bytes.

### Files
- `src/encoding/ternary.rs`
- `src/encoding/binary.rs`

### Features
- [ ] `trit_to_bits(trit: Trit) -> u8` — 2-bit encoding
- [ ] `bits_to_trit(bits: u8) -> Option<Trit>` — decode (rejects `11`)
- [ ] `word_to_bytes(word: &TernaryWord) -> Vec<u8>` — 16 trits → 4 bytes (32 bits, 2 wasted)
- [ ] `bytes_to_word(bytes: &[u8]) -> Result<TernaryWord, _>` — decode
- [ ] Efficient bulk encoding for memory blocks

### Tests
- [ ] Round-trip: `bytes_to_word(word_to_bytes(w)) == w`
- [ ] Invalid bit pattern `11` is rejected
- [ ] All 16-trit words encode/decode correctly

### Deliverable
`tests/encoding.rs` — full coverage, all passing.

---

## Phase 5 — Memory Abstraction

**Goal:** A host-backed memory system with ternary address translation.

### Files
- `src/memory/address.rs`
- `src/memory/host.rs`
- `src/memory/layout.rs`

### Features
- [ ] `TernaryAddress` type (wraps `TernaryWord`)
- [ ] `address_to_offset(addr: TernaryAddress) -> usize` — ternary addr → byte offset in host
- [ ] `HostMemory` struct — backed by `mmap()` or `Vec<u8>`
- [ ] `read_word(addr)` → `TernaryWord`
- [ ] `write_word(addr, word)`
- [ ] `read_trit(addr, trit_offset)` — sub-word access
- [ ] Memory layout constants: CODE region, DATA region, STACK region, HEAP region

### Notes
- `mmap()` gives Linux **virtual** address space, not physical RAM
- Bounds checking on every access in debug builds
- `unsafe` only in the `host.rs` backend, never in CPU/ISA layers

### Tests
- [ ] Write then read at same address returns same word
- [ ] Out-of-bounds access returns `Err` (no segfault)
- [ ] Address translation is deterministic and collision-free

### Deliverable
`tests/memory.rs` — full coverage, all passing.

---

## Phase 6 — ISA Design

**Goal:** A complete instruction set specification for the ternary CPU.

### Files
- `src/isa/opcode.rs`
- `src/isa/instruction.rs`
- `docs/instruction-set.md`

### Instruction Categories

#### Data Movement
```
PUT   Rdst, imm       — load immediate into register
MOVE  Rdst, Rsrc      — copy register
LOAD  Rdst, addr      — load word from memory
SAVE  Rsrc, addr      — store word to memory
```

#### Arithmetic
```
ADD   Rdst, Rsrc
SUB   Rdst, Rsrc
MUL   Rdst, Rsrc
DIV   Rdst, Rsrc
MOD   Rdst, Rsrc
```

#### Logic
```
AND   Rdst, Rsrc      — trit-wise MIN
OR    Rdst, Rsrc      — trit-wise MAX
NOT   Rdst            — trit complement (0↔2, 1→1)
```

#### Control Flow
```
JUMP  addr
JMPZ  addr            — jump if FLAGS.Zero
JMPN  addr            — jump if FLAGS.Negative
CALL  addr            — push PC, jump
RET                   — pop PC
```

#### System
```
STOP                  — halt execution
NOP                   — no operation
SYS   code            — syscall (future runtime)
```

### Features
- [ ] `Opcode` enum for all instructions
- [ ] `Instruction` struct: opcode + operands
- [ ] Instruction encoding: fixed-width or variable-width (to be decided)

### Deliverable
`docs/instruction-set.md` — full ISA reference.

---

## Phase 7 — Instruction Encoding

**Goal:** Map assembly instructions to their binary/ternary machine representation.

### Features
- [ ] Each instruction has a fixed ternary machine code format
- [ ] Opcode field, register fields, immediate/address fields
- [ ] `encode(instruction) -> TernaryWord` (or multi-word)
- [ ] `decode(word) -> Result<Instruction, _>`
- [ ] Encoding reference table in `docs/instruction-set.md`

### Deliverable
Assembler can produce machine code. VM can decode and execute it.

---

## Phase 8 — Virtual Machine

**Goal:** A working fetch/decode/execute CPU simulation.

### Files
- `src/vm/machine.rs`
- `src/vm/fetch.rs`
- `src/vm/execute.rs`

### Features
- [ ] `Machine` struct: CPU registers + memory
- [ ] `fetch()` — read instruction at PC, advance PC
- [ ] `decode(word)` — parse into `Instruction`
- [ ] `execute(instruction)` — dispatch to handler
- [ ] Main loop: `fetch → decode → execute → repeat until STOP`
- [ ] Debug mode: print register state after each instruction

### Example Program
```asm
PUT  R0, 42
PUT  R1, 10
ADD  R0, R1     ; R0 = 52
SAVE R0, 100    ; memory[100] = 52
STOP
```

### Deliverable
A running VM that executes TASM programs loaded into memory.

---

## Phase 9 — Assembler (TASM)

**Goal:** Compile `.tasm` source files into `.atx` executables.

### Files
- `src/assembler/lexer.rs`
- `src/assembler/parser.rs`
- `src/assembler/encoder.rs`

### Features
- [ ] Lexer: tokenize TASM source (opcodes, registers, literals, labels, comments)
- [ ] Parser: build AST of instructions and directives
- [ ] Encoder: emit binary machine code
- [ ] Label resolution: forward references, jump targets
- [ ] Error reporting: line number, column, message

### TASM Syntax
```asm
; This is a comment
start:
    PUT  R0, 42
    PUT  R1, 5
    ADD  R0, R1
    SAVE R0, result
    STOP

result: .word 0
```

### Deliverable
`tasm` binary: `tasm program.tasm -o program.atx`

---

## Phase 10 — Executable Format (ATX)

**Goal:** A defined binary format for compiled ternary programs.

### ATX Format
```
Offset  Field         Size    Description
──────────────────────────────────────────
0       Magic         4B      "ATX\0"
4       Version       2B      format version
6       Entry point   4B      ternary address of _start
10      Code offset   4B      byte offset of CODE section
14      Code length   4B      bytes in CODE section
18      Data offset   4B      byte offset of DATA section
22      Data length   4B      bytes in DATA section
26      Sym offset    4B      byte offset of SYMBOL table
30      Sym length    4B      bytes in SYMBOL table
34      [CODE]        var     ternary machine code
??      [DATA]        var     static initialized data
??      [SYMBOLS]     var     name → address table
```

### Features
- [ ] `atx::write(program, path)` — serialize to file
- [ ] `atx::read(path)` — deserialize and validate magic/version
- [ ] VM can load and execute an ATX file directly

---

## Phase 11 — Runtime

**Goal:** Process model, heap, stack, and basic I/O.

### Features
- [ ] Stack allocator (grows downward from top of address space)
- [ ] Heap allocator (simple bump allocator, later slab/buddy)
- [ ] Syscall table: `write`, `read`, `exit`, `alloc`, `free`
- [ ] Standard I/O mapped to host stdin/stdout
- [ ] File I/O (host filesystem passthrough)
- [ ] Process context: single process per VM instance

### Deliverable
Programs can do meaningful I/O and memory allocation.

---

## Phase 12 — Kernel

**Goal:** A minimal OS kernel running on the VM.

### Subsystems
- [ ] **Memory manager** — physical page allocator, virtual address space
- [ ] **Scheduler** — preemptive round-robin, multi-core ready
- [ ] **Interrupt controller** — timer, I/O, fault handling
- [ ] **Virtual memory** — page tables, mmap, demand paging
- [ ] **Filesystem** — VFS interface + simple flat FS
- [ ] **Driver model** — character and block device abstraction
- [ ] **Syscall ABI** — stable interface for user programs

### Deliverable
NeutronOS kernel boots inside the ternary VM, runs user programs.

---

## Phase 13 — Bootloader

**Goal:** Bootstrap the system from power-on to kernel.

### Features
- [ ] Stage 1: minimal ternary CPU init (register/flag reset)
- [ ] Stage 2: memory detection and layout
- [ ] Stage 3: load kernel from disk into memory
- [ ] Stage 4: handoff to kernel entry point

### Note
At this stage the ternary CPU is still simulated on x86_64 host hardware.

---

## Phase 14 — Real Hardware

**Goal:** Run ApexForge TenaryArch on actual ternary hardware.

### Option A — Software Emulation (default)
Continue running the ternary CPU as a software emulator on x86_64. This is already functional after Phase 8.

### Option B — FPGA Prototype
- Implement ternary ALU, register file, and control logic in HDL (Verilog/VHDL)
- Target a development FPGA board
- Validate against the software emulator (same test suite)

### Option C — ASIC
- Full custom silicon design
- Requires EDA toolchain, fab partnership
- Long-term / research-level goal

---

## Current Status

| Phase | Status | Notes |
|-------|--------|-------|
| Phase 0 — Architecture Design | ✅ Done | Decisions documented |
| Phase 1 — Trit Foundation | 🔄 In Progress | `trit.rs` written |
| Phase 2 — Ternary Word | 🔄 In Progress | `word.rs` written |
| Phase 3 — Registers | 🔄 In Progress | `registers.rs` written |
| Phase 4 — Encoding | ⬜ Not started | |
| Phase 5 — Memory | ⬜ Not started | |
| Phase 6 — ISA Design | ⬜ Not started | |
| Phase 7 — Instruction Encoding | ⬜ Not started | |
| Phase 8 — VM | ⬜ Not started | |
| Phase 9 — Assembler | ⬜ Not started | |
| Phase 10 — ATX Format | ⬜ Not started | |
| Phase 11 — Runtime | ⬜ Not started | |
| Phase 12 — Kernel | ⬜ Not started | |
| Phase 13 — Bootloader | ⬜ Not started | |
| Phase 14 — Real Hardware | ⬜ Not started | |

---

## File Creation Order (Immediate Next Steps)

```bash
# Phase 1+2+3 — already scaffolded
src/cpu/trit.rs
src/cpu/word.rs
src/cpu/registers.rs
src/cpu/flags.rs
src/cpu/mod.rs

# Phase 4
src/encoding/ternary.rs
src/encoding/binary.rs
src/encoding/mod.rs

# Phase 5
src/memory/address.rs
src/memory/host.rs
src/memory/layout.rs
src/memory/mod.rs

# Phase 6–7
src/isa/opcode.rs
src/isa/instruction.rs
src/isa/mod.rs

# Phase 8
src/vm/machine.rs
src/vm/fetch.rs
src/vm/execute.rs
src/vm/mod.rs
```

---

*ApexForge © 2026 — Building the future from the ground up.*