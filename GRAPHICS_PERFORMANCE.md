# ModernBASIC graphics performance

Graphics are part of the BASIC programming environment, not a separate export
tool. Unbuffered drawing therefore remains visible as statements execute, while
buffered programs can assemble a complete frame and reveal it with `FLIP`.

## Measure animation before choosing a background strategy

Run `cargo run --release --locked --example graphics_benchmark -- 30` to
measure actual CPU-rendered BASIC frames. See the
[benchmark report](docs/GRAPHICS_BENCHMARK_2026-09-08.md) for measurements,
methodology, and current bottlenecks. These are frame-production measurements,
not displayed FPS.

Buffered `PUT` now waits for `FLIP`. An opaque, full-buffer `PUT PSET` can
replace covered anonymous drawings and skip rendering the obsolete framebuffer.
Partial restores and scenes with retained state use the existing general path.
Caching still has encoding, copying, and rasterization costs; compare it with
redrawing your actual scenery rather than assuming it is faster.

A full 1920x1080 capture needs 2,073,602 array elements, exceeding the default
one-million-element limit. The benchmark explicitly raises its own allowance.
See the [post-fix measurements](docs/GRAPHICS_BENCHMARK_AFTER_FIXES_2026-09-08.md).

## Background capture pattern

The following demonstrates capturing scenery with `GET` and restoring it with
`PUT`. It is a functional example, not currently the fastest measured pattern.
Draw moving objects after the restore and call `FLIP` once.

```basic
10 W=960:H=540
20 SCREEN W,H,"#050914"
30 BUFFER 1
40 REM DRAW THE STATIC SKY, MOUNTAINS, AND GRID ONCE
50 CLS "#050914"
60 FOR X=0 TO W STEP 48
70 LINE (W/2,H/2)-(X,H-1),"#0b4650",1
80 NEXT X
90 GET (0,0)-(W-1,H-1),BACKGROUND
100 X=W/2
110 REM FRAME LOOP
120 PUT (0,0),BACKGROUND,PSET
130 K$=INKEY$()
140 IF K$="ESC" THEN END
150 IF MULTIKEY(75) THEN X=X-8
160 IF MULTIKEY(77) THEN X=X+8
170 CIRCLE (X,H-60),18,"#ffffff","#2470a0",2
180 FLIP
190 GOTO 120
```

The runtime recognizes the common opaque, native-size `PUT PSET` case. Its
dense graphics-array cache is shared between frames, retained raster data is
run-length encoded when that is smaller, and the framebuffer is copied by
scanline without resampling. If a BASIC statement changes any element in the
captured array, the cache is invalidated automatically and the next `PUT`
rebuilds it from the modified array values.

## Practical rules

- Use `BUFFER 1` for games and call `FLIP` once per completed frame.
- Compare redrawing static scenery with restoring a `GET` array; the current
  `PUT` encoding, copying, and rendering costs can outweigh caching benefits.
- Draw sprites, shots, particles, score changes, and other dynamic objects
  after the background `PUT`.
- Consecutive true-color `PSET` calls on one scanline are batched. Pixel loops
  are fastest when the inner loop advances `X` and the outer loop advances
  `Y`.
- Use unbuffered mode when watching a picture being constructed is intentional.
  Use buffered mode when stable frame pacing matters more than progressive
  display.
- Measure the whole frame loop. Object count, filters, large text, warped
  bitmaps, audio work, and host display scaling can still determine the final
  frame rate.

## Verification

Run the complete source check before distributing a build:

```sh
./verify.sh
./build-native.sh
./run.sh
```

`verify.sh` checks formatting, compilation, unit and integration tests, Clippy,
documentation, the release build, smoke tests, concurrency, and the BASIC
program catalog. A verification failure stops the script and must not be
reported as a passing build.
