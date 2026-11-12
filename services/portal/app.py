#!/usr/bin/env python3
"""ModernBASIC release and private issue portal, served behind an HTTPS proxy."""
import argparse
from contextlib import closing
from datetime import datetime, timezone
import hashlib
import hmac
from http.cookies import CookieError, SimpleCookie
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from html import escape
import json
import os
from pathlib import Path
import re
import secrets
import sqlite3
from stat import S_ISREG
import time
from urllib.parse import parse_qs, urlsplit
import site_pages

CATEGORIES = {'kernel': 'Linux kernel', 'basic': 'Modern BASIC environment',
              'integration': 'OS integration', 'unsure': 'Unsure / needs triage'}
STATUSES = ('new', 'confirmed', 'in-progress', 'needs-information', 'fixed', 'closed')
PLATFORMS = {'amd64': ('PC · x86-64', 'UEFI desktops and laptops'),
             'arm64': ('ARM64', 'Compatible 64-bit UEFI systems'),
             'riscv64': ('RISC-V 64', 'Compatible 64-bit UEFI systems'),
             'pi5': ('Raspberry Pi 5', 'Dedicated board image required')}
MAX_BODY = 2 * 1024 * 1024
ASSETS = Path(__file__).with_name('style.css')
MEDIA_ROOT = Path(__file__).with_name('media')
MEDIA_TYPES = {'modernbasic-intro.mp4': 'video/mp4',
               'modernbasic-intro-poster.jpg': 'image/jpeg',
               'modernbasic-intro.vtt': 'text/vtt; charset=utf-8'}


def now():
    return datetime.now(timezone.utc).isoformat(timespec='seconds')


def e(value):
    return escape(str(value), quote=True)


def options(values, selected=''):
    pairs = values.items() if isinstance(values, dict) else ((v, v.replace('-', ' ').title()) for v in values)
    return ''.join(f'<option value="{e(k)}"{" selected" if k == selected else ""}>{e(v)}</option>' for k, v in pairs)


def page(title, body):
    return f'''<!doctype html><html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1"><title>{e(title)} · ModernBASIC</title>
<link rel="stylesheet" href="/style.css"></head><body><a class="skip" href="#main">Skip to content</a>
<header><a class="brand" href="/"><span class="mark">mB</span>modern<span>BASIC</span></a>
<nav aria-label="Main navigation"><a href="/about">The distro</a><a href="/downloads">Downloads</a><a href="/start">Get started</a><a href="/manual">User manual</a><a href="/support">Support</a></nav></header>
<main id="main">{body}</main><footer class="site-footer"><div><strong>ModernBASIC</strong><p>A place to create.<br>Built on Linux.</p></div><nav aria-label="Explore"><strong>Explore</strong><a href="/about">About the distro</a><a href="/downloads">Platform downloads</a><a href="/releases">Release notes</a></nav><nav aria-label="Learn"><strong>Learn</strong><a href="/start">Getting started</a><a href="/manual">User manual</a><a href="/support">Support &amp; FAQ</a></nav><nav aria-label="Project"><strong>Project</strong><a href="/report">Report a bug</a><a href="/developer">Developers</a><a href="/privacy">Privacy</a><a href="/licenses">Licenses</a></nav></footer></body></html>'''


