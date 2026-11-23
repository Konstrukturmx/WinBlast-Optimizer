<#
.SYNOPSIS
  Authenticode-sign the WinBlast0 release binary with the existing Konstrukturmx code-signing
  certificate from Cert:\CurrentUser\My, timestamped by DigiCert (RFC 3161).

.DESCRIPTION
  - Never creates, exports or regenerates certificates and never uses a password: the private key
    stays in the Windows certificate store and is selected by thumbprint.
  - Without -Thumbprint, the newest valid code-signing certificate whose subject is
    "CN=Konstrukturmx, ..." is used; the script fails if none (or the key) is present.
  - The certificate is self-signed, so Windows reports the chain as untrusted on machines that do
    not trust it. The script checks that the signature itself is intact and timestamped.

.EXAMPLE
  pwsh -File scripts/sign_binary.ps1
  pwsh -File scripts/sign_binary.ps1 -Thumbprint 070A70E017B5BB8F7A345688C82D8A578DAC404D -Target target\release\winblast0.exe
#>
param(
    [string]$Thumbprint,
    [string]$Target = "target\release\winblast0.exe",
    [string]$TimestampUrl = "http://timestamp.digicert.com"
)
$ErrorActionPreference = 'Stop'

function Find-SignTool {
    $cmd = Get-Command signtool.exe -ErrorAction SilentlyContinue
    if ($cmd) { return $cmd.Source }
    $kits = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits\10\bin'
    $found = Get-ChildItem $kits -Recurse -Filter signtool.exe -ErrorAction SilentlyContinue |
        Where-Object { $_.FullName -match '\\x64\\' } | Sort-Object FullName -Descending | Select-Object -First 1
    if (-not $found) { throw "signtool.exe not found (install the Windows 10/11 SDK)." }
    return $found.FullName
}

if (-not (Test-Path $Target)) { throw "Target not found: $Target (run 'cargo build --release' first)." }

$subjectPattern = '^CN=Konstrukturmx(,|$)'
if ($Thumbprint) {
    $cert = Get-Item "Cert:\CurrentUser\My\$Thumbprint" -ErrorAction SilentlyContinue
    if (-not $cert) { throw "Certificate $Thumbprint not found in Cert:\CurrentUser\My." }
} else {
    $cert = Get-ChildItem Cert:\CurrentUser\My -CodeSigningCert |
        Where-Object { $_.Subject -match $subjectPattern -and $_.NotAfter -gt (Get-Date) } |
        Sort-Object NotBefore -Descending | Select-Object -First 1
    if (-not $cert) { throw "No valid Konstrukturmx code-signing certificate in Cert:\CurrentUser\My." }
}
if (-not $cert.HasPrivateKey) { throw "Certificate $($cert.Thumbprint) has no private key." }

$signtool = Find-SignTool
Write-Host "Signing $Target with $($cert.Thumbprint) ($($cert.Subject), valid to $($cert.NotAfter.ToString('yyyy-MM-dd')))"
& $signtool sign /sha1 $cert.Thumbprint /s My /fd SHA256 /tr $TimestampUrl /td SHA256 `
    /d "WinBlast Optimizer" /du "https://github.com/Konstrukturmx/WinBlast-Optimizer" $Target
if ($LASTEXITCODE -ne 0) { throw "signtool sign failed ($LASTEXITCODE)." }

$sig = Get-AuthenticodeSignature $Target
if (-not $sig.SignerCertificate -or $sig.SignerCertificate.Thumbprint -ne $cert.Thumbprint) { throw "Signature not found or signed by an unexpected certificate." }
if (-not $sig.TimeStamperCertificate) { throw "Signature has no timestamp." }
Write-Host "Signer:      $($sig.SignerCertificate.Subject) [$($sig.SignerCertificate.Thumbprint)]"
Write-Host "Timestamp:   $($sig.TimeStamperCertificate.Subject)"
Write-Host "Status:      $($sig.Status) — $($sig.StatusMessage)"
if ($sig.Status -ne 'Valid') {
    Write-Host "Note: a self-signed certificate is reported as untrusted unless it is installed in Trusted Root / Trusted Publishers on the machine."
}
Write-Host "SHA-256:     $((Get-FileHash $Target -Algorithm SHA256).Hash)"
