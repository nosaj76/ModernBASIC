"""Registered Linux desktop applications, discovered and launched by GIO."""
import os
import sys

MAX_BYTES = 262144
MAX_APPS = 2048


def valid_id(value):
    return (0 < len(value.encode('utf-8')) <= 255 and value.endswith('.desktop')
            and not any(c in '/\\' or ord(c) < 32 or ord(c) == 127 for c in value))


def field(value, limit):
    return ''.join(' ' if c.isspace() or ord(c) < 32 or ord(c) == 127 else c
                   for c in (value or ''))[:limit]


def applications():
    try:
        from gi.repository import Gio
    except ImportError as error:
        raise RuntimeError('Linux applications require system Python 3 with PyGObject/GIO (Debian: python3-gi, gir1.2-glib-2.0).') from error
    found = {}
    for app in Gio.AppInfo.get_all():
        ident = app.get_id() or ''
        if isinstance(app, Gio.DesktopAppInfo) and valid_id(ident) and app.should_show():
            found.setdefault(ident, app)
    return found


def launch(app):
    # Launched applications must not inherit the helper's response pipes or
    # hold the BASIC request open. Duplicates are close-on-exec in Python 3.
    saved = [os.dup(fd) for fd in (0, 1, 2)]
    try:
        with open(os.devnull, 'r+b', buffering=0) as sink:
            for fd in (0, 1, 2):
                os.dup2(sink.fileno(), fd)
            if not app.launch([], None):
                raise RuntimeError('Linux rejected the application launch.')
    finally:
        for fd, previous in enumerate(saved):
            os.dup2(previous, fd)
            os.close(previous)


def run(operation, ident):
    if operation not in ('list', 'launch') or (operation == 'list' and ident):
        raise ValueError('Invalid Linux application request.')
    if operation == 'launch' and not valid_id(ident):
        raise ValueError('Use a registered desktop application ID, not a path or command.')
    apps = applications()
    if operation == 'launch':
        app = apps.get(ident)
        if app is None:
            raise ValueError('This application is no longer available in the Linux menu. Refresh the list.')
        launch(app)
        return 'Launch requested.\n'
    rows = []
    size = 0
    for ident, app in sorted(apps.items(), key=lambda pair: ((pair[1].get_display_name() or '').casefold(), pair[0])):
        row = '\t'.join((ident, field(app.get_display_name(), 160), field(app.get_description(), 240))) + '\n'
        size += len(row.encode('utf-8'))
        if len(rows) >= MAX_APPS or size > MAX_BYTES:
            raise ValueError('Linux application list is too large (maximum 2048 entries / 256 KiB).')
        rows.append(row)
    return ''.join(rows)


def main():
    try:
        request = sys.stdin.buffer.read(1025)
        if len(request) > 1024:
            raise ValueError('Linux application request is too large.')
        operation, ident = request.decode('utf-8').split('\n', 1)
        result = run(operation, ident)
        sys.stdout.write(result)
    except Exception as error:
        print(str(error)[:2000], file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
