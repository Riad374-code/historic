# Historic

Historic is a Tauri desktop application (Vue + TypeScript + Rust) for extracting clean content from a URL and exporting results.

## Features

- Desktop app for Windows, macOS, and Linux.
- Clean extraction flow from a source link.
- Export workflow with PDF support.
- Open source and free to use.

## License

This project is licensed under the MIT License.
See [LICENSE](LICENSE).

## Development Setup

### Prerequisites

- Node.js 20+
- npm 10+
- Rust stable
- Tauri system dependencies for your operating system

### Install dependencies

```bash
npm ci
```

### Run the app in development

```bash
npm run tauri dev
```

## Build

Create production installers locally:

```bash
npm run tauri build
```

## Release Process (GitHub)

This repository uses a GitHub Actions workflow at [.github/workflows/publish.yml](.github/workflows/publish.yml).

### Trigger rule

Releases are built only when you push a version tag matching `v*`, for example:

```bash
git tag v0.1.1
git push origin v0.1.1
```

### Safety checks in CI

Before publishing, CI validates that:

- The git tag version matches [package.json](package.json).
- Version values are consistent across:
	- [package.json](package.json)
	- [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json)
	- [src-tauri/Cargo.toml](src-tauri/Cargo.toml)

### Release output

The workflow creates a draft GitHub Release with platform artifacts for:

- Windows
- macOS
- Linux

## Code Signing (Recommended for Public Distribution)

Unsigned apps may show trust warnings on user machines.
To reduce warnings and improve trust, configure signing secrets described in:

- [.github/release-signing-secrets.md](.github/release-signing-secrets.md)

## Contributors

Contribution guidance is available at [CONTRIBUTING.md](CONTRIBUTING.md).
