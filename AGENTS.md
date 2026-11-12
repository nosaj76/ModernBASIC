# ModernBASIC workspace and authoritative original source

Owner instructions, updated 2026-10-22:

- Active workspace: `/home/jason/modernbasic`. Run all development, builds,
  tests, VM disks and generated outputs here, using an explicit working directory.
- Authoritative original/main source (also the protected source backup): `/media/jason/1ae7c45c-4d30-4b65-8d60-c6986605ac67/jason/projects/modernbasic`.
  Preserve source code, history, original source archives, documents, programs,
  assets and evidence there. Never delete this project directory.
- Required workflow for every code change: start from the latest verified original
  source, develop in the home working copy, run appropriate tests/checks, then
  copy the verified source changes back into the original source directory.
  Do not finish a tested code checkpoint with the original source left stale.
  Verify copied file hashes and retain prior source history before replacement.
  Untested or failing changes stay in the home working copy until corrected.
  Before refreshing the working copy, reconcile unique changes on either side;
  never blindly overwrite or destructively mirror either tree.
- The owner authorized moving project work off the backup drive to reclaim space.
  Verify home copies before removing generated files from the backup drive.
  Compatibility symlinks may point generated build paths into the home workspace.
- Do not build or write new work on the backup drive. Source backup updates must
  preserve existing history and must never use a destructive mirror or --delete.
- Never write project source code, generated code, build outputs or test artifacts
  to `/tmp`; keep all project work in the active home workspace.
- Inspect cleanup paths individually; do not follow symlinks or recursively delete
  the protected project. Preserve independently verified recovery copies before
  removing user data or historical material.
- Retain the frozen revision-1 source archive and consolidation receipts. These
  safeguards do not imply filesystem immutability or restore earlier removed history.

The running launcher may temporarily retain its tiny session/lock metadata in the
old VM directory; the actual guest disk has been live-migrated into the home workspace.

## Space discipline — owner priority, 2026-10-25

- Reclaim avoidable development storage; do not let generated work accumulate.
- Use one shared Cargo target directory in the home workspace. Do not create
  per-checkpoint target trees. Keep routine debug symbols and incremental builds
  disabled; reuse the current toolchain and dependency caches.
- Before adding a large build, check current disk use. If shared debug/release
  outputs exceed 5 GiB, inspect and retire obsolete reproducible outputs before
  continuing. This review limit does not authorize deleting source or user data.
- Keep one reusable OS builder, now `work/workstation-build/build.qcow2`.
  Do not recreate the retired `work/allprofiles-build/build.raw` clone.
  Retire disposable test-installation disks after qualification; keep their
  commands, logs and reports. Never confuse them with the persistent user VM
  at `releases/revision-1/vm/modernbasic.qcow2`, which must be preserved.
  Retire the generated `work/workstation-build/bootstrap.raw` after image work
  ends, once its payload is verified against the retained Debian seed and no
  process uses it. The build launcher regenerates it when needed; preserve the
  seed and reusable builder. Do not keep redundant boot media between builds.
- Run integration tests in small named batches and retire their generated
  executables after recording results. Keep reusable dependencies and runtime
  binaries; do not prebuild and retain every integration-test executable at once.
  Inspect `.build/target` directories produced by standalone compilation too;
  retain generated source, exported applications and reports, then remove their
  inactive compiler caches. Record unfinished tests honestly before cleanup.
- After replacement installers are qualified and their source checkpoints are
  verified, retire superseded candidate ISO/package files and extracted staging
  copies. Preserve the frozen revision-1 release, sources, unique assets and logs.
- Reuse canonical package/ISO files instead of copying them into each evidence
  directory. Preserve frozen source archives, user recovery data and revision-1
  safeguards. Keep checkpoint evidence compact and clean generated scratch work
  when the checkpoint ends. Do not create new copies merely for cleanup itself.

## Owner-defined architecture and versioning

- Linux supplies the OS foundation; Rust supplies the interpreter/runtime and
  backend engines. Implement the advanced math engine in Rust and expose it with
  BASIC statements/functions. BASIC demos are consumers, not engine completion.
- Backend release versions track the Rust side. Users' BASIC source is freely
  editable; never require backend version bumps, frozen source, registration or
  release approval for ordinary user program changes.
- Distinguish bundled-artifact integrity inventories from backend versions and
  from user application source. Preserve API compatibility where possible.

## Product priority clarified by the owner

ModernBASIC aims to be the contemporary successor in spirit to Dartmouth BASIC
and TI-BASIC, with the integrated creative/programming reach of a modern Amiga-like
computer on current hardware. The advanced math engine is a primary foundation,
not a secondary demo feature. Prioritize trustworthy Rust numerical/symbolic
kernels and composable BASIC APIs, then their integration with graphics, audio,
animation, simulation and general application development. Preserve approachable
interactive programming and freely editable BASIC source. Validate real workloads,
precision, recovery, latency and resource use; historical names or feature counts
are not evidence of parity. The owner asked to keep the Python font maker unchanged.

## Documentation consistency

When changing documents used by the portal, regenerate
`services/portal/content.json` with `services/portal/build_content.py`, run its
`--check` mode, and include the verified generated manual in the source checkpoint.

## Task continuity

The owner asked that these requirements not be forgotten. Maintain `TASKS.md`,
especially its owner-requirements section, when resuming work. Keep accepted
milestones separate from open implementation, reference, image and hardware gates.

The owner further requires the Rust math engine to be the shared backbone for
physics simulation/engines, science research, analysis, finance, AI/ML, graphics
and music/sound/audio: one coherent foundation, composable BASIC APIs, direct
native reuse and cross-domain workload qualification. Keep this requirement in
`TASKS.md`; individual demos are not engine completion.
