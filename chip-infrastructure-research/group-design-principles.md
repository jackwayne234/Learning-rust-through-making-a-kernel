# Group Design Principles

## Overall Goal

The goal is to design a system of communication with known limits and known capabilities.

Once the communication system is understood, the architecture can support adding more compute groups over time.

The long-term shape:

```text
known communication rules
+ known bandwidth and delay limits
+ known memory ownership rules
+ known signaling rules
= groups that can be added without redesigning everything
```

This means the communication layer is the foundation.

The groups plug into it.

The dream architecture is not just:

```text
many groups
```

It is:

```text
a reliable communication system that lets us keep adding groups
```

## Communication First, Groups Second

A compute group boundary should be chosen after we understand the communication paths.

The key idea:

```text
group design follows communication design
```

Before deciding what groups the chip needs, ask:

```text
who needs to talk to whom?
how often?
how fast?
how much data moves?
does the data need to be durable?
does the data need to be private?
does the exchange need to happen on a shared clock?
can the exchange wait?
what happens if the receiver is busy?
```

The answers shape the groups.

## Why This Matters

A group boundary is not just a box on a diagram.

It creates a communication problem.

If two jobs constantly exchange tiny urgent signals, they may need to be close together, clocked together, or connected by a very fast buffer.

If two jobs only exchange slow durable state, they can be separated more cleanly and communicate through a database-like or log-like layer.

## Same-Type Groups vs Cross-Type Groups

There is a difference between communication inside a group family and communication between different group families.

Example:

```text
GPU Group 1
GPU Group 2
GPU Group 3
GPU Group 4
```

These four GPU groups may communicate with each other in a specialized way because they do similar work, use similar data, and may share the same timing assumptions.

That is same-type communication:

```text
GPU group -> GPU group
```

It may use:

```text
shared tile buffers
frame partitions
same-clock pipelines
high-bandwidth local links
work-stealing queues
```

But the whole GPU family talking to a different kind of group is a different problem.

Example:

```text
GPU family -> General CPU Group
GPU family -> Memory/Data Group
GPU family -> Display/I/O Group
```

That is cross-type communication.

It may need:

```text
clear command messages
input/output buffers
format conversion
permission checks
completion signals
slower but more general protocols
```

If the families use different clocks, the boundary may need a dedicated buffer or adapter group.

Example:

```text
GPU family clock -> Buffer/Adapter Group -> CPU family clock
```

That buffer can translate between different rhythms and different data widths.

Example idea:

```text
GPU side: 32-bit values
adapter: packs/stores into predictable registers
CPU side: reads 64-bit values
```

The exact design can come later. The architecture principle is that different-clock families should communicate through a boundary with known behavior.

The design principle:

```text
communication inside a family can be specialized
communication between families should be explicit and well-bounded
```

This creates a hierarchy:

```text
within-family communication
between-family communication
whole-system communication
```

Each layer can have different speed, rules, and safety requirements.

So the design process should be:

```text
1. List communication methods
2. List each method's strengths and weaknesses
3. List workloads and jobs
4. Match each workload to a communication method
5. Then decide the compute groups
```

## Communication Options Shape Group Boundaries

```text
shared memory / ring buffer
- best for fast repeated data movement
- bad for complex shared truth

interrupts
- best for urgent attention
- bad for carrying large data

message passing
- best for commands and coordination
- bad for giant payloads unless paired with memory buffers

SQL/database-like layer
- best for durable shared state
- bad for nanosecond real-time work

same-clock pipeline
- best for predictable flow
- bad for flexible unpredictable workloads
```

## Design Rule

Do not start by asking:

```text
How many groups should we have?
```

Start by asking:

```text
What must move between parts of the system?
```

Then the groups become easier to see.
