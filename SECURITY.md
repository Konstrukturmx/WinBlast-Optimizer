# Security Policy

## Supported versions

| Version | Supported |
|---|---|
| 1.7.x (latest release) | ✅ |
| 1.6.x and older | ❌ — please update |

## Reporting a vulnerability

Please **don't open a public issue** for security problems. Use GitHub's private reporting instead: **Security → Report a vulnerability** on this repository.

Include:
- the version (Support & About shows it) and Windows build,
- what an attacker can do and what they need (local user, admin, network position),
- steps to reproduce or a proof of concept.

You'll get a reply within 7 days. Fixes are released as a new signed version and credited in the changelog unless you prefer otherwise.

## What WinBlast0 does to stay safe

- **Signed releases.** `winblast0.exe` is Authenticode-signed (certificate thumbprint `070A70E017B5BB8F7A345688C82D8A578DAC404D`, DigiCert timestamp). Each release lists its SHA-256 and a VirusTotal report.
- **Pinned updates.** The updater talks only to this repository over HTTPS, accepts only `.exe` assets from its `releases/download/` URL, stages them in an admin-only folder and installs only if the signature matches the pinned certificate, checked after download and again before the swap. Nothing installs without clicking Restart.
- **Built-in catalog.** Release builds use only the tweak catalog compiled into the exe; rules files placed next to it are ignored.
- **No hijackable tool paths.** Windows tools are started by their full System32 path, with a timeout.
- **Recorded changes.** Every system change is recorded in the undo ledger (`C:\ProgramData\WinBlast0`) before it is made.
- **No telemetry.** WinBlast0 sends nothing about you or your PC anywhere.

## Scope

In scope: the WinBlast0 executable, its updater and the tweak catalog. Out of scope: the effects of a tweak that behaves as documented (each one lists its risk and what it breaks), and issues that need an already-compromised administrator account.