class Portal:
    def __init__(self, storage, releases, manifest, secret, origin, intake_token='', max_reports=10000):
        if len(secret) < 32 or '\n' in secret or '\r' in secret:
            raise ValueError('Developer credential must contain at least 32 characters on one line')
        parsed = urlsplit(origin)
        if (parsed.scheme not in ('https', 'http') or not parsed.netloc or parsed.username
                or parsed.password or parsed.path or parsed.query or parsed.fragment
                or (parsed.scheme == 'http' and parsed.hostname not in ('127.0.0.1', 'localhost'))):
            raise ValueError('Use an HTTPS origin, or HTTP on localhost for development')
        self.secret, self.origin, self.intake_token = secret.encode(), origin, intake_token
        self.secure = parsed.scheme == 'https'
        self.max_reports = max_reports
        self.storage = Path(storage)
        self.storage.mkdir(parents=True, exist_ok=True, mode=0o700)
        self.database = self.storage / 'issues.sqlite3'
        with closing(self.connect()) as db, db:
            db.executescript('''CREATE TABLE IF NOT EXISTS issues (
                id TEXT PRIMARY KEY, created TEXT NOT NULL, category TEXT NOT NULL,
                status TEXT NOT NULL, title TEXT NOT NULL, details TEXT NOT NULL,
                platform TEXT NOT NULL, version TEXT NOT NULL, kernel TEXT NOT NULL,
                diagnostic TEXT NOT NULL, fingerprint TEXT UNIQUE NOT NULL);
                CREATE TABLE IF NOT EXISTS history (
                issue TEXT NOT NULL REFERENCES issues(id), created TEXT NOT NULL,
                category TEXT NOT NULL, status TEXT NOT NULL, note TEXT NOT NULL);''')
        self.database.chmod(0o600)
        self.downloads = self.load_releases(Path(releases), Path(manifest))
        self.documents = json.loads(Path(__file__).with_name('content.json').read_text())['documents']

    def connect(self):
        db = sqlite3.connect(self.database, timeout=10)
        db.row_factory = sqlite3.Row
        db.execute('PRAGMA foreign_keys=ON')
        return db

    @staticmethod
    def load_releases(root, manifest):
        data = json.loads(manifest.read_text())
        if not isinstance(data, dict) or data.get('schema') != 1 or not isinstance(data.get('releases'), list):
            raise ValueError('Invalid release manifest')
        root = root.resolve()
        releases = {}
        for item in data['releases']:
            if not isinstance(item, dict):
                raise ValueError('Invalid release entry')
            platform, name = item.get('platform'), item.get('filename', '')
            if platform not in PLATFORMS or platform in releases or not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9._-]*\.(iso|img\.xz)', name):
                raise ValueError('Invalid or duplicate platform / image filename')
            path = root / name
            if path.is_symlink() or not path.is_file() or path.resolve().parent != root:
                raise ValueError('Release must be a regular file directly inside the release directory')
            if item.get('accepted') is not True or not isinstance(item.get('version'), str) or not item['version'].strip():
                raise ValueError('Publish only accepted, versioned images')
            with path.open('rb') as stream:
                digest = hashlib.file_digest(stream, 'sha256').hexdigest()
                stat = os.fstat(stream.fileno())
            if stat.st_size == 0 or item.get('sha256') != digest or item.get('bytes') != stat.st_size:
                raise ValueError('Release size or SHA-256 mismatch: ' + name)
            releases[platform] = dict(item, path=path, identity=(stat.st_dev, stat.st_ino, stat.st_size, stat.st_mtime_ns))
        return releases

    def signed(self, purpose, lifetime=3600):
        value = f'{purpose}:{int(time.time()) + lifetime}:{secrets.token_hex(16)}'
        return value + ':' + hmac.new(self.secret, value.encode(), hashlib.sha256).hexdigest()

    def valid(self, value, purpose):
        try:
            kind, expires, nonce, signature = value.rsplit(':', 3)
            message = ':'.join((kind, expires, nonce))
            return (kind == purpose and int(expires) > time.time()
                    and hmac.compare_digest(signature, hmac.new(self.secret, message.encode(), hashlib.sha256).hexdigest()))
        except (ValueError, TypeError):
            return False

    def csrf(self, purpose):
        return f'<input type="hidden" name="csrf" value="{e(self.signed(purpose))}">'

    def add_issue(self, values):
        canonical = json.dumps(values, sort_keys=True, ensure_ascii=True)
        fingerprint = hashlib.sha256(canonical.encode()).hexdigest()
        with closing(self.connect()) as db, db:
            db.execute('BEGIN IMMEDIATE')
            previous = db.execute('SELECT id FROM issues WHERE fingerprint=?', (fingerprint,)).fetchone()
            if previous:
                return previous['id'], False
            if db.execute('SELECT count(*) FROM issues').fetchone()[0] >= self.max_reports:
                raise OverflowError('Report storage is full; try again later')
            identifier = secrets.token_hex(12)
            db.execute('INSERT INTO issues VALUES (?,?,?,?,?,?,?,?,?,?,?)',
                       (identifier, now(), values['category'], 'new', values['title'], values['details'],
                        values['platform'], values['version'], values['kernel'], values['diagnostic'], fingerprint))
            db.execute('INSERT INTO history VALUES (?,?,?,?,?)',
                       (identifier, now(), values['category'], 'new', 'Report received'))
            return identifier, True


