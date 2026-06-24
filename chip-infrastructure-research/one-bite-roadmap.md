# One Bite Roadmap

How do you eat an elephant?

```text
One bite at a time.
```

That is the process for this project.

The goal is not to understand every layer at once. The goal is to make one tiny piece real, explain what it does to the machine, write the lesson down, then move to the next piece.

## Documentation Rule

Throughout this project, keep writing notes as we go.

Each meaningful step should leave behind:

```text
what we built
what the CPU or hardware is doing
what Rust concept appeared
how it connects to the larger chip-infrastructure idea
what the next bite is
```

The notes are part of the project, not an afterthought.

## Learning Draft Format

For hands-on code bites, keep a pasteable learning draft in:

```text
learning-drafts/
```

Each draft should be formatted like a short paper for humans, but every explanation line should be commented out with `//` so the same file can be pasted into Rust.

Use this pattern:

```text
title and purpose
hardware picture
architecture note
sectioned code
hardware/CPU meaning before Rust syntax meaning
next bite
```

This keeps the learning path readable without separating the explanation from the code.

## The Next Bites

### First Specialized Kernel Target

Decision:

```text
the first specialized kernel/firmware target is matrix multiplication
```

The immediate practical path is still tiny:

```text
screen output first,
then fixed 2x2 matrix multiplication,
then print the result,
then move inputs and outputs through known buffers
```

### 1. Write One Character To The Screen

What we want:

```text
CPU writes byte -> video memory changes -> character appears
```

What it teaches:

```text
memory-mapped hardware
raw pointers
unsafe Rust
why println! does not exist in a kernel yet
```

Why it matters for the big idea:

```text
a hardware group can communicate by writing to a known address
```

### 2. Make A Tiny Print Function

What we want:

```text
kernel_print("hello")
```

What it teaches:

```text
turn one raw hardware action into a reusable kernel procedure
```

Why it matters for the big idea:

```text
each compute group needs small, reliable procedures for its own hardware
```

### 3. Create A Shared Memory Buffer

What we want:

```text
one part of the kernel writes data into a known memory region
another part reads from it
```

What it teaches:

```text
memory ownership
fixed addresses
data layout
controlled communication
```

Why it matters for the big idea:

```text
this is the simplest version of group-to-group communication
```

### 4. Turn The Buffer Into A Ring Buffer

What we want:

```text
writer puts data into the next slot
reader takes data from the next filled slot
```

What it teaches:

```text
queues
read pointers
write pointers
backpressure
not reading half-written data
```

Why it matters for the big idea:

```text
the conveyor belt communication model becomes real
```

### 5. Simulate Two Kernel Groups

What we want:

```text
Group A writes messages
Group B reads messages
```

What it teaches:

```text
separation of responsibilities
message formats
bounded communication
```

Why it matters for the big idea:

```text
this is a small software model of specialized compute groups
```

### 6. Add Signals Or Interrupts

What we want:

```text
receiver is notified when work is ready
```

What it teaches:

```text
how hardware gets CPU attention
how work starts without constant checking
```

Why it matters for the big idea:

```text
groups need a way to say "look over here now"
```

### 7. Add A Timing Model

What we want:

```text
work moves in a predictable rhythm
```

What it teaches:

```text
ticks
waiting
ready/valid thinking
pipeline rhythm
```

Why it matters for the big idea:

```text
same-clock groups can move like a coordinated conveyor belt
```

### 8. Try Multicore Later

What we want:

```text
more than one CPU core participating
```

What it teaches:

```text
real parallelism
cache coherence problems
shared memory hazards
inter-core signaling
```

Why it matters for the big idea:

```text
this starts approaching real CPU-group design
```

## Operating Principle

Keep each bite small enough that it can be:

```text
built
explained
tested
documented
committed
```

Then take the next bite.

## Progress Log

### Bite 1 Complete: Write One Character To The Screen

What we built:

```text
the kernel writes M into the first VGA text cell
```

What the CPU or hardware is doing:

```text
CPU stores byte 0x4d at memory address 0xb8000
CPU stores color byte 0x0a at memory address 0xb8001
VGA text hardware treats those two bytes as one visible screen cell
```

Rust concept that appeared:

```text
raw pointer
unsafe block
volatile write
```

Why `volatile` matters:

```text
this write is meant for hardware, not normal memory
the compiler must not remove it just because no Rust variable reads it later
```

Important architecture note:

```text
VGA output is only a temporary way for us to visually inspect the kernel
the real matrix multiplier output probably should not be VGA/display output
after the visible proof works, the real output path should become a known result buffer
```

How it connects to the matrix firmware idea:

```text
this is the smallest proof that the kernel can write to a known hardware-like address
later, the matrix firmware result can be written to a result buffer instead
```

Next bite:

```text
print a small number
```
