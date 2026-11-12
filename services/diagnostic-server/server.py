#!/usr/bin/env python3
"""Minimal private diagnostic intake; place behind an HTTPS reverse proxy."""
import argparse
import datetime
import hashlib
import hmac
from http.server import BaseHTTPRequestHandler, HTTPServer
import json
import os
from pathlib import Path
import tempfile

MAX_BODY = 2 * 1024 * 1024


def validate_report(payload):
    report = json.loads(payload)
    if not isinstance(report, dict) or report.get('schema') != 1:
        raise ValueError('unsupported report schema')
    for name in ('created_utc', 'architecture', 'kernel', 'reason'):
        if not isinstance(report.get(name), str) or len(report[name]) > 300:
            raise ValueError('invalid ' + name)
    if report.get('category', 'unsure') not in ('kernel', 'basic', 'integration', 'unsure'):
        raise ValueError('invalid category')
    return report


def make_handler(storage, token):
    class Handler(BaseHTTPRequestHandler):
        def log_message(self, format, *args):
            # Do not log remote addresses, credentials or uploaded report text.
            pass

        def respond(self, status, value):
            body = json.dumps(value).encode()
            self.send_response(status)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Content-Length', str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_GET(self):
            self.respond(200 if self.path == '/health' else 404,
                         {'status': 'ready'} if self.path == '/health' else {'error': 'not found'})

        def do_POST(self):
            if self.path != '/v1/reports':
                return self.respond(404, {'error': 'not found'})
            if not hmac.compare_digest(self.headers.get('Authorization', '').encode(), ('Bearer ' + token).encode()):
                return self.respond(401, {'error': 'authentication required'})
            if self.headers.get('Transfer-Encoding'):
                return self.respond(400, {'error': 'Content-Length required'})
            if self.headers.get_content_type() != 'application/json':
                return self.respond(415, {'error': 'application/json required'})
            try:
                length = int(self.headers.get('Content-Length', '0'))
            except ValueError:
                length = 0
            if not 0 < length <= MAX_BODY:
                return self.respond(413, {'error': 'invalid report size'})
            self.connection.settimeout(20)
            try:
                payload = self.rfile.read(length)
                if len(payload) != length:
                    raise ValueError('incomplete body')
                report = validate_report(payload)
            except (OSError, ValueError):
                return self.respond(400, {'error': 'invalid diagnostic report'})
            identifier = hashlib.sha256(payload).hexdigest()
            destination = storage / (identifier + '.json')
            envelope = {'received_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'report': report}
            temporary = None
            status, response = 201, {'report_id': identifier, 'status': 'received'}
            try:
                # Publish only complete reports, including when a client retries.
                with tempfile.NamedTemporaryFile(mode='w', encoding='utf8',
                                                 dir=storage, prefix='.report-',
                                                 delete=False) as stream:
                    temporary = Path(stream.name)
                    json.dump(envelope, stream, indent=2)
                    stream.flush()
                    os.fsync(stream.fileno())
                os.link(temporary, destination)
            except FileExistsError:
                status, response = 200, {'report_id': identifier, 'status': 'already received'}
            except OSError:
                status, response = 503, {'error': 'storage unavailable'}
            finally:
                if temporary is not None:
                    try:
                        temporary.unlink(missing_ok=True)
                    except OSError:
                        status, response = 503, {'error': 'storage unavailable'}
            # Send completion only after temporary-file cleanup.
            self.respond(status, response)
    return Handler


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--port', type=int, default=8080)
    parser.add_argument('--storage', type=Path, default=Path('reports'))
    parser.add_argument('--token-file', type=Path, required=True)
    args = parser.parse_args()
    os.umask(0o077)
    token = args.token_file.read_text().strip()
    if len(token) < 32 or '\n' in token or '\r' in token:
        parser.error('use a token of at least 32 characters without newlines')
    args.storage.mkdir(parents=True, exist_ok=True, mode=0o700)
    server = HTTPServer(('127.0.0.1', args.port), make_handler(args.storage, token))
    print('Diagnostic intake listening on localhost:' + str(args.port), flush=True)
    server.serve_forever()

if __name__ == '__main__':
    main()
