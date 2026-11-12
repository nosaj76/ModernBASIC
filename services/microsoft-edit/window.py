#!/usr/bin/env python3
"""Host Microsoft Edit in a terminal window; never launch a shell."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


# Relay Edit's terminal through a PTY and export only bounded OSC 52 clipboard
# writes. GTK/VTE versions without OSC 52 support still share the system clipboard.
CLIPBOARD_PROXY = r'''
import base64,binascii,fcntl,os,pty,signal,struct,sys,termios
from pathlib import Path
LIMIT=1024*1024
class ClipboardExport:
    def __init__(self,path):self.path=Path(path);self.pending=b'';self.dropping=False
    def feed(self,data):
        data=self.pending+data;self.pending=b''
        prefix=b'\x1b]52;'
        while data:
            if not self.dropping:
                start=data.find(prefix)
                if start<0:self.pending=data[-(len(prefix)-1):];return
                data=data[start+len(prefix):]
            ends=[(p,n) for p,n in ((data.find(b'\x07'),1),(data.find(b'\x1b\\'),2)) if p>=0]
            if not ends:
                if len(data)>2*LIMIT:self.dropping=True;self.pending=data[-1:]
                else:self.pending=(b'' if self.dropping else prefix)+data
                return
            end,size=min(ends)
            payload=data[:end];data=data[end+size:]
            if self.dropping:self.dropping=False;continue
            selection,sep,encoded=payload.partition(b';')
            if not sep or selection not in (b'',b'c') or len(encoded)>2*LIMIT:continue
            try:
                raw=base64.b64decode(encoded,validate=True)
                if len(raw)>LIMIT or b'\x00' in raw:continue
                raw.decode('utf-8')
            except (ValueError,UnicodeError,binascii.Error):continue
            temporary=self.path.with_suffix('.new')
            temporary.write_bytes(raw);temporary.replace(self.path)

def main():
    binary,path,*args=sys.argv[1:];export=ClipboardExport(path);master=None
    def resize(*unused):
        if master is not None:
            try:fcntl.ioctl(master,termios.TIOCSWINSZ,fcntl.ioctl(0,termios.TIOCGWINSZ,bytes(8)))
            except OSError:pass
    def read(fd):
        nonlocal master
        if master is None:master=fd;resize()
        data=os.read(fd,65536);export.feed(data);return data
    signal.signal(signal.SIGWINCH,resize)
    return os.waitstatus_to_exitcode(pty.spawn([binary,*args],master_read=read))
if __name__=='__main__':sys.exit(main())
'''


def editor_binary():
    configured = os.environ.get('MODERNBASIC_EDIT_BINARY')
    candidates = [configured] if configured else [shutil.which('msedit'), shutil.which('edit')]
    for candidate in candidates:
        if not candidate:
            continue
        try:
            result = subprocess.run([candidate, '--version'], capture_output=True, text=True, timeout=5)
            if result.returncode == 0 and result.stdout.strip().startswith('edit version '):
                return str(Path(candidate).absolute())
        except (OSError, subprocess.TimeoutExpired):
            pass
    raise RuntimeError('Microsoft Edit is unavailable. Install msedit or run tools/install-microsoft-edit.py; /usr/bin/edit may be an unrelated Linux command.')


def main():
    root, document, font, foreground, background = sys.argv[1:]
    binary = editor_binary()
    import gi
    gi.require_version('Gdk', '3.0')
    gi.require_version('Gtk', '3.0')
    gi.require_version('Vte', '2.91')
    from gi.repository import Gdk, GLib, Gtk, Pango, Vte
    if not Gtk.init_check()[0]:
        raise RuntimeError('Microsoft Edit needs a graphical display')
    GLib.set_prgname('modernbasic-microsoft-edit')
    title = 'Microsoft Edit'
    if document:
        title += ' — ' + ' '.join(Path(document).name.split())[:160]
    window = Gtk.Window(title=title)
    window.set_default_size(1100, 740)
    window.set_icon_name('accessories-text-editor')
    terminal = Vte.Terminal()
    terminal.set_font(Pango.FontDescription(('Noto Sans Mono' if font == 'ui-monospace' else font) + ' 13'))
    terminal.set_scrollback_lines(10000)
    def color(value):
        rgba = Gdk.RGBA()
        if not rgba.parse(value):
            raise ValueError('Invalid terminal color')
        return rgba
    terminal.set_color_foreground(color(foreground))
    terminal.set_color_background(color(background))
    terminal.set_color_cursor(color(foreground))
    window.add(terminal)
    # Closing a terminal normally sends SIGHUP. Ask Edit itself to quit instead,
    # so its Save / Don't Save / Cancel dialog can protect unsaved documents.
    def request_close(*unused):
        terminal.feed_child(b'\x11')  # Edit's Ctrl+Q command.
        return True
    window.connect('delete-event', request_close)
    clipboard_directory = tempfile.TemporaryDirectory(prefix='modernbasic-edit-clipboard-')
    clipboard_path = Path(clipboard_directory.name) / 'selection'
    clipboard = Gtk.Clipboard.get(Gdk.SELECTION_CLIPBOARD)
    def sync_clipboard():
        try:
            claimed = clipboard_path.with_suffix('.pending')
            clipboard_path.replace(claimed)
            copied = claimed.read_text(encoding='utf-8')
            claimed.unlink()
            clipboard.set_text(copied, -1)
            clipboard.store()
        except FileNotFoundError:
            pass
        return True
    clipboard_watch = GLib.timeout_add(50, sync_clipboard)
    def cleanup(*unused):
        GLib.source_remove(clipboard_watch)
        clipboard_directory.cleanup()
    window.connect('destroy', cleanup)
    terminal.connect('child-exited', lambda *unused: window.destroy())
    window.connect('destroy', Gtk.main_quit)
    def keypress(widget, event):
        modifiers = Gdk.ModifierType.CONTROL_MASK | Gdk.ModifierType.SHIFT_MASK
        if event.state & modifiers == modifiers:
            key = Gdk.keyval_name(event.keyval).lower()
            if key == 'c':
                if terminal.get_has_selection():
                    terminal.copy_clipboard_format(Vte.Format.TEXT)
                else:
                    # Edit's own selection exports OSC 52 through the PTY bridge.
                    terminal.feed_child(b'\x03')
                return True
            if key == 'v':
                terminal.paste_clipboard()
                return True
        return False
    terminal.connect('key-press-event', keypress)
    def spawned(widget, pid, error, unused):
        print('ERROR ' + str(error) if error else 'READY', flush=True)
        if error:
            window.destroy()
    window.show_all()
    terminal.grab_focus()
    terminal.spawn_async(Vte.PtyFlags.DEFAULT, root, [sys.executable, '-I', '-c', CLIPBOARD_PROXY, binary, str(clipboard_path), *([document] if document else [])],
                         ['TERM=xterm-256color', 'COLORTERM=truecolor'], GLib.SpawnFlags.DEFAULT,
                         None, None, 10000, None, spawned, None)
    Gtk.main()


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        print('ERROR ' + str(error).replace('\n', ' ')[:1000], flush=True)
        sys.exit(1)
