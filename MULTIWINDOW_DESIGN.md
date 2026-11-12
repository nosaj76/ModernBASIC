// Multi-window architecture design for Modern64BASIC shell
// ============================================================
// 
// GOAL: Command terminal at boot + separate scalable windows for each
// loaded program. Each program runs in its own interpreter thread with
// its own graphics context.
//
// ARCHITECTURE:
//
// 1. WindowManager (owns the X11 display, event loop)
//    ├── CommandTerminal (the boot window, system commands)
//    └── ProgramWindow[] (one per loaded program)
//        ├── Window ID
//        ├── Interpreter thread handle
//        ├── Graphics context (own raster, own SCREEN dimensions)
//        ├── Input/Output channels
//        └── State: IDLE | RUNNING | STOPPED | ERROR
//
// 2. Event dispatch:
//    XNextEvent → event.xany.window → match against:
//      - command terminal window → handle command input
//      - program window N → route key/mouse/resize to that program
//
// 3. Command terminal commands:
//    LOAD "file.bas"  → open new ProgramWindow, load source
//    RUN [task_id]    → run the program in its window
//    TASKS            → list all open program windows
//    CLOSE [task_id]  → destroy program window, stop interpreter
//    FOCUS [task_id]  → raise a program window to front
//    LIST [task_id]   → list program source in command terminal
//    SAVE [task_id]   → save program source
//    NEW              → open a fresh empty program window
//
// 4. Program window lifecycle:
//    OPEN   → XCreateSimpleWindow, XMapWindow, spawn interpreter thread
//    RUN    → send "RUN" to interpreter via input channel
//    STOP   → send Ctrl+Esc interrupt to interpreter
//    CLOSE  → XDestroyWindow, join interpreter thread, remove from table
//
// 5. Per-program isolation:
//    Each ProgramWindow has:
//    - Own Window ID (X11)
//    - Own GC (graphics context)
//    - Own raster buffer (Option<RasterImage>)
//    - Own SCREEN dimensions (width, height)
//    - Own interpreter thread with thread-local screen sender
//    - Own input channel (Sender<Vec<u8>>)
//    - Own output channel (Receiver<ReplEvent>)
//    - Own screen event channel (Receiver<ScreenEvent>)
//
// 6. Task/Process Manager:
//    HashMap<task_id, ProgramWindow>
//    task_id = incrementing integer (1, 2, 3, ...)
//    TASKS command prints: ID  FILENAME              STATE
//                          1   painting_demo.bas     RUNNING
//                          2   music.bas             IDLE
//
// IMPLEMENTATION PLAN:
// Phase 1: Extract per-window state into a ProgramWindow struct
// Phase 2: Make X11Shell hold a HashMap of ProgramWindows  
// Phase 3: Command terminal stays as the main window
// Phase 4: LOAD creates a new ProgramWindow
// Phase 5: RUN sends to the right interpreter
// Phase 6: Event dispatch by window ID
// Phase 7: Task manager (TASKS, CLOSE, FOCUS commands)