# ModernBASIC central website

A dependency-free Python 3.11+ service for platform downloads, public bug
submission and private developer triage. It has no external trackers, scripts,
fonts or analytics. The default manifest deliberately publishes no images:
public release licensing, signing, hosting and hardware acceptance remain open.
Development revision-13.1 images have passed the recorded disposable-VM checks.

## Report queues

- **Linux kernel** (`kernel`): suspected kernel/driver/panic problems.
- **Modern BASIC environment** (`basic`): interpreter, statements, functions,
  editor, shell, graphics and sound APIs.
- **OS integration** (`integration`): installer, boot/service wiring, networking,
  packages, and distribution configuration.
- **Unsure** (`unsure`): unknown causes and legacy uncategorized diagnostics.

Category is a triage choice, not proof of root cause. Kernel messages in an
attachment do not automatically classify an issue. Developers can filter each
queue, change category/status, and append notes or upstream references. The
original report and every triage update remain in history. Statuses are new,
confirmed, in-progress, needs-information, fixed, and closed. Nothing is sent to
upstream developers automatically. Developer access uses a shared random key;
triage history records changes but does not identify individual developers.

Public users submit a summary, reproduction steps, platform/build/kernel, and
optional reviewed diagnostic JSON. They must explicitly consent. Details and
logs remain private; there is no public issue lookup or email collection. The
receipt provides a reference to quote in a later report. Byte-identical content
is deduplicated. The service caps storage at 10,000 reports (up to 2 MiB each);
monitor disk space and set an operator retention policy. Review privacy text
before public deployment if your policy differs.

## Local preview

Run from the repository root (these commands create no image):

```sh
mkdir -p /tmp/modernbasic-portal-preview/releases
python3 -c 'import secrets,pathlib; p=pathlib.Path("/tmp/modernbasic-portal-preview/developer.key"); p.touch(mode=0o600, exist_ok=False); p.write_text(secrets.token_hex(32)+"\n")'
python3 services/portal/app.py \
  --storage /tmp/modernbasic-portal-preview/data \
  --releases /tmp/modernbasic-portal-preview/releases \
  --manifest services/portal/releases.json \
  --secret-file /tmp/modernbasic-portal-preview/developer.key \
  --origin http://127.0.0.1:8081 --port 8081
```

Open `http://127.0.0.1:8081`. The developer key stays in the local file; enter it
at `/login`. It is not a public demo password. Session cookies expire after
eight hours. Rotating the key and restarting invalidates existing sessions and
forms. Use HTTPS for anything beyond a local preview.

## Server deployment

The loopback Python server is an application backend. Run it behind the supplied
nginx HTTPS configuration; buffering, header/body deadlines, connection limits,
and request limits are part of the deployment. Do not expose port 8081 publicly.
No domain, hosting credentials, endpoint or TLS certificate is bundled.

1. Create a dedicated `modernbasic-portal` system user. Install `app.py`, `site_pages.py`, `content.json` and
   `style.css` and the complete `media/` directory under `/opt/modernbasic-portal`,
   owned by root, readable by that
   user. Install the service unit in `/etc/systemd/system`.
2. Create `/etc/modernbasic-portal` owned by root and group `modernbasic-portal`,
   mode 0750. Generate a 32-byte random hex key as `developer.key`, mode 0640,
   root:`modernbasic-portal`. Do not reuse an OS upload key or an ordinary password.
3. Copy `releases.json` into that directory. Write `portal.env` containing
   `PORTAL_ORIGIN=https://YOUR-DOMAIN` without a trailing slash. Create
   `/srv/modernbasic/releases`, root-owned and readable by the service user.
   The portal must have no write permission on images, manifest, key or code.
4. Configure DNS and a TLS certificate for your domain. Adapt
   `nginx.conf.example`, including all domain/certificate occurrences. Check
   `nginx -t` and `systemd-analyze verify /etc/systemd/system/modernbasic-portal.service`.
   Enable/start the portal service and reload nginx. Open only HTTPS (and HTTP
   for redirect/certificate setup) in the server firewall.
5. Check `/health`, submit a non-sensitive test report, sign in, move it between
   queues and change its status. Confirm unauthenticated access cannot see
   `/developer` or report details. Check HTTP redirects to HTTPS, TLS validity,
   body limits, and rate-limit responses on the deployed domain.
6. Back up `issues.sqlite3` with SQLite's online backup API or while the service
   is stopped. Encrypt backups, restrict access, and test restoration. Monitor
   service failures, disk capacity and `/health`. This application is intended
   for one service instance using local storage, not an NFS database or replicas.

The optional `/v1/reports` machine endpoint is disabled without a distinct
`--intake-token-file`. To enable it, install a separate protected token, add the
argument via the service's `ExecStart` override, and configure the OS diagnostic
sender separately with that token and the HTTPS endpoint. Tokens are never baked
into public images. The normal website form works without machine credentials.
The older `services/diagnostic-server` remains a standalone minimal private intake;
it is not used as the portal database and old report files are not auto-imported.

