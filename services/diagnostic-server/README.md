# Diagnostic reporting server skeleton

This optional Python 3 service is not deployed and no endpoint or credential is
included in the distribution. It accepts the same schema-1 report produced by
`diagnostics report` on x86-64, ARM64 and RISC-V64.

## Local development

```sh
umask 077
python3 -c 'import secrets; print(secrets.token_urlsafe(32))' > reporting.token
python3 server.py --token-file reporting.token --storage reports
```

`GET /health` is a readiness check. `POST /v1/reports` requires a bearer token,
`Content-Type: application/json` and `Content-Length` between 1 byte and 2 MiB.
Accepted reports return HTTP 201 with a SHA-256 report ID; retries return 200 for
the same content. Reports are stored in private JSON files. There is no public
report listing or automated issue creation. Server logs omit request addresses,
authorization and report contents.

## Connecting it later

1. Deploy on a Linux server with private persistent storage.
2. Put an HTTPS reverse proxy in front of localhost port 8080; configure request
   limits, rate limits and operational retention/backups there.
3. Set the client `endpoint` to `https://YOUR-SERVER/v1/reports` in
   `/etc/modernbasic/diagnostics.json`, and `token_file` to a root-readable file
   containing the configured token. Do not bake shared credentials into ISOs.
4. Run `diagnostics send REPORT.json`, review the preview and type SEND.

The standard-library server is an intake skeleton, not a complete support portal.
Token provisioning/rotation, staff access, triage, dashboards and retention policy
are deployment decisions to implement when the central service is established.
