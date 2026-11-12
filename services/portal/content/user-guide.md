# ModernBASIC user guide

ModernBASIC is a native 64-bit BASIC programming environment, available as a Linux application and as an integrated Linux distribution. Use its workspace to write programs, make graphics, play music and explore the program catalog. Linux provides the drivers, networking and system services underneath it.

## Meet your workspace

The system hub is your starting point. Use it to open programs, switch tasks, change display settings and shut down. Each program opens in its own window with a `READY.` prompt, its own code and its own working memory. Hub commands and BASIC program statements serve different purposes.

| In the system hub | What it does |
| --- | --- |
| `DIR` | Browse the program directory |
| `CLS` or Ctrl+L | Clear command output while keeping the working directory and open tasks |
| `LOAD "examples/hello.bas"` | Open an existing program (choose a path shown by DIR) |
| `NEW "hello.bas"` | Create a new program file and open its window |
| `TASKS` | List open tasks |
| `FOCUS 1` | Focus the indicated task |
| `STOP 1` | Interrupt the indicated task |
| `CLOSE 1` | Close the indicated task |
| `SETTINGS` | Open sound/display/Wi-Fi/Bluetooth settings and hardware status |
| `SETUP` | Reopen the first-run checklist |
| `DISPLAY` | Change resolution with automatic rollback |
| `HELP` or F1 | Read the built-in manual |
| `BASH` or F12 | Open the Bash terminal |

In the native Linux application, the Bash terminal runs as your normal Linux user. In the integrated distribution, its administration terminal uses the configured appliance policy.

Use Alt+Tab to switch windows. Drag a title bar to move a window and its edges to resize it. Double-click a title bar to maximize or restore. Super+D shows the hub. Task numbers come from `TASKS`; use the number actually shown on your computer.

## Your first program

In the hub, enter `NEW "hello.bas"`. If that name already exists, load it or choose a different name. In the new window, type each of these lines and press Enter:

```basic
10 FOR I = 1 TO 5
20 PRINT "HELLO, WORLD!"
30 NEXT I
40 END
```

Type `RUN` to see the greeting five times. Type `LIST` to see your code. Change line 20 by typing a replacement with the same line number. A line number alone deletes that line.

```basic
20 PRINT "I MADE THIS!"
RUN
SAVE "hello.bas"
```

`SAVE` writes your source to a file. Changes in a live session are temporary unless copied to persistent storage. On an installed system, saved files survive reboot. Closing a program window discards unsaved changes and its working memory.

## Files and editing

`PWD` shows your current BASIC directory. `DIR` lists folders and files. `CD "examples"` enters a folder and `CD /` returns to the BASIC program root. Use paths from your actual directory listing.

At a program's `READY.` prompt, use `LOAD`, `SAVE`, `LIST`, `NEW`, `RUN`, `STEP` and `CONT` to manage and run code. READY `NEW` clears the stored program; hub `NEW "name.bas"` creates a file and opens a window.

F2 lists the program for editing. Ctrl+Up and Ctrl+Down select stored lines. Edit the selected line and press Enter to replace it. Ctrl+U switches between uppercase and mixed-case input.

Ctrl+Escape interrupts a running program and keeps its window and working state available. Closing the window ends the task. Plain Escape cancels editing or an input operation; it is not the general program interrupt.

In the Debian appliance, user programs are stored under `/home/modernbasic/programs` and generated output under `/home/modernbasic/output`. Back up both directories from Linux before reinstalling or replacing a disk. The initial catalog is copied on first boot; updates do not overwrite your edited copies.

## Appearance and display

Use `DISPLAY` in the hub to select a display mode. Change the default background and text with names:

```text
COLORS
SCREEN COLOR = navy
TEXT COLOR = white
```

`COLORS` lists the 32 supported names. Names ignore case, and `#rrggbb` values also work. Preferences save automatically. New text uses the new default; previously printed text retains its original colors. BASIC drawing colors and the `TEXTCOLOR` statement are separate programming controls.

## Sound and first-run setup

Open `SETTINGS`, choose Sound, then select your speakers, headphones, or TV.
Choose **Use for new playback** to make that output the default. Adjust its volume,
unmute if necessary, and use the left/right test. The test does not change the
volume or default output. If sound is still quiet, check the speaker or TV volume.
Restart already playing programs after changing outputs.

On the first appliance startup, a welcome checklist offers sound, display, and
network checks. **Finish setup** records completion; **Later** postpones it until
next startup. Use `SETUP` to revisit it. Live-session settings are temporary;
installed systems retain saved network profiles and Bluetooth pairings. Bluetooth
radio power follows the adapter’s boot policy.

## Bluetooth devices

