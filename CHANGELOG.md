# Changelog

## Consolidated revision 1 — October 22, 2026

- Consolidate the current tested master and installer display correction into
  one revision 1 baseline. Retire older release artifacts and source forks only
  after replacement validation and preservation of user programs/settings.
- Keep runtime version 0.8.2; use Debian epoch 1 so the numbering reset remains
  newer than legacy revision 18 for package updates.

## Unreleased — reliability and release gates

- Extend CPU SPCG incomplete-Cholesky preconditioning from IC(0) to bounded
  symbolic fill levels IC(0) through IC(8). Keep JACOBI as the default and cap
  combined triangular-factor storage at one million entries.
- Add optional CPU windowed-sinc alias suppression to `RESAMPLE`; keep linear
  interpolation as the default and enforce the shared math work limit.
- Export a typed `NativeSincResampler` block API with constructor-reserved CPU
  buffers, rational phase, chunk-invariant state, atomic flush and bounded work.
- Precompute bounded exact-phase CPU polyphase coefficients for common
  rational audio rates, retaining the checked fallback for larger phase sets.
- Render live native synth and WAV output through bounded caller-owned stereo
  blocks, reusing mono and PCM scratch across writes.
- Add CPU `IDAHYBRID` for residual DAE crossing events, direction filters,
  indexed reset callbacks and supported index-one consistency restoration.
  Bound reset callback work and event records; simultaneous resets compose in
  index order. Focused tests cover dense and preconditioned SPGMR trajectories,
  projection, atomic reset failures and close sequential root crossings.
- Add the editable `thermal_hysteresis_ida.bas` CPU DAE workload; its six heater
  switches and algebraic heat-transfer states are checked against analytic
  heating/cooling intervals.
- Add opt-in `IDASOLVE(...,init$="AUTO")` for CPU index-one initial-condition
  correction using the selected dense, SPGMR or KLU solver. Omitted or
  `"SUPPLIED"` initialization preserves the existing strict default.
- Add CPU `IMPLICITEVENTS` for nonterminal vector events on adaptive backward-
  Euler trajectories, including optional analytic Jacobians and direction
  filters. Extend opt-in tangency detection to it; caller-supplied event-value
  tolerance localizes smooth same-sign minima on accepted steps. Limits and
  multiple-root constraints are documented and independently checked.
- Add bounded `eventscan` subdivisions to terminal and nonterminal explicit and
  implicit event APIs. Terminal solvers stop at the first bracketed root; vector
  solvers localize multiple detected crossings on the original dense interpolant.
  Optional tangency minimization also runs per scan interval when `tangenttol` is set.
- Add optional CPU `eventdepth` refinement to explicit and implicit terminal,
  vector and hybrid event APIs. Each level bisects configured scan intervals;
  combined work is capped at 4096 intervals per accepted step and default scan
  behavior remains unchanged.
- Add CPU `ODEHYBRID` and `IMPLICITHYBRID` for bounded scalar-guard state resets.
  Event records preserve pre/post-reset states at a shared timestamp, and the
  explicit six-impact ball workload now uses native event restarts instead of a
  fabricated epsilon-time advance. Finite event scans and interpolation limits
  still apply.

- Keep the windowed graphical installer within small displays using a bounded
  primary-display probe and per-process Qt scaling. Accept the real window
  frame bounds and avoid early-exit grep pipelines in installer launch checks.
  Revision 18 graphical BIOS installation passed; this layout correction is
  development source and still requires a rebuilt image.

- Build a private revision 18 workstation candidate from one frozen source;
  retain package/image identity and disposable VM acceptance evidence. Correct
  ISO validation to extract the tracked ALSA restore rule before comparison;
  cover that path in the ISO fixture regression.

- Add Ed25519-signed package-pair update metadata, safe administrator-owned
  public-key loading, expiry/revocation checks and retained authorization.
  A configured key store requires signatures for forward updates without
  blocking rollback/recovery. Add an offline publisher tool; no production key
  is generated or installed. Legacy checksum mode remains explicitly warned.

- Harden root administration requests: atomically publish private window-ID
  markers and validate held nonblocking, non-following descriptors before root
  reads them. Reject wrong owners, unsafe modes, links, special files and
  replacements during validation. Retain the trusted-operator security model;
  this does not isolate hostile applications from the shared root X11 terminal.

- Add separate per-session BASIC prompt recovery snapshots, AUTOSAVE controls
  and explicit RECOVER/EDIT discovery. Preserve unfinished drafts and originals;
  report checkpoint failures without blocking editing. Unhandled execution
  errors retain bounded statement, procedure and library context with hints for
  common mistakes, while preserving Error values and ON ERROR semantics.

- Require the appliance target, audio supervisor and a responding private audio
  server before committing desktop updates. Clear audio service start limits
  when restarting after installation or rollback.

- Add repeatable installed-system lifecycle acceptance on a new disposable disk:
  normal firmware boot, token-bound saved-state checks, supervised service
  recovery, offline package commit/rollback and guest-confirmed shutdown.
  Persistence verification now fails on missing files instead of recreating them.

- Keep virtual PCI audio outputs awake between sounds to avoid suspend/reopen
  glitches, preserve exact failed-service names in diagnostic reports, repair
  Debian's duplicate ALSA restore-rule label without overriding local rules,
  and exercise actual program-window bounds in live close/audio acceptance.

- Stream native music from queued note events instead of rendering complete
  tracks and a WAV before playback. Keep envelopes, oscillator phases, filters,
  waveguides and per-note settings across bounded audio blocks. Stream mixed
  native/SoundFont scores through the same output process, add a bounded device
  buffer, and retain WAV generation for explicit export. Check long-score
  startup, PCM equivalence, and paced playback continuity.
  Keep appliance/player gain at unity; save loudness corrections in BASIC
  scores. Use a configurable 250 ms device buffer for VM scheduling jitter,
  with regression checks for literal argument forwarding and session overrides.

- Add broadcast batched `TMM` and differentiable `TCONV1D`/`TCONV2D` with
  explicit stride, symmetric padding and dilation. Bound forward/backward work,
  retain gradients on failed backward passes, and validate against independent
  NumPy contractions and finite differences. Connect numeric lists and DIM-array
  snapshots directly to `WAVE`; add image/geometry, music and learned-filter
  applications plus a concrete math completion roadmap.

- Add `SYMREFINE` for conditional real simplification with exact sign deductions,
  square-root branches, inverse compositions and contradiction checks. Preserve
  original domains and explicit assumptions. Require suitable neighborhoods for
  calculus on guarded results, so a pointwise equality cannot create a false
  derivative. Add independent reference checks and a BASIC example.

- Extend `SYMANTIDERIV` to polynomial-weighted exponential–trigonometric products
  and integer powers/products of affine sine and cosine functions. Use exact
  harmonic coefficients, checked conjugate pairing and polynomial coefficient
  recurrences; retain source domains and exceptional parameter conditions.
  Add generated independent reference checks, exact SymPy derivative identities,
  resource-limit regressions and a BASIC example.

- Fix modern-mode `SUM` losing cancellation residuals or overflowing before a
  finite final answer. List, DIM-array, indexed, complex and broadcast reductions
  now accumulate exactly before final rounding. Integer-only results remain
  checked integers; legacy mode retains its previous behavior. Extend independent
  Fraction-reference checks and BASIC integration regressions.

