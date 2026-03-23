# mini-hypervisor
 
A bare-metal x86 execution environment built on KVM in Rust. No OS, no bootloader, no abstractions — just raw machine code running on a virtual CPU.
 
## What It Does
 
Allocates a page of physical memory, writes raw x86 machine code into it byte by byte, boots a virtual CPU, and executes the code. The current guest program is seven bytes:
 
```asm
0xB0 0x48       → mov al, 0x48        ; load ASCII 'H' into al
0xBA 0xF8 0x03  → mov dx, 0x3F8       ; COM1 serial port address
0xEE            → out dx, al          ; write 'H' to serial port
0xF4            → hlt                 ; halt the CPU
```
 
That's a complete operating system in seven bytes. It boots, writes a character to a serial port, and halts.
 
## Why
 
I learn by going to the bottom. Most ML engineers interact with hardware through five layers of abstraction. I wanted to understand what actually happens when a CPU executes an instruction — how memory gets mapped, how registers get initialized, how the machine transitions from "powered on" to "running code."
 
This is the foundation for a series of experiments:
 
- Implementing matmul algorithms in raw x86 and benchmarking them against each other on bare metal
- Exploring how different instruction choices (scalar vs SIMD vs AVX-512) affect real cycle counts with zero OS overhead
- Eventually running a minimal transformer — just matrix multiplications and softmax — with no framework, no library, no operating system between the math and the metal
 
## How It Works
 
1. Creates a KVM virtual machine with a single vCPU
2. Allocates page-aligned guest physical memory
3. Writes raw x86 instructions directly into that memory
4. Configures the vCPU: code segment base at 0, instruction pointer at 0, real mode
5. Runs the vCPU in a loop, handling I/O exits and halt
 
## What's Next
 
Replacing `alloc_zeroed` with `mmap` for proper page-aligned memory mapping. Then expanding the guest code to perform actual arithmetic — starting with scalar matrix multiply, working up through SIMD, and measuring everything.
 
## Built With
 
Rust · `kvm-ioctls` · `kvm-bindings` · Linux KVM
