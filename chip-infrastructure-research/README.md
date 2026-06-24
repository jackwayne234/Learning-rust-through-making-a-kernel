# Chip-Level Infrastructure Research

This folder is for the big idea behind the kernel project:

What if an entire IT infrastructure could be designed down at the chip level?

Instead of separate servers connected over a network, imagine one large chip made of specialized compute groups. Each group owns a job, has local resources, and communicates with the other groups through fast on-chip messages.

## The Core Idea

Traditional infrastructure looks roughly like this:

```text
app server -> network -> database server -> network -> storage -> network -> app server
```

A chip-level infrastructure version might look like this:

```text
general CPU group -> on-chip message bus -> storage/I/O group
general CPU group -> on-chip message bus -> AI group
general CPU group -> on-chip message bus -> graphics/display group
```

The dream is not just "more CPUs."

The dream is:

```text
specialized compute groups + specialized kernels + fast local communication
```

## Possible Compute Groups

```text
General CPU Group
- runs normal control code
- coordinates work
- handles application logic

AI / NPU Group
- runs model inference
- handles tensor/math-heavy work
- keeps model data close to the compute units

Graphics / Display Group
- builds frames
- handles display output
- owns frame buffers

I/O Controller Group
- talks to storage, network, keyboard, mouse, sensors, or external devices
- handles interrupts from devices
- moves data into the chip

Memory / Data Group
- manages shared memory regions
- moves data between local memories
- enforces access rules
```

## First Timing Model

For any system, a useful first formula is:

```text
total time = compute time + data movement time + waiting time + coordination overhead
```

For a normal networked system:

```text
request time =
local compute
+ operating system overhead
+ network stack overhead
+ switch/router delay
+ wire delay
+ remote queue wait
+ remote compute
+ response path back
```

For a chip-level system:

```text
request time =
local kernel/group dispatch
+ on-chip message delay
+ local/shared memory access
+ target group queue wait
+ target compute
+ response path back
```

## Rough Latency Ladder

These are not exact numbers. They are starting-place mental models.

```text
CPU register operation:       less than 1 ns
CPU cache access:             about 1-10 ns
Main memory access:           about 50-150 ns
On-chip interconnect:         about 10-500 ns
PCIe/device access:           microseconds
Same data center network:     tens to hundreds of microseconds
Internet request:             milliseconds or more
```

The major question:

```text
How much time do we save by replacing network movement with on-chip movement?
```

## Example Speed Comparison

If a network message takes:

```text
500 microseconds
```

that is:

```text
500,000 nanoseconds
```

If an on-chip message takes:

```text
100 nanoseconds
```

then the message movement is:

```text
500,000 / 100 = 5,000 times faster
```

That does not mean the whole application is 5,000 times faster. If the actual computation is slow, compute time may still dominate.

But for systems that send many small messages, the savings could be huge.

## What We Want To Learn

This research connects directly to the kernel project.

When we learn how to write text to the screen, we are learning direct hardware ownership.

When we learn interrupts, we are learning how hardware asks a CPU for attention.

When we learn memory management, we are learning how compute groups can be isolated.

When we learn message passing, we are learning how different kernels or compute islands might communicate.

## Research Questions

1. How many CPU cores should one kernel control before returns diminish?
2. What jobs should become separate compute groups?
3. What data needs to move between groups?
4. How fast is network communication compared with on-chip communication?
5. What bottlenecks remain even when everything is on one chip?
6. How does memory isolation work between groups?
7. What would a programmer's interface to this chip look like?
8. What parts of Apple Silicon, GPUs, NPUs, and SoCs already resemble this idea?

## Notes

```text
communication-models.md
group-design-principles.md
one-bite-roadmap.md
```

This note explores designated memory addresses, conveyor-belt buffers, shared clocks, message passing, interrupts, and slower application-layer communication.

The group design note records the architecture rule that communication should be mapped before deciding the compute group boundaries.

The roadmap note keeps the project grounded in small steps: build one tiny piece, explain it, document it, commit it, then move to the next bite.

## First Drawing Prompt

Draw one big box named:

```text
BIG CHIP
```

Inside it, draw:

```text
General CPU Kernel Group
AI Kernel Group
Graphics Kernel Group
I/O Kernel Group
Memory/Data Group
On-Chip Message Bus
```

Then draw arrows between them.

For each arrow, label what moves:

```text
request
response
data block
interrupt
frame
model input
model output
```

The arrows are the system.

## Next Step

Pick one real-world system to compare against this idea.

Possible examples:

```text
web app + database
AI app + vector database
game engine + GPU + asset streaming
cloud function + storage + queue
robot sensor + control loop + motor output
```

Then estimate:

```text
how many messages move
how large each message is
how long each message takes
how much compute happens at each stage
what bottleneck appears first
```