- Prepare revision 16 for the expanded math engine, including exact symbolic
  certificates, CPU tensor/autograd, sparse linear solvers and adaptive implicit
  integration. Allow the Debian build guest to recover from an official standard
  live seed when previous temporary BASIC build images are unavailable. Install
  BLAS/LAPACK and speech/media test dependencies inside the Debian builder.
  Source qualification does not imply image or physical-hardware acceptance.

- Fill selected classic capability gaps with runtime EVAL, procedure libraries,
  indexed files, persistent debugger stops, same-session output windows, native
  menus, ON BREAK, TRANSLATE$, palette recoloring, PATTERN, LINPUT USING,
  sprite collision masks/borders and autonomous 2D animation. Retain bounded
  resources and document modern syntax and remaining parity limits. Validate
  with 94 explicit headless checks and seven isolated native X11 checks;
  diagnostics were not run. Evidence: `output/classic-gap-implementation-2026-10-12/`.

- Add a manual-linked classic BASIC capability audit with 51 selected
  requirements, six independent BASIC examples and a reproducible runner for
  60 focused checks. Correct stale parity claims; record missing/partial
  features and distinguish mock/headless evidence from hardware acceptance.
  That baseline audit changed no runtime behavior; its evidence remains
  separate from the subsequent gap-filling implementation.

- Prevent HTTPS HTTP-client requests from following plaintext redirects; restore
  standard POST redirect methods and make transfer writes, reads and process
  completion interruptible with child cleanup and retained byte/time limits.
  Bound browser download filenames by UTF-8 bytes and report finalization errors
  per download instead of closing the browser. Add local HTTP/TLS, cancellation,
  size-limit and browser download regressions.

- Harden the in-development firewall broker against IPv6 zone identifiers
  containing rule syntax and slow requests that delay rollback processing. Add
  isolated policy, transaction, persistence and protocol regressions; firewall
  UI, service packaging and live-system acceptance remain unfinished.

- Invalidate release-test caches for embedded applications/helpers and require
  their runtime identity to match. Extract desktop launchers during ISO audits,
  repair the distro fixture, and exclude checkout Python bytecode from source
  identities while keeping installed-package inventories strict.
- Make ordinary COMPILE interruptible with compiler-tree cleanup, bounded
  diagnostics and resource limits; supply the user bus in both appliance modes.
  Forward the desktop workspace to BASIC Console and reject linked console roots.
  Add explicit private-preview selection independently of release acceptance.

- Add a desktop BASIC Console with the interactive programming prompt, shared
  editor workspace and Ctrl+Shift+C/V clipboard shortcuts. Add a visible Switch
  environment menu action and an explicit save-work confirmation for immediate
  switching; cancellation preserves the active session and startup preference.

- Remove the browser engine's unsupported AutomationControlled feature override,
  which displayed a warning banner in the revision 15 desktop preview. Preserve
  the private control pipe and Chromium security defaults; previous streaming
  results using that workaround require renewed acceptance.
  Ignore shortcuts received before the embedded engine's READY handshake so
  pressing Ctrl+L during startup cannot turn into a startup error.

- Prepare revision 15 private workstation packaging with an optional compiled
  desktop, persistent startup selection, complete embedded helper sources and
  desktop executable integrity checks. Default to direct BASIC and preserve user
  work. Exclude the development catalog from new images while retaining required
  embedded application sources. Add explicitly scoped private-image validation.
  Authenticate update health against the selected BASIC or packaged desktop
  executable and its matching window, retaining desktop identity for rollback.

- Share embedded-source rebuild detection between normal launches and acceptance
  tools, including desktop applications and Python helpers. Preserve authorized
  Linux application access when executing installed programs or startup components.
  Correct the Rust 1.74 source gate to allow unsafe extern functions while still
  rejecting unsafe extern blocks.

- Add a BASIC taskbar for the desktop's running app and program windows, including
  descendant windows, state indicators, hover labels, overflow pages and F6/arrow
  keyboard switching. Preserve the text-free resting desktop and existing dock.
  Ignore invisible dock controls in standalone apps; recover Files after a failed
  folder open without leaving the previous directory.

- Confine default bundled app binary launches to `~/ModernBASIC` instead of the
  process working directory. Create `Programs` without replacing user files and
  reject linked default workspace/Programs directories. Show BASIC-relative paths
  and a Browser downloads shortcut in Files, keeping browser internals out of its
  listing. Add a standalone Files launcher. Preserve explicit host workspace selection and ordinary compiled
  program defaults. Add regression coverage for blocked read/write/rename/delete,
  directory escape, inherited task roots and document preservation.

- Launch Files, Text Editor, Writer, Sheet and Slides in independent BASIC windows
  through `TASK APP`. Preserve other apps when a parent window closes; allow
  parent task views to manage descendants. Add standalone editor/Office launchers.
  Replace Office bitmap text with system fonts, measured carets, cell alignment
  and slide wrapping. Handle window-manager close requests with discard confirmation.

- Render vector application typography at the fitted window resolution, keeping
  text clear when a 1280-pixel interface runs on a smaller display, including
  buffered FLIP frames that also carry a logical-resolution bitmap. Add Noto Sans
  UI and Noto Sans Mono editor faces, larger controls, and fractional text metrics
  that keep carets and selections aligned during resizing. Preserve spaced font
  family names through scene serialization. Drain queued desktop typing before
  handling toolbar clicks so Save includes the final typed characters.

- Add a BASIC-drawn browser application with embedded Chromium pages, live tabs,
  persistent profile/bookmarks/history, download controls and explicit confined
  uploads. Add `WEB VIEW EMBED`, bounded tab actions and `TASK BROWSER` for desktop
  launching. Retain the legacy temporary-profile `WEB VIEW OPEN` API.

- Run BASIC programs from desktop Files or the editor with F5. Execute unsaved
  editor snapshots without overwriting files; keep each program in an independent
  window. Add a Programs view for switching, status, stopping and confirmed close,
  backed by bounded `TASK` commands in the linked runtime.

- Use modern system sans-serif in desktop menus and controls, and system
  monospace in the editor. Add antialiased Pango/Cairo Unicode typography through
  `FONT SYSTEM` and matching `TEXTWIDTH` measurements for editor geometry.
  Keep portable built-in and bitmap fonts available to other BASIC programs.

- Replace the desktop landing-page layout with a text-free wallpaper surface,
  application icons and a compact dock. Add an active application menu with
  hover highlighting, keyboard navigation, application launching and confirmed
  exit; explanatory copy no longer fills the desktop.

- Make the desktop an explicit user choice: a bare interactive `run.sh` offers
  Desktop or No desktop, while `--desktop` and `--no-desktop` select directly.
  Direct BASIC and terminal commands remain available independently. Appliance
  packaging must preserve user choice; the current boot session is unchanged.

- Add desktop editor mouse/keyboard selection, select all, cut/copy, selection
  replacement and selection-aware undo/redo. Compare undo results with saved text
  so undoing a saved edit still triggers unsaved-change confirmation. Preserve Unicode character offsets
  and document limits when replacing selected text. Expose modified navigation
  and Ctrl+A/C/X to graphical BASIC programs; `SCREENCONTROL 5,text$` publishes
  UTF-8 text to the system clipboard.

- Add a BASIC desktop application with file navigation, a text editor and a
  Chromium browser controller, plus native build/launch scripts and an isolated
  compiled-GUI acceptance check. The appliance boot session remains unchanged.
