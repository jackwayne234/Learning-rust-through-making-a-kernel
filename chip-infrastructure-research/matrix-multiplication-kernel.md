# Matrix Multiplication Kernel

Decision:

```text
the first specialized kernel target is a matrix multiplication kernel
```

This means the first real "job kernel" should be designed around multiplying matrices, not around being a general-purpose operating system.

## Why This Kernel

Matrix multiplication is a good first target because it has clear inputs, clear outputs, and real architecture pressure.

It forces us to think about:

```text
data movement
buffer design
fixed-size inputs
output format
parallel work
producer/consumer routing
monitoring
clock and throughput limits
```

It also connects back to the optical computer work, where the data showed that wavelength selection can help with symbolic logic, but matrix multiplication eventually needs accumulation.

## First Tiny Version

Start with the smallest useful case:

```text
2x2 matrix A
2x2 matrix B
2x2 matrix C = A * B
```

Formula:

```text
C00 = A00*B00 + A01*B10
C01 = A00*B01 + A01*B11
C10 = A10*B00 + A11*B10
C11 = A10*B01 + A11*B11
```

This is small enough to understand completely.

## First Bare-Metal Goal

The first bare-metal version can use fixed input values compiled into the kernel.

Example:

```text
A = [1 2]
    [3 4]

B = [5 6]
    [7 8]

C = [19 22]
    [43 50]
```

The first version does not need dynamic memory, files, user input, or a database.

It only needs:

```text
fixed input data
matrix multiply procedure
fixed output storage
some way to observe the result
```

## Observation Problem

The kernel still needs a way to show or expose the result.

Options:

```text
write result to screen memory
write result to a known memory address
halt after computation so a debugger/emulator can inspect memory
later: send result through a ring buffer to another group
```

The cleanest next practical step is probably:

```text
make the kernel write text to the screen,
then print the matrix result
```

## Group Architecture Version

Later, this can become a specialized compute group:

```text
CPU/Coordinator Group -> Matrix Kernel Group: input matrices
Matrix Kernel Group -> Output Buffer: result matrix
Matrix Kernel Group -> Monitor Layer: status/health
```

Large data should not automatically return through the CPU unless the CPU needs it.

Possible better path:

```text
Input Buffer -> Matrix Kernel Group -> Result Buffer -> Consumer Group
```

The CPU may only need:

```text
command
status
completion signal
error code
```

## Communication Questions

Before scaling the matrix kernel, define:

```text
input width
output width
matrix size
number format
buffer depth
expected throughput
latency tolerance
overflow behavior
completion signal
monitoring signals
```

## Grid Size To Kernel Ratio

Important research question:

```text
what is the most efficient ratio of matrix grid size to kernel control?
```

For CPU groups, the question might look like:

```text
how many CPU cores should one kernel control?
```

For a matrix multiplication chip, the better question is:

```text
how large of a matrix tile/grid should one kernel control?
```

Possible ratio language:

```text
1 kernel : 1 matrix tile engine
1 kernel : 16x16 compute grid
1 kernel : 32x32 compute grid
1 kernel : 64x64 compute grid
1 kernel : 100x100 compute grid
```

The right ratio depends on whether the kernel spends too much time coordinating instead of letting the grid compute.

## Coordination Overhead

The kernel should not become the bottleneck for the matrix grid.

Questions to test:

```text
Can the kernel feed inputs fast enough?
Can it start jobs fast enough?
Can it collect or route results fast enough?
Does it spend too much time scheduling?
Does it spend too much time handling completion signals?
Does the grid sit idle waiting for commands?
Does the output buffer fill before the kernel routes results?
```

If the grid is too small, the kernel may spend too much time coordinating many tiny jobs.

If the grid is too large, it may be hard to feed with enough data, and parts of the grid may sit idle.

The goal is a balanced point:

```text
kernel coordination overhead is small
grid utilization is high
input buffers stay fed
output buffers drain predictably
```

## Ratio Hypothesis

A first hypothesis:

```text
1 kernel : 1 matrix tile engine : 32x32 compute grid
```

This is not a final answer.

It is a starting point to compare against:

```text
16x16
32x32
64x64
100x100
```

The best ratio should be discovered by modeling:

```text
coordination time
compute time
input bandwidth
output bandwidth
buffer depth
power and heat
idle time
```

## First Ratio Test: Perfect Grid Assumption

For the first model, assume the matrix grid has perfect uptime.

Assumption:

```text
the entire matrix grid is available 100% of the time
the grid itself does not fail
the grid itself does not stall internally
the grid is ready whenever the kernel gives it enough work
```

This lets us isolate one question:

```text
at what grid size does the kernel coordination become unmanageable?
```

In this model, if performance falls apart, blame the control path first, not the compute grid.

## Kernel Coordination Budget

The kernel must do coordination work such as:

```text
receive command
validate command
locate input buffers
configure grid job
start grid job
watch or receive completion signal
route output location
update status
accept the next job
```

The important comparison:

```text
kernel coordination time vs grid compute time
```

If:

```text
kernel coordination time << grid compute time
```

then the kernel is not the bottleneck.

If:

```text
kernel coordination time ~= grid compute time
```

then the kernel is starting to matter.

If:

```text
kernel coordination time > grid compute time
```

then the kernel can become unmanageable because it cannot keep the grid fed efficiently.

## Simple Model

Define:

```text
G = grid size, such as 16, 32, 64, or 100
K = kernel coordination time per job
C = grid compute time per job
U = useful grid utilization
```

First rough utilization model:

```text
U = C / (C + K)
```

Examples:

```text
if K is 1% of C, utilization is about 99%
if K is 10% of C, utilization is about 91%
if K is 50% of C, utilization is about 67%
if K equals C, utilization is about 50%
```

Working threshold:

```text
kernel coordination should stay under 10% of grid compute time
```

So:

```text
K <= 0.10 * C
```

If kernel coordination takes more than that, the grid size or job batching may need to change.

## What This Test Will Tell Us

This model helps decide whether one kernel should control:

```text
16x16 grid
32x32 grid
64x64 grid
100x100 grid
```

If small grids finish too quickly, the kernel may spend too much time coordinating many tiny jobs.

If huge grids require too much setup, too much data movement, or too much output handling, the kernel may also become overloaded.

The best ratio is where:

```text
the grid has enough work to hide coordination cost
but not so much work that input/output handling becomes unmanageable
```

## Next Model Inputs To Estimate

To make this less hand-wavy, estimate:

```text
kernel command setup time
input buffer setup time
output routing time
completion handling time
grid compute time for 16x16
grid compute time for 32x32
grid compute time for 64x64
grid compute time for 100x100
```

Then compare each grid size using:

```text
U = C / (C + K)
```

## One-Bite Roadmap

1. Write one character to the screen.
2. Print a small number.
3. Compute fixed 2x2 matrix multiplication in the kernel.
4. Print the 2x2 result.
5. Move the input matrix into a known memory buffer.
6. Move the result into a known output buffer.
7. Add a completion/status flag.
8. Turn the input/output into a simple message format.
9. Later, scale beyond 2x2.

## Design Principle

```text
start with the smallest matrix that proves the communication path
```

The real lesson is not only multiplication.

The real lesson is:

```text
how data enters a compute group,
how work happens,
how the result leaves,
and how the rest of the system knows it is done
```