## Publishing images after OS corrections and acceptance

Do not build or publish images as part of website setup. After the OS is fixed,
build from a frozen source snapshot and run the exact-image and architecture
acceptance described in `docs/RELEASE_READINESS.md`. Record hardware/firmware
limitations and preserve evidence. The portal does not perform boot acceptance;
`accepted: true` is the release operator's assertion that those gates passed.

Copy each accepted `.iso` or `.img.xz` to the release directory using a temporary
name, then rename it into place. Never alter a published image in place. Calculate
its SHA-256 and byte count, and atomically replace the manifest with entries like:

```json
{
  "schema": 1,
  "releases": [{
    "platform": "amd64",
    "version": "REPLACE-WITH-ACCEPTED-BUILD",
    "filename": "modernbasic-amd64-REPLACE.iso",
    "bytes": 123,
    "sha256": "REPLACE-WITH-64-HEX-DIGEST",
    "accepted": true
  }]
}
```

Platform keys are `amd64`, `arm64`, `riscv64`, `pi5`; one current image per platform.
The example is intentionally invalid until real values replace placeholders.
Restart the service after publication. It hashes every image before serving,
refuses a mismatched manifest, rejects symlinks, and refuses images whose file
identity/size/mtime changes while running. Only manifest-listed files are exposed.
Keep images immutable, retain their acceptance reports, and verify a downloaded
copy. SHA-256 detects corruption; it is not a substitute for release signing or
trust in the HTTPS site. Platforms without a validated image remain unavailable.
Large downloads stream with bounded memory; single byte ranges support resume.

## Verification

```sh
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s tests -p portal_tests.py
```

The repository `verify.sh` includes these HTTP integration tests. They cover
consent, CSRF, credential and cookie rejection, private queues, reclassification,
history, escaping, deduplication, storage limits, old/new diagnostic formats,
download bytes/checksums, path traversal, changed artifacts and persistence.

## Distro website and user manual

The public site includes `/about`, `/downloads` with platform selection, `/start`
with live/installation guidance, `/manual` with chapter navigation and full-text
search, `/releases`, `/support`, and `/licenses`. Report intake and private
triage retain their existing routes and authentication.

Generate the deployable documentation bundle from the repository sources:

```sh
python3 services/portal/build_content.py
python3 services/portal/build_content.py --check
```

Deploy `site_pages.py` and `content.json` beside `app.py` and `style.css`.
The generated bundle includes the user guide, language reference, audio reference,
editor guide, changelog and license status; it does not require a source checkout
on the web server. Raw HTML is escaped and documentation links are restricted.
Rebuild the bundle whenever its source documents change. No external JavaScript,
font service or documentation hosting service is required. Manual pages have a
print stylesheet for browser printing or saving as PDF.

The OS's new optional consent dialog uses the existing authenticated intake.
A root-controlled endpoint and separate intake credential must be provisioned
before the target's notification timer is enabled. The default OS configuration
keeps this disabled, and this website does not add unauthenticated machine intake.
See `appliance/distro/diagnostics/README.md` for the consent and deployment flow.

## Consistent database backup and restore

`backup.py` uses SQLite's online backup API, verifies database integrity, creates
mode-0600 output and refuses to overwrite a previous backup. Run it as the portal
account into a private backup directory managed by the operator:

```sh
python3 /opt/modernbasic-portal/backup.py \
  /var/lib/modernbasic-portal/issues.sqlite3 \
  /private-backups/issues-YYYYMMDDTHHMMSS.sqlite3
```

Include `backup.py` with the deployed application. Schedule the command after
selecting the production backup location and retention policy; monitor exit
status, age of the newest successful backup, free space and external `/health`
HTTPS response. An on-server copy alone is not disaster recovery: transfer an
encrypted copy to independent storage. Back up deployment configuration,
release manifests and credentials separately with equivalent access controls.

To restore, stop `modernbasic-portal`, retain the failed/current database for
investigation, and install a verified snapshot as `issues.sqlite3`, owner
`modernbasic-portal`, mode 0600, in the private state directory. Retain/remove any
old SQLite WAL/SHM sidecars together with the old database while the service is
stopped. Restart and verify private queue/history access and a synthetic consented
submission. Rotate credentials if restoring after suspected compromise.

`tests/portal_backup_tests.py` exercises a live WAL-backed database, restoration
of reports/history, private output permissions, collision rejection and corrupt
source failure. Production scheduling, off-server storage and alert delivery
still depend on the owner-selected hosting environment.

## Intro film

The homepage includes a 48-second 1080p introduction with real BASIC demos and
an original synth soundtrack. The player has native controls, English captions,
a transcript and a download link; it does not autoplay or preload the movie.
The three allowlisted files in `media/` must be deployed together. The backend
supports byte-range requests for seeking. No external video host is required.
See `tools/intro-video/README.md` in the source workspace for capture/render
commands and asset provenance.