- Add bounded `DIR$`, `READFILE$`, `MKDIR` and atomic `WRITEFILE` operations for
  BASIC applications. Extend compiled executables with explicit workspace,
  output and command-line options, standalone graphical lifecycle and headless
  self-tests; graphical sessions no longer expire at the instruction limit.
- Preserve long clipboard pastes through the running-program keyboard queue
  and expose editor shortcuts and Tab to graphical BASIC applications.

- Extend independent exact-executable regressions for portable BASIC core
  behavior, file fields, program replacement, and selection error recovery;
  distinguish language acceptance from the development-program catalog.

- Refresh running MERGE programs immediately and relocate saved loop, return,
  error-resume, and debugger positions when statement indices change.
- Clear non-COMMON declarations and stale error handlers when CHAIN starts
  another program, retaining declared types for COMMON values and arrays.
- Route CASE expression failures through ON ERROR, including RESUME retry and
  RESUME NEXT recovery at the enclosing SELECT statement.

- Use descriptor-relative Linux file deletion and renaming for BASIC and READY
  commands, preventing a swapped parent directory from escaping the file root.
- Preserve the caller's declared type, fixed string width, and physical array
  destination through BYREF aliases, even when nested procedures shadow names.
- Make INPUT# consume only its requested fields and continue across lines,
  preserving unread fields and the shared byte cursor for subsequent reads.
  Decode doubled quotes so WRITE#/INPUT# round-trip quoted Unicode and multiline
  strings while retaining input and storage limits.

- Reuse cached X11 frames when windows are uncovered, skip content rebuilds on
  position-only moves, and bound mouse/window event batches so program output
  continues to be serviced while dragging. Add pixel-preservation and captured
  audio-continuity regressions.
- Schedule VM previews at normal priority with a one-CPU aggregate ceiling and
  a 10 ms quota period, avoiding the long audio stalls allowed by the previous
  75% quota over 100 ms.

- Require installation ISOs to fit single-layer 4.7 GB DVDs; reject oversized
  or empty images before replacing any previously published output.

- Build workstation-only images through the shared disk-backed Debian workflow,
  including native source tests, current editor/installer staging and exported
  artifact verification. Require the bootstrap/editor inputs before disk creation
  and budget 48 GiB for a fresh workstation build.
- Separate current image instructions from historical notes and document the
  initial workstation release gates and experimental platform status.

- Preserve representable complex multiplication, logarithms and negative integer
  powers at extreme magnitudes, including subnormal results.
- Pass WEB GET/POST/DOWNLOAD URLs literally to curl, preventing brackets and
  braces from expanding one operation into multiple unintended requests.

- Bound image/video probe output while reading, reject excess decoded image
  bytes before waiting, and terminate/reap decoder children on error. Input
  feeder threads are joined on cleanup and thread-start failure returns an error.
- Share nesting and work limits across raw numerical evaluation and PIECEWISE;
  poll BREAK during nested evaluation instead of allowing unchecked recursive
  expression stacks and multiplicative numerical loops.
- Save complete files by synced temporary-file replacement, preserving the
  previous destination on write failure and retaining its permissions.
- Reserve image/media/SoundFont memory across capabilities and program windows,
  retain charges with asset ownership, and reuse equivalent local asset paths.
  Preflight SoundFont sample chunks and allocate their sample storage once;
  account separately for metadata and decode working space.
- Bound text and binary program storage consistently; rejected loads, merges
  and numbered edits preserve existing source.
- Replace broad catalog error suppression with declared per-program external
  dependencies; reject missing-line errors and abnormal process exits.

- Retain unfinished structured programs during numbered entry, AUTO, EDIT,
  DELETE, RENUM and MERGE; failed execution checks preserve the draft and cannot
  run stale source. EDIT a source filename to reopen unfinished blocks for repair.
- Add non-executing CHECK and read-only INSPECT at READY, including scoped
  variables, record fields and literal-index array elements during STOP/STEP.
  Program storage and session stability guarantees remain unchanged.
- Provision missing examples atomically without replacing user files or following
  user links, including when the provision marker is absent or after an upgrade.
- Run catalog probes in independent disposable source/fixture copies so BASIC
  file statements cannot modify the maintained examples or other test cases.
- Embed an offline programming guide and ship four beginner lessons plus installed
  editor, language and audio references on every platform.

- Use one verified master-source snapshot for all Debian deployment profiles,
  including runtime code, helpers, packaging, tests, assets and supplied notices.
  Centralize the candidate revision and target matrix in `release.json` (revision 14).
- Reject mismatched complete release identities before image assembly and check
  the snapshot again before publishing package/image output. Preserve an existing
  package instead of silently replacing the same release filename.
- Check build storage and host tools before launching a release VM; keep previous
  accepted images separate from new candidates.
- Report the actual Cargo target directory from `build.sh`.
- Resolve VM previews from the accepted `current` release and reject ambiguous
  images instead of falling back to a hard-coded older revision.

- Tokenize compact direct commands like stored statements, resume the direct
  line after GOSUB/RETURN, and reject branches into procedure bodies without
  leaving stale return frames.
- Make stored DATA available before the first RUN and refresh its cursor after
  a successful program replacement while preserving it after failed edits.
- Preserve explicit array types across REDIM and validate local/STATIC whole
  arrays correctly when passed BYREF. Reject incompatible PRESERVE type changes
  and record/primitive replacements before committing any resize.
- Apply default-type declarations atomically, report storage failures, and honor
  explicit suffixes when reading or swapping uninitialized scalar values.
- Enforce file byte limits on binary-byte and random-record reads even if a file
  grows after OPEN; rejected reads preserve logical cursors and record buffers.

- Keep multibyte LSET/RSET values within their byte-sized RANDOM file fields
  without splitting UTF-8 characters or overwriting adjacent fields.
- Update spaced GO TO and GO SUB destinations during RENUM.
- Honor DEFSTR in READ, INPUT, LINE INPUT, DIM, implicit arrays and REDIM,
  including scoped arrays and numeric suffix/declaration overrides.
- Enforce per-file byte limits on graphics exports, including PNG conversion,
  screenshots and default graph outputs, before replacing existing files.

- Keep FOR, WHILE, DO and REPEAT loop frames local to each structured
  procedure invocation, including recursive calls and empty FOR ranges.
- Execute deeply nested inline IF chains without recursive host-stack growth,
  and bind inline ELSE branches to their matching IF in stored and direct code.
- Make MID$ assignment read local, STATIC and BYREF string values, accept typed
  string names, and preserve commas inside nested argument expressions.
- Accept default-type ranges with identical endpoints, such as DEFSTR A-A.
- Preserve finite uniform-distribution results across opposite large bounds,
  annuity calculations near zero interest, relative IRR convergence, harmonic
  means of subnormal values, and negative gamma reflection near underflow.
- Add focused runtime regressions and permanent BASIC artifact reproducers for
  these cases.

- Add a BASIC-authored browser and optional Linux Chromium surface through
  `WEB VIEW`, with owned private profiles, navigation/events, explicit network
  grants, bounded pipe control and Widevine capability checks. Commercial DRM
  playback and deployment acceptance are distinct from engine capability.

- Fix retained MP4 playback when container metadata follows the media data by
  providing a seekable, protocol-restricted decoder cache.