def handler_for(portal):
    class Handler(BaseHTTPRequestHandler):
        server_version = 'ModernBASIC'

        def setup(self):
            super().setup()
            self.connection.settimeout(15)

        def log_message(self, *args):
            pass  # No credentials, report bodies or remote addresses in access logs.

        def respond(self, status, body, content_type='text/html; charset=utf-8', headers=()):
            if isinstance(body, str):
                body = body.encode()
            self.send_response(status)
            for name, value in [('Content-Type', content_type), ('Content-Length', str(len(body))),
                                ('Cache-Control', 'no-store'), ('X-Content-Type-Options', 'nosniff'),
                                ('Referrer-Policy', 'no-referrer'),
                                ('Content-Security-Policy', "default-src 'none'; style-src 'self'; img-src 'self'; media-src 'self'; form-action 'self'; frame-ancestors 'none'; base-uri 'none'"), *headers]:
                self.send_header(name, value)
            self.end_headers()
            if self.command != 'HEAD':
                self.wfile.write(body)

        def media(self, name):
            content_type = MEDIA_TYPES.get(name)
            if content_type is None:
                return self.error(404, 'Media not found')
            path = MEDIA_ROOT / name
            if not path.is_file() or path.is_symlink():
                return self.error(404, 'Media not found')
            try:
                descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
            except OSError:
                return self.error(404, 'Media not found')
            with os.fdopen(descriptor, 'rb') as stream:
                stat = os.fstat(stream.fileno())
                if not S_ISREG(stat.st_mode) or stat.st_size == 0:
                    return self.error(404, 'Media not found')
                etag = f'"{stat.st_mtime_ns:x}-{stat.st_size:x}"'
                start, end, status = 0, stat.st_size - 1, 200
                requested = self.headers.get('Range') if self.command == 'GET' else None
                if requested and self.headers.get('If-Range', etag) == etag:
                    match = re.fullmatch(r'bytes=(\d*)-(\d*)', requested)
                    if not match or not any(match.groups()):
                        return self.respond(416, '', headers=[('Content-Range', f'bytes */{stat.st_size}')])
                    first, last = match.groups()
                    if first:
                        start = int(first)
                        end = min(int(last), end) if last else end
                    else:
                        start = max(0, stat.st_size - int(last))
                    if start > end or start >= stat.st_size:
                        return self.respond(416, '', headers=[('Content-Range', f'bytes */{stat.st_size}')])
                    status = 206
                self.send_response(status)
                self.send_header('Content-Type', content_type)
                self.send_header('Content-Length', str(end - start + 1))
                self.send_header('Accept-Ranges', 'bytes')
                self.send_header('ETag', etag)
                self.send_header('Cache-Control', 'public, max-age=3600')
                self.send_header('X-Content-Type-Options', 'nosniff')
                if status == 206:
                    self.send_header('Content-Range', f'bytes {start}-{end}/{stat.st_size}')
                self.end_headers()
                if self.command != 'HEAD':
                    stream.seek(start)
                    remaining = end - start + 1
                    while remaining:
                        block = stream.read(min(256 * 1024, remaining))
                        if not block:
                            break
                        try:
                            self.wfile.write(block)
                        except (BrokenPipeError, ConnectionResetError):
                            return  # A viewer may cancel a request when seeking.
                        remaining -= len(block)

        def redirect(self, path, headers=()):
            self.respond(303, '', headers=[('Location', path), *headers])

        def error(self, status, message):
            self.respond(status, page('Request could not be completed', f'<section class="narrow"><h1>Request could not be completed</h1><p>{e(message)}</p><a href="/report">Return to the report form</a></section>'))

        def authenticated(self):
            try:
                cookie = SimpleCookie(self.headers.get('Cookie', ''))
                return 'mb_session' in cookie and portal.valid(cookie['mb_session'].value, 'session')
            except CookieError:
                return False

        def body(self):
            lengths = self.headers.get_all('Content-Length', [])
            if self.headers.get('Transfer-Encoding') or len(lengths) != 1:
                raise ValueError('One Content-Length header is required')
            length = int(lengths[0])
            if not 0 < length <= MAX_BODY:
                raise ValueError('Report must be at most 2 MiB')
            data = self.rfile.read(length)
            if len(data) != length:
                raise ValueError('Incomplete request')
            return data

        def form(self, purpose):
            if self.headers.get_content_type() != 'application/x-www-form-urlencoded':
                raise ValueError('Expected a browser form')
            origin = self.headers.get('Origin')
            if origin and origin != portal.origin:
                raise ValueError('Form origin does not match this site')
            fields = parse_qs(self.body().decode('utf-8'), keep_blank_values=True, max_num_fields=20)
            if any(len(v) != 1 for v in fields.values()):
                raise ValueError('Duplicate form fields')
            fields = {k: v[0] for k, v in fields.items()}
            if not portal.valid(fields.get('csrf', ''), purpose):
                raise ValueError('Form expired. Reload the page and try again')
            return fields

        def do_HEAD(self):
            self.do_GET()

        def do_GET(self):
            try:
                self.get()
            except (OSError, sqlite3.Error):
                self.error(503, 'Storage is temporarily unavailable')
            except (ValueError, KeyError):
                self.error(400, 'Invalid request')

        def get(self):
            parsed = urlsplit(self.path)
            path = parsed.path
            if path.startswith('/media/'):
                return self.media(path.removeprefix('/media/'))
            if path == '/style.css':
                return self.respond(200, ASSETS.read_bytes(), 'text/css; charset=utf-8')
            if path == '/health':
                with closing(portal.connect()) as db:
                    db.execute('SELECT count(*) FROM issues').fetchone()
                return self.respond(200, '{"status":"ready"}', 'application/json')
            public = site_pages.public_page(path, parsed.query, portal)
            if public is not None:
                title, body = public
                return self.respond(200, page(title, body))
            if path.startswith('/downloads/') or path.startswith('/checksums/'):
                name = path.rsplit('/', 1)[-1]
                item = next((v for v in portal.downloads.values() if v['filename'] == name), None)
                if not item:
                    return self.error(404, 'Image is not published')
                if path.startswith('/checksums/'):
                    return self.respond(200, f'{item["sha256"]}  {name}\n', 'text/plain; charset=utf-8')
                # Release directory is operator-owned and immutable while served.
                descriptor = os.open(item['path'], os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
                with os.fdopen(descriptor, 'rb') as stream:
                    stat = os.fstat(stream.fileno())
                    if (stat.st_dev, stat.st_ino, stat.st_size, stat.st_mtime_ns) != item['identity']:
                        return self.error(503, 'Image changed; publication must be revalidated')
                    start, end, status = 0, stat.st_size - 1, 200
                    etag = '"' + item['sha256'] + '"'
                    requested = self.headers.get('Range') if self.command == 'GET' else None
                    if requested and self.headers.get('If-Range', etag) == etag:
                        match = re.fullmatch(r'bytes=(\d*)-(\d*)', requested)
                        if not match or not any(match.groups()):
                            return self.respond(416, '', headers=[('Content-Range', f'bytes */{stat.st_size}')])
                        first, last = match.groups()
                        if first:
                            start = int(first)
                            end = min(int(last), end) if last else end
                        else:
                            start = max(0, stat.st_size - int(last))
                        if start > end or start >= stat.st_size:
                            return self.respond(416, '', headers=[('Content-Range', f'bytes */{stat.st_size}')])
                        status = 206
                    self.send_response(status)
                    self.send_header('Content-Type', 'application/octet-stream')
                    self.send_header('Content-Length', str(end - start + 1))
                    self.send_header('Accept-Ranges', 'bytes')
                    self.send_header('ETag', etag)
                    if status == 206:
                        self.send_header('Content-Range', f'bytes {start}-{end}/{stat.st_size}')
                    self.send_header('Content-Disposition', f'attachment; filename="{name}"')
                    self.send_header('X-Content-Type-Options', 'nosniff')
                    self.end_headers()
                    if self.command != 'HEAD':
                        stream.seek(start)
                        remaining = end - start + 1
                        while remaining:
                            chunk = stream.read(min(1024 * 1024, remaining))
                            if not chunk:
                                break
                            self.wfile.write(chunk)
                            remaining -= len(chunk)
                return
            if path == '/privacy':
                return self.respond(200, page('Report privacy', '<section class="narrow"><h1>Your report stays private.</h1><p>Report details, diagnostics, and developer notes are accessible only to the developer team. We do not request your name or email address.</p><p>Include a small reproducer and remove passwords, tokens, private programs and identifying log entries. Diagnostics are optional. Reports remain on the project server until the operator removes them under its retention policy.</p><p>Automatic OS failure reports stay on your computer. Uploading requires your explicit confirmation. Developers may reclassify a report; a suspected kernel issue is not automatically sent upstream.</p></section>'))
            if path == '/report':
                return self.respond(200, page('Report a bug', f'''<section class="narrow"><p class="eyebrow">HELP IMPROVE MODERNBASIC</p><h1>Tell us what happened.</h1><p>Choose the area that fits best. “Unsure” is fine; developers can reclassify your report. Reports are private.</p><form method="post" action="/report">{portal.csrf('report')}
<label>Area<select name="category" required>{options(CATEGORIES, 'unsure')}</select></label><p class="hint">Linux kernel: drivers, kernel panic, hardware faults. BASIC: language, editor, graphics or sound APIs. OS integration: installer, networking, startup or packages.</p>
<label>Short summary<input name="title" maxlength="160" required></label>
<div class="form-grid"><label>Platform<select name="platform" required>{options(dict([('unknown', 'Unsure')] + [(k, v[0]) for k, v in PLATFORMS.items()]))}</select></label><label>ModernBASIC version / build<input name="version" maxlength="100" placeholder="For example, 0.8.2-r12"></label></div>
<label>Linux kernel version, if known<input name="kernel" maxlength="200" placeholder="Shown by uname -r in BASH"></label>
<label>Steps to reproduce, expected result, and actual result<textarea name="details" rows="9" maxlength="20000" required></textarea></label>
<label>Optional diagnostic JSON<textarea name="diagnostic" rows="4" maxlength="1000000" spellcheck="false"></textarea></label><p class="hint">From BASH, use <code>diagnostics report --category basic</code> (or kernel, integration, unsure), then preview the saved file. Remove private information before pasting.</p>
<label class="check"><input type="checkbox" name="consent" value="yes" required> I have reviewed this report and agree to send it to the developer team.</label><button type="submit">Send report →</button></form></section>'''))
            if path == '/login':
                return self.respond(200, page('Developer sign in', f'<section class="narrow"><h1>Developer sign in</h1><form method="post" action="/login">{portal.csrf("login")}<label>Developer access key<input name="credential" type="password" autocomplete="current-password" required maxlength="256"></label><button>Sign in</button></form></section>'))
            if path == '/developer' or path.startswith('/developer/'):
                if not self.authenticated():
                    return self.redirect('/login')
                if path == '/developer':
                    filters = parse_qs(parsed.query, max_num_fields=3)
                    category = filters.get('category', [''])[0]
                    status = filters.get('status', [''])[0]
                    offset = max(0, int(filters.get('offset', ['0'])[0]))
                    if category and category not in CATEGORIES or status and status not in STATUSES:
                        raise ValueError('Invalid filter')
                    with closing(portal.connect()) as db:
                        rows = db.execute('SELECT * FROM issues WHERE (?="" OR category=?) AND (?="" OR status=?) ORDER BY created DESC, id DESC LIMIT 101 OFFSET ?', (category, category, status, status, offset)).fetchall()
                        counts = dict(db.execute('SELECT category,count(*) FROM issues GROUP BY category').fetchall())
                    queues = ''.join(f'<a class="queue" href="/developer?category={key}"><strong>{counts.get(key, 0)}</strong>{label}</a>' for key, label in CATEGORIES.items())
                    table = ''.join(f'<tr><td><a href="/developer/{r["id"]}">{e(r["title"])}</a><small>{e(r["id"])} · {e(r["created"])}</small></td><td>{e(CATEGORIES[r["category"]])}</td><td>{e(r["status"])}</td><td>{e(r["platform"])}</td></tr>' for r in rows[:100])
                    pagination = f'<a href="/developer?category={category}&amp;status={status}&amp;offset={offset+100}">Next 100 reports →</a>' if len(rows) > 100 else ''
                    return self.respond(200, page('Developer queues', f'<section><p class="eyebrow">PRIVATE DEVELOPER WORKSPACE</p><h1>Reports to resolve.</h1><div class="grid queues">{queues}</div><form class="filters" method="get"><label>Area<select name="category"><option value="">All areas</option>{options(CATEGORIES, category)}</select></label><label>Status<select name="status"><option value="">All statuses</option>{options(STATUSES, status)}</select></label><button>Filter</button></form><div class="table-scroll"><table><thead><tr><th>Report</th><th>Area</th><th>Status</th><th>Platform</th></tr></thead><tbody>{table or "<tr><td colspan=4>No matching reports.</td></tr>"}</tbody></table></div>{pagination}<form method="post" action="/logout">{portal.csrf("logout")}<button class="secondary">Sign out</button></form></section>'))
                identifier = path.removeprefix('/developer/')
                with closing(portal.connect()) as db:
                    report = db.execute('SELECT * FROM issues WHERE id=?', (identifier,)).fetchone()
                    history = db.execute('SELECT * FROM history WHERE issue=? ORDER BY rowid', (identifier,)).fetchall()
                if not report:
                    return self.error(404, 'Report not found')
                events = ''.join(f'<li><strong>{e(r["created"])}</strong> · {e(CATEGORIES[r["category"]])} · {e(r["status"])}<p>{e(r["note"])}</p></li>' for r in history)
                return self.respond(200, page(report['title'], f'''<section class="narrow wide"><a href="/developer">← Developer queues</a><h1>{e(report['title'])}</h1><p>{e(identifier)} · {e(report['platform'])} · Build {e(report['version'])} · Kernel {e(report['kernel'])}</p><h2>Reproduction</h2><pre class="report-text">{e(report['details'])}</pre><details><summary>Diagnostic JSON (private)</summary><pre class="report-text">{e(report['diagnostic'] or 'No diagnostic attached')}</pre></details><h2>Triage</h2><form method="post" action="/developer/{e(identifier)}">{portal.csrf('triage:'+identifier)}<div class="form-grid"><label>Area<select name="category">{options(CATEGORIES, report['category'])}</select></label><label>Status<select name="status">{options(STATUSES, report['status'])}</select></label></div><label>Developer note / upstream reference<textarea name="note" rows="4" maxlength="4000" required></textarea></label><button>Save triage update</button></form><h2>History</h2><ol class="history">{events}</ol></section>'''))
            return self.error(404, 'Page not found')

        def do_POST(self):
            try:
                self.post()
            except (ValueError, UnicodeError, RecursionError):
                self.error(400, 'Invalid or expired form. Check required fields and diagnostic JSON, then reload the form')
            except OverflowError:
                self.error(503, 'Report storage is full; try again later')
            except (OSError, sqlite3.Error):
                self.error(503, 'Storage is temporarily unavailable')

        def post(self):
            path = urlsplit(self.path).path
            if path == '/login':
                fields = self.form('login')
                if not hmac.compare_digest(fields.get('credential', '').encode(), portal.secret):
                    return self.error(403, 'Access key was not accepted')
                cookie = 'mb_session=' + portal.signed('session', 8 * 3600) + '; HttpOnly; SameSite=Strict; Path=/; Max-Age=28800' + ('; Secure' if portal.secure else '')
                return self.redirect('/developer', [('Set-Cookie', cookie)])
            if path == '/logout' or path.startswith('/developer/'):
                if not self.authenticated():
                    return self.error(403, 'Developer sign in required')
                if path == '/logout':
                    self.form('logout')
                    return self.redirect('/', [('Set-Cookie', 'mb_session=; Path=/; Max-Age=0; HttpOnly; SameSite=Strict')])
                identifier = path.removeprefix('/developer/')
                fields = self.form('triage:'+identifier)
                if fields.get('category') not in CATEGORIES or fields.get('status') not in STATUSES or not 1 <= len(fields.get('note', '').strip()) <= 4000:
                    raise ValueError('Invalid triage update')
                with closing(portal.connect()) as db, db:
                    changed = db.execute('UPDATE issues SET category=?,status=? WHERE id=?', (fields['category'], fields['status'], identifier)).rowcount
                    if not changed:
                        return self.error(404, 'Report not found')
                    db.execute('INSERT INTO history VALUES (?,?,?,?,?)', (identifier, now(), fields['category'], fields['status'], fields['note']))
                return self.redirect('/developer/'+identifier)
            if path == '/v1/reports':
                # Optional private machine intake, compatible with the OS sender.
                if not portal.intake_token or not hmac.compare_digest(self.headers.get('Authorization', '').encode(), ('Bearer '+portal.intake_token).encode()):
                    return self.respond(401, '{"error":"authentication required"}', 'application/json')
                if self.headers.get_content_type() != 'application/json':
                    raise ValueError('JSON required')
                report = validate_diagnostic(self.body())
                values = dict(category=report.get('category', 'unsure'), title=report['reason'][:160] or 'Diagnostic report', details=report['reason'], platform=report['architecture'], version=str(report.get('distribution', {}).get('BUILD_ID', '')), kernel=report['kernel'], diagnostic=json.dumps(report, ensure_ascii=True))
                identifier, created = portal.add_issue(values)
                return self.respond(201 if created else 200, json.dumps({'report_id': identifier, 'status': 'received'}), 'application/json')
            if path != '/report':
                return self.error(404, 'Page not found')
            fields = self.form('report')
            values = {key: fields.get(key, '').strip() for key in ('category', 'title', 'details', 'platform', 'version', 'kernel', 'diagnostic')}
            if (fields.get('consent') != 'yes' or values['category'] not in CATEGORIES
                    or values['platform'] not in (*PLATFORMS, 'unknown')
                    or not 1 <= len(values['title']) <= 160 or not 1 <= len(values['details']) <= 20000
                    or len(values['version']) > 100 or len(values['kernel']) > 200):
                raise ValueError('Invalid report fields')
            if values['diagnostic']:
                diagnostic = validate_diagnostic(values['diagnostic'])
                values['diagnostic'] = json.dumps(diagnostic, ensure_ascii=True)
            identifier, _ = portal.add_issue(values)
            return self.respond(201, page('Report received', f'<section class="narrow"><p class="eyebrow">THANK YOU</p><h1>Your report is in the queue.</h1><p>Reference: <strong>{identifier}</strong></p><p>Area: {e(CATEGORIES[values["category"]])}. Keep this reference for any follow-up report. The developer team can now review it.</p><a class="button" href="/">Back to ModernBASIC</a></section>'))
    return Handler


def validate_diagnostic(payload):
    report = json.loads(payload)
    if not isinstance(report, dict) or report.get('schema') != 1:
        raise ValueError('Unsupported diagnostic schema')
    for key in ('created_utc', 'architecture', 'kernel', 'reason'):
        if not isinstance(report.get(key), str) or len(report[key]) > 300:
            raise ValueError('Invalid diagnostic field')
    if report.get('category', 'unsure') not in tuple(CATEGORIES) or not isinstance(report.get('distribution', {}), dict):
        raise ValueError('Invalid diagnostic category or distribution')
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--storage', type=Path, required=True)
    parser.add_argument('--releases', type=Path, required=True)
    parser.add_argument('--manifest', type=Path, required=True)
    parser.add_argument('--secret-file', type=Path, required=True)
    parser.add_argument('--intake-token-file', type=Path)
    parser.add_argument('--origin', required=True)
    parser.add_argument('--port', type=int, default=8081)
    args = parser.parse_args()
    os.umask(0o077)
    secret = args.secret_file.read_text().strip()
    intake = args.intake_token_file.read_text().strip() if args.intake_token_file else ''
    if args.intake_token_file and (len(intake) < 32 or '\r' in intake or '\n' in intake):
        parser.error('Intake credential must contain at least 32 characters on one line')
    portal = Portal(args.storage, args.releases, args.manifest, secret, args.origin, intake)
    server = ThreadingHTTPServer(('127.0.0.1', args.port), handler_for(portal))
    print(f'ModernBASIC portal listening on 127.0.0.1:{args.port}', flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == '__main__':
    main()
