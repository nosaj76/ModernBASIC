# ModernBASIC 0.8.2 — Math Engine & Language Bugs

Found while building Game of Life: Mathematical Analysis Edition
and Hamurabi Remix. Date: August 22, 2026.

## Language / Syntax Issues

### 1. Inline and block ELSE — resolved

Single-line `IF condition THEN statement ELSE statement` now selects exactly one
branch, including colon-separated branch tails and numeric line targets.
Multi-line `IF...THEN`, `ELSE`, and `END IF`/`ENDIF`/`IFEND` blocks are validated,
nest correctly, and diagnose orphaned or duplicate terminators.

### 2. Reserved word prefix collision — resolved

Longer identifiers are now kept intact in assignment targets and expression
operands even when they start with a reserved token. Classic compact forms such
as `IFA=1THENPRINTA` and `FORI=1TO10` remain accepted.

| Prefix  | Formerly failing examples |
|---------|---------------------------|
| `TOTAL` | `TOTALPOP`, `TOTALSTARVED` |
| `MAX`   | `MAXPOP`, `MAXENT`, `MAXE` |
| `MIN`   | `MINPOP`, `MINP` |
| `SUM`   | `SUMPOP` |
| `FREQ`  | `FREQSUM` |
| `STAR`  | `STARVT`, `STARVED` |
| `YIELD` | `YIELD` (as variable) |
| `STV`   | `STV` (as variable) |
| `TOT`   | `TOT`, `TOTS` |
| `MAXP`  | `MAXP` |
| `MINP`  | `MINP` |

### 3. Bare RND — resolved

Bare `RND` is accepted as the classic advancing form `RND(1)`. `RND(0)` still
repeats the previous value, and negative arguments retain deterministic reseeding.

### 4. No underscore in variable names

`P_I` fails with "unexpected character '_'". Must use `PI`.

### 5. DIM array / math-list integration — resolved

A dimensioned numeric array can be read as a nested math list using `A()`.
For example, `FFT(A())`, `NORM(A())`, and `MATMUL(A(),B())` consume actual
array data, including unwritten zero values. `A()=expression` writes a
same-shaped numeric list back atomically while preserving declared bounds and
applying element types. Both OPTION BASE values and BYREF array aliases work.
String and whole-record arrays are outside this numeric conversion API.

### 6. INPUT and piped stdin — clarified

Prompted numeric and string `INPUT` work when running a file-backed program, and
were verified with piped input. `run -` uses standard input for the BASIC source
itself, so that form intentionally has no second stream available for runtime
input; use `run program.bas` when the program also reads standard input.

## Math Engine Bugs

### 7. COMB(n,k) returns 0 — resolved

`COMB` and `PERM` are now callable aliases of `NCR` and `NPR`.

```
PRINT COMB(5,2)  -> 10
PRINT PERM(5,2)  -> 20
```

### 8. No VARIANCE or STD function — resolved

`VARIANCE`, `STD`, `STDDEV`, and `STDEV` now provide sample statistics,
matching `VARS` and `STDEVS`. `VARP` and `STDEVP` remain the explicit
population forms.

### 9. AUTOCORR takes 1 argument, not 2 — resolved

`AUTOCORR(data,max_lag)` now returns the symmetric correlation window from
`-max_lag` through `+max_lag`. The one-argument form retains its full-range
behavior.

### 10. EXP(LOG(x)) precision loss — resolved

`EXP(LOG(7))` now returns exactly 7. The correction is restricted to integer
inverse values whose binary64 logarithm exactly matches the input to `EXP`.

## Graphics Bugs

### 11. PAINT flood-fill fails for LINE-bounded regions — resolved

`PAINT(x,y),"#color"` now performs a bounded four-neighbor flood fill in both
raster and SVG output, including regions assembled from separate `LINE`
statements.

### 12. No named colors — resolved

Graphics commands accept palette indexes, hexadecimal strings, and
case-insensitive named colors including `"SKYBLUE"`, `"GOLD"`, and `"RED"`.

### 13. RECTANGLE syntax undocumented — resolved

`RECTANGLE (x1,y1)-(x2,y2)[,stroke[,fill[,stroke_width]]]` is now accepted.
The existing `RECTANGLE x,y,width,height[,stroke[,fill[,stroke_width]]]`
form and both forms' argument meanings are documented in the language
reference.

## Statement & Feature Tests (Systematic)

### 14. XOR operator — resolved

`XOR` and the BBC spelling `EOR` are checked bitwise integer operators with the
same lower precedence as the classic logical operator family.

### 15. Structured loops — resolved

`DO WHILE`, `DO UNTIL`, unconditional `DO`, `LOOP WHILE`, `LOOP UNTIL`, and
BBC-style `REPEAT...UNTIL` are supported, nested, bounded by the loop-stack
limit, and structurally validated. `EXIT DO`, `EXIT FOR`, `EXIT WHILE`, and
`EXIT REPEAT` unwind the requested active loop.