- Keep sequential-file LOC/EOF accurate after invalid UTF-8 and oversized lines;
  consumed input remains consumed as specified by the language contract.
- Enforce append size limits across concurrent channels and program sessions.
- Track Unix advisory locks by open-file identity, including hard-link aliases,
  renames, unlinks, and reuse of an old filename.

- Add removable-drive identity and volume controls with confirmed mount/unmount,
  whole-drive power-off/eject, busy-device reporting, stale-device checks and
  confined UDisks permissions. Removal never forces a busy filesystem.
- Add graphical keyboard layout/variant selection with timed typing preview and
  automatic revert, saved graphical-map restoration at BASIC startup, and confirmed
  persistent time-zone selection through a dedicated confined helper account.
- Add graphical Bluetooth discovery, pairing confirmation and masked PIN entry,
  connect/disconnect, explicit trust and removal. Run a fixed pairing helper under
  its own non-login account; restrict agent prompts to the selected device and
  cancel daemon operations when the user cancels.

- Add graphical Wi-Fi discovery, NetworkManager password/hidden-network dialogs,
  saved-network controls and radio/disconnect actions. Keep credentials out of
  command lines, use temporary profiles until connection succeeds, and restore
  the previous adapter connection after failure or cancellation.

- Stage a separate offline ModernBASIC update/recovery service with retained
  previous packages, checksum/source/architecture checks, dependency preflight,
  durable transaction state, desktop health checks, rollback and boot recovery.
  Show its public status alongside Debian update scheduling in SETTINGS.

- Add an unprivileged OS settings center (`SETTINGS`) with real audio output
  names, default output, connector, volume, mute, and a bounded left/right test.
  Include hardware/update status and existing administration instructions.
- Add the appliance first-run checklist and explicit completion (`SETUP` to
  revisit). Preserve native display rollback through `DISPLAY` and from settings.
  Settings dialogs and audio-test processes close with their BASIC session.

- Keep the system hub available after invalid EDIT arguments.
- Route READY LOAD/MERGE/SAVE/LOADB/SAVEB/VERIFY/COPY through bounded,
  nonblocking sandbox file access. COPY to the same file or a hard-link alias
  preserves contents; binary program reads and writes share the 64 MiB limit.
- Parse NAME/COPY separators outside quoted filenames and reject renaming the
  current workspace or its ancestors.
- Reject trailing arguments to TRON, TROFF, WEND, and RESUME NEXT; validate
  RESUME syntax before changing the active error context.
- Keep diagnostic collection working with unavailable/corrupt build metadata,
  reject non-object saved reports cleanly, and bound subprocess output/time.
- Classify reports as Linux kernel, Modern BASIC environment, OS integration,
  or unsure. Service failures default to integration, without guessing root cause.
- Add the central release/report portal with private developer queues, category
  and status history, consent-based submission, and validated resumable platform
  downloads. Include deployment configs and HTTP security/workflow tests.
  No replacement ISO images are published by this change.
- Extend source validation to Python services and extensionless OS helpers;
  allow staged appliance contract checks to select an exact executable.

- Added 32 case-insensitive names for default screen/text colors in hub
  commands, startup options, and saved preferences. COLORS lists the names;
  hexadecimal colors remain supported. Invalid color input leaves the hub
  available for another command.

- Bound both matrix constructor dimensions before allocation, including zero
  rows or columns, so oversized empty matrices report BASIC errors.
- Reject special files without blocking sandboxed Linux opens; validate the
  descriptor before truncation. Added FIFO timeout coverage for all file modes,
  LOAD, and SAVE.
- Use row-relative pivot selection for DET, INVERSE, and LINSOLVE, preserving
  small nonsingular matrices and matrices with differently scaled rows.
- Preserve small components in complex square roots and avoid intermediate
  overflow/underflow in complex division by separating binary exponents.
- Normalize IRR cash flows before convergence checks so changing their common
  scale does not change the result.
- Added numerical and empty-matrix artifact regressions. These fixes preserve
  the revision 1 Values, Resources, and Files contract rows; no language syntax
  or contract revision changes.

- Fixed atomic INPUT/INPUT# storage preflight when replacements reuse existing
  payload and when BYREF destinations alias the same variable.
- Preserved whole-array element identity when forwarding a BYREF element into
  a nested procedure.
- Added generated loop/procedure and input-transaction checks, a permanent
  executable regression corpus, and a checked core language stability contract.
- Added measurable session lifecycle soaks, paced PCM fidelity/gap probes,
  repeatable X11 renderer cycling, and exact executable/Debian-artifact checks
  that record hashes and never silently rebuild the selected artifact.
- Added per-change MSRV/stable CI and scheduled long-session validation.
  Hardware certification and the reported full-workload live-audio issue remain
  explicit outstanding release requirements.

## 0.8.2 Neon Depth Runner acceptance - 2026-09-02

- Added `Neon Depth Runner`, a self-contained playable game demonstrating
  selectable HD-through-UHD screens, generated/resident mapped textures, shared
  sprite instances, XYZ control and motion, camera/light changes, retained
  objects/pixels, projected collision, pause/restart behavior, and a
  deterministic rendered self-test.
- Extended appliance QEMU validation to load the bundled game, generate its
  assets as the unprivileged runtime user, start gameplay through virtual
  hardware input, and require a changed live game frame before accepting the
  ISO.
- Made sprite motion/depth/collision and perspective-object query functions
  evaluate string-variable IDs consistently with their matching statements.
  The loop-driven game and a focused regression cover this pooled-entity path.

## 0.8.2 modern game screens and resident 3D sprite world - 2026-09-01

- Added named logical game resolutions for 720p, 900p, Full HD, QHD, 3440x1440
  ultrawide, and 4K while retaining independent fit/stretch host scaling.
- Extended `SPRITE IMAGE` with an `(x,y,z)` form. Mapped sprites now occupy the
  retained perspective world, share load-time-decoded texture pixels, project
  and depth-sort with all other entities, receive ambient/point-light tint,
  move on three axes, and participate in projected collision checks.
- Added a bounded 64 MiB per-render-thread cache for immutable engine-private
  textures. Cache hits reuse the same pixel allocation; transparency-key
  variants remain isolated and source files are never reopened per frame.
- Added table-level resolution coverage, model-level world-command checks,
  allocation-identity cache tests, and black-box BASIC tests that delete the
  source texture before rendering the live XYZ world.

## 0.8.2 native sprite and font asset workflow - 2026-09-01

- Added image-backed named sprites with scaling, exact-color transparency,
  movement, deterministic motion, collision queries/traps, and normal buffered
  rendering through `SPRITE IMAGE`.
- Added portable bitmap-font sheets selectable through `FONT`, including
  cached decoding, tinted alpha/brightness masks, and `FONT BUILTIN` restore.
- Added C128-inspired graphical `EDIT SPRITE` and `EDIT FONT` hub tools. They
  export eight 24x21 sprite assets and a complete 96-glyph 8x8 ASCII sheet in
  directly consumable PPM form.
- Added black-box rendering/collision/key-transparency coverage, hub routing,
  bounded headless editor-loop probes, and QEMU checks that require both editor
  windows to render distinct content and remain stable. Updated the embedded
  language manual and classic-capability matrix.

## 0.8.2 independent production hardening - 2026-09-01

- Added a separate seven-test hardening target using model-based arithmetic,
  binary-image mutation, parallel-session stress, live symlink swapping,
  hostile CLI input, and sandboxed printer-output probes. The target passes
  normally and under AddressSanitizer.
