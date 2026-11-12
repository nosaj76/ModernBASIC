# ModernBASIC environment architecture

The deployment target is an independent programming appliance built on Debian 13
for x86 workstations, lightweight RISC-V, and general-use ARM64, including Pi 5.
Debian supplies the Linux kernel, hardware drivers, and system services;
ModernBASIC supplies the operator-facing programming environment. These components
run together on the device. The kernel does not contain the BASIC interpreter.

The runtime can still be developed as a native Linux application. The older x86
VM image builder uses build-machine components; it is not the Debian deployment.
See [Debian deployment profiles](docs/DEBIAN_DEPLOYMENT.md) for package construction and
pending hardware validation, and [appliance boot](docs/APPLIANCE_BOOT.md) for the
service and privilege boundary.

## Product direction

The owner defines ModernBASIC as a contemporary Dartmouth BASIC/TI-BASIC-style
programming environment with the integrated creative reach of an Amiga-like
computer on modern hardware. The advanced mathematical engine is a primary
foundation: Rust implements numerical and symbolic computation, with statements
and functions usable directly in BASIC programs. Graphics, audio, animation,
simulation and general application facilities build on the same native backend
and composable data APIs.

Acceptance must exercise useful complete workloads rather than count commands:
scientific computation and visualization; mathematical signal generation,
filtering and sound; tensor learning and image processing; interactive animation
and simulation; persistent user applications and recovery. Native algorithms,
API behavior, source editing, standalone execution and eventual OS/hardware
qualification are distinct parts of that contract. This direction is a product
target, not a claim that every historical or modern facility is already complete.

## Shared mathematical backbone

The owner's requirement is one coherent Rust mathematical foundation for physics
simulation/engines, scientific research, analysis, finance, AI/ML, graphics and
music/sound/audio. Shared typed kernels and data structures underpin domain
engines; BASIC statements/functions expose them for freely written user programs.
The native kernel layer should be reusable without a running BASIC interpreter.
Real-time audio kernels require suitable allocation/lifetime behavior rather than
constructing interpreter value lists for every sample.

Cross-domain numerical correctness, useful workload sizes, conditioning,
precision, convergence, memory, performance and cancellation are acceptance
requirements. Examples exercise the engines but do not replace missing native
algorithms. Python reference libraries remain development-test tools, not a
runtime implementation of the math engine.

## Project responsibility and version boundary

Owner clarification, 2026-10-22: Linux provides the operating-system foundation;
Rust implements the ModernBASIC interpreter/runtime and backend engines. Advanced
mathematical algorithms, numerical kernels and their resource handling belong in
Rust, exposed through BASIC statements/functions. BASIC examples and applications
exercise those APIs; they do not substitute for implementing the requested engine.
User-defined equations or callbacks supply a model to a native solver, rather than
moving the solver implementation into BASIC.

ModernBASIC backend release versions track the Rust side. Users may create, edit,
run and save their BASIC programs freely, without backend version increments,
release registration, source hashes or source freezing being imposed on their work.
User program compatibility is an API responsibility of the backend. Users can
choose their own application versioning independently.

Build-input checksums and distribution manifests establish which files an artifact
contains; they are distinct from the backend version. Existing build inventories
also cover bundled BASIC environment components and examples. These packaging
checks must not become version controls or edit restrictions on user BASIC files.
Linux dependency/package identities remain useful operational metadata.

## Runtime layouts

The executable supports two front ends over the same interpreter:

- The terminal commands (`run`, `repl`, `eval`, and `graph`) execute in the
  invoking ModernBASIC process.
- The graphical shell is one X11 client process. Its hub window owns a program
  window manager. Each open program window has an interpreter worker thread
  and, after graphics presentation, a scene-rendering worker thread, connected to the X11 event thread by
  bounded or ordinary Rust channels.

The Program Manager also lists the separate Bash administration terminal as
`BASH`. This row has its own identity and routes focus/close to the terminal;
it does not allocate a BASIC interpreter or reuse a numeric program task ID.
The terminal and its shell/jobs run in the appliance's separate managed root
service. BASIC remains unprivileged. Closing Bash disconnects its terminal's
X11 client so the service can reap the session and its background jobs. Focus
and resize check the live embedded window under the X server lock, avoiding
stale window IDs when the terminal exits between polling and queued events.

New windows open fully on-screen. Manual moves can leave up to half of each
frame dimension outside the desktop; resizing still bounds the window's size
to the desktop. Alt+drag can recover a window whose title bar is off-screen.

The shell relies on an existing X server and window manager for display,
decorations, focus, movement, resizing, and stacking. `RESTART` restarts the
application. In the Debian appliance, confirmed `SHUTDOWN` exits with the
reserved status 85; the service then requests host power-off. Outside appliance
mode, `SHUTDOWN` closes the application. Crashes and ordinary service stops do
not request host power-off.

## Program task behavior

