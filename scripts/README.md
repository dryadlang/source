# Dryad Installer Scripts

This directory contains scripts for automatic installation of the Dryad programming language.

## Usage

### Unix-like (Linux/macOS)

```bash
curl -fsSL https://dryadlang.org/install.sh | bash
```

### Windows (PowerShell)

```powershell
iwr -useb https://dryadlang.org/install.ps1 | iex
```

## Hosting Requirements

The scripts assume binaries are hosted on GitHub Releases (or a similar structure) with the following naming convention:

- `dryad-linux-x86_64`
- `dryad-macos-arm64`
- `dryad-windows-x86_64.exe`
- `oak-linux-x86_64`
- ... etc

Update the `BASE_URL` variable in both scripts to point to your actual hosting location.

## Local Testing

To test the Windows installer locally using the existing binaries in `dryad_release/`:

1. Copy `dryad.exe` and `oak.exe` to a temporary "server" folder.
2. Update `install.ps1` to use a local `file://` URL or a simple local HTTP server.
3. Run the script in a PowerShell terminal.