- Routed default `LPRINT`/`LPRINT USING` output to a bounded spool beneath the
  configured output sandbox. Removed automatic physical-printer discovery and
  the predictable shared `/tmp` spool; explicit host spool overrides reject
  symlinks, use close-on-exec descriptors and private creation permissions,
  and obey the configured per-file byte limit.
- Replaced the graphical shell's predictable shared `/tmp` error log with
  stderr diagnostics, corrected the manual-matrix source path and native
  64-bit line-number contract, fixed a README code fence, and reconciled the
  verified catalog at 360 numbered programs after removing a zero-byte scratch
  artifact.
## 0.8.2 Music Forge responsiveness update - 2026-08-28

- Made `SOUND OFF` discard native and sampled score queues as well as stopping
  live playback. Interactive music programs can now stop, rebuild, and restart
  a score without stale notes accumulating in memory.
- Changed Music Forge playback to background operation so its graphics and
  keyboard loop remain responsive while a score plays. `S` stops playback and
  `P` restarts it from a clean queue.
- Removed the mouse-release busy loop and compressed consecutive empty score
  slots into longer rests before MML conversion, reducing input stalls and
  playback preparation work.

## 0.8.2 modern-native graphics fast path r2 - 2026-08-28

- Applied the exact `rustfmt` layout required by `cargo fmt --check` so the
  complete `verify.sh` gate can proceed to compilation and tests.

## 0.8.2 modern-native graphics fast path - 2026-08-28

- Kept graphics `GET` arrays fully compatible with ordinary BASIC array reads
  and writes while caching their dense ARGB pixels behind the array. Repeated
  `PUT` calls now share that cache instead of rebuilding a framebuffer-sized
  vector from sparse array values every frame; any BASIC element write
  invalidates and safely rebuilds the cache.
- Added a full-frame opaque `PUT PSET` path with scanline framebuffer copies.
  It avoids destination reads, per-pixel raster-action dispatch, repeated dirty
  rectangle expansion, and a second framebuffer-sized result allocation.
- Added automatically selected `raster_rle` retention for compressible copied
  frames, while preserving the established inline raster representation for
  images where run-length encoding would be larger.
- Added a native-size device-bitmap renderer path. Exact `PUT` rasters are no
  longer bilinearly resampled with floating-point calculations at every pixel,
  and retained bitmap payloads use shared storage when scene state is cloned.
- Changed the automatic-PSET presentation regression to verify the rendered
  white pixel instead of requiring one obsolete Artist command spelling.
  Added cache reuse/invalidation, compact-raster validation, scanline blit,
  opacity, and framebuffer correctness coverage.
- Added `GRAPHICS_PERFORMANCE.md` with the recommended draw-once, `GET`,
  `PUT`, dynamic-overlay, and single-`FLIP` animation pattern.

## 0.8.2 modern-native true-color graphics refresh - 2026-08-28

- Added native `RGB(red,green,blue)` and `RGBA(red,green,blue,alpha)`
  constructors. Calls now produce canonical true-color strings instead of
  falling through to BASIC array lookup and reporting `?bad subscript`.
- Added scanline raster batching for consecutive identity-coordinate `PSET`
  operations. Pixel paintings retain one command per contiguous row run while
  classic numeric palette colors and general transformed drawing retain their
  established behavior.
- Added expression, color-range, renderer-order, framebuffer, and runnable
  true-color gradient regressions.

## 0.8.2 modern-native refresh - 2026-08-27

- Corrected the complete catalog gate: explicit negative programs now have
  exact required diagnostics, disabled WAV exporters are classified as
  environmental probes, and an expected error that disappears or changes is
  a verification failure. Repaired three flipbook declarations, two Koto
  programs, one malformed SOUND statement, one obsolete note-analysis test,
  and normal `PLAY score$,beats` LOAD-manifest handling.
- Added one stable `install.sh` entry point. A normal installation is now
  `./install.sh`; release validation is `./install.sh --verify`; startup remains
  `./run.sh`.
- Made the asynchronous BREAK regression deterministic: the test now steps
  through its RAM setup before starting the interrupt timer, then verifies
  BREAK during `WAIT` and confirms the active program and variable survive.
  This removes a scheduler race exposed when LOAD-time preparation takes more
  than the test's former 20 ms head start.
- Moved statement splitting, line indexing, structured-control validation, and
  DATA collection to the successful LOAD/edit boundary. Interactive `RUN` now
  clones one resident execution-plan handle and resets only BASIC runtime
  state; it neither reparses the stored program nor reopens prepared assets.
  Added a regression that deletes a bitmap after `LOAD` and proves `RUN` still
  succeeds from the decoded in-memory object.
- Made `READY.` a strict successful-preparation boundary. Failed `LOAD` and
  `LOADB` requests now report `LOAD FAILED; OLD PROGRAM RETAINED.` without
  printing a misleading READY line for the rejected program.
- Kept live execution separate from optional output creation: ordinary
  `PLAY`, graphics drawing, `DISPLAY`, `SOUND RESUME`, `END`, and `STOP` write
  no files. Export statements remain explicit opt-in operations.
- Replaced the live VIDEO soundtrack's `/proc/self/fd` FFmpeg input with the
  native inherited `pipe:N` protocol. This fixes the target-machine encoder
  exit status 250 (`ENXIO`) and removes a needless Linux procfs dependency.
  Encoder failures now retain a bounded FFmpeg diagnostic instead of hiding
  stderr.
- Corrected the release formatting detected by `cargo fmt --check` and removed
  the unused legacy articulation helper and its isolated test. Sampled note
  gates continue to come from the active per-voice `SoundFontDsp` settings.
- Restored the declared Rust 1.74 minimum-toolchain compatibility by using
  edition-2021 FFI block syntax and explicit `std::mem::size_of` paths. This
  also fixes native builds with Ubuntu 24.04's Rust 1.75 packages.
- Consolidated sampled playback and WAV export onto one SoundFont score
  renderer and one fixed-gain PCM encoder. Removed the unused sequential
  note-on/render path, raw voice-zero exception, per-chunk peak normalization,
  and gain pumping; quiet passages now retain their programmed level.
- Removed unused additive/modal prototype modules and the no-op banjo body.
  Corrected banjo construction so it starts silent, selects only a physically
  playable string, gives the requested string stable full level, and excites
  only quiet harmonically related open strings.
- Added four-program sound/graphics concurrency verification. The system hub
  remains separate from every program's `READY.` statement environment, and
  each program task retains independent graphics, input, interrupt, and audio
  ownership while host playback mixes simultaneous songs.
- Made oscillator-limit integration tests select deterministic `STANDARD`
  explicitly instead of depending on the host-selected `AUTO` profile.
- Extended `verify.sh` through release smoke tests and a complete 346-program
  catalog pass with canonical loading plus bounded, classified headless runs.
- Kept ModernBASIC as one native Rust language and documented the boundary
  between compatible BASIC semantics and virtualized or native host services.
- Made aspect-preserving graphics fitting the actual shell default; deliberate
  non-uniform stretching remains available through `--graphics-scale stretch`.
- Made the hardware-scanned `AUTO` synthesis arrangement the modern default,
  while retaining explicit deterministic profiles and exact voice/oscillator
  overrides.