`LOAD` and `NEW` open program windows. `TASKS` reports the public `READY` or
`RUNNING` state; a window also tracks internal ready, running, stopped, and
error presentation states. `STOP` requests cooperative interruption. `CLOSE`
terminates the task: it uses the task-local interrupt only to unblock current
work, queues `QUIT`, destroys the X11 window immediately, and disconnects its
bounded event receiver. Worker threads are joined by a separate reaper so the
shell event loop remains responsive. When the interpreter worker exits, its
program, variables, arrays, prepared assets, and other task RAM are dropped.

Ctrl+Escape in either a program window or the shell hub requests BREAK on the
selected running task, falling back to another running task when the selection
is idle. Each task has its own atomic interrupt flag; the request also wakes a
blocked input channel. Statement, wait, media-frame, and presentation
checkpoints return the task to its prompt without terminating the shell. Native
and SoundFont synthesis, WAV mixing/encoding, silent-rest construction, speech
waits, and video finalization check the same flag in bounded chunks.
BREAK stops task-owned playback, speech, decoders, and unfinished encoders, but
it does not remove the task or window and does not clear BASIC RAM, the loaded
program, prepared assets, or queued synthesis data.

Each interpreter owns its variables, arrays, graphics scene, audio arrangement,
file channels, session clock, and control stacks. Some immutable or coordinated
resources are process-wide, including the parsed default SoundFont cache and
the advisory file-lock registry.

These tasks are threads in one address space, not protected processes. Rust
ownership and the runtime APIs provide logical state separation, but there is
no hardware memory boundary between program windows. A handled BASIC error or
worker-thread panic can be reported without necessarily stopping other tasks;
a native memory fault can terminate the complete ModernBASIC process. The
project therefore does not claim segfault containment.

## Resource bounds

Modern mode starts with explicit limits, including:

- 100,000 executed statements per run;
- call and loop depth of 1,000, and function depth of 100;
- 64 MiB of retained BASIC value payloads;
- 1,000,000 array elements and 1,000,000 `DATA` items;
- 16 concurrently open BASIC file channels;
- 512 MiB per ordinary file and per SoundFont asset;
- 16 MiB per approved network response;
- 1 MiB of supplied input across at most 100 lines; and
- a 512-event bounded interpreter-to-window queue.

Individual graphics, audio, video, source-line, listing, string, and matrix
operations have additional limits. These are language/runtime budgets, not a
256 MiB process heap limit. The host allocator, framebuffers, decoded media,
SoundFont samples, renderer data, and thread stacks are not all charged to the
64 MiB retained-value counter.

Limit violations return BASIC/runtime errors where the checked operation can do
so. They cannot promise recovery from host out-of-memory termination or native
undefined behavior.

## Files and capabilities

BASIC file statements are disabled until the host supplies a file root. Once
enabled, relative paths are resolved beneath that canonical root;
absolute paths, parent traversal, ambiguous case-insensitive matches, and
symlinks are rejected. A current directory may move only within the same root.
Generated files can use a separately configured output root. The root-level
`soundfont/` prefix can address a separate read-only SoundFont asset root.

A stored program has a stable asset root anchored to its directory. LOAD validates
the execution plan; image, SoundFont, and media inputs load only when an operation
needs them. Successful first use retains the input for subsequent runs. Asset
errors are execution errors. Program replacement releases old assets and optional
capabilities. Runtime CD changes only the ordinary file directory. Generated
raster and WAV outputs are registered in memory for later consumers.

The shell's file commands are likewise rooted in its configured program tree;
they do not grant unrestricted host filesystem administration. `CHAIN` is an
interpreter handoff with `COMMON` values, not inter-process shared memory.
Advisory `LOCK`/`UNLOCK` coordinates file channels inside the process and with
the host's file-lock mechanism where implemented.

Ambient process arguments and environment variables are not exposed by
default. The host must explicitly supply approved `COMMAND$` and `ENVIRON$`
values. Network statements are disabled by default and, when enabled, accept
validated HTTP(S) operations with fixed tool arguments and bounded responses.

## Host services

ModernBASIC implements the language, software rasterizer, X11 client, native
synthesizer, SoundFont renderer, and bounded orchestration. It delegates:

- hardware, filesystems, processes, and networking to Linux;
- display and input delivery to X11 and the window manager;
- live PCM playback to `paplay` or `aplay` when installed;
- compressed media decoding/encoding to FFmpeg and FFprobe;
- approved HTTP transfer to `curl`;
- browser opening to `xdg-open`; and
- optional speech to eSpeak NG or eSpeak.

Child media/network processes are invoked without a shell and with fixed
arguments. Decoder processes are killed and reaped if parsing, streaming, or a
consumer callback returns early.

## Security boundary

The sandbox and capability checks reduce accidental or BASIC-level access; they
are not a hostile-code security boundary. All tasks share the privileges of the
ModernBASIC host process. Running untrusted BASIC safely requires an external
OS sandbox, container, virtual machine, or separate unprivileged account in
addition to the runtime's own limits.


Bash defaults to a 640×480 client surface. Configure notifications are treated
as notifications rather than authoritative geometry: the integration queries
current server dimensions and resizes the embedded terminal only when those
dimensions change. Position-only edge corrections use XMoveWindow so they do
not reissue sizes while the window manager processes a resize sequence.
