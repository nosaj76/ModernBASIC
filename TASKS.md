# ModernBASIC revision 1 — consolidated project tasks

Updated: 2026-11-02. This is the single current project baseline. Revision numbers
were reset at the owner's request; the BASIC language/runtime version remains
0.8.2. Debian package epoch 1 keeps revision 1 newer than retired packages.

## Companion text processor — owner direction 2026-10-31

Preserve and develop the Software-Defined Text Processing Unit (SDTPU) concept
as separate Rust character/text/document infrastructure alongside mathcore.
The original note, scope, seven-section mapping and open TXT01–TXT14 contracts
are in [docs/SDTPU_TASKS.md](docs/SDTPU_TASKS.md). Character/text comes first within
this companion engine; document layout/export and optional VM/C interfaces follow.
Owner priority confirmed, 2026-10-31: finish mathcore first; implement this
companion engine afterward.
This records the request; no new text engine or complete word processor is claimed.
The active mathcore goal, dependency-free rule and standalone integration gates
remain in force. Existing user BASIC programs and Python font maker stay editable.

## Dependency-free reusable math engine — owner direction 2026-10-29

Owner's new practical direction, 2026-10-31: build mathcore-crate as a reusable
Virtual Math Coprocessor with a shared integer-only arithmetic foundation, its
own software x87/FPU and SIMD execution units, and separate scientific/math APIs.
The three supplied notes are preserved intact with hashes and mapped in
[docs/MATHCORE_VMC.md](docs/MATHCORE_VMC.md). VMC01–VMC09 in the standalone task
list cover profiles, state, x87, MMX, SSE through SSE4.2, adapter fixtures, no-FPU
subject composition and qualification. Wider vector extensions remain staged.
This records direction, not newly implemented compatibility. The dependency-free
rule, first native 386/486SX target, broader required math and standalone-first
ModernBASIC integration gate remain in force.

Latest owner clarification, 2026-10-31: the crate MUST function as a software
mathematical processor on hardware without an FPU. Native 386/486SX is the first
selected target; portable integer arithmetic should serve additional explicitly
qualified hardware. Implement software formats/rounding/flags and scientific
functions, with target ISA/helper/ABI and no-FPU execution evidence. This supersedes
the earlier treatment of software-FPU compatibility as optional. SFP01–SFP08 are
recorded in docs/MATHCORE_EXECUTION.md; the full engine remains incomplete.
Owner also asked for its own FPU/SSE: retain software floating-point and vector
functionality, plus explicitly scoped future virtual FPU/SSE register/control/ISA
compatibility. A no-SSE CPU uses software lanes; hardware acceleration is optional.

Owner extension, 2026-10-31: reusable native mathematical coprocessor capability
belongs to the same mathcore source base. Preserve portable CPU arithmetic,
explicit software precision/rounding, measured feature-safe acceleration and
bounded batches. Native 32/64-bit targets need qualification. Host service for
16-bit guests is distinct from native 16-bit execution. Full IEEE-format arithmetic needs separately specified acceptance contracts;
x87/SIMD execution units now belong to standalone processor development under
the later VMC direction. Define adapter contracts and fixtures early; C ABI and
real virtual-device/VM consumer integration remain unimplemented and deferred
until standalone qualification. Keep numerical kernels in this crate and
OS/device integration outside it. See docs/MATHCORE_EXECUTION.md.

**Overall completed/remaining implementation-task totals are not established.**
The earlier 147/97 headline was only a raw checkbox count and was misleading as
an overall work total. M15 financial mathematics has **15 verified components**.
The corrected summary and evidence are at the top of
[MATHCORE_TASKS.md](docs/MATHCORE_TASKS.md); `tools/mathcore-status.py` reports raw
entry counts separately and does not invent a total for unitemized work.
The checked entries below record completed checkpoints chronologically; their
remaining-work statements describe the state at that checkpoint and may be
superseded by later entries. The canonical subject list records current gaps.

- [x] Expose all eight qualified Gamma/Beta inverse operations through the
  standalone SoftwareProcessor, preserving arithmetic controls, flags and
  resumable/atomic transaction behavior. 31 crate tests (eight new), 1,498
  independent value/flag cases and 1,839 shared contract assertions pass.
  All 1,498 cases and shared contracts pass natively, with seven controls.
  Contract: [MATHCORE_INVERSE_PROCESSOR.md](docs/MATHCORE_INVERSE_PROCESSOR.md).
  Evidence: output/mathcore-inverse-processor-2026-11-01. Internal work accounting,
  in-command cancellation, queue/performance and aggregate gates remain open.

- [x] Add four certified integer-only Beta inverse probability/direct-log APIs,
  sharing exact polynomial probabilities, true tail comparisons and factor-work
  admission with existing kernels. 19,698 independent Beta cases, 10,092 retained
  inverse Gamma cases, 144 shared unit tests and 26 forward integration tests
  pass. 2,562 selected native cases and seven controls pass. Contract:
  [MATHCORE_INVERSE_BETA.md](docs/MATHCORE_INVERSE_BETA.md).
  Evidence: output/mathcore-inverse-beta-2026-11-01. Common processor dispatch,
  complete internal work accounting and all aggregate gates remain open.

- [x] Add four certified integer-only Gamma inverse probability/direct-log APIs,
  including true tail comparisons, strict extreme-coordinate rules and cumulative
  factor-term budgets. 10,092 independent host cases, 1,612 selected native cases,
  seven native controls, 137 distinct shared unit tests and ten existing Gamma
  integration tests pass. Contract: [MATHCORE_INVERSE_GAMMA.md](docs/MATHCORE_INVERSE_GAMMA.md).
  Evidence: output/mathcore-inverse-gamma-2026-11-01. Beta inverses, common processor
  dispatch, complete internal-work accounting and aggregate gates remain open.

- [x] Add a private certified positive-grid inverse-search foundation with exact
  dyadic comparisons, rounding witnesses and reusable callback budgets. Seven
  crate tests, 5,994 host and 5,994 native cases pass; each has 11,956 independent
  result/flag comparisons, plus seven native controls. Contract and limits:
  [MATHCORE_INVERSE_SEARCH.md](docs/MATHCORE_INVERSE_SEARCH.md). This does not complete
  inverse Gamma/Beta functions, comparator work accounting or aggregate gates.
- [x] Reuse one audited native test executable across separately loaded vector
  batches, preserving all 209,677 cases, 340 batch boundaries and seven existing
  controls. Seven parser tests and five new module controls pass; mathematical
  source is unchanged. Commands, independent baseline comparisons and measured
  verification cost: docs/MATHCORE_NATIVE_REUSE.md and
  output/mathcore-harness-reuse-2026-11-01. Mathematical performance and aggregate
  standalone gates remain open.
- [x] Qualify current Gamma/Beta integral source on the native no-FPU target and
  correct the unavailable division helper and emitted bit-scan instructions.
  209,677 native cases, 340 batches, seven controls and 881 shared checks
  pass; 157 focused Rust tests and 276,056 independent host comparisons pass
  after correction. The full earlier 200,040-case corpus is unchanged. Contract:
  docs/MATHCORE_INTEGRAL_NATIVE.md; evidence: output/mathcore-integral-native-2026-11-01.
  Whole-engine subject/ISA, internal resource, queue and physical-target gates remain open.
- [x] Add all eight Gamma/Beta integral operations to SoftwareProcessor, retaining
  the shared scalar kernels, formats and exception rules. 155 focused Rust tests,
  276,056 independent comparisons and 881 shared state/batch checks pass. Streaming,
  atomic rollback, host work admission and cancellation/deadline behavior are
  qualified. Contract: docs/MATHCORE_INTEGRAL_PROCESSOR.md; evidence:
  output/mathcore-integral-processor-2026-11-01. Current-source native execution,
  internal resource accounting, x87/SIMD and all aggregate gates remain open.
- [x] Resolve the diagnosed fractional-shape Beta exact-result family with
  bounded native dyadic powers and positive integer polynomials. 143 focused
  Rust tests and 225,328 independent comparisons pass (144,984 new; 80,344 retained).
  Exact values/logs retain exception flags; checked storage growth falls back to
  existing interval routes. Contract: docs/MATHCORE_BETA_POLYNOMIAL.md; evidence:
  output/mathcore-beta-polynomial-2026-11-01. Host processor integration is
  qualified by docs/MATHCORE_INTEGRAL_PROCESSOR.md; current-source native,
  resource/performance and all aggregate standalone engine gates remain open.
- [x] Add bounded software lower/upper Beta integral values and direct logarithms
  in all five formats; repair 224 unit-shape reciprocal-boundary precision failures.
  132 focused Rust tests, 80,344 Beta comparisons and 50,728 Gamma regressions pass.
  Contract: docs/MATHCORE_SOFTWARE_BETA_INTEGRALS.md; evidence:
  output/mathcore-software-beta-integrals-2026-11-01. The initially recorded 35
  exact-polynomial precision failures are resolved by the subsequent checkpoint
  in docs/MATHCORE_BETA_POLYNOMIAL.md. Processor/native integration and all
  aggregate standalone engine gates remain open.
- [x] Expose software lower/upper Gamma integral values and direct logarithms in
  all five formats with seven rounding modes, tininess policies and extended80
  precision. 119 focused Rust tests and 50,728 independent value/flag comparisons
  pass, including the repaired exact-leading-factor cancellation case. Contract:
  docs/MATHCORE_SOFTWARE_GAMMA_INTEGRALS.md; evidence:
  output/mathcore-software-gamma-integrals-2026-11-01. Beta integral APIs,
  processor/native qualification and all aggregate engine gates remain open.
- [x] Add private software incomplete-Gamma/Beta log-integral enclosures before
  public normalization/rounding. 107 focused Rust tests pass; 364 independent inputs
  produce 1092 verified enclosures and 0 lower-precision retries across 1,092 attempts.
  All inputs resolve at 512 bits; standalone/no_std checks pass. Contract:
  docs/MATHCORE_INCOMPLETE_INTEGRAL_LOGS.md; evidence:
  output/mathcore-incomplete-integral-logs-2026-11-01. Public values/log APIs,
  flags/processor integration, native qualification and whole-engine gates remain open.
- [x] Qualify the unchanged software Beta P/Q/log operations on the native no-FPU
  target: 200,040 reference cases (8,453 new), 190 normal batches, all seven controls,
  147 ternary state/transaction checks and 6,321 memory/runtime checks pass.
  Every normal executable passes the 386 instruction audit; the earlier 191,587
  vectors and all 809 core files are unchanged. Contract:
  docs/MATHCORE_SOFTWARE_BETA_NATIVE.md; evidence:
  output/mathcore-software-beta-native-2026-11-01. Observed stack is not a
  worst-case proof; full subject/resource/ISA and standalone gates remain open.
- [x] Add standalone software Beta P/Q and direct logarithms with ternary processor
  operations, exact-result paths and certified final rounding/flags. All 118
  focused Rust tests and 103,396 independent comparisons pass; 90 host workloads
  pass five zero-allocation trials each. Previous arithmetic bodies and fixtures
  remain unchanged; standalone/no_std compilation passes. Contract:
  docs/MATHCORE_SOFTWARE_BETA_PROBABILITY.md; evidence:
  output/mathcore-software-beta-probability-2026-11-01. Native no-FPU execution,
  worst-case resources and aggregate SFP/VMC/standalone gates remain open.
- [x] Route private Beta tails through the qualified power, ordinary and integration
  methods and compose cancellation-safe probability/log complements. All 107 focused
  tests and 572 new independent enclosures pass; all 158 tail inputs resolve at
  maximum precision, and 4,790 earlier Beta/Gamma checks pass. Prior arithmetic
  bodies remain unchanged. Contract: docs/MATHCORE_SOFTWARE_BETA_ROUTING.md;
  evidence: output/mathcore-software-beta-routing-2026-11-01. Public APIs,
  final rounding/flags, processor and native/resource gates remain open.
- [x] Extend private Beta endpoint integration to first shapes >=64 and supply
  upper factors retaining exact original coordinates. All 100 focused tests and
  249 new independent enclosures pass; 4,541 earlier Beta/Gamma checks and
  standalone/no_std compilation pass. Shared Taylor panels and the large-shape
  body remain unchanged. Contract: docs/MATHCORE_SOFTWARE_BETA_ORIENTATION.md;
  evidence: output/mathcore-software-beta-orientation-2026-11-01. Public routing,
  probability/log composition, rounding/processor and native/resource gates remain open.
- [x] Add private scaled Gamma increments, stable log(a*B(a,b)) and direct
  power-domain Beta logarithms that retain tiny corrections. All 93 focused
  tests and 837 new independent enclosures pass; 3,704 previous Beta/Gamma
  checks and standalone/no_std compilation pass. Earlier arithmetic bodies and
  constants remain unchanged. Contract: docs/MATHCORE_SOFTWARE_BETA_NORMALIZATION.md;
  evidence: output/mathcore-software-beta-normalization-2026-11-01. Public
  complement/domain routing, rounding/processor and native/resource gates remain open.
- [x] Add bounded private small-coordinate Beta factors with separately retained
  tiny signed corrections and mixed-shape endpoint integration with a valid b<1
  tail bound. All 89 focused tests, 647 new independent enclosures and 3,057 prior
  Beta/Gamma enclosures pass; standalone/no_std checks pass. Existing numerical
  algorithms remain unchanged. Reference/expectation corrections retain evidence.
  Contract: docs/MATHCORE_SOFTWARE_BETA_MIXED.md; evidence:
  output/mathcore-software-beta-mixed-2026-11-01. Mixed tiny tails, public APIs,
  processor/native/resource qualification and aggregate gates remain open.
- [x] Add bounded large-shape lower/upper incomplete-Beta integration with
  directed quadratic Taylor recurrences, positive global tail bounds and a
  65,535-unit work cap. All 84 focused tests and 201 new independent enclosures
  pass; earlier Beta/Gamma enclosures and standalone/no_std checks pass. Shared
  signed arithmetic and existing Gamma integration bodies remain unchanged.
  Contract: docs/MATHCORE_SOFTWARE_BETA_QUADRATURE.md; evidence:
  output/mathcore-software-beta-quadrature-2026-11-01. Small/mixed-shape tails,
  public APIs, processor/native/resource qualification and aggregate gates remain open.
- [x] Add private integer-only incomplete-Beta coordinates, true log-prefactors
  and bounded positive integral factors with explicit geometric remainders.
  All 81 focused tests and 1,551 independent enclosure checks pass; standalone
  and no_std compilation pass. Preserve exact centers, tiny residuals and explicit
  unresolved precision/work. Contract: docs/MATHCORE_SOFTWARE_BETA_COORDINATES.md;
  evidence: output/mathcore-software-beta-coordinates-2026-11-01. Large central
  factors, public P/Q/log APIs, processor/native/resource qualification and all
  aggregate SFP/VMC/standalone gates remain open.
- [x] Add integer-only incomplete-Gamma P/Q and direct logarithms in all five
  software formats, with processor commands, bounded rounding and flag policies.
  Corrected a<x<1 dispatch through independent boundary tests. All 364 focused
  tests, 863,808 independent comparisons and 191,587 native no-FPU cases pass;
  all 188,046 prior native vectors and 37,824 prior probability outputs remain
  byte-identical. Seventy-two workloads pass five zero-allocation trials.
  Contract: docs/MATHCORE_SOFTWARE_GAMMA_PROBABILITY.md; evidence:
  output/mathcore-software-gamma-probability-2026-11-01. Remaining special functions,
  no-FPU subject routing and aggregate SFP/VMC/standalone gates remain open.
- [x] Add bounded directed integration for large-shape Gamma factors, with
  signed Taylor intervals, proven panel/global remainder bounds and adaptive
  local work. All 51 isolated arithmetic tests, 201 new enclosure checks and
  1,104 prior factor checks pass; standalone cargo check passes. The new corpus
  covers 67 inputs at three precisions, up to near-maximum binary128 shapes.
  Evidence: output/mathcore-software-gamma-tails-2026-11-01; contract:
  docs/MATHCORE_SOFTWARE_GAMMA_FACTORS.md. Public tail composition, final rounding,
  native qualification and aggregate completion gates remain open.
- [x] Implement private integer-only incomplete-Gamma lower-series, upper-factor,
  small-coordinate and tiny-shape upper-tail enclosures. Independent tests cover
  1,104 intervals over 368 inputs and three precisions, plus budget exhaustion.
  Contract: docs/MATHCORE_SOFTWARE_GAMMA_FACTORS.md; evidence:
  output/mathcore-software-incomplete-gamma-2026-11-01. This internal component
  does not complete public probability APIs, native execution or aggregate gates.
- [x] Add integer-only log1pmx, Gamma deviance and Gamma kernel/log-kernel
  with near-mean displacement, normalized products and strict tiny corrections.
  All 351 focused tests, 935,538 reference cases, 14,112 boundary checks
  and 588,572 previous Beta outputs pass. Native checks pass 188,046 cases,
  preserving all 182,175 prior vectors; runtime checks, seven controls and the
  unchanged ISA audit pass. Seventy-two workloads pass five zero-allocation
  trials. Contract: docs/MATHCORE_SOFTWARE_GAMMA_KERNEL.md; evidence:
  output/mathcore-software-gamma-kernel-2026-11-01. Incomplete-integral tails,
  remaining special functions, no-FPU subject routing and aggregate gates stay open.
- [x] Add integer-only beta and log_beta with unrounded recurrence products,
  stable unequal-shape logarithms, strict tiny-shape witnesses and directed unit
  reductions. All 341 focused tests, 588,572 reference cases,
  26,460 independent boundary checks and 420,336 previous Gamma outputs pass.
  Native checks pass 182,175 cases, preserving all 178,007 prior vectors, with
  runtime checks, seven controls and unchanged ISA audit. Thirty-six workloads
  pass five zero-allocation trials. Contract: docs/MATHCORE_SOFTWARE_BETA.md;
  evidence: output/mathcore-software-beta-2026-11-01. Incomplete integrals,
  remaining special functions, no-FPU subject routing and aggregate gates stay open.
- [x] Add integer-only gamma, reciprocal_gamma, log_gamma and log_gamma_signed
  with exact factorials, strict tiny-input limits and unrounded negative-domain
  reflection. All 331 focused tests, 420,336 reference cases, 33,712 independent
  transition checks and 95,816 previous shifted-log outputs pass. Native checks
  pass 178,007 cases, preserving all 173,458 prior vectors, with runtime checks,
  seven controls and unchanged ISA audit. Seventy-two workloads pass five
  zero-allocation trials. Contract: docs/MATHCORE_SOFTWARE_GAMMA.md; evidence:
  output/mathcore-software-gamma-2026-11-01. Beta, remaining special functions,
  no-FPU subject routing and aggregate gates remain open.
- [x] Add integer-only log_gamma_1p in five formats, preserving tiny increments
  near Gamma(1)/Gamma(2), the -1 pole and large normalized products. Exact rational
  bounds certify 272 coefficients; 309 focused tests, 95,816 independent cases,
  3,920 additional transition-neighbor cases and 192,318 unchanged previous
  scaled-gamma outputs pass. Native qualification passes 173,458 cases including
  all 172,427 old vectors, runtime checks and seven controls with unchanged audit.
  Eighteen workloads pass five zero-allocation trials. Contract:
  docs/MATHCORE_SOFTWARE_LOG_GAMMA.md; evidence:
  output/mathcore-software-log-gamma-2026-11-01. Other Gamma/Beta, special functions,
  subject routing and aggregate completion gates remain open.
- [x] Add integer-only Stirling correction and scaled/log-scaled gamma over
  the full positive input range. Shared normalized interval/exp/log helpers move
  unchanged; 64 normalized Bernoulli coefficients have exact rational proofs.
  298 focused tests, 192,318 independent cases, all 373,228 prior error
  outputs and 172,427 native cases pass. All 170,072 prior native vectors,
  seven controls, runtime checks and unchanged ISA audit pass; 36 workloads
  allocate zero times in five trials. Contract: docs/MATHCORE_SOFTWARE_STIRLING.md;
  evidence: output/mathcore-software-stirling-2026-11-01. Remaining Gamma/Beta,
  special functions, subject routing and aggregate gates remain open.
- [x] Add integer-only erf, erfc, erfcx and log_erfc with directed interval
  certification, preserved probability tails and shared exp/log/fixed arithmetic.
  287 focused tests, 373,228 independent cases and 170,072 native cases
  pass, preserving all 165,495 previous native vectors; seven controls, runtime
  checks and unchanged ISA audit pass. Seventy-two workloads pass five trials
  with zero allocations. Contract: docs/MATHCORE_SOFTWARE_ERROR_FUNCTIONS.md;
  evidence: output/mathcore-software-error-functions-2026-11-01. Remaining special
  functions, full no-FPU subject routing and aggregate gates remain open.
- [x] Add certified integer-only cbrt and full-i128 nth_root, recognizing exact
  roots and rounding reciprocal roots only once. The existing perfect-root and
  exp endpoint helpers are shared. 274 focused tests, 840,104 independent
  root cases and all 847,272 prior base-function cases pass. Native qualification
  passes 165,495 cases, preserving all 145,573 prior vectors; runtime checks,
  seven controls and unchanged ISA audit pass. Thirty-six workloads pass five
  zero-allocation trials. Contract: docs/MATHCORE_SOFTWARE_ROOTS.md; evidence:
  output/mathcore-software-roots-2026-11-01. Remaining scientific/special functions,
  rational/complex powers, full subject routing and aggregate gates stay open.
- [x] Add certified integer-only exp2/exp10/log2/log10/log_base with exact
  rational-log dependency recognition and one final rounding. Shared real-power
  and ratio kernels are reused. 256 focused Rust tests, 847,272 independent
  reference cases and 145,573 native no-FPU cases pass; all 134,620 previous
  native vectors remain byte-identical. Fifty workloads pass five zero-allocation
  trials; 6,321 runtime checks, seven controls and the unchanged ISA audit pass.
  Contract: docs/MATHCORE_SOFTWARE_EXPONENTIAL_BASES.md; evidence:
  output/mathcore-software-exponential-bases-2026-11-01. General roots, remaining
  scientific/special functions, full subject routing and aggregate gates stay open.
- [x] Add twelve certified integer-only reciprocal and inverse-reciprocal APIs,
  preserving strict tiny corrections, signed poles, domains and final rounding.
  236 focused Rust tests, 1,105,332 independent new cases, all 428,740 prior
  inverse-angle and 539,136 prior hyperbolic cases, and 134,620 native no-FPU
  cases pass. The prior 104,173 native vectors remain byte-identical; 6,321 runtime
  checks and seven controls pass. 120 new workloads pass five zero-allocation
  trials. Contract: docs/MATHCORE_SOFTWARE_RECIPROCALS.md; evidence:
  output/mathcore-software-reciprocals-2026-11-01. Remaining scientific/special
  functions, full subject routing and aggregate SFP03–SFP08/VMC gates remain open.
- [x] Add certified integer-only degree/PI-phase sine/cosine/tangent, normalized
  sinc and angle conversions with exact rational reduction and scaled rounding.
  220 focused Rust tests, 940,938 independent angle-unit cases, all 276,888 prior
  radian cases and 104,173 native no-FPU cases pass; 6,321 runtime checks and
  seven controls pass. Ninety new workloads pass five zero-allocation trials.
  Contract: docs/MATHCORE_SOFTWARE_ANGLE_UNITS.md; evidence:
  output/mathcore-software-angle-units-2026-11-01. Reciprocal/special functions,
  full subject routing and aggregate SFP03–SFP08/VMC gates remain open.
- [x] Add certified integer-only sinh/cosh/tanh/asinh/acosh/atanh with scaled
  exponentials, directed roots/logarithms and exact domain/exception handling.
  204 focused Rust tests, 539,136 independent cases and 77,768 native no-FPU
  cases pass, with 6,321 runtime checks and seven controls. Sixty new direct/atomic
  workloads pass five zero-allocation trials. Contract:
  docs/MATHCORE_SOFTWARE_HYPERBOLIC.md; evidence:
  output/mathcore-software-hyperbolic-2026-11-01. Remaining angle/special APIs,
  full subject routing and aggregate SFP03–SFP08/VMC gates remain open.
- [x] Add certified integer-only atan/atan2/asin/acos, preserving tiny ratios,
  signed quadrants, endpoint precision, domains and explicit rounding/flags.
  189 focused Rust tests, 428,740 independent inverse-angle cases and 67,608 native
  no-FPU cases pass, with 6,321 runtime checks and seven controls. Forty-eight new
  direct/atomic workloads pass five zero-allocation trials. Contract:
  docs/MATHCORE_SOFTWARE_INVERSE_TRIGONOMETRY.md; evidence:
  output/mathcore-software-inverse-trigonometry-2026-11-01. Hyperbolic/remaining
  angle APIs, full subject routing and aggregate SFP03–SFP08/VMC gates remain open.
- [x] Add certified integer-only sine/cosine/tangent with all-range reduction,
  exact-rational constants, tiny-input rounding, explicit uncertainty errors and
  shared processor commands. 174 focused Rust tests, 276,888 independent trig
  cases (including every binary16 encoding for all three functions), 60,315 native
  no-FPU cases, 6,321 runtime checks and seven controls pass. Forty-two new
  direct/atomic workloads pass five zero-allocation trials. Contract:
  docs/MATHCORE_SOFTWARE_TRIGONOMETRY.md; evidence:
  output/mathcore-software-trigonometry-2026-11-01. Inverse/hyperbolic functions,
  full subject routing and aggregate SFP03–SFP08/VMC gates remain open.
- [x] Add certified real powers with exact floating-exponent classification,
  exact dyadic roots before final rounding, negative-base/domain rules and
  outward ln/product/exp composition. 162 focused Rust tests, 694,114 independent
  real-power cases, 496,360 integer-power and 340,188 scientific regressions pass;
  49,106 native no-FPU cases and 78 zero-allocation workloads pass. Contract:
  docs/MATHCORE_SOFTWARE_REAL_POWER.md; evidence:
  output/mathcore-software-real-power-2026-11-01. Trig/inverse/hyperbolic, complete
  subject routing and aggregate SFP03–SFP08/VMC gates remain open.
- [x] Add certified integer-exponent powers for all five software formats, the
  complete i128 exponent range, exact dyadics/midpoints and shared processor
  batches. 150 focused Rust tests, 496,360 independent power cases, 340,188
  scientific regression cases and 35,241 native no-FPU cases pass. All 54
  optimized workloads allocate zero times. Fix compiler-emitted REP BSF/TZCNT
  through a leading-scan identity; retain the strict 386 audit and rejected
  evidence. Contract: docs/MATHCORE_SOFTWARE_INTEGER_POWER.md; evidence:
  output/mathcore-software-integer-power-2026-11-01. General real powers,
  remaining subjects and aggregate SFP03–SFP08/VMC gates remain open.
- [x] Add cancellation-safe integer-only exp_m1/ln_1p for all five formats,
  preserving tiny increments, exact domain boundaries, reduced-precision midpoint
  direction and tininess/exception contracts. 140 focused Rust tests, 340,188
  independent four-function cases and 27,519 native no-FPU cases pass; 34 optimized
  workloads allocate zero times. Contract: docs/MATHCORE_SOFTWARE_SCIENTIFIC.md;
  evidence: output/mathcore-software-increments-2026-11-01. Powers, trig/inverse/
  hyperbolic functions, subject routing and aggregate SFP03–SFP08/VMC gates remain open.
- [x] Implement and independently qualify certified integer-only exp/ln for
  binary16/32/64, extended80 and binary128, including processor dispatch, explicit
  precision-limit errors, rounding and exception contracts. 131 focused Rust
  tests, 170,164 independent scientific cases and 24,858 native no-FPU cases pass;
  sixteen optimized workloads allocate zero times. Contract:
  docs/MATHCORE_SOFTWARE_SCIENTIFIC.md; evidence:
  output/mathcore-software-exp-log-2026-11-01. Remaining scientific functions,
  no-FPU subject routing and all aggregate SFP03–SFP08/VMC gates remain open.
- [x] Implement caller-owned software processor command dispatch, shared format
  controls/flags, checked conversions/comparisons, resumable bounded batches and
  atomic batches with caller scratch. Preserve the existing ExecutionContext's
  no-partial-result error policy through its adapter. 89 focused tests (15 new),
  22,190 native exact-reference cases, native batch checks, 6,321 runtime cases,
  seven controls and six zero-allocation workloads pass. Contract:
  docs/MATHCORE_PROCESSOR.md; evidence: output/mathcore-processor-2026-11-01.
  Full no-FPU scientific/subject routing, queue admission, worst-case stack and
  all aggregate SFP03–SFP08/VMC gates remain open.

- [x] Qualify the first freestanding 386 integer-only arithmetic leaf: pinned
  standard-core build, project memory runtime, 16,633 exact-reference cases,
  6,321 memory cases, isolated sticky state and seven negative/boundary controls.
  All executable bytes pass a conservative 386 ISA audit; no unresolved helpers
  or allocator are linked. QEMU 486 execution with FPU disabled is distinguished
  from physical 386/486SX qualification. Contract: docs/MATHCORE_NATIVE386.md;
  evidence: output/mathcore-native386-2026-11-01. SFP03–SFP08, whole-engine no-FPU
  routing, all VMC gates and SDTPU remain open; mathcore comes first.

- [x] Qualify all four legacy x87 environment layouts through one checked,
  allocation-free codec, preserving the prior protected32 API. Add the complete
  documented 80387 ESC/form classifier with explicit native-API/pending/reserved
  distinctions. 96 targeted Rust tests (14 new), 13,728 specification cases,
  6,864 protected hardware cases, 87,936 prior state cases, optimized same-source
  parity and nine zero-allocation workloads pass. Contracts:
  docs/MATHCORE_X87_ENVIRONMENTS.md and docs/MATHCORE_X87_80387_PROFILE.md;
  evidence: output/mathcore-x87-environments-2026-10-31. No complete instruction
  executor, real-mode execution, native386 or aggregate VMC completion claim.

- [x] Qualify the caller-owned x87 protected32 state and raw-transfer subset:
  register stack, controls, pending exceptions, masked/unmasked stack faults and
  save/restore. 121 targeted Rust tests (17 new), 87,936 host x87 comparison cases,
  optimized parity, core-only compilation and three zero-allocation workloads
  pass. Contract: docs/MATHCORE_X87_STATE.md; evidence:
  output/mathcore-x87-state-2026-10-31. This is scoped progress, not complete x87
  or native386 support; all aggregate VMC gates remain open. The former full
  suite was not rerun for unchanged arithmetic. SDTPU follows finished mathcore.

- [x] Complete SFP02 extended80/binary128 arithmetic and conversion contracts,
  reduced extended precision and explicit unsupported/pseudo-denormal handling.
  Reuse one four-word integer kernel across wide formats and earlier fused
  fallbacks. All 2,522 Rust tests (14 new), 63,652 new exact cases and 281,726
  earlier cases pass; optimized parity, core-only host compilation and 69 zero-
  allocation workloads are verified. Contract: docs/MATHCORE_EXTENDED_FLOAT.md;
  evidence: output/mathcore-extended-float-2026-10-31. Full-engine no-FPU routing,
  scientific functions, native386, x87/SSE and other subject gates remain open.

- [x] Complete and independently qualify SFP01 binary16/32/64 arithmetic with
  fused multiply-add, exact IEEE remainder, integral rounding and checked signed/
  unsigned integer conversions through 128 bits. Reuse exact dyadic storage and
  shared rounding; no external dependencies or heap allocations in the leaf.
  All 2,508 Rust tests (12 new), 180,910 new exact bit/value/flag cases, 100,816
  unchanged earlier cases, optimized parity and 21 zero-allocation workloads pass.
  Core-only compilation/object review passes on the host; native 386/486SX,
  extended80/binary128, software scientific functions and x87/SSE gates remain
  open. Contract: docs/MATHCORE_SOFTWARE_FLOAT.md; evidence:
  output/mathcore-software-float-operations-2026-10-31. No integration/image work.

- [x] Qualified the first integer-only software FPU arithmetic foundation:
  binary16/32/64 add/subtract/multiply/divide/square-root, comparisons and format
  conversions; shared rounding modes, tininess, signed special values and sticky
  exception state. The same leaf compiles with core only, without std.
  All 2,496 Rust tests (13 new), 100,816 independent exact encoding/flag cases,
  optimized parity and 18 zero-allocation CPU workloads pass. Contract:
  docs/MATHCORE_SOFTWARE_FLOAT.md; evidence: output/mathcore-software-float-2026-10-31.
  This does not complete SFP01–SFP08: FMA/remainder/integer operations, extended
  formats, software scientific functions, native 386/486SX qualification and
  virtual FPU/SSE compatibility remain open. Full mathcore completion unproven.

- [x] Qualified explicit rate quote conversions and annualization in the native
  standalone crate: effective/nominal/continuous/simple/discount, rational units,
  loss/pole boundaries, dated/value returns and reusable bounded batches. Shared
  Rational/calendar/log-exp kernels preserve tiny rates and cumulative limits.
  All 2,483 Rust tests (21 new), 1,636 independent cases / 4,758 checks,
  optimized/debug parity and four CPU workloads pass. Contract:
  docs/MATHCORE_RATE_CONVERSION.md; evidence: output/mathcore-rate-conversions-2026-10-31.
  F02-023 closed: 23 verified / two open in this scoped register; M15 has fifteen
  verified components. Overall work counts remain unknown. Market reference-period
  yields, inverse loans, other subjects and standalone CPU/queue/platform gates
  remain open. Owner's native coprocessor use case retained in execution design.

- [x] Qualified native arbitrary-quantum rounding, explicit currency units,
  monetary posting/arithmetic/conversion and currency-rounded loan schedules.
  Shared the exact amortization engine with separate interest/payment rules,
  combined/component requests, final payoff and retained rounding adjustments.
  Reused repeated requests and whole totals to keep 360-payment loans within
  the unchanged default work budget; guarded wide scratch unit counts.
  All 2,462 Rust tests (28 new), 2,229 independent cases / 64,183 checks,
  1,127 included Decimal cross-checks, 4,431 unchanged financial/loan,
  1,038 GCD and 172 quadrature outputs pass; four CPU workloads verified.
  Contract: docs/MATHCORE_MONETARY.md; evidence: output/mathcore-monetary-2026-10-31.
  F02 currency-rounded-amortization contract closed: 22 verified / three open
  within that register; M15 has fourteen named components. Broader accounting,
  quote/market/inverse-loan and mandatory engine gates remain open. Overall
  completed/remaining implementation totals remain unestablished.

- [x] Qualified exact standalone loan/amortization plans and ledgers: regular,
  changing and dated simple factors; balloon/level/series/equal-principal payments;
  extras, caps, final payoff and debt-increase policies; summary/detailed results.
  Improved shared exact integer GCD and removed redundant loan arithmetic so
  360-payment schedules fit the unchanged default work limit. All 2,434 Rust
  tests (28 new), 296 loan cases / 16,813 checks, 1,038 GCD cases, 4,135 previous
  financial and 172 quadrature cases pass; four repeated CPU workloads have
  independently verified exact outputs. Contracts: docs/MATHCORE_AMORTIZATION.md
  and docs/MATHCORE_GCD_BATCH.md. Evidence: output/mathcore-amortization-2026-10-31.
  F02 has 21 verified and four open named contracts; M15 has thirteen verified
  components. These are scoped records, not overall completed/remaining totals.
  Currency rounding, quote/market conventions, inverse loan models and broader
  scientific/engineering/financial/CPU/queue/platform requirements remain open.

- [x] Qualified standalone fixed-income cashflows, exact coupon amounts/accrual,
  explicit payment/ex-coupon ownership, three dated-yield conventions, analytical
  price risk, settlement curve rebasing and bracketed yield fitting with fresh
  exact price checks. All 2,406 Rust tests (29 new), 269 independent cases /
  5,235 checks, 3,866 unchanged financial and 172 quadrature outputs pass;
  four CPU/allocation workloads have independently verified results.
  Contracts: docs/MATHCORE_FIXED_INCOME.md; evidence:
  output/mathcore-fixed-income-2026-10-31. M15 has twelve verified components;
  market-specific yield conventions, loan amortization, currency/risk and wider
  mandatory coverage remain open. Overall implementation totals remain unknown.

- [x] Qualified dependency-free European/American/Bermudan CRR option trees with
  exact represented-input admissibility, scaled rolling induction, explicit grid
  exercise and deterministic/expiry limits. Fixed a reproduced false rejection
  near probability boundaries and retained tiny transition probabilities.
  All 2,377 Rust tests (26 new), 280 independent cases / 2,539 checks,
  3,586 unchanged financial and 172 quadrature outputs pass; four complete-call
  CPU/allocation workloads recorded. Contracts: docs/MATHCORE_OPTION_TREE.md;
  evidence: output/mathcore-option-tree-2026-10-31. M15 now has eleven verified
  components. Historical raw checkbox count: 147 checked / 97 unchecked (superseded as a completion measure); fixed-income,
  currencies, risk, tree sensitivities and other mandatory subjects remain open.

- [x] Qualified standalone European implied volatility with explicit brackets,
  prepared pricing reuse, unrounded signed residuals and fresh exact price-fit
  criteria. Rounded price boundaries and collapsed ranges have explicit outcomes;
  search/resource limits never imply a fitted volatility. All 2,351 Rust tests
  (24 new), 222 independent cases / 4,748 checks, 3,364 unchanged financial and
  172 unchanged quadrature outputs pass; four CPU workloads recorded. Contracts:
  docs/MATHCORE_IMPLIED_VOLATILITY.md; evidence:
  output/mathcore-implied-volatility-2026-10-31. M15 lists ten verified components;
  historical raw checkbox count: 147 checked / 97 unchecked (superseded as a completion measure). American/other instrument
  models, currency/risk and wider mandatory coverage remain open.

- [x] Qualified standalone European call/put pricing and six sensitivities:
  continuous-yield Black-Scholes, retained scaling and small time values,
  explicit Gaussian/Mills/integral paths, shared bounded quadrature and exact
  whole-sum sensitivity algebra. All 2,327 Rust tests (17 new), 302 independent
  cases / 2,396 checks, 3,062 unchanged financial and 172 unchanged quadrature
  outputs pass; four CPU workloads recorded. Contracts:
  docs/MATHCORE_EUROPEAN_OPTIONS.md; evidence:
  output/mathcore-european-options-2026-10-31. M15 lists nine verified components;
  historical raw checkbox count: 147 checked / 97 unchecked (superseded as a completion measure). Implied volatility, American/
  other instrument models, currency/risk and wider mandatory coverage remain open.

- [x] Qualified standalone fixed-cashflow curve calibration: exact price/amount/
  factor-bound inputs, native calendar grouping and curve/search composition,
  fresh exact price-fit comparisons, cumulative work and accepted-prefix recovery.
  All 2,310 Rust tests (24 new), 129 independent cases / 4,410 checks and 2,933
  unchanged previous financial outputs pass; four CPU workloads recorded.
  Contracts: docs/MATHCORE_CURVE_CALIBRATION.md; evidence:
  output/mathcore-curve-calibration-2026-10-31. M15 now lists eight verified
  components; historical raw checkbox count: 147 checked / 97 unchecked (superseded as a completion measure). Instrument/quote
  conventions, pricing/sensitivities, currency/risk and mandatory scientific/
  engineering/CPU/queue/standalone gates remain open.

- [x] Qualified the standalone discount-curve foundation: exact positive knots,
  exact calendar coordinates, explicit local log-linear interpolation and
  extrapolation, retained/log factor queries, stable zero/forward rates and
  whole-sum cashflow valuation. Shared growth arithmetic preserves previous
  outputs/work. All 2,286 Rust tests (23 new), 767 independent cases and
  2,166 unchanged earlier financial outputs pass; four CPU workloads recorded.
  Contracts: docs/MATHCORE_DISCOUNT_CURVES.md; evidence:
  output/mathcore-discount-curves-2026-10-31. M15 now lists seven verified
  components; historical raw checkbox count: 147 checked / 97 unchecked (superseded as a completion measure). Calibration,
  instrument/credit/inflation curve contracts, pricing/risk and broader mandatory
  scientific/engineering/CPU/queue/standalone qualification remain open.

- [x] Qualified prepared irregular-date investment returns in standalone Rust:
  exact calendar/cashflow grouping, signed normalized residuals, explicit one-
  bracket search diagnostics, cumulative callback/search budgets and immutable
  recovery. All 2,263 Rust tests (18 new), 325 independent reference cases
  and 1,841 unchanged prior financial outputs pass; three CPU workloads recorded.
  Contracts: docs/MATHCORE_DATED_IRR.md; evidence:
  output/mathcore-dated-irr-2026-10-31. M15 now lists six verified components;
  historical raw checkbox count: 147 checked / 97 unchecked (superseded as a completion measure). Curves, pricing, conventions,
  currency/accounting, portfolio/risk and full standalone gates remain open.

- [x] Qualified annual nominal-rate/frequency-duration compounding and exact
  periodic IRR root sets through shared arithmetic/Sturm kernels. Preserved tiny
  rates/counts, multiple/repeated returns, exact extreme values and current-policy
  work/storage checks. All 2,245 Rust tests (19 new), 411 independent cases,
  prior financial outputs and 271 root-isolation cases pass. Source contracts and
  CPU evidence: docs/MATHCORE_NOMINAL_IRR.md and
  output/mathcore-nominal-irr-2026-10-31. The M15 component table now separates
  verified capabilities from remaining work; the historical raw checkbox count was 147/97 (superseded as a completion measure).
  Dated returns/IRR/yields, curves/pricing/currency/risk and full standalone scope
  remain open.

- [x] Qualified numerical fractional-period annuities/compounding and dated
  cashflow valuation with explicit exact-versus-transcendental diagnostics. Shared
  rational factors/valuation preserve tiny rates, large intermediate cancellation,
  exact calendar fractions and current caller limits. All 2,226 Rust tests
  (15 new), 598 independent high-precision cases, 832 unchanged exact financial
  cases and 22,147 elementary reference cases pass; four CPU workloads are recorded.
  Contracts/evidence: docs/MATHCORE_FRACTIONAL_FINANCIAL.md and
  output/mathcore-fractional-financial-2026-10-31. IRR/returns/yields, nominal
  frequency-duration models, curves/calibration, pricing/sensitivities,
  currency/accounting and the broader financial/CPU/standalone scope remain open.

- [x] Qualified an exact native financial foundation: reusable integer-period
  annuity factors, PV/FV/payment, compounding and nominal-rate conversion, regular
  NPV and supplied-discount valuation. Shared RationalKernel work preserves tiny
  rates, exact cancellation, current caller limits and final nearest-even output.
  All 2,211 Rust tests (14 new), 832 independent Fraction cases and four
  five-repeat CPU workloads pass; 294,884 prior calendar outputs remain identical.
  Contracts/evidence: docs/MATHCORE_EXACT_FINANCIAL.md and
  output/mathcore-exact-financial-2026-10-31. This advances M15; fractional-period
  numerical and dated valuation, IRR, curves/calibration, pricing/sensitivities,
  currencies/risk and broader financial/CPU/standalone requirements remain open.

- [x] Qualified dependency-free Gregorian CivilDate over all i32 astronomical
  years, exact signed day-count fractions, explicit business-calendar rules,
  anchored schedules and adjusted accrual/payment periods. Shared work/sort and
  native rational kernels preserve limits, cancellation and all-or-error results.
  Corrected inherited modified-roll checks to compare year and month. All
  2,197 Rust tests (16 new), 294,884 independent result cases and three
  five-repeat CPU workloads pass; 220 prior evolution outputs remain byte-identical.
  Contracts/symbol audit: docs/MATHCORE_CALENDAR.md and
  docs/MATHCORE_CALENDAR_SYMBOLS.json; evidence: output/mathcore-calendar-2026-10-31.
  This completes M16's declared calendar extraction/qualification scope; M15
  financial models and all broader coverage/CPU/queue/standalone gates remain open.

- [x] Qualified native coordinate/state/rate/residual schemas, typed callbacks and
  Jacobian blocks, borrowed accepted-state views, component tolerances and
  derivative increments. Original statuses/reports are preserved, including
  nonconvergence and recovery. All 2,181 Rust tests (14 new), 220 independent
  Fraction cases and six five-repeat CPU workloads pass; 240 prior quantity/
  covariance outputs remain byte-identical. Analytic implicit/residual steps
  allocate zero after setup; accepted solver state composes with exact covariance
  without losing quantity kinds. Together with the previous typed-Jacobian and
  covariance interfaces, this completes M14 composition under explicit contracts.
  It does not certify all solver families, automatic flow sensitivities or model
  errors. Contracts/evidence: docs/MATHCORE_EVOLUTION_QUANTITIES.md and
  output/mathcore-evolution-quantities-2026-10-31. Broader mandatory subject and
  standalone CPU/queue/platform gates remain open.

- [x] Qualified shared unit-aware layouts, borrowed finite SI views, atomic typed
  conversion and sticky checked callbacks. Typed Jacobians reuse the native
  derivative kernel with one cumulative budget and retain every status/estimate.
  Exact covariance transport preserves the represented derivative coefficients
  and source diagnostics; missing/nonconverged Jacobians are rejected. All
  2,167 Rust tests (15 new), 240 independent cases, complete native-report
  parity and six five-repeat CPU workloads pass; 342 prior distribution outputs
  remain byte-identical. Measured adapter allocations are zero for 100 RK4 steps;
  typed Jacobians allocate no more than the raw kernel. Contracts/evidence:
  docs/MATHCORE_QUANTITY_COMPOSITION.md and
  output/mathcore-quantity-composition-2026-10-31. Prepared time/state/residual
  schemas, typed component tolerances and wider solver/sensitivity/error-evidence
  composition remain open; the M14 composition item is not yet complete.

- [x] Qualified retained nonlinear distribution propagation and population-content
  intervals: joint moments precede marginal sorting, each selected column is stored
  and sorted once, request order is preserved and late failures roll back sampler
  state. Certified preselected content bounds distinguish population fraction from
  confidence and retain infinite endpoints when finite evidence is insufficient.
  Optional numerical/model bounds preserve unknown components explicitly. All
  2,152 Rust tests (13 new), 342 independent cases, 108 exhaustive
  discrete-law content checks and six five-repeat CPU workloads pass; the previous
  600 oracle outputs remain byte-identical. Contracts/evidence:
  docs/MATHCORE_DISTRIBUTION_PROPAGATION.md and
  output/mathcore-distribution-propagation-2026-10-31. This completes the covariance/
  uncertainty extraction item under documented methods and assumptions. Typed
  solver/sensitivity composition and broader mandatory coverage remain open.

- [x] Qualified native interval/error primitives: exact empirical quantile and
  shortest fixed-count intervals report actual closed sample mass. Reusable
  fixed-horizon IID quantile confidence plans share certified binomial ranks,
  retain exact probability-step distinctions and expose infinite endpoints.
  Numerical/model error budgets keep unknown components explicit and widen
  intervals only with both supplied uniform bounds. All 2,139 Rust tests
  (18 new), 600 independent cases, 117 discrete-law confidence checks
  and six five-repeat CPU workloads pass; 432 previous reference outputs remain
  byte-identical. Contracts/evidence: docs/MATHCORE_COVERAGE_INTERVALS.md and
  output/mathcore-coverage-intervals-2026-10-31. Retained-distribution sampling,
  population-content/coverage adequacy, automatic solver/model evidence and
  typed composition remain open.

- [x] Qualified adaptive nonlinear sampling: mean/variance/quantile targets use
  exact rational batch error and batch-versus-pooled comparisons with consecutive
  checks. Shared fixed/adaptive state preserves sampler rollback and global
  budgets; bounded selected-column quantiles reuse all nine order kernels.
  All 2,121 Rust tests (17 new), 432 independent Fraction/replay cases
  and nine five-repeat CPU workloads pass; all 432 previous reference outputs
  remain byte-identical. Contracts/evidence: docs/MATHCORE_ADAPTIVE_SAMPLING.md
  and output/mathcore-adaptive-sampling-2026-10-31. Estimated stability is not
  a coverage or model-error guarantee; explicit errors, coverage adequacy and
  typed solver composition remain open.

- [x] Qualified general fixed-sample nonlinear propagation: native joint-input/model
  callbacks share bounded sampling and exact streaming multivariate moments.
  Storage does not retain trials; sampler publication is transactional. Reports
  distinguish conditional mean sampling errors from unestimated model errors.
  All 2,104 Rust tests (14 new), 432 independent Fraction/replay cases
  and four five-repeat CPU workloads pass. Contracts/evidence:
  docs/MATHCORE_NONLINEAR_SAMPLING.md and output/mathcore-nonlinear-sampling-2026-10-31.
  Adaptive/distribution/coverage diagnostics, explicit model error and typed
  solver composition remain open.

- [x] Qualified exact chaining and Gaussian quadratic part: shared rational matrix
  products retain covariance through repeated maps, consume native statistics
  covariance directly and preserve typed dimension labels. Gaussian quadratic
  models return exact mean shifts and linear/nonlinear covariance contributions.
  All 2,090 Rust tests (17 new), 624 independent moment/Fraction cases
  and five five-repeat CPU workloads pass. Contracts/evidence:
  docs/MATHCORE_EXACT_COVARIANCE.md and output/mathcore-exact-covariance-2026-10-31.
  At that checkpoint, general nonlinear/distribution/model-error and typed
  solver composition remained open; see the subsequent nonlinear checkpoint.

- [x] Qualified first-order part: reusable exact dense/sparse/rational semidefinite
  decisions and native J U J^T propagation share bounded rational/triple-product
  kernels. Standard deviations precede variance rounding; typed dimensions and
  atomic caller buffers are verified. All 2,073 Rust tests (25 new),
  755 independent principal-minor/Fraction cases and four five-repeat CPU
  workloads pass. Contracts/evidence: docs/MATHCORE_UNCERTAINTY.md and
  output/mathcore-uncertainty-2026-10-31.
  At that checkpoint, model error, exact output chaining and full solver
  composition remained open; see the subsequent exact covariance checkpoint.

- [x] Extract dimensional algebra, exact conversions and temperature
  point/interval semantics without foreign integer/rational dependencies.
  Native Units/Dimensions/Quantity and prepared exact conversions share one
  cumulative arithmetic budget; caller-buffer publication is atomic. All
  2,048 Rust tests (18 new), 3,626 independent Fraction cases and four
  five-repeat CPU workloads pass. Integer dimensions, bounded ASCII catalog and
  SI rounding boundaries are explicit. Contracts/evidence: docs/MATHCORE_UNITS.md
  and output/mathcore-units-2026-10-31. Uncertainty and solver composition remain open.

- [x] Qualify native separable-Hamiltonian symplectic Euler and Verlet in both orderings, canonical phase state, atomic reset/step and bounded fixed-step integration. Existing callbacks/resource admission and shared exact interval updates are reused; a proven exact-width FMA path retains the accumulator fallback. All 2,030 Rust tests (26 new), 1,204 independent cases / 105,136 callback-state checks, oscillator/orbit convergence and long-run invariants pass. Twenty five-repeat CPU workloads expose the initial stage cost and verify the guarded improvement, unchanged result bits and zero post-setup solver allocations. Contracts/evidence: docs/MATHCORE_SYMPLECTIC.md and output/mathcore-symplectic-2026-10-31. General nonseparable/constrained/composed methods, sensitivities, PDE/FEM and full subject/CPU/queue/standalone gates remain open.

- [x] Qualify shared fixed-storage exact two-node interpolation/slopes with all finite coordinates, both directions, extrapolation and once-rounded values. Scalar mappings and first-order BDF dense/event consumers reuse the same native ratio arithmetic. All 2,004 Rust tests (17 new), 11,673 independent exact cases, 35,871 scalar/reduction cases, 10,990 dense queries and 1,740 event cases pass. Ninety-nine paired five-repeat CPU workloads verify zero allocations for first-order direct queries and the original alternating-direction scan fixtures; higher-order fallback and existing cache-memory admission remain tested. Contracts/evidence: docs/MATHCORE_LINEAR_INTERPOLATION.md and output/mathcore-linear-query-2026-10-31. Broader subject, solver and CPU/queue/standalone gates remain open.

- Owner reaffirmed on 2026-10-30: FULLY dependency-free. Learn the mathematical
  capabilities and methods of Maxima, SymPy, SageMath, NumPy/SciPy, Octave,
  Mathics3 and Euler Math Toolbox, then implement native Rust equivalents.
  These are research/reference systems, not dependencies or runtime workers.
  The checked reference map and algorithm-level learning/qualification tasks
  are in docs/MATHCORE_TASKS.md. Function names or external-engine calls do not
  establish native completion.


- High-speed CPU calculation and bounded queuing are required. Execution design:
  docs/MATHCORE_EXECUTION.md. Preserve accuracy; prefer direct small operations,
  batched data reuse and measured bounded parallel work. No unbounded job or
  memory queues, implicit global pools or GPU requirement.
- [ ] Implement and qualify PERF01–PERF09: performance baselines, native execution
  contexts, bounded queues/backpressure/cancellation, reproducibility and measured
  latency/throughput/memory. This is required standalone work, not an installer task.

- Comprehensive scientific mathematics, engineering mathematics and financial
  analysis are mandatory. Required operation/model register:
  docs/MATHCORE_COVERAGE.md (S/A/E/F/P requirements). Include astronomy,
  astrometry, celestial mechanics, defined astrological chart calculations,
  physics, simulation, geospatial work and broad PC analytics/computation.
  Requirements are not accepted merely because a demonstration exists.
- [ ] Complete the comprehensive coverage register and its independent acceptance
  checks within standalone mathcore-crate; preserve the no-early-integration rule.

- Authoritative subject-organized extraction checklist: docs/MATHCORE_TASKS.md.
  Complete and independently test the entire standalone mathcore-crate FIRST.
  Do not connect individual unfinished kernels to ModernBASIC during extraction.
  ModernBASIC integration is held until the standalone completion gate passes.
- [x] Map the current source files by mathematical subject and proposed destination:
  docs/MATHCORE_EXTRACTION.json; regenerate/check with tools/mathcore-inventory.py.
  All 245 legacy math-engine Rust files are covered, plus runtime math candidates,
  Ruststrument donors, native bridges and example consumers. Routing is not
  implementation; function/type-level extraction review remains open.
- [x] Extract certified adaptive NURBS polylines with separate original
  closed-span pieces, exact whole-interval bounds including rounded output
  points, complete limit reports and raw-domain rejection. Share existing
  projection and regularity kernels; use one bounded heap/cache and cumulative
  execution policy. Direct degree-zero handling retains coordinate-rounding
  limits without futile subdivision. All 928 Rust tests (31 new), 124 independent
  cases / 1,975 point components / 684 finite segment certificates and eighteen
  CPU workloads pass under recorded policies. Prior chord/span/length references
  are unchanged. The 65,536-span output takes 12.683 ms;
  peak RSS 28,668 KiB, zero swap. Evidence:
  output/mathcore-curve-sampling-2026-10-30; contract/provenance:
  MATHCORE_CURVE_SAMPLING.md / MATHCORE_CURVE_SAMPLING_SYMBOLS.json.
  Interval sampling, inverse traversal, dedicated Bezier consumers, surfaces/CAD
  and remaining mandatory subject/CPU/queue gates stay open.
- [x] Optimize shared native arithmetic and lower-degree Bernstein restriction
  without changing geometric certificates or resource policies. Native u128/dyadic
  GCD, unit products/quotients and rational identity/equal-denominator paths;
  direct exact affine power composition before Bernstein elevation. All 937
  standalone tests pass, including nine new arithmetic/algebra tests.
  In the same checkpoint, the strict circle improves 981.267 -> 274.424 ms
  and the degree-16 parabola 1104.491 -> 120.075 ms. The parabola now fits
  default policy (7,950,869 work units); 17/18 CPU workloads pass default policy.
  Independent integer/fraction/square-root and geometry regressions pass;
  all prior sampling/span/chord/length numerical outputs remain unchanged.
  Evidence: output/mathcore-sampling-performance-2026-10-30.
- [x] Bring strict-circle sampling within the unchanged default 32-million
  work allowance while retaining exact coefficients, rounded points, whole-interval
  certificates, complete reports and memory/execution policies. Shared integer
  division/GCD and signed-unit fraction paths reduce the 1e-6 case from
  36,119,098 to 31,828,303 units; paired CPU time is 290.622 -> 252.341 ms.
  All eighteen benchmark calls return their expected reports under default
  policy. All 945 Rust tests (eight new) and six independent campaigns pass;
  previous geometry and square-root result fields/digests are unchanged.
  Evidence: output/mathcore-strict-sampling-2026-10-30.
  This closes the named workload gate; broader geometry and comprehensive
  subject/CPU/queue requirements remain open.

- [x] Extract dependency-free exact first/second sample and paired moments:
  finite f64 and exact i64 observations, merge and explicitly caller-owned
  removal, mean/variance/standard deviation/RMS, covariance/correlation and
  straight-line regression. Native rational arithmetic preserves cancellation,
  subnormals and representable results despite intermediate overflow.
  All 957 Rust tests pass, including 12 new tests; 130 independent datasets
  across three stream paths pass 4,680 result checks. A one-CPU probe admits
  one million paired observations in 21.84 ms using a fixed 5,456-byte state.
  Contracts and limitations: docs/MATHCORE_MOMENTS.md.
  Evidence: output/mathcore-exact-moments-2026-10-30.
  This does not complete M07/M10 or general CPU/queue/performance gates.

- [x] Add exact paired-difference summaries, atomic batch ingestion and
  bounded rolling windows owning their observation membership. Preserve exact
  i64/f64 values through wrap/eviction; failed updates publish no partial state.
  All 971 Rust tests (14 new) pass. Independent references cover 8,370 windows,
  66,960 exact fractions and 50,220 rounded results. Five one-CPU runs measure
  medians of 15.29 ms per million batch pairs and 435.48 ms per million rolling
  updates; a 4,096-pair ring uses 128 KiB with no allocation per update.
  Evidence: output/mathcore-moment-streams-2026-10-30.
  These workload measurements do not close comprehensive CPU/queue gates.

- [x] Add exact raw/central moments through fourth order and population/
  adjusted-sample skewness and excess kurtosis. Fixed native accumulators avoid
  prematurely rounded powers; shared rational kernels preserve final rounding.
  All 987 distinct Rust tests pass (986 full suite plus one added capacity
  boundary, verified in a 16-test focused rerun). Independent checks cover
  280 datasets, three ingestion paths and 11,760 exact/rounded results.
  All 8,370 prior rolling-window reference outputs remain byte-identical.
  Five one-CPU runs measure a 19.35 ms median per million observations,
  1.24 ms per 100 ordinary summaries, using a fixed 5,456-byte stream state.
  Evidence: output/mathcore-higher-moments-2026-10-30.
  These named workloads do not close general CPU/queue/performance gates.

- [x] Add exact nonnegative-weight sample/paired statistics: explicit
  population/frequency/reliability variance, mean/std/RMS/effective size,
  covariance/correlation and weighted straight-line regression. Preserve exact
  integer observations and weights, extreme triple products and fractional
  frequency-weight rejection; support merge/removal and atomic batch ingestion.
  All 1,007 Rust tests (20 new) pass. Independent references cover 396 datasets,
  three ingestion paths, 10,611 exact and 18,864 rounded results plus nine
  expected zero-total-weight errors. Five one-CPU runs measure a 26.09 ms
  median per million weighted pairs and 0.73 ms per 100 ordinary summaries;
  fixed state is 4,376 bytes (sample) / 8,728 bytes (pair).
  Evidence: output/mathcore-weighted-moments-2026-10-30.
  Weighted higher moments/windows, broader models and CPU/queue gates remain open.

- [x] Add bounded exact-order statistics for mixed i64/f64 inputs:
  nine explicit quantile conventions with exact probability/index arithmetic,
  ordinary median, five tie-ranking rules, interquartile range and unscaled MAD.
  Cancellable heapsort preserves input and exact integer ordering; MAD merges
  ordered deviation runs without retaining a full rational deviation array.
  All 1,023 Rust tests (16 new) pass. Independent references cover 275 datasets,
  253 output fields each (including rank vectors) and 11,475 scalar rank checks.
  Five one-CPU runs for 100,000 observations have median 41.24 ms sorting,
  2.37 ms ranking and 13.78 ms exact MAD under default arithmetic policy.
  Evidence: output/mathcore-order-statistics-2026-10-30.
  Weighted/alternate robust/order variants and broad CPU/queue gates remain open.

- [x] Extract shared finite erf/erfc and add scaled/log complementary forms,
  preserving fdlibm coefficients, the Sun license and bundled upstream evidence.
  Native split-exponential and exact-dot helpers retain representable tail/log
  results without a third-party dependency. All 1,034 Rust tests (11 new) pass.
  Independent 90-digit Decimal references cover 2,225 inputs / 8,900 outputs,
  including independently located underflow and logarithmic-overflow neighbors;
  61 references are repeated at 120 digits. Observed maximum errors are
  1/2/2/2 ULP for erf/erfc/erfcx/log_erfc, within predeclared 2/4/8/8 limits.
  No universal correct-rounding guarantee is claimed. Five one-CPU trials have
  median 207.33 ms for four million scalar evaluations.
  Evidence: output/mathcore-error-functions-2026-10-30.
  This is partial M08; other special families and M09 remain open.

- [x] Extract native Gaussian density/log-density, lower/upper/log probabilities,
  narrow intervals and inverse probabilities. Exact difference/scale and affine
  arithmetic avoid avoidable intermediate overflow; inverse iterations are
  bounded with explicit execution and nonconvergence errors. All 1,045 Rust
  tests (11 new) and 2,718 independent outputs across 302 inputs pass.
  The 100-digit Decimal oracle is repeated at 140 digits for 18 cases.
  Initial density range-reduction failures and the revised near-zero-log
  accuracy contract are retained in the evidence, not discarded.
  Five one-CPU trials: median 13.67 ms for 300,000 forward
  evaluations and 7.42 ms for 10,000 inverses.
  Contracts: MATHCORE_GAUSSIAN.md. Evidence: output/mathcore-gaussian-2026-10-30.
  This is partial M09; probability/inference and broad CPU gates remain open.
  Canonical source/API: probability/gaussian.rs and probability::NormalDistribution;
  statistics retains only a compatibility import. Routing qualification:
  output/mathcore-gaussian-routing-2026-10-30.

- [x] Extract complete gamma/beta functions, signed/log gamma, shifted log gamma,
  reciprocal gamma and Stirling-scaled/log-scaled gamma. Shared bounded helpers
  retain exact upstream MIT evidence. Native PI-phase reflection and reciprocal
  factoring retain meaningful extreme-range values. All 1,056 Rust tests
  (10 new) and 3,771 independent outputs over 419 inputs pass, including
  independently located range-boundary neighbors; 23 references repeat at
  140 digits. Median of five one-CPU trials: 18.98 ms for 600,000 calls.
  Contracts: MATHCORE_GAMMA_BETA.md. Evidence: output/mathcore-gamma-beta-2026-10-30.
  This is partial M08; incomplete integrals and probability/inference remain open.

- [x] Extract regularized incomplete gamma P/Q and direct log tails, with
  explicit domains, method/iteration reports, bounded work and typed execution/
  nonconvergence failures. Tiny shapes share a cancellation-safe gamma helper;
  3,771 prior gamma/beta outputs remain byte-identical. All 1,065 Rust tests
  (9 new) and 900 independent checks over 228 inputs pass; 20 reference rows
  repeat at higher precision. Large-shape logs survive probability underflow,
  and scaled continued-fraction terms avoid intermediate overflow.
  The independently checked faster-method threshold reduces the measured
  10,000-call shape-999999 median from 104.90 to 3.73 ms without relaxing
  accuracy targets. Contracts: MATHCORE_INCOMPLETE_GAMMA.md.
  Evidence: output/mathcore-incomplete-gamma-2026-10-30.
  This is partial M08/M09; unregularized integrals and inverses remain open.

- [x] Extract standalone gamma/beta inverse ratios with lower/upper tails,
  ordinary/log probabilities, adjacent-coordinate result brackets and explicit
  endpoint/range/convergence failures. Nested forward calls consume one shared
  caller budget. All 1,082 Rust tests (8 new), 232 independent inverse checks
  and 21 higher-precision repeats pass. Integer references use separate finite
  formulas; fractional/tiny shapes use independent high-precision kernels.
  Previously qualified scalar/gamma-ratio/beta-ratio outputs remain byte-identical.
  Two 1,000-call workloads measured in three one-CPU trials.
  Contracts: MATHCORE_RATIO_INVERSES.md.
  Evidence: output/mathcore-ratio-inverses-2026-10-30.
  This is partial M08/M09; unregularized integrals, distribution objects and
  sampling, other mathematical subjects and wider standalone gates remain open.

- [x] Add unregularized lower/upper gamma/beta integrals and logarithms with
  finite-real domain/range errors and shared execution policy. The gamma
  evaluator shares all existing coefficients and iteration kernels; direct
  exponent/scaling paths preserve finite values lost by ratio-times-total
  multiplication. Coefficient-preserving exponentiation fixes an isolated
  subnormal rounding loss; complete endpoints reuse scalar range handling.
  All 1,090 Rust tests (8 new), 772 independent checks over 193 inputs and 12
  precision repeats pass. Previous scalar/ratio/inverse outputs are byte-identical.
  Four integral workloads and the prior gamma workloads measured on one CPU.
  Contracts: MATHCORE_INCOMPLETE_INTEGRALS.md.
  Evidence: output/mathcore-incomplete-integrals-2026-10-30.
  This is partial M08/M09; distribution objects/sampling, other mathematical
  subjects and wider standalone workload gates remain open.

Next shared foundation: distribution families with validated densities,
parameter scaling, survival/log-tail and inverse wrappers, followed by sampling
and inference with independent references and explicit resource diagnostics.
Preserve every open special-function, weighted-higher/window/order, geometry/
CAD, science/engineering/finance and general execution gate; sequencing does
not reduce scope.

- [x] Extract shared certified traversal-length kernels and whole NURBS
  length: exact chord/control-polygon bounds, closed raw-domain checks,
  backtracking, disconnected spans, outward global totals and complete reports
  under subdivision limits. One native arithmetic budget covers all spans;
  one exact span cache and a bounded heap govern memory. All 897 Rust tests
  (33 new), 96 independent length calls and eighteen CPU workloads pass.
  Prior exact span/Bernstein/chord reference outputs remain unchanged.
  A direct degree-zero proof reduced 65,536 constant spans from 761.453 to
  0.652 ms with zero geometry buffers; all eighteen measured cases
  now also pass the default work policy. Strict curve tolerances and high-degree
  exact calculations remain measured CPU costs. RSS 6,636 KiB, zero swap.
  Evidence: output/mathcore-curve-length-2026-10-30; contracts/provenance:
  MATHCORE_CURVE_LENGTH.md / MATHCORE_CURVE_LENGTH_SYMBOLS.json.
  Partial/inverse traversal, adaptive sampling, dedicated Bezier consumers,
  surfaces/CAD and full science/engineering/finance/CPU/queue gates remain open.

- [x] Implement native exact-rational square roots with nearest-even
  binary64 output and tight directed finite enclosures. Preserve overrange and
  underflowed squares until final rounding; distinguish negative domain,
  final overflow and absence of finite bounds. Reuse bounded native integer
  division and cumulative kernels, with fixed significand scratch for small
  limb policies. Public constructor/input/result limits stay unchanged.
  All 864 Rust tests (12 new), 10,958 independent exact cases across all 2,047
  exponent bins and fourteen CPU workloads pass. Ordinary fraction bounds
  take about 0.3 microseconds; the measured ratio around 2^16384 takes 8.1
  microseconds; RSS 2,476 KiB, zero swap. Evidence:
  output/mathcore-rational-square-roots-2026-10-30. Contracts/provenance:
  MATHCORE_RATIONAL_SQRT.md / MATHCORE_RATIONAL_SQRT_SYMBOLS.json.
  Certified length/sampling consumers, general roots/radicals and broader
  subject/science/engineering/finance/CPU/queue gates remain open.

- [x] Add typed real/complex vectors and borrowed strided/broadcast views, exact
  bilinear/Hermitian dots, cross/outer products, scaled Euclidean/Frobenius norms
  and diagonal helpers. Normalization reuses original components, exact binary
  scaling when safe and exact-ratio fallback; caller-buffer output is atomic.
  All 453 Rust tests, 852 independent rational/high-precision cases and 28 CPU
  records pass, including 5,456 exact normalized-component ratio comparisons.
  A 4,096-value real normalization measured 0.051 ms after removing unnecessary
  wide division from the ordinary-value path (previous candidate 4.09 ms).
  Evidence: output/mathcore-vectors-2026-10-29. General linear operators, sparse
  kernels, remaining donor audit and mandatory CPU/domain gates remain open.

- [x] Establish canonical native real/complex CSR/CSC storage, immutable views,
  exact duplicate triplet assembly, checked format conversions, zero-copy
  transpose views and exact normal/transpose/adjoint vector actions. Native
  caller-buffer actions allocate no heap and preserve output on failure; opposite
  formats convert under one cumulative budget. Sparse limits count stored entries.
  All 470 Rust tests, 464 independent rational-reference cases (920 canonical
  format checks, 47,380 stored components and 27,812 action components) and 36
  CPU records pass. Evidence: output/mathcore-sparse-storage-2026-10-29.
  Sparse arithmetic/ordering/direct and iterative solvers/preconditioners, generic
  callbacks and mandatory domain/CPU/queue gates remain open.

- [x] Add native real/complex linear operators and callbacks, reusable staged
  outputs, cumulative application/work/allocation budgets, original-product
  residuals with scaled norms, positive-divisor Jacobi and CG/PCG. A zero
  underflowed working residual cannot silently establish convergence. All 492
  Rust tests, 342 independent rational-reference inputs (816 solver results),
  464 unchanged sparse regression cases and 24 CPU measurements pass.
  A 4,096-variable real/complex CG solve measures 30.7/73.4 ms after exact-scaling
  fast-path optimization; prior candidate 104.4/144.6 ms. Reference output is
  byte-identical. Peak benchmark RSS is 4,432 KiB. Evidence:
  output/mathcore-operators-cg-2026-10-29. Other Krylov methods, incomplete
  factors, sparse arithmetic/ordering/direct solvers and broader domain/CPU/queue
  gates remain open. Finite-vector CG has no universal forward-accuracy guarantee.

- [x] Add native real/complex restarted GMRES and flexible-right GMRES, with
  fixed-left or variable/nonlinear inverse actions, retained right directions,
  two-pass Arnoldi, scaled Hessenberg coefficients and original residual checks
  on every candidate. Buffers are reused across iterations/restarts under one
  cumulative budget. All 506 Rust tests, 126 independent rational-reference
  inputs (1,512 solves, including 504 restarted results), 342 byte-identical
  CG/operator regression cases and 30 CPU measurements pass. Peak benchmark
  RSS is 8,524 KiB with no swap. Evidence: output/mathcore-gmres-2026-10-29.
  ILU/IC, other Krylov methods, sparse arithmetic/ordering/direct factors and
  mandatory domain/CPU/queue gates remain open; finite-vector numerical solves
  do not guarantee convergence or forward accuracy for arbitrary systems.

- [x] Add native real/complex ILU(k)/IC(k), shared bounded forward fill
  propagation, exact coefficient accumulation and reusable normal/adjoint
  inverse actions. Correct the legacy IC star/chain fill defect. Pattern caps,
  zero/nonpositive pivots, cancellation and caller-output atomicity are tested.
  All 518 Rust tests, 704 independent rational-reference inputs (701 factors,
  three expected failures and 1,402 solves) and 156 single-CPU records pass.
  At 1,024 unknowns, real GMRES improves from 139.04 to 40.63 ms using ILU(2),
  plus 9.27 ms factor setup; complex GMRES improves from 1,120.42 to 137.37 ms,
  plus 21.68 ms setup. IC reduces iterations but can cost more than plain CG on
  easy systems; it remains an explicit choice. Peak RSS 4,640 KiB, no swap.
  Evidence: output/mathcore-incomplete-factors-2026-10-29. Pivoted sparse direct
  factors, ordering/arithmetic, other Krylov methods, and mandatory domain and
  CPU/queue gates remain open; standalone completion is not claimed.

- [x] Add native real/complex CSR/CSC scaling, weighted addition, Hadamard
  and sparse matrix products, Kronecker products/sums and exact bilinear/
  sesquilinear/quadratic forms. Matrix products use a bounded sparse frontier;
  forms retain triple cancellation and allocate no heap. All 531 Rust tests,
  746 independent rational inputs (984 matrix and 458 form results, 50 expected
  overflows) and 144 single-CPU measurements pass. A billion-column logical
  sparse product succeeds within a 128-byte allocation allowance. Complex
  quadratic evaluation improved from 1.900 to 0.176 ms after removing scaled
  conversion overhead; reference outputs are byte-identical. Peak RSS 3,444 KiB,
  no swap. Evidence: output/mathcore-sparse-arithmetic-2026-10-29. Sparse
  semidefinite checks, further transformations/ordering/direct solvers, other
  Krylov methods and mandatory domain/CPU/queue gates remain open.

- [x] Add reusable validated permutations, inverse/composition, generic atomic
  vector gather/scatter and dense/CSR/CSC matrix application. Share bounded
  heapsort with triplet assembly. Add deterministic componentwise RCM on the
  structural transpose union, bounded root probes and before/after metrics.
  All 545 Rust tests, 290 independent permutation/ordering cases (534 reports),
  464 byte-identical storage regressions and 64 CPU measurements pass.
  At 4,096 grid vertices RCM takes about 1.35 ms; bandwidth falls 4,035 to 64
  and IC(1) entries 18,836 to 16,129. RCM can worsen other inputs, so application
  remains explicit and its report retains both metrics. Evidence:
  output/mathcore-permutations-ordering-2026-10-29. Fill-reducing/direct sparse
  methods, remaining Krylov, exact semidefinite and domain/CPU/queue gates
  remain open.

- [x] Add complete-fill real/complex sparse Cholesky direct factors, sharing
  bounded symbolic/numerical kernels with IC while removing the fill-level
  cutoff. Retain original CSR and report original-product residuals, including
  scaled norms when finite residual components underflow. Caller-buffer solves
  reuse workspace without allocation and preserve output on failure.
  All 556 Rust tests, 171 independent rational-reference inputs and 48 CPU
  records pass; 704 prior ILU/IC cases remain byte-identical. On a 1,024-variable
  RCM grid, reused real/complex solves take 2.49/5.73 ms; including original
  residual checks takes 2.81/6.45 ms. Peak RSS 6,988 KiB, no swap. Evidence:
  output/mathcore-sparse-cholesky-2026-10-29 (qualified 2026-10-30).
  Further direct diagnostics, exact semidefinite checks, symbolic reuse,
  further Krylov/domain and required CPU/queue gates remain open.

- [x] Add native full real/complex sparse LU with partial row pivoting,
  bounded active-column memberships and geometric fill storage. Share indexed
  normal/adjoint triangular kernels, applying row permutations without extra
  solve allocation; retain original CSR for independent residuals.
  All 567 Rust tests, 212 independent inputs (206 factors, 412 solutions,
  six expected failures) and 64 CPU records pass. Earlier 704 ILU/IC and
  171 Cholesky references remain byte-identical. A 1,024-variable real grid
  factors in 18.80 ms; reused normal/adjoint solves take 2.70/2.96 ms.
  Peak RSS 11,336 KiB, no swap. Evidence: output/mathcore-sparse-lu-2026-10-30.
  Exact semidefinite checks, richer direct diagnostics/refinement,
  remaining Krylov/domain and required CPU/queue gates remain open.

- [x] Accelerate shared exact accumulator ratios without changing rounding:
  power-of-two denominators use exact binary rescaling; operands with at most
  53 significant bits use exactly constructed normal operands and one division.
  Wider ratios retain exact limb division. All 571 Rust tests, 13,257 new exact
  Fraction cases and 42,479 prior arithmetic/quadratic/sparse reference cases
  pass; 1,087 sparse outputs remain byte-identical. In 67 before/after CPU
  records, compact/dyadic ratios improve 3.24x/6.85x; 16,384-variable diagonal
  solve improves 23.77 to 10.07 ms. General ratio timing is essentially unchanged.
  Evidence: output/mathcore-ratio-performance-2026-10-30.
  No API, dependency, precision, cancellation or resource-limit policy changed.

- [x] Extract native B-spline basis and 2D/3D NURBS curve/tensor-surface
  evaluation, including polynomial/weighted/homogeneous controls, original-
  coordinate derivatives through three and mixed (3,3), explicit knot sides,
  raw denominator checks and transactional batches. Shared exact rational
  arithmetic replaces external BigInt; degree/element/buffer/work policies are
  caller controlled. All 759 Rust tests (27 new) and 99 independent inputs/
  1,874 queries pass: 4,410 coordinate and 8,178 basis components match exact
  references bit for bit. Thirty-two CPU records show 0.018..94.345 ms/query;
  high-degree off-grid exact derivatives remain a performance concern.
  Peak RSS 2,488 KiB, no swap. Evidence: output/mathcore-nurbs-2026-10-30;
  contracts/ledger: MATHCORE_NURBS.md / MATHCORE_NURBS_SYMBOLS.json.
  Further refinement, domain certification, sampling/length, meshes/seams, trimming,
  intersections, fitting, solid/CAD qualification and broader CPU gates stay open.

- [x] Extract native exact streaming curve-knot refinement and weighted/
  homogeneous representation conversion, with typed zero-weight/underflow
  failures, final-only rounding and upward homogeneous coefficient-error
  bounds. Add shared signed Rational::to_f64_bounds; reuse native arithmetic.
  All 787 Rust tests (28 new), 107 independent refinement cases / 3,340 stored
  components / 1,908 span-polynomial identities and 231 enclosure cases pass.
  Prior 99 curve/surface references remain byte-identical. Twenty-six CPU
  workloads cover degrees 1/3/8/16 and up to 4,096 insertions; maximum admitted
  buffers 767,792 bytes, RSS 2,860 KiB, zero swap. The 4,096-knot line regression
  is exact below 1 MiB. Evidence: output/mathcore-nurbs-refinement-2026-10-30.
  Contracts/ledger: MATHCORE_NURBS.md / MATHCORE_NURBS_SYMBOLS.json. Coefficient
  bounds do not certify geometric errors or regularity. Surface refinement,
  knot removal/degree operations, certified domains, length/sampling/meshes,
  full geometry and mandatory subject/CPU/queue gates remain open.

- [x] Extract shared exact Bernstein polynomials under algebra and normalized
  NURBS spans under geometry. Reuse RationalPolynomial/RationalKernel for
  power conversion, restriction, exact points and conservative closed-subspan
  chord certificates; do not silently cancel raw denominator zeros.
  All 814 Rust tests (27 new), 76 independent Bernstein cases / 2,718 coefficients
  and 239 spans / 6,473 coefficients / 2,854 point components pass.
  All 953 chord checks agree, including 827 finite certificates, 103
  inconclusive weight cases, eighteen undefined endpoints and five bound
  overflows; 14,059 exact Euclidean sample checks support the implementation.
  Degree-128 Bernstein and degree-32 span arithmetic are qualified.
  Twenty-three CPU records pass; cached point queries are 1.255..48.337 times
  faster in sixteen measured cases, excluding extraction. Peak RSS 2,808 KiB,
  zero swap; high-degree exact restriction remains a measured performance cost.
  Evidence: output/mathcore-nurbs-spans-2026-10-30; contracts:
  MATHCORE_BERNSTEIN.md / MATHCORE_NURBS.md; provenance: MATHCORE_NURBS_SYMBOLS.json.
  The whole-interval bound follows the exact convex-hull proof, not samples.
  Curve-domain classification is qualified below; sampling/length,
  surfaces/CAD and mandatory science/engineering/finance/CPU/queue gates remain open.

- [x] Add reusable exact rational-domain algebra and whole-curve NURBS
  classification: all-channel GCD cancellation, complete raw/reduced root counts
  and multiplicities, exact endpoints, bounded binary64 locations and explicit
  undefined spans. Share native square-free/Sturm kernels and one cumulative
  work/storage/refinement policy; retain both closed-span knot incidences.
  All 852 Rust tests (38 new), 122 independent rational and 139 NURBS cases pass:
  458 domain reports, 265 root records, 347 closed-span reports and eleven
  undefined spans. Thirteen CPU records pass; a 65,536-control complete report
  takes 2.510 ms, while degree-16 full root location takes 2.228 seconds.
  Maximum logical NURBS buffers 18,754,048 bytes; RSS 20,572 KiB, zero swap.
  Evidence: output/mathcore-rational-domains-2026-10-30; contracts/ledgers:
  MATHCORE_RATIONAL_DOMAINS.md / MATHCORE_RATIONAL_DOMAINS_SYMBOLS.json and
  MATHCORE_NURBS.md / MATHCORE_NURBS_SYMBOLS.json. A raw removable zero remains
  undefined. Surface domains, geometric consumers and full completion remain open.
- [ ] Optimize high-degree exact rational-domain location refinement while
  preserving exact counts/multiplicities, endpoint incidences, limit statuses
  and cumulative resource policies. The sixteen-root benchmark requires 833
  subdivisions and about 2 seconds on one CPU; cache reuse removes preparation
  only. Do not claim real-time full-domain classification from simple curves.


- [x] Extract native C2 cubic spline fitting: natural, clamped, not-a-knot,
  periodic and mixed first/second endpoint conditions, including small-knot
  conventions. Native variable-precision arithmetic replaces external BigInt;
  pivoted tridiagonal/periodic factors are shared across components with linear
  storage and cumulative resource limits. All 732 Rust tests (24 new), 187
  independent exact-reference cases and 1,790 slope/equation comparisons pass.
  Maximum scaled slope error 1.031e-16; original 150-case interpolation output
  unchanged. Twenty-seven CPU cases cover up to 1024 knots/16 components:
  104.67 ms largest periodic fit, 2,265,472 admitted bytes, 4,124 KiB RSS,
  no swap. Evidence: output/mathcore-cubic-spline-2026-10-30; contracts/ledger:
  MATHCORE_INTERPOLATION.md / MATHCORE_INTERPOLATION_SYMBOLS.json.
  Residual is a backward diagnostic, not exact C2 or a forward-error certificate.
  B-spline/NURBS, surfaces/scattered/smoothing, richer interpolation diagnostics,
  remaining subjects and comprehensive CPU/queue gates remain open.

- [x] Extract native Hermite and shape-preserving PCHIP interpolation into
  numerical_analysis, sharing exact rational/dot/triple kernels across values,
  derivatives through order three, definite integrals and forward/backward
  segment queries. Flat component storage, transactional destinations and explicit
  work/memory/growth limits replace runtime policy. All 708 Rust tests (19 new),
  150 independent inputs, 23,978 value/derivative components, 1,160 integral
  components and 546 exact monotonicity interval checks pass. Reference output
  remains byte-identical after exact guarded optimizations. In 24 before/after CPU
  records, the largest 32-query/16-component case improves 9.89 to 0.96 ms;
  whole-panel integration improves 7.75 to 0.074 ms. Peak RSS 2,704 KiB, no swap.
  Evidence: output/mathcore-interpolation-2026-10-30; contracts/ledger:
  MATHCORE_INTERPOLATION.md / MATHCORE_INTERPOLATION_SYMBOLS.json.
  C2 fitting is qualified in the later checkpoint above; surface/scattered
  interpolation, richer diagnostics and broader subject/CPU/queue gates remain open.

- [x] Extract native finite adaptive Gauss-Kronrod integration with breakpoints,
  strict interior nodes, optional complete estimates, typed callbacks and bounded
  work/memory. Accumulate original weighted samples across panels before rounding
  so cancellation and tiny contributions survive intermediate overflow/underflow.
  All 689 Rust tests (19 new), 150 analytic and 22 exact-arithmetic reference cases,
  and 12 CPU workloads pass. Prior Jacobian/Hessian outputs remain byte-identical.
  Measured integration time ranges from 25 microseconds to 2.02 ms; maximum logical
  buffers 590,856 bytes, peak RSS 3,184 KiB, no swap. Evidence:
  output/mathcore-quadrature-2026-10-30; contracts/ledger in MATHCORE_CALCULUS.
  Error estimates remain heuristic. Infinite/specialized/multidimensional
  integration, interpolation and broader calculus/subject/CPU/queue gates stay open.

- [x] Extract direct native Hessians with exact weighted/denominator products,
  central and one-sided directions, bilateral curvature checks, bounded sample
  caching and symmetric reports. All 670 Rust tests (19 new), 360 independent
  reports / 2,808 curvature entries and 18 CPU records pass. Scalar/Jacobian
  reference digests remain unchanged. An eight-dimensional signal Hessian takes
  1.41 ms with 128 cached samples; logical buffers 15,936 bytes, peak RSS 2,652 KiB.
  Cache size is a workload tradeoff, not a universal speedup. Evidence:
  output/mathcore-hessian-2026-10-30; contracts/ledger in MATHCORE_CALCULUS.
  General higher derivatives, integration/interpolation, certified calculus
  and broader subject/CPU/queue gates remain open.

- [x] Extract native gradients and output-major Jacobians, sharing scalar kernels
  and bounded vector samples. Checked output buffers reject unwritten values;
  one context bounds memory and cumulative work across every entry. All 651 Rust
  tests (18 new), 576 independent reports / 4,968 derivative entries and 12 CPU
  records pass. Cache policies produce identical entries; prior scalar reference
  output is unchanged. The largest signal case improves 38.36 to 13.32 ms;
  logical buffers 58,144 bytes, peak RSS 2,848 KiB. Evidence:
  output/mathcore-multivariate-2026-10-30; contracts/ledger in MATHCORE_CALCULUS.
  General higher derivatives, integration/interpolation, certified calculus
  and broader subject/CPU/queue gates remain open.

- [x] Extract native fixed central differences and adaptive five-point scalar
  first derivatives into calculus, with exact final ratio arithmetic, explicit
  direction/status/error estimates, typed callbacks and bounded sample reuse.
  All 633 standalone Rust tests (19 new) and 538 independent inputs pass,
  including 336 analytic derivatives; maximum scaled analytic error 1.416e-11. Nine CPU workloads pass
  at 15–74 microseconds/derivative including accuracy checks; peak RSS 2,800 KiB.
  Evidence: output/mathcore-adaptive-differentiation-2026-10-30. Contracts and
  donor-symbol ledger: docs/MATHCORE_CALCULUS.md / MATHCORE_CALCULUS_SYMBOLS.json.
  Higher derivatives, quadrature/interpolation, certified bounds and broader
  calculus/CPU/queue requirements remain open.

- [x] Add native real/complex short-recurrence MINRES with fixed SPD inverse
  actions, sign-independent native symmetry checks, scaled correction vectors,
  fresh original residual gates and explicit singular stationarity/breakdown
  diagnostics. All 614 Rust tests, 1,008 independent equation solves and 108
  singular weighted/unweighted stationarity checks pass. Dense/CSR/CSC agree;
  existing CG and ILU/IC reference digests remain unchanged.
  Guarded compact exact sums preserve byte-identical MINRES reference output.
  Sixty one-CPU baseline/optimized comparisons pass; peak RSS 7,828 KiB.
  Evidence: output/mathcore-minres-2026-10-30. Stationarity is not equation
  convergence or a minimum-length certificate; least-squares Krylov, broader
  arithmetic performance and mandatory subject/CPU/queue gates remain open.

- [x] Add native real/complex BiCGSTAB with fixed right inverse actions,
  constant vector storage, scaled recurrence coefficients, periodic/reliable
  restarts and fresh original-system residual gates after each candidate.
  Return explicit breakdown/underflow/stagnation outcomes and preserve the
  last validated candidate. All 594 Rust tests and 168 independent inputs /
  1,008 reference solutions pass; existing CG/GMRES output digests are identical.
  Forty-eight one-CPU comparisons with GMRES and reused ILU(0) pass; peak RSS
  8,572 KiB, zero swap. Dense/CSR/CSC results agree exactly.
  Evidence: output/mathcore-bicgstab-2026-10-30. No universal convergence/speed
  claim; remaining Krylov, arithmetic-performance, domain and queue gates stay open.

- [x] Add scaled native real symmetric/complex Hermitian sparse indefinite LDL
  with Bunch–Kaufman scalar/paired pivots, stable symmetric permutation,
  active-neighborhood updates and cumulatively bounded sparse fill.
  Reuse scaled dense block arithmetic, preserve inspectable singular factors
  and computed inertia, and provide multiple-RHS/caller-buffer solves with
  retained-original residual/backward-error reports. All 581 Rust tests and
  372 independent rational reference cases pass, including 106 paired blocks,
  192 symmetric interchanges and six singular solve rejections. Earlier dense
  LDL and sparse LU reference outputs remain byte-identical.
  Forty-two one-CPU records pass; peak RSS is 11,316 KiB, zero swap.
  Scaled solve arithmetic remains an explicit performance follow-up.
  Evidence: output/mathcore-sparse-ldl-2026-10-30.
  Computed inertia is not an exact inertia/semidefinite certificate; ordering,
  symbolic reuse, refinement/condition estimates and Krylov/domain gates remain.

- [ ] Complete F00 and M01–M22 in docs/MATHCORE_TASKS.md, then pass every
  standalone completion gate before beginning the separate integration phase.

- Target: one standalone Rust math crate with no third-party crate, build-time or
  external mathematical-provider requirements. Rust standard library/normal
  platform runtime are allowed. ModernBASIC must consume the same kernels through
  an adapter; other Rust applications must not require BASIC or the distro.
- This supersedes optional foreign providers as the final engine architecture.
  Keep working legacy capabilities until independently qualified replacements
  exist. Do not mislabel vendored dependencies or FFI as dependency-free Rust.
- Preserve the full mathematics/application scope already recorded below.
  Architecture and replacement order: docs/DEPENDENCY_FREE_MATH.md.
- [x] Inventory current Cargo dependencies, foreign providers and BASIC coupling.
  Reproduce with tools/audit-math-dependencies.py. Static inventory evidence:
  output/dependency-free-math-2026-10-29/dependencies.json.
- [x] Establish src/math_core as the standalone mathcore-crate crate and extract
  bounded exact summation/means. Zero Cargo dependencies; seven Rust tests and
  7,096 independent Fraction cases / 14,192 sequential/merged comparisons pass.
  Evidence: output/dependency-free-math-2026-10-29. No full application build,
  package or installation; small generated test executables are retired.
- [x] Organize exact reductions under numerical_analysis and add native exact
  dot products. All 14 Rust tests and 31,984 independent rational-reference
  comparisons pass; no Cargo dependencies. Evidence:
  output/mathcore-dot-products-2026-10-29. Bounded queue design is recorded;
  first executor and reduction measurements are now verified in the checkpoint below.
- [x] Implement the standalone bounded CPU reduction executor and direct batch
  APIs (PERF03/PERF04 for current typed kernels). 28 Rust tests and 12,393
  independent queued comparisons pass. First optimized direct/1/2/4-worker
  reduction benchmark recorded; broader performance gates remain open.
  Evidence: output/mathcore-execution-2026-10-29.
- [x] Implement native BigUint/BigInt with bounded arithmetic, normalized division,
  GCD, powers, shifts and radix conversion (M01 integer foundation). 36 Rust tests
  and 10,255 independent reference cases pass. Rationals, decimals and floating
  conversion are recorded in the following checkpoints. Evidence: output/mathcore-integers-2026-10-29;
  contracts: docs/MATHCORE_ARITHMETIC.md.
- [x] Implement native normalized rationals, cross-cancelled arithmetic, seven
  integral rounding modes and exact binary32/binary64 conversion (M01).
  45 Rust tests, 17,054 independent rational/conversion cases and all 10,255
  integer cases pass. Decimal completion follows; real/complex work remains open. Evidence:
  output/mathcore-rationals-2026-10-29; docs/MATHCORE_ARITHMETIC.md.
- [x] Implement native exact decimals, scientific literals, quantization, all
  seven rounding policies and exact/rounded rational interchange (M01).
  53 Rust tests, 11,109 general decimal cases, 4,192 cases with a 32-bit
  coefficient limit and 17,054 rational regressions pass. Evidence:
  output/mathcore-decimals-2026-10-29; docs/MATHCORE_ARITHMETIC.md.
  Checked real/complex arithmetic and remaining subjects/performance gates are open.
- [x] Extract finite real/complex basic arithmetic and checked machine integers.
  Fix discarded-bit left shifts in the new kernel; complex products/quotients use
  exact accumulators before rounding. 61 Rust tests, 21,216 independent floating/
  ratio cases and 31,984 sum/dot regression comparisons pass. Evidence:
  output/mathcore-floating-arithmetic-2026-10-29. Remaining functions are recorded
  in docs/MATHCORE_ARITHMETIC_SYMBOLS.json; installed runtime remains unchanged.
- [x] Extract real/complex elementary and trigonometric kernels, exact phase/
  vector-angle operations and shared binary/exponential scaling. Correct extreme
  asinh/acosh overflow, seventh-root precision and tiny upward scaling found by
  independent checks. 69 Rust tests, 22,147 high-precision cases, 31,984 sum/dot
  comparisons and 21,216 prior floating cases pass. Evidence:
  output/mathcore-elementary-trigonometry-2026-10-29; docs/MATHCORE_ELEMENTARY.md.
  Complex powers, wider qualification and remaining subjects
  remain open; this is standalone source work only.
- [x] Extract exact scalar interpolation/range mapping, cubic transitions and
  wrapping, backed by exact triple-product accumulation. 76 Rust tests and
  35,871 exact-reference cases pass; earlier sum/dot/floating suites also pass.
  Evidence: output/mathcore-scalar-mappings-2026-10-29; docs/MATHCORE_SCALARS.md.
- [x] Add finite complex reciprocal and sec/csc/cot/sech/csch/coth, preserving
  signed zeros, tiny denominators and subnormal tails. 80 Rust tests and 26,628
  independent reciprocal cases pass. Optimized scalar/complex benchmarks and
  shared-ratio/wrapping improvements: output/mathcore-complex-reciprocals-2026-10-29.
- [x] Complete named real/complex inverse-reciprocal functions, preserving
  principal branches and exact distances near cuts without reciprocal overflow.
  Fix tiny atan/atanh components. 84 Rust tests and 34,579 independent reference
  cases pass; 60 initially ambiguous references verified at 2,000 digits.
  Evidence: output/mathcore-inverse-reciprocals-2026-10-29.
- [x] Implement principal complex powers with bounded native guarded range/phase
  arithmetic, reusing integer and rational kernels. 89 Rust tests, 4,591 independent
  2,000-digit/exact-rational power cases and 27,309 shared integer/rational cases
  pass. Evidence: output/mathcore-complex-powers-2026-10-29. General powers currently
  cost milliseconds per call; optimization remains an explicit performance task.
- [x] Reduce guarded complex-power series work using refined square roots,
  half-angle identities and repeated squaring. The unchanged 4,591 reference cases
  and 89 Rust tests pass. Ordinary calls improve 2.15x (11.7 to 5.45 ms); measured
  results and remaining performance limits are retained in
  output/mathcore-power-performance-2026-10-29.
- [x] Add the bounded standalone polynomial foundation with exact evaluation,
  explicit fast fused Horner, exact convolution and coefficient calculus.
  94 Rust tests and 13,752 bit-for-bit Fraction comparisons pass. CPU baselines
  distinguish exact and fast policies. Fitting, roots, division, interpolation
  and symbolic algebra remain open. Evidence:
  output/mathcore-polynomial-foundation-2026-10-29; docs/MATHCORE_ALGEBRA.md.
- [x] Add exact-classification linear/quadratic solving with stable real/complex
  roots and shared scaled ratios. All 98 Rust tests, 20,176 new independent cases
  and 61,678 shared-kernel reference regressions pass. CPU baselines cover direct
  and guarded paths. General polynomial roots, division/interpolation and other
  algebra remain open. Evidence: output/mathcore-quadratic-roots-2026-10-29.
- [x] Add exact RationalPolynomial arithmetic, calculus, Euclidean division,
  monic GCD and rational/binary-input interpolation. One shared native budget
  covers each composed operation. All 105 Rust tests, 4,266 independent exact
  algebra cases and 32,355 rational/decimal regressions pass. CPU baselines and
  contracts: output/mathcore-rational-polynomials-2026-10-29 and
  docs/MATHCORE_ALGEBRA.md. General roots, fitting and symbolic expressions
  remain open.
- [x] Add exact square-free groups and multiplicity-aware Sturm real-root
  counts with finite/infinite half-open intervals and reusable decomposition.
  Normalize GCD remainders to control scalar growth; the degree-36 budget
  regression passes at the unchanged default limit. All 113 Rust tests, 2,906
  independent factor/root cases and 21,320 polynomial/rational regressions pass.
  Evidence: output/mathcore-root-counts-2026-10-29. Root isolation/approximation,
  complex root sets, fitting and broader symbolic algebra remain open.
- [x] Add reusable bounded Sturm caches and exact rational real-root
  isolation/refinement with multiplicities, complete sorted enclosures, point
  roots and cumulative work limits. All 123 Rust tests, 271 independent input
  cases (716 enclosures / 179 refinements) and 2,906 root-count regressions pass.
  Explicit cache/interval/split caps and CPU baselines are recorded in
  output/mathcore-root-isolation-2026-10-29. Complex root sets, floating
  approximation contracts, fitting and broader symbolic algebra remain open.
- [x] Add shared bounded real/complex dense matrices, immutable strided views,
  exact products/trace and explicitly faster fused products. Caller-buffer APIs
  avoid allocation and leave output unchanged on error. All 133 Rust tests and
  3,512 independent matrix cases / 21,646 entries pass, with CPU/memory baselines.
  Evidence: output/mathcore-dense-matrices-2026-10-29. LU/QR, decompositions,
  sparse operators, diagnostics and broader M03/performance gates remain open.
- [x] Implement standalone reusable real/complex LU factors, square vector/
  multiple-RHS solves, inverse and exact original residual/backward-error reports.
  Qualify overflow/subnormal quotients, source-preserving failures, bounded
  caller buffers and cumulative composed budgets. All 145 Rust tests and 1,081
  independent cases pass, with 261 exact original-system RHS comparisons and
  CPU/memory baselines. Evidence: output/mathcore-lu-solvers-2026-10-29.
  QR, condition/rank estimation, refinement and other M03 work remain open.
- [x] Implement compact standalone real/complex column-pivoted QR and reusable
  full-column-rank least-squares reports. Qualify Q reconstruction/unitarity,
  exact original normal equations, explicit rank/cancellation/resource errors,
  subnormal fitting and guarded projection overflow. All 156 Rust tests and
  164 independent cases pass, including 175 exact RHS comparisons and CPU/memory
  baselines. Evidence: output/mathcore-qr-least-squares-2026-10-29.
  Subsequent checkpoints qualify unpivoted compatibility, minimum-norm/weighted
  fits, polynomial convenience APIs and condition estimates. Remaining M03/domain
  and CPU/queue gates stay open.
- [x] Add normalized unweighted real polynomial fitting through shared pivoted QR,
  with centered coordinates, exactly reversible power-of-two response scaling,
  exact output-scaled evaluation and rational affine coefficient conversion.
  One cumulative budget covers construction/factor/solve; rank failures and
  unrepresentable ordinary coefficients are explicit. All 166 Rust tests,
  257 independent fitting cases and 13,916 shared polynomial/QR regressions
  pass, with single-CPU timing/memory evidence. Evidence:
  output/mathcore-polynomial-fitting-2026-10-29. Weighted/complex/statistical
  diagnostics, caller buffers/queues and remaining M02/M03/domain gates stay open.
- [x] Implement native real symmetric/complex Hermitian Cholesky factors,
  reusable multiple-RHS/vector solves, inverse and original residual/error
  reports. Exact Schur pivots retain positive values below binary64 range;
  pivot square roots are rounded with exact midpoint checks. Caller-buffer
  solves allocate nothing and publish atomically. All 178 Rust tests and
  2,326 independent cases pass, including 2,507 original-system RHS comparisons.
  CPU/resource evidence: output/mathcore-cholesky-2026-10-29. Conditioning,
  updates/refinement, other decompositions and broader performance gates remain open.
- [x] Implement dependency-free real symmetric/complex Hermitian eigensystems
  with direct unitary Jacobi rotations, sorted distinct/multiple eigenvalues,
  bounded sweeps and original residual/orthogonality quality checks. Tiny
  rotation ratios and separately scaled disconnected subnormal blocks retain
  useful small answers. All 188 Rust tests, 201 independent spectrum/diagnostic
  cases and 2,326 Cholesky regressions pass. Evidence:
  output/mathcore-hermitian-eigen-2026-10-29. Numerical and resource contracts
  are in docs/MATHCORE_LINEAR_ALGEBRA.md; no exact/relative spectral certificate.
- [x] Remove redundant full-limb division by one from shared exact sum,
  dot-product and triple-product finalization. Non-unit divisors and all rounding
  rules remain unchanged. All 188 Rust tests and 57,902 independent reference
  cases pass. The same symmetric/Hermitian benchmark improves 2.10-2.56x with
  matching result checksums; 64x64 now takes 445 ms real / 993 ms complex.
  Evidence: output/mathcore-accumulator-performance-2026-10-29. Broader
  high-speed/queue/hardware qualification remains open.
- [ ] Optimize native symmetric/Hermitian eigensolver CPU cost while preserving
  the documented numerical gates. Current 64x64 baseline is 445 ms real /
  993 ms complex including diagnostics on one Ryzen 7 5700G CPU; peak RSS 2776 KiB.
  Values-only optimization, scratch reuse, batching/queue support and broader
  hardware/latency qualification remain open; this is not a high-speed gate pass.
- [x] Implement direct native real/complex thin singular value decomposition
  for tall, square, wide, empty and rank-deficient matrices. Preserve small
  singular values through exact products, extended exponents and independently
  raised subnormal columns; use orthonormal zero-column completion, explicit
  rank cutoff, convergence/resource limits and original reconstruction checks.
  All 200 Rust tests and 194 independent cases pass (451 high-precision singular
  values and 760 exact diagnostics). Evidence: output/mathcore-svd-2026-10-29.
  A 64x32 complete factorization takes 65 ms real / 179 ms complex on the recorded
  single CPU. The subsequent solve checkpoint below adds minimum-norm solves,
  pseudoinverses and caller scratch; full subspaces are added afterward.
  Optimized values-only factorizations, queues and broader performance remain open.
- [x] Implement native real/complex SVD minimum-norm multiple-RHS/vector solves,
  pseudoinverses and original residual/gradient/minimum-norm diagnostics. Reuse
  scaled norms and extended-exponent coefficients to avoid premature overflow
  and subnormal precision loss. Precreated workspaces support allocation-free,
  atomic caller output; composed calls share one budget and pseudoinverse never
  forms a dense identity. All 217 Rust tests and 154 independent cases pass,
  including 291 exact original-system RHS comparisons and 576 Penrose identity
  checks. Evidence: output/mathcore-svd-solves-2026-10-29. Weighted fits,
  conditioning/refinement and broader CPU/queue gates remain open. Full numerical
  nullspaces are added in the next checkpoint below.
- [x] Add full real/complex column/row/right-null/left-null subspaces using
  retained SVD factors and shared orthonormal completion. Include directions
  omitted by thin rectangular SVD; report original-equation residuals and
  retained-space/within-basis orthogonality. Use output plus two linear scratch
  vectors without an extra square work matrix; preserve cutoff and empty shapes.
  All 227 Rust tests, 136 independent cases (12,296 rational projector entries
  and 816 exact diagnostics), and 348 SVD/solve regressions pass. Evidence:
  output/mathcore-nullspaces-2026-10-29. Matching-coordinate search improves the
  recorded 32x64 case 42x real / 45x complex. General/generalized eigen,
  conditioning/refinement and broader mathematical/CPU/queue gates remain open.
- [x] Add native real/complex scaled determinants and log-magnitude/phase
  reporting from reusable LU factors. Preserve separate component exponents,
  finite results after temporary overflow/underflow and distant midpoint signs.
  Empty/singular/final-overflow conventions, zero-allocation reuse and cumulative
  composed budgets are explicit. All 238 Rust tests, 278 independent determinant
  inputs and 1,081 LU regressions pass. Evidence:
  output/mathcore-determinants-2026-10-29. This is a floating LU determinant;
  exact algebraic determinants, spectral conditioning/refinement, other matrix
  functions and broader CPU/queue gates remain open.
- [x] Add dependency-free real/complex scaled SVD and spectral condition
  estimates using the shared Jacobi core. Preserve connected tiny/large entries
  and independent complex components; distinguish empty spectra, computed zero,
  overflowing conditions and positive reciprocals below binary64 range.
  Rank cutoff, reconstruction/orthogonality gates, bounded exact scratch and
  cumulative budgets are explicit. All 249 Rust tests, 218 independent spectral
  cases (501 high-precision values, 648 exact diagnostics, 388 endpoint ratios)
  and 36,519 affected regression cases pass. Evidence:
  output/mathcore-spectral-conditioning-2026-10-29. This is numerical conditioning,
  not an exact rank certificate. Refinement, general/generalized eigen, sparse
  operators, remaining domains and broader CPU/queue gates remain open.
- [x] Add native real/complex iterative refinement with reusable LU factors,
  original-equation scaled residuals/errors, per-RHS convergence/stagnation/limit
  reports, numerical correction failures and monotone accepted backward error.
  Preserve useful underflowing residuals and finite updates after overflowing
  corrections; reuse shared LU substitutions and bounded scratch once per call.
  All 257 Rust tests, 308 independent refinement cases (2,935 solution and
  2,935 residual components, 548 stopping reports, 526 original RHS columns)
  and 1,783 LU/SVD/solve/subspace regressions pass. Evidence:
  output/mathcore-refinement-2026-10-29. No arbitrary ill-conditioned forward
  accuracy certificate; general/generalized eigen, sparse/matrix operators,
  remaining domains and broader CPU/queue/hardware gates remain open.
- [x] Add shared real/complex Hessenberg and complex Schur forms with native
  scaled plane rotations, implicit QR, coupling-aware deflation, bounded sweeps
  and exact original similarity/orthogonality diagnostics. Retain independently
  scaled eigenvalues, immutable factor reuse and explicit finite conversion.
  All 265 Rust tests and 224 independent cases pass, including 444 exact
  diagnostics and 582 high-precision eigenvalues. Evidence:
  output/mathcore-schur-2026-10-29. Schur vectors are not general eigenvectors;
  near-cancelling tiny roots have no relative accuracy certificate. General
  eigenvectors/sensitivity, generalized eigen, sparse/matrix operators and
  remaining mathematical/CPU/queue/hardware gates remain open.
- [x] Add native general real/complex left and right eigenvectors from reusable
  scaled Schur factors, original eigenpair/norm quality gates and simple-root
  reciprocal sensitivity estimates. Repeated represented roots have explicit
  multiplicity/restart reports; dependent vectors are not called a full basis.
  All 275 Rust tests, 193 independent cases (2,660 exact original diagnostics,
  916 high-precision vector columns and 597 independent overlap estimates) and
  224 Schur regressions pass. Evidence:
  output/mathcore-general-eigen-2026-10-29. Generalized eigen, invariant-subspace
  sensitivity, matrix/sparse operators and remaining domain/CPU/queue gates stay open.
- [x] Add native real/complex paired Hessenberg and generalized Schur/QZ factors
  with singular B support, independent alpha/beta exponents, computed
  finite/infinite/indeterminate classification and original A/B quality gates.
  Extreme scaled products retain cancellation through a common exponent shift.
  All 288 Rust tests, 190 independent pencil cases (752 exact original
  diagnostics, 508 projective and four normwise spectral comparisons), 224 Schur
  and 193 general-eigen regressions pass. Evidence:
  output/mathcore-generalized-schur-2026-10-29. Near-zero relative accuracy and
  exact pencil rank/regularity are not certified; no new external dependency.
- [x] Add native generalized real/complex left/right eigenvectors, homogeneous
  original-pair quality gates and scaled absolute/relative projective sensitivity.
  Reuse shared triangular and normalization kernels; retain singular/infinite
  behavior, explicit absence of zero/zero directions and repeated-root restarts.
  All 301 Rust tests and 174 independent cases pass, including 2,128 exact
  original diagnostics, 768 high-precision vector columns and 944 sensitivity
  values; 190 paired Schur and 193 ordinary general-eigen regressions also pass.
  Evidence: output/mathcore-generalized-eigen-2026-10-29. No complete independent
  basis, exact pencil rank/multiplicity or forward-accuracy certificate is claimed.
- [x] Add standard real/complex Sylvester equations op(A) X +/- X op(B)=C,
  with all normal/adjoint combinations, immutable Schur reuse, scaled RHS/output,
  exact original residual and componentwise diagnostics, explicit computed-zero
  pivots, cancellation and cumulative budgets. All 314 Rust tests and 159 exact
  reference cases pass (459 diagnostic checks, 153 forward comparisons and
  16 identical reused-factor results). Reused 32x32 solves measure 72.38 ms real /
  82.07 ms complex on this host; broader CPU performance gates remain open.
  Evidence: output/mathcore-sylvester-2026-10-29. No exact uniqueness or arbitrary
  forward-accuracy certificate; bounded intermediate scaling fails explicitly.
- [x] Add coupled generalized Sylvester equations and operator-adjoint solves
  for real/complex data, reusable paired Schur factors and scaled RHS/outputs.
  Exact local determinant/Cramer sums, independent original-equation quality
  gates, cancellation and cumulative resource checks are qualified. All 326 Rust
  tests and 204 exact original-operator reference cases pass: 1,164 diagnostic
  checks, 388 solution matrices compared independently, 28 identical factor
  reuses and ten explicit failures. Evidence:
  output/mathcore-generalized-sylvester-2026-10-29. Separation estimates and
  broader CPU/queue gates remain open; no exact uniqueness certificate.
- [x] Add shared matrix-free inverse one-norm and reciprocal operator-separation
  estimates for standard/generalized Sylvester equations, with original-equation
  checks on every normal/adjoint probe, scaled outputs, explicit stopping reasons
  and one cumulative budget. All 338 Rust tests and 222 independent cases pass,
  including exact inverse-norm comparisons, reported-probe checks, 36 identical
  factor reuses and explicit underestimation examples. Observed norm estimates
  range from 0.641 to approximately 1 times the true norms; these are estimates,
  not certified convergence or 2-norm separations. Evidence:
  output/mathcore-sylvester-separation-2026-10-29.
- [x] Add immutable stable selection/reordering of ordinary real/complex Schur
  factors, scaled adjacent exchanges, preserved diagonal provenance and fixed
  local/original backward-error checks. The leading selected vectors provide
  the chosen right invariant subspace without pretending to be eigenvectors.
  All 348 Rust tests and 182 independent cases pass, including 130 high-precision
  original-matrix subspace projectors and 356 exact diagnostic comparisons.
  Evidence: output/mathcore-schur-reordering-2026-10-29. Generalized reordering,
  scaled block extraction and actual separation estimates remain open.
- [x] Add immutable stable selected generalized Schur reordering for real/complex
  pairs, finite/infinite projective roots, scaled exact cross-products, local
  reconstruction checks and fixed original A/B/Q/Z gates. Preserve tiny computed
  homogeneous components instead of losing them during an exchange; reject
  exchanges involving indeterminate zero/zero pairs explicitly. All 361 Rust
  tests and 194 independent cases pass, including 280 independently computed
  left/right deflating-subspace projectors and 744 exact original diagnostics.
  Evidence: output/mathcore-generalized-schur-reordering-2026-10-29.
- [x] Add borrowed exponent-preserving ordinary/generalized Schur partitions
  and both directional inverse one-norm separation estimates. Selected APIs
  return the actual reordered factor/provenance and share one cumulative budget.
  Use the existing substitution, diagnostics and estimator kernels; retain all
  585 prior equation/estimator reference outputs unchanged. All 372 Rust tests
  and 186 new independent cases pass (296 exact inverse/probe comparisons).
  Evidence: output/mathcore-eigenspace-one-norm-2026-10-29.
- [x] Qualify Frobenius-based invariant-subspace/eigenvector separation for
  ordinary/generalized selected blocks: compute every inverse column with reused
  scratch, accumulate component squares exactly, and return scaled inverse
  Frobenius norm plus numerical spectral endpoints 1/F and sqrt(N)/F.
  Both directions/selection share one budget; empty/singular cases are explicit.
  All 385 Rust tests and 186 independent cases pass, including 296 exact
  Frobenius comparisons and 296 high-precision spectral interval checks;
  all 771 previous reference cases remain byte-identical. These are numerical
  estimates, not outward-rounded certificates or exact smallest singular values.
  Evidence: output/mathcore-frobenius-separation-2026-10-29.
- [x] Add reusable real/complex weighted minimum-norm SVD fitting with scaled
  row roots/design/RHS/output, excluded-row compaction and original-weight
  residual/gradient/norm diagnostics. Numerical-null deflation is explicit and
  disabled by a zero rank cutoff. All 401 Rust tests, 178 independent weighted
  cases (170 exact original-weight solutions / 288 RHS diagnostic columns),
  1,523 previous reference cases and 24 single-CPU measurements pass.
  The prior normalized-root precision loss and newly exposed excluded/null-row
  failures are corrected. Evidence:
  output/mathcore-weighted-least-squares-2026-10-29.
  Remaining dense/sparse methods, donor review and CPU/queue/domain gates are open.
- [x] Complete native real/complex unpivoted QR compatibility using the shared
  compact Householder kernel, original column order, reusable Q/Q^H transforms
  and composed reduced Q/R under one budget. Separate factor type avoids a
  misleading unpivoted-diagonal rank claim. Mixed tiny/large entries survive
  without global normalization. All 411 Rust tests, 132 independent QR cases,
  421 pivoted-QR/polynomial-fit references and 32 single-CPU measurements pass.
  Evidence: output/mathcore-unpivoted-qr-2026-10-29.
- [x] Replace symmetric-indefinite provider solves with reusable native
  real symmetric / complex Hermitian Bunch--Kaufman LDL factors, scaled
  components, one/two-row pivots, numerical inertia and singular diagnostics.
  Multiple RHS/vector and allocation-free caller-buffer solves preserve output
  on failure; reports check the complete original matrix. All 424 Rust tests,
  212 independent rational-reference cases (1,364 original solution entries,
  175 known-inertia cases, 374 RHS diagnostic columns) and 24 CPU measurements
  pass. Evidence: output/mathcore-indefinite-ldl-2026-10-29.
  Matrix exponential, sparse/operator work and broader CPU/domain gates remain open.
- [x] Add native real/complex matrix exponentials with adaptive Padé degree,
  power-norm scaling, scaled Gaussian denominator solves, triangular diagonal/
  first-superdiagonal corrections, direct diagonal and exact square-zero paths.
  Reuse one squaring buffer; bound work, allocation, exponents and squarings.
  All 439 Rust tests, 214 independent 180-digit/closed-form cases (2,109 entries)
  and 26 single-CPU measurements pass. The old triangular overscaling failure
  and seven cancellation-heavy candidate failures are covered and corrected.
  Evidence: output/mathcore-matrix-exponential-2026-10-29.
  Remaining donor/operator/sparse and mandatory CPU/domain gates stay open.
- [ ] AFTER standalone completion only: connect the completed crate through a
  compatibility adapter and qualify BASIC/native consumers before replacing
  legacy implementations. No early per-kernel ModernBASIC integration.
- [ ] Implement shared native numeric/error/resource/cancellation contracts and
  extract suitable kernels without interpreter state.
- [ ] Replace external exact-integer/rational, table/calendar, dense/sparse solver,
  optimization, ODE/DAE and exact-algebra/lattice dependencies with qualified Rust.
- [ ] Complete the domain inventory and qualify standalone-crate/BASIC parity,
  accuracy, recovery, resource limits and dependency closure. The existence of
  this first crate does not complete the math engine.
- Continue source edits and focused tests only. No release/installer/OS image
  work without an explicit owner request; keep test artifacts small and temporary
  under the home workspace, never the OS temporary directory.

## Current owner direction — 2026-10-29

- Latest owner direction: stop release/package/installation and cleanup churn;
  focus on source editing and tests. The owner subsequently explicitly requested
  removal of all builds. Release and installation workers were stopped, and the
  generated development/release/duplicate target trees, generated docs, old
  executable and installer/provider package outputs were removed. Preserve
  installed application, sources/history, user programs and evidence.
  Removal: output/remove-builds-2026-10-29/removal.json. Do not restart release,
  installer or OS-image work without an owner request.
  The development binary is now absent by request; do not claim it is available.

- [ ] Deploy the native Bash visibility correction: the GTK plug existed but its
  X11 child stayed unmapped, giving a blank terminal. Added host-side XMapWindow;
  actual development-app prompt pixels, resize and close-to-BASIC checks pass.
  Evidence: output/native-bash-display-2026-10-29/app-validation.json.
  Permanent checker: tools/check-native-bash-window.py. It rejects the old
  installed binary as unmapped and passes the corrected development executable;
  positive/negative receipts are retained under the same evidence directory.
  Source is backed up. The owner stopped the optimized release/install work and
  removed builds. This fix is not in the installed desktop application; deployment
  is deferred by owner direction. Do not restart packaging to finish this checkbox.

- [x] Add a direct development loop: tools/dev.py supports no-build BASIC runs,
  Rust check, focused library tests and a shared-cache development build.
  See tools/DEVELOPMENT.md and output/development-loop-2026-10-29/validation.json.
  Rust changes still require compilation; no installer or OS image is required.

- Defer all OS image builds and VM/image qualification. The owner is testing the
  native Linux installation now; do not recreate images or VMs without a new
  owner request.
- Remove obsolete generated builds while retaining the active native install,
  its matching installer, source/history, user files and build/test records.
  The owner explicitly approved deletion of the formerly protected revision-1
  ISO, persistent VM and pre-migration recovery disk. This supersedes their
  earlier retention requirement; frozen source archives remain protected.
  Cleanup evidence: output/build-cleanup-2026-10-29/removal.json (52.8 GiB reclaimed).
- [x] Qualify and install the command-hub usability update: visible Ctrl+L,
  CLS/CLEAR commands, and a native-user Bash terminal instead of the obsolete
  message directing native-app users to a Debian image. Preserve working
  directory, open tasks and user programs. Evidence:
  output/native-hub-usability-2026-10-29/installation-qualification.json.
  Verified 842 installed files, preserved 631 user programs/settings; 291 named
  provider checks plus exact-provider probes passed. Native Bash PTY and 66 shell
  tests passed; interactive review of the newly installed window remains pending.
- [ ] After the local update is qualified, retire superseded local runtime and
  installer outputs and unused shared target executables; retain reproducible
  build notes. Prior unused native release and installer retired (97,330,007
  logical bytes); shared development cache retained for the requested fast test
  loop. Review remaining obsolete target outputs separately.

## Owner requirements and current priorities — 2026-10-22

This section records the owner's current direction and governs older milestone
notes below. Checked items mean the stated scope is complete, not that the whole
OS is complete. Unchecked items remain open even where preliminary code exists.
Update this list and retain evidence as work progresses.

Owner continuation directive (2026-10-23): keep working until complete. Continue
through verified software milestones without requiring repeated “proceed” prompts.
Keep remaining implementation and OS/image/hardware acceptance gates explicit;
individual passing checkpoints do not complete the whole project.

Owner clarification (2026-10-28): native installation is complete. Do not reopen
installation or routine repackaging as project work; revisit the installed app
only for a confirmed issue. Continue the complete Linux distro using ModernBASIC,
keeping the frozen revision-1 ISO and persistent user VM intact.

Owner native-install directive (2026-10-28): create an installable native
Linux Mint application and replace the current VM desktop/menu shortcut.
Preserve the persistent VM, existing source, programs and settings. Build the
optimized x86-64 runtime in the shared home target and package it without new
VM disks or duplicated instrument banks. The full OS builder was intentionally
interrupted to release memory for this priority; this is not a test failure.

- [x] Qualify and install the compact native Mint binary installer, including
  numerical regressions, editable-program preservation and managed-window close.
  Evidence: output/native-mint-2026-10-28; instructions:
  docs/NATIVE_INSTALLATION.md. The optimized runtime is 23,190,096 bytes;
  installer 11,842,891 bytes. Eight installer tests, 439 exact-reference cases,
  20 additional regression outputs, SVG/PCM checks and actual Mint window close
  passed. Desktop/menu now launch the native app; existing banks are reused.
  External solver/SDK and OS-image gates remain separately tracked.

- [x] Extend the native Mint installer with a private, CPU-only solver pack:
  HiGHS, NLopt, SuiteSparse, serial SUNDIALS, 4ti2 and PARI/GP, plus dependency
  and license inventories. Full-payload package identity permits library/manual
  updates without changing the Rust version; format-1 upgrades preserve user
  BASIC edits. Evidence: output/native-cpu-providers-2026-10-28. Eighteen installer/
  staging tests, 291 independent solver checks, exact PARI/4ti2 entry-point checks,
  439 exact-reference regressions and 20 additional numerical outputs pass.
  SVG, paced PCM and actual native window startup/close also pass. The installer
  is 27,736,556 bytes and its payload is 67,009,134 bytes; the existing SoundFont
  directory is reused. The installed native release contains all 841 verified
  files; 631 existing programs and settings were preserved. The native export SDK,
  full math capability inventory and OS/hardware/publication gates remain open.
- [x] Triage the supplied partial Strata math review against complete source.
  The two reports share a body; the later run summary records 25 of 863 sections
  attempted, 23 output-limited and 838 unattempted. Correct the confirmed NaN
  range-check comment without changing numerical behavior. Accumulator capacity,
  harmonic reciprocal/exponent, square-root significand and F endpoint/tail
  allegations checked so far are unsupported by the actual implementation.
  Six exact F endpoint checks and 192 independent SciPy beta-tail comparisons
  pass. Evidence: output/strata-review-triage-2026-10-28. This is focused claim
  triage, not full review coverage or a new full-suite qualification.
- [ ] Finish the remaining external-review claims when the complete inputs are
  available again; both supplied Downloads paths subsequently disappeared.
  Direct instrumentation subsequently exercised beta_grat in 3,504 cases:
  no iteration exhaustion occurred; converged cases used at most eight terms.
  All 341 public calls corresponding to private-kernel/reference discrepancies
  returned finite values. Three nonzero subnormal results that SciPy rounded
  to zero agree exactly with binary64 rounding of independent PARI quadrature
  at both 80 and 120 decimal digits. Evidence:
  output/strata-review-triage-2026-10-28/grat-probe/subnormal-references.json.
  These checks resolve the three observed discrepancies, not universal
  convergence or accuracy for every input; no numerical kernel was changed.

- [x] Permit an explicit 4 GiB builder VM for package compilation while retaining
  the existing 2.5 GiB default and bounded CLI validation. The real packaging run
  exhausted 2.5 GiB guest RAM plus 1 GiB guest swap; its compiler was OOM-killed.
  Twenty builder recovery, shutdown, memory-argument and swap-fixture tests pass.
  Evidence: output/builder-package-memory-2026-10-28. The recovery run uses a
  5 GiB host scope, the unchanged frozen guest source and its verified source
  tests. No host swap, kernel or mount settings change. Successful package/ISO
  creation still needs confirmation; this change qualifies the host controller.

- [ ] Complete the refreshed OS image and installed-system qualification.
  The exact Debian provider candidate passed all 201 build/test stages: 4,038
  tests passed, none failed or ignored. Its original in-guest verifier passed,
  and all 201 exported log hashes were independently checked after a host reboot
  interrupted subsequent package compilation. Evidence:
  output/debian-exact-providers-2026-10-28/stopped-after-source-tests/source-tests-proof.json.
  Packaging resumes on the same frozen source and reusable builder, with full
  identity verification before accepting retained test results. ISO creation,
  live boot, fresh installation and signed legacy migration/update/rollback
  remain open. This candidate predates the later audio rollback fix.

- [x] Add compact, verified solver rebuild materials and enable the complete
  IDAS/KLU provider in amd64 release-guest preparation. Record archive/header
  identities at provider-build time; require source, build rules, notices and
  relinking inputs in staged KLU packages. Evidence: output/klu-relink-2026-10-28.
  Sixty-six preparation/packaging/build-workflow tests pass. All 11 circuit tests
  that previously stopped the guest now pass on the native host with KLU; the
  same 11 pass with a modified-source rebuilt KLU. Thirty-eight independent DAE
  checks, an observable changed KLU parameter and a real provider-package round
  trip pass. The rebuild kit is 1,215,300 bytes compressed.
- [x] Run the complete Debian amd64 guest source suite with KLU and the pinned
  exact providers. All 201 stages and 4,038 tests pass, including the previously
  failing bipolar-charge and 4ti2 targets. The guest reverified the complete
  report and retained worker; exported source/context/log identities match.
  Prior interrupted prefixes and missing-provider failures remain historical
  evidence in output/builder-resume-2026-10-28. Current evidence:
  output/debian-exact-providers-2026-10-28/stopped-after-source-tests/source-tests-proof.json.
  This closes the source-suite gate for that frozen candidate, not its
  package/image, cross-target, physical hardware or public-release gates.

- [x] Correct exact-provider integration for the Debian amd64 candidate:
  admit the coherent Trixie 4ti2/GMP build family and PARI 2.17.2, preserve the
  Mint family, remove PARI test early returns and the QSopt test's historical
  output-folder dependency. Provision pinned Debian packages in the builder and
  appliance; add fail-fast package/ELF/dependency/version preflight and required
  exact-answer image checks. Evidence: output/debian-exact-providers-2026-10-28.
  Fifty Rust checks pass, including real-provider exact references and standalone
  BASIC compilation. Seventy-eight infrastructure tests pass. Both real provider
  families pass five exact BASIC calls; four mixed-family probes fail as required.
  Debian package signatures/hashes are verified. This is native-host integration
  qualification; the corrected Debian guest and installed image still need full
  qualification. Cross-target exact-provider admission remains open.
- [x] Extend installed legacy acceptance to provision the three explicit exact
  solver tools alongside cached library prerequisites. Reject unrelated commands,
  foreign architectures, duplicates and runtime packages; recheck every fixture
  hash before invoking APT and preserve the signed runtime update boundary.
  Evidence: output/legacy-exact-prerequisites-2026-10-28. All 119 lifecycle,
  migration, transaction, update and signature tests pass without skips, including
  the retained legacy updater. All 17 cached packages (8,999,400 bytes) match
  their recorded pins; isolated offline APT simulation against the exact old
  ISO's 1,600-package inventory passes. No host package installation occurred.
  This qualifies the harness and dependency preparation; the actual installed
  migration/update/rollback remains open until the new image has passed.

- [x] Make batch progress durable and add exact-executor recovery for interrupted
  builds. Atomic, fsynced reports and logs preserve completed work; explicit
  prefix verification rejects changed source/toolchains/workers and damaged logs.
  Recovery preserves previous evidence and separately records its controller
  identity while retaining the original executor identity. Seventy batch,
  recovery, release-workflow and builder-memory tests pass, including the actual
  frozen executor. Evidence: output/builder-resume-2026-10-28. The real guest
  prefix verification now passes; the supervised continuation and later image
  acceptance remain separate, incomplete gates.
- [x] Preserve guest shutdown on build-controller interruption and timeout.
  QEMU runs outside the terminal's signal group; the controller handles SIGINT/
  SIGTERM, allows a 120-second guest flush/shutdown grace, drains and fsyncs logs,
  and escalates a stuck guest without claiming success. Repeated stop requests
  preserve that grace. Thirty-seven builder/release checks pass, including real
  subprocess/socket tests for success, failure, OOM, timeout, both signals,
  repeated termination and stuck-guest cleanup. Evidence:
  output/builder-shutdown-2026-10-28. The frozen candidate remains unchanged;
  later packaging uses this verified host controller. Full image acceptance
  remains open.
- [x] Include the verified KLU rebuild kit in the native CPU-provider installer.
  Evidence: output/native-klu-relink-2026-10-28. The installer is 28,951,594 bytes
  with a 68,225,436-byte payload (842 files). Its exact installed hashes, all 291
  independent solver checks, PARI/4ti2 entry points, desktop entry and reinstall
  preservation pass. All 631 existing BASIC programs and SoundFont settings were
  preserved. The optimized Rust binary is unchanged; the current verified KLU
  provider and its 1.2 MB rebuild kit are included. The previous native release
  remains available for rollback. Public redistribution gates remain separate.

- [x] Repair the three causes of the ten failed OS library tests without
  changing production numerical kernels: build the current-source worker before
  unit tests, compare NORMALPDF list values numerically with scalar parity, and
  check absent-KLU rejection as well as nested KLU ownership. The Cargo runner
  binds retained worker hashes into its reusable evidence and stops if the worker
  build fails. Evidence: output/os-test-repairs-2026-10-28. All 1,067 Rust library
  tests pass on the native host with the base IDAS profile; six KLU-profile tests
  also pass, including 512/1,000/1,024-state sparse DAE allocation checks.
  Twenty-seven batch-runner tests, formatting and portal consistency pass.
  Both actual provider mappings and hashes are recorded. The full Debian guest
  rerun has 201 stages (one worker build plus 200 test targets); later image,
  installed-system, cross-architecture and physical-hardware gates remain open.

### 1. Project identity and implementation boundary

- [x] Record the goal: a contemporary Dartmouth BASIC/TI-BASIC-style environment
  with the integrated creative/programming reach of an Amiga-like computer on
  modern hardware. The advanced math engine is a primary foundation.
- [x] Record Linux as the OS foundation and Rust as the interpreter/runtime,
  mathematical, graphics, audio and other backend implementation layer.
- [x] Record that BASIC statements/functions expose native capabilities for users
  to combine freely in their programs. BASIC demos do not substitute for Rust
  implementations of the requested engines.
- [x] Record backend release version tracking on the Rust side. Users' BASIC code
  remains freely editable, runnable and saveable without backend version bumps,
  source freezing, release registration or approval. Preserve API compatibility.
- [x] Audit implementation, documentation and release tooling for compliance with
  that boundary. `Cargo.toml` owns the Rust backend version; Debian packaging and
  source/build hashes are separate integrity identities. The embedded-only image
  provisions required product BASIC files and leaves `/home/modernbasic/programs`
  writable and outside installed runtime provenance. Regression evidence:
  `tests/source_provenance_tests.py`, `tests/release_source_tests.py`,
  `tests/appliance_session_tests.py::test_embedded_only_catalog_matches_release_identity`,
  and `docs/RELEASE_AUDIT.md`.

Owner core-footprint clarification (2026-10-25): the installed product is a
BASIC programming/running environment with a complete shared Rust math backend,
graphics, sound and I/O for user-written applications. Keep that base lean and
measure it separately from development VM disks, builds and source archives.
Do not reduce the core math or programming capabilities merely to cut bytes.
The retained candidate appliance payload is 2.356 GB, of which 2.332 GB (99.01%)
is instrument banks; the current optimized runtime is 11.168 MB. These figures
exclude Linux/dependencies and are not installed-OS measurements. Evidence and
comparisons: `docs/PRODUCT_FOOTPRINT.md` and
`output/product-footprint-2026-10-25/measurement.json`.

- [x] Measure existing runtime, installer and package payload sizes without
  building images or extracting another package tree; identify the dominant
  assets and distinguish OS requirements from development storage.
- [ ] Measure and qualify a lean installed base including all required Rust math,
  graphics, audio and I/O providers; establish separate installed, installer and
  update budgets, plus accurate Debian Installed-Size metadata.
- [ ] Separate large optional instrument banks and evaluate an optional Rust/C
  export SDK, retaining a usable default bank and ordinary BASIC edit/run access.
  Preserve user content and qualify dependency/ownership migration, forward and
  backward updates, interrupted transactions and offline rollback before shipping.
  The existing monolithic updater/package must remain functional throughout.

Owner size requirement (2026-10-25): keep the product and development workspace
reasonably sized. Measure the runtime, providers/assets and generated build data
separately; investigate avoidable growth and duplicate artifacts. Keep all project
scratch files under the home workspace, never `/tmp`. Preserve protected source,
original archives, user programs and evidence during cleanup.

Owner space-recovery directive (2026-10-25): reclaim accumulated development
storage now. Reuse one shared Cargo target and one retained builder disk; avoid
per-checkpoint build trees and duplicate media. Check disk use before large builds,
review shared debug/release outputs above 5 GiB, and retire disposable test disks
and scratch outputs once their reports are preserved. Keep the persistent user VM,
source, frozen archives and user recovery copies. Follow the space-discipline
rules in `AGENTS.md`; ongoing feature work must not recreate the retired caches.

Owner space follow-up (2026-10-25): run integration targets in small named
batches, capture their results, and remove disposable test executables and
standalone `.build/target` caches before moving on. Preserve reusable dependencies,
current runtime binaries and source. Retire superseded candidate installers once
replacement media and archived inputs are verified. Do not rebuild passed suites
just to consolidate a report; verify their exact executable/checker/report hashes.

Earlier storage checkpoint (2026-10-25): the home workspace reached approximately
65.9 GB, down from 258.9 GB (193.0 GB reclaimed at that point). The final pass shared
230 byte-identical compiler/package and boot-fixture files, removed an archived
build staging tree, and retired the completed audit executable. Its 27 unique
files are preserved in a 454 KB overlay alongside the verified existing archive
and a complete recovery manifest. Source/history, the running user VM, one
reusable builder and frozen revision-1 media remain. Compiler version checks and
both current debug/release runtime smoke checks pass after cleanup. Immutable
compiler/package files now share storage; replace updated cache files atomically
rather than editing their contents in place. Reuse these caches and retire passed
test executables; do not recreate per-checkpoint build trees or frozen staging
copies during routine development.
Evidence: `output/storage-final-pass-2026-10-25/verification.json` and
`output/storage-final-pass-2026-10-25/frozen-staging-recovery.json`.

Latest space recheck (2026-10-25): the workspace is now approximately 64.7 GB,
194.3 GB below the 258.9 GB baseline. Retire the inactive 1.286 GB
`work/workstation-build/bootstrap.raw` only after verifying that its sole payload
is byte-identical to the retained Debian seed, its lost+found is empty, and no
process or loop device uses it. The existing launcher recreates this generated
boot medium on the next image build. The reusable builder passes `qemu-img check`
and has no backing-image dependency. All 915 inventoried source/program input
files remain unchanged; preserve the unfinished home-only callback work as well.
No new build or test executable was created for this cleanup. Source, history,
archives, exported applications, running VM and recovery copy remain preserved.
Keep one shared Cargo cache and review compiled outputs above 5 GiB; do not
recreate discarded images or per-checkpoint build trees during routine math work.
Evidence: `output/storage-owner-recheck-2026-10-25/cleanup.json` and
`output/storage-owner-recheck-2026-10-25/seed-verification.json`.

### 2. Complete the advanced native math backend — highest feature priority

Owner clarification: this is the backbone of the entire programming environment,
with mathematics in one coherent Rust backend. Required domains include physics
simulations/engines, scientific research, analysis/statistics, finance, AI/ML,
graphics/geometry and music/sound/audio. Domain demonstrations alone do not meet
the engine requirement.

Owner general-purpose mathematics directive (2026-10-25): make geometry,
calculus, algebra, trigonometry and the other common mathematical areas complete
for general-purpose programming, then verify that this same Rust foundation
serves graphics, sound, CPU ML, physics, science, analysis, finance and other
applications. Existing function names and demonstrations are not acceptance.
Do not call an area complete until its operation inventory, shared native/BASIC
APIs, numerical/symbolic limits and independent workload evidence are reviewed.
Keep Linux/Rust as the implementation; users compose the functions in BASIC.

- [x] Qualify shared Rust inverse arc-length traversal for polynomial/rational
  Bezier and multispan NURBS curves, with BASIC distance/fraction queries.
  Preserve original-curve distance certificates, explicit discontinuous/stationary
  semantics, bounded work and honest resolution/range outcomes. Verified
  qualification: `output/curve-travel-2026-10-27/validation.json`. This does not
  complete the broader geometry, engineering or installed-system requirements.
- [ ] Audit and finish general-purpose geometry: 2D/3D points and vectors,
  distances/projections, line/segment/ray and shape queries, polygon measurements
  and containment, triangle coordinates, transforms/rotations and curve/path
  interpolation. Explicitly qualify boundary, collinear, degenerate and extreme-
  scale cases. The present formula/transform/collision subset is not completion.
- [x] Qualify shared Rust point/line/ray/segment/plane projections and distances,
  exact endpoint decisions, native/BASIC contracts, degenerate/extreme-scale
  recovery, independent rational references and exported clearance workloads.
  Inventory: `docs/GEOMETRY.md`; verified evidence:
  `output/geometry-projection-2026-10-26/validation.json`. Broader curves, closest pairs/intersections and installed
  CAD/physics workloads remain open under the full geometry requirement.
- [x] Correct the three reproduced legacy shape-query defects: sphere separation
  lost under large common offsets, overflowing ray/box intermediates with finite
  hit parameters, and near-parallel ray/triangle false misses. Reproductions:
  `output/geometry-projection-2026-10-26/next-work-audit.json`. Expose shared typed
  Rust queries and preserve the BASIC interfaces while qualifying boundary cases.
  Acceptance evidence: `output/spatial-queries-2026-10-26/validation.json`; include detailed hit points/barycentrics and explicit optional ray
  parameters. Coplanar overlap constructions and broad geometry remain open.
- [x] Qualify shared Rust linear/quadratic/cubic Bezier curves: original-input
  points/derivatives/split controls, bounded adaptive sampling and shared numerical
  length; native/BASIC APIs and shared renderer formulas. Verify exact references,
  rendering, resource use and exported motion workloads with explicit work and
  precision limits. Evidence: `output/bezier-curves-2026-10-26/validation.json`.
  Rational families, composite paths and broad geometry remain open.
- [x] Qualify shared Rust typed affine/projective transforms, exact-input composition
  and 4-by-4 inverse, quaternion rotation and camera matrices. Correct reproduced
  intermediate overflow, lost tiny components and large-coordinate camera failures;
  verify independent rational/high-precision references, native resource use and
  BASIC export parity. Evidence: `output/geometry-transforms-2026-10-26/validation.json`.
  Coplanar constructions, closest primitive pairs, broader
  curve/path families and sustained installed CAD/physics workflows remain open.
- [x] Correct standalone graphics-mode classification: ordinary identifiers such
  as the camera workload variable VIEW currently select the graphical wrapper.
  Distinguish actual graphics statements from assignments/expressions, preserve
  nested graphics commands, and verify default launch and binary footprint.
  Evidence: `output/geometry-transforms-2026-10-26/footprint-review.json`.
  Qualification: `output/compiler-graphics-mode-2026-10-26/validation.json`
  when accepted. General math/geometry and OS/hardware gates remain open.
- [x] Qualify shared Rust triangle coordinates and closest points in 2D/3D,
  native/BASIC packets, degenerate convex hulls, exact feature decisions,
  independent rational references, resource use and an exported curve/clearance/
  material workload. Evidence: `output/triangle-coordinates-2026-10-26/validation.json`.
  Coplanar constructions, closest pairs, broader curves and installed CAD/physics
  workloads remain open under the general geometry requirement.
- [x] Qualify shared Rust closest pairs for every 2D/3D line/ray/segment
  combination, with explicit collapsed/parallel/nonunique/intersection reports,
  shared exact geometry helpers, BASIC access, independent references, resource
  measurements and exported animated-rod clearances. Evidence:
  `output/linear-closest-pairs-2026-10-26/validation.json`.
  Other primitive pairs, coplanar constructions, broad geometry and installed
  CAD/physics/OS/hardware workflows remain open.
- [x] Qualify shared Rust intersection sets for every 2D/3D line/ray/segment
  pair and 3D linear-primitive/plane query, including collapsed, parallel and
  coplanar cases. Reuse closest-pair primitive/ratio machinery; verify BASIC
  packets, independent constructions, CPU resources and exported plane sections.
  Evidence: `output/linear-intersections-2026-10-26/validation.json`.
  Coplanar triangle/mesh, plane/plane and other shapes, symbolic exact geometry,
  broad math/application and installed OS/hardware gates remain open.
- [x] Qualify shared Rust full line/ray/segment versus closed-triangle
  intersection sets in 2D/3D, including coplanar chords, collapsed hulls,
  endpoint barycentrics and optional overflowing parameters. Verify independent
  constructions, native/BASIC contracts, CPU resources and exported composed
  surface workloads. Evidence:
  `output/triangle-intersections-2026-10-26/validation.json`.
  Triangle/triangle, plane/plane, mesh constructions, broader geometry and
  general math/application/OS/hardware gates remain open.
- [x] Qualify shared Rust triangle-pair intersection constructions in 2D/3D:
  full point/segment/coplanar polygon results, collapsed hulls, exact topology,
  both barycentric sets and canonical ordering. Reuse exact triangle clipping;
  verify independent rational feasibility references, BASIC/native contracts,
  CPU resources and exported surface-overlap workloads. Evidence:
  `output/triangle-pair-intersections-2026-10-26/validation.json`.
  Plane/plane, mesh-wide constructions, other shapes/curves and general
  math/application/OS/hardware gates remain open.
- [x] Qualify shared Rust plane systems and triangle/plane slices:
  exact rank/consistency for 1..64 planes, canonical bounded representatives,
  full coplanar/degenerate triangle hulls, barycentric weights and shared
  construction ordering. Verify independent exact references, BASIC/native
  contracts, CPU resources and an exported moving cube-section workload.
  Evidence: `output/plane-intersections-2026-10-26/validation.json`.
  Mesh-wide stitching/acceleration, other shapes/curves and general
  math/application/OS/hardware gates remain open.
- [x] Qualify shared Rust indexed mesh/plane slices: reuse the mesh BVH with
  exact support bounds, preserve original face/vertex/edge identities and all
  coplanar cut cells, retain barycentric materials, and bound work/output with
  cancellation and no partial success. Verify independent per-face references,
  native repeated-query resources and exported BASIC section workloads.
  Evidence: `output/mesh-plane-slices-2026-10-26/validation.json`.
  Contour assembly, coplanar union, solid fill/toolpaths and broad
  math/application/OS/hardware gates remain open.
- [x] Qualify shared Rust mesh contour topology and bounded BASIC plane batches:
  assemble indexed open/closed paths without arbitrary branch pairing, group
  coplanar patches, remove ordinary internal seams and expose ambiguity. Reuse
  one mesh index across layer queries with shared work/output budgets and whole-
  batch recovery. Verify independent graph/geometry references, native resources
  and exported layered-section composition.
  Include the indexed keyword-prefix assignment correction uncovered by the
  layer workload; qualify stored/direct execution and compact syntax compatibility.
  Correct the reproduced LLVM out-of-memory failure in bounded standalone
  compilation; measure cold/warm export resource use under the existing 4 GiB
  address-space limit and verify cache/cancellation behavior.
  Evidence: `output/mesh-contours-batch-2026-10-26/validation.json`.
  Geometric union/offset/fill, printer toolpaths and broad
  math/application/OS/hardware gates remain open.
- [x] Qualify shared Rust nonuniform cubic Hermite paths: preserve original
  knot/value/derivative inputs, expose derivatives and exact piecewise integrals,
  bound native/BASIC batches and reuse the kernel in ODE event interpolation.
  Verify exact and independent numerical references, boundary/extreme-scale
  behavior, cancellation/recovery, resources and interpreted/exported composition.
  Evidence: `output/hermite-paths-2026-10-26/validation.json`.
  Rational curves, composite
  continuity/arc-length workflows and broad math/OS/hardware gates remain open.
- [x] Qualify shared Rust automatic cubic spline fitting: natural/clamped,
  mixed endpoint derivatives, not-a-knot and periodic conditions, small-sample
  conventions, reusable bounded factors, range-aware precision and residual
  diagnostics. Correct the measured uneven-grid forward-error failure that a
  small residual alone did not reveal. Reuse shared
  Hermite queries/integrals; verify independent exact/SciPy references, extreme
  scaling, cancellation/recovery, CPU resources and native/BASIC/export parity.
  Evidence: `output/cubic-splines-2026-10-26/validation.json`.
  Rational/composite/arc-length work and broad math/OS/hardware
  acceptance remain open.
- [x] Qualify shared Rust shape-preserving cubic interpolation: exact local
  PCHIP slopes, flat/extrema/endpoint conventions, rounding-safe monotonicity,
  componentwise vector behavior, bounded resources and cancellation. Reuse
  editable Hermite packets, queries and integrals; verify whole-interval
  derivative-sign certificates, independent SciPy references and native/BASIC
  export composition. Evidence:
  `output/monotone-splines-2026-10-26/validation.json`.
  Rational/composite/arc-length and broad math/OS/hardware gates remain open.
- [x] Qualify shared Rust rational Bezier foundations: weighted/homogeneous
  2D/3D curves, exact quotient derivatives, signed/zero-weight and infinite-
  control semantics, finite subdivision representations and bounded batches.
  Verify conic/symbolic/exact references, native resources and cancellation,
  editable BASIC packets and standalone CAD composition. Refresh earlier
  curve summaries to recognize accepted spline/monotone implementations.
  Evidence: `output/rational-bezier-2026-10-26/validation.json`.
  Rational surfaces,
  composite/arc-length and broad math/OS/hardware gates remain open.
- [x] Qualify exact rational-curve interval domains: shared symbolic/curve
  Sturm kernel with independent algebra certificate verification, exact
  denominator/root multiplicities, all-coordinate cancellation and pole orders,
  endpoint/open intervals and adjacent-binary64 clusters. Bound work and
  refinement, retain exact coverage on limits, verify independent references
  and native/BASIC/export/cancellation behavior. Evidence:
  `output/rational-domain-2026-10-27/validation.json`.
  Rational surfaces, composite/arc-length and
  broad math/OS/hardware acceptance remain open.
- [x] Qualify adaptive rational-curve sampling: reject exact interval
  denominator singularities; bound original-curve deviation from rounded
  output segments using exact restricted homogeneous controls, including
  signed/zero weights, backtracking, endpoint rounding and deep parameter
  compression. Return full ordered polylines and explicit limit statuses;
  bound memory/work and qualify independent geometric certificates,
  cancellation, native/BASIC/export composition. Evidence:
  `output/rational-sampling-2026-10-27/validation.json`.
  Rational surfaces, composites/arc-length and broad
  math/OS/hardware acceptance remain open.
- [x] Qualify certified rational-curve length: exact original homogeneous
  restrictions, directed chord/control-polygon enclosures, repeated traversal,
  signed/zero weights, full-domain rejection, extreme scales and the retained
  subnormal-width excursion of exact length 2. Expose native Rust and BASIC
  APIs with honest tolerance/resource/representation status; verify independent
  references, cancellation/resources and exported CAD/feed-time composition.
  Evidence: `output/rational-length-2026-10-27/validation.json`.
  Rational surfaces, composite continuity, arc-length parameterization and broad
  math/application/OS/hardware gates remain open.
- [x] Qualify shared Rust B-spline/NURBS curve foundations: local exact
  degree-0..16 basis and quotient derivatives, nonuniform/repeated knots,
  explicit one-sided joins and endpoint semantics, weighted/homogeneous and
  polynomial curves, finite/resource admission and cancellation. Expose editable
  BASIC packets and bounded batches; verify independent exact/SciPy references,
  native resource behavior and exported CAD paths. Evidence when accepted:
  `output/nurbs-curves-2026-10-27/validation.json`.
  Rational surfaces, continuity workflows, arc-length traversal and broad math/OS gates
  remain open.
- [x] Qualify exact NURBS whole-span domain reports: shared exact polynomial/
  Sturm classification in original signed knot coordinates, both closed-span
  incidences at joins, raw/removable/pole multiplicities and identically-zero
  denominator spans. Bound global refinement, work and report storage; retain
  full counts on refinement/resolution limits and never return partial clean
  output on errors. Verify exact references, high-degree/extreme/compressed
  cases, native resource/cancellation and BASIC/export guards. Evidence:
  `output/nurbs-domain-2026-10-27/validation.json`.
  Rational surfaces, continuity/arc-length traversal
  and broader general math/application/OS/hardware work remain open.
- [x] Qualify exact working NURBS knot refinement: sorted streaming insertion,
  repeated/interior/nonclamped endpoint knots, weighted/homogeneous conversion,
  explicit EXACT/ROUNDED output and upward homogeneous coefficient error.
  Verify independent span identities and refinement references, bounds, large
  batches, cancellation and native/BASIC/export use. Evidence when accepted:
  `output/nurbs-refinement-2026-10-27/validation.json`.
  Rational surfaces, continuity/arc-length traversal and
  broad general math/application/OS/hardware acceptance remain open.
- [x] Qualify exact-data adaptive NURBS sampling: independent closed-span
  pieces, original homogeneous restrictions, shared geometric certificates,
  global segments/depth/work limits, complete bounded output on parameter
  limits, and domain/overflow/cancellation errors without partial reports.
  Verify exact interval references, discontinuities and narrow excursions,
  degree-16/native resources and BASIC/export consumers. Evidence when accepted:
  `output/nurbs-sampling-2026-10-27/validation.json`.
  Rational surfaces, continuity/arc-length workflows and broad
  general math/application/OS/hardware acceptance remain open.
- [x] Qualify certified NURBS traversal length: complete original closed-span
  coverage, exact restrictions and shared directed chord/control-polygon bounds,
  no artificial jump lengths, repeated travel and narrow excursions, global
  budgets and honest representation limits. Verify independent high-precision
  references, degree-16/native resources and BASIC/export/cancellation recovery.
  Evidence when accepted: `output/nurbs-length-2026-10-27/validation.json`.
  Inverse arc-length traversal, rational surfaces, composite continuity and broad
  general math/application/OS/hardware acceptance remain open.
- [x] Qualify native tensor-product B-spline/NURBS surface evaluation:
  independent U/V knots/degrees, exact local weighted/homogeneous mixed jets,
  explicit one-sided joins, bounded complete batches and editable BASIC grids.
  Verify analytic plane/cylinder/sphere models, independent exact/library
  references, extreme scales, resources, cancellation and native/BASIC/export.
  Evidence when accepted: `output/nurbs-surface-2026-10-27/validation.json`.
  Whole-patch denominator reports are qualified in the following milestone.
  Tessellation, trimming/intersections, area integration, inverse arc-length
  and broad math/OS/hardware remain open.
- [x] Qualify whole-patch NURBS surface denominator reports: complete
  closed-cell coverage, exact original-data sign/range certificates, raw-zero
  and opposite-sign witnesses, separate join continuations and honest
  unresolved cells under global subdivision/representation limits. Verify
  independent polynomial certificates, hidden/tangent/boundary zeros,
  extreme scales, resources, cancellation and native/BASIC/export consumers.
  Evidence when accepted: `output/nurbs-surface-domain-2026-10-27/validation.json`.
  Complete bivariate zero-set classification, certified tessellation,
  trimming/intersections, area, inverse arc-length and broad OS gates remain open.
- [x] Qualify certified NURBS surface triangle meshes: complete original-patch
  coverage, conforming global parameter cuts, separate one-sided seams,
  rounded vertices and exact same-parameter positional error bounds for both
  triangles. Verify saddle twist, hidden bulges, rational models, degenerate
  geometry, extreme scales, topology/limits, independent references and native/
  BASIC/export/cancellation resources. Evidence when accepted:
  `output/nurbs-surface-mesh-2026-10-27/validation.json`.
  Closed-solid construction/seam welding, trimming/intersections, area,
  inverse arc-length and broad math/application/OS/hardware gates remain open.
- [x] Qualify native/BASIC NURBS indexed-mesh integration: proven identical
  original rational boundaries, explicit periodic closure, coordinate-preserving
  maps and retained positional certificates. Reject nonconverged/degenerate
  output; verify independent seam algebra, mesh topology, embedded-solid/STL
  composition, cancellation/resources and interpreted/exported workloads.
  Evidence when accepted: `output/nurbs-indexed-mesh-2026-10-27/validation.json`.
  General CAD tools, trimming/booleans, surface intersections, area/volume error
  bounds and installed math/application/OS/hardware gates remain open.
- [ ] Audit and finish general-purpose algebra: scalar/complex/exact arithmetic,
  polynomial operations, equation/root solving, dense/sparse linear algebra,
  decomposition/conditioning and the supported symbolic simplification,
  factorization and solve classes. Document unsupported classes; test real use.
- [ ] Audit and finish general-purpose calculus: numerical and supported symbolic
  derivatives/integrals, gradients/Jacobians/Hessians, limits/series, interpolation
  and differential-equation composition; qualify singular/discontinuous inputs,
  error estimates and recovery. Broader PDE/FEM remains an explicit workstream.
- [x] Extend shared Rust numerical quadrature and expose typed native reports:
  qualify global estimated-error control, explicit breakpoints, bounded heap/work,
  exact accumulated panel totals, triple-product weighting, native/BASIC reports,
  cancellation and recovery. Verify the three recorded endpoint/jump failures,
  independent physical/scaled references and interpreter/export parity. Evidence:
  `output/quadrature-2026-10-26/validation.json` once accepted. Finite sampling,
  difficult endpoint resolution, extrapolation/infinite/principal-value and
  certified quadrature remain open alongside the broad calculus requirements.
- [x] Qualify shared adaptive scalar differentiation: correct the measured NDERIV
  step failures near LOG's boundary and at a large sine phase; preserve explicit
  steps, add native/BASIC reports and one-sided derivatives, and qualify bounds,
  actual sample spacing, callback recovery and independently known derivatives.
  Evidence: `output/differentiation-2026-10-26/validation.json` once accepted.
  Heuristic finite differences do not establish arbitrary differentiability,
  certified error bounds or completion of the broad calculus inventory.
- [x] Qualify shared CPU numerical gradients and Jacobians for ordinary callbacks:
  reuse adaptive scalar derivatives with per-axis directions/steps, bounded vector
  sample caching, per-entry error/status reports, explicit missing partial entries,
  global work caps, failure/cancellation recovery and shared native/BASIC APIs.
  Check independent matrices, mechanical/graphics/audio/financial sensitivities,
  native workloads and interpreter/export parity.
  Verified evidence: `output/multivariate-differentiation-2026-10-27`.
  Ordinary-callback Hessian qualification is tracked below; broader calculus remains open.
- [x] Qualify shared CPU numerical Hessians for ordinary scalar callbacks:
  direct second/mixed derivative stencils, representable coordinates, exact
  accumulated sample differences, central consistency and one-sided domains,
  per-entry errors/status, symmetric cache reuse, work limits and recovery.
  Check independent curvature, native workloads and interpreter/export parity.
  Verified evidence: `output/hessian-2026-10-27`.
  General higher/certified derivatives and the broad calculus inventory remain open.
- [x] Integrate shared adaptive numerical Jacobians into backward-Euler/DAE
  consumers after reproducing the former fixed-step square-root domain failure.
  Regressions cover small/large states, frozen DAE coordinates and narrow domains.
  Preserve analytic/sparse callback behavior, cumulative work limits, solver
  accuracy and failure recovery; qualify the numerical square-root boundary case.
  Verified evidence: `output/physics-jacobian-2026-10-27`.
- [x] Qualify shared bounded scalar searches: native/BASIC roots and extrema,
  safeguarded interpolation and contraction, exact ratio/interval arithmetic,
  residual/bracket/work reports, objective-resolution diagnostics, cancellation,
  independently known difficult/scaled roots and extrema, and export parity.
  Evidence: `output/scalar-search-2026-10-26/validation.json` once accepted.
  Continuity/unimodality and callback accuracy remain caller assumptions; this
  does not complete general nonlinear equations or global optimization.
- [x] Qualify the native Linux Mint installed trigonometry runtime against
  400/800-digit independent references: 16,689 cases across reciprocal, real/
  complex elementary, angular-coordinate and phase-waveform consumers passed.
  Installed executable identity and reports:
  output/installed-trigonometry-2026-10-29/validation.json. This does not close
  other architectures, certified transcendental bounds or distro-image gates.
- [ ] Audit and finish general-purpose trigonometry: direct/inverse and hyperbolic
  functions, angle units/conversion, atan2/polar/vector angle operations, domain
  boundaries, large arguments and consistent real/complex behavior.
- [x] Qualify and preserve shared explicit trigonometry: PI/degree phase reduction,
  normalized sinc, stable reciprocal tails and real ACOSH/ATANH boundaries, and exact-product vector
  angles with native/BASIC APIs and both sinc-resampler consumers. Require
  independent high-precision/domain checks, signal regressions, chunked recovery
  and native/interpreter/exported workloads. Inventory: `docs/TRIGONOMETRY.md`;
  evidence: `output/trigonometry-2026-10-26/validation.json` once accepted.
- [x] Qualify complex elementary functions across large/small components,
  subnormal tangent tails, representable exponential/hyperbolic outputs and
  canonical branch-cut boundaries. Preserve native/BASIC behavior and qualify
  independent references, failure recovery and standalone consumers.
  Verified evidence: `output/complex-elementary-2026-10-27`.
- [x] Qualify shared angular coordinates and degree graphics consumers: native/BASIC
  conversion, polar and rotation contracts; exact accumulated rotation products;
  large-phase renderer agreement, cursor axes, recovery and CPU/export workloads.
  Verified evidence: `output/angular-coordinates-2026-10-27`.
- [x] Finish reciprocal-trigonometry API review: expose checked direct native
  SEC/CSC/COT/COTH counterparts, qualify poles, underflow/overflow and independent
  references, and reconcile the full trigonometry acceptance inventory.
  Verified evidence: `output/reciprocal-trig-2026-10-27`.
  Keep certified bounds, other-target and installed-system acceptance open.
- [x] Qualify a shared Rust error-function port from Sun fdlibm: retain upstream
  source/license and exact coefficient provenance; replace the low-accuracy
  portable fallback, expose native/BASIC APIs, and qualify small/subnormal tails,
  piecewise boundaries, statistical/financial consumers, resource use and export.
  Verified evidence: `output/error-functions-2026-10-27`.
  Platform exponential accuracy, other CPU/installed-system gates and the broader
  probability/statistics inventory remain separate requirements.
- [x] Qualify shared normal-distribution/quantile APIs and scaled affine results:
  fix the reproduced finite affine-quantile overflow and disappearing tiny
  interval probabilities; expose checked native and BASIC upper-tail APIs.
  Independent references, failure recovery, related consumers, CPU resources
  and interpreter/export parity pass. Baseline: 248 failures in 642 cases;
  final normal-distribution qualification: 834 passing cases.
  Verified evidence: `output/normal-distribution-2026-10-27`.
  General inference, other CPU/installed-system gates and certified bounds
  remain separate requirements.
- [x] Qualify shared Rust binomial, geometric and Poisson count distributions:
  replace the Poisson normal approximation, preserve small geometric/tail
  probabilities, avoid binomial scalar table allocation, and expose checked
  native/BASIC APIs with bounded, interruptible CPU work and direct upper tails.
  Distribution count arguments explicitly reject fractions. Independent
  references, one-million-entry tables, cancellation/recovery, scalar allocation
  checks and interpreter/export parity pass. Reviewed baseline: 158 failures
  in 968 preserved cases; final count-distribution grid: 1,584 passing cases.
  Verified evidence: `output/discrete-distributions-2026-10-27`.
  Larger binomial models, sampling/quantile/inference extensions and installed
  system/other-CPU qualification remain separate requirements.
- [x] Qualify shared CPU exponential, uniform and Weibull distributions:
  direct lower/upper tails, inverse functions, stable near-scale Weibull
  evaluation, overflow-aware scaled inverses and exact uniform interpolation.
  Native/BASIC APIs, cancellation/recovery, 3,272 independent numerical cases,
  resource checks and standalone parity pass. Baseline: 12 failures in 1,456
  existing-function cases; corrected all six initial new-inverse overflow cases.
  Verified evidence: `output/continuous-distributions-2026-10-27`.
  Broader laws, inference/fitting, random generation and installed/other-target
  application acceptance remain separate requirements. The first export
  launcher ended with status 143 while Cargo/rustc survived; recovery reused
  the completed runtime and passed export checks. The initial full-build
  resource measurement is incomplete; interruption evidence is retained.
- [x] Correct and unify exponential scaling across financial and probability
  consumers: preserve tiny growth terms, subnormal halfway displacements,
  finite scaled results and genuine overflow. Expose Rust `scaled_exponential`
  and BASIC `SCALEEXP`, and preserve exact one-period cash-flow cancellation.
  Baseline: 68 financial failures in 1,380 cases; final scaling/financial grid:
  3,104 passing cases. Related curve/bootstrap/probability suites, native
  resource checks and interpreter/export parity pass. Fused multiply-add
  removes ordinary-path accumulator overhead. The necessary fresh standalone
  runtime build now has a complete resource measurement and reuses its cache.
  Verified evidence: `output/exponential-scaling-2026-10-27`.
  Ill-conditioned multi-period models, broader financial/application workloads
  and installed/other-target acceptance remain separate requirements.
- [x] Add shared CPU beta-distribution density, direct tails and inverse
  probabilities to Rust and BASIC. Adapt selected MIT-licensed CDFLIB Rust
  routines with exact source provenance, bounded work and cancellation.
  Correct tiny-tail inverse comparisons, subnormal rounding and concentrated
  density cancellation. All 4,197 independent numerical checks, native CPU/
  resource checks and interpreter/export parity pass; no Cargo dependency added.
  Verified evidence: `output/beta-distribution-2026-10-27`.
  Sampling/fitting, broader inference, certified bounds and installed/other-target
  acceptance remain separate requirements.
- [x] Complete shared Rust gamma/chi-square lower and upper inverse probabilities.
  Preserve logarithmic tails during bounded searches and expose native/BASIC
  functions, including cancellable list inputs and atomic assignment errors.
  Pass 2,898 independent inverse cases, 1,201 gamma forward checks and 1,584
  related discrete-distribution cases, plus 52 Rust regressions, native CPU/
  resource checks and interpreter/export parity. No dependency added.
  Verified evidence: `output/gamma-quantiles-2026-10-27`.
  Binary64 coordinate limits, sampling/fitting, certified bounds and installed/
  other-target application acceptance remain separate requirements.
- [x] Complete the shared Student t density, direct survival and upper inverses.
  Accept finite positive degrees without the artificial one-million-degree cap,
  preserve tiny tails and central mass, reuse qualified beta deep-tail handling,
  and bound inverse searches to 63 coordinate steps. All five scalar APIs
  support cancellable BASIC lists with atomic failure recovery.
  Baseline: 266/836 existing-function cases rejected by the former degree cap.
  Final: 2,270 expanded plus 909 retained independent references, 51 Rust
  regressions, native CPU/resource checks and interpreter/export parity pass.
  Verified evidence: `output/student-t-complete-2026-10-27`.
  Binary64 precision, sampling/fitting, broader inference and installed/
  other-target application acceptance remain separate requirements.
- [x] Qualify exact-moment mean inference: repaired seven observed failures,
  including an overflow panic, offset/subnormal loss, integer admission and tiny
  confidence intervals. Shared exact moments support one-sample, Welch, paired
  and pooled Rust/BASIC APIs. Passed 55 Rust regressions, 364 independent cases,
  four million native observations, 65,536-observation BASIC samples,
  cancellation/recovery and interpreter/export parity.
  Evidence: `output/mean-inference-2026-10-28`.
- [x] Repair quadratic indexed mutation of numeric BASIC lists: update only
  the selected item/subtree after path, type and storage checks; preserve
  local/shared/static/BYREF scope, independent copies and atomic errors.
  Passed 499 Rust tests, including an allocation bound for a 32,768-item list;
  65,536-item initialization plus mean inference finished in 0.805 seconds.
  Interpreter/export outputs match and repeated export reuses the shared cache.
  Evidence: `output/list-item-updates-2026-10-28`. The subsequent checkpoint
  below removes the separate scoped-expression copying cost.
- [x] Remove whole-value cloning from procedure expression contexts and math
  callback bindings: borrow visible scalar payloads and overlay only callback
  arguments/SADD addresses. Preserve local/shared/static/BYREF precedence,
  declared defaults, free variables, nested DEF FN/EVAL and string memory.
  Passed 515 Rust tests, bounded allocations with large unrelated values,
  a 65,536-item BYREF procedure workload in 1.711 seconds, and
  interpreter/export parity. Public Rust evaluator APIs remain compatible.
  Evidence: `output/expression-bindings-2026-10-28`. DIM-array aliases retain
  scalar snapshots; requested list values, BYVAL and assignment retain copies.
- [x] Qualify the central Fisher F family through shared Rust beta kernels and
  BASIC scalar/list APIs: density, direct tails and lower/upper critical values.
  Passed 57 Rust tests, 1,742 independent numerical cases, native CPU/cancellation
  checks, a 65,536-value BASIC workload and interpreter/export parity. Corrected
  seven measured tail/normalization failures during qualification.
  Evidence: `output/fisher-f-2026-10-28`. Noncentral laws, model-specific
  ANOVA/inference and installed/other-target acceptance remain separate gates.
- [x] Add shared exact-moment variance-ratio inference and ordinary one-way
  ANOVA over the qualified F law. Native Rust and BASIC `FTEST2`/`ANOVA1`
  preserve INTEGER precision and finite ratios across overflowing/underflowing
  component variances. Qualified 64 Rust tests, 221 independent cases (1,042
  numeric fields), million-observation native workloads, cancellation and
  interpreter/export parity. Confidence below 1e-6, degenerate designs and
  unrepresentable positive fields fail explicitly. Assumptions are documented.
  Evidence: `output/variance-anova-2026-10-28`. Other ANOVA designs, post-hoc
  procedures and installed/other-target qualification remain separate gates.
- [x] Qualify shared Welch unequal-variance one-way inference with exact sample
  moments and bounded dyadic weights (relative rounding <=2^-127), retaining
  INTEGER precision and extreme scales. Passed 65 Rust tests, 197 independent
  cases (760 numeric fields), million-observation/native cancellation checks,
  a 65,536-observation BASIC workload and interpreter/export parity.
  Evidence: `output/welch-anova-2026-10-28`. Repeated/factorial designs, post-hoc
  analysis and installed/other-target qualification remain separate gates.
- [x] Qualify shared Rust/BASIC multiple-testing adjustments: Bonferroni, Holm,
  Hochberg, Hommel, BH/BY and none; explicit family sizes and dependence
  assumptions; bounded/cancellable sorting and Hommel closure. Passed 58 Rust
  tests, 396 independent cases (2,143 adjusted values), million-probability
  native workloads, the 3,162-input Hommel boundary and a composed Welch/Holm
  interpreter/export workflow. Evidence: `output/multiple-testing-2026-10-28`.
  Specialized contrast/range procedures, model design and installed/other-target
  acceptance remain separate gates.
- [x] Correct the five reproduced math-review defects: small predictor logistic
  scaling, parameter-dependent cross-entropy derivatives 3–9, stable tensor
  scalar/axis reductions, range-safe MSE and signed harmonic-mean cancellation.
  Include BASIC/native and autodiff regressions, independent numerical reference
  checks and bounded resource checks. Evidence:
  `output/math-review-fixes-2026-10-28/validation.json`.
- [ ] Audit common probability/statistics, distributions, fitting/optimization,
  number theory, financial and signal/transform operations as coherent APIs;
  fill measured gaps instead of treating a large function count as completeness.
- [x] Qualify shared Rust physical units and dimension-checked quantities:
  exact transforms, compound expressions, temperature point/interval rules and
  atomic bounded lists. Passed 895 independent cases, 43 Rust tests, three
  million native conversions, mid-batch cancellation/recovery, 65,536-row BASIC
  conversion and mixed engineering/audio interpreter/export parity.
  Evidence: `output/physical-units-2026-10-28`. Installed/other-target/hardware
  gates and further solver/application unit integration remain open.
- [x] Qualify shared multivariate uncertainty propagation: exact covariance
  admission, correlated Jacobian propagation, range-safe standard uncertainties
  and quantity-aware native/BASIC APIs. Passed 103 independent model/error cases
  (1,016 comparisons), 61 Rust tests, 256-input/output resource workloads,
  cancellation/recovery, differentiation composition and interpreter/export parity.
  Evidence: `output/uncertainty-propagation-2026-10-28`. Nonlinear/distribution
  propagation, inference and installed/other-target/hardware gates remain open.
- [ ] Extend unit-aware integration across solver/application boundaries, with
  explicit semantic quantity kinds, uncertainty and user-defined unit needs.
- [ ] Qualify shared-kernel graphics/animation, sound/DSP, CPU ML, physical
  simulation and analysis workloads in BASIC and direct Rust. Check coordinate/
  unit/data interchange, precision, cancellation, sustained resource use and
  interpreter/export parity. Sample-time audio work must reuse bounded buffers.
- [x] Extend the installed-image math acceptance path with a composed current-
  foundation BASIC fixture: unit-aware structure/circuits, image-to-solid STL,
  exact-integer inference/typed CSV, spectral energy, calculus and finance.
  All five generated payload programs passed on the current host runtime with
  an empty tool PATH and 1 GiB address-space limit; deliberately wrong results
  in five new stages were rejected. Strict output/file checks and frozen-source
  re-verification are covered by generator regressions.
  Evidence: `output/math-image-foundation-2026-10-28/validation.json`.
  This qualifies the acceptance path on the host, not a rebuilt/installed image.
- [ ] Qualify this general-purpose baseline in the refreshed installed OS,
  preserving the lean core, clear optional-provider availability and freely
  editable BASIC programs. Host tests alone do not establish installed coverage.

Owner application-scope clarification (2026-10-25): the common math foundation
must support 3D design/CAD, 3D-printer designs, game engines, physics engines,
scientific research, emulation, simulation and all other identified general-
purpose use-case families. Keep graphics, sound and CPU ML in this same scope.
The acceptance inventory is `docs/GENERAL_MATH_ACCEPTANCE.md`; add newly identified
scenarios and missing backend operations instead of silently narrowing the goal.

- [ ] Complete the CAD/printing math and data path: robust geometric predicates,
  curves/surfaces, solid/mesh construction and queries, topology/units, mesh
  validation and portable model interchange; verify printable closed models and
  independent import/measurement. Rendering triangles alone does not pass it.
- [ ] Complete shared game/physics foundations: spatial acceleration, collision
  detection, rigid-body/contact/constraint evolution, transforms/interpolation,
  deterministic step/state behavior and measured graphics/audio integration.
- [ ] Qualify research/simulation foundations across analytic, independently
  computed and conservation/convergence workloads, including boundary/initial
  conditions, sparse assembly, uncertainty/error reporting and reproducibility.
- [ ] Qualify emulation foundations: exact bit/word and numeric behavior,
  deterministic clocks/events, reproducible state/save/load, memory/device I/O
  boundaries and synchronized graphics/audio; define fidelity per emulated model.

Owner engineering and analysis clarification (2026-10-27): CAD means computer-
aided engineering design across electrical, mechanical, electronic/IC, structural
and other engineering disciplines. Include design/model editing, simulation,
component/material libraries, units/constraints, drawings and interchange.
Surface/triangle mathematics supports mechanical CAD but does not complete CAD.
Game-engine mathematics, scientific mathematics, financial mathematics and
analytics are explicit requirements over the same shared CPU Rust foundation.
Software engineering and possible AI assistance were also discussed; preserve
general BASIC application development, with AI assistance optional rather than
a prerequisite for engineering or analysis.

- [x] Record these explicit disciplines, their acceptance workloads and the
  engine-reuse strategy in `docs/GENERAL_MATH_ACCEPTANCE.md`. This records scope;
  it does not accept implementation of the disciplines.
- [ ] Complete electrical design workflows: schematic/network models, component
  libraries, connectivity/netlists, units and applicable power/control analyses,
  with independent circuit/network references and design-file round trips.
- [ ] Complete electronic and IC design workflows: device-model and schematic
  composition, PCB/layout connectivity and design rules, IC layout with declared
  process/device models, DRC/LVS and format interchange. Qualify PCB and IC gates
  separately; a circuit solver alone does not implement either design workflow.
- [ ] Complete mechanical CAD workflows: constrained parametric sketches, parts,
  assemblies/mates, dimensions/tolerances, material properties, drawings and
  engineering interchange, using the shared geometry and simulation kernels.
- [x] Qualify the shared Rust planar static frame/beam backend and BASIC APIs:
  consistent nodal/member loads, supports and settlements, member forces,
  rigid-mode restraint checks, stable nodal strain energy and bounded scaling.
  37 independent model/error cases (12,158 scalar comparisons), 34 Rust tests,
  native CPU/resources, maximum-node/member workloads and interpreter/export
  parity pass on the qualification host. Evidence: `output/frame2d-2026-10-28`.
  This accepts the documented linear 2D model only; broader structural,
  installed-image and hardware gates below remain open.
- [x] Qualify continuous planar member displacement/rotation/force/strain fields
  and total strain energy including uniform-load interior deformation. Share
  Rust frame models and bounded FRAME2DFIELD/FRAME2DENERGY BASIC functions.
  59 field/error cases (6,642 comparisons), 37 retained frame cases (12,158
  comparisons), 46 Rust tests, maximum-batch CPU/resources/cancellation and
  interpreted/exported BASIC parity pass. Evidence: `output/frame2d-fields-2026-10-28`.
  Section/fiber stress, properties varying within members, releases/shear,
  3D/nonlinear/dynamic/buckling/design and installed/hardware gates remain open.
- [ ] Complete structural analysis/design workflows: materials/sections,
  loads/supports and combinations, beam/frame/plate/solid models, finite-element
  assembly, static/modal/buckling response and convergence/independent checks.
  Declare supported models and design standards; do not imply certification.
- [ ] Complete general-purpose financial math: explicit cash-flow/date/calendar,
  compounding/discount/curve, valuation, sensitivity, risk and scenario contracts;
  qualify currency/decimal rounding and independently known model results.
- [x] Qualify sequential Rust fixed-cashflow discount-curve bootstrapping:
  explicit payment/price equations, stable log-factor search, separate price-fit
  criteria, native/BASIC APIs and diagnostic accepted-prefix results. Verify
  coupon/off-knot payments, signed/multiple-root equations, extreme scales,
  resource limits, cancellation, independent prices and standalone export.
  Verified evidence: `output/curve-bootstrap-2026-10-27`. Instrument-specific
  quote helpers, floating-payment/multi-curve calibration, sensitivities/risk,
  inconsistent-quote fitting and installed gates remain open.
- [x] Qualify exact Rust decimal arithmetic: exact string/integer input,
  declared quantum and eight rounding policies, fused operations and final-
  rounded sums through native/BASIC APIs. Passed 49 Rust tests, 766 independent
  decimal cases, 1,092 high-precision compatibility checks, million-entry native
  and 65,536-entry BASIC sums, cancellation/resources and interpreter/export parity.
  Evidence: `output/decimal-arithmetic-2026-10-28`. Currency identity, allocation
  conventions, financial model integration and installed gates remain open.
- [x] Qualify bracketed Rust dated internal rates of return: exact same-time
  cash-flow grouping, scale-normalized exact product accumulation, shared
  safeguarded root search, native/BASIC APIs and explicit convergence/work reports.
  Verify irregular and multiple-root flows, extreme scales, cancellation,
  independent high-precision results, resource bounds and standalone export.
  Verified evidence: `output/dated-irr-2026-10-27`. Automatic root discovery,
  additional return models, calibration, decimal currency/risk and installed
  gates remain open.
- [x] Qualify Rust discount-factor curves and dated term-structure valuation:
  positive ordered factors, exact civil time ratios, log-linear interpolation,
  explicit flat-forward extrapolation, stable spot/forward rates and exact
  finite-product cash-flow sums. Expose native/BASIC APIs and batch queries;
  verify extreme scales, small rates, cancellation, resource bounds, independent
  high-precision results and standalone export. Verified evidence:
  `output/discount-curves-2026-10-27`. Instrument-specific calibration, multi-curve
  models, sensitivity/risk, additional return models, decimal currency and installed gates remain
  open.
- [x] Qualify anchored Rust date schedules and accrual/payment periods: original-
  anchor day/month intervals, short/long front/back stubs, explicit regular
  boundaries, month-end preservation, independently adjusted accrual/payment
  dates and signed payment lags. Expose composable native/BASIC APIs and exact
  day-count ratios; qualify no-drift dates, boundary failures, cancellation,
  resource caps and standalone export. Verified evidence:
  `output/date-schedules-2026-10-27`. Other roll/stub conventions, coupon-reference
  day counts, country calendars, curve calibration/additional return models, currency/risk and installed
  system gates remain open.
- [x] Qualify shared Rust civil/business calendars and dated cash-flow valuation:
  reuse the already-vendored Chrono date arithmetic; expose explicit date epochs,
  Gregorian/month-end rules, caller-supplied holidays/weekends, signed business
  counts/adjustment/advance, exact named day-count ratios and annual-effective
  dated valuation through native/BASIC APIs. Verify independent date/financial
  references, boundary recovery, resource limits and interpreted/export parity.
  Verified evidence: `output/calendar-math-2026-10-27`. Additional schedule/roll conventions,
  country calendars, further conventions, curve calibration/additional return models, decimal currency/risk,
  typed DATE tables, installed OS and other CPU gates remain open.
- [x] Measure a minimal existing Rust analytics engine before product adoption:
  Polars core 0.36.2 builds on the existing Rust 1.75 toolchain with pinned
  dependencies. Verify million-row grouping/filtering, chunked-input agreement,
  source checksums and resources. Reproduce numerical/semantic differences and
  verify a prototype using ModernBASIC's existing exact accumulator and
  canonical zero keys. Evidence: `output/tabular-analytics-2026-10-27`.
  This accepts the candidate investigation and adapter proof only.
- [x] Qualify the initial shared typed-table product surface: implemented Rust
  NativeTable and BASIC TABLECREATE/SELECT/FILTER/GROUP/ROWS/SCHEMA/SIZE, with
  nullable types, checked/exact reductions, offline source/export plumbing and
  bounded core resources. Evidence: `output/tabular-runtime-2026-10-27`.
  This initial surface leaves joins/windows/interchange/streaming, standalone
  distribution notices/source packaging and installed/architecture gates open.
- [x] Qualify the analytics standard-library dependency replacement: remove
  unmaintained smartstring and unused static_assertions, and replace fast-float
  (RustSec 2024-0379 / 2025-0003) with Rust's standard UTF-8/float parsing.
  Verify string boundaries, numeric rounding and malformed input; repeat exact
  arithmetic, memory/cancellation, offline and standalone checks. Preserve
  upstream/preimage history and enforce the complete package/patch inventory.
  The current query covers all 68 public dependency entries in Cargo.lock with
  no matching version advisories; this is not a source-safety proof or native
  library audit. Evidence: `output/analytics-std-strings-2026-10-27`.
  Standalone distribution, installed OS, other CPU targets and broader analytics
  acceptance remain separate open gates.
- [x] Qualify stable typed Rust/BASIC equijoins: INNER/LEFT/RIGHT/FULL/SEMI/ANTI,
  exact composite keys, explicit null matching, deterministic ordering and
  right-field naming. Reject excessive duplicate rows/cells and repeated text
  before result allocation; verify empty inputs, cancellation/recovery,
  independent pair references and standalone/native workload parity.
  Evidence: `output/table-joins-2026-10-27`. Other join predicates, grouped
  windows, typed interchange, streaming and installed/CPU-target gates remain open.
- [x] Qualify shared exact variance/deviation and ordered row-frame windows:
  retain all integer bits, skip null observations with explicit minimum/sample
  counts, preserve input order within exact-key partitions and reject oversized
  output before allocation. Compare grouped/global/sliding results against exact
  independent references; measure cancellation/resources and interpreter/native/
  standalone parity. Verified evidence: `output/table-windows-2026-10-27`.
  Timestamp/range windows, ranking/lag/lead, streaming and installed gates remain open.
- [x] Qualify ordered analytics: stable typed multi-field sort; row, competition,
  dense and fractional peer ranks; typed partition lag/lead; and exact numeric
  distance windows including tied coordinates. Bound comparisons/text work and
  result amplification; verify independent references, old-window compatibility,
  CPU memory/cancellation, BASIC/native/export and offline source reuse.
  Verified evidence: `output/table-ordering-2026-10-27`. Date/time/calendar schemas,
  typed interchange, bounded streaming and installed gates remain open.
- [ ] Integrate the qualified table design behind shared typed Rust/BASIC APIs:
  nullable typed data, schema validation, filtering/projection, checked grouped
  statistics, joins and windows; preserve exact integer keys and explicit null
  semantics. Vendor pinned source/notices, qualify bounded allocation and
  cancellation, native/BASIC/export parity and installed availability.
  Design and measured gaps: `docs/TABULAR_ANALYTICS.md`.
- [x] Qualify typed CSV interchange and bounded batch reads: explicit schemas,
  exact INTEGER values, null versus empty TEXT, multiline UTF-8 quoting,
  cancellable native streams, BASIC text conversion, shared file-channel cursors
  and atomic file export. Retain independent interchange, native/BASIC/export,
  malformed-data recovery and resource evidence. Verified evidence:
  `output/table-csv-2026-10-27`. Other formats, date/calendar schemas, global
  streaming operations and installed/other-target gates remain open.
- [x] Qualify exact grouped/whole-dataset streaming accumulators in Rust and
  BASIC: append batches, merge unrounded states, query/inspect/reset/free state;
  enforce row/cardinality/memory/work limits with transactional errors and
  cancellation. Validate independent references and CSV/native/BASIC/standalone
  workloads. Verified evidence: `output/table-stream-2026-10-27`. Streaming joins/
  windows, covariance/regression, persisted recovery and installed gates remain open.
- [x] Qualify shared exact paired moments: covariance, Pearson correlation and
  simple regression across scalar/BASIC lists, matrices, typed grouped tables
  and mergeable streams. Preserve integer bits, pairwise null semantics, exact
  residuals, finite-result errors, bounded state and independent CPU evidence.
  Verified evidence: `output/table-pair-2026-10-27`. Sliding paired windows,
  weighted/multiple streaming regression and installed gates remain open.
- [x] Qualify reversible exact paired row and numeric-range windows in Rust and
  BASIC, including mixed single/paired reductions, complete-pair minimum counts,
  nulls, exact integer/real keys and axis endpoints, cancellation and bounded
  resource use. Verify independent frame scans and native/interpreted/export
  parity. Verified evidence: `output/table-pair-window-2026-10-27`.
  Cross-batch windows/joins, calendar frames, persistence and installed gates
  remain separate open work.
- [x] Qualify shared Rust cross-batch trailing/prefix row windows and BASIC
  ROLLCREATE/ROLLADD/ROLLINFO/ROLLRESET/ROLLFREE commands. Reuse exact single/pair
  reducers, retain bounded observations per group, stage updates through output
  assignment, and check partition-independent results, cancellation, resource
  plateaus and native/interpreter/export parity. Verified evidence:
  `output/table-rolling-2026-10-27`. Cross-batch lookahead/range/calendar frames,
  streaming joins, persisted recovery and installed/architecture gates remain open.

- [ ] Complete analytics workflows: typed data interchange and missing-data
  rules, filtering/join/group/window operations, descriptive/inferential
  statistics, regression/time series, uncertainty and reproducible reports.
  Qualify bounded batch/streaming CPU execution and native/BASIC/export parity.
- [ ] Evaluate existing math/physics/analytics engines against measured gaps:
  reuse suitable Rust code, selectively port licensed algorithms or integrate
  native providers behind Rust. Record upstream identity/license, provenance,
  dependencies/toolchain, numerical contracts and CPU/resource measurements;
  compare original and independent references before exposing stable BASIC APIs.
  Keep Rust ports distinct from foreign-library integrations, avoid redundant
  engines, and qualify installed availability separately. Owner clarification:
  existing engines may be dissected into useful algorithms/modules, ported or
  reused in Rust and exposed through BASIC; automatic translation alone is
  not qualification and whole-engine imports are not required.

Owner media/engineering scope clarification (2026-10-25): extend the common Rust
math backbone and composable BASIC APIs to image/photo/graphics processing and
image tracing into triangulated surfaces; painting, shading, gradients, depth,
scaling, mirroring and related transforms; advanced electronic-circuit simulation;
engineering and technology mathematics; audio/sound/music generation and production;
and video/movie production and generation. The intended result remains a complete
general-purpose Linux-based OS with BASIC as its main user programming language
and Rust supplying the native runtime/backends. Keep these requirements alongside
CAD, physics, research, emulation and simulation; do not replace those earlier
requirements or declare an application family complete from isolated demos.
The family-specific operation/workload criteria and shared implementation sequence
are maintained in `docs/GENERAL_MATH_ACCEPTANCE.md`.

- [x] Complete shared explicit CPU color/alpha mathematics in Rust and BASIC:
  extended sRGB and linear D65 XYZ/luminance, straight/premultiplied conversion,
  twelve coverage operators, transparent interpolation and bounded typed batches.
  Qualify independent transfer/matrix/coverage references, HDR/tiny-alpha/error
  cases, cancellation, native resources and interpreter/exported BASIC pixels.
  Keep shared image buffers, tracing, profiles, shading and production gates open.
  Evidence: `output/color-foundation-2026-10-26/validation.json`.
- [x] Complete bounded shared CPU image buffers and resampling in Rust/BASIC:
  linear-light RGBA, strided views, point sampling, nearest/linear/area and
  per-axis automatic resize, mirroring, coverage composition, byte/PPM bridges
  and native output strips. Qualify rational references, alpha/HDR/border cases,
  cancellation, lifecycle/memory budgets, current-file reloads and interpreted/
  exported BASIC pixels. Keep tracing, profile, renderer and production gates
  open. Evidence: `output/image-foundation-2026-10-26/validation.json`.
- [x] Complete the shared image-to-geometry bridge: channel-threshold pixel-cell
  contour rings with holes/area/perimeter and explicit dimensioned height meshes
  with pixel-center UVs. Qualify digital connectivity, exhaustive small masks,
  rational height references, native resource/cancellation behavior, BASIC
  admission and interpreted/exported preview/STL parity. Preserve touching
  boundary semantics and open-surface limits; keep smooth vectorization, general
  polygon holes, reconstruction and production gates open. Evidence:
  `output/image-geometry-2026-10-26/validation.json`.
- [x] Qualify and preserve shared Rust image filtering and projective warps:
  alpha-correct normalized convolution, signed channel kernels and physical Sobel
  gradients, exact homogeneous maps, explicit borders and bounded subpixel grids.
  Check independent SciPy/rational pixels, extreme scales/alpha/cancellation,
  strided/strip agreement, native resources, BASIC admission/atomicity and exported
  image parity. Keep adaptive reconstruction, vectorization, surface/shading and
  production/OS gates open. Evidence:
  `output/image-processing-2026-10-26/validation.json`.
- [x] Qualify and preserve the shared Rust surface/renderer bridge: exact
  perspective attributes, normalized image UVs, robust triangle normals, cosine
  gain and linear material shading. Bind named images as budgeted snapshots in
  the existing CPU renderer. Check independent pixels, extreme scales, depth,
  cancellation/lifetimes, legacy compatibility, native resources and exported
  scene parity. Keep broader graphics/production/OS gates open. Evidence:
  `output/surface-math-2026-10-26/validation.json`.
- [x] Qualify and preserve shared crease-aware smooth surface normals: exact
  area-weighted indexed corner fans, explicit hard boundaries, robust oriented
  cofactor transforms and clipped perspective-correct per-pixel CPU lighting.
  Check independent geometry/pixels, extreme scale/cancellation, native resources,
  atomic recovery, legacy rendering and interpreted/exported flat/smooth scenes.
  Preserve silhouette/weighting/coverage limits and broader graphics/OS gates.
  Evidence: `output/smooth-surface-2026-10-26/validation.json`.
- [ ] Complete shared image/photo/graphics mathematics: color-space and transfer
  functions, compositing/alpha, sampling/reconstruction, filtering, gradients,
  projective/affine transforms, scaling/mirroring, contour/edge/region tracing,
  vectorization and triangulation, UV/surface mapping, depth/visibility, shading
  and painting/image synthesis. Separate 2D tracing, depth-map surfaces and
  reconstruction from calibrated/multiple views or user-provided geometry;
  qualify geometry/precision/uncertainty rather than infer exact 3D shape from
  an arbitrary single photograph. Check reference images, seams, boundaries,
  color/alpha correctness and reusable CPU kernels without per-pixel BASIC calls.
- [x] Qualify and preserve the linear Rust circuit foundation: native R/C/L and
  independent/controlled sources, sparse modified nodal assembly, checked DC/AC,
  shared descriptor residual/Jacobian APIs and bounded BASIC CIRCUIT statements.
  Check rational DC/power, independent RLC admittance, native IDA composition,
  sparse resources, cancellation/lifecycle and interpreted/exported parity.
  Evidence: `output/circuit-foundation-2026-10-26/validation.json`.
- [x] Qualify and preserve native static semiconductor and nonlinear circuit
  support: Shockley junctions, reciprocal NPN/PNP Ebers–Moll models, analytic
  Jacobians, bounded operating points/source continuation, bias-dependent AC,
  tied terminals, low-leakage scaling, BASIC/native APIs and IDA composition.
  Check independent high-precision laws/derivatives/networks, convergence/recovery,
  resources/cancellation and interpreted/exported parity. Evidence:
  `output/nonlinear-circuits-2026-10-26/validation.json`.
- [x] Qualify and preserve capacitor-aware circuit transients in Rust/BASIC:
  exact-incidence storage coordinates, analytic consistent initialization,
  constant/sine/ramp/jump sources, zero-storage algebraic time sweeps,
  explicit source intervals/right-hand outputs,
  dense index-one IDA composition and aggregate work/admission limits. Check
  independent physical-coordinate RC/RLC, orthogonal floating capacitors, Radau
  nonlinear references, jump convolution, recovery and native/export parity.
  Sparse transients and independent-source reductions are qualified below;
  general controlled-source reductions, state switching, restart/advanced-device
  and production gates remain open. Evidence:
  `output/circuit-transients-2026-10-26/validation.json`.
- [x] Qualify and preserve structural sparse circuit transients in Rust/BASIC:
  fixed topology-derived slots may contain exactly zero coefficients; shared
  analytic KLU initialization/integration handles 257–1024 circuit unknowns,
  including source/final jumps. Avoid dense coordinate-transform scratch.
  Check independent sparse RC exponentials, orthogonal floating-capacitor
  reductions, nonlinear Radau/RLC references, 1024-state native resources,
  canonical CSR compatibility, provider failure, cancellation and export parity.
  Independent-source reductions are qualified below. Keep matrix-free
  preconditioning, general controlled-source reduction, state events/restart,
  advanced devices and OS/hardware gates open. Evidence:
  `output/sparse-circuit-transients-2026-10-26/validation.json`.
- [x] Qualify and preserve independent-source circuit constraint reduction:
  voltage-first capacitor forests and signed inductor-cutset/cycle coordinates,
  source derivatives in shared Rust Newton/IDA paths, native/BASIC coordinate
  maps, stored-initial-state compatibility and explicit impulse diagnostics.
  Check independent physical laws/RLC exponentials, nonlinear loads, circulating
  currents, source corners/common-mode jumps, existing dense/sparse regressions,
  1023-variable native resources and interpreter/export parity. Keep general
  controlled-source reductions, impulse redistribution, state switching/restart,
  advanced devices and OS/hardware gates open. Evidence:
  `output/circuit-constraints-2026-10-26/validation.json`.
- [x] Qualify and preserve physical circuit checkpoint/restart in Rust/BASIC:
  immutable versioned records of exact model parameters, physical and reduced
  storage state (preserving small capacitor differences under large common modes),
  absolute source schedules/time and numerical options; bounded corruption/model
  validation, ADVANCE/RESUME atomic results and shared dense/KLU reinitialization.
  Check fresh-process analytical/nonlinear references, source corners/circulating
  currents, malformed records, cancellation/admission, resources and export parity.
  Keep solver-history serialization, state switching/topology changes, impulses,
  advanced devices and production/OS/hardware gates explicit. Evidence:
  `output/circuit-restart-2026-10-26/validation.json`.
- [x] Qualify shared oscillator phase and circuit sinusoid precision: exact
  represented time/frequency product reduction, large radian offsets, signed and
  subnormal phases, source derivatives and native/BASIC physical load references.
  Preserve real overflow errors and distinguish input/sampling limits. Evidence:
  `output/phase-waveforms-2026-10-26/validation.json`.
- [x] Qualify native voltage-controlled hysteretic circuit switching: shared
  dense/structural sparse IDA roots, analytic consistency with stored C/L state
  preserved, right-hand scheduled jumps, simultaneous/cascade ordering, pure
  algebraic sweeps, bounded chatter/work/memory and native/BASIC integration.
  Check independent RC/diode/RLC references, cancellation, resource usage,
  shared nested KLU quotas and interpreter/export parity. Evidence:
  `output/circuit-switching-2026-10-26/validation.json`.
- [x] Add versioned switched-circuit checkpoint/resume: preserve exact base model,
  switch configuration/modes, reduced/physical state, sources and options;
  qualify fresh-process continuation/corruption and BASIC/native/export reuse.
  MBCSW1 records retain original configuration and current modes separately;
  MBCIR1 remains the ordinary-circuit format. Keep native history serialization,
  arbitrary topology/impulse/device expansion and OS/hardware gates explicit.
  Evidence: `output/switched-circuit-restart-2026-10-26/validation.json`.
- [x] Qualify native junction charge storage and dynamic diode circuits: bounded
  depletion/diffusion laws, consistent charge/capacitance derivatives, physical
  AC/residual/full-rate Jacobians, dense/KLU transients, prescribed-source loads,
  switching and checkpoint reuse, BASIC consumers and independent references.
  Keep bipolar/MOS dynamics, high injection, breakdown, non-quasistatic transport,
  general topology reduction and OS/hardware acceptance open. Verified evidence:
  `output/circuit-charge-2026-10-26/validation.json`.
- [x] Qualify native quasistatic bipolar charge: NPN/PNP BE/BC depletion and
  diffusion storage, terminal charge derivatives/conservation, shared topology,
  source loads, full-rate Jacobians, AC/transients, sparse switching/restart and
  BASIC/native/export consumers. Check independent amplifier laws and charge
  balance, precision, recovery and resources. Keep MOS, advanced bipolar effects,
  general index reduction and OS/hardware acceptance open. Verified evidence:
  `output/bipolar-charge-2026-10-26/validation.json`.
- [x] Qualify native long-channel MOS terminal charge: smooth implicit channel
  conduction, Ward–Dutton partition, signed four-terminal derivatives and bulk
  junctions, NMOS/PMOS symmetry, shared DC/AC/dense/KLU transients, storage-aware
  source constraints and portable checkpoints. Resolve charge-coupled IDA Newton
  stagnation without relaxing integration tolerances; check general IDA regression.
  Check independent CMOS inverter
  laws, charge balance, precision, cancellation, export and resource use. Keep
  process-calibrated/short-channel models, noise/thermal/NQS, general index
  reduction and OS/hardware acceptance open. Verified evidence:
  `output/mos-charge-2026-10-26/validation.json`.
- [x] Qualify exact controlled-source circuit constraints: reuse Rust rational
  algebra to reduce linear source/KCL constraints on capacitor and inductor
  storage, differentiate known forcing, retain physical current/charge laws,
  and expose the same native/BASIC maps, sparse dynamics and restart behavior.
  Check amplifier/Miller RC/RLC, current control, algebraic dividers, nonlinear
  loads, source corners/jumps, singularity, cancellation, switching and resource
  bounds against independent references; qualify native and exported consumers.
  Nonlinear/higher-index constraints, constraint-changing switch projections and
  general impulse redistribution remain open. Verified evidence:
  `output/controlled-constraints-2026-10-26/validation.json`.
- [ ] Complete advanced circuit-simulation mathematics over the shared Rust
  sparse/complex/ODE/DAE/nonlinear foundation: circuit topology and equation
  assembly, analog/nonlinear device models, DC operating points, AC/small-signal
  and transient analyses, initial conditions, events, convergence/recovery and
  numerical diagnostics. Qualify analytic circuits and independently computed
  device/network workloads, units, energy/passivity and difficult stiff cases.
  Existing RC examples and general DAE solvers alone do not complete this engine.
- [ ] Complete general engineering/technology workflows using the same native
  foundation: dimensional units, complex/frequency response, control/state-space
  systems, mechanics/materials/thermal/field models, estimation, optimization,
  tolerances/uncertainty and reproducible analysis. Define and qualify concrete
  operation inventories and workloads; domain names alone are not completion.
- [ ] Complete audio/sound/music generation and production mathematics: shared
  synthesis/oscillators/envelopes, physical/spectral models, filters/resampling,
  convolution/effects, mixing/gain/dynamics/loudness, spatial audio, timing and
  synchronization. Qualify bounded streaming state, numerical/audio references,
  native reuse, BASIC composition and actual device latency/listening separately.
- [ ] Complete video/movie generation and production foundations: frame/image
  processing, animation/interpolation, transforms/depth/compositing, timeline and
  sample/frame clocks, audio-video synchronization, effects and reproducible
  render/export pipelines. Reuse the image/geometry/audio kernels; qualify long
  sequences, seeking/recovery, timing, color and resource use, format/provider
  availability and editable BASIC workloads. Track codecs/editors/workflows and
  installed-OS/device acceptance as deliverables beyond mathematical primitives.

Owner clarification (2026-10-23): keep development CPU-first. Implement and
qualify backend math in Rust for modern host CPUs; GPU kernels, GPU dependencies
and tensor expansion as a separate development track are not current priorities.
Tensor code already present remains part of the CPU backend and should grow only
when it directly advances the shared math APIs or a real CPU workload.

Reference-project review (2026-10-25): [KnightOS](https://knightos.org/),
[RPN83P](https://github.com/bxparks/rpn83p),
[PineappleCAS](https://github.com/nathanfarlow/PineappleCAS),
[BOS](https://github.com/beckadamtheinventor/BOS),
[TI84Calculator](https://github.com/bifdu9898/TI84Calculator),
[ticalc.org](https://www.ticalc.org/), and
[awesome-ti-docs](https://github.com/TI-Toolkit/awesome-ti-docs) were reviewed
for transferable concepts. Keep these as design references, not presumed
dependencies or code to copy. Useful lessons are: expose composable calculator
workflows (including statistics, conversions, financial functions, and alternate
stack interaction) over shared Rust kernels; treat symbolic/exact arithmetic,
bounded memory, and leak-free ownership as first-class; maintain clear language,
OS-call, and developer references; and make recovery, file handling, updates,
and hardware/provider limitations explicit. ModernBASIC already has pieces of
these capabilities; the remaining work is to qualify their consistency,
usability, and cross-domain composition rather than count names or features.
The `math_engine` crate on docs.rs is a focused expression parser/evaluator with
symbolic differentiation and constant propagation, not a complete precision,
CAS, physics, or scientific-computing foundation. Rust 1.98's opt-in algebraic
floating-point methods permit reassociation and may change numerical results;
keep strict floating-point behavior as the backend default and consider such
methods only for isolated kernels after accuracy, repeatability, and performance
qualification. The current bundled Rust toolchain is older, so using those APIs
would also require a separately tested toolchain upgrade.

- [ ] Maintain shared native scalar/complex/exact/precision, array/matrix,
  sparse, transform, differentiation and solver foundations across those domains.
- [ ] Ensure Rust physics, graphics, audio and application engines can reuse the
  shared kernels directly, with consistent BASIC bindings above them. Real-time
  sample kernels must not require interpreter/value-list allocation per sample.
- [ ] Verify coherent data interchange and composed workloads across domains;
  keep interpreter and standalone application behavior consistent.
- [x] Complete bounded native ASCII/binary STL geometry interchange with composable
  BASIC MESH READ/WRITE, exact coordinate welding, explicit binary rounding
  allowance, topology-loss rejection and confined atomic file access. Qualify
  independent file records/geometry/normal/volume references, damaged inputs,
  cancellation/bounds, interpreter/export parity and large native round trips.
  Keep richer formats, arbitrary repair/booleans, slicer/printer and OS gates open.
  Evidence: `output/stl-interchange-2026-10-25/validation.json`.
- [x] Complete shared closed embedded triangle-domain mathematics in Rust/BASIC:
  exact point/batch containment, shell nesting/orientation, cavity/island volume
  and explicit whole-shell correction. Qualify independent exact non-axis rays,
  graph-grouped shells, concave/genus/nested/extreme-coordinate workloads, bounded
  cancellation/recovery, native reuse and interpreter/exported hollow parts.
  Keep arbitrary repair, booleans, formats and manufacturing/device gates open.
  Evidence: `output/solid-domain-2026-10-25/validation.json`.
- [x] Complete shared exact triangle contacts and bounded reusable mesh spatial
  intersection scans in Rust/BASIC. Distinguish proper indexed adjacency from
  overlap, never report a partial scan as clean, and qualify independent rational
  clipping, exhaustive face pairs, extreme scales, cancellation/budgets and native/
  interpreter/exported part-contact workloads. Keep solid containment/orientation,
  booleans, interchange and printer gates open. Evidence:
  `output/mesh-intersection-2026-10-25/validation.json`.
- [x] Complete shared bounded triangle surfaces and profile extrusion in Rust
  and BASIC: bounds, indexed edge/vertex topology, components/Euler, scaled area
  and exact-sum signed volume. Verify independent graph/exact/radical references,
  large meshes, extreme scales, cancellation, and native/interpreter/exported
  dimensioned-part behavior. Keep intersection/embedded-solid/model-format and
  printer gates open. Evidence: `output/triangle-mesh-2026-10-25/validation.json`.
- [x] Complete shared polygon centroid/triangulation APIs in Rust and BASIC.
  Reuse exact rational rounding with the symbolic engine; preserve original
  boundary vertices and winding in a nondegenerate triangle partition. Qualify
  exact moments/edge topology, extreme scales, cancellation/recovery and a
  BASIC profile feeding the mesh renderer, including exported application parity.
  Correct SVG/PNG fallback omission of raster mesh/image/filter layers and verify
  visible rendered pixels, not just matching files.
  Evidence: `output/polygon-mesh-2026-10-25/validation.json`.
- [x] Qualify shared robust Rust geometry: exact-input ORIENT2D/ORIENT3D,
  segment topology and validated simple-polygon area/perimeter/containment,
  native reuse and BASIC APIs. Check independent Fraction references, invalid
  topology, scale/size limits, cancellation, recovery and a dimensioned profile
  in the interpreter/exported application. Broader geometry/CAD gates stay open.
  Evidence: `output/robust-geometry-2026-10-25/validation.json`.
- [x] Complete flexible right-preconditioned GMRES through the shared Rust
  Arnoldi kernel and BASIC MFFGMRES, retaining each changing/nonlinear inverse
  action. Qualify combined V/Z memory and shared callback/work bounds, failure
  recovery, cancellation, independent dense/sparse references, and an editable
  inner/outer transport workload. Preserve the ordinary fixed-left API.
  Evidence: `output/flexible-gmres-2026-10-25/validation.json`.
- [x] Add fixed left inverse-action callbacks to native matrix-free GMRES and
  BASIC `MFGMRES`, reusing the shared recurrence and original-residual stopping.
  Qualify reused native/BASIC LU factors, singular projections, partial/nonfinite
  callbacks, failures, cancellation and combined call/work limits; independently
  check dense and sparse nonlocal models, a 32768-state native workload and a
  2048-state interpreted/standalone transport program. Evidence:
  `output/matrix-free-preconditioner-2026-10-25/validation.json`.
  Flexible/variable preconditioners are qualified in the subsequent checkpoint
  above. Retained operator graphs and full release/image/hardware acceptance
  remain open.
- [x] Add fixed linear callback composition through typed Rust
  `NativeLinearOperator` and BASIC `MFGMRES`, sharing the existing GMRES recurrence.
  Validate output shape/finiteness, row divisors, solver work/application/basis
  limits, scoped callback errors and cancellation. Qualify against independent
  SciPy dense/Woodbury solutions and an editable 8192-state low-rank system,
  including interpreter/standalone parity. Evidence:
  `output/matrix-free-callback-2026-10-25/validation.json`.
  Callbacks must be fixed linear maps; their internals are not sandboxed by the
  solver's work bounds. Retained graph objects, stronger preconditioners and
  broad release/hardware acceptance remain open.
- [x] Reuse the restarted sparse GMRES recurrence for nonsymmetric matrix-free
  Kronecker sums through Rust `gmres`/`gmres_with_restart` and BASIC `KRONSGMRES`.
  Preserve original residual checks, explicit non-convergence, row equilibration,
  cancellation and shared basis/work limits. Qualify native/BASIC recovery,
  existing sparse/implicit behavior, independent SciPy systems and a 2-D
  reaction-advection-diffusion workload. Evidence:
  `output/matrix-free-gmres-2026-10-25/validation.json`.
  General operator graphs, stronger matrix-free preconditioners and broader
  release/hardware qualification remain open.
- [x] Expose calibrated one-sided spectral power density through shared Rust
  `NativeSignalMath::periodogram` and BASIC `PERIODOGRAM`, using the current FFT,
  explicit sample/window/rate semantics, finite/resource validation and cancellation.
  Qualify against independent SciPy density and Parseval references and a two-tone
  editable BASIC workload. Evidence: `output/native-periodogram-2026-10-25/validation.json`.
  This does not complete audio capture, pitch/profile inference or device DSP.
- [x] Connect matrix-free separable systems to the shared CPU CG kernel through
  `NativeKroneckerSum::conjugate_gradient` and BASIC `KRONSCG`. Preserve existing
  sparse/implicit CG behavior, original-system residual stopping, cancellation,
  explicit non-convergence and work limits. Verify sparse parity, extreme common
  scales, invalid inputs and a 65,536-state analytic Poisson workload. Include an
  editable BASIC consumer; no full grid matrix is assembled. Broader operator
  graphs, nonsymmetric matrix-free solves and OS/hardware qualification remain open.
  Evidence: `output/matrix-free-cg-2026-10-25/validation.json`.
- [x] Correct `KRONSMV` argument validation so a missing vector returns a
  recoverable BASIC error instead of crashing the interpreter. Verify variable
  preservation, direct CLI recovery and `ON ERROR`/`RESUME NEXT` recovery.
  Audit 470 math entrypoints across 167,790 argument-count/data-shape calls with
  no escaping panic; run the 16 structured-math and three cancellation tests,
  formatting and strict all-target Clippy. This is bounded robustness coverage;
  numerical, packaged-image and hardware gates remain separate.
  Evidence: `output/math-call-boundaries-2026-10-25/validation.json`.
- [x] Add the typed CPU `NativeKroneckerSum` Rust operator and BASIC `KRONSMV`
  so users can apply `I_n ⊗ A + B ⊗ I_m` to row-major fields without allocating
  the full grid matrix. Factor storage and vector size are bounded to one
  million entries; each apply is capped at ten million nonzero-factor products,
  cancellation-aware and exactly accumulated per output. Verify direct Rust and
  BASIC parity with explicit sparse assembly, limits, invalid shapes, overflow
  and an editable 2-D stencil example. A 300-by-300 pair of tridiagonal factors
  applies to a 90,000-value field without the 8.1-billion-entry dense operator.
  This advances one matrix-free structured operator; general operator graphs
  and wider scientific workloads remain open.
  Evidence: `output/matrix-free-kronecker-sum-2026-10-25/validation.json`.
- [x] Expose the existing quaternion rotation kernel as typed crate-root
  `NativeQuaternion` operations and add BASIC `QUATSLERP` using shortest-arc
  spherical interpolation. Reuse the typed rotation and matrix paths in the
  existing BASIC functions; qualify endpoints, antipodal/near-parallel cases,
  unit-length output, and vector rotation parity. The typed Rust and BASIC
  interpreter checks pass; standalone packaged-app parity remains in the broader
  open integration gate. Evidence:
  `output/quaternion-backend-2026-10-24/validation.json`.
- [x] Expose the shared adaptive CPU RK4-step-doubling ODE solver to native Rust
  consumers as `NativePhysicsEquation::integrate_adaptive_rk4`, returning typed
  time/state samples without interpreter `Value` allocation. Keep BASIC
  `ODESOLVE` on the same kernel. Verify analytic oscillator accuracy, full
  interpreter/native endpoint parity, reverse integration and zero-length
  intervals. Evidence:
  `output/native-physics-adaptive-rk4-2026-10-25/validation.json`.
- [x] Expose the existing bounded SUNDIALS BDF/Adams integration core to direct
  Rust physics consumers with typed requested-time trajectories, solver options
  and work diagnostics. Support dense and matrix-free SPGMR paths plus analytic
  BDF Jacobians, terminal roots and forward parameter sensitivities; verify
  stiff/nonstiff references, analytic sensitivity references, BASIC parity,
  input bounds, callback failures, thread-local cancellation and staged provider
  behavior. Bounded finite-difference terminal-root time and total event-state
  sensitivities are now available. `CVODEEVENTVAR` adds analytic variational
  event-time/event-state sensitivities; adjoint sensitivities remain open.
  Evidence:
  `output/native-cvode-event-sensitivity-2026-10-25/validation.json`.
  Evidence: `output/native-cvode-rust-api-2026-10-25/validation.json`.
- [x] Expose crate-root `NativeTensor` and `NativeTensorTape` for direct Rust
  consumers: bounded forward operations, native reverse-mode graph construction,
  graph-preserving reshape/transpose and axis reductions, convolution, pooling,
  gather, softmax/masked softmax, MSE and cross-entropy losses, gradients, and
  tape-local handle ownership; checked SGD/Adam updates; analytic HVP, dense
  Hessian, directional derivative orders 3–8, and versioned exact leaf/Adam
  checkpoints. Reuse BASIC kernels,
  pullbacks, validation rules and work budgets without interpreter `Value`
  conversion. Verify typed Rust/BASIC parity, gradients, optimizer convergence,
  higher derivatives, stale-handle behavior and atomic rejection. Evidence:
  `output/native-tensor-tape-spatial-2026-10-25/validation.json` and
  `output/native-tensor-optimizers-2026-10-25/validation.json` and
  `output/native-tensor-higher-derivatives-2026-10-25/validation.json` and
  `output/native-tensor-state-2026-10-25/validation.json`.
- [x] Add versioned, checksummed exact stream snapshots to native Rust `BasicRng`
  and BASIC `RNGSTATE$()`/`RNGRESTORE()`. Resume includes the cached value used
  by `RND(0)`; malformed, unsupported or corrupted records fail atomically.
  Verify uniform, normal and binomial replay through Rust, plus BASIC `RND`
  replay and cached-value behavior.
  Evidence: `output/random-checkpoints-2026-10-24/validation.json`.
- [x] Run the extended exact-executable CPU math campaign: all 71 reference
  suites pass, totaling 206,951 checks, including 8,192 seeded randomized
  cases across eight families and configured NLopt, UMFPACK, CVODES and HiGHS
  provider checks. This does not close performance/memory, composed-workload,
  image, or hardware gates. Evidence:
  `output/math-reference-campaign-2026-10-24/validation.json`.
- [x] Refresh and qualify the native-host optimized runtime after the new
  matrix-free, signal and probability checkpoints. Build through the normal
  launcher, require every registered numerical-reference suite against that
  executable, check lifecycle/launcher/interpreter behavior and retain source,
  provider and executable identities. Evidence location:
  `output/native-release-refresh-2026-10-25/validation.json`.
  Keep private host acceptance separate from VM/image/hardware release gates.
- [ ] Qualify useful research/simulation/analysis workloads and scalability,
  including precision/conditioning, convergence, performance, memory and graceful
  cancellation. Document justified resource limits and remaining capability gaps.

- [x] Add CPU Rust `BSOPTION` European call/put valuation with continuous
  dividend yield and BASIC binding; qualify canonical prices, independent
  normal-CDF references, put-call parity, expiry/zero-volatility/extreme-volatility
  limits and invalid-domain errors. Add an editable BASIC example. Evidence:
  `output/black-scholes-cpu-2026-10-24/validation.json`.
- [x] Add CPU Rust `BSGREEKS` for price, delta, gamma, vega, calendar-time
  theta and rho; return a composable BASIC list, reject derivative singularities
  at zero time/volatility, and qualify both option types against independent
  formulas and finite differences. Expand the BASIC example. Evidence:
  `output/black-scholes-cpu-2026-10-24/validation.json`.
- [x] Expose typed Rust pricing/Greeks APIs and input/result structs at the crate
  root so native consumers can reuse the finance kernel without constructing
  interpreter values. Validate direct Rust reuse, finite input/result checks,
  and parity with BASIC APIs. Evidence:
  `output/black-scholes-greeks-cpu-2026-10-24/validation.json`.
- [x] Add bounded CPU `BINOMIALOPTION` American call/put pricing in Rust with
  a BASIC binding and typed crate-root API. Check early exercise, dividend calls,
  risk-neutral probability validation, finite-tree convergence against European
  Black-Scholes, and the 2,048-step work/memory limit. Evidence:
  `output/american-binomial-cpu-2026-10-24/validation.json`.
- [x] Add bounded CPU European implied-volatility inversion as BASIC `BSVOL`
  and a typed crate-root Rust API. Check call/put dividend-price recovery,
  zero-volatility intrinsic handling, no-arbitrage rejection, volatility and
  iteration limits, and BASIC/native parity. Evidence:
  `output/implied-volatility-cpu-2026-10-24/validation.json`.
- [x] Replace BASIC `FNINT`'s Simpson refinement with bounded embedded CPU
  7/15-point Gauss-Kronrod quadrature. Verify the existing extreme-scale cases,
  analytic smooth and oscillatory references, reversed bounds, narrow peaks,
  the refinement-depth error path and BASIC composition. The one-million
  function-evaluation ceiling is documented and enforced. Evidence:
  `output/gauss-kronrod-cpu-2026-10-24/validation.json`.
- [x] Add `SYMSOLVE` as a Rust/LAPACK CPU kernel with a typed Rust API and BASIC
  frontend. Qualify Bunch-Kaufman 2-by-2 pivoting on a zero-diagonal indefinite
  system, multiple RHS columns, common scales, original residuals and invalid
  input rejection. Evidence:
  `output/symmetric-indefinite-cpu-2026-10-24/validation.json`.
- [x] Add CPU complex Householder QR as `CLQR` and typed
  `NativeComplexMatrix::qr`, returning reduced Q/R for tall, square, wide and
  rank-deficient matrices. Scale reflector work and qualify reconstruction,
  orthogonality, extremes, BASIC composition and failure recovery. Evidence:
  `output/complex-qr-cpu-2026-10-24/validation.json`.
- [x] Add CPU complex singular values as `CSVDVALS` and
  `NativeComplexMatrix::singular_values`, using the CPU LAPACK realification
  path with multiplicity-preserving exact averaging. Qualify analytic,
  rectangular, rank-deficient, zero and scaled cases plus BASIC composition.
  Evidence: `output/complex-svdvals-cpu-2026-10-24/validation.json`.
- [x] Complete and qualify the bounded CPU reduced complex SVD as BASIC `CSVD`
  and typed `NativeComplexMatrix::svd`, returning complex `U`, descending `S`
  and `Vᴴ`. Verify tall, square, wide, rank-deficient and zero matrices,
  reconstruction, orthogonality, extreme common scales, BASIC composition,
  failure recovery and the Linux LAPACK ABI. Targeted Rust/BASIC tests,
  library tests and documentation checks pass; the full repository suite was
  stopped before a test could write a compiled executable under `/tmp`, as
  required by the owner. Evidence: `output/complex-svd-cpu-2026-10-24/`.
- [x] Add CPU Hermitian complex eigenvalues as `CHEIGVALS` and
  `NativeComplexMatrix::hermitian_eigenvalues`, with relative symmetry checks,
  ascending eigenvalues and multiplicity preservation through CPU LAPACK. Qualify
  analytic, repeated, zero and scaled spectra; reject invalid shapes/symmetry.
  Evidence: `output/complex-hermitian-eigenvalues-cpu-2026-10-24/validation.json`.
- [x] Add bounded CPU Hermitian complex eigensystems as `CHEIGEN` and typed
  `NativeComplexMatrix::hermitian_eigen`, returning ascending eigenvalues and
  complex-orthonormal eigenvectors. Recover repeated realified eigenspaces,
  check eigenpair residuals, and qualify scaled, degenerate and BASIC workloads.
  Evidence: `output/complex-hermitian-eigenvectors-cpu-2026-10-24/validation.json`.
- [x] Add general CPU complex eigenvalues/eigenvectors as `CGEIGVALS`/`CGEIGEN`
  and typed `NativeComplexMatrix::eigen` using LAPACK ZGEEV. Scale the input,
  normalize right eigenvectors, sort matching pairs, check original-matrix
  eigenpair residuals, and qualify non-normal, defective, repeated, conjugate,
  extreme-scale, BASIC and failure-recovery cases. Evidence:
  `output/complex-eigen-cpu-2026-10-24/validation.json`.
- [x] Expose general nonsymmetric eigensystems for real matrices through typed
  `NativeRealMatrix::general_eigen()` and BASIC `GEIGEN`/`GEIGVALS`, reusing the
  checked complex CPU solver so real inputs can yield conjugate modes. Verify
  the rotation-matrix ±i spectrum, eigenpair residuals, BASIC parity and example.
  Evidence: `output/real-general-eigen-cpu-2026-10-24/validation.json`.
- [x] Add typed `NativeRealMatrix::generalized_eigen()` and BASIC `GEIGGEN` for
  real matrix pencils, retaining projective alpha/beta pairs and complex modes.
  Verify the two-mass analytic spectrum, trace/product invariants, pencil
  residuals, real BASIC composition and editable workload. Evidence:
  `output/real-generalized-eigen-cpu-2026-10-24/validation.json`.
- [x] Add CPU `CGEIGCOND` and typed `NativeComplexMatrix::eigen_sensitivity`
  for reciprocal eigenvalue condition estimates from matching left/right
  eigenvectors. Check both eigenpair residuals, analytic normal/non-normal
  references, near-defective sensitivity, extreme common scales and BASIC
  composition/recovery. Evidence:
  `output/complex-eigen-sensitivity-cpu-2026-10-24/validation.json`.
- [x] Expose the checked eigenvalue-sensitivity calculation directly to real
  matrix Rust consumers as `NativeRealMatrix::eigen_sensitivity()`. Verify exact
  parity with the complex API on an analytic non-normal matrix, unit condition
  estimates for a normal matrix, and nonsquare rejection. Evidence:
  `output/real-eigen-sensitivity-cpu-2026-10-24/validation.json`.
- [x] Add bounded generalized complex eigensystems as `CGEIGGEN` and typed
  `NativeComplexMatrix::generalized_eigen`, preserving finite, infinite and
  indeterminate eigenvalues as homogeneous alpha/beta pairs and checking right
  eigenvectors against the pencil residual. Qualify singular B, extreme common
  scaling, BASIC composition, and an analytic two-mass vibration model with
  independent modal references. Focused evidence:
  `output/complex-generalized-eigen-cpu-2026-10-24/validation.json`.
- [x] Add CPU generalized-pencil condition estimates as BASIC `CGEIGGENCOND`
  and typed `NativeComplexMatrix::generalized_eigen_sensitivity()`, using the
  Linux LP64 LAPACK ZGGEVX driver with balancing disabled. Preserve matching
  projective alpha/beta pairs; verify scalar and non-normal references, complex
  coefficients, extreme common scaling, singular B, BASIC composition and
  error recovery; include an editable two-mass BASIC workload. State clearly
  that LAPACK estimates may exceed one and are not forward-error bounds. Evidence:
  `output/generalized-eigen-sensitivity-cpu-2026-10-24/validation.json`.
- [x] Add CPU complex inverse as `CLINVERSE` and
  `NativeComplexMatrix::inverse`, reusing one LU factorization for all identity
  columns and checking fresh original-system residuals. Qualify extreme common
  scales, singular/shape failures, BASIC composition and recovery. Evidence:
  `output/complex-inverse-cpu-2026-10-24/validation.json`.
- [x] Add bounded CPU complex Hermitian positive-definite Cholesky as
  `CCHOLESKY` and `NativeComplexMatrix::cholesky`. Use scale normalization and
  exact binary inner products; qualify reconstruction from `1e-200` to `1e200`,
  Hermitian validation, positive-definiteness failures, BASIC composition and
  recovery. Evidence: `output/complex-cholesky-cpu-2026-10-24/validation.json`.
- [x] Add reusable CPU complex LU factors as `CLUFACT`/`CLLUSOLVE` and typed
  `NativeComplexLuFactor`/`solve_many` APIs. Qualify scaled partial pivoting,
  repeated vector and multiple-RHS solves, extreme common scales, original
  residuals, malformed dimensions and atomic BASIC recovery. Evidence:
  `output/complex-lu-cpu-2026-10-24/validation.json`.
- [x] Add CPU complex least squares as `CLSTSQ` and typed
  `NativeComplexMatrix::least_squares`, using column-pivoted Householder QR for
  full-column-rank overdetermined systems. Verify residual and normal-residual
  reports, rank and shape rejection, scaling, overflow, BASIC composition and
  editable source. Evidence:
  `output/complex-least-squares-cpu-2026-10-24/validation.json`.
- [x] Add SVD-based CPU complex minimum-norm least squares as `CLSTSQMIN` and
  typed `NativeComplexMatrix::least_squares_minimum_norm`, with a relative
  singular-value cutoff, estimated rank and freshly recomputed original fit and
  normal residuals. Verify underdetermined, rank-deficient, truncated-rank,
  extreme-scale, zero-matrix, BASIC composition and assignment recovery cases.
  Evidence: `output/complex-minnorm-lsq-cpu-2026-10-24/validation.json`.
- [x] Add `CLSTSQMINM` and typed complex multi-response minimum-norm least
  squares, sharing one SVD across RHS columns and reporting each fit/stationarity
  residual with a common numerical-rank cutoff. Verify typed parity with
  separate solves, rank-deficient and BASIC composed cases, example execution,
  dimensions/cutoffs, and bounded total work. Evidence:
  `output/complex-multi-minnorm-cpu-2026-10-24/validation.json`.
- [x] Bound CPU `PARIFACTORS` receiver verification by the input integer's bit
  length and stop exact product reconstruction as soon as it exceeds the input.
  A hostile maximum `u32` multiplicity now fails and returns control through
  BASIC `ON ERROR` under a subprocess deadline. Restore APIs compatible with the
  declared Rust 1.74 baseline in DAE/event result guards and remove deprecated
  float-pattern matches in finance and event validation. The full Rust library
  suite passes (987 tests) with configured HiGHS, NLopt, SuiteSparse and serial
  SUNDIALS CPU providers; seven PARI runtime tests pass. Evidence:
  `output/pari-verifier-bound-2026-10-24/validation.json`.
- [x] Establish a source-based domain capability/gap matrix in
  `docs/MATH_CAPABILITIES.md`, covering actual modules, BASIC APIs, direct native
  reuse boundaries and remaining work. Keep it current as capabilities change.
- [x] Add crate-root typed `NativeRealMatrix`/`NativeRealSolveResult` APIs for
  bounded shape-aware real dense data, transpose/products, determinant/inverse,
  and residual-reported solves reusing checked BASIC CPU kernels. Verify Rust/BASIC
  product parity, pivoting, residuals, singular/input rejection and empty axes.
  Evidence: `output/native-real-matrix-api-cpu-2026-10-24/validation.json`.
- [x] Extend `NativeRealMatrix` with reduced CPU QR and SVD results; qualify
  rectangular reconstruction, orthogonality, rank-deficient QR, near-zero SVD
  values, output shapes and invalid decomposition results. Keep rank estimates
  distinct from exact certificates. Evidence:
  `output/native-real-matrix-decompositions-cpu-2026-10-24/validation.json`.
- [x] Expose reusable typed CPU real LU factors from `NativeRealMatrix`, with
  vector and multiple-RHS solves and fresh per-RHS residual norms against the
  retained original matrix. Qualify pivot swaps, factor reuse, singular rejection
  and malformed RHS handling. Evidence:
  `output/native-real-lu-cpu-2026-10-24/validation.json`.
- [x] Expose typed CPU real least-squares using the BASIC `LSTSQ` SVD and
  relative-rank policy, including minimum-norm results, numerical-rank reporting
  and fresh fit/normal residuals. Qualify tall, rank-deficient, underdetermined,
  empty-axis and invalid-input cases. Evidence:
  `output/native-real-least-squares-cpu-2026-10-24/validation.json`.
- [x] Expose typed CPU real Cholesky and symmetric eigensystem APIs using the
  BASIC `CHOLESKY`/`EIGH` kernels. Qualify factor/eigenpair reconstruction,
  orthogonality, symmetry and definiteness rejection, BASIC parity and empty
  square matrices. Evidence:
  `output/native-real-symmetric-factors-cpu-2026-10-24/validation.json`.
- [x] Add shape-aware typed Rust multiple-RHS symmetric-indefinite solves with
  per-column original-system residuals, reusing the BASIC `SYMSOLVE` kernel.
  Qualify 2-by-2 Bunch–Kaufman pivoting, 3-by-3 indefinite solves, malformed
  dimensions and symmetry failures. Evidence:
  `output/native-real-symmetric-solve-cpu-2026-10-24/validation.json`.
- [x] Expose typed scale-aware real matrix rank and condition diagnostics using
  the shared BASIC `MATRANK`/`RCOND`/`COND` SVD analysis, including singular and
  zero-matrix semantics, relative cutoffs, extreme common scales and BASIC parity.
  Evidence: `output/native-real-diagnostics-cpu-2026-10-24/validation.json`.
- [x] Expose direct Rust mean, variance, standard deviation, RMS and covariance
  over finite real samples, sharing stable scale-normalized kernels with BASIC.
  Verify sample/population semantics, extreme offsets/scales, overflow behavior,
  BASIC parity, cancellation checks and empty-sample recovery. Evidence:
  `output/native-real-statistics-cpu-2026-10-24/validation.json`.
- [x] Expose typed Rust correlation and simple linear regression with the same
  scale-normalized bivariate kernels used by BASIC `CORREL`/`LINREG`. Qualify
  coefficient and BASIC parity, extreme scale separation, constant samples and
  invalid pairs. Evidence:
  `output/native-real-bivariate-analysis-cpu-2026-10-24/validation.json`.
- [x] Add typed Rust and BASIC multivariable sample/population covariance and
  Pearson correlation matrices over row-wise observations. Share scale-normalized
  kernels, enforce work/matrix bounds, and qualify symmetry, pairwise parity,
  BASIC parity and invalid domains. Evidence:
  `output/native-real-multivariate-statistics-cpu-2026-10-24/validation.json`.
- [x] Expose typed Rust multiple linear regression and BASIC `MULTIREG` over
  the shared CPU SVD least-squares kernel, with explicit intercept control,
  coefficients, numerical rank and residual diagnostics. Qualify known models,
  BASIC/native parity, rank deficiency, invalid inputs and coefficient reuse in
  the runnable BASIC `DOT` prediction example. Evidence:
  `output/native-real-multiple-regression-cpu-2026-10-24/validation.json`.
- [x] Add typed Rust `NativeRealMatrix::least_squares_many` and BASIC `LSTSQM`
  for multiple RHS columns with a single shared CPU SVD, per-column fit and
  normal-residual diagnostics, rank and explicit resource bounds. Qualify known
  models, parity with repeated single-RHS solves, BASIC `MATMUL` composition,
  invalid dimensions/cutoffs and recovery. Evidence:
  `output/native-real-multi-response-least-squares-cpu-2026-10-24/validation.json`.
- [x] Add typed Rust weighted multi-response least squares and BASIC `WLSTSQ`/
  `WLSTSQM` over the shared CPU SVD. Normalize nonnegative weights safely,
  return the common weight scale and per-response residual/rank diagnostics, and
  qualify zero-weight exclusions, common rescaling, errors and BASIC workflows.
  Evidence: `output/weighted-real-least-squares-cpu-2026-10-24/validation.json`.
- [x] Add typed CPU NativeCsrMatrix::weighted_lsqr and BASIC SPWLSQR with
  zero-weight row exclusion, maximum-weight normalization, typed residual/scale
  reporting, optional column preconditioning, strict work/cancellation bounds,
  explicit stored-coefficient underflow rejection and a 512-variable sparse
  weighted fit. Evidence:
  output/weighted-sparse-lsqr-cpu-2026-10-24/validation.json.
- [x] Run the complete Rust library unit suite with the configured amd64 CPU
  providers and workspace-local compiler; all 992 tests pass. Preserve the full
  log and provider/test-binary hashes. Integration, Python, image and hardware
  gates remain separate. Evidence:
  output/native-math-library-suite-2026-10-24/validation.json.
- [ ] Complete the defined supported scope in `docs/MATH_APPLICATIONS.md` and
  `docs/MATH_MATURITY.md`, with clear domains, precision, non-convergence, resource
  limits and cancellation. No placeholder or demonstration-only completion claims.
- [x] Remove 13 straightforward warning-denied Clippy findings in the complex
  matrix and 4ti2 backends: flatten array iterators directly, iterate RHS and
  solution data without index-only loops, and write hexadecimal provider IDs
  without allocating one formatted String per byte. Focused verification passes:
  all 53 `complex_linalg_tests`, the provider-backed `basic_zsolve` test, and
  `cargo fmt --all -- --check`. Both changed Rust modules are clear of Clippy
  diagnostics. Evidence: `output/clippy-after-complex-iterators-2026-10-25.log`.
- [x] Give the internal Huber-regression kernel a named result struct rather
  than an eight-field positional tuple; preserve the public result values. Also
  iterate polynomial output coefficients directly and match LAPACK INFO status
  explicitly. Focused verification passes: 17 statistics tests, 4 polynomial
  tests, and 20 real linear-algebra tests; the changed Rust files produce no
  Clippy diagnostics. Evidence: `output/clippy-after-statistics-polynomial-2026-10-25/validation.json`.
- [x] Refactor IDA reset sensitivity projection, event-time sensitivity and
  saltation helpers to use named request/result structs. Replace index-only IDA
  sensitivity loops with checked iterator traversal and validate all output
  matrix dimensions before copying into SUNDIALS vectors. All 66 IDA integration,
  native API and sensitivity tests pass; `ida_native.rs` has no remaining Clippy
  findings. Evidence: `output/ida-native-clippy-refactor-2026-10-25/validation.json`.
- [x] Collapse the redundant nested root-callback branch in the Rust CVODE
  dispatcher. All 21 CVODE integration tests and formatting checks pass. Strict
  all-target Clippy fell from 55 to 54 errors across the remaining files. Evidence:
  `output/clippy-cvode-else-if-2026-10-25/cvode-tests.log` and
  `output/clippy-cvode-else-if-2026-10-25/clippy.log`.
- [x] Remove five warning-denied Clippy diagnostics from shared math code:
  simplify the regularized-beta domain check, iterate ILU diagonal and GMRES
  basis entries directly, and use an ordered match for exact algebraic-root
  decimal selection. The focused probability, sparse GMRES and high-precision
  suites pass all 30 tests; library Clippy falls from 54 diagnostics in 19 files
  to 49 in 16. Evidence: `output/clippy-math-kernels-2026-10-25/`.
- [x] Remove 25 further library Clippy findings through direct string writes,
  checked iterator traversal and a named quadratic-solver candidate record;
  preserve calculation order and public API signatures. Correct 13 additional
  unit-test style findings exposed by the broader check. The full library suite
  passes 1,008 tests; six targeted integration suites pass another 92, with no
  failures or skips. Formatting passes. Evidence:
  `output/rust-quality-gate-2026-10-25/validation.json`.
- [x] Clear the package-wide warning-denied Clippy gate for all Rust targets.
  Group internal CVODE callbacks/sensitivity inputs, IDA linearization inputs
  and tangent-event brackets into named structures. Remove unchecked sparse
  callback option assumptions and retain explicit native Jacobian write bounds.
  Preserve all 27 public CVODE/IDA function signatures; six published functions
  retain individually documented argument-count exceptions for API compatibility.
  Repair the stale unavailable-backend CVODE signature and compile its fallback
  in unit tests, comparing both entry-point types against Linux. Resolve the 31
  further test/example findings revealed when downstream targets became reachable.
  All 1,298 selected tests pass: 1,009 library tests, 287 tests across 15 complete
  integration targets, plus two focused integration tests. All-target Clippy,
  formatting and warning-denied Rustdoc pass. The 4,096-state diffusion program
  matches its Fourier reference within 1.04e-9, preserves mass, and produces
  identical interpreter/standalone CSV and console output; the standalone run
  peaks at 14,612 KiB RSS. Nine debug graphics smoke scenarios also pass; they
  are not release-performance or hardware qualification. Remove only this
  standalone run's verified disposable compile cache, retaining its executable,
  source, CSV and compact evidence. The whole math/OS acceptance remains open.
  Evidence: `output/physics-quality-2026-10-25/validation.json`.
- [ ] Expose useful Rust features through documented BASIC statements/functions;
  verify that outputs/data compose with other backend APIs and user programs.
- [x] Expose `NativeIda::integrate_variational_hybrid` through BASIC
  `IDAHYBRIDVAR`, with editable parameterized residual/event/reset callbacks,
  IDASens trajectory sensitivities, event/reset saltation derivatives, error
  estimates, solver bounds and explicit result shapes. Verify a three-parameter
  index-one DAE against closed-form event-time, reset and post-event sensitivities
  through the BASIC interpreter using SPGMR; preserve `IDAHYBRIDSENS` behavior.
  Evidence: `tests/ida_tests.rs::ida_hybrid_variational_sensitivities_compose_from_basic_with_spgmr`.
- [x] Expose `NativeRealStatistics::percentile` and `median` to Rust using the
  same overflow-safe linear interpolation as BASIC `PERCENTILE`/`MEDIAN`; verify
  endpoints, interpolation, extreme/subnormal samples, invalid inputs and BASIC
  parity. Evidence: `output/native-statistics-quantiles-2026-10-25/validation.json`.
- [x] Expose robustly scaled bias-corrected `skewness` and excess `kurtosis` in
  `NativeRealStatistics`, sharing BASIC's kernels, finite/sample-size checks and
  cancellation checkpoints. Verify BASIC parity, constant samples and rejection
  cases. Evidence: `output/native-statistics-moments-2026-10-25/validation.json`.
- [x] Expose `NativeFinancialMath::net_present_value` for Rust consumers and route
  BASIC `NPV` through that shared typed kernel. Verify exact accumulation,
  period-zero convention, finite-input/rate errors, empty streams, cancellation
  checkpoints and BASIC parity. Evidence:
  `output/native-financial-npv-2026-10-25/validation.json`.
- [x] Extend `NativeFinancialMath` with typed present value, future value and
  payment operations; route BASIC `PV`, `FV` and `PMT` through the same kernels.
  Verify ordinary/due timing, zero-rate and zero-period limits, input domains,
  and native/BASIC parity. Evidence:
  `output/native-financial-cashflows-2026-10-25/validation.json`.
- [x] Expose typed `NativeFinancialMath::internal_rate_of_return` and route BASIC
  `IRR` through the same bounded solver. Preserve the Newton guess plus
  bracket/bisection fallback, enforce a ten-million evaluation budget, and check
  cancellation in preprocessing and solver loops. Verify native/BASIC roots,
  scaled-flow replay, invalid signs/guess and cancellation. Evidence:
  `output/native-financial-irr-2026-10-25/validation.json`.
- [x] Expose typed `NativeFinancialMath::compound` and route BASIC `COMPOUND`
  through the same scaled periodic-growth kernel. Verify native/BASIC parity,
  tiny rates, zero-period/zero-rate cases and full-loss boundary. Evidence:
  `output/native-financial-compound-2026-10-25/validation.json`.
- [x] Add bounded CPU binary logistic regression through typed Rust
  `NativeRealStatistics` and BASIC `LOGITREG`, with scaled predictors, explicit
  L2/intercept semantics, cancellation, work limits and gradient-based convergence.
  Qualify independently constructed grouped-binomial coefficients and likelihood,
  `1e150` predictor rescaling, BASIC/native parity, invalid labels/parameters,
  work-limit rejection and the editable science example. Evidence:
  `output/native-logistic-regression-cpu-2026-10-25/validation.json`.
- [x] Add typed Rust `NativeStudentT` CDF and quantile with BASIC `TCDF`/`TINV`;
  qualify known critical values, round trips into the `1e-50` tail, invalid
  domains, cancellation and interpreter/native parity, and document the limits.
  The mixed math workload additionally verifies `1e-30` tail behavior through
  both interpreted and standalone BASIC. Do not claim a complete inference
  package. Evidence:
  `output/native-student-t-cpu-2026-10-25/validation.json` and
  `output/standalone-math-student-t-cpu-2026-10-25/validation.json`.
- [x] Add typed Rust one-sample t inference and BASIC `TTEST1`, sharing the
  stable mean/standard-deviation and Student's t kernels. Return t, degrees of
  freedom, two-sided p-value and a central confidence interval; test a known
  reference, tiny tails, invalid/constant samples, cancellation and Rust/BASIC/
  standalone parity. Pooled and paired procedures remain open.
  Evidence: `output/native-student-t-cpu-2026-10-25/validation.json` and
  `output/standalone-math-student-t-cpu-2026-10-25/validation.json`.
- [x] Share gamma/chi-square probability density and both tails between typed
  Rust `NativeGammaDistribution` and BASIC, adding direct `GAMMASF`/`CHISQSF`.
  Correct scaled-quotient underflow, small-shape tail cancellation, large-shape
  normalization and exact binary mean displacement. Add 1,201 independent
  reference cases to the permanent math gate (now 73 suites), retain disputed
  library comparisons, and check cancellation/domain/assignment recovery.
  Reuse the existing shared build cache; no new runtime dependency, VM disk or
  provider is needed. Broader distribution/inference work, full release,
  cross-architecture and image gates remain open. Evidence:
  `output/gamma-probability-2026-10-25/validation.json`.
- [x] Correct Student's t central and extreme-tail arithmetic in the shared CPU
  Rust kernel. Preserve representable tails when t squared would overflow,
  invert subnormal probabilities with logarithmic comparisons, resolve quantiles
  near the median and reject answers beyond finite binary64. Fix the documented
  two-sided p-value formula to use the small tail directly. Add 909 independent
  SciPy/analytic checks to the permanent math release gate (now 72 suites);
  retain BASIC/native parity, cancellation and failed-assignment recovery tests.
  Reuse the single compact Cargo target; no provider, VM disk or OS image is
  rebuilt. Broad accuracy and image/hardware acceptance remain open. Evidence:
  `output/student-t-range-2026-10-25/validation.json`.
- [x] Add typed Rust Welch inference and BASIC `TTEST2` for independent
  unequal-variance samples. Qualify reference t/df/p/interval values, invalid
  and constant samples, interpreter/native parity, standalone parity and the
  editable science example; document the combined-input cap and distinguish it
  from pooled and paired procedures. Synchronous t-test kernels do not expose a
  cancellation checkpoint. Evidence:
  `output/native-welch-t-cpu-2026-10-25/validation.json` and
  `output/standalone-math-welch-t-cpu-2026-10-25/validation.json`.
- [x] Expose the shared bounded CPU arbitrary-length complex FFT/IFFT as typed
  crate-root `NativeComplexFft` operations; route BASIC `FFT`/`IFFT` through the
  same staged kernel. Verify arbitrary-length transforms against an independent
  direct DFT, inverse round trips, BASIC/native parity and atomic errors. Document
  work and workspace caps and that this allocating API is not a hard real-time
  callback primitive. Evidence: `output/native-complex-fft-cpu-2026-10-25/validation.json`.
- [x] Expose bounded Rust `NativeSignalMath` convolution, autocorrelation and
  cross-correlation, and route BASIC `CONVOLVE`/`AUTOCORR`/`XCORR` through the
  same exact-product kernels. Verify known lag/orientation results, BASIC/native
  parity, finite and empty-input rules, operation limits and cancellation. Document
  bounds and binary64 result limits. Evidence:
  `output/native-signal-math-cpu-2026-10-25/validation.json`.
- [x] Expose typed `NativeSignalMath::window` with `NativeSignalWindow` shapes
  for Hann, Hamming, Blackman, Blackman–Harris and Kaiser, and route BASIC window
  functions through the same formulas and bounds. Verify BASIC/native parity,
  zero/one-length behavior, Kaiser beta/work limits and cancellation. Evidence:
  `output/native-signal-math-cpu-2026-10-25/validation.json`.
- [x] Add a bounded CPU FFT convolution path as typed
  `NativeSignalMath::convolve_fft` and BASIC `FFTCONVOLVE`, keeping exact
  `CONVOLVE` semantics intact. Verify BASIC/native parity, a closed-form impulse
  response at 32,768 by 16,384 samples (536,854,528 direct products), FFT
  workspace limits, and cancellation. Evidence:
  `output/fft-convolution-cpu-2026-10-25/validation.json`.
- [x] Add bounded CPU matrix exponential as typed `NativeRealMatrix::exponential()`
  and BASIC `MATEXPM`; qualify analytic matrices, scaling/squaring, BASIC
  composition, overflow, shape and work-limit behavior. Evidence:
  `output/matrix-exponential-cpu-2026-10-24/validation.json`.
- [x] Qualify attention masking/shared softmax gradients: 1,797 native tests,
  453 Python tests without skips, all 55 math-reference suites (187,819 checks),
  strict Clippy/Rustdoc, recovery and paced native/mixed audio. Evidence:
  `output/attention-math-2026-10-23/validation.json`; no image/VM deployment.
- [x] Implement native `TMASKEDSOFTMAX` for causal/padding attention with
  right-aligned truth-mask broadcasting, exact excluded zero probabilities/
  gradients, defined all-masked rows, rejection of differentiable masks and
  stronger shared softmax gradient scaling. Pass independent forward/central-
  difference checks, attention composition and 40-step masked Adam training.
- [x] Qualify exact RREF/shared rational parsing and rank: 1,792 native tests,
  453 Python tests without skips, all 54 math-reference suites (187,791 checks),
  strict Clippy/Rustdoc, recovery and paced audio. Evidence:
  `output/rref-math-2026-10-23/validation.json`; no image/VM deployment.
- [x] Implement bounded exact Rust `RREF`/`MATRREF`, `SYMRREF`, `RREFPIVOTS`
  and `RREFINFO`, typed integer/real/text constructors, exact pivot decisions,
  final-only numeric rounding, 64-dimensional support and 216 independent
  SymPy/Fraction checks. Share elimination with high-precision rank and share
  decimal parsing with high-precision evaluation; require SymPy in the gate.
- [x] Qualify subspace bases: 1,787 native tests, 452 Python tests without skips,
  all 53 math-reference suites (187,575 checks), strict Clippy/Rustdoc,
  recovery and paced native/mixed audio. Evidence:
  `output/subspace-math-2026-10-23/validation.json`; no image/VM deployment.
- [x] Implement native numerical `COLSPACE`/`MATCOLSPACE`, `ROWSPACE`, and
  `NULLSPACE`/`MATNULL`, sharing normalized inputs and relative rank policy with
  `MATRANK`. Include all wide-matrix null directions through bounded full-right
  SVD, orthonormal column outputs, zero-dimensional basis shapes and 337
  independent SciPy/projector/residual/application checks.
- [x] Qualify reusable LU: 1,782 native tests, 452 Python tests without skips,
  all 52 math-reference suites (187,238 checks), strict Clippy/Rustdoc,
  recovery and paced native/mixed audio. Evidence:
  `output/lu-math-2026-10-22/validation.json`; no image or VM deployment.
- [x] Implement reusable native LU factors with `LU`/`MATLU`, `LUFACTOR` and
  `LUSOLVE`: rectangular/singular inspection, explicit row permutations, repeated
  vector/matrix RHS solves, validated editable packets, work/cancellation limits
  and 133 independent reconstruction/SciPy/analytical checks. Preserve the
  existing solve/determinant implementations and reject the unsafe donor code.
- [x] Qualify the conditioning checkpoint: 1,777 native tests, 452 Python tests
  without skips, all 51 math-reference suites (187,105 checks), strict Clippy/
  rustdoc, recovery and paced audio. Evidence:
  `output/conditioning-math-2026-10-22/validation.json`; no image/VM deployment.
- [x] Implement native `COND`/`MATCOND`, reciprocal 2-norm `RCOND`, relative
  numerical `MATRANK` and values-only `SVDVALS`. Use a shared typed Rust analysis
  interface, scale-aware estimates, checked work/cancellation and explicit
  singular/overflow behavior. Pass 93 independent NumPy/analytical checks;
  do not import the archived absolute-threshold condition-number algorithm.
- [x] Qualify the structured-math checkpoint: 1,772 native tests, 452 Python
  tests, all 50 math-reference suites (187,012 checks), strict Clippy/rustdoc,
  recovery and paced native/mixed audio. Evidence:
  `output/structured-math-2026-10-22/validation.json`. No VM/image deployment.
- [x] Adapt archived Kronecker-product construction as native `KRON`/`MATKRON`
  and add `KRONSUM`, with typed Rust reuse, BASIC composition, checked allocation,
  cancellation, domain errors and 42 independent structured-operator checks.
- [x] Implement native `TGATHER` with arbitrary-axis/nested indices and repeated
  index gradients; pass Rust, BASIC dispatch and independent numerical checks.
- [x] Implement native max/average 1D/2D pooling, gradients, padding/tie rules and
  work limits; pass all Rust targets, strict Clippy and rustdoc checks.
- [x] Complete local exact-executable acceptance for current gathering, pooling
  and DSP adaptations: 49 reference suites, 1,767 native tests, 452 Python tests,
  no skips/ignored tests, strict Clippy/rustdoc and paced audio/recovery gates.
  Evidence: `output/archived-lessons-and-native-backend-2026-10-22/validation.json`.
  Debug executable qualification does not deploy these features into the user VM.
- [x] Complete the supported bounded exact RREF scope, alongside numerical
  column/row/null-space bases and reusable LU factors/permutations. Preserve the
  numerical/exact distinction and avoid rejected absolute-threshold donor code.
- [x] Provide explicit shape-aware transpose/products and native zero-axis
  matrix interchange, including empty-subspace projectors and coefficient
  reconstruction. See the qualified checkpoint below.
- [x] Extend explicit zero-axis shape metadata through square `DET`, `INVERSE`
  and `TRACE` calls, with empty-identity semantics and BASIC-dispatch tests.
- [ ] Defer zero-axis tensor interchange until core CPU math and domain workloads
  need it. If implemented, keep it CPU-side, preserve explicit shapes, and document
  each mathematical domain; plain nested lists cannot store a zero-row width.
- [ ] Address remaining scope deliberately: broader application checkpointing and higher derivatives beyond nine (analytic CPU tensor HVP, dense Hessian and bounded directional third through ninth derivatives are implemented); broader constrained optimization; nonlinear events/Jacobians and larger stiff workloads,
  events/boundaries; reliable sample-buffer DSP and measured acceleration.
- [x] Add allocation-free Rust `process_sample`, `process_into` and
  `process_in_place` methods for IIR and cascaded SOS filters, with preallocated
  delay scratch and per-sample atomic state commits. An allocator test observed
  zero allocations over 48,000 samples; release measurements found throughput
  parity, so acceleration and device callback gates remain open. Evidence:
  `output/native-signal-zeroalloc-cpu-2026-10-24/validation.json` and
  `benchmark.json`.
- [ ] Validate real scientific, engineering, analytical and ML workloads through
  BASIC calls to Rust, including standalone packaged applications. Compare with
  independent references and record residuals, tolerances, time and memory.
- [x] Add `programs/science/ida_hybrid_variational.bas` as a runnable, editable
  BASIC science workload for Rust `IDAHYBRIDVAR`. Check a parameterized index-one
  DAE against closed-form event time, event-time derivatives, terminal state,
  trajectory sensitivity and reset state using SPGMR. Interpreter execution
  passes all checks and reports 143 solver steps. This validates one model, not
  the broader cross-domain or standalone-app workload gate. Evidence:
  `output/ida-hybrid-variational-basic-2026-10-25/validation.json`.
- [x] Extend `programs/examples/math_workload.bas` with analytic Hann/Kaiser
  windows, convolution and correlation checks. Verify identical interpreter and
  release-compiled standalone output; measure the standalone executable at 0.04 s
  and 9,852 KiB peak RSS. This qualifies one composed workload, not the broader
  scientific/engineering/ML or scalability gate. Evidence:
  `output/standalone-math-signal-2026-10-25/validation.json`.
- [x] Extend engineering qualification with the thermostat hysteresis DAE in
  both the interpreter and a release-compiled standalone Linux app. Check six
  analytic event times, algebraic constraints before/after resets, exact output
  parity, executable identity, run time and peak RSS. This is one engineering
  workload and does not close the broader scientific/engineering/analysis/ML
  suite or scale qualification. Evidence:
  `output/standalone-thermal-hysteresis-2026-10-25/validation.json`.
- [x] Qualify the periodic sparse advection–diffusion workload in the
  interpreter and a release-compiled standalone Linux app. Independently verify
  every saved Fourier semidiscrete reference/error row and mass invariant, plus
  second-order fixed-step convergence, adaptive diagnostics, and interpreter /
  standalone CSV parity. Record executable identity, run time and peak RSS.
  Evidence: `output/standalone-advection-diffusion-2026-10-25/validation.json`.
  This is one PDE workload, not broad scientific or scalability acceptance.
- [x] Qualify the stiff Robertson chemical-kinetics BASIC application in the
  interpreter and a release-compiled standalone Linux app. Verify exact output
  parity, its positivity and total-concentration checks at seven output times,
  and record executable identity, qualification time and peak RSS. Independent
  SciPy Radau/BDF reference checks remain in the CVODE campaign. Evidence:
  `output/standalone-robertson-chemistry-2026-10-25/validation.json`.
  This is one stiff chemistry workload, not broad scientific or scalability
  acceptance.
- [x] Add a 4,096-state CPU sparse periodic-diffusion workload through BASIC
  `CVODESOLVE` and the Rust BDF/SPGMR backend. Independently verify every state
  at three times against the exact semidiscrete Fourier mode, check conservation,
  compare interpreter and standalone CSV byte for byte, and record executable
  identity, qualification time and peak RSS. Evidence:
  `output/standalone-periodic-diffusion-4096-2026-10-25/validation.json`.
  This large sparse case advances scalability evidence but does not close broader
  scientific or cross-platform qualification.
- [x] Qualify the representative mixed `math_workload.bas` through both the
  BASIC interpreter and a release-compiled standalone Linux application. The
  workload covers third derivatives, finance, regression, signal transforms,
  physics, exact/symbolic math and CPU learning; exact executable, test and
  workload hashes and parity are retained in
  `output/tensor-seventh-derivative-2026-10-24/validation.json`.
  This is one integrated qualification, not closure of the broader workload,
  scalability, memory or hardware gates.
- [x] Qualify the editable ten-orbit Kepler BASIC physics workload through the
  interpreter and standalone Linux app. Compare all 1,001 saved trajectory
  samples against the analytic circular orbit and check energy/angular-momentum
  drift; outputs and CSV match byte for byte. Maximum position/velocity errors
  were `2.077e-4`/`2.067e-4`, energy drift `1.219e-11`, and angular-momentum
  drift `1.243e-14`. Evidence:
  `output/standalone-kepler-orbit-2026-10-24/run-580743-1792882319305627043/validation.json`.
  This adds one independently referenced physics application, not broad
  scientific/engineering/ML qualification or performance/memory acceptance.
- [x] Run the updated mixed workload in the release interpreter for at least
  180 seconds and 50 cycles, checking cancellation/error recovery, tensor
  cleanup, stable file-descriptor count and no more than 16 MiB post-warm-up RSS
  growth. The run completed 4,760 cycles and 4,760,000 training steps with
  2,064 KiB peak growth. Evidence:
  `output/mixed-math-soak-2026-10-24/validation.json`.
- [x] Compile the editable `programs/ai/cpu_neural_network.bas` workload into a
  standalone Linux application and run it end-to-end. Verify CPU tensor kernels,
  500-epoch Adam training, XOR classification, BASIC file-based weight save/restore,
  graph cleanup and successful packaged-runtime execution. Evidence:
  `output/standalone-ai-neural-network-cpu-2026-10-24/validation.json`.


### 3. Recover useful archived Rust code

- [x] Compare the former math-engine directory with the active engine: 71 former
  Rust files are present; 69 matched, with tensor changes in the other two.
- [x] Inspect the dedicated older math dump; identify useful DSP candidates and
  reject placeholder QR/SVD replacements for the working current implementations.
- [x] Inventory and deduplicate all BASIC-related Rust archives: 113 snapshots,
  55,716 file sections, 3,592 distinct Rust bodies; retain paths and hashes in
  `output/archived-lessons-and-native-backend-2026-10-22/legacy-rust-inventory-verified.json`.
- [x] Semantically review six archived linear-algebra donors. Adapt the useful
  Kronecker formula; reject unsafe condition/rank/null/column/LU implementations
  for direct import. Preserve findings in `docs/ARCHIVED_RUST_REVIEW.md` and
  `output/structured-math-2026-10-22/donor-review.json`; broader review stays open.
- [ ] Complete semantic mapping and review of those candidates against the current
  backend; identifier matches alone do not prove equivalent capabilities.
- [ ] Review useful math, graphics, audio, fonts, application and runtime code;
  adapt missing capabilities rather than importing an incompatible old runtime.
- [x] Adapt native Kaiser/Blackman-Harris windows, causal FIR and low/high/band-pass
  filters from the older math subsystem. Pass 114 independent SciPy checks,
  native regressions and exact-executable qualification; retain explicit domains,
  cancellation/work bounds and error handling.
- [ ] For every adopted capability, add BASIC APIs, documentation, numerical or
  behavioral regression tests, cancellation/resource checks and source provenance.
- [ ] Preserve original archives; never overwrite working current algorithms
  with older placeholders, silent fallbacks or weaker unchecked implementations.

Donor locations supplied by the owner:

- Former engine: `/media/jason/1ae7c45c-4d30-4b65-8d60-c6986605ac67/jason/projects/modernbasic/src/math_engine`
- Older archives: `/media/jason/1ae7c45c-4d30-4b65-8d60-c6986605ac67/jason/projects/project_archive/oldcode`

### 4. Examine and improve the older Rust synthesizer

- [x] Review the 903-line March 2 archived `spectrum_analyzer.rs`; the March 1
  snapshot has no section with that name. Record uncalibrated levels, fragile
  pitch assumptions, input/capture/serialization risks and adopt only the useful
  sample-buffer analysis concept through the current Rust FFT.
  See `docs/ARCHIVED_RUST_REVIEW.md` and the periodogram checkpoint above.
- [x] Compare both archived modulation-matrix modules and hybrid filter sections.
  Reject the cutoff-formula mismatch, unchecked routing/rate state and silent
  capacity behavior. Share the current synth's stronger filter recurrence with
  Rust `NativeStateVariableFilter` and BASIC `SVFILTER`; validate modulated
  controls, continuation, failures, cancellation and allocation-free native
  processing. Qualify independent numerical references, existing synth PCM and
  an interpreted/standalone BASIC workload. Evidence:
  `output/shared-svf-2026-10-25/validation.json`.
  Complete hybrid/scheduler/backend review and physical listening remain open;
  the profile/schema review is recorded in the following checkpoint.
- [x] Complete semantic review of both archived acoustic-profile modules and
  the March 2 instrument schema; confirm the March 1 schema is absent. Check the
  actual hybrid partial consumer before interpreting profile values. Record the
  per-sample decay and inharmonicity mismatches, malformed/unbounded parsing,
  forced gain, incomplete route validation, unsafe save/roundtrip behavior and
  inconsistent archived range assertions. Retain exact source hashes, line
  anchors and arithmetic witnesses. Evidence:
  output/archived-instrument-profiles-2026-10-28/donor-review.json;
  permanent decisions: docs/ARCHIVED_RUST_REVIEW.md. No donor implementation was
  copied or run, and no new profile feature is claimed.
- [x] Review both archived engine scheduling/output paths, the five capture/backend
  modules and sound trait, plus synth lifetime/allocation/scheduler contracts.
  Evidence: output/archived-audio-delivery-2026-10-28/donor-review.json records
  16 section identities, 28 source anchors, 12 decisions and arithmetic witnesses.
  Reject callback file I/O/unbounded draining, arbitrary score timeouts,
  unstable equal-time ordering, malformed-note panics, stale render budgets,
  unlinked stereo limiting, nonterminating device paths and incomplete capture.
  Preserve useful single ownership and explicit sample clocks through the
  current shared renderer and bounded process path. No donor code was run or
  imported; full hybrid/oscillator review and hardware listening remain open.
- [x] Review archived hybrid signal/control paths in both sound snapshots.
  Record additive morph/envelope disconnects, negative chorus-delay interpolation,
  modal/pitch/rate contracts, frozen FX tails, global part mutation, headroom
  errors and limits of archived stress assertions. Retain two archive/section
  identities, 44 matched source anchors, 11 decisions and arithmetic witnesses.
  Evidence: output/archived-hybrid-controls-2026-10-28/donor-review.json;
  permanent decisions: docs/ARCHIVED_RUST_REVIEW.md. No old code was executed
  or imported. Remaining standalone synth internals, implementation of missing
  validated models and hardware/listening acceptance remain open.
- [x] Fix native score and temporary-patch error recovery. A trapped
  SOUND .5,1,10,.9 previously left the temporary volume behind, making the next
  note 3.6 times louder than its configured .25 baseline. Tone/sweep overrides
  now restore the complete voice on errors, and sequence/tone/sweep failures
  roll back appended samples, deferred events, cursors and glide history using
  bounded metadata rather than copying existing audio. All 66 synth unit tests,
  four new native/BASIC integration tests and four existing SOUND tests pass,
  including eight new regressions covering both rendering modes, failed patch
  selection, score capacity, cancellation and subsequent successful output.
  Actual BASIC ON ERROR recovery now exports the exact baseline WAV bytes.
  Formatting and portal consistency pass. Evidence:
  output/synth-command-atomicity-2026-10-28/tests-status.json.
- [x] Deploy the verified audio rollback fix to the native installed application.
  Installed release 898913fd1a15634588abbb341d67afcf51d728e9de83a426d530b60e30587ade
  passes the actual BASIC error-recovery WAV comparison with baseline.
  Evidence: output/native-hub-usability-2026-10-29/installation-qualification.json.
  Distro deployment remains deferred with OS-image qualification; the preserved
  old distro candidate predates this fix. No distro completion is claimed.

- [ ] Qualify audio timing and device contracts across the full BASIC/Rust path:
  explicit sample timestamps and same-time control ordering, bounded preparation
  outside rendering, rate/format/channel negotiation, linked stereo dynamics,
  long scores/rests, start/stop/reconnect and failures under load. Future capture
  must be explicitly requested by BASIC, bounded, cancellable and resource-safe.
  Reuse current rendering/math and pipe lifecycle; reject unsupported controls
  explicitly instead of silently selecting another instrument or doing nothing.

- [ ] Implement bounded, validated acoustic-profile/preset data and file APIs in
  Rust with BASIC consumers, shared FFT/periodogram/filter reuse, correct
  sample-rate-aware decay and pitch contracts, explicit route/index limits,
  atomic model/file changes and editable user data. Preserve SYNTHPROFILE's
  existing voice/oscillator-capacity meaning. Qualify numerical references,
  malformed inputs, cancellation/resources, live/export and physical listening.
- [x] Review standalone synth oscillator/envelope/trigger and mono/stereo
  chorus paths in both March snapshots. Preserve 42 matched source locations,
  arithmetic witnesses and nine permanent decisions covering trigger control
  loss, sync ordering, zero-gain semantics, timing, rate-dependent delays,
  stereo bypass, gain bounds, aliasing/noise and retrigger continuity.
  Evidence: output/archived-standalone-synth-2026-10-28; permanent lessons:
  docs/ARCHIVED_RUST_REVIEW.md. No donor code was imported. Capability
  implementation and listening/device gates remain open.
- [ ] Review both archived `sound_code` snapshots and BASIC-related synth modules,
  including hybrid synthesis, acoustic profiles, modulation matrix, instrument
  schema, spectrum analyzer and audio backends. Candidate details and initial
  decisions are in `output/archived-lessons-and-native-backend-2026-10-22/donor-review.json`.
- [ ] Compare those capabilities with the current native/sampled engines and
  adapt useful missing functionality without restoring duplicate sound paths.
- [ ] Expose useful synthesis, instrument and analysis controls through BASIC.
  Preserve real-time playback and immediate startup without pre-rendering a WAV.
- [x] Check BASIC-to-WAV mute and stereo control behavior on the verified
  runtime. Thirty-six native cases cover six waveforms and exact zero gains/
  channel isolation. Fourteen sampled piano/violin cases cover gain, expression,
  pan and disabled layers. Two disabled-layer cases correctly have no score to
  export. The violin's opposite-channel endpoint signal is isolated to its bank
  pan generator: removing pan eliminates it; removing reverb leaves identical
  bytes. Preserve original-bank identity, exact test mutations and WAV hashes;
  temporary bank copies and passing WAVs are retired. Document additive bank
  panning. Evidence: output/native-audio-control-contracts-2026-10-28 and
  output/sampled-audio-control-contracts-2026-10-28. This is export/control
  qualification only; tonal quality, live latency and hardware remain open.
- [ ] Verify SoundFont loading/selection, correct instrument tone, per-voice
  dynamics, balanced levels, adequate clean output and live/export consistency.
- [ ] Reproduce and eliminate startup clicks, crackling, stuttering and playback
  hiccups under realistic multi-voice load; retain cancellation/error recovery.
- [ ] Preserve technical captures and obtain human listening/hardware evidence;
  automated PCM checks alone do not certify speaker quality or instrument balance.
- [x] Bound Linux live-player pipe stalls and end-of-stream waits; terminate and
  reap the player after producer errors or unwinding panics, preserve stop and
  replacement isolation, and retain errors for the task. Writes allow five
  seconds without progress; final draining allows the remaining accepted PCM
  duration plus five seconds. Verify 82 focused Rust tests, three appliance
  wrapper tests, exact live/export PCM agreement in native, SoundFont and mixed
  modes (two concurrent workers each), and actual BASIC recovery from blocked
  and EOF-stalled players in about five seconds. The optimized executable is
  11,065,872 bytes, excluding providers/assets/OS; no dependency was added.
  This closes these process/pipe hangs, not negotiated-rate DSP, physical
  latency, hardware listening or the broader audio acceptance gates.
  Evidence: `output/audio-stall-recovery-2026-10-25/validation.json`.
- [x] Make the Rust speech backend discover eSpeak NG/eSpeak from standard Linux
  paths and then PATH; phoneme translation and retained-score WAV export pass when
  supplied by the local build-tools path.

### 5. Integrated creative and application environment

- [ ] Integrate native math with graphics, image processing, audio, animation,
  visualization, simulation and general software development through BASIC APIs.
- [ ] Improve native painting/image quality and animation, including believable
  flowing/sparkling water and scene composition; distinguish BASIC rendering from
  externally generated art and validate actual output and performance.
- [ ] Keep the command interface and controls visible on small displays; use
  windowed VM operation with reliable focus, escape, task switching and shutdown.
- [ ] Preserve interactive source editing, errors that return control, recovery
  of unsaved work and predictable program/window/process lifetimes.
- [x] Inspect the supplied font maker: it is Python/FontForge, not Rust, including
  its ZIP. Owner chose to keep it unchanged and focus on the math backend.
- [ ] Keep true native BASIC statement compilation as future work; retain honest
  descriptions of the existing runtime/executable packager and code-protection
  limits. Preserve the owner's licensing/copyright decisions as separate work.
- [x] Keep generated COMPILE builds offline against the locally bundled Rust
  crates, and keep the generated-app solver integration test inside the home
  workspace. Verify a standalone generated BASIC app can call the native LP and
  convex-QP workers and returns their known results.
  Evidence: `output/compiled-solver-workspace-2026-10-24/validation.json`.

### 6. Make historical lessons permanent and finish OS maturity

- [x] Extend exact-executable acceptance to native math references, draft recovery,
  native/mixed paced audio, immutable artifact/checker hashes and fresh evidence.
- [x] Bound acceptance output and execution time, including inherited pipes;
  add cancellation propagation and regression coverage for nested math checkers.
- [x] Preserve full-source cache identities, updater environment/space guards and
  exact-image/kernel checks already implemented from previous failures.
- [x] Rerun the expanded gates for this native backend checkpoint; preserve the
  earlier catalog-path failure, headless-audio test failure and invalid initial
  donor index, and link the accepted results above. Requalify future changes.
- [x] Run the full offline Rust all-target suite against the current source and
  locally extracted providers: all 2,011 tests pass with no failures, ignores or
  filters across 71 test-target summaries. PATH-based eSpeak discovery passes
  phoneme and retained-score WAV export tests. Strict all-target Clippy and
  Rustdoc pass with warnings denied. The Python project regression suites report
  472 pass and one ISO extraction test skipped because `xorriso` is unavailable.
  Regenerate and verify the derived portal manual after language-reference drift.
  Evidence: `output/4ti2-evaluation-2026-10-23/current-source-regression-v1.json`,
  `full-source-suite-v6.log`, `python-project-suite-v2.log`,
  `clippy-current-source-v1.log`, and `rustdoc-current-source-v1.log`.
- [ ] Continue security, stability and efficiency work: application isolation,
  trusted administration, storage quotas, bounded helpers, recovery and updates.
- [x] Apply serial source-test batches to the Debian build guest, preserving
  lib/bin/integration coverage, explicit architecture/source identity, compact
  logs, verified cache reuse and exact passed-test executable retirement.
  Keep failed/in-use executables and ordinary runtime/compile-only outputs.
  Also fix updater integrity for newly packaged `.cargo` source configuration,
  retaining legacy-package support and rejection of tampered/missing/link inputs.
  Evidence: `output/debian-test-batches-2026-10-28/validation.json`. Host/native
  and guest-command qualification do not replace a rebuilt-image test.
- [x] Permit release freezing under the home workspace's excluded `work/`
  subtree, while retaining rejection of overlapping inputs, symlink aliases,
  existing snapshots and future inventory conflicts. Align active build docs
  with consolidated revision 1/epoch 1 and the reusable workstation builder.
  Evidence: `output/home-release-freeze-2026-10-28/validation.json`.
- [x] Correct the reproduced build-guest OOM: retain the 2,560 MiB RAM limit
  and add one validated, reusable 1 GiB swap file inside the builder disk.
  Stream Rust batch progress, preserve pipeline failures, and detect kernel
  OOM/panic even when the build shell cannot print its status. Request clean
  shutdown with time for the seed's service-stop grace period.
  Qualification: 85 infrastructure tests, source checks, and actual KVM guest
  swap creation/reuse and clean shutdown. Evidence:
  `output/builder-memory-2026-10-28/validation.json`. The interrupted full Rust
  guest suite completed no targets; image qualification remains open.
- [x] Implement signed updater-migration bundle creation and read-only preflight.
  Authenticate immutable manifest/helper bytes with Ed25519, require an independent
  trust key, bind the exact legacy helper/distribution/runtime baseline, reject
  stale or malformed metadata and active package transactions, and never execute
  the downloaded helper during inspection. Eighteen signature/baseline tests pass.
  The actual retained older ISO also passes inspection of 488 selected files;
  tampering with its real updater is rejected. Test-only signing is not production
  key provisioning. Evidence: output/updater-migration-preflight-2026-10-28.
- [x] Implement locked, durable application/recovery for authenticated updater
  migration. Revalidate current trust and baseline under updater/APT/dpkg locks;
  preserve original bytes, ownership, permissions and distribution identity;
  atomically replace and sync files; block old and new ModernBASIC updater actions
  through incomplete replacement. Qualify process-exit recovery, directory-sync
  failure, key expiry/removal and compatible helper rollback. Evidence:
  output/updater-migration-transaction-2026-10-28. The actual older ISO extraction
  passes interrupted replacement/recovery and exact rollback of 488 original files.
  The CLI refuses Mint and live media. Booted installed-system migration and
  runtime forward/backward transitions remain separate open gates.
- [x] Extend installed lifecycle qualification for the real legacy updater path:
  independently authenticate and snapshot migration and runtime authorizations,
  require a distinct candidate package, provision explicit offline Linux library
  prerequisites, apply the production migration CLI, verify reboot persistence,
  perform signed runtime update/rollback, restore the original helper and verify
  another reboot. Preserve programs/settings and require exact result markers.
  Evidence: output/installed-migration-harness-2026-10-28. All 115 related tests
  pass, including the actual older helper fence check; orchestration tests use
  controlled VM fixtures and do not claim a booted-system pass. The retained old
  ISO lacks nine required solver libraries, whose pinned cached inputs are
  available. Run the full installed gate after the current OS build completes.
- [ ] Qualify migration from the previously installed updater: old deployed
  versions omit `.cargo` from the accepted source inventory, so the corrected
  updater in a fresh image alone does not prove authenticated upgrades from
  that baseline. Test the transition and rollback before claiming compatibility.
  The exact retained older package has now passed source/binary provenance
  checks with both its deployed updater and the corrected updater. Both reject
  altered Cargo source; the corrected helper also rejects undeclared Cargo
  configuration. Evidence: output/resumed-workstation-2026-10-28/legacy-package/report.json.
  Backward-compatible package inspection and isolated authenticated helper
  migration/recovery now pass. A booted installed transition, runtime update and
  rollback, followed by compatible helper rollback, remain open.
- [ ] Finish refreshed OS images and architecture-specific acceptance after the
  native backend is qualified. Include install/save/reboot/shutdown/update/rollback.
- [ ] Complete long interactive/audio testing, physical hardware and ARM64/RISC-V/
  Pi 5 qualification, production signing/provisioning, licensing and publication
  requirements recorded in the release work below. Do not claim these are done.

### 7. Workspace, backups and preservation

- [x] Record the owner's authoritative-source workflow: the original source
  directory on the backup drive is the main source. Develop and test in home,
  then copy verified source changes back, verify hashes and preserve prior
  history. Every successful code checkpoint must leave the original current;
  builds, caches, VM disks and generated outputs stay in home.

- [x] Use `/home/jason/modernbasic` as the active workspace for all code changes,
  builds, outputs and VM storage; update desktop/application launchers.
- [x] Move verified generated work off the backup drive, retain source/history
  archives and reclaim approximately 24.5 GiB. Migration receipts are in
  `output/workspace-migration-2026-10-22/completed.json`.
- [x] Checkpoint this pass's modified Rust source, tests, documentation and task
  list on the protected source backup, retaining prior versions and verifying
  hashes. Copy no builds. Receipt: `output/archived-lessons-and-native-backend-2026-10-22/source-backup.json`.
- [x] Preserve this structured-math pass's source/docs on the protected backup,
  keeping verified previous source copies; receipt:
  `output/structured-math-2026-10-22/source-backup.json`.
- [x] Promote tested conditioning/rank/SVD source to the authoritative original,
  preserving prior source and verifying every copied hash; receipt:
  `output/conditioning-math-2026-10-22/source-backup.json`.
- [x] Promote the tested LU backend, bindings, references and docs into the
  authoritative original source, preserve prior copies and verify copied hashes.
  Receipt: `output/lu-math-2026-10-22/source-backup.json`.
- [x] Promote tested subspace/shared-rank/full-right-SVD source, tests and docs
  to the authoritative original, preserving prior copies and verifying hashes.
  Receipt: `output/subspace-math-2026-10-23/source-backup.json`.
- [x] Promote tested exact RREF/shared rational source, tests and docs into the
  authoritative original, preserving prior copies and verifying hashes.
  Receipt: `output/rref-math-2026-10-23/source-backup.json`.
- [x] Promote tested attention-mask/softmax-gradient source, tests and docs to
  the authoritative original, preserving prior versions and verifying hashes.
  Receipt: `output/attention-math-2026-10-23/source-backup.json`.
- [ ] Repeat source-only checkpoints as further native backend work proceeds.
- [ ] Keep build/cache growth controlled and cleanup evidence-backed. Never
  delete the protected source project or original historical donor archives.
- [x] Make routine dev/test builds compact by default (`debug = 0`,
  `incremental = false`); keep explicit debugging/incremental opt-ins. Remove
  337 older, fingerprint-confirmed Cargo integration-test executables after
  checking every hash, keeping 71 newest binaries and verified test-source
  recovery copies. Reclaim 35,369,644,032 allocated bytes (32.94 GiB); retain
  per-file cleanup evidence. The current dev runtime is 48,969,584 bytes versus
  the prior 176,260,520-byte debug build. The existing release executable is
  10,168,848 bytes; this is not a measurement of the complete installed OS.
  Evidence: `output/rust-quality-gate-2026-10-25/cleanup-receipt.json` and
  `output/rust-quality-gate-2026-10-25/validation.json`.

- [x] Remove 26,117 obsolete Rust incremental-cache files from the two explicitly
  inspected home-workspace cache directories, with Cargo locks held and per-file
  hashes/inode metadata recorded. Reclaim 9,012,756,480 allocated bytes (8.39 GiB).
  Verify 1,491 source/runtime files unchanged and run the release BASIC executable
  afterward. Keep logs, source history, user programs, VM disks and release images.
  The release executable remains 11,065,872 bytes. The remaining development
  workspace at that checkpoint was about 259 GB, dominated by retained build
  and VM/image artifacts; it was not the installed BASIC runtime footprint.
  The later space-recovery checkpoint below supersedes that total. Cache control remains
  open above. Evidence:
  `output/obsolete-incremental-cleanup-2026-10-25/cleanup.json` and
  `output/obsolete-incremental-cleanup-2026-10-25/cache-files.jsonl.gz`.


- [x] Reclaim approximately 170.3 GB of redundant development storage, reducing
  the home workspace from 258.95 GB to 88.63 GB. Retire obsolete Cargo outputs,
  the duplicate raw builder, disposable installation disks and the completed
  cross-provider build overlay; retain exported libraries, probes, source and
  reports. Replace five byte-identical media copies with links to canonical files.
  Compress the retained builder from 38.21 GB to 28.34 GB after a successful
  structural check and full logical-image comparison. Preserve the running user
  VM, its independent recovery copy, revision-1 safeguards and source history.
  Teach release preflight and the VM launcher to resume raw or qcow2 caches and
  reject duplicate cache formats; the Python suite passes 486 tests with one
  tool-availability skip (487 run). No Rust rebuild or new OS image was needed.
  Evidence: `output/space-recovery-2026-10-25/validation.json` and
  `output/space-recovery-2026-10-25/promotion.json`.

- [x] Complete a further storage cleanup after the extended CPU test run:
  retire 35 inactive Cargo cache trees and 90 reproducible integration-test
  executables, while preserving exported applications, generated source and
  logs. Remove two superseded candidate ISO/package pairs, a disposable signed
  update package and extracted staging files individually verified against the
  retained qualified package. Keep unmatched staging metadata, all frozen source
  archives, revision-1 safeguards, the builder and live user VM. Final measured
  sizes and per-file receipts: `output/storage-followup-2026-10-25/validation.json`.
- [x] Correct the numerical campaign harness to preserve saved BASIC line
  numbers and jump targets; add three boundary regressions. Correct a worker test
  readiness-file race using atomic publication; the production worker is
  unchanged. All 73 math reference suites pass (209,061 checks) for the current
  optimized executable, reusing 72 unchanged reports and rerunning the corrected
  campaign. Evidence: `output/cpu-release-qualification-2026-10-25/verified-math.json`
  and `output/storage-followup-2026-10-25/validation.json`.
- [x] Replace `verify.sh`'s eager all-target Rust test build with
  `tools/check-rust-batches.py`: select Cargo targets from metadata, run them
  serially, retain hashed logs and incomplete-run status, and retire each passed
  integration-test ELF. Keep current runtime binaries and reusable libraries.
  Reject source/path aliases, hardlinks and unexpected artifacts; keep files
  still in use and reject a passing receipt if test inputs changed. Eleven
  focused regressions cover cleanup boundaries, all target kinds, failure
  handling and changed-input rejection. The current qualification completed 81
  previously outstanding integration targets plus the non-compilation cases in
  three remaining groups. Evidence:
  `output/rust-batch-qualification-2026-10-25/validation.json`.
- [x] Correct native-packager cache reuse: COMPILE honors the host's shared
  Cargo target and defaults to one private cache per output directory. Preserve
  rebuildable generated source; lock source generation through executable export
  to prevent mixed programs under concurrent builds. Locks remain held by live
  compiler children after abrupt caller exit; cancellation and failed builds
  preserve the preceding export. Cover unsafe cache paths, linked inputs, lock
  interruption, simultaneous outputs and project edits. Validate both ordinary
  process launches and the installed systemd scope launch path.
- [x] Reuse optimized runtime machine code across generated applications, keeping
  release level 3, one code-generation unit and stripped symbols. Disable repeated
  cross-crate LTO for generated entry points; the main runtime's release profile
  is unchanged. Three warm new/edited exports take 6.52–6.67 seconds and an
  unchanged repeat 0.81 seconds; runtime cache hashes/timestamps are unchanged.
  All eleven deferred standalone workloads pass. The sampled scientific program
  retains byte-identical output/trajectory data; its size rises by 73,728 bytes
  and measured median execution changes from 0.49 to 0.53 seconds. Keep this
  measured compilation/runtime tradeoff explicit in `docs/PERFORMANCE.md`.
  Correct the benchmark to avoid a false cold-cache label and reject a failed
  rebuild that leaves an older executable in place. Evidence:
  `output/compiler-shared-cache-2026-10-25/validation.json`.
- [x] Complete the current CPU math resource soak: 4,551 mixed-workload cycles
  and 4,551,000 training steps over 180.02 seconds, with 2,136 KiB peak RSS growth
  after warm-up (16 MiB limit) and stable file-descriptor count. Warning-denied
  Rust documentation checks pass. Evidence:
  `output/rust-batch-qualification-2026-10-25/math-soak.json`.
- [x] Finish the eleven standalone-compilation cases after cache correction:
  rerun all 412 main integration, 28 HiGHS and 15 4ti2 cases successfully. The
  compiler checkpoint also covers 1,023 library cases, five new CLI cache cases,
  the existing two process-cancellation cases, strict all-target Clippy and
  formatting. Six performance-tool tests pass, including two new regressions.
  Retained earlier evidence plus the updated affected targets covers 2,461
  distinct Rust cases and 503 Python cases; this is combined evidence, not a new
  single all-target run against a rebuilt OS. Retire the passed test executables,
  old per-program caches and obsolete optimization-profile cache, retaining
  source and reports. Current CPU checks have not rebuilt or installed an OS
  image or completed physical-hardware acceptance. Evidence:
  `output/compiler-shared-cache-2026-10-25/validation.json`.

## Consolidation acceptance

- [x] Select the current master source with the accumulated language, math,
  graphics, streaming soundfont audio, recovery, update and security corrections.
- [x] Include the small-display installer launcher and real frame-bounds checks.
- [x] Build the consolidated revision 1 package and image from one frozen source.
- [x] Validate artifact identity, BIOS/UEFI boot, installed lifecycle, signed
  update/recovery, draft recovery, graphics/windows and audio continuity.
- [x] Preserve the active VM's user programs/settings in independent storage,
  switch its launcher to revision 1, and verify the transferred data.
- [x] Remove retired release media, source forks and dependent build snapshots
  after the replacement and user data are verified. Keep compact test evidence
  and unique user assets, not duplicate historic source/build trees.

## Maturity milestone — 2026-10-22

- [x] Complete the optional firewall application: dedicated UI account, package,
  real IPv4/IPv6 filtering, peer authorization, timed rollback, crash/reboot
  recovery and external-edit protection. Installed in the VM with policy off.
- [x] Add an explicit live BASIC `app` launcher and forward missing function
  keys to running programs. The firewall package carries its own tested runner.
- [x] Add a source/history backup tool that requires an external destination,
  verifies every archived content hash and preserves prior backups.
- [x] Finish and verify the full protected-project checkpoint on the separate
  home drive. Live VM disks and build caches are explicitly excluded.
- [x] Give root administrator-terminal children a fixed environment; verify
  poisoned-environment resistance and terminal lifecycle in a disposable VM.
  Development source only; evidence: `output/maturity-admin-environment-2026-10-22/`.
- [x] Add an opt-in PAM administrator console outside X11, with account/VT
  preflight, temporary Xorg restart prevention and desktop recovery. Staged by
  the development installer; current VM and accepted r1 images unchanged.
  Acceptance evidence: `output/maturity-admin-console-2026-10-22/`.
- [x] Qualify separate-account PAM login and password-required sudo in a
  disposable installed amd64 VM. Add the missing UTF-8 locale image default
  while preserving existing language policy; verify PAM warnings are removed.
  Evidence: `output/maturity-admin-sudo-2026-10-22/`. No current VM deployment.
- [x] Add an opt-in offline headless native runner with namespace/data isolation,
  fixed cgroup budgets and pidfd-based launcher-death cleanup. Qualify adversarial
  native probes and a real BASIC runtime executable in a disposable amd64 VM.
  Evidence: `output/maturity-isolation-2026-10-22/`. Existing trusted desktop
  launch routes and the current VM are unchanged.

- [x] Bind opt-in isolated application data IDs to explicitly registered executable
  SHA-256 identities and the fixed offline permission profile. Refuse unknown IDs,
  changed code and registration replacement before launching or creating data.
  Evidence: `output/maturity-app-identity-2026-10-22/`. This is owner provisioning;
  signed publishers, managed upgrades and host UID separation remain open.

- [x] Build a separate private amd64 security-integration candidate. Pass all
  415 Python tests, live/installed packaged-security gates, eight installed
  lifecycle phases and normal BIOS/UEFI first-run flows and shutdown. Correct
  missing locale/firewall extraction in the image auditor, with regression
  coverage. Preserve failed checks, accepted r1 and the actual user VM.
  Evidence: `output/maturity-security-candidate-2026-10-22/`. The frozen product
  source predates these validation-only fixes, which are retained separately.

- [x] Remove inherited caller settings from privileged updater child processes;
  reproduce and block Bash startup-hook execution and poisoned environment
  forwarding. All 46 updater/signature tests pass in Debian userspace.
  Development source only; rebuilt-image acceptance remains required.
  Evidence: `output/maturity-updater-environment-2026-10-22/`.

- [x] Extend updater preflight to recheck root byte/inode availability before
  stopping BASIC; retain prepared transactions for retry. Count real package
  entries before extraction and refuse legacy prepared metadata safely.
  All 51 updater/signature tests pass. Real ext4 inode and root disk pressure,
  retry/reinstall/rollback and audio-failure recovery passed on a
  disposable installed clone; see `output/maturity-os-completion-2026-10-22/`.
- [x] Run the full native amd64 Rust suite on a verified snapshot: 1,757 tests,
  55 binaries, no failures or ignored tests. This is userspace qualification,
  not physical hardware/audio acceptance.
- [x] Complete graphical UEFI installation, normal disk boot,
  GUI program save/run and post-shutdown persistence evidence. Retained OVMF
  variables now copy into each test workspace instead of losing boot entries.
- [x] Build and independently qualify the private 6e82 candidate containing
  the updater guards, corrected image auditor and acceptance harnesses. All seven
  qualification phases and exact-candidate graphical UEFI install/persistence passed;
  see `work/qualification-6e82/report.json` and
  `output/maturity-os-completion-2026-10-22/graphical-uefi-6e82/`.
  This does not qualify later source edits or authorize production deployment.

- [x] Bound privileged updater command capture to 16 MiB, stream stdin/stdout
  without pipe deadlocks and kill the process group on output overflow or
  timeout. Regression tests exercise flooding and descendant cleanup; all 426
  Python tests pass. Installed transaction acceptance is being rerun with this
  final helper before it enters the next private build.

- [x] Bound updater provenance/state/status reads, reject FIFO/link metadata
  without blocking and handle oversized/deeply malformed JSON cleanly. Add
  real child-process regressions alongside archive validation.
- [x] Separate build package caches by complete frozen-source identity, keeping
  same-revision prior packages intact. Force staged reinstallation when only
  full-source provenance changed. The first real collision and failed build
  are preserved; corrected pipeline retry is pending.
- [x] Make failed installed-lifecycle runs kill their entire process group even
  when a descendant ignores TERM. Preserve the reproducer and accept process
  disappearance atomically in the deadline regression.

- [x] Add full staged-base/security refresh using authenticated APT indexes,
  refusing failed metadata refresh and package removals. A fresh installed-clone
  audit found 26 available upgrades plus a new kernel; do not publish a refreshed
  candidate until those are integrated and independently checked.
- [x] Select the newest Debian-versioned kernel with its matching initramfs
  after base refresh, retain older kernel files and refuse missing/ambiguous or
  escaping image pairs. All 438 Python tests pass; real rebuilt-image boot and
  post-build repository freshness remain pending.

## Remaining product and release work

- [ ] Finish refreshed release acceptance beyond the private amd64 candidate:
  graphical installer, longer interactive/audio checks, versioned authenticated
  updates and remaining architecture images. Include the corrected image auditor
  and packaged-security gate in the next frozen build. Keep accepted revision 1
  and its source intact; do not deploy the same-version private candidate.

- [ ] Separate application identities/capabilities and replace shared-console
  root administration with an authenticated trusted display boundary before
  supporting hostile native applications. The optional tty2 prototype still
  needs physical secure-attention and hardware qualification before
  becoming the default. The headless isolation runner is a tested foundation;
  GUI/audio brokers, signed package identities and managed upgrades, storage/inode quotas,
  host identity separation and default application integration remain open.
- [ ] Establish production signing-key custody, trusted provisioning, rotation,
  revocation, hosting and monotonic release policy.
- [ ] Extend editor grapheme/bidirectional caret handling and debugger array/
  FUNCTION support, library source maps and complete language capability audit.
- [ ] Qualify graphical UEFI installation and longer interactive authoring,
  suspend/resume, scaling, keyboard layouts and human audio listening.
- [ ] Test physical x86 audio/network/graphics/USB/power and board-specific
  ARM64/RISC-V/Pi 5 boot, drivers and peripherals. Record exact devices/hashes.
- [ ] Owner selects licensing and confirms source/asset redistribution rights.
- [ ] Configure public HTTPS hosting, report endpoint/credentials, independent
  backups/monitoring and publish only independently accepted artifacts.
- [ ] Future: true native BASIC statement compilation. Keep the existing native
  executable packager, with no statement-level optimization or secrecy claims.

Consolidation records and the compact previous planning history are in
`releases/revision-1/evidence/`. A private VM pass is not physical certification,
public-release approval or a claim that remaining work is complete.

- [x] Correct staged live-tools initramfs generation with a temporary build marker, preserve externally owned markers, refuse shipping a leftover marker, and repair configured kernels missing their initramfs on retries. Regression tests cover marker cleanup and repair selection.
- [x] Qualify the rebuilt private 6e82 image with its matching updated kernel/initramfs and final artifact acceptance gates; scope is recorded above.

- [x] Rebuild and require nonempty kernel module dependency indexes before staged initramfs creation; refresh existing boot images as well as missing ones after base/security updates. Empty-index refusal has a cleanup regression. Rebuilt-image qualification remains pending.

- [x] Give only the unattended disposable build VM Debian live-tools’ supported `noeject` boot option so successful builds can power off without a physical media-removal prompt. The actual builder command is regression-tested for both seed paths.

## Archived lessons made permanent — 2026-10-22

- [x] Review the owner-supplied calculator OS, RPN, CAS, online-calculator,
  program-archive and Rust math-crate references. Record transferable ideas,
  hardware-specific boundaries, adoption criteria and the non-deterministic
  semantics of Rust algebraic float operations in `docs/CALCULATOR_REFERENCE_REVIEW.md`.
  No external implementation was copied or added as a runtime dependency.

- [x] Extend exact-artifact acceptance with Rust math-reference qualification,
  forced draft recovery, four paced native audio streams and two mixed SoundFont
  streams. Evidence uses fresh subdirectories and immutable artifact/checker hashes.
- [x] Bound acceptance output and process lifetime, including inherited-pipe
  descendants; exercise floods, invalid UTF-8, closed-pipe hangs and orphan writers.
- [x] Record the permanent release/OS/hardware distinction and retained regression
  gates in `docs/RELEASE_AUDIT.md`; reconcile completed 6e82 private acceptance above.
- [x] Record Linux/Rust/BASIC responsibilities and keep backend versions separate
  from freely editable user BASIC programs and bundled-content integrity hashes.
- [ ] Finish native backend capability stages in `docs/MATH_APPLICATIONS.md`.
  TGATHER, native pooling, attention masks, analytic tensor HVP and bounded dense
  Hessian materialization are implemented;
  see the current owner-priority checklist above for exact acceptance status.
  Higher derivative orders beyond nine, broader numerical optimization,
  nonlinear stiff/events workflows and release/hardware qualification remain open.


### CPU multivariable robust regression — 2026-10-24

- [x] Add bounded Rust Huber IRLS for multivariable linear regression, exposed
  through BASIC `HUBERREG` and `NativeRealStatistics`. Require an explicit
  response-unit threshold, normalized loss/scale reporting, rank and convergence
  status, cancellation and cumulative work limits; qualify outlier resistance,
  an independent Huber stationarity check, BASIC/Rust parity, invalid thresholds
  and iteration limits, no-intercept behavior, and maximum matrix dimensions.
  Evidence: `output/cpu-huber-regression-2026-10-24/validation.json`. No residual-
  scale inference or confidence intervals are claimed.

### CPU robust trend analysis — 2026-10-24

- [x] Add the bounded Rust Theil-Sen simple linear trend estimator and BASIC
  `THEILSEN`, using the median of pairwise slopes and median intercept, skipping
  repeated predictors and reporting a median absolute residual. Qualify an
  outlier-resistant editable BASIC example, extreme scales, invalid inputs and
  the one-million-pair work bound. It does not claim confidence intervals or
  replace multivariable least squares.

### CPU derivative-free bound optimization — 2026-10-24

- [x] Extend the checked NLopt adapter with local derivative-free BOBYQA and
  SBPLX for finite box-bounded objectives. Reject nonlinear constraint callbacks
  for algorithms that do not support them; verify BASIC assignment/error behavior,
  bounded optima, and document that these methods offer no global certificate.
  Existing COBYLA/SLSQP APIs and analytic-gradient rules remain unchanged.
- [x] Expose the bounded NLopt nonlinear optimizer as a documented crate-root
  Rust API with typed options, callback samples and feasibility reports. Verify
  an equality-constrained analytic-gradient solve and callback-error recovery
  through an external integration test; the API retains the existing dimension,
  time, evaluation and callback-work caps. Evidence:
  `output/native-nlopt-public-api-2026-10-24/validation.json`.

### Nonlinear implicit-step checkpoint — 2026-10-23

- [x] Implement bounded Rust nonlinear backward Euler as `IMPLICITSTEP`, sharing
  BASIC callback scope, work/cancellation checks and native linear solves.
- [x] Qualify the checkpoint: 1,800 native tests, 453 Python tests, 56 math
  reference suites and all ten artifact phases passed. Preserve previous source
  and synchronize verified files to the authoritative original.
  Evidence: `output/nonlinear-math-2026-10-23/validation.json`.
- [x] Extend the fixed-step solver to bounded adaptive nonlinear trajectories
  in the checkpoint below.
- [x] Add bounded terminal sign-crossing event localization in the checkpoint below.
- [x] Add supported dense analytic Jacobian workflows to every nonlinear implicit API.
- [x] Add bounded sparse nonlinear Jacobian workflows through shared GMRES.
  Fixed-step residual convergence does not establish time-integration accuracy
  or complete the physics engine.


### Adaptive nonlinear trajectory checkpoint — 2026-10-23

- [x] Implement Rust `IMPLICITSOLVE` with step-doubling error control, shared
  evaluation/cancellation budgets and selective Newton retries.
- [x] Pass 1,803 native tests, 453 Python tests, 57 math suites and all ten
  artifact phases; preserve source history and synchronize verified changes
  to the authoritative original. Evidence:
  `output/nonlinear-adaptive-2026-10-23/validation.json`.
- [x] Add bounded terminal event localization and a six-impact hybrid workload.
- [x] Add dense analytic Jacobians and a qualified stiff reaction workload.
- [x] Add bounded sparse nonlinear Jacobians and qualify larger roots/PDEs/events.
- [x] Add supported semi-explicit index-one DAE steps and trajectories below.
- [ ] Extend general DAE workflows and qualify further physical workloads/resource use.


### Terminal ODE event checkpoint — 2026-10-23

- [x] Implement Rust `ODEEVENT` and `IMPLICITEVENT`, sharing accepted-step
  solvers, budget/cancellation checks, direction filtering and bounded terminal
  root localization with documented interpolation/detection limits.
- [x] Qualify 1,807 native tests, 453 Python tests, 58 reference suites and all
  ten artifact phases, including 31 independent event/workload check groups.
  Preserve prior source and synchronize verified changes to the original.
  Evidence: `output/ode-events-2026-10-23/validation.json`.
- [x] Add CPU `ODEEVENTS` for up to 64 simultaneous nonterminal event functions,
  per-event direction filters and ordered event-state records while the adaptive
  RK4 trajectory continues to its endpoint. Preserve `ODEEVENT` and `IMPLICITEVENT`
  contracts, bound event results and test against independent SciPy DOP853.
- [x] Add opt-in CPU tangency localization to explicit, implicit-terminal and
  nonterminal event APIs. Require caller-supplied event-value tolerance, share
  solver callback/work budgets, respect direction filtering, and qualify smooth
  analytic contacts and near misses. The method assumes one relevant minimum
  per accepted step; it does not enumerate multiple hidden roots/extrema. Pass
  14 adaptive BASIC tests and 46 independent event checks; see
  `output/ode-event-tangencies-2026-10-24/validation.json`.
- [x] Add CPU `IMPLICITEVENTS` with 1–64 nonterminal event functions, optional
  analytic dense/CSR1 Jacobians, linear dense-output localization, per-event
  direction filtering, tangencies, shared solver budgets and bounded event rows.
  The 15-test adaptive suite and 53 independent event checks cover crossings,
  direction filtering, initial roots and tangencies; see
  `output/implicit-events-2026-10-24/validation.json`.
- [x] Add `eventscan` to terminal and nonterminal explicit/implicit ODE event
  APIs: a bounded 1–256 interval scan of each accepted dense segment detects
  multiple sign crossings within one step. Terminal solvers stop at the first
  bracketed root; vector solvers record each detected crossing. Tests cover two
  roots in one accepted step for both solvers, terminal first-root behavior,
  invalid bounds and interpolated states. The scan remains finite-work and can
  miss crossings or tangencies packed within one scan interval; tangent search
  assumes at most one relevant smooth minimum per scan interval. The 18-test
  adaptive suite and 75 independent checks passed. Evidence:
  `output/event-scan-2026-10-24/validation.json`.
- [x] Fix adaptive RK4 final-step interval rounding so accepted state evolution
  matches the representable timestamps returned in the trajectory; cover a
  large-time endpoint regression in the adaptive BASIC suite.
- [x] Implement, qualify and promote the CPU `ODEHYBRID`/`IMPLICITHYBRID` reset
  API. Twenty adaptive BASIC tests pass, including initial
  zero resets, zero-surface rearming, finite shape-preserving resets, implicit
  solver operation and event-cap failure. The revised six-impact sample passes
  analytical contact-time, restitution and energy checks; the independent
  event checker passes 75 checks. Evidence:
  `output/hybrid-events-2026-10-24/validation.json` and the hash-verified
  source promotion receipt in `source-backups/`.
- [x] Add optional `eventdepth` binary refinement to every explicit/implicit
  terminal, vector and hybrid event API. Scan density is bounded to 4096
  intervals per accepted step; default behavior retains zero extra refinement.
  Test explicit and implicit crossings only 0.005 time units apart, two packed
  tangencies, hybrid reset localization, and depth/product rejection. Evidence:
  `output/event-scan-depth-2026-10-24/validation.json`.
- [ ] Further expand isolation of roots/extrema packed within the finest scan
  interval and implement general DAE resets. Verified on 2026-10-24 with
  `cargo test --offline --all-targets --locked -- --test-threads=1`: all Rust
  targets passed with the locally pinned CPU providers configured, including
  SUNDIALS, HiGHS, NLopt, SuiteSparse and 4ti2. The 4ti2 runtime group passed
  15/15 after setting `MODERNBASIC_4TI2_MARKOV_EXECUTABLE` to the package's
  checksum-admitted `4ti2gmp` binary under `libexec`. The standalone-dispatch test
  now builds its generated executable under `output/test-scratch`, not `/tmp`; the
  previously failing Markov, standalone, editable-example and Hilbert/Graver tests
  pass with the corrected provider path. Evidence:
  `output/4ti2-workspace-build-2026-10-24/validation.json`. Finite scan intervals can
  still miss clustered roots/tangencies, and general residual DAE reset maps
  remain open. This CPU-only work adds no GPU code or dependency. Evidence:
  `output/cpu-all-targets-2026-10-24/validation.json`.


### Shape-aware matrix interchange checkpoint — 2026-10-23

- [x] Implement native Rust shape-preserving matrices and optional shape-aware
  BASIC `TRANSPOSE`/`MATMUL`, including empty subspace projectors and coefficient
  reconstruction, strict shape/data checks and allocation/work/cancel limits.
- [x] Pass 1,811 native tests, 453 Python tests, 59 reference suites and all
  ten artifact phases, including 79 independent shape-aware matrix groups.
  Preserve prior source and synchronize verified changes to the original.
  Evidence: `output/shaped-matrices-2026-10-23/validation.json`.
- [x] Extend zero-axis interchange to `DET`, `INVERSE` and `TRACE` with optional
  explicit shapes; qualify empty identity behavior and invalid-shape rejection.
- [x] Requalify the square-operation change across 2,008 Rust tests in 71 targets
  (five host/environment filters), 17 math-maturity tests, 13 portal tests,
  strict Clippy and Rustdoc. Evidence: `output/zero-axis-square-2026-10-24/validation.json`.
- [x] Extend explicit zero-axis shapes into reduced QR/SVD, symmetric EIGH,
  Cholesky, pseudoinverse and least-squares operations; define empty results
  without calling LAPACK on zero dimensions.
- [x] Requalify zero-axis operations across 2,010 Rust tests in 71 targets (five
  environment/packaging filters), all 18 math-maturity tests, 13 portal tests,
  strict Clippy/Rustdoc and the portal bundle check; fix the QSopt cancellation
  test's PID-file write race. Evidence:
  `output/zero-axis-decompositions-2026-10-24/validation.json`.
- [ ] Extend interchange to tensor paths deliberately; ordinary lists carry no
  hidden zero-row width.


### Native analytic nonlinear Jacobian checkpoint — 2026-10-23

- [x] Implement optional square matrix Jacobian callbacks for every nonlinear
  implicit BASIC API, using the shared Rust Newton/step/event algorithms.
- [x] Expose native Rust equation construction and the bounded nonlinear
  stepper for direct backend reuse, sharing work/cancellation safeguards.
- [x] Publish this CPU physics path at the crate root as
  `NativePhysicsEquation`/`NativePhysicsJacobian`, with RK4, implicit
  backward-Euler and index-one DAE consistency methods. Verify external-crate
  use without interpreter values against analytic ODE/DAE solutions and strict
  Rust documentation. Evidence:
  `output/native-physics-api-cpu-2026-10-24/validation.json`.
- [x] Pass native work/domain/recovery tests, 37 independent derivative/solver
  groups and a stiff reaction workload with 3 independent physics checks.
  Full qualification: 1,816 native tests, 453 Python tests, 60 math suites and
  all ten artifact phases; preserve prior source and synchronize verified
  changes to the original. Evidence:
  `output/analytic-jacobians-2026-10-23/validation.json`.
- [x] Add bounded sparse nonlinear Jacobian workflows in the following checkpoint.
- [x] Add supported semi-explicit index-one DAE workflows in the following checkpoint.
- [ ] Extend general DAE workflows and further cross-domain backend consumers;
  this checkpoint does not complete the engine.


### Sparse nonlinear Jacobian checkpoint — 2026-10-23

- [x] Reuse canonical CSR and residual-checked GMRES for sparse nonlinear Newton
  corrections in native Rust and every existing nonlinear implicit BASIC API.
- [x] Support up to 4,096 sparse states without dense conversion, preserving
  cumulative work, Krylov memory, callback and cancellation safeguards.
- [x] Pass 27 independent sparse root/PDE/event check groups and an additional
  empty-Jacobian/nonzero-forcing edge check; 1,820 native tests, 453 Python
  tests, 61 math suites and all ten artifact phases. Preserve prior source
  and synchronize verified changes to the authoritative original. Evidence:
  `output/sparse-jacobians-2026-10-23/validation.json`.
- [x] Add a qualified real-square sparse direct LU foundation, as recorded below.
- [x] Add CPU `SPFMSOLVE` so a reused UMFPACK factor solves several BASIC RHS
  columns in one call, preserving a true-residual report per column and shared
  input/work ceilings. Validate column ordering, residuals and malformed inputs.
  Evidence: `output/sparse-multirhs-cpu-2026-10-24/validation.json`.
- [x] Expose the same bounded UMFPACK factor as crate-root Rust
  `NativeSparseDirectFactor`, with typed single/multiple-RHS residual reports,
  so Rust engines can use the sparse backend without interpreter values. Correct
  the language reference's earlier claim that a private factor was directly
  callable. Evidence: `output/sparse-direct-native-api-cpu-2026-10-24/validation.json`.
- [x] Expose the existing bounded CPU GMRES kernel through `NativeCsrMatrix`,
  with default row equilibration and typed `ROW`/`NONE`/`ILU(0..8)` controls.
  Match BASIC restart, work, cancellation, factor and original-residual contracts;
  qualify general sparse solutions and invalid ILU requests. Evidence:
  `output/sparse-gmres-native-api-cpu-2026-10-24/validation.json`.
- [x] Expose the existing bounded CPU CG kernel through `NativeCsrMatrix`,
  with typed Jacobi and incomplete-Cholesky IC(0..8) controls and true residuals.
  Qualify known SPD systems and confirm indefinite curvature never reports
  convergence. Evidence:
  `output/sparse-cg-native-api-cpu-2026-10-24/validation.json`.
- [x] Add CPU sparse LSQR as `SPLSQR` and `NativeCsrMatrix::lsqr` for
  rectangular overdetermined, underdetermined and rank-deficient least-squares
  systems. Avoid normal-equation formation; report original fit and normal
  residuals separately; qualify minimum-norm behavior, inconsistency, zero axes,
  BASIC composition and bounded failures. Evidence:
  `output/sparse-lsqr-cpu-2026-10-24/validation.json`.
- [x] Add opt-in CPU LSQR column 2-norm equilibration through BASIC and the
  typed Rust API. Keep the default Euclidean minimum-norm behavior; document the
  changed metric for singular/underdetermined systems, use mode-specific and
  componentwise stationarity checks, and validate original fit residuals,
  coefficient underflow, BASIC composition and 24-order column imbalance. Evidence:
  `output/sparse-lsqr-column-equilibration-cpu-2026-10-24/validation.json`.
- [x] Add bounded CPU complex dense `CLINSOLVE` and the typed
  `NativeComplexMatrix::solve` API with partial row pivoting and a fresh complex
  residual. Qualify complex coefficients, zero-leading pivots, singular and
  shape errors, BASIC composition and atomic recovery. Other general complex factorizations, non-Hermitian eigenvalues and complex
  eigenvectors remain open. Evidence:
  `output/complex-linear-solve-cpu-2026-10-24/validation.json`.
- [x] Add CPU complex `CLSTSQ` and typed `NativeComplexMatrix::least_squares`
  for bounded full-column-rank overdetermined data; report original fit and
  normal residuals, with explicit rank/shape failure and common-scale checks.
  Evidence: `output/complex-least-squares-cpu-2026-10-24/validation.json`.
- [x] Add CPU complex Householder QR as `CLQR` and typed
  `NativeComplexMatrix::qr`, returning reduced Q/R for tall, square, wide and
  rank-deficient matrices. Scale reflector work and qualify reconstruction,
  orthogonality, extremes, BASIC composition and failure recovery. Evidence:
  `output/complex-qr-cpu-2026-10-24/validation.json`.
- [x] Add CPU complex singular values as `CSVDVALS` and
  `NativeComplexMatrix::singular_values`, using the CPU LAPACK realification
  path with multiplicity-preserving exact averaging. Qualify analytic,
  rectangular, rank-deficient, zero and scaled cases plus BASIC composition.
  Evidence: `output/complex-svdvals-cpu-2026-10-24/validation.json`.
- [x] Add CPU Hermitian complex eigenvalues as `CHEIGVALS` and
  `NativeComplexMatrix::hermitian_eigenvalues`, with relative Hermitian checks,
  ascending eigenvalues and multiplicity preservation through CPU LAPACK. Qualify
  analytic, repeated, zero and scaled spectra; reject invalid shapes/symmetry.
  Evidence: `output/complex-hermitian-eigenvalues-cpu-2026-10-24/validation.json`.
- [x] Add bounded CPU complex products and conjugate transpose as `CLMATMUL` and
  `CLADJOINT`, backed by `NativeComplexMatrix::multiply` and `adjoint`. Use exact
  binary product accumulation, cap axes at 128 and work at 4,194,304 real
  product terms, and check cancellation between output rows. Evidence:
  `output/complex-matrix-products-cpu-2026-10-24/validation.json`.
- [x] Add CPU `SPBICGSTAB` and typed `NativeCsrMatrix::bicgstab` for general
  nonsymmetric systems. Reuse bounded ROW/NONE/ILU(0..8) left preconditioners,
  retain O(n) iteration vectors and check each reported convergence against the
  original residual. Qualify pivotable zero-diagonal systems, ill-scaled rows,
  a 512-state sparse workload, BASIC composition and breakdown/recovery limits.
  Evidence: `output/sparse-bicgstab-cpu-2026-10-24/validation.json`.
- [x] Add bounded CPU sparse Kronecker assembly as `SPKRON` and
  `NativeCsrMatrix::kronecker`. Preserve canonical CSR order, enforce dimension,
  entry and work limits, reject overflow, and qualify rectangular/empty shapes
  plus composition with BASIC `SPCG`. Evidence:
  `output/sparse-kronecker-cpu-2026-10-24/validation.json`.
- [x] Add bounded CPU sparse Kronecker-sum assembly as `SPKRONSUM` and
  `NativeCsrMatrix::kronecker_sum` for square component matrices. Coalesce
  overlapping diagonal entries exactly, preserve canonical CSR order, enforce
  dimension/entry/work limits, and qualify the Laplacian product, empty and
  invalid shapes, BASIC `SPCG` composition and editable example. Evidence:
  `output/sparse-kronecker-sum-cpu-2026-10-24/validation.json`.
- [x] Add typed CPU `NativePolynomial` APIs shared with the BASIC polynomial
  functions for arithmetic, calculus, evaluation and scaled QR fitting. Verify
  native/BASIC parity, exact coefficient accumulation, conditioning failures
  and finite-resource/error behavior. Evidence:
  `output/native-polynomial-cpu-2026-10-24/validation.json`.
- [x] Add CPU `SPGMRES` preconditioner controls: retain row equilibration as the
  default, allow unscaled GMRES, and provide bounded-pattern ILU(0) with explicit
  zero-pivot failure. The residual is still checked against the original system.
  Evidence: `output/ilu0-gmres-2026-10-23/validation.json`.
- [x] Add positive incomplete-Cholesky IC(0..8) preconditioning to CPU `SPMINRES`
  in Rust and BASIC, using a symmetric triangular coordinate transform and true
  residual checks against the original matrix. Validate BASIC/native parity,
  one-iteration IC(0) convergence, common scales of 1E200 and 1E-200, and invalid
  factor inputs. Evidence: `output/sparse-minres-ic-cpu-2026-10-24/validation.json`.
- [ ] Add further sparse direct algorithms and broader preconditioners; general DAE
  workflows and larger cross-domain workloads remain open.


### Semi-explicit index-one DAE checkpoint — 2026-10-23

- [x] Implement Rust differential/algebraic residual rows, consistent-initial-data
  checks and dense/sparse Newton assembly, exposed as `DAESTEP`/`DAESOLVE`.
- [x] Pass 46 independent DAE block/constraint/trajectory check groups and four
  RC-circuit application checks; 1,827 native tests, 453 Python tests, 62 math
  suites and all ten artifact phases. Preserve prior source and synchronize
  verified changes to the original. Evidence:
  `output/dae-foundation-2026-10-23/validation.json`.
- [x] Add explicit consistent-state initialization for the supported index-one class.
- [ ] Extend general residual/higher-index DAE, constraint-preserving events and
  higher-order workflows. The current first-order index-one class does not
  complete general DAE support or the engine.


### Explicit DAE initialization checkpoint — 2026-10-23

- [x] Add Rust/native and BASIC `DAEINIT`, holding differential state fixed while
  solving algebraic states with current-value scales and shared Newton safeguards.
- [x] Pass native preservation/budget/recovery checks, 35 independent initialization
  groups and five initialized RC application checks; 1,832 native tests, 453
  Python tests, 63 math suites and all ten artifact phases. Preserve previous
  source and synchronize verified changes to the authoritative original.
  Evidence: `output/dae-initialization-2026-10-23/validation.json`.
- [ ] Extend general residual/higher-index initialization and index/branch
  qualification deliberately; local index-one initialization is not general DAE completion.


### Native tensor/Adam persistence checkpoint — 2026-10-23

- [x] Implement exact Rust leaf/Adam serialization and validated atomic restore
  with fresh handles, exposed as sandboxed TSAVE/TLOAD and native byte methods.
- [x] Cover supported engine state beyond string limits, preserving independent
  data-schema/backend/source versions and refusing executable graph payloads.
- [x] Pass full-capacity/recovery/sandbox tests and 16 independent binary/CRC
  and fresh-process training checks. Pass 1,842 native tests, 453 Python tests,
  64 math suites (188,152 checks), strict Clippy/Rustdoc and all ten artifact
  phases. Preserve prior source and hash-verify promotion to the original.
  Evidence: `output/tensor-checkpoints-2026-10-23/validation.json`.
- [x] Add bounded exact analytic `THVP(loss,parameter,direction)` and
  `THESSIAN(loss,parameter)` for the CPU tensor graph, including broadcast
  arithmetic, activations, softmax/cross-entropy, matrix products, convolutions
  and fixed-branch pooling. Twelve BASIC-level regression tests pass; see
  `output/tensor-hvp-2026-10-24/`.
- [x] Add exact analytic third directional derivatives as
  `TDERIV3(loss,parameter,direction)` across the CPU tensor graph using bounded
  Taylor jets. Verify polynomial, smooth activation/loss, matrix, convolution,
  pooling, gather and fixed-mask softmax paths against closed forms, plus domain
  validation and interruption recovery. The tape is limited to four million
  reachable tensor elements; piecewise branch behavior at ReLU and max-pool
  kinks is explicit. Evidence:
  `output/tensor-third-directional-2026-10-24/validation.json`.
- [x] Add exact analytic fourth directional derivatives as
  `TDERIV4(loss,parameter,direction)` through the shared CPU Taylor graph. Check
  closed-form polynomial, exponential, logarithmic, sigmoid, tanh, softmax and
  cross-entropy derivatives, plus matrix product, convolution, repeated gather
  and fixed-branch pooling composition. Preserve `TDERIV3` behavior when fourth
  coefficients overflow, validate domains/cancellation and bound the reachable
  tape to three million tensor elements. Evidence: 20 passing tensor derivative
  integration tests plus the internal cancellation test; editable example:
  `programs/examples/tensor_fourth_derivative.bas`.
- [x] Extend the shared Taylor graph with exact analytic
  `TDERIV5(loss,parameter,direction)`, bounded to two million reachable tensor
  elements. Check polynomial, exponential, logarithmic, reciprocal, sigmoid,
  tanh, cross-entropy and matrix-composition references, overflow behavior and
  the editable BASIC example. All 23 tensor HVP/derivative integration tests
  pass, and the exact optimized executable passes all 71 math-reference suites
  (206,951 checks). Evidence:
  `output/tensor-fifth-derivative-2026-10-24/validation.json`.
- [x] Extend the Taylor graph with exact analytic
  `TDERIV6(loss,parameter,direction)`, bounded to 1.5 million reachable tensor
  elements. Check sextic, exponential, logarithmic, reciprocal, sigmoid, tanh,
  cross-entropy and matrix-composition references; preserve lower-order overflow
  behavior; and run the editable BASIC example. All 26 derivative integration
  tests pass, eight release outputs match 80-digit `mpmath`, and all 71 math
  suites (206,951 checks) pass against the same release binary. Evidence:
  `output/tensor-sixth-derivative-2026-10-24/validation.json`.
- [x] Extend the Taylor graph with exact analytic
  `TDERIV7(loss,parameter,direction)`, bounded to one million reachable tensor
  elements. Qualify polynomial, exponential, logarithmic, reciprocal, sigmoid,
  tanh and matrix compositions, domains, overflow and the editable BASIC example;
  add it to the interpreter/standalone mixed math workload.
  Evidence: `output/tensor-seventh-derivative-2026-10-24/validation.json`.
- [x] Extend the shared CPU Taylor graph through exact analytic
  `TDERIV8(loss,parameter,direction)`, bounded to 750,000 reachable tensor
  elements. Verify eighth derivatives for polynomial, exponential, logarithmic,
  reciprocal, sigmoid, tanh, cross-entropy, matrix, convolution, pooling,
  repeated-gather and masked-softmax paths against independent references; cover
  invalid domains, overflow, typed Rust/BASIC parity and an
  editable BASIC example. Add the eighth-order case to the interpreted and
  standalone mixed math workload and verify exact application-output parity.
  Evidence: `output/tensor-eighth-derivative-2026-10-25/validation.json`.
  Orders beyond eight, whole-application checkpointing and broader workload gates
  remain open.
- [x] Extend the shared CPU Taylor graph through analytic `TDERIV9`, bounded to
  500,000 reachable tensor elements. Independently check polynomial, exponential,
  logarithmic, reciprocal, sigmoid, tanh and convolution paths; cover shape,
  overflow and work-cap failures; compare typed Rust and BASIC APIs; and include
  an editable BASIC example plus the interpreted/standalone mixed workload. The
  nine-order extension is a bounded increment, not unrestricted high-order AD.
  Evidence: `output/tensor-ninth-derivative-2026-10-25/validation.json`.
- [ ] Extend whole-application/tensor/scheduler checkpointing and derivative
  orders beyond nine; RNG stream snapshots and weight/optimizer persistence
  alone do not complete all AI/ML requirements.

### Stateful Rust sample-buffer DSP — 2026-10-23

- [x] Implement reusable normalized FIR/IIR and cascaded biquad kernels with
  explicit delay state and atomic block recovery, exposed as LFILTER/SOSFILTER.
- [x] Pass 65 independent SciPy/chunked audio checks, 1,850 native tests,
  453 Python tests, 65 math suites (188,217 checks), strict Clippy/Rustdoc
  and all ten artifact phases; preserve prior source and hash-verify promotion.
  Evidence: `output/stream-filters-2026-10-23/validation.json`.
- [x] Expose the same stateful CPU filter kernels as crate-root
  `NativeIirFilter`/`NativeSosFilter` APIs for direct Rust reuse; verify chunked
  output/state equivalence, state restoration and BASIC/native parity through
  external integration tests. Block methods allocate scratch/output per block and do not claim a
  zero-allocation device-callback contract.
- [x] Add opt-in CPU windowed-sinc anti-aliasing to `RESAMPLE`, retaining linear
  interpolation as the default. Cutoff follows output Nyquist on downsampling;
  bound work and cancellation, preserve constant signals and qualify passband,
  alias rejection, BASIC calls and legacy behavior. Evidence:
  `output/sinc-resample-2026-10-24/validation.json`.
- [x] Add public typed Rust `NativeSincResampler` processing with constructor-
  allocated history/output scratch, rational phase, bounded blocks, group-delay
  reporting, atomic errors/flush and no processing-path allocation. Verify
  chunk-invariant output, DC, alias rejection, reset and recovery. Evidence:
  `output/sinc-stream-2026-10-24/validation.json`.
- [x] Add bounded exact-phase CPU polyphase coefficient tables for rational
  audio rates, with normalized rows, a 500,000-coefficient/work cap and a bounded
  on-demand fallback for larger phase sets. Verify table admission, fallback,
  chunk invariance, DC and cancellation atomicity. Evidence: `output/sinc-polyphase-2026-10-24/validation.json`.
- [x] Add `NativeStream::render_into` with constructor-reserved mono scratch and
  caller-owned stereo blocks. Reuse rendered/encoded buffers in live playback and
  WAV export; compare output bit-for-bit across 137-frame chunks and reject
  oversized blocks before mutation. This removes repeated mixed/mono block-vector
  allocation. Evidence:
  `output/audio-block-render-2026-10-24/validation.json`.
- [x] Reserve fixed per-voice DSP state when a native stream is created; note
  starts and instrument-state changes reuse that storage. Replace the Santur
  resonator's per-note mode vector with a fixed six-mode bank. Verify repeated
  Guitar, Banjo and Santur transitions, stable reserved slots, bit-identical
  eager/deferred output across 137-frame chunks and all 983 library tests.
  Evidence: `output/realtime-note-state-2026-10-24/validation.json`.
- [ ] Integrate streaming resampling/filtering with negotiated device rates and a
  live scheduling contract, then qualify physical latency/listening. The current
  playback worker is allocation-free for note activation but still uses the
  existing process/pipe scheduler and has no negotiated device-rate resampling.

### Established engines with a programmable BASIC frontend — 2026-10-23

- [x] Record the owner's Debian math catalog and architecture direction in
  `docs/MATH_BACKEND_ADAPTERS.md`; preserve the parsed candidate inventory and
  attachment hash in `output/backend-package-review-2026-10-23/catalog.json`.
- [ ] Evaluate established native libraries through checked Rust adapters before
  reimplementing missing numerical engines. Start with NLopt optimization,
  SuiteSparse direct sparse solves and SUNDIALS simulation/DAE capabilities;
  also assess GLPK/lp_solve and algebra/number-theory engines against real gaps.
- [ ] Inventory build/guest dependencies and pin candidate ABIs; design ordinary
  programmable BASIC APIs, typed data/callback conversion, explicit backend and
  termination reporting, bounded execution and interruption/failure recovery.
- [ ] Qualify each adopted adapter independently and in composed BASIC workloads,
  preserve tested source checkpoints and complete separate OS/image/hardware
  acceptance. Package installation or a generic command launcher is not engine
  integration or completion. BASIC remains freely editable; Rust owns integration.

### First Rust/native optimization adapter — 2026-10-23

- [x] Implement typed Rust/NLopt C integration and programmable NLOPTMIN/BACKEND
  with local COBYLA/SLSQP, analytic gradients, finite bounds and nonlinear
  inequalities/equalities; preserve raw termination and final violation reports.
- [x] Prepare checksum-pinned Debian NLopt privately in the home workspace and
  add reproducible preparation tooling and future package dependency declarations.
- [x] Pass 1,859 native tests, 453 Python tests, 43 independent adapter groups
  including the constrained-allocation application, 66 math suites (188,260
  checks), strict Clippy/Rustdoc and all ten artifact phases. Verify missing-
  backend, all-fixed and dependency-tampering/reuse behavior. Preserve prior
  source and hash-verify promotion to the original.
  Evidence: `output/nlopt-adapter-2026-10-23/validation.json`.
- [ ] Qualify additional algorithms/versions/architectures, KKT/global guarantees
  where supported, and rebuilt OS deployment. Complete the remaining library,
  scientific workload and OS/hardware tasks; this adapter alone is not completion.

### Reusable Rust/SuiteSparse sparse LU — 2026-10-23

- [x] Implement native owned UMFPACK factors and BASIC SPFACTOR/SPFSOLVE,
  transpose solves, true residual reports, metadata and explicit free/count/clear.
  Preserve session lifetime, distinct negative handles and error recovery.
- [x] Make validated sparse linear reports composable across DEF FN scopes;
  retain canonical CSR/numeric-data checks and reject unrelated mixed strings.
- [x] Pass 1,869 native tests, 453 Python tests, 57 independent sparse/circuit/
  cross-domain groups, 67 math suites (188,317 checks), strict Clippy/Rustdoc
  and all ten artifact phases. Verify dependency tampering/reuse, missing backend
  and late native overflow recovery. Preserve prior source and hash-verify
  promotion. Evidence: `output/sparse-direct-2026-10-23/validation.json`.
- [x] Add CPU incomplete-Cholesky IC(0) preconditioning to SPCG, retaining the
  Jacobi default, same-pattern storage, bounded factor/solve work and explicit
  nonpositive-pivot errors. Preserve original-system residual checks.
  Evidence: `output/ic0-cg-2026-10-23/validation.json`.
- [x] Extend SPCG to CPU symbolic level-of-fill incomplete Cholesky IC(k),
  levels 0 through 8, with the Jacobi default and IC(0) compatibility preserved.
  Bound combined triangular-factor storage at one million entries and charge
  symbolic setup, numeric factorization and solves to the shared sparse budget.
  Validate factor growth on a two-dimensional stencil and BASIC solves for IC(0),
  IC(1) and IC(2) against a known solution and the fresh original residual. More
  fill is not claimed to reduce iterations. Evidence:
  `output/ic-k-cg-2026-10-24/validation.json`.
- [x] Add CPU level-one symbolic-fill ILU preconditioning (`ILU1`) to SPGMRES,
  keeping ILU(0) available, enforcing a one-million-entry factor cap and the
  shared work budget, and qualifying exact original-system residuals. On the
  400-state Poisson workload, ILU(1) reduced iterations from 10 to 7 versus
  ILU(0), with both converging below tolerance.
  Evidence: `output/ilu1-gmres-2026-10-23/validation.json`.
- [x] Add bounded level-two symbolic-fill CPU preconditioning (`ILU2`) and
  independently compare row scaling/ILU(0)/ILU(1)/ILU(2) on a 400-state Poisson
  workload. They converged in 29/10/7/6 iterations with exact-rational residual
  agreement; 24 scaled sparse references and pivot refusals passed.
  Evidence: `output/ilu2-gmres-2026-10-23/validation.json`.
- [x] Expose bounded user-selectable CPU ILU fill levels 0 through 8; qualify
  level 3 against the lower levels with exact residual references and pivot
  failures, verify the factor cap under generated fill, and accept level 8 through
  BASIC. On the 400-state Poisson workload, levels 0/1/2/3 took 10/7/6/4
  iterations. Levels 4 through 8 are implemented but not performance-qualified.
  Evidence: `output/ilu3-gmres-2026-10-23/validation.json`.
- [x] Add CPU `SPMINRES` for exactly symmetric real sparse matrices, covering
  indefinite systems that SPCG cannot accept. Use the short MINRES recurrence,
  O(n) working vectors, bounded shared work/cancellation and an original-matrix
  residual check on every candidate. Add optional symmetric Jacobi diagonal
  scaling; preserve NONE as the default and reject missing/zero Jacobi diagonals.
  Validate consistent singular systems, wide scaling, initial guesses,
  malformed/nonsymmetric input and independent SciPy MINRES/direct references
  through 513 unknowns. Evidence: `output/sparse-minres-jacobi-2026-10-24/validation.json`
  (792 independent checks, including a 20,000-state sparse solve beyond the
  dense matrix limit).
- [ ] Extend to broader sparse algorithms and pivoted preconditioners;
  qualify versions/architectures, hard native-call isolation/latency and rebuilt
  OS/images.
  The broader math, creative-computing and hardware/release gates remain open.

### Serial Rust/SUNDIALS CVODE foundation — 2026-10-23

- [x] Implement CVODESOLVE/CVODEBACKEND with typed Rust BDF/Adams integration,
  ordinary BASIC RHS callbacks, component tolerances, exact requested output
  times, diagnostics, shared equation budgets and atomic error/cancellation recovery.
- [x] Prepare a checksum-pinned private serial source build and configuration ABI
  bridge, isolating provider symbols from incompatible system/MPI installations.
- [x] Pass 1,877 native tests, 453 Python tests, 34 independent stiff/scientific/
  composed backend groups, 68 math suites (188,351 checks), strict Clippy/Rustdoc
  and all ten artifact phases. Verify four dependency integrity/reuse checks and
  explicit missing-provider behavior. Preserve prior source and hash-verify
  promotion. Evidence: `output/cvode-2026-10-23/validation.json`.
- [x] Add a bounded, checked BASIC analytic dense Jacobian callback to CPU CVODE
  BDF and qualify it on the stiff Robertson system against independent SciPy
  Radau/BDF references. Adams and the default finite-difference behavior remain
  unchanged.
- [x] Add configurable CPU matrix-free SPGMR to BDF, avoiding dense matrix
  allocation through 4,096 states. Qualify a 512-state periodic diffusion mode
  through BASIC CSR/`SPMV` composition and the analytic Fourier-mode reference.
- [x] Add an optional BASIC analytic Jacobian-times callback to CPU SPGMR BDF.
  Validate its shape, finite values, work budget and native error containment;
  compare a 512-state CSR periodic-diffusion integration with difference
  quotients, the analytic Fourier mode and independent SciPy Radau/BDF results.
  Evidence: `output/cvode-jtimes-2026-10-24/validation.json`.
- [x] Add optional terminal scalar root events to CPU CVODE BDF/Adams solves.
  Report stable found/not-found status, append the interpolated root state, detect
  roots at the initial state, and keep shape/error/cancellation failures atomic.
  Compare a terminal decay crossing to its analytic time/state. Evidence:
  `output/cvode-events-2026-10-24/validation.json`.
- [x] Add an optional sparse `CSR1` matrix-Jacobian callback to CPU BDF/SPGMR.
  Validate shape/canonical storage, cache identical linearization points and
  multiply through Rust without dense expansion. Compare a 512-state periodic
  diffusion model with direct J-times, SciPy Radau/BDF and its analytic mode.
  Evidence: `output/cvode-sparse-jacobian-2026-10-24/validation.json`.
- [x] Add `CVODESENSOLVE` on the serial CPU CVODES provider, with BASIC RHS
  callbacks receiving an explicit parameter list, optional initial-sensitivity
  matrices, BDF/Adams and BDF/SPGMR support, centered parameter differences,
  sensitivity error control, bounded dimensions and typed Rust reuse. Qualify
  analytic decay sensitivities for both methods against SciPy Radau/BDF and a
  512-state SPGMR periodic-diffusion sensitivity against its Fourier mode. Pass
  77 independent CVODE/CVODES reference groups; evidence:
  `output/cvode-cvodes-sensitivity-2026-10-24/validation.json`.
- [x] Add an optional BASIC analytic total sensitivity-RHS callback receiving
  state, parameters, the current sensitivity vector and a one-based parameter
  index. It returns `df/dy*S + df/dp`; validate finite values, dimensions,
  callback failure atomicity, work/cancellation and callback statistics. Compare
  two parameter sensitivities with exact exponential derivatives and a coupled
  SIR model against SciPy Radau/BDF. Check cancellation inside the native
  sensitivity callback. The default centered-difference CVODES path remains
  available. Evidence:
  `output/cvode-sir-sensitivity-cancellation-2026-10-24/validation.json` (90
  independent groups).
- [x] Expose `NativeCancellationToken` for cross-thread cancellation of bounded
  Rust math operations. Keep the token scoped to a worker thread, persistent until
  explicitly reset, and restore any prior scope on return/panic. Poll it from
  ordinary native CVODE RHS callbacks as well as the shared math checkpoints;
  verify mid-solve CVODE and IDA cancellation, signal-kernel cancellation, reset and
  scope isolation. Evidence:
  `output/native-math-cancellation-token-2026-10-25/validation.json`.
- [x] Add bounded native CVODE terminal-root time and total event-state
  sensitivity estimates with centered perturbations, Richardson refinement and
  optional initial-state derivatives. Verify against closed-form decay/root
  derivatives, integration counts, invalid perturbation rejection and missing
  root failure. Evidence:
  `output/native-cvode-event-sensitivity-2026-10-25/validation.json`.
- [x] Expose terminal-root time and total state sensitivities to freely editable
  BASIC programs as `CVODEEVENTSENS`. Include BDF/Adams, dense/SPGMR controls,
  optional initial-state sensitivities, finite-difference error estimates and
  aggregate work statistics. Verify BASIC callback argument/return shapes,
  analytic event-time and event-state derivatives, missing-root failure and
  assignment atomicity, plus interpreted/standalone output parity. Evidence:
  `output/native-cvode-event-sensitivity-2026-10-25/basic-validation.json`.
- [x] Add typed Rust and BASIC `CVODEEVENTVAR` analytic variational event
  sensitivities. Integrate supplied total sensitivity equations with CVODES,
  accept event partials in `{g_t,g_y...,g_p...}` order, compute event-time and
  total event-state derivatives, report the transversality denominator, and
  reject malformed or cancellation-dominated event gradients. Qualify the
  exponential threshold model with BDF/dense, BDF/SPGMR and Adams/dense,
  including nonzero initial sensitivities; reject malformed gradients and a
  zero-transversality root, verify BASIC assignment atomicity and exact
  interpreted/standalone output.
  Evidence: `output/native-cvode-variational-event-2026-10-25/validation.json`.
- [ ] Add adjoint sensitivities and broader
  physical/scientific sensitivity models. Bundle/qualify the CVODES provider and
  notices in OS packages/images and on other architectures/hardware. This
  foundation does not complete the math engine or OS. Prior SPGMR evidence:
  `output/cvode-spgmr-2026-10-24/validation.json`.

### Serial Rust/SUNDIALS IDA residual DAE foundation — 2026-10-23

- [x] Add CPU-only `IDASOLVE` with a Rust-owned serial SUNDIALS IDA bridge,
  BASIC residual callbacks `F(t,y,yp)`, differential/algebraic ID vectors,
  component tolerances, exact output times, bounded callback work, cancellation,
  safe native cleanup and atomic failure behavior. Supply consistent initial
  values explicitly; offer dense Newton through 256 states or matrix-free SPGMR
  through 4,096 states.
- [x] Compare an index-one exponential/algebraic system to its analytic solution,
  test reverse-time and one-time-grid behavior, reject invalid IDs/residual shapes,
  preserve assignments on errors, retain CVODE integration and pass focused
  provider ABI/format checks.
- [x] Add dense analytic Newton-matrix callbacks and matrix-free analytic
  Jacobian-vector callbacks to CPU IDA solves and consistent initialization.
- [x] Add optional CSR1 IDA Newton Jacobians through the CPU SPGMR path. Rust
  validates and caches the assembled matrix and computes each Krylov product; this
  is not sparse direct factorization. Exercise it in IDASOLVE and IDACONSISTENT,
  including malformed-shape atomicity.
- [ ] Independently qualify broader index-one physical/scientific workloads.
  KLU sparse-direct availability on non-amd64 targets, higher-index systems,
  cross-target runtime,
  package/image and hardware qualification remain separate gates; this
  implementation does not complete the advanced math engine or the OS.

### Typed Rust IDA consumer API — 2026-10-25

- [x] Expose the shared serial CPU IDA residual solver as `NativeIda` with typed
  differential/algebraic component metadata, bounded dense/SPGMR options,
  direct Rust residual closures, typed requested-time trajectories, and solver
  diagnostics. No BASIC `Value` conversion is used by the consumer API.
- [x] Expose IDA's index-one consistent-initial-condition calculation through a
  typed Rust method returning corrected state and derivative vectors. Verify
  repair from an intentionally inconsistent mixed DAE initial guess.
- [x] Add a direct Rust dense analytic Newton-matrix callback for IDA and verify
  the shared `dF/dy + c_j dF/dy'` contract against the mixed exponential DAE.
- [x] Add the corresponding direct Rust analytic Jacobian-vector callback for
  matrix-free SPGMR and verify it against the same coupled DAE solution.
- [x] Allow a direct Rust inverse-diagonal setup callback to be composed with
  the analytic SPGMR product; verify both callback counters and the DAE result.
- [x] Expose the existing fixed-pattern CSR1/KLU CPU path to Rust through
  canonical `NativeCsrMatrix` Jacobians, requiring a fixed exact nonzero count;
  compare the nonlinear RC DAE trajectory with the dense analytic solve.
- [x] Expose native IDA root localization and resets with typed event direction,
  event records, bounded event count, one-based reset indices, and automatic
  index-one consistency restoration. Verify the analytic exponential/algebraic
  trajectory and pre/post-reset values.
- [x] Compose native hybrid events/resets with a Rust dense analytic Jacobian,
  including its use during post-reset consistency restoration; verify the same
  closed-form root and trajectory.
- [x] Compose hybrid resets with matrix-free SPGMR Jacobian products and an
  inverse-diagonal preconditioner; verify event time, restored constraints,
  final trajectory and native operator/preconditioner counters.
- [x] Compose hybrid resets with fixed-pattern CSR/KLU Jacobians and use the
  same matrix callback for post-reset consistency restoration; verify the root,
  reset projection and final trajectory against the analytic model.
- [x] Qualify the typed Rust KLU API at 512 states with periodic heat diffusion,
  an exact discrete Fourier-mode decay reference, component tolerances, and a
  fixed three-entry-per-row Jacobian. Maximum state error is checked at all four
  requested output times. Evidence:
  `output/native-ida-rust-512-heat-2026-10-25/validation.json`.
- [x] Qualify the typed Rust KLU API at its 1,024-state limit using periodic
  heat diffusion, an exact discrete Fourier-mode reference at four output times,
  and a fixed three-entry-per-row Jacobian. The full focused/native and IDA
  regression suites pass; the combined run used 40,248 KiB maximum RSS. This
  verifies the current amd64 provider cap for this workload, not larger KLU
  systems or cross-target availability. Evidence:
  `output/native-ida-rust-1024-heat-2026-10-25/validation.json`.
- [x] Verify a mixed differential/algebraic exponential system against its
  analytic solution, reject mismatched metadata, and propagate callback errors
  atomically. Focused validation:
  `output/native-ida-rust-hybrid-klu-2026-10-25/validation.json`.
- [x] Qualify the typed Rust residual API on stiff Robertson kinetics with an
  algebraic conservation constraint. Compare dense analytic-Jacobian and SPGMR
  J-times trajectories at six requested times; check nonnegative concentrations,
  total concentration, cross-solver parity and the final SciPy Radau/BDF reference.
  Evidence: `output/native-ida-rust-robertson-2026-10-25/validation.json`.
- [x] Add direct Rust `NativeIda::integrate_parametric` sensitivity estimates
  using centered parameter differences and Richardson extrapolation, with optional
  initial-state/derivative sensitivities, an eight-parameter bound, solver-work
  diagnostics and explicit numerical-error limitations. Verify one- and two-
  parameter analytic DAE solutions, initial-condition derivatives, shape/step
  rejection and direct Rust use. Evidence:
  `output/native-ida-parametric-sensitivity-2026-10-25/validation.json`.
- [x] Add optional CPU IDASens provider packaging and direct Rust
  `NativeIda::integrate_variational` support with centered residual differences,
  separate sensitivity tolerances, initial-state/derivative sensitivities and a
  bounded eight-parameter interface. Verify one- and two-parameter analytic
  exponential systems with dense and SPGMR solves, plus ordinary IDA regression
  behavior with and without the optional provider. Evidence:
  `output/native-ida-idasens-2026-10-25/validation.json`.
- [x] Add `NativeIda::integrate_variational_with_sparse_jacobian` for IDASens
  with fixed-pattern CSR/KLU and parameter-aware residual/Jacobian callbacks.
  Verify the KLU sensitivity trajectory and derivative against the closed-form
  exponential model; rerun typed IDA and sensitivity suites with both the IDAS+
  KLU provider and the default provider, and build warning-free API docs. Evidence:
  `output/native-ida-idasens-klu-2026-10-25/validation.json`.
- [x] Add bounded Rust `NativeIda::integrate_hybrid_parametric` event-time and
  pre/post-reset state estimates using centered differences and Richardson
  refinement. Hold initial conditions fixed, reject changed event topology, and
  report aggregate solver work and finite-difference error estimates. Verify
  parameterized threshold/reset derivatives, invalid-step rejection and event
  topology changes; run the 15 native IDA, 7 IDASens, 30 BASIC IDA and 1,008 Rust
  library tests, plus warning-denied API docs. This is not variational saltation
  propagation and does not provide post-event trajectory sensitivities. Evidence:
  `output/native-ida-hybrid-parametric-sensitivity-2026-10-25/validation.json`.
- [x] Expose bounded hybrid event/reset parameter estimates to BASIC through
  `IDAHYBRIDSENS`, passing the parameter vector explicitly to editable residual,
  event and reset functions. Verify event-time and pre/post-reset derivatives in
  the BASIC interpreter against the closed-form threshold/reset DAE. The BASIC
  adapter supports dense and unpreconditioned SPGMR; KLU, analytic hybrid
  Jacobians and variational saltation remain open. Evidence:
  `output/native-ida-hybrid-parametric-sensitivity-2026-10-25/validation.json`.
- [x] Add variational IDASens hybrid/event-time and reset saltation propagation.
  For guard `g(t,y,p)=0`, derive `dt_event/dp =
  -(g_y S^- + g_p)/(g_t + g_y ydot^-)`; for reset `y^+=R(t,y^-,p)`, initialize
  the fixed-time post-reset sensitivity with
  `S^+ = R_t dt_event/dp + R_y(S^- + ydot^- dt_event/dp) + R_p - ydot^+ dt_event/dp`.
  Provide a bounded, documented numerical linearization policy or analytic
  gradient callbacks for the event/reset maps, and report their error estimates.
  Reinitialize IDASens after each reset, run IDA's consistent-IC calculation
  with sensitivities active, and retrieve the corrected sensitivity conditions
  for index-one constraints. For simultaneous roots, require a common event-time
  derivative across active guards or reject the nondifferentiable ordering.
  Qualify scalar and multi-parameter threshold/reset systems, algebraic
  constraints, reset composition and post-event trajectories against closed-form
  solutions. SUNDIALS' [IDAS forward-sensitivity documentation](https://sundials.readthedocs.io/en/v7.1.1/idas/Usage/FSA.html)
  describes `IDASensReInit`, sensitivity-aware `IDACalcIC` and retrieval of
  corrected sensitivity initial conditions.
  Resolution and verification (2026-10-25): a standalone C reproduction with
  bundled SUNDIALS 7.1.1 isolated the strict-tolerance nonlinear-corrector failure
  and automatic SPGMR startup stall. The typed options now expose a validated
  sensitivity nonlinear-convergence coefficient (default 8.0), separate from
  local-error tolerances. Hybrid sensitivity reset consistency uses staggered
  IDASens initialization with a ten-iteration corrector cap, retrieves corrected
  conditions, then resumes simultaneous integration. Variational SPGMR seeds its
  initial step from the requested interval and state/sensitivity tolerances,
  capped at 0.001 of the interval and the caller's maximum step, and reapplies
  the seed after restarts. For hybrid reset projection with at most 256 states,
  SPGMR now uses a bounded dense direct solver only during consistency projection
  and restores the selected SPGMR solver for trajectory integration. This also
  handles a far-off algebraic reset guess. Strict one- and three-parameter
  nonlinear index-one systems, reset saltation, event-time derivatives and
  post-event trajectories pass closed-form checks. Final verification: 31 BASIC
  IDA tests, 15 native IDA API tests, 18 native sensitivity tests, 4 native
  physics API tests, 1,008 Rust library tests, and `cargo fmt --all -- --check`
  pass with bundled providers. Remaining gates are reset projection above 256
  states, broader physical/scientific DAE families, and provider packaging and
  qualification on other targets. The full all-target test suite and OS/image
  acceptance remain separate project gates.
- [ ] Broaden physical/scientific DAE workloads and qualify IDAS provider
  packaging on other targets. This DAE milestone does not complete the math
  engine or OS.

### CPU IDA hybrid event/reset checkpoint — 2026-10-24

- [x] Add BASIC `IDAHYBRID` over the serial CPU IDA provider: up to 64 crossing
  guards, direction filters, one-based reset callbacks, bounded event records,
  safe native restart and supported index-one consistency restoration. Dense and
  preconditioned SPGMR reset paths pass analytic trajectory checks; malformed
  resets preserve destination assignment and release native solver state.
  Simultaneous roots are applied in ascending index order, with each reset seeing
  the preceding consistent state.
- [x] Qualify two simultaneous guards with reset composition in event-index order
  and two sequential crossings about `2E-5` apart across reset restarts, restoring
  the algebraic constraint each time.
- [x] Add and qualify `programs/science/thermal_hysteresis_ida.bas`: a CPU
  thermostat DAE with six switch transitions checked against closed-form heating
  and cooling times and the algebraic heat-transfer equation.
- [x] Add and qualify `programs/science/bouncing_ball_ida.bas`: a three-state
  index-one residual DAE with discontinuous impact-velocity resets. Verify seven
  impact times and restitution velocities against the analytic geometric flight
  recurrence and check the algebraic force constraint across the trajectory.
  Evidence: `output/ida-bouncing-ball-2026-10-24/validation.json`.
- [x] Pass the 30-test IDA integration target, including both physical DAE
  workloads, residual and consistent-initialization behavior, SPGMR and KLU
  regressions.
  Evidence:
  `output/ida-hybrid-2026-10-24/validation.json`.
- [ ] Add further physical DAE reset workloads and broaden clustered-root
  qualification beyond the tested exponential trajectory, bouncing-ball
  impacts, and `2E-5` separation.
  Tangencies without sign changes, higher-index and
  discontinuous DAE resets, broad reset projection, cross-target/provider image
  packaging and hardware qualification remain open. This CPU milestone does not
  complete general DAE support or the advanced math engine.

### CPU matrix-free IDA Krylov scaling — 2026-10-23

- [x] Add an explicit SPGMR choice to IDASOLVE, link the pinned serial CPU
  SUNDIALS iterative solver into the isolated provider, retain dense solving as
  the default for small systems, and bound Krylov dimension and callback work.
- [x] Solve a 512-state residual system and a coupled periodic-diffusion system
  through BASIC callbacks; verify analytic decay, CVODE compatibility, combined
  provider export admission, provider reuse integrity and Rust formatting.
  Evidence: `output/ida-spgmr-cpu-2026-10-23/validation.json`.
- [x] Add an optional BASIC-supplied inverse-diagonal CPU preconditioner to IDA
  SPGMR, with finite/shape checks, callback cancellation and work budgets, cached
  application in Rust, appended setup-call statistics, and a coupled periodic
  diffusion regression. This is a user-supplied diagonal approximation, not an
  automatic sparse Jacobian or a convergence guarantee.
- [x] Exercise the BASIC periodic-diffusion DAE through the 4,096-state SPGMR
  ceiling with analytic J-times and diagonal preconditioning. A later indexing
  review found the initial point comparator used the adjacent state because each
  result row includes its time column; its claimed sample-error bound is
  superseded by the full-profile reference check below. Preserve the former
  report at `output/ida-spgmr-full-profile-2026-10-24/evidence-preimage/`.
- [x] Upgrade the maximum-size DAE check to compare all 4,096 BASIC result
  values against independent SciPy sparse BDF and Radau runs and the exact
  discrete Fourier mode. Maximum absolute errors are below 7.3E-15; the checker
  also verifies conservation, symmetry and solver statistics. Pass all 27 Rust
  IDA tests and 38 independent checker groups. Evidence:
  `output/ida-spgmr-full-profile-2026-10-24/validation.json`.
- [ ] Independently qualify additional coupled, stiff/sparse DAE families and
  cross-target runtime, image/package and hardware gates.

### CPU IDA consistent initial-condition calculation — 2026-10-24

- [x] Add `IDACONSISTENT` on the Rust/SUNDIALS CPU path. It explicitly calls
  IDA's index-one calculation, supports dense or matrix-free SPGMR solves, and
  validates and returns corrected state/derivative vectors while preserving
  existing `IDASOLVE` behavior for callers that already have consistent data.
- [x] Compare correction of a nonlinear algebraic constraint and differential
  derivative against exact values, pass corrected vectors into IDASOLVE, verify
  callback-shape failure preserves assignment, and admit both required native
  exports in the isolated provider/package checks.
- [x] Extend initialization to matrix-free SPGMR, including a preconditioned
  512-state BASIC callback regression.
- [ ] Independently qualify broader physical/scientific DAE workloads.
  Higher-index DAEs remain outside the supported scope; cross-target runtime,
  package/image and hardware gates remain separate.
  Evidence: `output/ida-consistent-initialization-spgmr-2026-10-24/validation.json`.

### CPU IDASOLVE automatic consistent initialization — 2026-10-24

- [x] Add final `init$="AUTO"` option to `IDASOLVE`; keep omitted and
  `"SUPPLIED"` behavior strict and backward-compatible. AUTO uses IDA's
  index-one calculation before integration, preserves differential initial
  states, corrects algebraic states and differential derivatives, and requires
  two output times to establish integration direction.
- [x] Qualify dense, preconditioned SPGMR/J-times, and KLU paths against analytic
  index-one systems; verify corrected first and final trajectory rows, and check
  one-time-grid and callback-failure assignment safety.
- [x] Pass all 27 focused CPU IDA tests and the full Rust all-target suite.
  Evidence: `output/ida-auto-init-2026-10-24/validation.json`.
- [ ] Extend physical initialization coverage beyond tested index-one systems;
  higher-index DAE classification and cross-target/image/hardware qualification
  remain open.

### CPU IDA stiff scientific DAE qualification — 2026-10-24

- [x] Model the stiff Robertson chemical-kinetics system as a residual DAE with
  an algebraic total-concentration constraint; provide dense analytic Newton and
  matrix-free SPGMR/J-times and CSR1/SPGMR Jacobian callbacks in BASIC.
- [x] Compare all three CPU IDA linearization paths at six requested times
  through 40 seconds against independent SciPy Radau and BDF trajectories; verify
  conservation, nonnegative concentrations and callback/solver statistics. Pass
  15 independent checks and all 15 focused Rust IDA tests.
- [x] Add the CVODE and IDA reference checkers to the full verification and
  release-artifact acceptance gates so the CPU providers remain covered.
- [x] Add a nonlinear RC circuit residual DAE with analytic dense Jacobian;
  compare voltage and algebraic current at six times against independent SciPy
  Radau and BDF integrations, and check the resistor law, positive current and
  monotonic discharge. The focused Rust IDA suite now has 16 tests.
- [x] Add optional amd64 CPU sparse-direct IDA through checksum-pinned SUNDIALS
  KLU and statically linked SuiteSparse, retaining only libc/libm dynamic
  dependencies; validate provider exports, fixed-pattern CSR callbacks and a
  512-state periodic-diffusion trajectory against its analytic Fourier mode.
- [x] Add a 32-by-32 Dirichlet heat equation (1,024 states, 4,992 fixed CSR
  entries) as a user-editable BASIC workload using KLU. Compare every output
  value against its exact discrete eigenmode with maximum error below 2E-7.
  Measure peak SuiteSparse allocation at 1,795,088 bytes under the 64 MiB cap.
  The regression also exposed and fixed a false overlap rejection based on
  pointer distance between separate native vector/matrix buffers.
  Evidence: `output/ida-klu-1024-heat-2026-10-24/validation.json`.
- [ ] Qualify further physical/scientific DAE families, LGPL relinkable
  materials, non-amd64 targets, packaged images and hardware. Prior evidence:
  `output/ida-nonlinear-rc-2026-10-24/validation.json` and
  `output/ida-robertson-dae-2026-10-24/validation.json`.

### CPU KLU allocation bound — 2026-10-24

- [x] Install a process-wide SuiteSparse allocator shim inside the isolated
  serial CPU KLU provider and enforce a 64 MiB per-solve allocation ceiling.
  The bound applies to SuiteSparse allocations, not total process RSS or
  SUNDIALS vectors. Exceeding it fails allocation and returns a BASIC error;
  scope accounting is thread-local and checked through cleanup.
- [x] Verify under-limit allocation, freeing/accounting, and over-limit rejection;
  rerun all 42 CPU physics unit tests, all 27 BASIC-facing IDA integration tests,
  and the 11 provider packaging tests. The new ABI marker and export contract are
  admitted by both CVODE and IDA loaders. Evidence:
  `output/ida-klu-allocation-cap-2026-10-24/validation.json`.
- [x] Measure 1,795,088 bytes of SuiteSparse peak allocation for a 1,024-state
  two-dimensional heat DAE with 4,992 CSR entries, and 2,241,040 bytes for a
  1,000-state three-dimensional heat DAE with 6,400 CSR entries. Both editable
  BASIC workloads match every output value to their exact discrete eigenmode
  below 2E-7 maximum error; both Rust and BASIC IDA regressions pass.
  Evidence: `output/ida-klu-1024-heat-2026-10-24/validation.json` and
  `output/ida-klu-1000-heat3d-2026-10-24/validation.json`.
- [ ] Measure further fill-heavy physical DAE patterns and include KLU in
  packaged-image and hardware checks.

### CPU IDA analytic linearization callbacks — 2026-10-24

- [x] Add optional analytic dense Newton matrices for IDA dense solves and
  analytic Jacobian-vector products for matrix-free SPGMR, including during
  consistent initialization. Keep callbacks inside Rust finite/shape,
  cancellation, panic and shared-work checks; report operator callback counts.
- [x] Qualify a nonlinear dense residual against its analytic Jacobian and a
  512-state periodic diffusion DAE against its analytic trajectory; exercise
  analytic J-times together with preconditioned matrix-free consistent
  initialization and ensure malformed callback failure preserves assignment.
- [x] Run focused IDA, CVODE, adaptive and implicit Rust suites (33 tests)
  against the isolated serial CPU SUNDIALS provider; check Rust formatting.
- [x] Add CSR1 sparse Jacobian callbacks through SPGMR for IDASOLVE and
  IDACONSISTENT; Rust validates, caches and applies the sparse matrix. This remains
  matrix-free Krylov iteration rather than sparse direct factorization.
- [x] Add fixed-pattern CSR1 sparse-direct KLU factorization to IDASOLVE and
  IDACONSISTENT on the amd64 provider, with exact nonzero-count validation,
  Rust CSR-to-CSC conversion, pattern-stability checks and bounded 1,024-state /
  250,000-entry inputs. Compare BASIC nonlinear-circuit outputs with dense IDA
  and independent SciPy Radau/BDF; verify a 512-state sparse periodic-diffusion
  solve against its exact Fourier-mode trajectory.
- [ ] Broader independent physical DAE references, LGPL relinkable materials,
  KLU on other targets, OS package/image and hardware qualification remain open.
  Dense analytic callbacks are capped at 256 states; SPGMR supports through 4,096.

### Native provider release-source integrity — 2026-10-23

- [x] Include native provider bridge sources in every frozen release snapshot;
  pass 19 release tests and all 454 Python tests, including bridge preservation
  and tamper rejection. Retain exact upstream SUNDIALS license/notice from the
  pinned source, preserve prior originals and hash-verify source promotion.
  Evidence: `output/native-release-source-2026-10-23/validation.json`.
- [ ] Complete provider packaging, notices, image deployment and architecture
  qualification. Snapshot coverage alone does not supply the OS runtime library.

### CVODE provider package staging — 2026-10-23

- [x] Add opt-in Debian provider staging with checked ELF/configuration/integrity,
  provenance and notices. Pass six staging regressions, all 460 Python tests and
  34 CVODE reference groups using the actual provider loaded after a private
  provider-only .deb round trip. Preserve prior source and hash-verify promotion.
  Evidence: `output/cvode-packaging-2026-10-23/validation.json`.
- [x] Integrate IDAS-enabled preparation into release-guest automation; select
  one DAE archive and admit the complete base/IDAS/KLU loader interfaces.
  Require all 13 IDAS sensitivity exports for IDAS profiles and the common
  hybrid/root and solver controls; report feature/architecture identities and
  admit the qualified KLU thread-local glibc dependency explicitly. Verify real
  IDAS and IDAS/KLU package round trips, extracted-library mappings, six BASIC
  workload executions, 128 independent numerical-reference checks and 66 Rust
  DAE/API tests. The full Python suite has 480 passes and one ISO-tool skip.
  Correct its release-check output inventory and regenerate the stale portal
  manual. Standard IDAS library size: 644,160 bytes; no BASIC runtime Python
  dependency. Evidence: `output/idas-package-2026-10-25/validation.json`.
- [ ] Qualify complete appliance packages/images and IDAS execution across
  architectures; host provider-only package qualification is not OS acceptance.

### Target-aware provider builds and release automation — 2026-10-23

- [x] Add explicit target compiler configuration, completed ELF checks and
  provider package-content gates before release cache reuse/export. Pass a fresh
  amd64 build, 11 provider regressions, all 465 Python tests and 34 reference
  groups with the rebuilt/extracted provider. Preserve prior source and hash-verify
  promotion. Evidence: `output/sundials-target-build-2026-10-23/validation.json`.
- [x] Cross-build actual ARM64/RISC-V serial providers and execute the retained
  C ABI/trajectory probe under user emulation. Pass 20 native/emulated groups,
  including independent SciPy Radau/BDF chemistry, analytic decay, conservation,
  target ELF and staged-provider hash checks. Preserve prior source and hash-verify
  promotion. Evidence: `output/sundials-cross-2026-10-23/validation.json`.
- [x] Pass 16 focused Rust/BASIC CVODE tests across ARM64/RISC-V under user
  emulation, run the actual interpreters on Robertson chemistry, and pass six
  independent SciPy/conservation groups. Record immutable source/test identities,
  exact debug binaries/providers and Rust 1.85.1; preserve prior source and
  hash-verify promotion. Evidence:
  `output/sundials-rust-cross-2026-10-23/validation.json`.
- [ ] Complete broader math/application suites and release binaries per target,
  full release-guest execution, appliance package/image and hardware qualification.
  Focused cross-runtime emulation alone does not pass these gates.

### Complete solver-provider ABI admission — 2026-10-23

- [x] Require every callable export used by the Rust CVODE loader, check contract
  drift and reject a matching-checksum partially linked ELF before staging.
  Pass 13 provider tests, all 467 Python tests, real amd64/ARM64/RISC-V provider
  admission and the real provider package readback. Preserve prior source and
  hash-verify promotion. Evidence:
  `output/sundials-complete-abi-2026-10-23/validation.json`.
- [ ] Finish the separately running full cross-platform runtime gate and release
  dependency checkpoint; provider admission alone does not qualify OS images.

### Established LP/MIP/convex-QP engine assessment — 2026-10-23

- [x] Assess HiGHS 1.10.0 through its native C interface using privately extracted,
  signed-metadata/hash-verified Debian packages. Pass six analytic/enumerated and
  termination groups, check the actual 32-bit ABI and retain reproducible evidence.
  Preserve prior documentation and hash-verify promotion. Evidence:
  `output/highs-candidate-2026-10-23/validation.json`.
- [x] Implement composable checked Rust/BASIC LP/MIP APIs and the continuous
  convex-QP `QPSOLVE` candidate API, with bounded worker transport, recovery,
  original-model primal/objective checks and relative KKT diagnostics. Evidence:
  `output/highs-runtime-2026-10-23/validation.json` and
  `output/quadratic-model-2026-10-23/validation-basic-qp-v1.json`.
- [ ] Independently broaden QP qualification across randomized/reference models,
  precision and conditioning, then finish packaged-provider, composed-workload,
  target, OS/image and hardware gates. Candidate results and numerical KKT
  residuals are not exact optimality certificates. Numeric-admission probes
  confirm HiGHS drops small coefficients by default; reject or losslessly scale
  affected models rather than silently solving a changed problem. Evidence:
  `output/highs-model-admission-2026-10-23/probe.json`.
- [ ] Qualify composed application workloads and OS/architecture/hardware gates.

### Optimization-provider deployment integration — 2026-10-25

- [x] Declare the qualified HiGHS runtime package explicitly in appliance
  dependencies and provision the same pinned version for each release-guest
  architecture. Dynamic Rust loading had hidden this requirement from ELF
  dependency discovery. Reuse Debian's runtime package without shipping its
  development headers or a duplicate provider inside the appliance archive.
- [x] Require real LP/MIP/QP results and the QP KKT result in the generated image
  test. Keep all solver checks inside the live, user-owned home working directory;
  fix the optional 4ti2 program's escaped final newline and avoid treating common
  GMP support libraries alone as a partially installed 4ti2 provider. Cover dependency
  resolution, failed/mismatched solver output and directory lifetime.
- [x] Verify all three pinned runtime archive hashes/metadata. Qualify the
  extracted amd64 library with the generated HiGHS BASIC program and 63 independent
  LP/MIP references; observe that exact library in worker memory mappings. Run
  all four generated 4ti2 BASIC calls with the existing private provider. Runtime
  sizes: BASIC executable 11,065,872 bytes, HiGHS shared library 3,893,448 bytes.
  No Rust binary rebuild or runtime size increase was needed for these fixes.
  Evidence: `output/highs-deployment-2026-10-25/validation.json`.
- [ ] Build and boot complete updated appliance images, validate installed-library
  discovery and execute these checks on each target. Private 4ti2 extraction
  does not register gmpxx in the host linker cache; its full installed-image gate
  remains open. Broader QP/reference, cross-domain and hardware gates remain open.

### Reproducible optimization-provider preparation — 2026-10-23

- [x] Add checksum-pinned private HiGHS preparation for amd64/ARM64/RISC-V from
  six signed-index-verified archive pins. Pass seven real cache/tree/inode/target/
  offline integrity groups and six native optimization groups on the prepared
  amd64 library; preserve prior source and hash-verify promotion. Evidence:
  `output/highs-preparation-2026-10-23/validation.json`.
- [ ] Implement/qualify the checked Rust/BASIC adapter and full foreign-runtime,
  composed-workload, OS/image and hardware gates. Prepared packages alone do not qualify an
  implementation; native LP/MIP is now qualified below, while these broader
  gates and math-engine completion remain open.

### Shared native sparse-model foundation — verified checkpoint

- [x] Expose canonical CSR storage and exact-product multiplication as a validated
  Rust API, including native zero-constraint shapes and a single-rounded exact
  triple-product quadratic form. Share admission checks with Newton/physics.
  Preserve the positive-dimension BASIC CSR1 interface; expose the same quadratic
  kernel to user programs as `SPQUAD(A,x)`.
- [x] Pass 1,888 native Rust tests across 65 targets, strict Clippy/Rustdoc, all ten
  artifact phases and 68 math suites (188,351 checks). ARM64 and RISC-V each pass
  72 affected-component/API tests under emulation, including 264 native and 263
  BASIC quadratic reference cases. Add a permanent exact-executable checker with
  263 reference cases and four checker success/failure/output-protection groups.
  Preserve prior source and hash-verify promotion. Evidence:
  `output/native-csr-2026-10-23/validation.json` and `source-backup.json`.
- [x] Include `tools/check-native-csr.py` in the regular math-gate registry. Pass
  all 69 suites (188,614 checks) on the exact retained executable and 467 Python
  tests. Evidence: `output/native-csr-registry-2026-10-23/validation.json`.
- [x] Consume this foundation in the checked Rust/BASIC LP/MIP and continuous
  convex-QP APIs. Broader model-family qualification remains open below.

### Complete retained ARM64/RISC-V Rust test cohorts — 2026-10-23

- [x] Correct release-guest target BLAS/LAPACK, NLopt, UMFPACK and Pango/Cairo
  preparation. Pass all 1,877 debug Rust tests across 64 targets on each target
  under user emulation, plus 467 Python tests. Retain exact source/test/binary/
  dependency records, failed diagnostic runs and the explicit resource setup;
  preserve prior source and hash-verify promotion. Evidence:
  `output/full-rust-cross-2026-10-23/validation.json`.
- [x] Qualify the native CSR/triple-product changes separately with 72 affected
  component/API tests per architecture. The full cross cohorts above cover the
  retained pre-CSR snapshot only; they do not establish a full 1,888-test cross run.
- [ ] Complete independent cross-target reference/application suites, release
  binaries, full release-guest execution, OS packages/images and hardware gates.

### HiGHS staged Rust model/transport — 2026-10-23

- [x] Preserve explicit finite/unbounded bounds, variable kinds and objective
  sense in a checked CSR-backed LP/MIP model; reject provider coefficient
  reinterpretation. Permit large finite objective offsets, confirmed natively.
  Validate bounded versioned binary transport, bit preservation, truncations,
  allocation guards, tags and byte mutations in nine strict standalone Rust
  tests. Retain native probes and source snapshots. These components are staged
  source, not a public runtime/BASIC optimizer. Evidence:
  `output/native-csr-registry-2026-10-23/validation.json`.
- [x] Connect the model/transport to the Rust HiGHS ABI and isolated worker,
  independently checked primal/status reports and composable BASIC APIs; see
  the native-host LP/MIP runtime checkpoint below. Independent dual/KKT
  certificates remain open.
- [x] Complete continuous convex-QP representation/admission, worker recovery,
  and a composable BASIC candidate API with receiver-side primal/objective/KKT
  checks; see the later native QPSOLVE checkpoint.
- [ ] Complete independent QP model-family, cross-domain, architecture and OS
  qualification; do not treat residual checks as exact optimality certificates.

### HiGHS pinned Rust LP/MIP ABI — staged checkpoint, 2026-10-23

- [x] Implement RAII library/model ownership, exact 1.10.0/32-bit ABI admission,
  checked options, rowwise CSR transport and model-size/nonzero verification.
  Preserve native load/run/termination/primal/dual statuses and optional vectors;
  do not fabricate absent solutions or claim MIP dual certificates. Independently
  recompute primal activities/objective and bound/row/integrality violations.
- [x] Pass 20 warnings-denied Rust component tests in a private-provider process
  capped at 512 MiB address space and 30 CPU seconds: seven new ABI groups plus
  model/transport/exact-accumulation regressions. Include analytic LP, finite
  integer enumeration, small coefficients, finite offsets, infeasible/unbounded/
  time-limit cases, wrong providers and reuse after rejection. Preserve source
  history and hash-verify promotion. Evidence:
  `output/highs-rust-abi-2026-10-23/validation.json`.
- [x] Implement and qualify bounded result transport with receiver-side primal
  recomputation; see the staged result-protocol checkpoint below.
- [x] Implement and qualify the confined worker lifecycle as a staged component;
  see the worker checkpoint below for actual failure/cancellation/resource tests.
- [x] Wire the worker into CLI/compiled-app dispatch and qualify native-host
  runtime/BASIC integration and cleanup; see the runtime checkpoint below.
- [x] Expose composable BASIC/native LP/MIP APIs and a continuous convex-QP
  candidate API with receiver-side KKT diagnostics; see the later QPSOLVE
  checkpoint for its exact supported interface and limits.
- [ ] Complete independent QP, composed workload, full cross-target and OS gates.
  Numerical KKT checks do not certify exact duality or global optimality.

### HiGHS bounded result protocol — staged checkpoint, 2026-10-23

- [x] Implement versioned result frames capped at 4 MiB, with original-model
  shape admission before allocation, finite candidate/dual payloads and strict
  native status/vector-presence contracts. Preserve absent solutions; suppress
  MIP dual certificates. Do not serialize worker-computed primal quality claims:
  recompute activities, objective and bound/row/integrality violations in the
  receiver using the original model and explicit absolute tolerance.
- [x] Pass 27 warnings-denied confined component tests, including seven new
  live-result/forged-quality/status/truncation/oversize/nonfinite/mutation groups,
  plus strict standalone Clippy at Rust 1.74 MSRV. Preserve failed harness
  diagnostics, prior source and hash-verified promotion. Evidence:
  `output/highs-result-wire-2026-10-23/validation.json`.
- [x] Connect the staged components to production worker launch, bounded pipe
  handling, cancellation/timeout/abort/OOM cleanup and reusable BASIC/native APIs;
  see the native-host LP/MIP runtime checkpoint below.
- [ ] Independently qualify native optimality/dual claims and complete QP and
  full cross-domain/architecture/OS qualification.

### HiGHS confined worker lifecycle — staged checkpoint, 2026-10-23

- [x] Implement a fresh resource-limited process per solve, nonblocking bounded
  request/stdout/stderr pipes and cancellation/wall deadlines. Limit the worker
  to 512 MiB address space, bounded CPU time, 64 file descriptors and no cores;
  preserve tighter inherited limits. Use an unreaped leader as the process-group
  identity anchor, kill descendants before reaping, and clean up on every error.
  Direct entry admits one capped model before loading the provider.
- [x] Pass 38 warnings-denied confined component tests and strict standalone
  Clippy at Rust 1.74 MSRV. Ten worker groups cover native solving, cancellation
  with input backpressure, closed-pipe deadlines, abort/malformed output, stdout/
  stderr floods, pipe-holding descendants, memory exhaustion, invalid launch/
  provider/arguments, malformed/oversized direct input and C-header ABI checks.
  Retain the failed temporary-path harness and corrected immutable cohorts;
  preserve prior source and hash-verify promotion. Evidence:
  `output/highs-worker-2026-10-23/validation.json`.
- [x] Wire production CLI and compiled-application dispatch before UI/runtime
  initialization, expose BASIC/native APIs, and qualify the native-host runtime.
- [x] Complete the continuous convex-QP candidate path and expose it through
  BASIC `QPSOLVE`; see the qualification checkpoint below.
- [ ] Complete broad independent/composed native/cross/OS/hardware gates. The
  worker provides resource containment, not a filesystem/network sandbox.

### LP/MIP runtime and BASIC integration — native-host verified checkpoint

- [x] Wire shared model/ABI/protocol/worker into CLI and compiled-application
  startup, public Rust APIs, `LPSOLVE`, `MIPSOLVE`, `HIGHSBACKEND` and validated
  DEF FN report transport. Verify supervisor-death cleanup and ordinary
  cancellation/error paths using actual native workers and fault fixtures.
- [x] Pass 1,929 Rust tests across 66 targets, 467 Python tests, strict Clippy/
  Rustdoc, all ten artifact phases and 70 math suites (188,677 checks), including
  63 exact rational vertex/integer/mixed enumeration and scope/status checks.
  Retain failed diagnostics and a proved Rustdoc-only repair; preserve previous
  source and hash-verify promotion. Evidence:
  `output/highs-runtime-2026-10-23/validation.json` and `source-backup.json`.
- [x] Add continuous convex-QP `QPSOLVE` with independent primal/objective/KKT
  checks; see the dedicated QP checkpoints below.
- [ ] Complete broader independent QP qualification, provider license/packaging/
  deployment, composed and cross-target workloads, release binaries/OS images and
  physical hardware gates. Native LP/MIP acceptance is not full math-engine or
  OS completion.

### Shared exact curvature — native-host verified checkpoint

- [x] Reuse one Rust exact Schur-complement kernel for symbolic quadratic and
  native CSR curvature; expose PSD/NSD predicates to BASIC and native Rust.
  Use exact sparse diagonal-dominance certificates before bounded dense proof;
  preserve inconclusive/resource errors and per-call cancellation/recovery.
- [x] Use bit-based CSR admission/rational conversion/sign/symmetry checks.
  Qualify exact principal minors, subnormal/near-zero negative curvature, large
  sparse matrices and legal assembly-contained CPU-state characterization. The
  earlier experiment running Rust under nondefault controls violates Rust's
  contract and must not count as qualification. Preserve/hash-promote verified
  source only after the corrected gates pass. Pass 1,936 Rust tests across
  67 targets, strict Clippy/Rustdoc, 467 Python tests, all ten artifact phases
  and 71 math suites (188,782 checks), including 105 BASIC curvature checks.
  Evidence: `output/semidefinite-2026-10-23/validation.json`.
- [x] Add BASIC-facing `QPSOLVE` for admitted continuous convex QP models.
  It returns a composable `HIGHSQP1` candidate with original-model objective,
  primal quality, status fields, dual vectors and independently recomputed
  relative KKT residuals. `HIGHSBACKEND()` probes both LP and QP worker paths.
  Verified by `output/quadratic-model-2026-10-23/basic-qp-checks-v4.log` and
  phase logs in `output/native-backend-checks-Qln6Ug0J/`.
- [x] Add reproducible BASIC-level seeded CPU QP families with 35 independently
  constructed two-variable diagonal optima, 24 three-variable optima with full
  non-diagonal positive-definite Hessians, and 12 five-variable optima with
  linearly dependent active rows and non-unique duals. The diagonal family
  sweeps condition ratios up to 1,048,576. Check provider status, reference
  coordinates, original objective and receiver KKT residuals; explicit
  tolerances are recorded with each family. Evidence:
  `output/native-qp-seeded-2026-10-24/validation.json`.
- [ ] Continue native QP qualification beyond the now-tested three-dimensional
  singular nullspace: add broader non-unique optima and degenerate models, tighter
  precision/conditioning sweeps
  and explicit failure thresholds, plus provider packaging and cross-target/
  hardware checks. Seeded independent references now reach seven variables, and
  bounded zero-curvature directions with active linear bounds are covered.
  Relative KKT checks are
  numerical evidence, not exact optimality certificates; mixed-integer QP remains
  unsupported. All optimization-provider development and qualification here is
  CPU-only.

### Owner-supplied Debian math catalog — integration candidates

- [x] Evaluate and shortlist the supplied Debian mathematics catalog as a source of optional
  engines behind programmable BASIC APIs. Preserve Linux/Rust ownership of the
  runtime: use checked Rust adapters, shared data/results, bounded execution,
  cancellation and independent workload qualification. BASIC programs remain
  freely editable consumers. Package presence alone does not complete an API.
  The review recommends PARI/GP as the first new-domain adapter evaluation,
  followed by Singular and cvc5, while deferring broad PDE stacks and duplicate
  CAS runtimes. See `docs/DEBIAN_MATH_CATALOG.md`.
- [x] Assess 4ti2 as a complementary exact-integer backend. Its zsolve, Hilbert,
  Graver, Markov and Groebner tools cover integer-lattice and toric workloads not
  supplied by the current continuous LP/QP or symbolic APIs. Debian stable
  publishes amd64, arm64 and riscv64 builds; upstream documents both command-line
  formats and a C API. Evidence: `output/4ti2-evaluation-2026-10-23/assessment.json`.
- [x] Implement the optional Rust-owned, worker-isolated 4ti2 C API adapter for
  arbitrary-precision integer systems and BASIC `ZSOLVE`. Preserve exact decimal
  values beyond binary64 and signed 64-bit output, bound dimensions/input/files,
  propagate cancellation/time limits, recheck every returned vector exactly, and
  exercise the real provider through both interpreter and compiled BASIC. Evidence:
  `output/4ti2-evaluation-2026-10-23/validation-zsolve-v1.json` and
  `output/quadratic-model-2026-10-23/math-backend-checks-v9.log`. The adapter
  also rejects oversized directly constructed Rust `BigInt` values before
  decimal formatting and caps BASIC vector length before conversion.
- [x] Add worker-isolated Rust and BASIC Hilbert/Graver/Markov basis calls
  (`HBASIS`, `GRAVER`, `MARKOV`), exact kernel-vector verification, Hilbert
  nonnegativity checks, duplicate-row removal, composable tagged reports, and
  interpreter/generated-program integration against the real provider. Markov
  uses the 4ti2 executable as a child of the bounded worker group. Evidence:
  `output/quadratic-model-2026-10-23/math-backend-checks-v17.log` and
  `output/4ti2-evaluation-2026-10-23/validation-4ti2-bases-v3.json`. Returned-vector
  checks do not establish basis completeness.
- [x] Create 4ti2 project directories with owner-only permissions and verify
  they are removed after provider work; retain the filesystem-permission
  regression in the native unit suite. Evidence: v17 gate and
  `validation-4ti2-bases-v3.json`.
- [x] Check exact, analytically known Hilbert and Graver sets for the
  one-dimensional and three-variable kernels, and a two-variable Markov move;
  retain the returned-set comparisons in the real-provider integration test.
  Evidence: `output/4ti2-evaluation-2026-10-23/validation-4ti2-bases-v4.json`.
- [x] Add an independent complete bounded-enumeration oracle for Hilbert and
  Graver bases of the kernel `[1,1,-2]`. A minimal signed zero-sum relation
  over contributions of magnitudes 1, 1 and 2 has at most three terms, which
  proves the oracle bound complete for this matrix; the real 4ti2 provider's
  entire returned sets match it. Evidence:
  `output/4ti2-evaluation-2026-10-23/basis-completeness-v6.json`.
- [x] Gate the Markov CLI on the exact tested 4ti2 1.6.10 release and GMP
  arbitrary-precision mode. The isolated worker probes `--version` with a
  five-second deadline and bounded output before solving; tests cover accepted
  and mismatched versions, non-GMP mode, output flooding and a stalled provider.
  Evidence: `output/4ti2-evaluation-2026-10-23/provider-admission-v2.json`.
- [x] Extend complete Hilbert and Graver basis references with Pottier's proven
  finite rank-based enumeration bound. Compare the real provider against
  exhaustive exact oracles for 12 reproducible seeded one-row matrices, one
  rank-two matrix, eight reproducible rank-three matrices and nine rank-four
  matrices. The cases cover mixed signs, zeros, common divisors and coupled
  equations.
  Reference: Zetzsche, ICALP 2016, Theorem 5.2. Evidence:
  `output/4ti2-evaluation-2026-10-23/basis-completeness-v12.json`.
- [x] Independently verify Markov functionality by exhaustively enumerating
  nonnegative fibers through target 16 for `[1,1,2]` and `[1,2,3]`, then
  checking graph connectivity under the real provider's moves.
  Evidence: `output/4ti2-evaluation-2026-10-23/markov-fiber-connectivity-v1.json`.
- [x] Record the tested amd64 4ti2 C API package and exact library builds using
  Debian package hashes, ELF GNU build IDs, SONAMEs, GMP configuration and
  package-payload comparisons.
  Evidence: `output/4ti2-evaluation-2026-10-23/c-api-build-identity-v1.json`.
- [x] Extend the complete Pottier-bound oracle to four seeded rank-five
  seven-column matrices with sparse ±1 free-column coefficients. The real 4ti2
  Hilbert and Graver sets match all eight independently enumerated results.
  Evidence: `output/4ti2-evaluation-2026-10-23/rank-five-v1.json`.
- [x] Enforce the qualified amd64 GNU build IDs for the actual 4ti2 C API
  libraries, the Markov executable, and direct 4ti2/GMP dependencies before use;
  reject unadmitted builds through the BASIC error path. Correct the identity
  evidence to describe `libzsolve` (the loaded C API) rather than confusing it
  with the separate `lib4ti2gmp` Markov library. Evidence:
  `output/4ti2-evaluation-2026-10-23/runtime-identity-v2.json`.
- [x] Add an offline delivered-image gate for the optional 4ti2 stack: report a
  wholly absent provider when optional, reject partial installations, and exercise
  all four BASIC entry points when present; provide `--require-4ti2` for images
  intended to ship it. The Rust runtime still gates provider build identity.
  Evidence: release-check script generation and Python/shell syntax checks.
- [x] Extend complete-basis references to four deterministic dense rank-five
  5x7 ±1 matrices and fourteen distinct dense matrices with coefficients through
  magnitude 2, including an all-rows `(2,2)` pair matrix with bound 7776. Compare
  complete Hilbert and Graver sets against Pottier L1 bounds 1024, 3125 and 7776
  through the Rust BASIC APIs (36 exact sets). A further eight matrices use a
  separate seed and the Cargo-built runtime. Evidence:
  `output/4ti2-evaluation-2026-10-23/rank-five-dense-v1.json`,
  `output/4ti2-evaluation-2026-10-23/rank-five-coeff2-v1.json`,
  `output/4ti2-evaluation-2026-10-23/rank-five-coeff2-v2.json`,
  `output/4ti2-evaluation-2026-10-23/rank-five-coeff2-toolchain-v1.json`, and
  `output/4ti2-evaluation-2026-10-23/rank-five-coeff22-checker-v1.json`.
- [x] Extend exact Hilbert/Graver reference qualification to four seeded rank-eight
  8x10 identity-block matrices with one negative unit coefficient per row across
  two free columns. Derive the complete Hilbert sets and sign-reduced Graver sets
  from the exact direct-sum structure; verify all eight provider result sets and
  each generator against Pottier's rank-eight bound of 6,561. This exact family
  does not imply completeness for general rank-eight matrices. Evidence:
  `output/4ti2-evaluation-2026-10-23/rank-eight-v4.json`.
- [ ] Extend complete-basis references to broader coefficient families and ranks
  above eight. The rank-seven Pottier-enumerated family has four seeded complete
  references, and rank eight has four exact one-hot-family references, but
  receiver vector checks alone do not prove general basis completeness. Complete
  transitive loaded-object and package-image identity enforcement, qualify
  cross-target providers, and finish packaged-image integration and GPL
  notice/release review before bundling. The extracted identity-qualified Noble
  amd64 provider passes all 15 targeted integration tests, including generated
  standalone BASIC compilation and dispatch. Evidence:
  `output/4ti2-evaluation-2026-10-23/configured-provider-full-integration-v2.json`
  and `.log`. A general all-target Cargo invocation remains host-limited by the
  absent unversioned X11 development link; this does not affect the passing 4ti2
  target. The earlier missing-provider-path run remains a setup diagnostic.
- [x] Add independent complete rank-six Hilbert/Graver references for four
  seeded 6x8 one-negative-unit coefficient matrices (bound 729, eight exact sets)
  and one coefficient-two matrix (bound 4096, up to 745,836 enumerated kernel
  candidates), plus two dense unit-coefficient matrices (bound 4096, up to
  2,740,140 candidates). The dense cases qualify both empty and nonempty Hilbert
  bases as well as Graver bases. Preserve the failed initial harness diagnostic
  and subsequent corrected complete comparisons. Broader dense families, higher
  ranks, and release/image/cross-target gates remain open. The generalized
  checker also passes a rank-five compatibility case. Evidence:
  `output/4ti2-evaluation-2026-10-23/rank-six-v2.json`,
  `rank-six-coeff2-v1.json`, `rank-six-dense-v1.json`,
  `rank-six-dense-hilbert-v1.json`, and `rank-six-attempt-v1-failure.json`, plus
  the final-checker rank-five compatibility report
  `rank-five-harness-compat-v2.json`. Four seeded rank-seven 7x9 one-negative-unit
  matrices also match complete Hilbert and Graver references under bound 2187, with
  up to 532,170 candidates; see `rank-seven-v2.json`.
- [x] Implement a Rust-owned optional PARI/GP worker and typed Rust/BASIC
  `PARIFACTORS` APIs for signed arbitrary-precision integer factorization up to
  256 decimal digits. The worker admits the tested 2.15.4 release, requests PARI's
  rigorous factor proof mode, checks each prime with `isprime`, caps output and
  runs within the shared process-group CPU/memory/wall limits; Rust independently
  checks the complete factor product. Interpreter, compiled BASIC and typed Rust
  integration checks pass against an extracted, uninstalled Ubuntu Noble 2.15.4
  package. A stalled-provider timeout also returns control to BASIC error
  handling. Evidence: `output/pari-prototype-2026-10-23/validation-v1.json`.
- [ ] Complete PARI qualification for the target release: pin runtime package
  hashes/build IDs and transitive dependencies; validate error/recovery and
  cancellation/resource abuse; independently verify primality certificates or
  state a trusted-provider boundary; compare broader exact workloads; finish
  GPL-2-or-later notice/redistribution review and packaged-image tests. The
  prototype is optional and not release or bundling approval.
- [x] Increase the supervised PARI worker's initial stack from the provider's
  8 MiB default to a fixed 64 MiB inside the existing 512 MiB address-space cap.
  Add invalid-result BASIC recovery and independently qualify `2^89-1` and
  `2^127-1` with Lucas-Lehmer checks, then factor their 78-digit product under
  the 30-second limit. All six focused runtime tests pass with the extracted
  Ubuntu Noble provider; this improves a larger CPU workload and does not close
  package identity, proof-certificate,
  licensing, image or cross-target gates. Evidence:
  `output/pari-cpu-maturity-2026-10-24/validation.json`.
- [ ] Select additional symbolic, PDE/finite-element, discrete-geometry or logic
  engines only against named workloads and measured gaps. For each candidate,
  inspect APIs, licensing, footprint and target support; avoid indiscriminate
  installation and duplicated CAS runtimes. See
  `docs/DEBIAN_MATH_CATALOG.md`.
- [x] Assess QSopt_ex as an optional exact-rational LP provider. Its `esolver`
  CLI accepted a bounded LP, returned the exact optimum `1/3`, and identified a
  contradictory model as infeasible. A separate process is the viable initial
  Rust integration boundary; validate the exact input-number contract, status
  and solution parser, cancellation/resource containment, provider admission,
  and BASIC composition before exposing a frontend. The CLI package is
  GPL-3-or-later; the source also contains LGPL-2.1-or-later library files, so
  release bundling and notices require a separate review. The Debian package
  was extracted only under the home workspace and was not installed on the
  host. Evidence: `output/qsopt-ex-evaluation-2026-10-23/assessment-v1.json`.
- [x] Implement a Rust-owned optional QSopt_ex 2.5.10 `esolver` worker and
  BASIC `EXACTLPSOLVE`. Preserve exact integer/decimal/ratio strings, admit a
  bounded dense model, enforce process/CPU/address-space/file-size/time/output
  limits, probe the provider version, parse exact rational results, restore
  omitted zero variables and independently recompute returned primal feasibility
  and objective. Export the typed native Rust model and candidate API. Focused
  real-provider interpreter/native API tests, the configured full Rust suite, strict Clippy, Rust formatting and Rust documentation checks pass; the standalone `COMPILE` integration also passes in an isolated run. The optional eSpeak NG speech test is excluded because its executable is unavailable. Evidence:
  `output/qsopt-ex-evaluation-2026-10-23/validation-v1.json`.
- [x] Independently verify QSopt_ex optimality with exact rational dual
  certificates. Before returning `OPTIMAL`, the receiver checks primal feasibility,
  exact objective, reduced-cost stationarity, row-dual signs and complementary
  slackness across row and variable bounds. Forged stationarity and wrong-sign
  duals are rejected; real-provider MIN/MAX and bounded/free-variable cases pass.
  Evidence: `output/qsopt-ex-evaluation-2026-10-23/dual-certificate-v1.json`.
- [x] Compare bounded exact rational LP results against independent exact
  vertex enumeration for 32 deterministic two-variable model families spanning
  both objective senses and mixed row directions. Every returned optimum matches
  the independently computed exact rational result. Evidence:
  `output/qsopt-ex-evaluation-2026-10-23/rational-reference-v1.json`.
- [x] Extend independent exact vertex enumeration to 16 deterministic
  three-variable rational LPs. Cover both objective senses, fractional
  coefficients and bounds, mixed ranged rows, redundant/degenerate rows, exact
  objective agreement and receiver-certified optimal status. Preserve the
  two-dimensional 32-case oracle. Evidence:
  `output/qsopt-ex-evaluation-2026-10-23/rational-reference-v2.json`.
- [x] Add 20 reproducibly seeded three-variable rational LPs to the independent
  vertex oracle, with fractional objective/row data, bounded variables and
  deliberate duplicate rows. All provider objectives and receiver status checks
  match the independently enumerated optima. Evidence:
  `output/qsopt-ex-evaluation-2026-10-23/rational-reference-v3.json`.
- [x] Extend the exact vertex oracle to eight four-variable LPs, including
  fractional bounds, both senses and repeated rows; verify exact objective and
  receiver-certified status for each. Evidence:
  `output/qsopt-ex-evaluation-2026-10-23/rational-reference-v4.json`.
- [x] Add 12 reproducibly seeded four-variable exact LPs with fractional row
  limits, both senses and repeated-row degeneracy. Compare each exact objective
  and certified status against independent vertex enumeration. Evidence:
  `output/qsopt-ex-evaluation-2026-10-23/rational-reference-v7.json`.
- [x] Extend the exact independent vertex oracle to eight reproducibly seeded
  five-variable rational LPs with fractional data, both objective senses and
  repeated-row degeneracy. Enumerate all 8,568 five-active-constraint sets per
  model using exact rational Gaussian elimination and compare exact objectives
  plus receiver-certified statuses. Evidence:
  `output/qsopt-ex-five-variable-2026-10-24/rational-reference-v8.json`.
- [x] Independently verify infeasible models with normalized exact Farkas
  witnesses. The receiver checks nonnegative multipliers, exact cancellation of
  every variable and a strictly negative combined bound; BASIC marks the status
  verified only after these checks. Forged witnesses are rejected. Evidence:
  `output/qsopt-ex-evaluation-2026-10-23/farkas-certificate-v1.json`.
- [x] Verify `UNBOUNDED` with an exact feasible base point and improving recession
  direction. The receiver checks every row and variable direction, exact base
  feasibility, and strict objective improvement before setting `status_verified`.
  The base and ray solves share the original deadline and extended bounded model
  limits. Forged rays are rejected; real-provider BASIC/native calls cover free
  variables and a row-constrained ray. Evidence:
  `output/qsopt-ex-evaluation-2026-10-23/ray-certificate-v1.json`.
- [x] Qualify QSopt_ex worker failure handling: cancel a blocked provider and
  confirm process-group reaping and private-workspace cleanup; kill a provider
  during solve and verify cleanup; reject an 8 KiB version probe before solving;
  reject a `/dev/zero` solution symlink without an unbounded read. Evidence:
  `output/qsopt-ex-evaluation-2026-10-23/terminal-hardening-v6.json`.
- [x] Close the solution-file check/open race by opening with Linux
  `O_NOFOLLOW|O_NONBLOCK|O_CLOEXEC`, then rechecking regular-file type, size,
  device and inode before the bounded read. Deterministically replace a checked
  regular response with a `/dev/zero` symlink and a FIFO; verify the symlink is
  not followed and FIFO opening cannot block. The new unit test and all 16 real-
  provider QSopt integration tests pass. Evidence:
  `output/qsopt-ex-solution-open-hardening-2026-10-24/validation.json`.
- [x] Extend the independent exact vertex oracle above five variables with eight
  seeded six-variable rational LPs. Cover both objective senses, fractional
  coefficients/bounds and repeated-row degeneracy; enumerate all 74,613 six-
  active-constraint sets per model (596,904 total), checking exact feasibility,
  objective and receiver-certified status. Evidence:
  `output/qsopt-ex-six-variable-2026-10-24/validation.json`.
- [x] Qualify large exact rationals through both the public Rust API and BASIC
  `EXACTLPSOLVE`: preserve a 13,000-bit-denominator optimum whose magnitude
  underflows binary64, accept normalized numerator and denominator values up to
  the 16,384-bit limit, and reject oversized serialized models before building
  their LP text. Input rational tokens allow up to 10,000 ASCII characters.
  The QSopt_ex CLI truncates very long solution lines near 4 KiB; truncated
  responses fail exact receiver verification, so full-width provider output
  remains an open capability. Evidence:
  `output/qsopt-ex-rational-limit-2026-10-24/validation.json`.
- [ ] Broaden exact reference models further and continue live concurrent
  solution-file race, flood and cleanup tests; evaluate provider identity
  integrity, cross-target execution, packaged-image behavior and release
  notices/licensing before bundling. Never present uncertified statuses as
  receiver-proven.

### Native quadratic model/objective foundation — staged checkpoint

- [x] Add a continuous quadratic model with exact shared curvature admission,
  original-model single-round objective/gradient evaluation and lossless binary
  objective scaling. Preserve freely mutable model data and cancellation/reuse;
  reject inexact scaling, unsupported dynamic range and mixed-integer QP.
- [x] Finish bounded native QP model/result transport, lower-triangular provider
  Hessian packing/readback, isolated worker integration and public Rust candidate
  APIs. The receiver rechecks feasibility and recomputes the original objective;
  provider termination is not an optimality certificate. Validation:
  `output/quadratic-model-2026-10-23/compatibility-checks-v14.log` and the
  referenced per-phase logs in `output/native-backend-checks-YFSI6Io3/`.
- [x] Carry native dual vectors through the bounded QP worker protocol and
  independently recompute relative primal-feasibility, stationarity, dual-sign
  and complementarity residuals on the receiver. Expose the diagnostics in Rust;
  verify five analytic cases covering active lower/upper bounds, active lower/
  upper rows, concave maximization, and deliberate false-dual rejection. Evidence:
  `output/quadratic-model-2026-10-23/kkt-checks-v5.log`.
- [x] Expose the native candidate service as composable BASIC `QPSOLVE`, with
  a ten-field `HIGHSQP1` record and receiver-verified primal and KKT diagnostics.
  Evidence: `output/quadratic-model-2026-10-23/basic-qp-checks-v4.log`.
- [x] Extend seeded independent-reference CPU QP qualification to eight
  six-variable cases with strictly diagonally dominant positive-definite
  Hessians, a known unique minimizer, a lower active row and variable bound, and
  three dependent scaled rows. Check all coordinates, objective and receiver KKT
  status. Evidence: `output/qp-six-variable-2026-10-24/validation.json`.
- [x] Extend degenerate convex-QP references to seven variables. Eight seeded
  non-diagonal positive-definite models use independently constructed KKT optima,
  an active lower row and variable bound, and three positively scaled duplicate
  rows. All coordinates, original objectives and receiver KKT status pass at
  `1e-6`; a direct Rust check also confirms the default `1e-7` threshold reports
  a larger measured stationarity residual as unsatisfied. Evidence:
  `output/native-qp-seven-variable-cpu-2026-10-24/validation.json`.
- [x] Qualify five bounded singular-Hessian QPs through BASIC and the typed
  Rust API. Four seven-variable PSD models use a nonzero linear objective to
  select a bound on a zero-curvature coordinate; one additional model verifies
  a non-unique optimum over a flat bounded interval. Check status, coordinates
  or flat-coordinate bounds, objective, feasibility and receiver KKT residuals.
  Evidence:
  `output/native-qp-singular-cpu-2026-10-24/validation.json`.
- [x] Verify a two-dimensional flat optimal face on a seven-variable singular
  convex QP through both BASIC and the typed Rust API. Accept any feasible point
  in the bounded nullspace while checking the curved coordinates, invariant
  objective, independent primal feasibility and receiver KKT residuals. Evidence:
  `output/native-qp-two-dimensional-nullspace-cpu-2026-10-24/validation.json`.
- [x] Extend singular-QP qualification to a three-dimensional flat optimal face
  on an eight-variable positive-semidefinite model. BASIC and typed Rust checks
  verify the five curved coordinates, bounds on all three flat coordinates,
  invariant original objective, independent feasibility and receiver KKT status.
  One positive curvature is `2E-12`, giving a positive-curvature ratio of
  `2E12` alongside the exact three-dimensional nullspace.
  Evidence:
  `output/native-qp-three-dimensional-nullspace-cpu-2026-10-24/validation.json`.
- [x] Qualify an exact non-axis-aligned three-dimensional QP nullspace using
  `H=BᵀB` for a rank-five integer matrix B. BASIC and typed Rust checks verify
  the invariant `Bx`, box feasibility, original objective, receiver feasibility
  and KKT residuals. Evidence:
  `output/native-qp-rotated-nullspace-cpu-2026-10-24/validation.json`.
- [x] Qualify 20 seeded rank-five `BᵀB` models with three-dimensional
  non-axis-aligned nullspaces, through BASIC and typed Rust. Check five
  independent `Bx` invariants, bounds, original objective, feasibility and KKT
  residuals in every case. Evidence:
  `output/native-qp-rotated-nullspace-seeded-cpu-2026-10-24/validation.json`.
- [x] Add conditional CPU variable equilibration for severely ill-conditioned
  diagonal QPs. Exact power-of-two transforms are enabled above 30 binary
  orders for row-constrained models and above 8 for row-free models; a
  condition-ratio `1e12` convex QP passes BASIC and Rust reference checks with
  original coordinates/objective and receiver KKT validation at `1e-6`. The 12
  focused CPU QP runtime tests and the HiGHS library QP test pass. Broader
  conditioning families remain open. Evidence:
  `output/native-qp-conditioning-cpu-2026-10-24/validation.json`.
- [x] Add a four-point CPU conditioning sweep at diagonal ratios `1e2`, `1e4`,
  `1e8` and `1e12`. A failure at `1e4` motivated earlier exact power-of-two
  equilibration for row-free, fully unbounded QPs; constrained models retain the
  prior conservative threshold after seeded active-set regression tests exposed
  dual-residual sensitivity. BASIC and typed Rust both pass original-coordinate,
  objective and KKT checks at each sweep point. The HiGHS integration target
  passes 23 tests; its generated native-program worker test remains a slow
  independent gate. Evidence:
  `output/qp-kkt-condition-sweep-cpu-2026-10-24/validation.json`.
- [x] Add bounded box-QP active-bound refinement for up to 256 free variables.
  Snap provider-identified active bounds, solve the free principal Hessian with
  the CPU LU kernel, and accept the point only when it remains feasible, does
  not worsen the objective, and passes receiver KKT checks with rebuilt bound
  duals. Raise early equilibration to row-free bounded models while retaining
  the conservative 30-bit threshold for row-constrained models. BASIC and typed
  Rust references pass at diagonal ratios `1e4`, `1e8` and `1e12`; the seeded
  active-bound/row family and 24 HiGHS integration tests pass. One generated-
  program worker test remains unverified. Evidence:
  `output/qp-box-active-set-polish-cpu-2026-10-24/validation.json`.
- [x] Extend the bounded active-face correction to include active linear rows
  alongside active variable bounds. Build and solve a capped CPU KKT system,
  rebuild row and bound duals, and retain the provider candidate unless original
  feasibility, objective, and receiver KKT checks pass. BASIC and typed Rust
  pass an ill-scaled condition-ratio `1e12` model with a binding row and bound;
  all 25 HiGHS runtime tests (excluding the known slow generated-program worker),
  all-target compilation, and formatting checks pass. Evidence:
  `output/qp-active-row-polish-cpu-2026-10-24/validation.json`.
- [x] Broaden the ill-scaled active-row QP reference beyond one hand-built case:
  18 seeded, independently constructed three-variable equality-row models with
  an active variable upper bound and condition ratio `1e12` pass through both
  BASIC and the typed Rust API. Check optimal coordinates, original objective,
  receiver feasibility and KKT diagnostics. The HiGHS runtime suite passes 26
  tests, excluding its known slow generated-program worker; all-target
  compilation and formatting checks pass. Evidence:
  `output/qp-active-row-seeded-cpu-2026-10-24/validation.json`.
- [x] Verify the active-face solve when two independent equality rows and one
  variable upper bound are active together in a four-variable QP with a `1e12`
  curvature ratio. Extend the correction to remove numerically dependent active
  rows from its small solve, while rechecking every original constraint and
  receiver KKT residual before acceptance. A duplicate scaled row passes in
  BASIC, typed Rust and a direct unit test; the previously marginal seven-variable
  duplicate-row case now passes at the default KKT tolerance. The focused HiGHS
  suite passes 27 tests, the full Rust library passes 994 tests with pinned CPU
  providers, and all-target compilation/format checks pass. Evidence:
  `output/qp-multiactive-rows-cpu-2026-10-24/validation.json`.
- [x] Reduce QP provider sensitivity to row units within existing raw-input
  admission limits: apply exact power-of-two constraint-row equilibration after
  variable transformation, scale finite row bounds identically, and reverse row
  duals before receiver validation. BASIC and typed Rust pass with row scales `2^-30`
  and `2^30`, a dependent duplicate row, an active variable bound and a `1e12`
  Hessian ratio. All 27 focused QP tests, 994 provider-enabled Rust library
  tests, all-target compilation and format checks pass. Evidence:
  `output/qp-active-row-scaling-cpu-2026-10-24/validation.json`.
- [ ] Extend KKT qualification beyond the three-dimensional singular nullspace
  and current non-unique-optimum models, with randomized families,
  precision/conditioning sweeps, failure thresholds and provider comparisons.
  Existing seeded CPU families cover two-, three-, five-, six- and
  seven-variable models. Passing floating-point residuals remain numerical
  evidence, not an exact optimality certificate; full convex-optimization
  qualification remains open. Investigation on 2026-10-24 found that lowering
  the variable-scaling trigger to eight bits for row-constrained active-set
  models caused seeded stationarity residuals of `1.60e-5` and `1.28e-5` against
  a `1e-5` threshold; tighter HiGHS tolerances did not change those values.
  Row-free box models and bounded ill-scaled one- and two-row active faces now
  use receiver-checked primal/dual refinement, including a scaled duplicate row.
  Near-dependent/conflicting rows, singular free-Hessian faces, broader active
  families and evidence for lowering the constrained trigger remain open. Details:
  `output/qp-constrained-equilibration-cpu-2026-10-24/investigation.json`.

- [x] Extract regularized incomplete beta and complementary/log tails with
  typed domains, iteration/method reports and bounded execution. Fixed inherited
  work-accounting gaps and analytic subnormal-log quantization; restricted
  unequal-shape limits reuse the native gamma foundation. All 1,074 Rust tests
  (9 new), 1,616 independent checks across 404 inputs and 22 precision repeats
  pass. Separate tests cover 2,888 finite-binomial comparisons; all 3,771 prior
  gamma/beta scalar outputs remain byte-identical. Five CPU workloads measured
  in three one-CPU trials. Contracts: MATHCORE_INCOMPLETE_BETA.md.
  Evidence: output/mathcore-incomplete-beta-2026-10-30.
  This is partial M08/M09; unregularized integrals and inverse/distribution
  APIs, other mathematical subjects and full standalone workload gates remain open.





- [x] Qualify standalone beta distribution: shared density/tails/inverses, support/endpoint contracts and stable mean/variance. All 1,097 Rust tests (7 new), 3,504 independent checks across 438 inputs and 20 precision repeats pass. Six prior scalar/ratio/inverse/integral probe outputs remain byte-identical. Tiny log-density cancellation corrected with exact shape-one/two identities. Three CPU trials and contracts/provenance are retained in output/mathcore-beta-distribution-2026-10-30 and docs/MATHCORE_BETA_DISTRIBUTION.md. Other distributions, sampling, inference and full standalone gates remain open.
- [x] Qualify chi-square density, moments, tails and physical-coordinate inverse APIs using the shared gamma core. Retain exact half-shapes for odd degrees below twice MIN_POSITIVE, including the lowest normal binade. All 1,120 Rust tests (8 new), 2,900 independent checks over 457 inputs and 25 precision repeats pass; nine prior output sets remain byte-identical. The independently reproduced smallest-normal log-density error is corrected without relaxing tolerances. Contracts/CPU evidence: docs/MATHCORE_CHI_SQUARE.md and output/mathcore-chi-square-2026-10-30. Other distributions, sampling/inference and all wider gates remain open.

- [x] Qualify shared scaled-gamma probability coordinates: preserve finite log tails when value/scale underflows or overflows and exact mean displacement for concentrated shapes. All 1,103 Rust tests (6 new), 1,248 independent checks over 312 inputs and 17 precision repeats pass; seven previous probe output sets remain byte-identical. CPU batches of 10,000 calls take 7.2–79.8 ms on this host. Evidence: output/mathcore-scaled-gamma-2026-10-30; contracts: docs/MATHCORE_SCALED_GAMMA.md. Gamma/chi-square objects, densities, scaled inverses, half-shape underflow and all wider gates remain open.

- [x] Qualify native gamma distribution density/log density, moments, tails and physical-coordinate inverses. Exact triple products retain representable variance; scaled inverse search recovers physical coordinates even when the unit inverse underflows. All 1,112 Rust tests (9 new), 2,864 independent checks over 428 inputs and 19 precision repeats pass; eight prior probe output sets remain byte-identical. CPU evidence/contracts: output/mathcore-gamma-distribution-2026-10-30 and docs/MATHCORE_GAMMA_DISTRIBUTION.md. Chi-square half-shape, remaining distributions/sampling and all broader standalone gates stay open.

- [x] Qualify exponential and uniform distribution densities/logs, moments, tails and ordinary inverses, plus exponential log-probability inverses. Exact arithmetic preserves overflowing interval widths, tiny upper-inverse probabilities and intermediate-square range. All 1,131 Rust tests (11 new), 3,250 independent checks over 947 inputs (1,301 exact-rational comparisons) and 26 precision repeats pass; ten previous probe output sets remain byte-identical. Contracts/CPU evidence: docs/MATHCORE_EXPONENTIAL_UNIFORM.md and output/mathcore-exponential-uniform-2026-10-30. Weibull, other families, sampling/inference and broader standalone gates remain open.

- [x] Qualify native Weibull density/log density, moments/log moments, both tails and ordinary/log-probability inverses, plus shared normalized gamma dispersion. All 1,140 Rust tests (9 new), 4,489 independent checks over 1,905 inputs and 33 precision repeats pass; eleven prior probe output sets remain byte-identical. Corrected a reproduced intermediate log-gamma overflow without relaxing tolerances. CPU/provenance/contracts: docs/MATHCORE_WEIBULL.md and output/mathcore-weibull-2026-10-30. Remaining distributions, sampling/inference and all broader standalone gates stay open.

- [x] Qualify native geometric mass/log mass, exact rounded moments, full-u64 tails and certified ordinary inverse counts. New shared dyadic power comparisons use directed integer intervals, adaptive precision and one cumulative work budget. All 1,150 Rust tests (10 new), 3,005 independent checks over 1,122 inputs, 853 inverse comparisons (424 exact rational), 436 exact value checks and 27 precision repeats pass; twelve prior probe sets remain byte-identical. Contracts/proof/CPU evidence: docs/MATHCORE_GEOMETRIC.md and output/mathcore-geometric-2026-10-30. Binomial, Poisson, other distributions, sampling/inference and full standalone gates remain open.

- [x] Qualify binomial/Poisson forward masses/logs, exact moments and direct/log tails using shared beta/gamma kernels; add bounded complete binomial tables with exact scaled accumulation. All 1,160 Rust tests (10 new), 4,808 independent comparisons over 483 inputs, 944 exact moment checks and 25 precision repeats pass. Eleven table cases include million-row normalization/monotonicity and 344 independently checked rows. Thirteen prior output sets remain byte-identical. Contracts/CPU evidence: docs/MATHCORE_COUNT_LAWS.md and output/mathcore-count-laws-2026-10-30. Count inverses, sampling, wider count domains and all broader standalone gates remain open.

- [x] Qualify bounded binomial/Poisson numerical count inverses for ordinary/log CDF and strict survival, with explicit candidate ranges near probability steps, distinct count-limit/infinite results, neighboring evaluations and cumulative execution budgets. All 1,175 Rust tests (15 new), 5,177 independent checks across 940 inputs, 241 exact-rational count checks and 21 precision repeats pass; fourteen prior probe sets remain byte-identical. The corpus contains 144 explicitly ambiguous ranges; numerical single-count results are not certificates. Contracts/CPU evidence: docs/MATHCORE_COUNT_INVERSES.md and output/mathcore-count-inverses-2026-10-30. Certified refinement, remaining distributions/sampling and all broader standalone gates remain open.

- [x] Qualify certified ordinary-probability binomial inverses with native directed scaled-dyadic arithmetic, exact endpoint/symmetry proofs, adaptive precision, optional untrusted hints and cumulative limb/term/comparison budgets. All 1,191 Rust tests (16 new), 3,817 independent checks across 1,876 cases, including 1,244 exact-rational and 597 exact-endpoint count comparisons and 41 adaptive-precision cases, pass; fifteen prior probe sets remain byte-identical. Corrected premature recurrence-ratio rounding that prevented exact CDF-step certification. Contracts/proof/CPU and memory evidence: docs/MATHCORE_BINOMIAL_CERTIFICATION.md and output/mathcore-binomial-certification-2026-10-30. Poisson/log-target certification, faster large-central certification and all broader standalone gates remain open.

- [x] Qualify certified ordinary-probability Poisson inverses with native exponential enclosures, directed finite sums, exact Markov bounds, adaptive precision and cumulative budgets. All 1,204 Rust tests (13 new), 1,530 independent checks over 570 cases, 70 exact-rational enclosure count checks and 25 adaptive cases pass; sixteen prior output sets remain byte-identical. Distinguish proven count-domain limits from term exhaustion and mathematical infinite endpoints. Proof, domain and CPU/memory evidence: docs/MATHCORE_POISSON_CERTIFICATION.md and output/mathcore-poisson-certification-2026-10-30. Log-target certification, efficient large-central laws, remaining distributions and broader standalone gates remain open.

- [x] Qualify shared product-preserving beta coordinates, exact rounded mean displacement, transformed tails/logs and a rescaled continued fraction. All 1,216 Rust tests (12 new), 3,415 independent checks over 375 inputs, 20 exact-rational probability identity cases and 17 precision repeats pass; seventeen prior output sets remain byte-identical. Corrected fraction indices and missing convergent normalization found by independent tail checks. Contracts/provenance/CPU evidence: docs/MATHCORE_BETA_COORDINATES.md and output/mathcore-beta-coordinates-2026-10-31. Student t/F adapters, exact half-shapes and all broader standalone gates remain open.

- [x] Qualify central Student t density, moments, both tails/logs and ordinary/log physical-coordinate inverses, plus shared beta evaluation retaining exact original half-parameters. All 1,231 Rust tests (15 new), 3,255 independent forward comparisons over 519 inputs and 178 independently checked inverse cases pass; eighteen prior output sets remain byte-identical. Small central mass and represented logarithmic median offsets are retained; inverse brackets are numerical, not certificates. Contracts/provenance/CPU evidence: docs/MATHCORE_STUDENT_T.md and output/mathcore-student-t-2026-10-31. Fisher F, other distributions, sampling/inference and broader standalone gates remain open.

- [x] Qualify central Fisher F density/logs, exact rounded moments, both tails and ordinary/log physical-coordinate inverses. Shared four-factor accumulation prevents moment overflow; normalized density avoids premature log overflow; guarded native small-shape inverses retain target differences hidden by rounded probability plateaus. All 1,249 Rust tests (18 new), 2,766 independent comparisons over 536 forward/moment inputs (150 exact-rational moment cases) and 184 inverse cases pass; twenty prior output sets remain byte-identical. Contracts, numerical limits, provenance and CPU/memory evidence: docs/MATHCORE_FISHER_F.md and output/mathcore-fisher-f-2026-10-31. Probability/model inventory, sampling/inference and broader standalone gates remain open.

- [x] Extract and qualify the native seeded RNG/sampling foundation: explicit SplitMix64 state/skip/snapshots, full-range integer rejection maps, exact binary Bernoulli thresholds, direct binomial trials, finite-grid uniform/normal/exponential samples and transactional batches/permutations. All 1,279 Rust tests (30 new), 20,931 independent comparisons across 8,950 sampling cases and 300 snapshot checks pass. Contracts, finite-grid limits, source-clone requirements, provenance and measured CPU/memory: docs/MATHCORE_SAMPLING.md and output/mathcore-sampling-2026-10-31. Efficient large-count/tail sampling, additional generators/parallel streams, process/model inventory and broader standalone gates remain open.

- [x] Qualify native one-sample, paired, pooled and Welch mean inference using exact shared moments/rationals and Student t probabilities, with reusable correctly rounded rational square-root offsets. Preserve exact integer inputs, paired differences/intermediate variance beyond floating range, tiny confidence widths and cumulative arithmetic/probability budgets. All 1,305 Rust tests (26 new), 1,519 independent comparisons over 223 mean-test inputs (ten precision repeats), and 631 exactly certified endpoint cases pass. Contracts, model assumptions, numerical limits and CPU evidence: docs/MATHCORE_MEAN_INFERENCE.md and output/mathcore-mean-inference-2026-10-31. Variance/ANOVA, broader inference/regression and all wider standalone gates remain open.

- [x] Qualify native variance-ratio tests and ordinary/Welch one-way ANOVA with shared exact moments/rationals, explicit dyadic weight precision and Fisher F probabilities. Add reusable nearest-even binary-significand rounding, preserve tiny confidence targets, and compose arithmetic/probability resource budgets. All 1,328 Rust tests (23 new), 780 independent exact rounding cases, and 2,695 comparisons over 268 variance/ANOVA inputs pass. Contracts, numerical limits and the 1,000-group CPU benchmark: docs/MATHCORE_GROUP_INFERENCE.md and output/mathcore-group-inference-2026-10-31. Scaled/log-F support, broader inference/regression and all wider standalone gates remain open.

- [x] Qualify all seven native multiple-testing corrections (None, Bonferroni, Holm, Hochberg, Hommel, BH and BY), preserving exact final rounding, input order/ties and explicit omitted-hypothesis semantics. BY uses directed harmonic bounds with exact fallback; sorting shares one fallible native kernel with order statistics. All 1,347 Rust tests (19 new), 8,506 independent comparisons across 1,548 inputs, 194 closed-Simes reference cases and million-family CPU cases pass. Contracts, dependence assumptions, rounding/resource guarantees and measurements: docs/MATHCORE_MULTIPLE_TESTING.md and output/mathcore-multiple-testing-2026-10-31. Regression/model extraction and all wider standalone gates remain open.

- [x] Qualify native ordinary/multiple/weighted linear regression models, scaled affine prediction and exact rational statistics of reported residuals. Reuse existing weighted SVD and moment kernels; retain rank-deficient/underdetermined fits, constant responses, explicit undefined diagnostics and cumulative matrix/arithmetic budgets. All 1,365 Rust tests (18 new), 3,819 independent comparisons across 181 datasets and one-CPU fit/prediction benchmarks pass. Contracts and evidence: docs/MATHCORE_LINEAR_REGRESSION.md and output/mathcore-linear-regression-2026-10-31. Coefficient uncertainty/intervals, robust/logistic/time-series models and wider standalone gates remain open.

- [x] Qualify native exact Theil–Sen robust line estimates, explicit joint/separate intercepts, median absolute residuals and reusable exact affine models. Shared rational arithmetic and cancellable in-place selection preserve mixed i64/f64 values, subnormals and predictions with out-of-range coefficients. Actual retained capacities, all candidate pairs and cumulative arithmetic work are bounded. All 1,388 Rust tests (23 new), 14,010 independent comparisons across 450 cases and five one-CPU workloads pass. Contracts and evidence: docs/MATHCORE_THEIL_SEN.md and output/mathcore-theil-sen-2026-10-31. Slope confidence intervals, Huber/logistic/time-series models and wider standalone gates remain open.

- [x] Qualify shared native real/complex row-scaled SVD factors and minimum-norm solves, preserving nonnegative scale exponents without squaring into binary64 weights. Existing weighted APIs delegate to the same factor/solve implementation and retain original-weight diagnostics. All 1,403 Rust tests (15 new), 412 independent row-scaled cases and one-CPU factor/reuse workloads pass; all 178 prior weighted and 181 linear-regression reference outputs remain byte-identical. Contracts and evidence: docs/MATHCORE_ROW_SCALED_LEAST_SQUARES.md and output/mathcore-row-scaled-least-squares-2026-10-31. This prepares native Huber regression; its model, convergence/inference and wider subject/CPU/queue gates remain open.

- [x] Qualify native fixed-threshold Huber regression with exact original-model loss/residuals/gradient, scaled coefficients and row multipliers, bounded IRLS and fresh stationarity-based termination. Shared rational square roots retain exponents and shared affine blending preserves tiny-step cancellation. All 1,430 Rust tests (27 new), 72 independent Huber cases, 353 exact scaled-root checks and three one-CPU workloads pass; all 10,958 prior square-root, 178 weighted and 181 linear-regression outputs remain byte-identical. A slow case honestly reaches the default 100-update limit and converges after 224 updates when explicitly allowed. Contracts and evidence: docs/MATHCORE_HUBER.md and output/mathcore-huber-2026-10-31. Broader robust variants, inference/logistic/time-series models and subject/CPU/queue gates remain open.

- [x] Qualify native binary logistic regression, exact column normalization and original-unit rational coefficients, paired/log Bernoulli probabilities, bounded damped Newton fits and exact sufficient separation certificates. Shared arithmetic stages charge rejected recoverable work; predictions recheck caller shape limits. All 1,457 Rust tests (27 new), 75 independent model cases, 412 scalar cases and three one-CPU workloads pass; previous 72 Huber models, 353 scaled square roots and 181 linear-regression outputs remain byte-identical. Contracts and evidence: docs/MATHCORE_LOGISTIC.md and output/mathcore-logistic-2026-10-31. Stationarity does not certify a finite MLE; complete separation detection, weighted/multinomial/inference/time-series and broader subject/CPU/queue gates remain open.

- [x] Qualify native exact unweighted/weighted covariance matrices and correctly rounded Pearson correlations with population, frequency and reliability estimators. Shared pair accumulators preserve mixed i64/f64 values, extreme weights and exact covariance beyond floating range; undefined correlations and cumulative shape/work/storage limits are explicit. All 1,479 Rust tests (22 new), 216 independent matrix cases, 3,328 exact comparisons, 1,654 rounded correlations and 591 positive-semidefinite/transformation checks pass; 390 prior pair, 1,188 weighted pair and 75 logistic probe outputs remain byte-identical. Four one-CPU workloads pass, including 10,000 rows and eight variables in a median 9.637 ms. Contracts and evidence: docs/MATHCORE_COVARIANCE.md and output/mathcore-covariance-2026-10-31. Multivariate streaming/windows, robust/shrinkage estimators, inference/time-series and broader subject/CPU/queue gates remain open.

- [x] Qualify native exact/rounded products and ratios, signed harmonic means, correctly rounded positive geometric means and exact mixed-value mode. Shared directed dyadic intervals certify product and geometric-mean rounding; fixed reciprocal bounds and shared Rational arithmetic preserve signed cancellation. The moments accumulator is reused unchanged. All 1,507 Rust tests (28 new), 1,242 independent exact-reference cases and three one-CPU workloads pass; 390 prior pair, 1,188 weighted pair and 216 covariance outputs remain byte-identical. Contracts and evidence: docs/MATHCORE_REDUCERS.md and output/mathcore-reducers-2026-10-31. Weighted/general power means, retained streaming state, remaining reduction inventory and broader subject/CPU/queue gates remain open.

- [x] Qualify native safeguarded scalar root and open-interval minimum/maximum searches using shared exact interpolation, typed callbacks and cumulative execution policy. Reports distinguish sampled zero, interval tolerance, resolution, evaluation/iteration limits and unresolved/non-unimodal objectives. Shared callback/error/work types retain calculus compatibility. All 1,534 Rust tests (27 new), 409 independent reference cases and 2,998 trace/accuracy checks pass; 538 prior derivative and 172 integration outputs remain byte-identical. Three one-CPU workloads pass. Contracts and evidence: docs/MATHCORE_SCALAR_SEARCH.md and output/mathcore-scalar-search-2026-10-31. Continuity/unimodality remain caller assumptions; general conditioning, certified interval-function analysis, multidimensional optimization and broader subject/CPU/queue gates remain open.

- [x] Qualify reusable native explicit RK4 stepping and adaptive step doubling with finite owned state, preallocated buffers, typed shared vector callbacks, exact represented-interval stage arithmetic, component tolerances, rejected-step recovery and explicit time/evaluation/trial/minimum-step limits. All 1,570 Rust tests (36 new), 246 independent exact/analytic cases and a reconstruction of 10,039 adaptive trials pass; 192 previous multivariate and 120 Hessian probe outputs remain byte-identical. Three one-CPU workloads pass. Contracts/evidence: docs/MATHCORE_ODE.md and output/mathcore-ode-2026-10-31. Local error estimates are heuristic; stiff/implicit/DAE, event/reset, dense-output, sensitivity, symplectic and PDE/FEM requirements remain open.

- [x] Qualify reusable real/complex LU workspaces around the single shared partial-pivoting kernel, including retained original/packed matrices and row swaps, invalidation after failed refactor, caller-buffer solving, storage/work policy rechecks and exact work replay. All 1,591 Rust tests (21 new) pass; owned and reusable interfaces each pass 1,081 independent exact-reference cases, and 246 prior ODE outputs remain byte-identical. Test instrumentation records zero heap allocations for 400 real/complex refactor-and-solve pairs after setup. CPU/storage comparisons are recorded without claiming universal speedup. Contracts/evidence: docs/MATHCORE_REUSABLE_LU.md and output/mathcore-reusable-lu-2026-10-31. This supplies a reusable linearization kernel; implicit/stiff/DAE solvers and broader subject/CPU gates remain open.

- [x] Qualify first-order implicit backward Euler with damped Newton and adaptive step doubling, analytic/numerical Jacobians, shared LU/differentiation work and storage admission, exact represented residuals, recoverable nonlinear stops and last-accepted state preservation. All 1,631 Rust tests (40 new) pass; 185 independent Fraction/Decimal cases verify 381 fixed implicit steps and 7,072 adaptive trials with 81,922 checks. Prior 246 explicit ODE, 192 Jacobian and 120 Hessian outputs remain byte-identical. Analytic stepping has zero measured solver allocations after setup; numerical Jacobian scratch and CPU costs are recorded explicitly. Contracts/evidence: docs/MATHCORE_IMPLICIT.md and output/mathcore-implicit-2026-10-31. This is partial M13; general stiff/DAE, events/sensitivities, sparse corrections, PDE/FEM and broad standalone gates remain open.

- [x] Qualify native semi-explicit index-one DAE consistent initialization and fixed/adaptive stepping through the shared Newton/LU/differentiation/controller kernels. Differential variables remain bitwise frozen during initialization; independent algebraic residual scales are never multiplied by the time step. Mixed-equation line search uses linearization row scales without relaxing final requested tolerances. All 1,656 Rust tests (25 new) pass; 124 independent Fraction/Decimal cases check 32 initializations, 214 fixed steps and 6,358 adaptive trials with 124,414 assertions. Prior 743 implicit/explicit/Jacobian/Hessian probe outputs remain byte-identical. Analytic initialization plus 100 steps measures zero heap allocation. Contracts/evidence: docs/MATHCORE_DAE.md and output/mathcore-dae-2026-10-31. This is partial M13; general residual-form DAEs, variable-order stiff methods, sparse corrections, events/sensitivities, PDE/FEM and broad standalone gates remain open.

- [x] Qualify native general residual DAE state and consistent initialization for F(t,y,yp)=0 with one selected state/derivative unknown per component, bitwise frozen counterpart values and atomic pair publication. Reuse the shared Newton/LU/differentiation kernels and cumulative resource budgets; normalize numerical mixed rows from the constructed linearization without loosening tolerances. All 1,678 Rust tests (22 new) pass; 162 independent Fraction/Decimal cases pass 25,629 checks, including 17,799 frozen-probe checks. The original 124 DAE cases are independently requalified and 743 other reference outputs remain byte-identical. One hundred analytic nonlinear initializations measure zero heap allocations after setup. Contracts/evidence: docs/MATHCORE_RESIDUAL_DAE.md and output/mathcore-residual-dae-2026-10-31. This is partial M13 initialization only; general residual stepping/adaptive integration, variable-order stiff methods, sparse corrections, events/sensitivities, PDE/FEM and broad standalone gates remain open.

- [x] Qualify general residual-form fixed/adaptive backward-Euler integration using the shared native Newton/LU/differentiation/controller kernels. Derivatives use exact represented differences/ratios; analytic corrections combine h*Fy+Fyp before row scaling; accepted-derivative prediction supports nonlinear derivative domains. Time/y/yp publish atomically, with the fine derivative taken from the actual final substep. All 1,710 Rust tests (32 new), 294 independent Fraction/Decimal cases, 488 fixed steps and 1,650 adaptive trials pass, including 112,660 exact represented-derivative checks. All 1,029 earlier reference outputs remain byte-identical. Analytic 100-step and adaptive workloads measure zero heap allocations after setup. Contracts/evidence: docs/MATHCORE_RESIDUAL_STEPPING.md and output/mathcore-residual-stepping-2026-10-31. This completes the first-order residual stepping milestone only; higher/variable-order stiff integration, events/reset, dense output, sensitivities/adjoints, sparse corrections, PDE/FEM and broad standalone gates remain open.

- [x] Qualify native fixed BDF orders 1–5 with exact represented-grid coefficients, bounded accepted history and the shared residual/Newton/LU kernels. Whole history and time/y/yp publish only after a successful step. All 1,733 Rust tests (23 new), 577 independent Fraction cases and 8,115 exact callback-derivative comparisons pass; all 1,323 earlier reference outputs remain byte-identical. Order-convergence, stiff negative-real decay, nonlinear derivative domains, analytic/numerical Jacobians and rollback checks pass. Prepared derivative application allocates nothing; per-step coefficient preparation still allocates and is explicitly measured. Contracts/evidence: docs/MATHCORE_BDF.md and output/mathcore-bdf-2026-10-31. This is partial M13: adaptive step/order selection, qualified high-order startup, events/dense output/sensitivities, sparse/PDE/FEM and broad CPU/standalone gates remain open.

- [x] Qualify native adaptive BDF orders 1–5 with exact history-dependent full/two-half error coefficients, persistent order/retry state, cumulative work/callback/storage limits and atomic fine-history publication. All 1,754 Rust tests (21 new), 318 independent cases, 3,185 adaptive trials and 108,590 exact callback-derivative comparisons pass; all 1,900 prior reference outputs remain byte-identical. Twelve strict residual requests explicitly require NonlinearLimit; an adjacent-float proof explains the discovered infeasibility and admission is never loosened. Prepared estimation allocates nothing; rational preparation and three solves per trial remain efficiency work. Contracts/evidence: docs/MATHCORE_ADAPTIVE_BDF.md and output/mathcore-adaptive-bdf-2026-10-31. Partial M13 only: higher-order prediction, coefficient/factor reuse, broad stability/CPU/queue gates, dense output/events/sensitivities, sparse/PDE/FEM and standalone completion remain open.

- [x] Qualify exact BDF accumulator specialization and prepared denominator-scale reuse. Derivatives retain the full 3,000-bit coefficient range; analytic corrections select the smaller shared accumulator only with a proved range bound. All 1,758 Rust tests (four new), 1,720 exact correction cases, 344 exact derivative cases and the 895 independent fixed/adaptive BDF cases pass. All 2,218 earlier reference cases retain numerical values, callback traces and statuses; only BDF charged-work counters change. Five-repeat one-CPU before/after measurements cover dimensions 1/8/32 and fixed orders/adaptive caps 1/3/5. Contracts/evidence: docs/MATHCORE_BDF_EFFICIENCY.md and output/mathcore-bdf-efficiency-2026-10-31. Higher-order prediction, time-grid coefficient/factor reuse, dense output/events and broad CPU/queue/standalone gates remain open.

- [x] Qualify native exact first-order/equal-spacing BDF coefficient preparation. Shared exact sums distinguish represented intervals even when rounded gaps agree; reusable small integer coefficients preserve general rational fallback, early bounds and timestamp bits. All 1,764 Rust tests (six new), 483 independent preparation cases, the existing 895 fixed/adaptive cases and 2,064 exact product/derivative fixtures pass. All 2,218 previous reference cases retain numerical results, callback traces and statuses; resource accounting reflects less preparation. Warmed 1,000-preparation CPU workloads and fixed/adaptive solver comparisons are recorded. Contracts/evidence: docs/MATHCORE_BDF_PREPARATION.md and output/mathcore-bdf-preparation-2026-10-31. General grid/factor reuse, higher-order prediction, events/dense output and broad CPU/queue/standalone gates remain open.

- [x] Qualify reusable exact global Hermite value/derivative evaluation with arbitrary consecutive derivative multiplicities, both node directions and extrapolation. Native adjoint confluent divided differences prepare bounded integer weights; shared exact products provide once-rounded scalar/component evaluation and allocation-free atomic caller-buffer application. All 1,782 Rust tests (18 new), 468 independent confluent moment-system cases and 1,380 exact-reference checks pass; 2,218 prior solver outputs remain byte-identical. One thousand two-component applications allocate nothing after preparation. Contracts/evidence: docs/MATHCORE_HERMITE_EVALUATION.md and output/mathcore-hermite-evaluation-2026-10-31. BDF high-order predictor integration, conditioning/error bounds, general grid/factor reuse, dense output/events and broad CPU/queue/standalone gates remain open.

- [x] Qualify higher-order BDF initial prediction through the shared native Hermite kernel, with whole-vector overflow fallback and one linear retry for initial domain/derivative failure. Failed preparation and recovery retain cumulative work/callback/storage limits; fatal output/callback errors do not retry and accepted history remains atomic. All 1,799 Rust tests (17 new), 577 fixed and 318 adaptive independent cases pass. Independent confluent moments verify 9,549 guesses / 13,815 components; 468 Hermite cases pass and 1,323 non-BDF outputs remain byte-identical. Twelve strict residual requests still return NonlinearLimit. Thirty-six five-repeat CPU workloads and independent benchmark endpoints expose large-system iteration savings and small-system preparation overhead; no universal speedup is claimed. Contracts/evidence: docs/MATHCORE_BDF_PREDICTION.md and output/mathcore-bdf-prediction-2026-10-31. General coefficient/factor reuse, dense output/events, broad conditioning, CPU/queue and standalone gates remain open.

- [x] Qualify exact uniform-grid Hermite prediction preparation through independently derived moment coefficients and shared full-range interval recognition. Finite error-free two-term differences avoid costly full accumulators on ordinary grids; extreme intermediates retain exact fallback. All 1,809 Rust tests (10 new), 606 dedicated preparation cases, 468 general Hermite cases, 895 BDF cases and 9,549 independent predictor checks pass. All 2,686 prior numerical reference cases preserve values/traces/statuses; resource costs may change. Sixty five-repeat CPU workloads show faster uniform preparation and roughly halved scalar order-3/5 step timings; an extreme irregular timing regression remains explicitly recorded. Contracts/evidence: docs/MATHCORE_HERMITE_PREPARATION.md and output/mathcore-hermite-preparation-2026-10-31. General storage/factor reuse, dense output/events, broader CPU/queue and standalone gates remain open.

- [x] Qualify accepted BDF interval output at full orders 1–5, retaining all q+1 rows before history rollover and both fine adaptive substeps. Values and derivatives reuse native exact Hermite evaluation; resource/callback failures preserve prior history/output. All 1,829 Rust tests (20 new), 10,990 independent Lagrange queries across 248 cases, and 2,686 byte-identical prior reference outputs pass. Decimal trajectory/convergence checks cover both directions and three adaptive tolerances. Thirty-six five-repeat CPU workloads verify no added capture allocations and zero endpoint-query allocations; interior-query preparation still allocates. Contracts/evidence: docs/MATHCORE_BDF_DENSE_OUTPUT.md and output/mathcore-bdf-dense-output-2026-10-31. Event localization/reset, sensitivities/adjoints, sparse/PDE/FEM, preparation reuse and broad CPU/queue/standalone gates remain open.

- [x] Qualify bounded multi-condition event scans over accepted BDF substeps, with physical crossing directions, explicit start/shared-endpoint policies, cached endpoint samples, atomic sorted publication and bracket evidence for unresolved searches. Dense interpolation and scalar root search share cumulative work/storage limits. All 1,849 Rust tests (20 new), 372 independent cases / 744 events / 9,476 exact callback-state checks pass; 3,095 prior reference outputs and 10,990 dense queries remain byte-identical. Thirty-six five-repeat CPU workloads verify allocation-free no-crossing endpoint scans while exposing interior-preparation allocations. Contracts/evidence: docs/MATHCORE_BDF_EVENTS.md and output/mathcore-bdf-events-2026-10-31. This is accepted-snapshot scanning only: integrator event observation, terminal stop/consistent reset, sensitivity/adjoint, sparse/PDE/FEM, preparation reuse and broad standalone gates remain open.

- [x] Qualify native adaptive BDF event observation, explicit terminal selection, residual-consistent interior stops and transactional current-time resets. Shared trial scans publish with accepted history; stop/reset restarts order one and clears untraversed dense output. All 1,868 Rust tests (19 new), 192 independent cases / 4,597 checks and six five-repeat CPU workloads pass. The campaign includes 180 successful trajectory/reset cases and 12 explicit tight-tolerance stagnation cases matching the event-free controller; no residual admission is weakened. All 3,095 preceding reference outputs, 10,990 dense queries and 372 event-scan cases remain byte-identical. Contracts/evidence: docs/MATHCORE_BDF_EVENT_INTEGRATION.md and output/mathcore-bdf-event-integration-2026-10-31. Pre-trial initial-time events, repeated/overlapping-root policy, precision-floor diagnostics, sensitivity/adjoint, sparse/PDE/FEM, preparation/factor reuse and broad standalone gates remain open.

- [x] Qualify pre-trial initial-time BDF point observation, including zero-span calls, immediate checked stops, exact-zero policy and optional physical-slope filtering. Initial consistency is reused on continuation; shared callback/work/storage limits preserve prior publication on failure. All 1,891 Rust tests (23 new), 462 independent cases / 576 initial zeros / 6,949 checks and 36 five-repeat CPU workloads pass. Repeated point observations allocate zero times after setup. All 3,095 older reference outputs, 10,990 dense queries, 372 event scans and 192 prior integration cases remain byte-identical. Contracts/evidence: docs/MATHCORE_INITIAL_EVENTS.md and output/mathcore-initial-events-2026-10-31. Zero slopes at directed initial roots remain explicitly unresolved; general simultaneous/overlapping-event/reset ordering, near-zero rearming, precision-floor diagnostics, preparation/factor reuse, remaining subjects and standalone gates remain open.

- [x] Qualify bounded BDF event ordering with explicit connected overlap groups, union/all-member intersection evidence, sampled-zero point enclosures and deterministic traversal order retaining every candidate. All 1,913 Rust tests (22 new), 598 independent graph-oracle cases / 15,940 candidates / 8,970 checks and 32 five-repeat CPU workloads pass. Repeated ordering allocates zero times after setup. All 3,095 older numerical outputs, 10,990 dense queries, 372 scans, 192 event integrations and 462 initial-event cases remain byte-identical. Contracts/evidence: docs/MATHCORE_EVENT_ORDERING.md and output/mathcore-event-ordering-2026-10-31. Connected overlap does not prove simultaneous events; automatic refinement, explicit rearming/reset ordering, higher-order initial direction, precision-floor diagnostics and broad standalone gates remain open.

- [x] Qualify explicit BDF event exit-band rearming, selected-stop disarming and atomic Retain/ArmAll/DisarmAll/Set reset policies. Only ordered initial/cell samples change eligibility; early stops discard rearming observed in the untraversed future. All 1,948 Rust tests (35 new), 714 independent cases / 7,928 checks and 33 five-repeat CPU workloads pass, with zero post-setup allocations in consistent point reset/stop workloads and no extra scan allocations versus matched ungated grids. Interior interpolation preparation still allocates and remains the next efficiency task. Previous numerical, dense, event, initial and ordering outputs/work remain byte-identical. Contracts/evidence: docs/MATHCORE_EVENT_REARMING.md and output/mathcore-event-rearming-2026-10-31. Finite sampling can miss within-cell exits/reentries; physical simultaneous-reset ordering, overlap refinement, higher-order initial directions, precision-floor diagnostics and broad standalone gates remain open.

- [x] Qualify immutable reusable exact Hermite node geometry and separately owned query weights, sharing the existing arithmetic/application kernels. Mixed multiplicities, either node direction, raw derivatives, extreme scales, exact output rounding and atomic caller-buffer publication retain explicit work/storage/limb limits. All 1,967 Rust tests (19 new), 468 original exact cases and 900 repeated-query cases (873 cache reuses) pass; previous numerical/dense/event results and work remain byte-identical. Forty five-repeat CPU workloads show lower repeated preparation work/allocations, with separate grid setup/storage costs; query preparation still allocates. Contracts/evidence: docs/MATHCORE_HERMITE_GRID.md and output/mathcore-hermite-grid-2026-10-31. Dense/event adoption, further preparation/factor reuse, conditioning and broad subject/CPU/queue/standalone gates remain open.

- [x] Qualify persistent exact interpolation-grid reuse in BDF dense queries, event scans and checked stop reconstruction. Changed node bits/order invalidate each substep cache; new state samples never reuse old values. Dynamic observer storage is admitted before every later adaptive trial, while cumulative failed-setup work and atomic public output/events are preserved. All 1,987 Rust tests (20 new), 10,990 independently checked cached queries and 1,740 event/integration/initial/rearming cases pass. Previous numerical values, statuses and traces remain identical; charged work reflects cache state. Ninety-nine paired five-repeat CPU workloads include an explicit one-off switch and the original alternating-direction scan fixtures: four/sixteen-cell allocation totals fall from 46,800/234,000 to 42,600/193,800. Some small timing cases regress; no universal speedup or allocation-free query preparation is claimed. Contracts/evidence: docs/MATHCORE_DENSE_REUSE.md and output/mathcore-dense-reuse-2026-10-31. Remaining query allocations, factor reuse, event semantics, sensitivities, PDE/FEM and broad subject/CPU/queue/standalone gates remain open.

## Standalone inverse calculation control — verified, 2026-11-01

- [x] Qualify caller-owned work/cancellation inside Gamma/Beta inverse search,
  series and quadrature with arithmetic rollback and retained consumed work.
  182 crate tests, 1,498 independently sourced answers and 656 shared assertions
  pass; native no-FPU qualification passes with seven controls. Contract:
  `docs/MATHCORE_INVERSE_CONTROL.md`; evidence:
  `output/mathcore-inverse-control-2026-11-01`. Complete elementary/limb accounting,
  processor/queue routing, latency and all mandatory subject/platform gates
  remain open. Standalone mathcore is incomplete; no application/OS integration.

## Controlled processor and CPU queue — verified, 2026-11-01

- [x] Compose qualified inverse controls into processor batches and the existing
  CPU queue, with explicit shared work and retained-result memory reservations.
  63 focused Rust tests, 1,498 independent dispatch answers and 321 queued
  answers pass; native no-FPU processor execution passes 1,498 cases with
  92 shared assertions and seven controls. Contract:
  `docs/MATHCORE_CONTROLLED_QUEUE.md`; evidence:
  `output/mathcore-controlled-queue-2026-11-01`. Complete command/subject scope,
  elementary/limb accounting, fairness, hardware and performance gates stay open.

## Controlled forward probabilities — verified, 2026-11-01

- [x] Add controlled forward Gamma/Beta probability calculations to the shared
  scalar/processor/queue path. 90 focused Rust tests, 5,203 independent
  forward results on all three paths, 285 shared control assertions and the full
  scoped no-FPU run with seven controls pass. Contract:
  `docs/MATHCORE_PROBABILITY_CONTROL.md`; evidence:
  `output/mathcore-probability-control-2026-11-01`. Full command and subject scope,
  elementary/limb accounting, performance and physical-platform gates remain open.

## Controlled elementary calculations — verified, 2026-11-02

- [x] Add exp/log/increment range/series control, selected probability shortcut
  composition and shared processor/queue support. 368 focused Rust tests,
  5,329 independent elementary answers on scalar/processor/queue paths and
  734 shared control assertions pass. Same-source no-FPU execution passes
  12,030 elementary/forward/inverse answers with seven controls. Contract:
  `docs/MATHCORE_ELEMENTARY_CONTROL.md`; evidence:
  `output/mathcore-elementary-control-2026-11-02`. Full normalizer/power/limb work
  accounting, command/subject, fairness, performance and physical-platform gates
  stay open. Standalone mathcore remains incomplete; integration is deferred.

## Probability normalization controls — verified, 2026-11-02

- [x] Forward the same work/cancellation ledger through scaled transforms,
  normalizers, complements, recurrence products and inverse targets.
  371 distinct focused Rust tests, 81 helper cases/2,133 assertions and
  1336 shared public assertions pass. Same-source no-FPU execution passes
  12,030 independent answers with seven controls. Contract:
  `docs/MATHCORE_NORMALIZATION_CONTROL.md`; evidence:
  `output/mathcore-normalization-control-2026-11-02`. Full primitive accounting,
  other commands/subjects, fairness, timing, ISA and physical-platform gates
  remain open. Standalone mathcore is incomplete; integration stays deferred.

## Power and exact probability controls — verified, 2026-11-02

- [x] Share work/cancellation controls through integer/real powers, Beta power
  shortcuts, exact probability polynomial preparation and Gamma leading bounds.
  The existing processor and queue now support 22 controlled operation variants.
  377 distinct focused Rust tests, 7,208 independent power answers,
  4088 shared public assertions and six helper paths/696 counted assertions pass.
  Same-source no-FPU execution passes 19,238 reference cases and seven controls.
  Contract: `docs/MATHCORE_POWER_CONTROL.md`; evidence:
  `output/mathcore-power-control-2026-11-02`. Full primitive/resource accounting,
  remaining commands/subjects, queue fairness, x87/SIMD and physical-platform
  gates remain open. Standalone mathcore is incomplete; integration stays deferred.

## Gamma/Beta integral controls — verified, 2026-11-02

- [x] Qualify shared controls for lower/upper Gamma/Beta integrals and their
  logarithms, including complete-function endpoints and exact-ratio preparation.
  409 distinct focused Rust tests, 9,637 independent integral answers,
  20056 shared assertions and 24 helper cases/660 counted assertions pass.
  Same-source no-FPU execution passes 28,875 reference cases and seven controls.
  Contract: [MATHCORE_INTEGRAL_CONTROL.md](docs/MATHCORE_INTEGRAL_CONTROL.md).
  Evidence: output/mathcore-integral-control-2026-11-02. This extends the processor
  and CPU queue to 30 controlled variants; full primitive accounting, other
  commands/subjects, fairness, latency and platform/aggregate gates remain open.

Standalone mathcore remains incomplete; ModernBASIC integration stays deferred.

## Angle controls — verified, 2026-11-02

- [x] Qualify cumulative work/cancellation controls for radian and inverse
  trigonometry, pi/degree functions, sinc and angle conversion. All 16 new
  controlled variants share internal reduction, series, roots and precision work.
  375 distinct focused Rust tests, 44,907 independent angle answers,
  59,930 shared assertions and 27 helper cases/765 assertions pass.
  Same-source no-FPU execution passes 73,782 reference cases and seven controls.
  The source-derived 12,800-step bound covers the 16 angle operations, not
  CPU cycles, callback cost or wall-time guarantees.
  Contract: [MATHCORE_ANGLE_CONTROL.md](docs/MATHCORE_ANGLE_CONTROL.md).
  Evidence: output/mathcore-angle-control-2026-11-02. There are now 46 supported
  controlled operation variants. Complete primitive accounting, other
  commands/subjects, fairness, latency and platform/aggregate gates remain open.

Standalone mathcore remains incomplete; ModernBASIC integration stays deferred.

## Hyperbolic and reciprocal controls — verified, 2026-11-02

- [x] Qualify shared work/cancellation controls for six hyperbolic and twelve
  reciprocal circular/hyperbolic functions, including their principal inverses.
  349 distinct focused Rust tests, 40,607 independent answers,
  79,562 shared assertions and 24 helper cases/696 assertions pass.
  Same-source no-FPU execution passes 45,054 reference cases and seven controls.
  The source-derived 12,800-step bound covers these 18 operations; fixed-limb
  work, callback cost and wall-time guarantees remain separate.
  Contract: [MATHCORE_HYPERBOLIC_CONTROL.md](docs/MATHCORE_HYPERBOLIC_CONTROL.md).
  Evidence: output/mathcore-hyperbolic-control-2026-11-02. There are now 64
  controlled operation variants; signed complete Gamma, other functions/subjects,
  primitive accounting, fairness, latency and platform/aggregate gates stay open.

Standalone mathcore remains incomplete; ModernBASIC integration stays deferred.

## Complete Gamma controls — verified, 2026-11-02

- [x] Qualify cumulative work/cancellation controls for complete signed Gamma,
  reciprocal Gamma, log-Gamma, signed log-Gamma, shifted log-Gamma and
  scaled/log-scaled Gamma. Shared reflection, recurrence, normalization and
  series retain the caller ledger; signed results and context publish atomically.
  319 distinct focused Rust tests, 10,044 independent answers,
  42,302 shared assertions and 27 helper cases/837 assertions pass.
  Same-source no-FPU execution passes 10,886 answers and seven controls.
  The reviewed 24,000-step bound covers these seven operations, with fixed-limb
  internals grouped. Contract: [MATHCORE_GAMMA_CONTROL.md](docs/MATHCORE_GAMMA_CONTROL.md).
  Evidence: output/mathcore-gamma-control-2026-11-02. There are now 71 controlled
  processor variants; remaining functions/subjects, full primitive accounting,
  fairness, latency, ISA and physical/aggregate platform gates stay open.

Standalone mathcore remains incomplete; ModernBASIC integration stays deferred.

## Scientific scalar controls — verified, 2026-11-02

- [x] Qualify cumulative work/cancellation for five exponential/logarithmic
  base operations, cube/integer-index roots, four error functions, four Gamma
  coordinates and complete Beta/log-Beta. Exact preprocessing, wide-index
  division, recurrence and series share the ledger; failures preserve contexts.
  349 distinct focused Rust tests, 38,267 independent answers,
  87,222 shared assertions and 33 helper cases/867 assertions pass.
  Same-source no-FPU execution passes 39,727 answers and seven controls.
  A reviewed 100,000-step ceiling covers these 17 operations, with fixed-limb
  internals grouped. Contract: [MATHCORE_SPECIAL_CONTROL.md](docs/MATHCORE_SPECIAL_CONTROL.md).
  Evidence: output/mathcore-special-control-2026-11-02. Controlled processor
  coverage is 88 of 98 variants. Primitive controls, broader subjects,
  fairness, latency, ISA and physical/aggregate platform gates remain open.

Standalone mathcore remains incomplete; ModernBASIC integration stays deferred.

## Primitive arithmetic controls — verified, 2026-11-02

- [x] Qualify controlled primitive arithmetic, comparisons, format conversions,
  integral rounding and signed/unsigned conversions across all five formats.
  Division, roots and remainder loops share cumulative work; interruption and
  panic preserve the original context. Completed invalid conversions retain
  ordinary IEEE flags; failed atomic batches roll back.
  334 distinct focused tests, 12,747 independent answers,
  38,834 shared assertions and 47 helper cases/11,996 assertions pass.
  Same-source no-FPU execution passes 12,747 answers and seven controls.
  Reviewed primitive ceiling: 9,000 algorithm steps, grouping fixed-limb work.
  Contract: [MATHCORE_PRIMITIVE_CONTROL.md](docs/MATHCORE_PRIMITIVE_CONTROL.md).
  Evidence: output/mathcore-primitive-control-2026-11-02.
  All 98 current processor variants have controlled routes; scientific subcall
  accounting, full subjects, isolation, fairness/latency and ISA/platform gates
  remain separately open.

Standalone mathcore remains incomplete; ModernBASIC integration stays deferred.

## Independent mathcore consumer — verified, 2026-11-02

The unchanged standalone crate builds and runs without ModernBASIC source,
configuration, installed providers or an existing Cargo package cache.
329 selected Rust tests, eight binary-only public-API workloads and two
negative controls pass. Dependency and current independent-consumer gates in
docs/MATHCORE_TASKS.md are closed; broader mathematical, execution, ISA and
physical-platform requirements remain open. No ModernBASIC integration.

Contract: docs/MATHCORE_ISOLATED_CONSUMER.md.
Evidence: output/mathcore-isolated-consumer-2026-11-02/report.json.


## CPU queue individual limits — verified, 2026-11-02

Standalone mathcore now supports optional per-job memory, work and command-count
limits alongside aggregate pool reservations. 272 focused tests and
112 verified CPU batch samples pass. Existing constructor behavior is preserved;
full fairness, resource/latency, mathematical and platform gates remain open.
Contract: docs/MATHCORE_QUEUE_POLICY.md.
Evidence: output/mathcore-queue-policy-2026-11-02/host-report.json.


## Standalone Fourier foundation — verified, 2026-11-02

Reusable native FFT/DFT plans and workspaces now exist in mathcore's signal
subject. 35 focused Rust tests, 236 independent cases/18,648 component checks,
zero-allocation instrumentation and 84 CPU samples pass. Runtime BASIC source
and the installed application are unchanged. Full M17 signal/audio work,
software no-FPU composition and aggregate standalone gates remain open.
Contract: docs/MATHCORE_FOURIER.md; donor ledger: docs/MATHCORE_SIGNAL_SYMBOLS.json.
Evidence: output/mathcore-fourier-2026-11-02/reference-report.json.


## Standalone convolution and correlation — verified, 2026-11-02

Full linear real/complex convolution, cross-correlation and full autocorrelation
now have reusable exact-direct and explicit approximate-FFT APIs. 26 focused
tests, 768 independent cases/51,632 component checks and 90 CPU samples pass.
900 prepared invocations allocate nothing. All 236 prior Fourier reference
cases remain byte-identical after sharing the execution ledger.
M17 windows/spectra, bounded-lag/streaming modes, no-FPU composition and all
aggregate standalone gates remain open. No ModernBASIC integration.
Contract: docs/MATHCORE_CONVOLUTION.md; evidence:
output/mathcore-convolution-2026-11-02/reference-report.json.


## Standalone windows and calibrated spectra — verified, 2026-11-02

Six symmetric/periodic window families, gain/bandwidth calibration and reusable
real/complex periodograms now support explicit sides and density/spectrum units.
43 focused tests, 476 independent cases/13,099 checks and 60 CPU samples pass;
1,300 prepared invocations allocate nothing. Runtime source is unchanged.
M17 bounded-lag/energy conveniences, broader estimators, streaming/filters,
software no-FPU composition and aggregate standalone gates remain open.
Contract: docs/MATHCORE_SPECTRA.md. Evidence:
output/mathcore-spectrum-2026-11-02/reference-report.json.


## Signal foundation extraction — verified, 2026-11-02

Bounded lag ranges and exact streaming energy, mean power, RMS and uniform-time
integrals complete the first M17 foundation-extraction item in MATHCORE_TASKS.md.
61 focused tests, 822 independent cases/13,178 checks and 96 CPU samples pass;
768 earlier full-product cases remain byte-identical. Allocation tests cover
900 mathematical calls and 100 clears with zero allocations.
Broader P06 spectral/time-frequency/wavelet work, filters/resampling/streaming,
synthesis, no-FPU composition and aggregate engine gates remain open.
Contract: docs/MATHCORE_SIGNAL_LEVELS.md. Closure evidence:
output/mathcore-signal-levels-2026-11-02/foundation-closure-audit.json.


## Standalone transactional streaming filters — verified, 2026-11-02

FIR, original-coefficient IIR/SOS, snapshot restoration and whole-block rollback
are independently qualified, including one-pole designs and explicit pole
diagnostics. 75 focused tests and 306 independent cases/20,088 checks pass;
1,000 processing/reset calls allocate nothing and 54 CPU samples are recorded.
M17 resampling, state-variable/modulated filters, broader design/synthesis,
no-FPU composition and aggregate engine gates remain open.
Contract: docs/MATHCORE_STREAM_FILTERS.md. Evidence:
output/mathcore-stream-filters-2026-11-02/reference-report.json.


## Standalone streaming sinc resampling — verified, 2026-11-02

Integer sample timing, bounded cached/on-demand kernels, exact weighted
accumulation and transactional streaming/flush/reset are independently qualified.
90 focused tests, 161 reference cases/75,630 exact checks and 25,399 independent
coefficients pass. 1,200 prepared calls allocate nothing; 90 CPU measurements
record storage and reuse. Original runtime is unchanged. M17's remaining
modulation/synthesis/resampling, no-FPU and aggregate engine gates stay open.
Contract: docs/MATHCORE_SINC_RESAMPLING.md. Evidence:
output/mathcore-sinc-resampling-2026-11-02/reference-report.json.


## Standalone shared state-variable filter — verified, 2026-11-02

Low/high/band-pass and notch modes share a range-safe exact nodal solve and
transactional state. Constant/sample controls, prepared coefficients, state
restoration and atomic caller-buffer/in-place processing are qualified.
104 focused tests, 157 independent cases/32,464 output-state checks and
1,400 coefficients pass; 600 prepared calls allocate nothing and 54 CPU samples
are recorded. Original BASIC and synthesizer adapters remain unchanged.
M17 synthesis/remaining resampling/spectral, no-FPU and aggregate gates stay open.
Contract: docs/MATHCORE_STATE_VARIABLE_FILTER.md. Evidence:
output/mathcore-state-variable-filter-2026-11-02/reference-report.json.


## Standalone exact phase and oscillators — verified, 2026-11-02

Fixed 1074-bit fractional phase, raw/PolyBLEP waveforms, seeded noise/hold and
cyclic-table interpolation now have independent transactional Rust APIs.
118 focused tests, 124 phase cases/11,160 checks and 217 waveform cases pass,
including 19,041 exact waveform/state checks and 834 sine checks.
600 prepared calls allocate nothing; 90 CPU samples are recorded.
Envelope release-overlap behavior and retained MIT notice were inspected;
envelopes/physical models, broader audio and no-FPU/aggregate gates remain open.
Contract: docs/MATHCORE_OSCILLATORS.md. Evidence:
output/mathcore-oscillators-2026-11-02/reference-report.json.

## Standalone sample-count envelopes — verified, 2026-11-02

Reusable exact linear/approximate exponential segments and delay/attack/hold/
decay/sustain/release now have typed Rust APIs, captured release, explicit
retrigger choices, snapshots, bounded skipping and atomic event blocks.
132 focused tests, 1,049 segment cases and 121 independent event schedules pass:
7,386 exact values, 4,262 approximate values and 1,089 state/counter checks.
700 prepared calls allocate nothing; 54 CPU samples and an answer-preserving
guarded integer-ratio optimization are recorded.
Contract: docs/MATHCORE_ENVELOPES.md. Evidence: output/mathcore-envelopes-2026-11-02.
Physical models, dynamics/spatial math, remaining M17 and no-FPU/aggregate gates
remain open. No ModernBASIC integration, installer or OS image was made.

## Standalone fractional-delay waveguide — verified, 2026-11-02

Causal integer/fractional delay, signed amplitude-per-loop feedback, explicit
one-pole loss, phase-compensated tuning, caller excitation, snapshots and atomic
blocks now have reusable Rust APIs. Whole retained-state metrics preserve exact
energy and correctly rounded RMS; there is no hidden silence cutoff.
148 focused tests, 556 recurrence cases, 128 tuning cases and 72 CPU samples pass.
Independent checks cover 30,474 exact values, 6,672 metrics and 1,668 frame counts.
600 prepared calls allocate nothing. Source/contract: docs/MATHCORE_WAVEGUIDE.md;
evidence: output/mathcore-waveguide-2026-11-02.
Modal banks, richer physical/excitation models and aggregate/no-FPU gates remain
open. No ModernBASIC integration, installer or OS image was made.

## Standalone causal modal resonators — verified, 2026-11-02

Reusable complex-pole banks now have explicit drive/readout, frequency/decay/
phase design, exact coefficient stability, resolution diagnostics, snapshots,
whole-state metrics and atomic state/output mixing.
171 focused tests pass; 172 independent bank cases cover 10,170 exact values,
2,064 metrics and 516 frame counts. 302 design cases include 284 accepted designs
and 18 correct resolution errors. 600 prepared calls allocate nothing; 54 CPU
samples pass. Earlier subnormal direction distortion was identified by independent
references and corrected with an exact radius-scaled admission check.
Contract: docs/MATHCORE_MODAL.md; evidence: output/mathcore-modal-2026-11-02.
Dedicated excitation helpers, richer physical/dynamics/spatial math and aggregate/
no-FPU gates remain open. No ModernBASIC integration, installer or OS image was made.

## Standalone finite excitation — verified, 2026-11-02

Harmonic/pluck/noise excitation now uses explicit coefficients, exact integer
phase, alias rules and reproducible seed/window controls. Gain and sampled-peak
normalization retain overrange terms and tiny cancellation until final rounding.
190 focused tests pass, including 19 new tests. Independent qualification covers
246 excitation cases (five expected errors), 1,249 bit-exact values, 1,311
approximate values, 723 metadata checks and 88 pluck coefficients.
300 prepared calls allocate nothing; 72 CPU samples pass. Generated arrays also
drive the reusable waveguide and modal banks in tests.
Contract: docs/MATHCORE_EXCITATION.md; evidence: output/mathcore-excitation-2026-11-02.
Gain/dynamics/loudness/spatial and richer physical/spectral/resampling math,
mandatory subjects and aggregate CPU/no-FPU gates remain open.
No ModernBASIC integration, installer or OS image was made.

## Standalone gain and peak dynamics — verified, 2026-11-02

Explicit amplitude/power decibels, range-safe gain application and exact
hard/soft-knee compression now support reusable feed-forward peak processing.
Attack/release update weights/time constants, external sidechain, makeup,
configuration-matched snapshots and whole-block rollback are explicit.
211 focused tests pass. Independent references cover 879 scalar cases,
85 streaming cases/255 executions, 520 exact curve values, 866 exact states,
566 approximate states and 1,432 outputs; 52 expected errors are checked.
700 prepared calls allocate nothing; 36 CPU samples pass.
Contract: docs/MATHCORE_DYNAMICS.md; evidence: output/mathcore-dynamics-2026-11-02.
Finite attack is not a guaranteed ceiling. RMS/linked/lookahead/multiband
dynamics, gates/expanders, loudness/spatial and aggregate CPU/no-FPU/subject
gates remain open. No ModernBASIC integration, installer or OS image was made.

## Standalone rolling RMS and linked dynamics — verified, 2026-11-02

Finite-window detector energies remain exact as old frames leave. Startup uses
an explicit available-frame or zero-padded divisor; linking uses maximum-channel
RMS or combined mean power. A shared compressor gain is applied to interleaved
channels with external-sidechain support and transactional snapshots/output.
229 focused tests pass. Independent references cover 158 RMS and 72 linked
cases/690 executions: 2,835 exact RMS values, 690 metadata checks, 686 gain-state
checks and 1,798 output checks. Previous dynamics reference output is byte-identical.
800 prepared calls allocate nothing; 72 CPU samples pass.
Contract: docs/MATHCORE_LINKED_RMS.md; evidence: output/mathcore-linked-rms-2026-11-02.
Lookahead/ceiling, weighting/hold/multiband/gates/expanders, loudness/spatial and
aggregate subject/CPU/no-FPU gates remain open.
No ModernBASIC integration, installer or OS image was made.

Lookahead limiter in progress (2026-11-02): its private transactional monotone
peak queue passes four focused tests and 30,720 independent window-maximum
comparisons, including failure atomicity and counter boundaries. The queue is
not yet wired into the crate. Delay, release, ceiling scaling, flushing and full
limiter qualification remain open. Evidence: output/mathcore-lookahead-2026-11-02.