- Raised the guarded non-interactive execution budget from 100,000 to
  10,000,000 statements. Interactive `READY.` sessions remain unlimited until
  completion or BREAK.
- Added `build-native.sh` for host-CPU optimized builds and `source-check.sh`
  for compiler-independent structural validation.
- Integrated the current 346-program catalog, repaired duplicate/out-of-order
  lines in two Persian music programs, and added a native 64-bit/core-BASIC
  conformance program.
- Kept optional third-party SoundFont banks outside the source archive and
  documented the exact bank names required by sampled-instrument programs.

## 0.8.2 — 2026-08-19

- Added validated `LOCATE`, `CSRLIN`, `VIEW PRINT`, `HOME`, `HTAB`, `VTAB`,
  `TEXTCOLOR`, key-abortable fractional-second `SLEEP`, and live bare
  `INKEY$`. The new identifier-encoded statement names remain assignable when
  followed by `=`. Text cursor state, region changes, output, and color events
  stay causally ordered for the X11 shell.
- Corrected Unicode `PRINT`/`WIDTH` column tracking and wrapping, Microsoft
  comparison-before-`NOT` precedence, and numeric `PRINT USING` type checks.
  Invalid text coordinates now raise `IllegalQuantity`, and `HOME` uses the
  active print-region origin.
- Added generation-tracked lazy synchronization from retained Artist scenes to
  each graphics framebuffer, providing coherent pixels for future query and
  capture statements. Fullscreen startup now requests managed EWMH
  `_NET_WM_STATE_FULLSCREEN` instead of using `override_redirect`.
- Isolated PCM playback and speech queues per interpreter/program window and
  recognized both `MODERNBASIC_NO_AUDIO` and `MODERN64BASIC_NO_AUDIO`
  consistently across synthesis, speech, and shell status reporting.
- Added retained `FLIPBOOK CREATE`/`REMOVE`/`CLEAR`, paced `PLAY FLIPBOOK`, and
  direct `VIDEO EXPORT` so hundreds of complete still images form one bounded
  movie asset without a duplicate video engine. Extended `PLAY` explicitly to
  sandboxed local `AUDIO` and `VIDEO` while preserving classic note notation.
  Added a 72-frame old-paper stickman demo and a 120-frame 960x540 one-leaf
  photo-detail demo with an anchored bending/turning texture mesh, and produced verified GIF/H.264 artifacts from both.
- Added `BRUSH` with size, hardness, opacity, and flow; PNG/JPEG/WebP/BMP/TIFF
  BITMAP import through fixed FFmpeg arguments; alpha-preserving premultiplied
  bilinear bitmap scaling; the organized BITMAP WARP subcommand for anchored
  bend, turn, angle, and light; flipbook letterboxing; and raster-first PNG output that preserves imported
  photographic pixels and compositing. Dynamic string graphics identifiers now
  evaluate correctly instead of being mistaken for literal names.

- Added explicit `MIXER voice,synth-level,midi-level` routing so each `PLAY`
  voice can use the computer generator, the MIDI/SoundFont instrument, or both
  sample-accurately layered. Native and sampled queues are now combined for
  live output and `WAVEFILE` instead of one queue replacing the other.
- Retained validated `SING` scores for optional `WAVEFILE` export. Vocals now
  render at 44.1 kHz, append in source order on their own timeline, and mix
  sample-accurately with native and SoundFont music.
- Completed the native instrument-building core with a configurable bounded bank
  of 1 through 8 per-voice `OSCILLATOR` slots, `SYNTHPROFILE` host scanning,
  per-program `VOICES`/`OSCILLATORS` overrides, and per-voice multimode
  `FILTER`, `FILTERADSR`, plus simultaneous
  target-specific `LFO` controls, `GLIDE`, and `DRIVE`. These are independent
  BASIC controls; no catch-all `SYNTH` statement was introduced. Added compact
  patch and arrangement examples plus `magnetic_dawn.bas`, an original
  eight-voice extended native-synthesis showcase.

- Established one ModernBASIC tone interface, `SOUND frequency,voice,duration`
  with optional per-note level and sweep controls. `WAVE`, `ADSR`,
  `PULSEWIDTH`, `VOL`, `STEREO`, and `FILTER` remain independent voice controls,
  and `SOUND` can queue complete oscillator tone sequences without `PLAY` note
  notation. Removed the
  reversed voice-first signature and its unrelated delay argument rather than
  retaining a compatibility overload.

- Made interactive `LOAD` parse and retain distinct static SoundFont assets
  before committing the program or returning the prompt; failed preparation is
  atomic. `RUN` reuses those banks, and live sampled playback now writes bounded
  2,048-frame chunks instead of rendering the entire song before its first
  sound, with ADSR and per-voice DSP state continuous across chunks.
- Corrected the raster font's column-major glyph interpretation so printable
  ASCII renders as upright English instead of rotated symbol-like shapes. Large
  capitals, digits, and presentation punctuation now use a dependency-free
  antialiased geometric display face instead of enlarged 5x7 cells. Added a
  15-second Full-HD capabilities presentation with dancing type, real sprite
  motion/collision state, gradients, compositing, curves, generated stereo
  music, and data-driven graphics.
- Defined `SCREEN` as a resolution-independent logical graphics device in the
  native shell. Window resizing now preserves and centers its aspect ratio by
  default, pointer coordinates map only through the fitted viewport, and black
  bars are outside `MOUSEINSIDE()`. Added compact, desktop, HD, and Full HD host
  window profiles, an explicit stretch mode, physical-display clamping, atomic
  failed screen changes, and repeated buffer/resize/extreme-geometry stress
  coverage.
- Tightened `CLS` invariants: it resets the selected graphics buffer, sprites,
  pixels, and drawing point without touching the visible front buffer before
  `FLIP`; argument-free split-screen clears now clear both text and graphics.
  Regressions prove retained rendering cannot preserve shapes removed by a
  clear. The capabilities presentation now defaults to file-free live BASIC,
  follows wall-clock time, and resets animation and native synthesized music
  together; recording is an explicit source switch.
- Added bounded program-driven `VIDEO START`/`FRAME`/`STOP` export to MP4,
  WebM, and GIF through a fixed FFmpeg encoder interface, plus a 360-frame
  1920x1080, 60 fps graphics-only cinematic animation demo and real H.264
  acceptance coverage. `VIDEO START` now accepts an optional sandboxed WAV
  soundtrack for MP4/WebM. MP4 uses a high-detail CRF 17 real-time H.264
  encode with eight bounded encoder threads. Batch runs can explicitly raise
  the otherwise unchanged safety guard with `--max-steps` for bounded long-form
  rendering. Generated graphics,
  audio, and video can use an independent `--output-root`; packaged runs gather
  them in the root-level `output/` folder instead of the BASIC source catalog.
  Soundtrack muxing pads short audio and clips long audio at video EOF, avoiding
  an equal-duration codec-timebase race that could close the final frame pipe.
- Activated true linear/radial gradient paints and all advertised blend modes
  in raster/video output, corrected radial-gradient radii, added subpixel edge
  coverage for thin/thick lines and polygon slopes, added simultaneous
  fill/stroke rendering for core primitives, and made rounded rectangles
  genuinely rounded. `LINE ...,color,width` now honors expression and variable
  widths instead of confusing them with the legacy `B`/`BF` box mode. Headless
  animation no longer
  rerenders the accumulated scene after every drawing statement; the HD demo's
  measured single-frame time fell from 13.86 seconds to 0.15 seconds.
  Full-HD raster storage is now binary32, exact common scene prefixes are
  retained in a 32 MiB cache, and final RGB packing is parallel. The
  capabilities benchmark improved from 12.1 to 36.1 generated fps without
  changing its
  8-bit output bytes; sustained 1080p H.264 generation reaches 34.8 fps.
