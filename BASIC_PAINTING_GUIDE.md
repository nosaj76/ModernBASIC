# Modern64BASIC 0.8.2 — Painting & Graphics Reference

A practical guide to painting with Modern64BASIC. All syntax verified against the
live interpreter at `/home/jason/projects/OSprojects/BASIC/modernbasic-0.8.2/`.

## Running Programs

```bash
cd /home/jason/projects/OSprojects/BASIC/modernbasic-0.8.2

# Run from stdin
echo '10 PRINT "HI"' | ./target/release/modernbasic run -

# Run a file (use --file-root . for SAVEIMAGE/file I/O)
./target/release/modernbasic run --file-root . programs/painting_demo.bas

# Boot interactive shell (like booting a PC)
./target/release/modernbasic shell
# At READY. type: LOAD "program.bas" <Enter>
# Then type: RUN <Enter>
```

## Core BASIC Syntax (for painting loops)

### Variables
- Numeric: `X = 42` or `LET X = 42`
- String: `S$ = "hello"` (suffix `$`)
- No underscores in names: use `PI` not `P_I`
- Reserved prefixes that BLOCK variable names: TOTAL, MAX, MIN, SUM, FREQ, STAR,
  YIELD, STV, TOT — never start a variable name with these
- Use `RND(1)` for random float 0..1, not bare `RND`
- Use `INT(RND(1)*N)` for random integer 0..N-1

### Control Flow
```basic
10 FOR I=0 TO 100 STEP 5    ' counting loop
20   PSET (I, I), "#ff0000"
30 NEXT I

40 WHILE X < 100            ' condition loop (no DO/LOOP)
50   X = X + 1
60 WEND

70 IF X > 50 THEN 200       ' branch to line (no ELSE)
80 IF X > 50 THEN LET Y=1   ' inline statement

90 GOSUB 5000               ' subroutine call
100 RETURN                  ' return from subroutine

110 ON EXPR GOTO 100,200,300  ' computed branch (1-indexed)
```