### 16. SELECT CASE — resolved

`SELECT CASE` supports numeric and string selectors, value lists, inclusive
`TO` ranges, `CASE IS` relations, `CASE ELSE`, first-match behavior, and
nesting. BBC `CASE expression OF`, `WHEN`, `OTHERWISE`, and `ENDCASE` are also
accepted.

### 17. Integer division — resolved

Both Microsoft/BBC backslash (`\`) and named `DIV` perform checked integer
division. Real operands are rounded before `DIV`, `\`, and `MOD`, matching the
documented Microsoft conversion rule.

### 18. PI call syntax — clarified

`PI()` returns 3.14159265358979... Bare `PI` remains an ordinary classic BASIC
variable and therefore defaults to 0 until assigned.

### 19. TIME$, DATE$, and TIMER — clarified

`TIME$()` and `DATE$()` return the current local time and date. Bare `TIME$` and
`DATE$` are ordinary string variables and default empty. `TIMER` is not
implemented and remains an ordinary numeric variable that defaults to 0.

### 20. ENVIRON$ and COMMAND$ broken

`PRINT ENVIRON$("HOME")` -> "type mismatch". `PRINT COMMAND$` returns
empty string. Neither provides access to environment or command-line args.

### 21. BIN$ function — resolved

`BIN$(42)` returns `"101010"` using the same checked integer conversion policy
as `HEX$` and `OCT$`.

### 22. FACT and FACTORIAL naming — resolved

`FACT(5)` and `FACTORIAL(5)` both call the same checked factorial implementation
and return 120.

### 23. 0^0 returns 1

`PRINT 0^0` returns 1. Mathematically debatable (some define as 1,
some as undefined). Matches Python/Excel behavior. Noting for
completeness.

### 24. MOD by zero returns a structured error — clarified

`PRINT 5 MOD 0` -> "division by zero" runtime error. This is a checked guard,
not a process crash, and `ON ERROR` can catch it just like `PRINT 10/0`.

### 25. LOG(0) and SQR(-1) error correctly

`LOG(0)` -> "math domain error in LOG" (correct, no crash).
`SQR(-1)` -> "math domain error in SQR" (correct, no crash).
These produce clean errors rather than NaN or crashes.

## Stress-testing fixes

### 26. Deep expressions abort the process — resolved

Hundreds of nested parentheses or thousands of right-associative powers could
overflow the Rust call stack. Delimiter, unary, call, list, and power recursion
now share a 32-level expression budget and return `expression nesting limit (32)
exceeded` without aborting.

### 27. Long flat expressions tokenize quadratically — resolved

Keyword probes previously rebuilt the entire unconsumed source at every token.
They now compare only bounded case-insensitive prefixes. A 20,000-term debug
expression completed in 0.70 seconds in the verification pass; 10,000 terms had
previously exceeded an eight-second timeout.

### 28. Oversized graphics buffers can exhaust memory — resolved

Custom `SCREEN` dimensions and Artist raster/flood rendering now reject pixel
products above 16,777,216 before allocating. A 1024x1024 two-region flood fill
was rendered to both PPM and SVG, while 4097x4097 and 65535x65535 screens returned
clean `illegal quantity` errors.

### 29. Bundled prime examples fail on real loop bounds — resolved

Both prime-number examples passed `SQR(I)` directly to the integer-only `FOR`
statement and failed on the first non-square input. They now trial-divide through
`INT(SQR(I))`, explicitly handle inputs below three, and produce the correct
prime lists for boundary inputs 1, 2, and 3 as well as the normal input 30.

### 30. Continuous GUI graphics can starve repaint and override BREAK — resolved

The X11 shell previously restarted its render debounce timer for every graphics
update, so a tight drawing loop could remain visually blank. A queued graphics
event could also switch the shell back to graphics after Ctrl+Escape requested
the text screen. The timer now remains anchored to the first pending frame, and
the shell restores text only after pre-BREAK screen events are drained. Automated
X11 acceptance confirmed continuous contrasting-color rendering followed by a
visible `BREAK`, `READY.`, and text view.

### 31. Launchers can run a stale release binary — resolved

`run.sh` and `audio-test.sh` previously trusted only the static revision marker,
so local Rust or manifest edits could leave them executing an older binary.
They now use the same source-newer-than-binary guard as `smoke-test.sh`. A
verification probe advanced a source timestamp, confirmed that `run.sh version`
rebuilt the optimized binary, and then passed WAV and smoke checks.

## Features That Work Correctly

These were tested and confirmed working:

- PRINT with `;` (concat) and `,` (tab zones) and `TAB()` / `SPC()`
- LET and implicit assignment (`A = 42`)
- String operations: `+`, `LEN`, `LEFT$`, `RIGHT$`, `MID$`, `CHR$`,
  `ASC`, `STR$`, `VAL`, `INSTR` (2-arg and 3-arg), `STRING$`, `SPACE$`,
  `BIN$`, `HEX$`, `OCT$`, `LCASE$`, `UCASE$`
- `MID$` as lvalue (replacement): `MID$(S$, 7, 5) = "BASIC"`
- String comparison: `<`, `>`, `=`, `<>` (case-sensitive)
- `FOR...NEXT` with positive, negative (STEP -1), and nested loops
- `IF...THEN...ELSE` in single-line and validated nested block forms
- `GOSUB...RETURN` and spaced `GO SUB` (nested)
- Structured `SUB...END SUB` with recursive `CALL`, default/explicit `BYVAL`,
  explicit exact-type `BYREF` scalar/array-element/record-field/whole-array
  aliases (including typed whole-record arrays), local scalars/primitive and
  record values/arrays, typed scalar/record `STATIC`, scalar/array `SHARED`,
  `EXIT SUB`, and classic `GOSUB` interop
- Structured numeric/string `FUNCTION...END FUNCTION` with recursion, typed
  return slots, `BYVAL`/`BYREF`, local/shared/static scope, `EXIT FUNCTION`,
  `SUB` calls, and lazy expression/control-flow evaluation
- User-defined `TYPE...END TYPE` records with typed/fixed-string and nested
  record fields, forward references with cycle rejection, scalar/array
  instances, multi-level dotted access, typed I/O, scoped local/`STATIC`
  values and arrays, typed root/subrecord whole-array formals, exact-type
  whole-record/subrecord copy, and atomic resize/erase
- `ON...GOTO` and `ON...GOSUB` (1-indexed, correct branch selection)
- `WHILE...WEND`
- `DO...LOOP` with top/bottom `WHILE` or `UNTIL`, plus `REPEAT...UNTIL`
- `EXIT DO`, `EXIT FOR`, `EXIT WHILE`, and `EXIT REPEAT`
- `SELECT CASE` lists/ranges/relations/defaults and BBC `CASE...OF...WHEN`
- `SWAP A, B`
- `DEF FN` (single-line user functions with one or more numeric parameters)
- `DATA...READ...RESTORE`
- Dartmouth `CHANGE` string-to-numeric-array and array-to-string conversion
- `DIM`, atomic `REDIM`/`REDIM PRESERVE`, multidimensional numeric and
  string arrays, `OPTION BASE`, and actual-shape `LBOUND`/`UBOUND`
- Visible movable native sprites, `PCOLLISION`, and edge-triggered collision
  traps; seconds-since-midnight `TIMER` and non-reentrant timer traps
- Immutable calculated numeric and string `CONST` declarations
- `ON ERROR GOTO...RESUME` (error trapping works)
- `PEEK` / `POKE` (virtual memory, round-trip verified)
- `RANDOMIZE`
- `REM` comments
- `END` / `STOP`
- File I/O: standard and mode-first shorthand `OPEN`, `PRINT #`, `INPUT #`,
  `LINE INPUT #`, and `CLOSE #`
- Math: `SQR`, `ABS`, `INT`, `FIX`, `CINT`, `ROUND`, `SGN`, `SIN`, `COS`,
  `TAN`, `ATN`, `EXP`, `LOG`, `MOD`, integer `DIV`/`\`, `XOR`/`EOR`, bare
  `RND`, `FACT`/`FACTORIAL`, and `^`
- Math engine lists: `{1,2,3}` syntax, `TOTAL`, `NORM`, `DOT`, `PRODUCT`,
  `GEOMEAN`, `HARMEAN`, `RMS`, `MEAN`, `MEDIAN`, `MODE`, `SKEWNESS`,
  `KURTOSIS`, `PERCENTILE`, `QUARTILE`, `SORT`, `REVERSE`, `UNIQUE`,
  `DIFF`, `CUMSUM`, `HISTOGRAM`, `CONVOLVE`, `ZCR`, `HYPOT`, `LERP`,
  `CLAMP`, `WRAP`, `SMOOTHSTEP`, `FLOOR`, `CEIL`, `GCD`, `FACT`,
  `COMB`, `PERM`, `VARIANCE`, `STDDEV`, `AUTOCORR`, `ISPRIME`,
  `FIBONACCI`, `FACTORS`, `FFT`, `IFFT`
- Graphics: `SCREEN GRAPHICS`, `SCREEN TEXT`, `CLS "#color"`, `CIRCLE`,
  `LINE`, `RECTANGLE`, `PSET`, bounded `PAINT`, named colors, `SAVEIMAGE`
- `INKEY$` (returns empty non-interactively, as expected)

## Workarounds Summary

| Remaining distinction | Current approach |
|-----------------------|------------------|
| Bare `PI` variable | Call `PI()` or assign a `PI` variable |
| Calendar date/time | Call `DATE$()` and `TIME$()` |
| DIM array vs list | Read with `A()`; write a same-shaped result with `A()=expression` |
| Host environment/arguments | Supply values through `INPUT` or sandboxed files |
| Identifier underscores | Use letters and digits in classic variable names |