- Added deterministic sprite velocity/acceleration stepping with live
  position/motion queries, logical-screen mouse polling, and a real bounded
  nonblocking `INKEY$()` queue for interactive graphical programs.
- Defined the Amiga 500 creative-programming profile and its modern hardware,
  safety, dialect, and no-photo/no-video-editor boundaries.
- Added per-voice `DSP voice,drive,tone,presence,compression,gate` for sampled
  amplifier saturation, cabinet tone, pick/strike presence, dynamics, and
  note-release articulation before voices are mixed. Steel Thunder now gives
  all ten instrument voices role-specific DSP configurations.
- Added `--soundfont-root` so multi-user installations can keep one shared,
  administrator-owned instrument backend while each account uses a separate
  writable `--file-root`. Added common system-wide backend discovery paths.
- Replaced Steel Thunder's synthesized-sounding lead with paired clean/distorted
  CC0 Fender samples and replaced its doubled pitch-shifted/GM bass with a
  dedicated CC0 Yamaha RBX picked-bass bank. The compact clean lead avoids a
  425 MiB decoded bank in every concurrent process. Explicit low-sustain ADSR
  shaping emphasizes the recorded pick transient on both guitar and bass.
- Added per-voice and inclusive voice-range `SOUNDFONT` bindings so one BASIC
  `PLAY` score can combine multiple sampled banks on a shared timeline. Fixed
  one-based BASIC voice instrument selection, made `END`/`STOP` wait for queued
  live playback, and kept file creation exclusive to explicit `WAVEFILE` use.
- Kept optional banks as backend hardware in the dedicated root-level
  `soundfont/` directory and made that read-only asset root independent of
  READY's current directory, fixing OS error 2 after `CD` or nested loads.
- Applied explicit per-voice BASIC `ADSR` settings to sampled SoundFont output;
  voices without `ADSR` retain the instrument bank's natural envelope.
- Added sandboxed `SOUNDFONT path$` selection for real sampled instruments,
  automatic MuseScore General/FluidR3 system-bank discovery, and an installer
  for the free MIT-licensed MuseScore General SF2 and its license. Added BASIC
  control of sampled volume, pan, note velocity, expression, modulation, sustain,
  pitch bend, reverb, and chorus.
- Renamed the product to ModernBASIC; the Cargo package, library crate, and
  executable are now `modernbasic`, and configuration variables use the
  `MODERNBASIC_` prefix.
- Printed the classic `READY.` line whenever `RUN` or `CONT` returns control,
  including completion, STOP/BREAK, and runtime errors, while correctly ending
  any unterminated PRINT line first. The status bar now labels Ctrl+U as the
  uppercase/mixed-case input toggle.
- Added graphical list editing: Ctrl+Up/Ctrl+Down automatically capture and
  navigate a loaded program, F2 remains an explicit LIST/edit entry, the normal
  cursor keys edit selected lines, and Enter replaces them without retyping.
  Ordinary Up/Down retain command and numbered-statement history behavior.
- Prevented X11 special keysyms such as Ctrl+Left from being truncated into
  control letters, bounded cached line indices, invalidated stale line caches
  on command transitions, and kept F-key commands out of a running program's
  INPUT stream.
- Replaced prompt-text guessing with an explicit READY event from the REPL, so
  program output cannot accidentally enable command-only keyboard actions.
- Made existing sandboxed file and directory lookup case-insensitive, allowing
  uppercase BASIC input such as `CD EXAMPLES` to reach packaged lowercase host
  paths without weakening canonical root and symlink-escape checks.
- Completed the READY file browser with BASIC-relative path reporting,
  subfolder/file/byte totals, directories-first sorting, root and parent
  navigation, both slash styles, compact `CD..`, wildcard listings, and bounded
  recursive `TREE` browsing that never follows links.
- Made `programs` the complete visible root name and suppressed the canonical
  `/home/...` prefix from `PWD`, `DIR`, `TREE`, and `STATUS`. Displayed paths
  such as `programs/music` can be passed back to every READY file command.
- Made Ctrl+Escape the sole hard BREAK shortcut and return graphics/demo views
  to text so the result is visible; plain Escape remains a local view/cancel
  key without interrupting BASIC.
- Added a non-audio BASIC conformance example and included it plus sandboxed
  WRITE#/INPUT# round-trip behavior in the release smoke test.
- Ported the attached 210-file runnable program catalog to the current 0.8
  statement set instead of adding removed legacy commands to the interpreter.
  Corrected duplicate/out-of-order numbered lines, replaced the TCP chat and
  character-ROM editor demos with portable programs, and rewrote old audio
  programs around `WAVE`, `ADSR`, `VOL`, `PLAY`, `SOUND`, and `WAIT`.
- Unified native oscillator playback with the SoundFont Linux backend:
  `paplay` is preferred and `aplay` is the fallback for both paths. Added
  `audio-test.sh`, deterministic RIFF/WAVE smoke coverage, and catalog source
  checks that reject legacy statements and duplicate or unsorted line numbers.
- Made the packaged `programs` directory the default BASIC user root, with
  `--file-root` retained as an explicit override.
- Made `shell program.bas` actually load the requested program.
- Implemented native interruptible `WAIT milliseconds` and classic
  `WAIT port,mask[,xor]` over the bounded virtual port device.
- Corrected `WHILE`/`WEND` pre-test skipping, nesting, frame reuse, and
  unmatched-loop diagnostics.
- Added colon-aware inline `ELSE` and validated nested block
  `IF...THEN...ELSE...END IF` forms.
- Added `DO`/`LOOP` top/bottom conditions, BBC `REPEAT`/`UNTIL`,
  and structured `EXIT DO`/`FOR`/`WHILE`/`REPEAT`.
- Added nested standard `SELECT CASE` with values, inclusive ranges, relational
  clauses, strings, and defaults, plus BBC `CASE...OF`/`WHEN`/`OTHERWISE`.
- Added `XOR`/BBC `EOR`, integer `DIV` and `\`, Microsoft-style rounded integer
  operands, bare `RND`, `BIN$`, and the checked `FACTORIAL` alias.
- Added spaced `GO SUB`, one-or-more-parameter numeric `DEF FN`, ASCII
  `UCASE$`/`LCASE$`, and C128 hexadecimal-string `DEC` fallback.
- Added `LINE INPUT #` with bounded whole-line reads that preserve commas and
  empty lines.
- Added MS/GW-BASIC `OPEN mode$,#channel,path$` shorthand while retaining the
  standard `OPEN path$ FOR mode AS #channel` grammar and file sandbox.
- Added bounded, atomic Dartmouth `CHANGE` conversion between strings and
  numeric character-code arrays.
- Added bounded, atomic `REDIM` and `REDIM PRESERVE` for numeric and string
  arrays. Preservation keeps the complete overlapping region of same-rank
  multidimensional shapes. Corrected `ERASE` to release sparse value storage
  and full reserved array capacity.
