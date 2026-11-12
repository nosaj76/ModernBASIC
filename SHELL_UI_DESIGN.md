# Shell UI Design — Frame Styles and Panels

## Window Frame Styles

### Command Terminal (boot surface)
- NO window manager frame (no title bar, no close button, no minimize/maximize)
- Override-redirect window: XSetWindowOverrideRedirect(display, window, True)
- This tells the WM to leave it alone — no decorations, no border
- Full screen at boot
- Internal layout (top to bottom):
  ┌─────────────────────────────────────────┐
  │ HARDWARE REPORT PANEL                    │
  │ Display: 1920x1080  CPU: Ryzen 7 5700G   │
  │ Memory: 14GB  Audio: PipeWire            │
  ├─────────────────────────────────────────┤
  │ TASK MANAGER PANEL                       │
  │ ID  FILE              STATE    WINDOW    │
  │  1  painting_demo.bas  RUNNING  800x600  │
  │  2  music.bas          IDLE    640x480   │
  ├─────────────────────────────────────────┤
  │ COMMAND PANEL                            │
  │ READY.                                   │
  │  LOAD "painting_demo.bas"                │
  │  RUN                                     │
  │  >_                                      │
  └─────────────────────────────────────────┘

### Program Windows
- Standard window manager frame (title bar with close/minimize/maximize)
- Title: "ModernBASIC - Task N - filename.bas"
- User can resize, move, minimize, close via WM controls
- Closing via WM close button = CLOSE command (clean shutdown)
- Each window is scalable and independent

## X11 Implementation

### Override-redirect for command terminal
```rust
// In X11Shell::open, for the command terminal window:
let wm_override_redirect = intern_atom(display, "WM_OVERRIDE_REDIRECT");
// Set override-redirect so the WM doesn't decorate it
unsafe {
    XChangeProperty(display, window, wm_override_redirect, 
                    XA_INTEGER, 32, PropModeReplace, &1, 1);
}
// Or use XSetWindowAttributes with override_redirect = True
```

### Program windows stay normal
Program windows use normal XCreateSimpleWindow — the WM handles
decoration, focus, stacking. They get title bars and close buttons.

### Command terminal escape (development only)
- Esc — minimizes the command terminal to a small window in the corner
- Press Esc again to restore fullscreen
- This lets the user interact with program windows when the command terminal is in the way
+- NOTE: This is development-only. In production, the Linux kernel IS the hardware.
++  It provides X11 (graphics), ALSA/PipeWire (audio), input, USB, network, filesystem.
++  Modern64BASIC is PID 1 — the entire userspace. It is both the X11 client and the
++  window manager. No separate desktop environment, no systemd, no GNU userspace.

## Command Terminal Internal Layout — Three Columns

### Layout: command column left, hardware+task stacked on the right
┌────────────────────────────┬──────────────────┐
│ COMMAND                    │ HARDWARE         │
│                            │ Display: 1920x1080│
│ READY.                     │ CPU: Ryzen 7     │
│  LOAD "painting_demo.bas"  │ RAM: 14GB        │
│  RUN                       │ Audio: PipeWire  │
│  >_                        ├──────────────────┤
│                            │ TASK MANAGER     │
│                            │ ID  FILE    STATE│
│                            │  1  demo.bas RUN │
│                            │  2  music.bas IDL│
│                            │                  │
│                            │                  │
└────────────────────────────┴──────────────────┘

Column widths:
- Command: 65% (left, full height — needs space for typing, history, output)
- Right side: 35% (split top/bottom)
  - Hardware: top portion (static info, compact)
  - Task Manager: bottom portion (live, grows with tasks)

### Command Column (left, 65%, full height)
- NO READY. prompt — this is the OS shell, not a BASIC prompt
- NO RUN — programs run in their own windows
- Shell-level commands only:
- LOAD <file> — open a program in a new window
- NEW <file> — create a new blank program file and open it
- DELETE <file> — delete a file from disk
- CLOSE <file> — close a program window (also via window close button)
- FOCUS <file> — bring a program window to the front
- TASKS — list running files
- DIR — show directory contents
- CD — change directory
- RESTART — reboot the environment (asks "ARE YOU SURE? Y/N")
- SHUTDOWN — power off (asks "ARE YOU SURE? Y/N")
- Clean, minimal — like a Linux boot console

### Program Windows (separate X11 windows, each with full BASIC REPL)
Each program window has its own command area with the full BASIC experience:
- READY. prompt (inside the program window, not the shell)
- RUN [number] — execute the program
- LIST [range] — show program source
- EDIT — enter line navigation mode
- SAVE "file.bas" — save the program
- STEP [count] — single-step debugging
- CONT — continue after STOP
- RENUM — renumber lines
- Direct mode: type line numbers to add code, type expressions to evaluate
- PRINT output goes to this window
- Graphics output goes to this window
- Video playback goes to this window
- The user can have multiple programs open, each in its own window, each with its own READY. prompt and command history

## Drawing the Panels
The command terminal draws its own panel borders using X11 primitives:
- XFillRectangle for panel backgrounds (dark theme)
- XDrawLine for panel separators
- XDrawString for panel text (hardware info, task list, prompt)
- No external UI toolkit — pure X11 drawing