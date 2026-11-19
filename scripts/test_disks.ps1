$d0 = Get-PhysicalDisk | Where-Object { $_.DeviceId -eq '0' } | Select-Object -First 1
$r0 = $d0 | Get-StorageReliabilityCounter -ErrorAction SilentlyContinue
Write-Host "Disk 0: $($d0.FriendlyName) Size: $($d0.Size) Wear: $($r0.Wear) Temp: $($r0.Temperature) TBW: $($r0.TotalBytesWritten) POH: $($r0.PowerOnHours)"

$d1 = Get-PhysicalDisk | Where-Object { $_.DeviceId -eq '1' } | Select-Object -First 1
$r1 = $d1 | Get-StorageReliabilityCounter -ErrorAction SilentlyContinue
Write-Host "Disk 1: $($d1.FriendlyName) Size: $($d1.Size) Wear: $($r1.Wear) Temp: $($r1.Temperature) TBW: $($r1.TotalBytesWritten) POH: $($r1.PowerOnHours)"
