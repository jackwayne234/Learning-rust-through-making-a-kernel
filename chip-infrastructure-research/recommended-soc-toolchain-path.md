# Recommended SoC Toolchain Path

Core decision:

```text
Do not build the whole SoC from scratch by hand.
Use existing SoC frameworks as scaffolding, then customize the memory and
accelerator pieces that matter to this project.
```

This keeps the dream ambitious without making the first step impossible.

The point is not to avoid learning how the machine works. The point is to learn
inside a toolchain that already knows how to connect cores, buses, memories,
accelerators, simulators, and FPGA targets.

## Why This Is The Better Path

Building an SoC means solving many problems at once:

```text
CPU core
bus/interconnect
memory map
interrupts
boot process
device registers
accelerator interface
simulation
FPGA build
software toolchain
debugging
```

Trying to invent all of that at the same time would bury the real research
question.

The real research question is:

```text
Can we design an open memory contract for accelerator-style work?
```

So the strategy should be:

```text
borrow the SoC skeleton,
then customize the memory/accelerator contract
```

## Recommended Stack

### Chipyard

Use Chipyard as the main research reference for RISC-V SoC generation.

Why it matters:

```text
RISC-V cores
accelerator integration
memory hierarchy work
MMIO devices
DMA devices
simulation
FPGA-accelerated simulation through FireSim
VLSI/tapeout-oriented paths through Hammer
```

Chipyard is not the easiest first tool, but it is close to the kind of system we
eventually care about: a configurable RISC-V SoC with real accelerator and memory
integration points.

Project use:

```text
long-term SoC research framework
```

### LiteX

Use LiteX as the practical FPGA-friendly SoC builder to keep in view.

Why it matters:

```text
builds SoCs for FPGA boards
supports multiple CPU cores
provides common peripherals
is friendlier for hardware bring-up experiments
```

LiteX may be the easier bridge when the goal becomes:

```text
put a small SoC on an FPGA and make it talk to a custom block
```

Project use:

```text
first real FPGA SoC experiments
```

### Vortex

Use Vortex as a reference for open RISC-V GPGPU thinking.

Why it matters:

```text
RISC-V-based GPGPU design
OpenCL-oriented programming model
SIMT-style accelerator ideas
parallel memory pressure
```

Vortex should not be treated as the first build target. It should be studied as
a reference for what an open GPU-like architecture looks like.

Project use:

```text
reference design for open accelerator architecture
```

### OpenROAD

Use OpenROAD only much later.

Why it matters:

```text
RTL-to-GDS physical design flow
floorplanning
placement
routing
chip-layout path
```

This is not the next step. OpenROAD matters if the project ever moves from
simulation/FPGA toward ASIC layout.

Project use:

```text
future chip-layout path, not the beginner path
```

## What We Should Build First

The first goal should stay small:

```text
software model of the memory contract
```

Then:

```text
bare-metal Rust version of the same contract
```

Then:

```text
simulated SoC version
```

Then:

```text
FPGA SoC version
```

Only after that should we worry about ASIC-style flows.

## First Contract To Carry Forward

The first accelerator contract can be tiny:

```text
CPU/coordinator writes command
CPU/coordinator writes input buffer
accelerator reads input buffer
accelerator computes fixed 2x2 matrix result
accelerator writes output buffer
accelerator sets done/status flag
CPU/coordinator reads status and result
```

That contract can exist in many forms:

```text
Rust software simulation
bare-metal Rust memory addresses
Chipyard MMIO peripheral
LiteX FPGA peripheral
eventual custom accelerator block
```

This is the useful trick:

```text
keep the contract stable while changing the hardware underneath it
```

## Staged Roadmap

### Stage 1: Kernel Learning

Keep learning direct hardware ownership:

```text
screen memory
raw pointers
fixed addresses
simple output
small matrix multiply
```

This is where we learn what memory-mapped hardware feels like.

### Stage 2: Software Contract

Write a simple memory contract in normal code:

```text
command region
input buffer
output buffer
status flag
error code
```

This gives us a clean shape before hardware gets complicated.

### Stage 3: Bare-Metal Contract

Move the same idea into the bare-metal Rust kernel:

```text
known memory addresses
fixed buffer layout
status bytes
no dynamic allocation
observable result
```

This connects the contract to the machine.

### Stage 4: SoC Framework Study

Study the minimum path through Chipyard and LiteX:

```text
how to add an MMIO peripheral
how memory maps are declared
how interrupts are wired
how simulation runs
how FPGA targets are built
```

The goal is not mastery of everything. The goal is to find the smallest place
where our contract can plug in.

### Stage 5: FPGA Prototype

Build the smallest hardware version:

```text
RISC-V CPU
MMIO matrix accelerator
input registers or buffer
output registers or buffer
status flag
```

This is the first real proof that the memory contract can become hardware.

### Stage 6: Deeper Memory Research

Only after the small version works, study harder topics:

```text
DMA
cache coherence
scratchpad memory
shared memory arbitration
page-table-like protection
multiple accelerator groups
monitoring taps
backpressure
```

## What Not To Do Yet

Do not start by trying to:

```text
build a full GPU
design a complete MMU
write a complete operating system for a custom SoC
make an ASIC
replace CUDA or ROCm
compete with NVIDIA or AMD performance
```

Those are later horizon items.

The first win is much smaller:

```text
an open, inspectable accelerator memory contract that actually runs somewhere
```

## Bottom Line

This is the sane path:

```text
learn bare metal,
define the contract,
simulate it,
map it into a recommended SoC framework,
then prototype on FPGA.
```

We use the existing SoC software because it lets us focus on the unique idea:

```text
owning the memory behavior between a coordinator and an accelerator.
```

That is how this project stays real.

## References To Study Later

Start with the official project material:

```text
Chipyard documentation:
https://chipyard.readthedocs.io/

LiteX repository:
https://github.com/enjoy-digital/litex

Vortex repository:
https://github.com/vortexgpgpu/vortex

OpenROAD documentation:
https://openroad.readthedocs.io/
```