- Added immutable numeric and string `CONST` declarations with calculated and
  comma-separated initializers, typed assignment, `CLEAR` persistence, and
  protection across scalar assignment, input, data, swap, and string-lvalue
  paths.
- Added validated multi-line `SUB`/`END SUB` procedures with value parameters,
  recursive and nested `CALL`, per-call local scalars, scalar `SHARED` and
  persistent `STATIC` storage, `EXIT SUB`, classic `GOSUB` interoperability,
  shared stack/storage limits, atomic argument binding, and branch-boundary
  enforcement.
- Added validated multi-line numeric and string `FUNCTION`/`END FUNCTION`
  declarations with typed return-slot assignment, `EXIT FUNCTION`, recursion,
  local/`SHARED`/`STATIC` scope, calls to `SUB`, expression and `READY.` use,
  bounded depth/storage, and lazy `IF`, loop, `SELECT CASE`, and `PIECEWISE`
  evaluation. Corrected `WHILE` conditions to run exactly once per iteration.
- Added coherent procedure argument modes: parameters remain `BYVAL` by
  default, may state `BYVAL` explicitly, and may use exact-type `BYREF` aliases
  to scalars, array elements, record fields, and record-array field elements.
  Aliases forward through nested/recursive `SUB` and `FUNCTION` calls, mutate
  immediately, and do not duplicate retained-storage charges.
- Added explicit exact-type whole-array `BYREF name()` formals and `name()`
  actuals, including nested forwarding, record-field arrays, `READ`, `MAT`,
  `REDIM [PRESERVE]`, `ERASE`, fixed strings, and scope isolation. Exact-type
  whole-record arrays use `BYREF name() AS type`, alias every field without a
  copy, forward through nested `SUB`/`FUNCTION` calls, resize atomically, and
  propagate erasure back to the caller.
- Added per-call local primitive arrays declared with procedure `DIM`, plus
  persistent `STATIC name(bounds)` arrays for `SUB` and `FUNCTION`. Recursive
  frames isolate physical storage, return releases local capacity, `CLEAR` and
  `CHAIN` release STATIC capacity, and primitive `AS` metadata is restored
  transactionally so identically named procedures cannot contaminate types.
- Added fully scoped procedure-local and persistent `STATIC` record arrays.
  Recursive calls isolate every physical field array, local capacity is
  released on return, STATIC fields survive calls and reset on `CLEAR`, and
  record/type metadata rolls back atomically on failed dimensioning.
- Added scoped typed primitive and record scalars inside `SUB`/`FUNCTION`, plus
  `STATIC name AS type`. Record fields inherit the base record's `SHARED` or
  `STATIC` scope; recursive locals isolate values and metadata, STATIC types
  cannot silently change, and `CLEAR`/`CHAIN` release both values and type state.
- Added exact-type whole-record assignment for scalar records and indexed
  record-array elements, including values reached through whole-record BYREF
  formals. Every source field is captured before writing and late storage-limit
  failures roll back all destination fields.
- Added nested user-defined record fields with forward TYPE references,
  pre-execution undefined-type and cycle rejection, multi-level scalar/array
  access, exact-type whole-subrecord assignment, and root/subrecord typed-array
  aliases that forward through nested procedures. Local, STATIC, REDIM, ERASE,
  fixed-string, and retained-storage behavior reuse the same atomic record
  machinery.
- Corrected `CALL` argument tokenization so procedure names beginning with a
  reserved prefix, such as `SCALEALL`, `FORWARD`, or `MODIFY`, remain identifiers
  while classic compact control syntax stays unchanged.
- Added `LBOUND`/`UBOUND` over real stored shapes, with multidimensional,
  `OPTION BASE`, record-field, local, STATIC, REDIM, and whole-array alias
  coverage.
- Made `SPRITE`/`MOVESPRITE` visible in the built-in raster/SVG renderer,
  tracked per-buffer sprite geometry, and added live pair/any `PCOLLISION`
  queries. Added bounded, non-reentrant `ON TIMER(...) GOSUB` and `ON COLLISION
  GOSUB` event traps with `ON`/`OFF`/`STOP` lifecycle controls.
- Added validated `TYPE`/`END TYPE` records and `DIM ... AS type` for scalar
  and array instances. Primitive numeric, dynamic string, and fixed-width
  string fields support dotted reads/writes, `DATA`/`READ`, console/file input,
  bounded atomic `REDIM [PRESERVE]`, and record-aware `ERASE`.
- Corrected token-stream serialization for every extension keyword beyond the
  historical one-byte range using an escaped 16-bit form; legacy token bytes
  remain unchanged. Qualified record identifiers now round-trip through source
  and binary forms, including the classic `array(index).field` spelling.
- Corrected `CLEAR`/`RUN` state reset to discard active structured frames,
  persistent procedure `STATIC` values, and FUNCTION result handoff state
  together, preventing stale values and retained-storage accounting.
- Corrected `WRITE#` to emit quoted, comma-delimited BASIC fields instead of
  `PRINT#` zone spacing, including embedded quotes and nested expressions.
- Corrected `ON ERROR`/`RESUME`: plain `RESUME` retries the failing statement,
  `RESUME NEXT` advances once, loop depth is restored, the handler remains
  armed for later errors, and `ON ERROR GOTO 0` disables trapping cleanly.
- Completed inverse keyword mappings for `NAME`, `LINE INPUT`, and the internal
  default-type token.
- Added editor, directory-root, statement-token, and native hardware regression
  coverage plus a statement-conformance reference.

## 0.8.1 — 2026-08-19

- Preserved the attached 0.8.0 source as the authoritative branch instead of
  overwriting it with the differently structured 0.7 package.
- Corrected SoundFont `PLAY` voice scheduling. Voice numbers are now converted
  to zero-based cursor indices correctly, and `Vn` changes both the cursor and
  MIDI channel for subsequent notes.
- Added regression coverage for independent SoundFont voice cursors and MIDI
  channels.
- Preserved left/right SoundFont samples so `STEREO` and `PAN` affect both live
  playback and WAV output instead of collapsing the result to mono.
- Peak-normalized live SoundFont output is encoded and written in bounded
  blocks instead of allocating a second song-sized PCM buffer.
- Live playback now prefers `paplay` and falls back to `aplay`.
- Added dotted note-length coverage such as `C4/8.`.
- Removed unused mastering constants and corrected lint-sensitive code in the
  attached uncommitted audio changes.
- Synchronized Cargo manifest and lockfile versions at 0.8.1.
- Added release documentation, examples, verification scripts, an attachment
  comparison report, and the required RustySynth/MeltySynth MIT notice.
- Removed repository history and Python bytecode from the distributable tree.

## 0.8.0 attachment baseline

- Native ModernBASIC interpreter with 16-voice audio, SoundFont synthesis,
  speech, graphics, graphing, matrices, and a Linux/X11 shell.
- Consolidated the earlier DSP/transcription experiment out of the runtime.
- Removed the older separate `MUSIC` and `SOUNDFONT` statement families.

- Corrected SoundFont generator parsing for banks without unused terminal padding.
- Sampled scores retain per-PLAY mixer, controller, pitch, ADSR and DSP settings.
- Added exact SoundFont preset names and optional INSTRUMENT bank selection; unavailable presets now report errors.
- Improved Linux player startup fallback and retained bounded diagnostics; appliance playback now uses unity gain.
- Restored sampled instruments in ten music programs that contained removed-command placeholders.
