# WinBlast Optimizer

A modern, native Windows system optimizer, debloater, and hardware tuning tool written in pure Rust.

## Features

- **Tweak Matrix**: Over 160 configurable system and privacy tweaks.
- **Hardware Telemetry**: NT kernel scheduler metrics for per-core utilization and latency monitoring.
- **CPU Core Parking**: Unpark and manage logical cores on standard and hybrid CPU topologies.
- **GPU Tuning**: Gated controls for vendor-specific graphics architectures.
- **Clean & Apps**: Built-in UWP package debloater and WinGet package manager.
- **Multi-language**: Full internationalization across 8 languages.

## Security & VirusTotal Analysis

Every release is scanned for security and transparency:

- **Executable**: `winblast0.exe` (v0.9.0)
- **SHA-256**: `19895c54d3699391aebedb5c3e384aeb7ac5a73f8d1bee9ff7da4cd623d2fa00`
- **VirusTotal Report**: [View Scan Results](https://www.virustotal.com/gui/file/19895c54d3699391aebedb5c3e384aeb7ac5a73f8d1bee9ff7da4cd623d2fa00)

## Building from Source

Prerequisites:
- Windows 10/11 (x64)
- Rust toolchain (`cargo`, `rustc`)

```bash
cargo build --release
```

The resulting executable will be located at `target/release/winblast0.exe`.

## License

This project is licensed under the open MIT License. See [LICENSE](LICENSE) for details.
