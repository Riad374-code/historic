# Contributing

Thanks for contributing to Historic.

## Development Setup

### Prerequisites
- Node.js 20+
- npm 10+
- Rust (stable) with Cargo
- Tauri system prerequisites for your OS

### Install
1. Clone the repository.
2. Install frontend dependencies:
   - npm ci
3. Build/check Rust dependencies when needed:
   - cd src-tauri
   - cargo check

## Run Locally

### Frontend only
- npm run dev

### Full desktop app (Tauri + Vue)
- npm run tauri dev

## Build
- npm run build
- cd src-tauri && cargo check

## Release Notes for Maintainers
- Publishing is tag-driven via [.github/workflows/publish.yml](.github/workflows/publish.yml).
- Create and push a semantic tag like `v0.1.1` to start a draft release build.
- Keep versions aligned in:
   - [package.json](package.json)
   - [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json)
   - [src-tauri/Cargo.toml](src-tauri/Cargo.toml)

## Pull Request Guidelines
1. Create a focused branch per change.
2. Keep commits small and descriptive.
3. Run the build/check commands before opening a PR.
4. Include a short PR description: what changed and why.
5. Add screenshots for UI changes.

## Dependency Updates
- Dependency updates are handled weekly by Dependabot.
- Please review Dependabot PRs like regular PRs and ensure checks pass.

## Code Style
- Keep changes simple and readable.
- Avoid unrelated refactors in the same PR.
- Follow existing project conventions for Vue, TypeScript, and Rust.
