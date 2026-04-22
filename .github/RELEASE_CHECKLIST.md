# Release Checklist

Use this checklist before publishing a new version.

## 1. Versioning

Ensure version is identical in:

- package.json
- src-tauri/tauri.conf.json
- src-tauri/Cargo.toml

## 2. Icons

Regenerate icons from the source image:

```bash
npm run tauri icon public/historic-favicon.svg
```

## 3. Local validation

Run a local production build:

```bash
npm run tauri build
```

## 4. Optional signing setup

If publishing to public users, set signing secrets as documented in:

- .github/release-signing-secrets.md

## 5. Publish trigger

Commit and push to main:

```bash
git add .
git commit -m "chore: prepare release workflow"
git push origin main
```

## 6. Review draft release

In GitHub Releases:

- Verify Windows, macOS, and Linux artifacts are present.
- Verify version and notes are correct.
- Publish the draft release.
