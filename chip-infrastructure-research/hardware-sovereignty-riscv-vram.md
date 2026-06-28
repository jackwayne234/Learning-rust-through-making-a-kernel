# Hardware Sovereignty: Why RISC-V Is The Way Forward

Core realization:

```text
Modern high-performance GPU memory management is mostly a black box.
```

That matters because the thing we want to study is not only how to call a GPU.
The deeper goal is to understand and eventually control how compute units, memory
controllers, page mappings, buffers, and accelerators cooperate.

On closed consumer GPUs, the most important low-level pieces are hidden behind
manufacturer firmware, drivers, and APIs. CUDA, ROCm, Metal, Vulkan, and similar
interfaces can expose powerful compute, but they do not normally let us rewrite
the memory controller, the command processor, the page-table machinery, or the
physical arbitration rules.

So if the goal is:

```text
design our own memory behavior for accelerator workloads
```

then the long-term path is not just "write a better GPU driver."

The long-term path is:

```text
open hardware + open memory hierarchy + open accelerator controller
```

## The Walled Garden

Current GPU platforms are powerful, but they are not fully inspectable.

The usual software layers let us:

```text
allocate buffers
launch kernels
copy data
use shared memory inside a compute kernel
profile some performance behavior
```

But they usually do not let us directly own:

```text
physical VRAM arbitration
GPU page-table design
MMU behavior
command processor firmware
cache hierarchy rules
memory-controller scheduling
```

That creates a hard ceiling for this project.

We can learn a lot through existing APIs, but the actual memory-management heart
of the device remains controlled by a closed stack.

## Why RISC-V Changes The Shape Of The Problem

RISC-V is important because the instruction set is open.

That does not magically give us a finished GPU. It gives us permission to build
and inspect the machine from lower layers.

With an open RISC-V-based system, we can theoretically define:

```text
custom instructions
custom accelerators
custom memory-mapped devices
custom page-table behavior
custom buffer protocols
custom memory-controller policy
```

The key shift is this:

```text
closed GPU: learn the exposed API
open RISC-V system: design the hardware/software contract
```

This is why RISC-V belongs in the long-term roadmap. It moves the project from
being only a user of a device toward being the architect of a device.

## The Hard Part

This is no longer only software engineering.

To control accelerator memory at this level, the work crosses into computer
architecture:

```text
RTL design
FPGA prototyping
bus protocols
cache behavior
MMU design
memory-controller scheduling
accelerator command queues
verification
timing closure
```

That is a steep climb, but it is also the honest version of the dream.

If we want to control the equivalent of VRAM behavior, then we must understand
the hardware that decides:

```text
who gets memory access
when data moves
where data is stored
how buffers are protected
how compute groups signal completion
what happens when pressure exceeds capacity
```

## Practical Bridge: Simulation And FPGA

The practical path should be staged.

First, simulate:

```text
model buffers
model command queues
model page-like regions
model producer/consumer pressure
model accelerator completion signals
```

Then prototype:

```text
small RISC-V core
small memory-mapped accelerator
small shared buffer
small controller protocol
FPGA implementation
```

An FPGA is the bridge between software and custom silicon. It lets us test real
hardware logic without manufacturing a chip.

The first FPGA goal should not be "build a modern GPU."

The first FPGA goal should be:

```text
CPU writes command -> accelerator reads buffer -> accelerator writes result -> CPU observes status
```

That is enough to make the memory-control problem real.

## Unified Memory Goal

The deeper architecture goal is a unified system where CPU-like control logic
and accelerator logic share a memory design that we can inspect.

Possible shape:

```text
Coordinator CPU Group
-> command queue
-> Matrix / Vector Accelerator Group
-> result buffer
-> monitor/status path
-> shared memory controller
```

The important part is not that every group sees one giant magical memory space.
The important part is that the rules are explicit:

```text
which group owns which region
which group can read or write
what the buffer depth is
what happens on overflow
how completion is signaled
how monitoring observes without slowing the main path
```

## Why This Fits The Existing Project

This matches the chip-infrastructure direction already in this repo.

The project has been moving toward:

```text
specialized compute groups
known communication limits
bounded buffers
monitoring that does not slow the data path
matrix multiplication as the first accelerator-like target
```

The RISC-V/open-hardware direction explains why those pieces matter.

They are not just learning exercises. They are the small software shadows of the
hardware contracts we eventually want to own.

## Near-Term Roadmap

Keep the immediate path tiny:

```text
1. Finish direct screen output.
2. Print a fixed 2x2 matrix multiplication result.
3. Move matrix inputs and outputs through a fixed shared buffer.
4. Turn that buffer into a ring buffer.
5. Simulate a coordinator group and matrix group in software.
6. Add ready/status/completion flags.
7. Document the memory contract.
8. Later, map the same contract onto a small RISC-V or FPGA prototype.
```

The goal is not to jump straight to a custom chip.

The goal is to make each contract explicit enough that it could later become
hardware.

## Bottom Line

This is the way forward:

```text
Hardware sovereignty.
```

Closed GPUs are useful learning tools, but they are not the final home for this
idea.

RISC-V and FPGA prototyping are the path from:

```text
user of a closed accelerator
```

to:

```text
architect of an open accelerator memory system
```

That path will take years, but it points in the right direction.
