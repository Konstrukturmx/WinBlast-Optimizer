$signtool = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64\signtool.exe"
$targetExe = "target\release\winblast0.exe"
$pfxFile = "scripts\Konstrukturmx_CodeSigning.pfx"
$password = ConvertTo-SecureString -String "***REMOVED***" -Force -AsPlainText

Write-Host "Creating Code Signing Certificate for Konstrukturmx..."
$cert = New-SelfSignedCertificate `
    -Type CodeSigningCert `
    -Subject "CN=Konstrukturmx, O=Konstrukturmx, OU=WinBlast Development, C=US" `
    -FriendlyName "Konstrukturmx WinBlast Code Signing" `
    -CertStoreLocation "Cert:\CurrentUser\My" `
    -NotAfter (Get-Date).AddYears(5)

Write-Host "Exporting Certificate to $pfxFile..."
Export-PfxCertificate -Cert $cert -FilePath $pfxFile -Password $password | Out-Null

Write-Host "Signing $targetExe with Microsoft SignTool..."
& $signtool sign /f $pfxFile /p "***REMOVED***" /fd SHA256 /d "WinBlast Optimizer v1.5.0" /du "https://github.com/Konstrukturmx/WinBlast-Optimizer" $targetExe

if ($LASTEXITCODE -eq 0) {
    Write-Host "Code signing succeeded! Attempting RFC 3161 timestamping..."
    & $signtool timestamp /tr "http://timestamp.digicert.com" /td SHA256 $targetExe
    if ($LASTEXITCODE -ne 0) {
        # Fallback timestamp server
        & $signtool timestamp /tr "http://timestamp.sectigo.com" /td SHA256 $targetExe
    }
}

Write-Host "Verifying signature on $targetExe..."
& $signtool verify /pa /v $targetExe

Write-Host "Done."
