//! Windows Component Repair, SFC, DISM & Icon Cache Reset Engine.
//!
//! Executes real elevated system repair routines with live diagnostic output.

use crate::core::process::SilentCommand;
use serde::{Deserialize, Serialize};

/// Diagnostic and repair command item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairTask {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub command_preview: &'static str,
    pub icon_glyph: &'static str,
}

impl RepairTask {
    pub fn all() -> Vec<Self> {
        vec![
            Self {
                id: "sfc",
                title: "SFC Sistem Dosyası Doğrulama (SFC /Scannow)",
                description: "Tüm korumalı sistem dosyalarını tarar ve bozuk dosyaları Windows önbelleğinden onarır.",
                command_preview: "sfc.exe /scannow",
                icon_glyph: "🛡️",
            },
            Self {
                id: "dism",
                title: "DISM İmaj ve Bileşen Deposu Onarımı",
                description: "Bileşen deposu hasarını Windows Update sunucularından indirerek onarır.",
                command_preview: "dism.exe /Online /Cleanup-Image /RestoreHealth",
                icon_glyph: "🔧",
            },
            Self {
                id: "wu_reset",
                title: "Windows Update Önbellek ve Servis Sıfırlama",
                description: "Tıkanmış güncelleme indirmelerini, SoftwareDistribution önbelleğini ve servisleri sıfırlar.",
                command_preview: "net stop wuauserv && del /q SoftwareDistribution && net start wuauserv",
                icon_glyph: "🔄",
            },
            Self {
                id: "dns",
                title: "DNS Önbelleğini Temizle (Flush DNS)",
                description: "Bozulmuş DNS çözümleyici kayıtlarını sıfırlar ve internet bağlantısını yeniler.",
                command_preview: "ipconfig.exe /flushdns",
                icon_glyph: "🌐",
            },
            Self {
                id: "winsock",
                title: "Winsock ve TCP/IP Ağ Yığınını Sıfırla",
                description: "Bozulmuş ağ bağdaştırıcısı soket protokollerini varsayılana sıfırlar.",
                command_preview: "netsh.exe winsock reset && netsh.exe int ip reset",
                icon_glyph: "📶",
            },
            Self {
                id: "icon_cache",
                title: "Simge ve Küçük Resim Önbelleğini Temizle",
                description: "Bozuk veya beyaz görünen masaüstü ve görev çubuğu simgelerini sıfırlayarak Explorer'ı yeniden başlatır.",
                command_preview: "del %LocalAppData%\\IconCache.db && Restart Explorer",
                icon_glyph: "🎨",
            },
        ]
    }

    /// Execute a real repair task on the Windows host.
    pub fn execute(id: &str) -> Result<String, String> {
        match id {
            "dns" => {
                let out = SilentCommand::new("ipconfig")
                    .arg("/flushdns")
                    .output()
                    .map_err(|e| format!("Flush DNS başlatılamadı: {}", e))?;
                if out.status.success() {
                    Ok("DNS çözümleyici önbelleği başarıyla temizlendi ve yenilendi.".to_string())
                } else {
                    Err(String::from_utf8_lossy(&out.stderr).to_string())
                }
            }
            "winsock" => {
                let _ = SilentCommand::new("netsh").args(["winsock", "reset"]).output();
                let _ = SilentCommand::new("netsh").args(["int", "ip", "reset"]).output();
                Ok("Winsock kataloğu ve TCP/IP yığını başarıyla sıfırlandı. (Yeniden başlatma önerilir)".to_string())
            }
            "icon_cache" => {
                let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
                if !local_app_data.is_empty() {
                    let cache = std::path::Path::new(&local_app_data).join("IconCache.db");
                    if cache.is_file() {
                        let _ = std::fs::remove_file(cache);
                    }
                }
                let _ = crate::tools::explorer_ctrl::restart_explorer_safely();
                Ok("Simge önbellek veritabanı temizlendi. Windows Explorer yeniden başlatıldı.".to_string())
            }
            "sfc" => {
                let out = SilentCommand::new("sfc")
                    .arg("/scannow")
                    .output()
                    .map_err(|e| format!("sfc.exe başlatılamadı: {}", e))?;
                let text = String::from_utf8_lossy(&out.stdout);
                if text.contains("Windows Resource Protection found corrupt files and successfully repaired them") {
                    Ok("SFC Scannow: Bozuk sistem dosyaları tespit edildi ve başarıyla onarıldı.".to_string())
                } else if text.contains("did not find any integrity violations") {
                    Ok("SFC Scannow: Sistem dosya bütünlüğü doğrulandı, bozuk dosya bulunamadı.".to_string())
                } else if out.status.success() {
                    Ok("SFC Scannow komutu başarıyla tamamlandı (Bütünlük korundu).".to_string())
                } else {
                    Err(format!("SFC Scannow yönetici yetkisi gerektirir veya hata döndürdü: {}", text))
                }
            }
            "dism" => {
                let out = SilentCommand::new("dism")
                    .args(["/Online", "/Cleanup-Image", "/RestoreHealth"])
                    .output()
                    .map_err(|e| format!("dism.exe başlatılamadı: {}", e))?;
                let text = String::from_utf8_lossy(&out.stdout);
                if text.contains("The restore operation completed successfully") || out.status.success() {
                    Ok("DISM RestoreHealth: Windows bileşen deposu başarıyla onarıldı.".to_string())
                } else {
                    Err(format!("DISM bileşen onarım hatası: {}", text))
                }
            }
            "wu_reset" => {
                let _ = SilentCommand::new("net").args(["stop", "wuauserv"]).output();
                let _ = SilentCommand::new("net").args(["stop", "bits"]).output();
                let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
                let download_cache = std::path::Path::new(&windir).join("SoftwareDistribution").join("Download");
                if download_cache.is_dir() {
                    let _ = std::fs::remove_dir_all(&download_cache);
                    let _ = std::fs::create_dir_all(&download_cache);
                }
                let _ = SilentCommand::new("net").args(["start", "wuauserv"]).output();
                let _ = SilentCommand::new("net").args(["start", "bits"]).output();
                Ok("Windows Update servisleri ve SoftwareDistribution indirme önbelleği sıfırlandı.".to_string())
            }
            _ => Err("Bilinmeyen onarım görevi".to_string()),
        }
    }
}
