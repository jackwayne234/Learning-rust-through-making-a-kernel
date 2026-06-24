# Monitoring Layer

This note is for lightweight monitoring inside the chip-level infrastructure.

The goal:

```text
observe hard failures without slowing normal communication
```

## Passive Link Taps

A passive link tap watches a communication path without sitting in the main data path.

Example:

```text
GPU Group -> Display Group
          \
           -> Passive Monitor Tap
```

The main payload still moves directly:

```text
producer -> consumer
```

The monitor only observes enough information to detect problems.

It should not be required for the transfer to complete.

## What The Monitor Can Watch

The monitor might watch:

```text
link alive/dead
clock present/missing
buffer full too long
buffer empty too long
error signal
retry count
temperature warning
voltage warning
checksum or parity error
completion timeout
unexpected silence
```

## Failure Broadcast

When something is down, the monitor can send a lightweight signal or broadcast.

Example:

```text
Monitor -> System Health Group: GPU-to-display link down
Monitor -> CPU Group: buffer overflow warning
Monitor -> Power Group: clock domain unstable
```

The alert should be small:

```text
source
failure type
timestamp or tick count
severity
optional small detail code
```

The monitor should not send the full payload unless specifically requested.

## Hot Path Rule

The monitor must not slow down normal communication.

Core rule:

```text
monitoring is out-of-band
```

Meaning:

```text
normal data path does not wait for monitor
normal data path does not route through monitor
monitor failure should not stop normal data movement
```

If monitoring becomes required for every transfer, then it is no longer passive. It has become part of the main communication system.

## Known Limits

Every monitor should have known limits:

```text
what it watches
what it ignores
maximum alert rate
alert format
where alerts go
what counts as failure
what counts as warning
whether it can be reset
whether it can be disabled
```

## Design Principle

```text
data paths carry work
monitor paths carry health
```

The health system should make physical and communication failures visible without taking ownership of the actual work.
