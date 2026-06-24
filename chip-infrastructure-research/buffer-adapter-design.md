# Buffer Adapter Design

This note is for the communication buffer group that sits between different compute group families.

Example:

```text
GPU Group Family -> Buffer/Adapter Group -> CPU Group Family
```

The adapter exists because the two sides may have different:

```text
clock rates
data widths
message formats
burst behavior
latency tolerance
```

## First Rule Of Thumb

Hypothesis:

```text
as long as the I/O buffer moves faster than both groups it connects,
it can keep communication predictable
```

This is a useful starting intuition.

But the more precise version is:

```text
the buffer must have enough throughput and depth for the producer and consumer
```

Clock speed alone is not enough.

The adapter's useful speed depends on:

```text
clock rate
bits moved per clock
how often it can accept input
how often it can produce output
how deep the buffer is
whether traffic arrives smoothly or in bursts
```

## Throughput Formula

```text
throughput = data width * transfer rate
```

Example producer:

```text
GPU output width = 32 bits
GPU output rate  = 100 MHz
```

Then:

```text
32 bits * 100,000,000/sec = 3.2 gigabits/sec
3.2 gigabits/sec = 400 megabytes/sec
```

Example consumer:

```text
CPU input width = 64 bits
CPU rate        = 3 GHz
```

If the CPU can consume one 64-bit value every cycle:

```text
64 bits * 3,000,000,000/sec = 192 gigabits/sec
192 gigabits/sec = 24 gigabytes/sec
```

In that simple case, the CPU side is much faster than the GPU stream.

But real systems also need to ask:

```text
Can the CPU actually read every cycle?
Is the CPU busy doing other work?
Are there multiple GPU streams?
Does memory access slow the consumer?
Does the adapter need to pack 32-bit values into 64-bit values?
What happens during bursts?
```

## Width Conversion

If one side produces 32-bit values and the other side reads 64-bit values, the adapter may pack two producer values into one consumer value.

```text
GPU output:
[32 bits][32 bits][32 bits][32 bits]

adapter output:
[        64 bits        ][        64 bits        ]
```

This is not just storage. It is translation between two communication styles.

## Routing Question: Does The CPU Need The Data?

Before designing a buffer from one group back to the CPU, ask why the data is returning to the CPU.

Example:

```text
GPU Group -> CPU Group -> Display Group
```

If the CPU is only acting as a router, that may be unnecessary.

Maybe the better path is:

```text
GPU Group -> Display Group
```

Then the CPU only sends commands and receives status:

```text
CPU Group -> GPU Group: render this frame
GPU Group -> Display Group: finished frame data
GPU Group -> CPU Group: completion/status signal
```

This reduces traffic through the CPU and keeps large data close to the group that actually uses it.

The design rule:

```text
route data to the group that needs it,
not automatically back to the CPU
```

The CPU should coordinate when needed, but it should not become the default middleman for every data path.

This matters most for large outputs:

```text
frames
textures
audio buffers
AI tensors
sensor streams
storage blocks
```

For those, the CPU may only need:

```text
pointer
status
completion signal
error code
small metadata
```

The large payload can move directly between producer and consumer groups.

## Clock Rule Draft

A first draft rule:

```text
adapter clock should be fast enough that adapter throughput is greater than or equal to the highest required sustained throughput
```

More simply:

```text
adapter must not be the bottleneck during normal operation
```

If the adapter is slower than the producer, then the buffer fills.

If the adapter is slower than the consumer, then the consumer waits.

If the adapter has enough speed but not enough depth, bursts can still overflow it.

## Known Limits To Define

Every adapter group should have known limits:

```text
input width
output width
input clock
output clock
adapter clock
maximum sustained input rate
maximum sustained output rate
buffer depth
burst tolerance
latency
overflow behavior
underflow behavior
data ordering rule
valid/ready rule
```

## Open Questions

```text
Should each group family have its own adapter type?
Should adapters be same-clock with one side or have their own faster clock?
How should overflow be handled: stall, drop, compress, or signal error?
When should the adapter copy data versus pass pointers?
How does this compare to CPU/GPU communication today?
```
