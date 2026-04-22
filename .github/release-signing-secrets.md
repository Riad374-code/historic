# Release Signing Secrets (Windows/macOS)

This project can publish unsigned builds out of the box. To avoid trust warnings in public distribution, configure code-signing secrets in GitHub.

## Optional Windows code signing

Add these repository secrets:

- WINDOWS_CERTIFICATE: Base64-encoded .pfx certificate content
- WINDOWS_CERTIFICATE_PASSWORD: Password for the .pfx file

## Optional macOS code signing and notarization

Add these repository secrets:

- APPLE_CERTIFICATE: Base64-encoded .p12 signing certificate
- APPLE_CERTIFICATE_PASSWORD: Password for the .p12 file
- APPLE_SIGNING_IDENTITY: Signing identity, for example Developer ID Application: Your Name (TEAMID)
- APPLE_ID: Apple ID email used for notarization
- APPLE_PASSWORD: App-specific password for the Apple ID
- APPLE_TEAM_ID: Apple Developer Team ID

## Optional Tauri updater signature

If you use Tauri updater metadata/signatures, add:

- TAURI_PRIVATE_KEY
- TAURI_KEY_PASSWORD

## Enable in workflow

The file .github/workflows/publish.yml already contains commented env placeholders for these secrets.

To enable signing:

1. Add the required secrets in GitHub repository settings.
2. Uncomment the corresponding env lines in .github/workflows/publish.yml.
3. Push a version tag like v0.1.1 to trigger a signed draft release.

## Important

- Never commit certificate/private key files to git.
- Rotate credentials if they are ever exposed.
- Use separate certificates for test and production when possible.