Open `SETTINGS` → **Bluetooth**, select an adapter, turn it on, and choose
**Find nearby devices**. Put the accessory in pairing mode and select it by name
and address. Choose **Pair this device**. Compare any displayed code on both
devices before confirming; PIN entry is masked. Some keyboards require typing
the displayed code on the keyboard itself and pressing Enter there.

Choose **Connect** after pairing. For speakers/headphones, then use **Sound** to
select the output and test it. **Allow this device to reconnect** explicitly trusts
it, allowing future connections/services without another prompt; use this only
for your devices. Actual reconnection depends on device power, range and profile.
**Disconnect** retains pairing; **Forget** removes the saved pairing keys. Have
another input device available before disconnecting a Bluetooth keyboard or mouse.

Scanning stops after 12 seconds or when you choose **Show devices**. Canceling an
operation asks BlueZ to cancel pairing or disconnect the attempted connection;
refresh to see its current state. Completed pairings are removed only by **Forget**.
If no adapter is found, check the hardware switch, firmware or USB connection.
Saved pairings persist on installed systems; live sessions remain temporary.

## Networking and Linux tools

Open `SETTINGS` → Wi-Fi to find a network or connect to a hidden network. Enter
credentials in the masked-password dialog, then choose **Remember** or **Connect
once**. Remember saves the profile and password in NetworkManager’s root-owned
connection storage and enables automatic reconnection; Connect once forgets the
new profile on disconnect. The live medium remains temporary.

The page also offers saved-network reconnection/removal, disconnect, and Wi-Fi
on/off controls. Canceling connection attempts restores the previous connection on
that adapter. If BASIC closes during an attempt, allow up to three minutes for
network recovery. A connection does not guarantee Internet access; captive portals
may require a browser login.

Use `BASH` then `nmtui` for administrator or advanced network setup. Network access
by BASIC programs is a separate capability and is disabled by default in the appliance.

| Linux command | Purpose |
| --- | --- |
| `files` | Open the file manager |
| `edit FILE` | Open the supplied Linux text editor |
| `software` | Open the package manager |
| `backend audio` | Show the audio backend and devices |
| `backend storage` | Show storage devices |
| `backend printers` | Show printer status |
| `backend help` | List available Linux backend tools |
| `exit` | Close the Bash terminal |

The appliance's BASH terminal is a trusted-console administration tool with full machine access. Use BASIC program windows for ordinary programming; BASIC's file restrictions do not apply to Linux administration commands.

## Updates and shutdown

On an installed system, the distribution configures daily Debian security updates when networking is available. It does not automatically reboot. Live-session updates are disabled; the live overlay is temporary. Use Linux administration tools to review other package updates and restart when required.

The development distribution now stages `modernbasic-update` for offline
ModernBASIC package updates. It checks trusted package checksums, architecture,
profile and dependencies, retains the exact installed package, and restores it
if installation or desktop health checks fail. Save your work and keep an
independent backup before applying an update. Use F12 and
`modernbasic-update --help`; `modernbasic-update status` also appears on the
Settings update page. This is package recovery, not kernel or whole-disk rollback.
Public signed release downloads are still pending.

At the system hub, `SHUTDOWN` followed by `Y` requests a clean operating-system power-off. `RESTART` restarts the BASIC environment. `QUIT` or `EXIT` at a program prompt closes that program window.

Save your programs before closing their windows or shutting down. Keep backups independently of the installation disk.

## If something goes wrong

Start by checking the matching manual chapter. For an unresponsive program, use Ctrl+Escape. For audio trouble, start with Sound in `SETTINGS`. Network status and administration instructions are also available there. A service crash may restart the affected desktop component.

Internal failure reports are saved locally. Once reporting is configured and the desktop is available, a reporting prompt lets you review a report and choose whether to send it to the configured project website. Sending requires an explicit consent checkbox and button. Canceling, closing the dialog, or letting it expire keeps the report locally without uploading. An offline upload failure also keeps the local copy; it does not silently retry later.

A report can include kernel/build versions, service states, recent logs, memory and storage information. Common credentials are redacted, but logs can still identify hardware or contain private text. Read the preview before agreeing to send. No source programs or home files are automatically attached.

From BASH, you can create a report yourself:

```sh
diagnostics report --category basic --reason "Describe what went wrong"
diagnostics list
```

Use the returned report path with `diagnostics preview REPORT` or `diagnostics send REPORT`. The terminal sender also asks before uploading. Categories are `kernel`, `basic`, `integration` and `unsure`. The website keeps these queues separate and developers can reclassify reports.

A machine that cannot boot or display a dialog cannot ask for consent immediately. Reports that were successfully saved can be reviewed after recovery. Hard crashes or power loss can prevent evidence from being written at all.
