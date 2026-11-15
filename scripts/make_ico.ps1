Add-Type -AssemblyName System.Drawing

$source = [System.Drawing.Bitmap]::FromFile((Resolve-Path "winblast.png"))
$sizes = @(256, 128, 64, 48, 32, 16)
$pngBytesList = @()

foreach ($sz in $sizes) {
    $bmp = New-Object System.Drawing.Bitmap $sz, $sz
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    $g.DrawImage($source, 0, 0, $sz, $sz)
    $g.Dispose()

    $ms = New-Object System.IO.MemoryStream
    $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
    $pngBytesList += ,($ms.ToArray())
    $ms.Dispose()
}
$source.Dispose()

$targetPath = Join-Path (Get-Location) "winblast.ico"
$fs = [System.IO.File]::Create($targetPath)
$bw = New-Object System.IO.BinaryWriter($fs)

# ICONDIR header: Reserved (0), Type (1 = ICO), Count
$bw.Write([uint16]0)
$bw.Write([uint16]1)
$bw.Write([uint16]$sizes.Count)

$offset = 6 + ($sizes.Count * 16)

for ($i = 0; $i -lt $sizes.Count; $i++) {
    $sz = $sizes[$i]
    $data = $pngBytesList[$i]

    $w = if ($sz -eq 256) { [byte]0 } else { [byte]$sz }
    $h = if ($sz -eq 256) { [byte]0 } else { [byte]$sz }
    $bw.Write($w)
    $bw.Write($h)
    $bw.Write([byte]0) # Color palette count
    $bw.Write([byte]0) # Reserved
    $bw.Write([uint16]1) # Color planes
    $bw.Write([uint16]32) # Bits per pixel
    $bw.Write([uint32]$data.Length) # Image size
    $bw.Write([uint32]$offset) # Image offset

    $offset += $data.Length
}

for ($i = 0; $i -lt $sizes.Count; $i++) {
    $data = $pngBytesList[$i]
    $bw.Write($data)
}

$bw.Close()
$fs.Close()

if (-not (Test-Path "assets")) { New-Item -ItemType Directory -Path "assets" | Out-Null }
Copy-Item "winblast.ico" "assets\winblast.ico" -Force
Write-Host "winblast.ico generated successfully with sizes: $($sizes -join ', ')"
