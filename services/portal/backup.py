#!/usr/bin/env python3
"""Take a consistent private SQLite snapshot without stopping the portal."""
import argparse
from contextlib import closing
import os
from pathlib import Path
import sqlite3
import tempfile


def backup(source, destination):
    source = source.resolve(strict=True)
    if destination.exists() or destination.is_symlink():
        raise ValueError('backup destination already exists')
    descriptor, name = tempfile.mkstemp(prefix='.portal-backup-', dir=destination.parent)
    temporary = Path(name)
    os.close(descriptor)
    try:
        with closing(sqlite3.connect(source.as_uri() + '?mode=ro', uri=True, timeout=10)) as live:
            with closing(sqlite3.connect(temporary)) as snapshot:
                live.backup(snapshot)
                if snapshot.execute('PRAGMA integrity_check').fetchall() != [('ok',)]:
                    raise ValueError('backup database failed integrity check')
                snapshot.commit()
        with temporary.open('rb') as stream:
            os.fsync(stream.fileno())
        # No overwrite, including a concurrent backup using the same name.
        os.link(temporary, destination)
        directory = os.open(destination.parent, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        temporary.unlink(missing_ok=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('database', type=Path)
    parser.add_argument('destination', type=Path)
    args = parser.parse_args()
    try:
        backup(args.database, args.destination)
    except (OSError, ValueError, sqlite3.Error) as error:
        parser.exit(1, f'error: {error}\n')
    print(f'Private database backup verified: {args.destination}')


if __name__ == '__main__':
    main()