NOTE: No ELSE, no DO/LOOP, no SELECT CASE, no XOR, no integer division `\`.
Use `INT(a/b)` instead of `\`.

### Math Functions
```basic
SQR(16)     ' = 4   (square root)
ABS(-5)     ' = 5
SIN(0)      ' = 0   (radians)
COS(0)      ' = 1
ATN(1)      ' = 0.785... (arctangent)
INT(-3.7)   ' = -4  (floor)
FIX(3.7)    ' = 3   (truncate)
SQR(2)      ' = 1.414...
3.14159     ' must define PI yourself (built-in PI returns 0 — bug)
```

---

## GRAPHICS / PAINTING STATEMENTS

### Screen Setup
```basic
SCREEN 800, 600                    ' custom resolution, default bg
SCREEN 800, 600, "#0a0a1a"         ' custom resolution + bg color
SCREEN 12                          ' classic VGA mode 12 (640x480, 16 colors)
SCREEN GRAPHICS                    ' switch to graphics view
SCREEN TEXT                        ' switch to text view
SCREEN SPLIT                       ' split text + graphics view
SCREEN GRAPHICS2D                  ' 2D mode
SCREEN GRAPHICS3D                   ' 3D perspective mode
```

### Clear / Background
```basic
CLS "#1a1a2a"    ' fill entire canvas with color
CLS              ' clear with default background
```

### Colors
Colors are hex strings in quotes: `"#ff8800"`, `"#00aaff"`.
With alpha (8-digit hex): `"#ff880080"` (50% transparent).
Special: `"none"` = no stroke/fill, `"WHITE"` = named (unreliable, prefer hex).
Palette indices 0-15 also work for classic modes.

For computed true color, use `RGB(red,green,blue)` or
`RGBA(red,green,blue,alpha)`. Every channel must be an integer from 0 through
255. These functions return canonical color values for any graphics statement:

```basic
PSET (X,Y),RGB(R,G,B)
PSET (X,Y),RGBA(255,210,96,128)
```

### PSET — Single Pixel
```basic
PSET (x, y), "#ff0000"        ' draw one pixel
PSET (x, y)                   ' draw with current foreground
PSET (x, y, z), "#ff0000"     ' 3D perspective pixel (z = depth)
```

Consecutive 2D pixels on a scanline are retained as a compact raster run.
Wrap large pixel-painting loops in `SCREENLOCK` / `SCREENUNLOCK` so the live
window presents the finished frame once instead of repainting after each PSET.

### LINE
```basic
LINE (x1,y1)-(x2,y2), "#ff0000"          ' line with color
LINE (x1,y1)-(x2,y2), "#ff0000", 3       ' line with stroke width 3
LINE (x1,y1)-(x2,y2), 14, B              ' box outline (palette color 14)
LINE (x1,y1)-(x2,y2), 14, BF             ' filled box
```

### DRAWTO — Line from current cursor
```basic
MOVE 100, 100                  ' set cursor position
DRAWTO 200, 200, "#00ff00", 2  ' line from cursor to point, width 2
```

### RECT / RECTANGLE
```basic
RECT x, y, w, h, "#stroke", "#fill", stroke_width
RECTANGLE 100, 100, 200, 150, "#ff0000", "#00ff00", 2
RECT 100, 100, 200, 150, "none", "url(#GRAD)", 1   ' gradient fill
```

### ROUNDEDRECT
```basic
ROUNDEDRECT x, y, w, h, radius, "#stroke", "#fill", line_width
```

### CIRCLE
```basic
CIRCLE (x, y), radius, "#stroke", "#fill", stroke_width
CIRCLE (320, 240), 100, "#ff0000", "#00aaff", 2
CIRCLE (320, 240), 100, "none", "#ff8800", 1      ' filled, no stroke
CIRCLE (320, 240), 100, 11                          ' palette color
```

### ELLIPSE
```basic
ELLIPSE (x, y), rx, ry, "#stroke", "#fill", stroke_width
ELLIPSE (400, 300), 100, 50, "#ff0000", "none", 2
```

### ARC
```basic
ARC cx, cy, rx, ry, start_angle, end_angle, "#stroke", "#fill", width
' angles in radians (0 = right, PI = left, 2*PI = full circle)
ARC 400, 300, 50, 30, 3.14159, 6.28318, "#ff0000", "none", 2
```

### POLYGON
```basic
POLYGON (x1,y1),(x2,y2),(x3,y3), "#stroke", "#fill", stroke_width
POLYGON (100,100),(200,50),(300,150),(150,250), "#080", "#0a0", 1
```

### BEZIER — Cubic Bezier Curve
```basic
BEZIER x1,y1, cx1,cy1, cx2,cy2, x2,y2, "#color", width
BEZIER 0,300, 200,100, 500,500, 800,300, "#ff00ff", 3
```

### PATH — SVG Path Data
```basic
P$ = "M 0 100 L 100 50 L 200 100 L 200 200 L 0 200 Z"
PATH P$, "#stroke", "#fill", stroke_width
PATH P$, "none", "url(#GRAD)", 1
```
Path commands: M (move), L (line), C (cubic bezier), Q (quadratic bezier),
H (horizontal), V (vertical), A (arc), Z (close). Lowercase = relative.

### TEXT
```basic
TEXT x, y, font_size, "string", "#color"
TEXT 100, 50, 28, "HELLO WORLD", "#ffffff"
```

### PAINT — Flood Fill
```basic
PAINT (x, y), "#fillcolor"               ' fill enclosed region
PAINT (x, y), "#fillcolor", "#border"    ' fill with border color
```
NOTE: PAINT flood-fill works inside CIRCLE. It does NOT reliably work
inside regions bounded only by LINE. Use PATH or CIRCLE for fillable shapes.

### PAINT TEXTURE — Procedural Noise Texture
```basic
PAINT TEXTURE path$, seed, scale, octaves, "#color1", "#color2", opacity, "BLENDMODE"
PAINT TEXTURE P$, 42, 8, 3, "#0a2010", "#1a4020", 0.5, "OVERLAY"
```
Fills a path region with procedural noise (value noise with N octaves).
Blend modes: NORMAL, OVERLAY, SOFT-LIGHT, SCREEN, MULTIPLY, etc.

### PAINT FILTER — Post-Process Filter
```basic
PAINT FILTER "BLUR", radius
PAINT FILTER "SHARPEN", amount
PAINT FILTER "COLOR", hue, saturation, brightness, contrast
```

### BRUSH — Paint Stroke
```basic
BRUSH (x1,y1)-(x2,y2), size, "#color", density, scatter, flow
```
Paints a brush stroke between two points with given size, color, and
brush parameters (density 0-1, scatter 0-1, flow 0-1).

---

## GRADIENTS

### Linear Gradient
```basic
GRADIENT id, x1, y1, x2, y2, "#startcolor", "#endcolor"
GRADIENT SKY, 0, 0, 0, 600, "#000040", "#ff8040"
```
Then use as fill: `RECTANGLE 0, 0, 800, 600, "none", "url(#SKY)", 1`

### Radial Gradient
```basic
RADIALGRADIENT id, cx, cy, radius, "#innercolor", "#outercolor"
RADIALGRADIENT GLOW, 400, 300, 200, "#ffffff", "#00000000"
```
Then use as fill: `CIRCLE (400, 300), 200, "none", "url(#GLOW)", 1`

Colors with alpha (8-digit hex): `"#ff880080"` = 50% transparent.
Gradient stops are always 0.0 (first color) and 1.0 (second color).

---

## COMPOSITING

### BLEND — Blend Mode
```basic
BLEND "screen"       ' screen blend (lightens)
BLEND "multiply"     ' multiply blend (darkens)
BLEND "overlay"      ' overlay blend
BLEND "normal"       ' reset to normal
```
Modes: normal, multiply, screen, overlay, darken, lighten, difference,
add, exclusion, color-dodge, color-burn, hard-light, soft-light, xor, erase.

### OPACITY
```basic
OPACITY 0.5           ' set scene-wide opacity (0.0 to 1.0)
```

### Transforms
```basic
TRANSLATE dx, dy       ' shift origin
ROTATE degrees         ' rotate coordinate system
SCALE sx, sy           ' scale (or SCALE s for uniform)
```

---

## ADVANCED PAINTING FEATURES

### NGON — Regular Polygon
```basic
NGON cx, cy, sides, radius, "#stroke", "#fill", stroke_width
NGON 400, 300, 6, 80, "#ff0000", "#00ff00", 2   ' hexagon
```

### STAR
```basic
STAR cx, cy, points, outer_r, inner_r, "#stroke", "#fill", width
STAR 400, 300, 5, 80, 35, "#ffff00", "#ff8800", 2
```

### BITMAP — Load Image
```basic
BITMAP "photo.png", x, y, w, h                   ' place image
BITMAP WARP "photo.png", x, y, w, h, angle, bend, turn, light  ' warped
```

### 3D Perspective
```basic
PERSPECTIVE horizon_x, horizon_y, focal_length, near, far, ambient
OBJECT CAMERA x, y, z
OBJECT LIGHT name, x, y, z, intensity, range, "#color"
OBJECT SHAPE id, x, y, z, w, h, "#fill", "#stroke"
OBJECT MOVE id, x, y, z
OBJECT SCALE id, w, h
PSET (x, y, z), "#color"            ' 3D pixel
```

---

## SPRITES

```basic
SPRITE NAME, x, y, w, h, "#fill", "#stroke"
MOVESPRITE NAME, x, y
SPRITEX(NAME)     ' get x position
SPRITEY(NAME)     ' get y position
PCOLLISION(A, B)  ' collision detection
ADVANCESPRITES dt ' advance sprite motion
```

---

## ANIMATION / VIDEO

```basic
BUFFER 1               ' enable double buffering
FLIP                   ' swap buffers (show frame)
VIDEO START "out.mp4", fps, "audio.wav"
VIDEO FRAME            ' capture current frame
VIDEO STOP
FLIPBOOK CREATE NAME, "pattern####.ppm", start, end, fps
VIDEO EXPORT NAME, "output.mp4"
SAVEIMAGE "frame0001.ppm"   ' save frame as PPM
```

---

## OUTPUT

```basic
SAVEIMAGE "painting.png"      ' save as PNG
SAVEIMAGE "frame.ppm"         ' save as PPM
SCREENSHOT "screenshot.png"   ' same as SAVEIMAGE
DISPLAY                        ' render to screen (X11 window)
```

---

## COMPLETE PAINTING EXAMPLE

```basic
10 REM SUNSET LANDSCAPE
20 W=800:H=600
30 SCREEN W,H,"#0a0a1a"
40 REM Sky gradient
50 GRADIENT SKY,0,0,0,H*0.6,"#0a0a3a","#ffa040"
60 RECTANGLE 0,0,W,H*0.6,"none","url(#SKY)",1
70 REM Sun glow
80 RADIALGRADIENT GLOW,560,200,120,"#ffe080","#ff800000"
90 ELLIPSE (560,200),120,120,"none","url(#GLOW)",1
100 BLEND "screen"
110 CIRCLE (560,200),45,"none","#ffd060",1
120 BLEND "normal"
130 REM Mountains
140 M$="M 0 360 L 120 280 L 200 320 L 300 250 L 420 310 L 550 270 L 680 330 L 800 300 L 800 400 L 0 400 Z"
150 PATH M$,"#1a1a3a","#202050",1
160 REM Stars
170 FOR I=1 TO 40
180   PSET (RND(1)*W,RND(1)*150),"#ffffff"
190 NEXT I
200 REM River
210 BEZIER 0,450,200,430,500,470,800,440,"#2060a090",4
220 REM Grass texture
230 P$="M 0 420 L 800 420 L 800 600 L 0 600 Z"
240 PAINT TEXTURE P$,42,8,3,"#0a2010","#1a4020",0.5,"OVERLAY"
250 SAVEIMAGE "landscape.png"
260 END
```

## KEY GOTCHAS

1. No ELSE — use separate IF lines
2. No DO/LOOP — use WHILE/WEND
3. `RND(1)` not `RND`
4. `PI` returns 0 (bug) — define `PI=3.14159265` yourself
5. PAINT flood-fill works in CIRCLE, not reliably in LINE-bounded regions
6. Colors must be hex strings `"#rrggbb"` — named colors unreliable
7. No `FIELD` as gradient ID (reserved word) — use `FLD` or similar
8. 100,000 step execution limit — keep loops efficient
9. Gradient IDs: avoid reserved words (FIELD, STAR, etc.)
10. Alpha colors use 8-digit hex: `"#rrggbbaa"`
