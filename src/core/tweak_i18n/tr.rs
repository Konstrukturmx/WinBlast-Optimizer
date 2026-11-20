use super::LocalizedTweakText;

pub fn lookup(id: &str) -> Option<LocalizedTweakText> {
    match id {
        // === ai_privacy_ext.toml (21) ===
        "disable_windows_ai_recall_extended" => Some(LocalizedTweakText {
            title: "Windows Recall, Click to Do ve Copilot+ Yapay Zeka Ajanlarını Devre Dışı Bırak",
            description: "Windows 11 24H2+ ile gelen Grup İlkesi anahtarları üzerinden Windows Recall ekran görüntüsü yakalamayı, Click to Do özelliğini, Ayarlar Yapay Zeka Ajanını, Yapay Zeka Ajan Bağlayıcılarını/Çalışma Alanlarını ve Copilot Runtime bileşenini engeller.",
            broken_features: &["Windows Recall zaman çizelgesi anlık görüntüleri ve Click to Do yapay zeka katmanı"],
        }),
        "disable_generative_ai_app_access" => Some(LocalizedTweakText {
            title: "Uygulamaların Üretken ve Sistem Yapay Zeka Modellerine Erişimini Engelle",
            description: "AppPrivacy ve CapabilityAccessManager ConsentStore ilkeleri aracılığıyla arka plandaki UWP/WinUI uygulamalarının yerel üretken yapay zeka ve sistem yapay zeka modeli yeteneklerine erişimini reddeder.",
            broken_features: &["Üçüncü taraf uygulamalar yerleşik Windows yapay zeka model uç noktalarını sorgulayamaz"],
        }),
        "disable_copilot_shell_and_hardware_key" => Some(LocalizedTweakText {
            title: "Copilot Görev Çubuğu Düğmesini, İpuçlarını, Donanım Tuşunu ve Mikrofonunu Kapat",
            description: "Windows Copilot görev çubuğu düğmesini ve yönlendirmelerini gizler, özel Copilot klavye tuşu atamasını etkisizleştirir, hesap durumuna bağlı yapay zeka tanıtımlarını engeller ve Copilot ile OfficeHub için mikrofon erişimini reddeder.",
            broken_features: &[
                "Özel Copilot tuşu Copilot'u başlatmaz",
                "Copilot sesli girişi engellendi",
            ],
        }),
        "disable_paint_ai_features" => Some(LocalizedTweakText {
            title: "Microsoft Paint Bulut ve Üretken Yapay Zeka Özelliklerini Devre Dışı Bırak",
            description: "Resmi İlke anahtarları üzerinden Windows 11 Paint içindeki Image Creator, Cocreator, Üretken Doldurma, Üretken Silme ve Yapay Zeka Arka Plan Kaldırma araçlarını devre dışı bırakır.",
            broken_features: &["Microsoft Paint içindeki yapay zeka oluşturma araçları gizlenir"],
        }),
        "disable_edge_ai_policies" => Some(LocalizedTweakText {
            title: "Microsoft Edge Copilot, Kenar Çubuğu ve Yerleşik Yapay Zeka API'lerini Kapat",
            description: "Copilot Sayfa Bağlamı, Merkezler Kenar Çubuğu, Entra Copilot, Satır İçi Yazma, Yapay Zeka Geçmiş Araması, Yerel Temel Modeller, DevTools GenAI ve Geçmiş Paylaşımı dahil 12 Microsoft Edge yapay zeka ilkesini devre dışı bırakır.",
            broken_features: &["Edge Copilot kenar çubuğu ve satır içi yazma yardımı devre dışı bırakılır"],
        }),
        "disable_office_ai_copilot" => Some(LocalizedTweakText {
            title: "Microsoft Office Yapay Zeka Eğitimini ve Uygulama İçi Copilot'ları Devre Dışı Bırak",
            description: "Microsoft 365 yapay zeka eğitimini, bağlı içerik analizini, İçerik Güvenliği Hizmetini ve Word, Excel ile OneNote içindeki uygulama bazlı Copilot anahtarlarını devre dışı bırakır.",
            broken_features: &["Word, Excel ve OneNote içindeki Copilot yardımcısı devre dışı bırakılır"],
        }),
        "disable_wmi_telemetry_autologgers" => Some(LocalizedTweakText {
            title: "10 WMI Çekirdek Telemetri AutoLogger Oturumunu Devre Dışı Bırak",
            description: "Windows'un kalıcı ETW arka plan AutoLogger izleme oturumları (Diagtrack-Listener, SQMLogger, DiagLog, Cellcore, DataMarket, AppModel, CloudExperienceHostOobe, LwtNetLog, WdiContextLog, WiFiSession) başlatmasını engeller.",
            broken_features: &["Çekirdek ETW tanılama izleme günlükleri devre dışı bırakılır"],
        }),
        "block_silent_preinstalled_apps" => Some(LocalizedTweakText {
            title: "Sessiz Ön Yüklü Uygulamaları, Başlat ve Ayarlar Önerilerini Engelle",
            description: "Windows ContentDeliveryManager bileşeninin OEM veya Microsoft Store tanıtım uygulamalarını sessizce kurmasını engeller; Ayarlar, Başlat menüsü ve Zaman Çizelgesindeki önerilen içerikleri gizler.",
            broken_features: &[],
        }),
        "disable_extra_telemetry_policies" => Some(LocalizedTweakText {
            title: "Geri Bildirim, AppCompat Envanteri, OOBE Uyarıları ve İzlemeyi Sıkılaştır",
            description: "MaxTelemetryAllowed=0 sınırını uygular, Windows geri bildirim (SIUF) istemlerini susturur (PeriodInNanoSeconds değerini siler), Uygulama Uyumluluğu Envanteri/UAR, Cihazımı Bul, Hoş Geldiniz/Kurulumu Tamamla uyarıları, uygulama başlatma izleme, İş Yerine Katılma istemleri ve Uzaktan Yardımı devre dışı bırakır.",
            broken_features: &[
                "Uzaktan Yardım davetleri devre dışı bırakılır",
                "Başlat menüsü Sık/Son Kullanılan uygulama başlatma takibi kapatılır",
            ],
        }),
        "disable_typing_and_inking_telemetry" => Some(LocalizedTweakText {
            title: "Mürekkep Oluşturma, Yazma Kişiselleştirme ve Giriş İçgörüleri Telemetrisini Kapat",
            description: "Örtülü tuş vuruşu/mürekkep verisi toplamayı, özel yazma sözlükleri için kişi bilgisi derlemeyi, dilbilimsel tanılama günlüğünü (TIPC) ve Windows Giriş İçgörülerini kısıtlar.",
            broken_features: &["Kişiselleştirilmiş yazma ve el yazısı tahmin sözlüğü kapatılır"],
        }),
        "disable_search_cloud_and_history" => Some(LocalizedTweakText {
            title: "Windows Arama Öne Çıkanlarını, Bulut Aramasını ve Cihaz Geçmişini Devre Dışı Bırak",
            description: "Dizin oluşturma hizmetini kapatmadan Windows Arama Öne Çıkanlarını (Dinamik Arama Kutusu), MSA/AAD bulut içerik aramasını ve yerel cihaz arama geçmişi günlüğünü kapatır.",
            broken_features: &["Arama öne çıkanları günlük görseli ve OneDrive bulut sonuçları gizlenir"],
        }),
        "disable_wifi_sense_and_location_extras" => Some(LocalizedTweakText {
            title: "WiFi-Sense Otomatik Bağlanmayı, Haritalar Güncellemesini ve Konum İznini Kapat",
            description: "Wi-Fi etkin noktalarına otomatik bağlanmayı (WcmSvc\\AutoConnectAllowedOEM=0), Windows Haritalar çevrimdışı otomatik güncellemelerini ve genel Konum Özelliği ConsentStore iznini devre dışı bırakır.",
            broken_features: &["Otomatik Windows Haritalar indirmeleri ve sistem geneli uygulama konum istemleri"],
        }),
        "block_onedrive_kfm_auto_backup" => Some(LocalizedTweakText {
            title: "OneDrive Otomatik Bilinen Klasör Taşımayı (Masaüstü/Belgeler Eşitleme) Engelle",
            description: "KFMBlockOptIn=1 ayarlayarak OneDrive'ın Masaüstü, Belgeler ve Resimler klasörlerini sessizce bulut yedeklemeye almasını engeller. Yalnızca yerel depolama veya elle eşitleme isteyenler için güvenlidir.",
            broken_features: &["OneDrive otomatik Bilinen Klasör Taşıma sihirbazı devre dışı bırakılır"],
        }),
        "disable_bitlocker_auto_encryption" => Some(LocalizedTweakText {
            title: "Otomatik Windows BitLocker Cihaz Şifrelemesini Devre Dışı Bırak",
            description: "PreventDeviceEncryption=1 ayarlayarak Microsoft Hesabı ile oturum açmada veya yeni kurulumlarda kurtarma anahtarları yedeklenmeden sürücülerin sessizce şifrelenmesini engeller.",
            broken_features: &["Otomatik sıfır dokunuşlu BitLocker sürücü şifrelemesi etkinleşmez"],
        }),
        "disable_extra_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Ek Telemetri, WER Kuyruğu, SQM, Güç Tanılama ve Harita Görevlerini Devre Dışı Bırak",
            description: "Ek tanılama ve arka plan zamanlanmış görevlerini devre dışı bırakır: QueueReporting, Sqm-Tasks, MareBackup, StartupAppTask, PcaPatchDbTask, MapsUpdateTask, AnalyzeSystem ve FamilySafetyMonitor.",
            broken_features: &["Otomatik güç verimliliği raporu ve çevrimdışı harita güncellemeleri devre dışı bırakılır"],
        }),
        "disable_windows_ai_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Windows AI Recall Arka Plan Zamanlanmış Görevlerini Devre Dışı Bırak",
            description: "Windows 11 24H2+ ile gelen Windows Recall yapılandırma ve makine öğrenimi arka plan iş hattı zamanlanmış görevlerini devre dışı bırakır.",
            broken_features: &["Windows Recall arka plan dizinleme görevleri devre dışı bırakılır"],
        }),
        "disable_office_actions_scheduled_task" => Some(LocalizedTweakText {
            title: "Microsoft Office Yapay Zeka Eylemleri Sunucu Görevini Devre Dışı Bırak",
            description: "Office yapay zeka arka plan CPU/bellek uyanmalarını azaltmak için Microsoft Office Actions Server zamanlanmış görevini devre dışı bırakır.",
            broken_features: &[],
        }),
        "disable_optional_bloat_services" => Some(LocalizedTweakText {
            title: "Kullanılmayan İsteğe Bağlı Windows Hizmetlerini Kapat (Haritalar, Faks, Demo, Cüzdan, tzautoupdate)",
            description: "Çekirdek işletim sistemine veya Windows Update hizmetlerine dokunmadan MapsBroker, RetailDemo, Fax, WalletService, WMPNetworkSvc, AJRouter, shpamsvc ve tzautoupdate hizmetlerini devre dışı bırakır.",
            broken_features: &["Windows Faks, Çevrimdışı Harita Aracısı, Mağaza Demosu, WMP Paylaşımı ve Otomatik Saat Dilimi devre dışı kalır"],
        }),
        "disable_xbox_live_services" => Some(LocalizedTweakText {
            title: "Xbox Live Arka Plan Hizmetlerini Devre Dışı Bırak (XblAuthManager, GameSave, Networking)",
            description: "Xbox Uygulaması / Game Pass oyunlarını oynamayan kullanıcılar için Xbox Live Kimlik Doğrulama Yöneticisi, Oyun Kaydı ve Ağ İletişimi arka plan hizmetlerini devre dışı bırakır.",
            broken_features: &["Xbox Game Pass ve Microsoft Store Xbox Live oturum açma / bulut kayıtları çalışmaz"],
        }),
        "disable_ai_fabric_service" => Some(LocalizedTweakText {
            title: "Windows AI Fabric Hizmetini Devre Dışı Bırak (WSAIFabricSvc)",
            description: "Windows 11 24H2+ üzerindeki WSAIFabricSvc arka plan yerel yapay zeka düzenleme servisini devre dışı bırakır.",
            broken_features: &["Copilot+ yapay zeka düzenleme hizmeti durdurulur"],
        }),
        "deny_global_camera_mic_diagnostics" => Some(LocalizedTweakText {
            title: "Kamera, Mikrofon, Hesap Bilgisi ve Tanılamaya Genel Uygulama Erişimini Engelle [İsteğe Bağlı]",
            description: "Tüm masaüstü ve UWP uygulamalarında web kamerası, mikrofon, userAccountInformation ve appDiagnostics için ConsentStore izinlerini genel olarak reddeder. Yüksek etki riski: geri alınana kadar Zoom/Discord/OBS kamera ve mikrofonlarını devre dışı bırakır.",
            broken_features: &["Yeniden izin verilene kadar TÜM uygulamalarda Kamera ve Mikrofonu devre dışı bırakır"],
        }),

        // === bloatware_appx.toml (26) ===
        "remove_bing_weather" => Some(LocalizedTweakText {
            title: "Bing Hava Durumu Uygulamasını Kaldır",
            description: "Microsoft Bing Hava Durumu UWP uygulamasını kaldırır. Hava durumuna tarayıcı üzerinden erişilebilir.",
            broken_features: &["Bing Hava Durumu uygulaması kaldırılacak"],
        }),
        "remove_bing_news" => Some(LocalizedTweakText {
            title: "Bing Haberler Uygulamasını Kaldır",
            description: "Microsoft Bing Haberler (Microsoft News) UWP uygulamasını kaldırır.",
            broken_features: &["Bing Haberler / Microsoft News uygulaması kaldırılacak"],
        }),
        "remove_get_help" => Some(LocalizedTweakText {
            title: "Yardım Al Uygulamasını Kaldır",
            description: "Microsoft Yardım Al (Get Help) UWP uygulamasını kaldırır. Kullanıcılar desteğe web üzerinden erişmeye devam edebilir.",
            broken_features: &["Yardım Al uygulaması kaldırılacak"],
        }),
        "remove_getstarted_tips" => Some(LocalizedTweakText {
            title: "İpuçları (Başlarken) Uygulamasını Kaldır",
            description: "Microsoft İpuçları / Başlarken UWP uygulamasını kaldırır.",
            broken_features: &["İpuçları / Başlarken uygulaması kaldırılacak"],
        }),
        "remove_solitaire" => Some(LocalizedTweakText {
            title: "Microsoft Solitaire Collection Uygulamasını Kaldır",
            description: "Reklam ve telemetri içeren Microsoft Solitaire Collection UWP uygulamasını kaldırır.",
            broken_features: &["Solitaire Collection kaldırılacak"],
        }),
        "remove_people" => Some(LocalizedTweakText {
            title: "Kişiler Uygulamasını Kaldır",
            description: "Microsoft Kişiler (People) UWP uygulamasını kaldırır.",
            broken_features: &["Kişiler uygulaması kaldırılacak"],
        }),
        "remove_windows_maps" => Some(LocalizedTweakText {
            title: "Windows Haritalar Uygulamasını Kaldır",
            description: "Microsoft Windows Haritalar UWP uygulamasını kaldırır.",
            broken_features: &["Windows Haritalar uygulaması kaldırılacak"],
        }),
        "remove_groove_music" => Some(LocalizedTweakText {
            title: "Groove Müzik (Zune Music) Uygulamasını Kaldır",
            description: "Groove Müzik (ZuneMusic) UWP uygulamasını kaldırır. Bunun yerine Medya Oynatıcı veya üçüncü taraf oynatıcılar kullanılabilir.",
            broken_features: &["Groove Müzik uygulaması kaldırılacak"],
        }),
        "remove_zune_video" => Some(LocalizedTweakText {
            title: "Filmler ve TV (Zune Video) Uygulamasını Kaldır",
            description: "Filmler ve TV (ZuneVideo) UWP uygulamasını kaldırır. Bunun yerine VLC veya diğer oynatıcılar kullanılabilir.",
            broken_features: &["Filmler ve TV uygulaması kaldırılacak"],
        }),
        "remove_clipchamp" => Some(LocalizedTweakText {
            title: "Clipchamp Uygulamasını Kaldır",
            description: "Windows 11 üzerinde ön yüklü gelen Clipchamp video düzenleyici UWP uygulamasını kaldırır.",
            broken_features: &["Clipchamp video düzenleyici kaldırılacak"],
        }),
        "remove_todos" => Some(LocalizedTweakText {
            title: "Microsoft To Do Uygulamasını Kaldır",
            description: "Microsoft To Do UWP uygulamasını kaldırır.",
            broken_features: &["Microsoft To Do uygulaması kaldırılacak"],
        }),
        "remove_bing_search_appx" => Some(LocalizedTweakText {
            title: "Microsoft Bing Arama AppX Uzantısını Kaldır",
            description: "Web araç takımlarını ve Windows Arama web tümleştirmelerini destekleyen Microsoft.BingSearch Store paketini kaldırır.",
            broken_features: &["Bing Arama AppX tümleştirmesi kaldırılır"],
        }),
        "remove_feedback_hub" => Some(LocalizedTweakText {
            title: "Microsoft Geri Bildirim Merkezi Uygulamasını Kaldır",
            description: "Ön yüklü Windows Geri Bildirim Merkezi telemetri/anket uygulamasını kaldırır.",
            broken_features: &["Geri Bildirim Merkezi uygulaması kaldırılır"],
        }),
        "remove_copilot_appx" => Some(LocalizedTweakText {
            title: "Microsoft Copilot UWP Paketini Kaldır",
            description: "Bağımsız Microsoft.Copilot Store paketinin kurulumunu kaldırır.",
            broken_features: &["Copilot uygulaması kaldırılır"],
        }),
        "remove_edge_game_assist" => Some(LocalizedTweakText {
            title: "Microsoft Edge Oyun Yardımcısı Katmanını Kaldır",
            description: "Edge Oyun Yardımcısı (Game Assist) oyun içi tarayıcı katmanı AppX paketini kaldırır.",
            broken_features: &["Edge Oyun Yardımcısı araç takımı kaldırılır"],
        }),
        "remove_dev_home" => Some(LocalizedTweakText {
            title: "Windows Dev Home Uygulamasını Kaldır",
            description: "Windows 11 üzerinde ön yüklü gelen Microsoft.Windows.DevHome pano paketini kaldırır.",
            broken_features: &["Dev Home panosu kaldırılır"],
        }),
        "remove_power_automate_desktop" => Some(LocalizedTweakText {
            title: "Microsoft Power Automate Desktop Uygulamasını Kaldır",
            description: "Microsoft Power Automate Desktop RPA paketini kaldırır.",
            broken_features: &["Power Automate masaüstü akışları uygulaması kaldırılır"],
        }),
        "remove_office_hub" => Some(LocalizedTweakText {
            title: "Microsoft 365 Copilot (OfficeHub) Uygulamasını Kaldır",
            description: "Ön yüklü Microsoft 365 Office Hub tanıtım başlatıcı uygulamasını kaldırır.",
            broken_features: &["Office tanıtım başlatıcısı kaldırılır (masaüstü Office'i kaldırmaz)"],
        }),
        "remove_family_safety" => Some(LocalizedTweakText {
            title: "Microsoft Aile Koruması Uygulamasını Kaldır",
            description: "Microsoft Family Safety tüketici UWP uygulamasını kaldırır.",
            broken_features: &["Aile Koruması UWP uygulaması kaldırılır"],
        }),
        "remove_3d_viewer_and_mixed_reality" => Some(LocalizedTweakText {
            title: "3D Görüntüleyici ve Karma Gerçeklik Portalını Kaldır",
            description: "Eski Microsoft 3D Viewer ve Mixed Reality Portal paketlerini kaldırır.",
            broken_features: &["Eski 3D Görüntüleyici ve Windows Karma Gerçeklik Portalı kaldırılır"],
        }),
        "remove_outlook_for_windows" => Some(LocalizedTweakText {
            title: "Windows için Yeni Outlook'u Kaldır (WebView İstemcisi)",
            description: "Ön yüklü Microsoft.OutlookForWindows web sarmalayıcı uygulamasını kaldırır.",
            broken_features: &["Yeni Outlook UWP istemcisi kaldırılır"],
        }),
        "remove_your_phone_link" => Some(LocalizedTweakText {
            title: "Telefon Bağlantısı (YourPhone) Uygulamasını Kaldır",
            description: "Microsoft.YourPhone mobil eşitleme yardımcı uygulamasını kaldırır.",
            broken_features: &["Android/iOS Telefon Bağlantısı masaüstü eşitlemesi kaldırılır"],
        }),
        "remove_sound_recorder_and_sticky_notes" => Some(LocalizedTweakText {
            title: "Windows Ses Kaydedicisi ve Yapışkan Notları Kaldır",
            description: "Ses Kaydedicisi ve Yapışkan Notlar Store paketlerini kaldırır.",
            broken_features: &["Yerleşik Ses Kaydedicisi ve Yapışkan Notlar uygulamaları kaldırılır"],
        }),
        "remove_ms_teams_and_quick_assist" => Some(LocalizedTweakText {
            title: "Bireysel Microsoft Teams ve Hızlı Yardım Uygulamalarını Kaldır",
            description: "Bireysel MSTeams kişisel sohbet ve Hızlı Yardım destek uygulamalarını kaldırır.",
            broken_features: &["Kişisel Teams ve Hızlı Yardım destek aracı kaldırılır"],
        }),
        "disable_recall_optional_feature" => Some(LocalizedTweakText {
            title: "Windows Recall İsteğe Bağlı Özelliğini Kaldır (DISM)",
            description: "İşletim sistemi bakım düzeyinde isteğe bağlı 'Recall' Windows özellik paketini devre dışı bırakır.",
            broken_features: &["Windows Recall özellik ikili dosyaları temiz şekilde devre dışı bırakılır"],
        }),
        "remove_legacy_steps_recorder_ie_wordpad" => Some(LocalizedTweakText {
            title: "Eski Adım Kaydedicisi ve Internet Explorer 11 Özelliğini Devre Dışı Bırak",
            description: "DISM aracılığıyla eski Adım Kaydedicisi (App.StepsRecorder) ve Internet Explorer modu özelliğini kaldırır.",
            broken_features: &["Adım Kaydedicisi (psr.exe) ve IE11 Modu kaldırılır"],
        }),

        // === hardware_tuning.toml (16) ===
        "enable_hags" => Some(LocalizedTweakText {
            title: "Donanım Hızlandırmalı GPU Zamanlamasını (HAGS) Etkinleştir",
            description: "GPU'nun kendi VRAM zamanlamasını doğrudan yönetmesini sağlayan Donanım Hızlandırmalı GPU Zamanlamasını etkinleştirir. GPU odaklı iş yüklerinde gecikmeyi ve CPU yükünü azaltır. Uyumlu GPU (NVIDIA GTX 1000+, AMD RX 5000+) ve sürücü desteği gerektirir.",
            broken_features: &["Bazı eski oyunlar veya GPU araçları kararsızlık gösterebilir"],
        }),
        "disable_mpo" => Some(LocalizedTweakText {
            title: "Çok Düzlemli Katmanlamayı (MPO) Devre Dışı Bırak",
            description: "Multi-Plane Overlay donanım kompozitör özelliğini devre dışı bırakır. MPO, özellikle farklı yenileme hızlarına sahip çoklu monitör kurulumlarında pek çok GPU/sürücü kombinasyonunda takılma, siyah ekran ve kare zamanlama sorunlarına yol açmasıyla bilinir.",
            broken_features: &["Dizüstü bilgisayarlarda GPU kompozitör güç tüketimi hafifçe artabilir"],
        }),
        "disable_core_parking" => Some(LocalizedTweakText {
            title: "CPU Çekirdek Park Etmeyi Devre Dışı Bırak",
            description: "Park edilmeden kalması gereken minimum işlemci çekirdeği yüzdesini %100 olarak ayarlayarak tüm çekirdeklerin iş yükleri için her zaman hazır olmasını sağlar. Daha yüksek boşta güç tüketimi karşılığında çekirdek uyanma gecikmesini ortadan kaldırır.",
            broken_features: &[
                "Boşta güç tüketimi artar",
                "Dizüstü bilgisayarlarda pil ömrü kısalır",
            ],
        }),
        "disable_cstates" => Some(LocalizedTweakText {
            title: "CPU Boşta C-State Durumlarını Devre Dışı Bırak",
            description: "Boşta durum ilkesini performans moduna ayarlayarak işlemcinin derin boşta C-state (C6/C7/C8) durumlarına girmesini engeller. Boşta güç çekimi pahasına CPU güç durumları arasında geçiş yaparken oluşan gecikme sıçramalarını azaltır.",
            broken_features: &[
                "Boşta güç tüketimi belirgin şekilde artar",
                "Dizüstü bilgisayarlarda pil ömrü kısalır",
                "Boşta CPU sıcaklıkları yükselir",
            ],
        }),
        "gpu_shader_cache_size" => Some(LocalizedTweakText {
            title: "GPU TDR Zaman Aşımını Uzat ve Sürücü Sıfırlamalarını Önle",
            description: "WDDM Zaman Aşımı Algılama ve Kurtarma (TdrDelay) süresini 2 saniyeden 10 saniyeye çıkararak yoğun DirectX gölgelendirici derlemesi sırasında hatalı GPU sürücüsü sıfırlamalarını ve takılmaları önler.",
            broken_features: &["Gerçek bir GPU kilitlenmesi yaşanırsa ekran sürücüsü kurtarması 10 saniyeye kadar bekler"],
        }),
        "enable_game_mode" => Some(LocalizedTweakText {
            title: "Windows Oyun Modunu Etkinleştir",
            description: "Windows Oyun Modunun etkin olmasını (AutoGameModeEnabled=1, AllowAutoGameMode=1) garanti eder; önceden kapatılmış veya kaldırılmışsa oyun süreçlerine öncelik verir ve oyun sırasında Windows Update ile yeniden başlatma bildirimlerini bastırır.",
            broken_features: &[],
        }),
        "disable_fullscreen_optimizations" => Some(LocalizedTweakText {
            title: "Tam Ekran İyileştirmelerini Genel Olarak Devre Dışı Bırak",
            description: "Tüm uygulamalar için Windows tam ekran iyileştirmelerini (kenarlıksız tam ekran hibriti) genel olarak kapatır. Bu ayar, eski oyunlarda giriş gecikmesini ve kare süresi sapmasını azaltabilen gerçek özel tam ekran modunu zorunlu kılar.",
            broken_features: &[
                "Tam ekran oyunlardan Alt-Tab geçişi daha yavaş olur",
                "Özel tam ekran modunda Game Bar katmanı çalışmaz",
            ],
        }),
        "4433ce93-b632-4d67-ae71-0241c31099c0" => Some(LocalizedTweakText {
            title: "AMD ULPS Özelliğini Devre Dışı Bırak",
            description: "AMD Radeon sürücü yığınında Ultra Düşük Güç Durumunu (ULPS) devre dışı bırakır. İkincil GPU çekirdeklerinin veya uyku durumlarının 3D oyun sırasında mikro takılmalar ve kare gecikmesi sıçramalarıyla uyanmasını önler.",
            broken_features: &["AMD çoklu GPU / Crossfire dizüstü bilgisayarlarda boşta güç tüketimi hafifçe artar"],
        }),
        "8a4b4c4d-000e-42db-8bd7-293067f8fae7" => Some(LocalizedTweakText {
            title: "AMD ASPM Özelliğini Devre Dışı Bırak",
            description: "AMD ekran kartları için PCIe Etkin Durum Güç Yönetimini (ASPM) devre dışı bırakır. Kare işleme boşluklarında PCIe veri yolu bağlantısının düşük güçlü L0s/L1 durumlarına geçmesini önleyerek PCIe iletim titremesini azaltır.",
            broken_features: &["PCIe veri yolu boşta güç tüketimi artar"],
        }),
        "a3836587-c461-4beb-9c4c-88493b36c138" => Some(LocalizedTweakText {
            title: "AMD Power Gating Özelliğini Devre Dışı Bırak",
            description: "AMD hesaplama birimleri ve işleme arka uçlarında agresif dinamik güç kesmeyi (DisablePowerGating=1, PP_GPUPowerDownEnabled=0) devre dışı bırakır. Saat frekanslarını kısmadan sürekli korur.",
            broken_features: &["Düşük kullanımda GPU yonga sıcaklıkları yükselir"],
        }),
        "db379bb5-fc8c-4ad3-be32-11be09e76fba" => Some(LocalizedTweakText {
            title: "AMD Video Clock Gating Özelliğini Devre Dışı Bırak",
            description: "AMD VCE/UVD multimedya ve donanım görüntü motorlarında saat ve güç kesmeyi (DisableVCEPowerGating=1, DisableVceClockGating=1, EnableUvdClockGating=0, EnableVceSwClockGating=0) engeller. DPC gecikme sıçramalarını ortadan kaldırır.",
            broken_features: &[],
        }),
        "df8739ff-e322-4958-b369-df584bb9b0c1" => Some(LocalizedTweakText {
            title: "Intel Async Flips Özelliğini Devre Dışı Bırak",
            description: "Eşzamanlı görüntü motoru kare çevirmelerini zorlamak ve sunum kuyruğu titremesini ortadan kaldırmak için Intel Graphics görüntü bağdaştırıcısı sınıf anahtarlarında Display1_DisableAsyncFlips=1 ayarlar.",
            broken_features: &[],
        }),
        "ebfb0ded-5d1a-4cd1-b5c0-4f7018dd6054" => Some(LocalizedTweakText {
            title: "Intel Uyarlamalı V-Sync Özelliğini Devre Dışı Bırak",
            description: "Kare hızı monitör yenileme hızının altına düştüğünde uyarlamalı yenileme kısıtlamasını önlemek için Intel Graphics görüntü bağdaştırıcısı sınıf anahtarlarında AdaptiveVsyncEnable=0 ayarlar.",
            broken_features: &[],
        }),
        "fa58802f-b98c-4e98-aa67-067c5a394296" => Some(LocalizedTweakText {
            title: "NVIDIA Dinamik P-State Durumunu Devre Dışı Bırak",
            description: "Saat hızı dalgalanmalarını önlemek amacıyla NVIDIA GPU'yu sabit performans durumuna kilitlemek için DisableDynamicPstate=1 ayarlar. Boşta güç çekimini artırır.",
            broken_features: &["Boşta GPU güç çekimini ve ısıyı artırır"],
        }),
        "9e7702be-9e03-4c96-a316-c4e435cb993e" => Some(LocalizedTweakText {
            title: "NVIDIA Async P-State Geçişlerini Devre Dışı Bırak",
            description: "NVIDIA eşzamansız güç durumu geçişlerini önlemek için DisableAsyncPstates=1 ayarlar ve VR/gerçek zamanlı uygulamalar için öngörülebilir performans sağlar.",
            broken_features: &["Video oynatma sırasında GPU bellek saat hızının düşmesini engelleyebilir"],
        }),
        "5f1a2b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c" => Some(LocalizedTweakText {
            title: "NVIDIA Maksimum Performans PowerMizer Modunu Etkinleştir",
            description: "Priz gücünde PowerMizer maksimum performans modunu zorunlu kılar; PCIe veri yolunu ve bellek denetleyicilerini kısmadan tam hıza kilitler.",
            broken_features: &["Daha yüksek GPU sıcaklıkları ve boşta fan hızı"],
        }),

        // === os_settings.toml (21) ===
        "disable_animations" => Some(LocalizedTweakText {
            title: "Pencere Animasyonlarını Devre Dışı Bırak",
            description: "Pencereleri simge durumuna küçültürken ve ekranı kaplarken oynatılan görsel animasyon efektini kapatarak hissedilen arayüz gecikmesini azaltır.",
            broken_features: &[],
        }),
        "disable_transparency" => Some(LocalizedTweakText {
            title: "Saydamlık Efektlerini Devre Dışı Bırak",
            description: "Görev çubuğu, Başlat menüsü ve İşlem Merkezindeki akrilik/saydamlık efektini devre dışı bırakarak GPU birleştirme yükünü azaltır.",
            broken_features: &["Görev çubuğu ve Başlat menüsü saydam görünümü"],
        }),
        "disable_ntfs_last_access" => Some(LocalizedTweakText {
            title: "Windows Arka Plan Uygulamalarını Genel Olarak Devre Dışı Bırak",
            description: "UWP / Microsoft Store uygulamalarının simge durumuna küçültüldüğünde veya kapatıldığında uyanmasını, ağ yoklaması yapmasını ve arka planda sessizce CPU/RAM tüketmesini engeller.",
            broken_features: &["Store uygulamaları kapatıldığında arka planda eşitlemeyi durdurur"],
        }),
        "disable_nagle_algorithm" => Some(LocalizedTweakText {
            title: "MSMQ Nagle Algoritmasını Devre Dışı Bırak (TCPNoDelay)",
            description: "Özellikle Microsoft Message Queuing (MSMQ\\Parameters\\TCPNoDelay) için Nagle algoritmasını kapatarak küçük mesajlı MSMQ iş yüklerinde kuyruk gecikmesini azaltır.",
            broken_features: &["Küçük paketler için ağ bant genişliği kullanımını hafifçe artırabilir"],
        }),
        "disable_sticky_keys" => Some(LocalizedTweakText {
            title: "Yapışkan Tuşları ve Erişilebilirlik Kısayollarını Devre Dışı Bırak",
            description: "5 kez Shift basıldığında çıkan Yapışkan Tuşlar penceresini, Filtre Tuşlarını ve Geçiş Tuşları uyarı seslerini kapatarak rekabetçi oyunlarda istenmeyen açılır pencereleri ve giriş kesintilerini önler.",
            broken_features: &["Erişilebilirlik kısayolları (5x Shift, 8sn Sağ Shift) devre dışı bırakılır"],
        }),
        "disable_mouse_shadow" => Some(LocalizedTweakText {
            title: "Gezgin Alt Gölgelerini ve Seçim Alfa Saydamlığını Kapat",
            description: "Masaüstü simge etiketi alt gölgelerini ve Dosya Gezgini sürükleme seçim dikdörtgenindeki yarı saydam alfa harmanlamasını kapatarak DWM GPU birleştirme işini azaltır.",
            broken_features: &[],
        }),
        "high_performance_power_plan" => Some(LocalizedTweakText {
            title: "Agresif İşlemci Performans Artırma (Boost) Modunu Ayarla",
            description: "PowerSchemes PerfBoostMode üzerinden priz gücünde İşlemci Performans Artırma Modunu Agresif olarak ayarlar; etkileşimli yüklerde turbo frekans kısmalarını önler.",
            broken_features: &[
                "Boşta güç tüketimi artar",
                "Dizüstü bilgisayarlarda ısı üretimi artar",
            ],
        }),
        "disable_bing_search_start" => Some(LocalizedTweakText {
            title: "Başlat Menüsünde Bing Web Aramasını Devre Dışı Bırak",
            description: "Windows Aramanın her tuş vuruşunu Bing sunucularına göndermesini engeller; Başlat menüsü uygulama ve yerel dosya aramasını bulut ağ gecikmesi olmadan anlık hale getirir.",
            broken_features: &["Başlat menüsünde web arama sonuçları görünmez"],
        }),
        "disable_windows_copilot" => Some(LocalizedTweakText {
            title: "Windows Copilot Yapay Zeka Tümleştirmesini Devre Dışı Bırak",
            description: "Grup İlkesi üzerinden sistem genelinde Windows Copilot'u kapatır; Copilot arka plan ajanlarını, görev çubuğu düğmelerini ve yapay zeka ağ sorgularını durdurur.",
            broken_features: &["Windows Copilot kenar çubuğu ve kısayolu (Win+C) devre dışı kalır"],
        }),
        "disable_game_bar_presence_writer" => Some(LocalizedTweakText {
            title: "Game Bar Varlık Algılamayı, Nexus Başlatıcıyı ve Başlangıç İpuçlarını Kapat",
            description: "Kullanıcı tarafından yazılabilir HKCU\\Software\\Microsoft\\GameBar anahtarları üzerinden Game Bar kontrolcü Nexus başlatmasını ve başlangıç istem panellerini kapatır; TrustedInstaller ACL hataları olmadan katman kancası yükünü ortadan kaldırır.",
            broken_features: &["Xbox Guide Nexus tuşu başlatması ve Game Bar başlangıç paneli devre dışı kalır"],
        }),
        "disable_widgets_news" => Some(LocalizedTweakText {
            title: "Windows Araç Takımlarını (Widgets) ve MSN Haber Akışını Devre Dışı Bırak",
            description: "Görev çubuğu Araç Takımları simgesini devre dışı bırakır; MSN haber akışlarının, gündem konularının ve hava durumu araçlarının arka planda web'den veri çekmesini engeller.",
            broken_features: &["Görev çubuğu Araç Takımları açılır penceresi devre dışı kalır"],
        }),
        "gamedvr_config_store" => Some(LocalizedTweakText {
            title: "Xbox GameDVR Kayıt Katmanını Devre Dışı Bırak",
            description: "Arka plan oyun video kaydını ve Xbox Game Bar kanca enjeksiyonunu kapatarak kare süresi düşüşlerini ve mikro takılmaları ortadan kaldırır.",
            broken_features: &["Xbox Game Bar anlık arka plan tekrar klibi kaydı devre dışı kalır"],
        }),
        "gamedvr_app_capture" => Some(LocalizedTweakText {
            title: "Windows GameDVR Uygulama Yakalamayı Devre Dışı Bırak",
            description: "Windows GameDVR alt sistemindeki arka plan ses/video kayıt arabelleklerini devre dışı bırakır.",
            broken_features: &[],
        }),
        "gamedvr_policy_disable" => Some(LocalizedTweakText {
            title: "GameDVR Sistem İlkesini Devre Dışı Bırak",
            description: "Windows'un arka planda GameDVR yakalama hizmetlerini yeniden etkinleştirmesini engelleyen sistem geneli Grup İlkesi kilidi uygular.",
            broken_features: &[],
        }),
        "consumer_features_disable" => Some(LocalizedTweakText {
            title: "Windows Tüketici Bulut Özelliklerini Devre Dışı Bırak",
            description: "Windows'un bakım sırasında üçüncü taraf sponsorlu uygulama ve oyunları (örn. TikTok, Disney, Candy Crush) otomatik olarak indirip kurmasını engeller.",
            broken_features: &[],
        }),
        "disable_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Geri Bildirim, Autochk ve Haritalar Telemetri Görevlerini Devre Dışı Bırak",
            description: "Windows Geri Bildirim Merkezi SIUF tanılama istemlerini, Autochk telemetri vekilini ve arka plan Haritalar bildirim güncelleme görevlerini devre dışı bırakır (Sophia Script'ten uyarlanmıştır).",
            broken_features: &["Geri Bildirim Merkezi proaktif anket bildirimleri devre dışı kalır"],
        }),
        "disable_hpet_synthetic_timer" => Some(LocalizedTweakText {
            title: "Dinamik Tick (Sentetik Çekirdek Zamanlayıcısı) Özelliğini Kapat",
            description: "Windows NT çekirdeğinde dinamik tick güç tasarrufunu kapatır; sentetik zamanlayıcı saat kaymasını önler ve çok düşük DPC gecikmesi için güvenilir yüksek frekanslı Invariant TSC zamanlayıcı sorgularını zorlar.",
            broken_features: &["Boşta CPU güç kullanımı hafifçe artar"],
        }),
        "optimize_tcp_congestion_tuning" => Some(LocalizedTweakText {
            title: "Ağ TCP Tıkanıklık ve Pencere Otomatik Ayarlamasını Optimize Et",
            description: "Çevrimiçi oyun ağ gecikmesini ve paket serileştirme gecikmesini en aza indirmek için RFC 1323 yüksek performanslı TCP zaman damgalarını etkinleştirir ve Web Services Dynamic Discovery (WSD) yayın trafiğini kapatır.",
            broken_features: &[],
        }),
        "disable_windows_error_reporting" => Some(LocalizedTweakText {
            title: "Windows Hata Bildirimi Grup İlkesini ve Yüklemelerini Devre Dışı Bırak",
            description: "Windows Hata Bildirimi Grup İlkesi kilidini uygular ve Microsoft sunucularına ek tanılama çökme dökümü verilerinin gönderilmesini devre dışı bırakır.",
            broken_features: &["Otomatik hata çökme raporları ve telemetri yüklemeleri engellenir"],
        }),
        "disable_bing_search_start_menu" => Some(LocalizedTweakText {
            title: "Başlat Menüsü Önerilerini ve Iris Reklamlarını Devre Dışı Bırak",
            description: "Windows 10/11 Başlat düzenlerindeki sponsorlu Başlat Menüsü web sitesi/hesap önerilerini, Iris ipuçlarını ve Son Eklenenler uygulama kalabalığını kaldırır.",
            broken_features: &["Başlat Menüsündeki tanıtım önerileri gizlenir"],
        }),
        "disable_cortana_autostart" => Some(LocalizedTweakText {
            title: "Kullanıcı Bazlı Cortana Aramasını ve Görev Çubuğu Düğmesini Kapat",
            description: "Kullanıcı bazlı Cortana etkinleştirmesini, arama konum izinlerini kapatır ve Cortana görev çubuğu öğesini gizler.",
            broken_features: &["Kullanıcı bazlı Cortana görev çubuğu arama tümleştirmesi devre dışı kalır"],
        }),
        // === perf_power_ext.toml (20) ===
        "enable_long_file_paths" => Some(LocalizedTweakText {
            title: "NTFS Win32 Uzun Dosya Yollarını Etkinleştir (>260 Karakter)",
            description: "LongPathsEnabled=1 ayarı ile Windows 10/11 içindeki eski 260 karakterlik MAX_PATH sınırını kaldırarak derleme ve arşiv çıkarma sırasındaki yol kesme hatalarını önler.",
            broken_features: &[],
        }),
        "disable_background_search_toggle_and_app_policy" => Some(LocalizedTweakText {
            title: "Arka Plan Uygulamalarının Çalışmasını ve Arama Arka Plan Anahtarını Reddet",
            description: "AppPrivacy\\LetAppsRunInBackground=2 (Zorunlu Reddet) ve Search\\BackgroundAppGlobalToggle=0 ayarlayarak boşta duran UWP/Store uygulamalarının arka planda CPU veya RAM tüketmesini engeller.",
            broken_features: &["Store/Posta gibi UWP uygulamaları kapalıyken eşitleme yapmaz"],
        }),
        "extend_webclient_file_size_limit" => Some(LocalizedTweakText {
            title: "WebClient / WebDAV Dosya Boyutu Sınırını Azamiye Çıkar (4 GB)",
            description: "Windows WebClient hizmeti indirme sınırını 50 MB'den 4 GB üst sınırına (0xFFFFFFFF) yükselterek bağlı WebDAV / ağ paylaşımı kopyalama hatalarını çözer.",
            broken_features: &[],
        }),
        "reduce_services_pipe_timeout" => Some(LocalizedTweakText {
            title: "Hizmet Başlatma ve Kanal Kapanma Zaman Aşımını 15 Saniyeye Düşür",
            description: "ServicesPipeTimeout değerini varsayılan 30000 ms (30 sn) yerine 15000 ms (15 sn) seviyesine düşürerek takılı kalan arka plan hizmetlerinin açılış ve kapanışta daha hızlı sonlanmasını sağlar.",
            broken_features: &["Açılışta 15 saniyeden uzun süren çok yavaş özel veritabanı hizmetleri zaman aşımına uğrayabilir"],
        }),
        "mmcss_scheduler_full_optimization" => Some(LocalizedTweakText {
            title: "Tam MMCSS Multimedya Zamanlayıcı Optimizasyonu (NoLazyMode, AlwaysOn, Yüksek SFIO)",
            description: "Kesintisiz multimedya ve oyun iş parçacığı zamanlaması için Multimedia\\SystemProfile NoLazyMode=1, AlwaysOn=1 ile Games Tasks Scheduling Category='High' ve SFIO Priority='High' değerlerini yapılandırır.",
            broken_features: &[],
        }),
        "enable_directx_windowed_game_optimizations" => Some(LocalizedTweakText {
            title: "DirectX Pencereli Flip-Model ve VRR İyileştirmelerini Etkinleştir (Kayıpsız Birleştirme)",
            description: "Oyun başına GPU veya AutoHDR tercihlerinizin üzerine yazmadan DirectXUserGlobalSettings içinde SwapEffectUpgradeEnable=1 ve VRROptimizeEnable=1 değerlerini güvenle birleştirir.",
            broken_features: &[],
        }),
        "disable_hardware_overlays_and_min_fps" => Some(LocalizedTweakText {
            title: "Çekirdek Donanım Katmanlarını ve DWM Minimum FPS Kısıtlamasını Kapat",
            description: "Düşük yenileme hızlarında DWM donanım katmanı düzlemi yükseltme duraklamalarını ve takılmaları önlemek için GraphicsDrivers\\DisableOverlays=1 ve Dwm\\OverlayMinFPS=0 ayarlar.",
            broken_features: &["Dizüstü bilgisayarlarda boşta DWM kompozitör güç kullanımı hafifçe artar"],
        }),
        "nvidia_restore_legacy_sharpening" => Some(LocalizedTweakText {
            title: "Eski NVIDIA Sürücü Keskinleştirme İlkesini Geri Yükle (EnableGR535=0)",
            description: "Global\\FTS\\EnableGR535=0 ayarlayarak NVIDIA Denetim Masasının daha yeni katman filtresi yolunu zorlamak yerine klasik düşük yüklü Görüntü Keskinleştirme seçeneğini göstermesini sağlar.",
            broken_features: &[],
        }),
        "nvidia_dlss_debug_indicator" => Some(LocalizedTweakText {
            title: "NVIDIA DLSS Sürüm ve Ön Ayar Hata Ayıklama Filigranını Göster",
            description: "DirectX/Vulkan oyunlarında etkin DLSS SDK sürümünü, çözünürlük ölçeklemeyi ve ön ayarı ekranda göstermek için Global\\NGXCore\\ShowDlssIndicator=1024 ayarlar.",
            broken_features: &["Desteklenen oyunlarda sol altta DLSS tanılama filigranı gösterir"],
        }),
        "disable_game_bar_tips_and_capture" => Some(LocalizedTweakText {
            title: "Game Bar Kontrolcü Nexus'unu, Başlangıç İpuçlarını ve Geçmiş Yakalamayı Kapat",
            description: "UseNexusForGameBarEnabled (Xbox Kılavuz düğmesi başlatıcısı), ShowStartupPanel ipuçları ve arka plan HistoricalCaptureEnabled halka arabellek kaydını kapatır.",
            broken_features: &["Oyun kumandası Nexus düğmesi Game Bar katmanını tetiklemez"],
        }),
        "optimize_mouse_thresholds_and_hover" => Some(LocalizedTweakText {
            title: "Fare İvme Eşiklerini Sıfırla ve 10 ms Anlık Üzerine Gelme Tepkisi Ayarla",
            description: "İşaretçi ivme eşiklerinin tamamen düz olması ve görev çubuğu/araç ipucu üzerine gelme tepkisinin anında çalışması için MouseThreshold1='0', MouseThreshold2='0' ve MouseHoverTime='10' ms değerlerini ayarlar.",
            broken_features: &[],
        }),
        "disable_extra_accessibility_hotkeys" => Some(LocalizedTweakText {
            title: "Fare Tuşları, Yüksek Karşıtlık ve Ekran Okuyucusu Kazara Kısayollarını Kapat",
            description: "Oyun sırasında Alt+Shift+NumLock (Fare Tuşları), Alt+Shift+PrtScn (Yüksek Karşıtlık) ve Win+Ctrl+Enter (Ekran Okuyucusu) kısayol açılır pencerelerini devre dışı bırakır.",
            broken_features: &["Erişilebilirlik klavye kısayolları Ayarlar uygulamasından açılıp kapatılmalıdır"],
        }),
        "disable_smart_multi_homed_name_resolution" => Some(LocalizedTweakText {
            title: "Windows Akıllı Çoklu Ağ Bağlantılı DNS Ad Çözümlemeyi Devre Dışı Bırak",
            description: "Windows'un VPN/sanal bağdaştırıcılar arasında paralel DNS istekleri yayınlamak yerine yalnızca birincil arabirimde DNS sorgusu yapması için DNSClient\\DisableSmartNameResolution=1 ayarlar.",
            broken_features: &["Bölünmüş tünelli (split-tunnel) VPN'lerde bağdaştırıcı DNS metrikleri temiz yapılandırılmalıdır"],
        }),
        "harden_windows_update_ux_and_store" => Some(LocalizedTweakText {
            title: "WU Sürücü Üzerine Yazmalarını, Beklenmedik Yeniden Başlatmaları ve Store Otomatik İndirmeyi Engelle",
            description: "Kalite güncellemeleri sırasında GPU/çevre birimi sürücü gönderimlerini hariç tutar, oturum açıkken zorunlu yeniden başlatmaları önler, uzun güncelleme duraklatma süresini (3650 gün) açar ve güvenlik güncellemelerini açık tutarken sessiz Store arka plan indirmelerini kapatır.",
            broken_features: &["Microsoft Store uygulama güncellemeleri elle denetlenmelidir"],
        }),
        "power_settings_pcie_usb_nvme_epb" => Some(LocalizedTweakText {
            title: "Etkin Güç Planı Çok Düşük Gecikme Ayarları (PCIe ASPM, USB Askıya Alma, NVMe, EPP, PowerThrottling)",
            description: "Etkin güç düzeninde (Dengeli, Yüksek Performans ve özel planlarda çalışır) yerel Windows powrprof API'lerini kullanarak prizde PCIe ASPM, USB seçmeli askıya alma, USB 3 bağlantı güç tasarrufu, birincil/ikincil NVMe boşta uyku ve CPU Enerji Performans Tercihini (EPP=0) kapatır; çekirdek PowerThrottling özelliğini devre dışı bırakır.",
            broken_features: &["Masaüstü ve prize takılı dizüstü bilgisayarlarda prizde boşta güç tüketimi artar"],
        }),
        "disable_storage_sense_policy" => Some(LocalizedTweakText {
            title: "Otomatik Akıllı Depolama (Storage Sense) Arka Plan Taramalarını Kapat",
            description: "Etkileşimli oturumlar sırasında Windows Akıllı Depolama özelliğinin otomatik arka plan disk taramaları çalıştırmasını engeller.",
            broken_features: &["Geçici dosyalar ve Geri Dönüşüm Kutusu Temizlik & Uygulamalar üzerinden elle temizlenmelidir"],
        }),
        "consolidate_svchost_split_threshold" => Some(LocalizedTweakText {
            title: "Hizmet Ana Bilgisayarlarını Fiziksel RAM'e Göre Dinamik Birleştir (SvcHostSplitThresholdInKB) [İsteğe Bağlı]",
            description: "Algılanan sistem RAM miktarından (TotalRamInKb) SvcHostSplitThresholdInKB değerini dinamik hesaplayarak 8 GB+ sistemlerde işlem sayısını ve sanal bellek yükünü azaltır. Uyarı: hizmet başına işlem yalıtımını azaltır.",
            broken_features: &["Paylaşılan svchost çökme yalıtımı azalır; Görev Yöneticisi daha az gruplanmış hizmet gösterir"],
        }),
        "disable_vbs_deviceguard" => Some(LocalizedTweakText {
            title: "Sanallaştırma Tabanlı Güvenliği (VBS) Devre Dışı Bırak [Güvenlik Ödünlü İsteğe Bağlı]",
            description: "EnableVirtualizationBasedSecurity ve RequirePlatformSecurityFeatures ayarlarını devre dışı bırakır. Sanallaştırma yükü bulunan işlemcilerde oyun kare hızını %5-15 artırabilir, ancak çekirdek sanallaştırma güvenlik sınırlarından ödün verir.",
            broken_features: &["VBS güvenlik yalıtımını kapatır; hile önleme / Windows Güvenliği uyarısı çıkabilir"],
        }),
        "disable_hvci_memory_integrity" => Some(LocalizedTweakText {
            title: "Çekirdek Yalıtım / Bellek Bütünlüğünü (HVCI) Devre Dışı Bırak [Güvenlik Ödünlü İsteğe Bağlı]",
            description: "Hiper Yönetici Zorlamalı Kod Bütünlüğünü (HVCI) kapatır. HVCI'nin kare sürelerini düşürdüğü sistemlerde rekabetçi oyunlardaki DPC/kesme gecikmesini azaltır, ancak çekirdek sürücüsü kod imzalama zorlamasından ödün verir.",
            broken_features: &["Windows Güvenliği Çekirdek Yalıtım / Bellek Bütünlüğü Kapalı uyarısı gösterir"],
        }),
        "disable_prefetch_and_sysmain" => Some(LocalizedTweakText {
            title: "Hızlı NVMe SSD'ler İçin Çekirdek Prefetcher ve SysMain (Superfetch) Özelliğini Kapat [İsteğe Bağlı]",
            description: "RAM önbelleklemenin gereksiz arka plan disk G/Ç yükü oluşturduğu hızlı NVMe sürücülerde uygulama/önyükleme Prefetcher ve SysMain bellek ön yüklemesini durdurur.",
            broken_features: &["SATA mekanik disklerde (HDD) soğuk başlatma hızı hafifçe yavaşlayabilir"],
        }),

        // === privacy.toml (12) ===
        "disable_advertising_id" => Some(LocalizedTweakText {
            title: "Reklam Kimliğini Devre Dışı Bırak",
            description: "Windows'un hedefli reklamcılık ve uygulamalar arası izleme amacıyla uygulamalara sağladığı kullanıcı başına reklam kimliğini devre dışı bırakır.",
            broken_features: &["Uygulamalar benzersiz bir reklam kimliği alamaz"],
        }),
        "disable_cortana" => Some(LocalizedTweakText {
            title: "Cortana'yı Devre Dışı Bırak",
            description: "Grup İlkesi üzerinden Cortana'yı devre dışı bırakarak çalışmasını, dinlemesini ve Microsoft bulut hizmetlerine veri göndermesini engeller.",
            broken_features: &["Cortana sesli yardımcısı devre dışı bırakılır"],
        }),
        "disable_activity_history" => Some(LocalizedTweakText {
            title: "Etkinlik Geçmişini / Zaman Çizelgesini Devre Dışı Bırak",
            description: "Windows Etkinlik Geçmişi (Zaman Çizelgesi) özelliğini devre dışı bırakarak Windows'un etkinliklerinizin, ziyaret ettiğiniz web sitelerinin ve açtığınız belgelerin geçmişini toplayıp saklamasını engeller.",
            broken_features: &["Görev Görünümü içindeki Zaman Çizelgesi/Etkinlik Geçmişi boş olur"],
        }),
        "disable_online_speech_recognition" => Some(LocalizedTweakText {
            title: "Çevrimiçi Konuşma Tanımayı Devre Dışı Bırak",
            description: "Bulut tabanlı çevrimiçi konuşma tanıma hizmetini devre dışı bırakır. Ses verileri işlenmek üzere Microsoft sunucularına gönderilmez.",
            broken_features: &["Bulut tabanlı konuşma tanıma çalışmaz; yerel/çevrimdışı tanıma etkilenmez"],
        }),
        "disable_tailored_experiences" => Some(LocalizedTweakText {
            title: "Özelleştirilmiş Deneyimleri Devre Dışı Bırak",
            description: "Microsoft'un Windows içinde kişiselleştirilmiş ipuçları, reklamlar ve öneriler sunmak için tanılama verilerini kullanmasını engeller.",
            broken_features: &["Kişiselleştirilmiş Windows önerileri ve ipuçları devre dışı bırakılır"],
        }),
        "disable_windows_recall" => Some(LocalizedTweakText {
            title: "Windows Recall (Yapay Zeka Anlık Görüntüsü) Özelliğini Devre Dışı Bırak",
            description: "Windows 11 24H2 ile gelen ve etkinliğinizin yapay zeka dizinli ekran görüntülerini düzenli olarak alan Windows Recall özelliğini kapatır. Derleme 26100+ gerektirir.",
            broken_features: &["Windows Recall yapay zeka anlık görüntü özelliği devre dışı bırakılır"],
        }),
        "disable_cloud_content_spotlight" => Some(LocalizedTweakText {
            title: "Bulut İçeriğini ve Windows Spotlight Önerilerini Devre Dışı Bırak",
            description: "Windows Spotlight'ı, Ayarlar'daki önerilen içerikleri ve bulut üzerinden sunulan ön yüklü uygulama önerilerini devre dışı bırakır.",
            broken_features: &[
                "Kilit ekranı Spotlight görselleri devre dışı bırakılır",
                "Başlat menüsünde önerilen uygulamalar görünmez",
            ],
        }),
        "disable_location_tracking" => Some(LocalizedTweakText {
            title: "Konum İzlemeyi Devre Dışı Bırak",
            description: "Windows konum hizmetini genel olarak devre dışı bırakarak uygulamaların ve işletim sisteminin coğrafi konumunuza erişmesini engeller.",
            broken_features: &["Konum gerektiren uygulamalar (Haritalar, Hava Durumu) konumunuza erişemez"],
        }),
        "disable_activity_cloud_sync" => Some(LocalizedTweakText {
            title: "Bulut Panoyu ve Cihazlar Arası Eşitlemeyi Devre Dışı Bırak",
            description: "Grup İlkesi üzerinden Windows Pano Geçmişi kalıcılığını ve cihazlar arası bulut pano eşitlemesini kapatarak kopyalanan parolaların ve hassas verilerin kesinlikle yerel kalmasını sağlar.",
            broken_features: &["Win+V Pano Geçmişi ve cihazlar arası yapıştırma eşitlemesi devre dışı bırakılır"],
        }),
        "disable_cdp_service" => Some(LocalizedTweakText {
            title: "Bağlı Cihazlar Platformu Hizmetini Devre Dışı Bırak",
            description: "Bağlı Cihazlar Platformu Hizmetini (CDPUserSvc) kapatarak cihazlar arası telemetriyi, uygulama eşitleme yayınlarını ve arka plan etkinlik işaretlerini engeller.",
            broken_features: &["Yakındaki paylaşım ve mobil cihazlarla cihazlar arası eşitleme devre dışı bırakılır"],
        }),
        "spotlight_desktop_disable" => Some(LocalizedTweakText {
            title: "Windows Spotlight Kilit Ekranı İndirmelerini Devre Dışı Bırak",
            description: "Windows Spotlight kilit ekranı duvar kağıtlarının ve Microsoft tanıtım görsellerinin arka planda otomatik indirilmesini devre dışı bırakır.",
            broken_features: &[],
        }),
        "spotlight_features_disable" => Some(LocalizedTweakText {
            title: "Kilit Ekranı İpuçlarını ve Reklamlarını Devre Dışı Bırak",
            description: "Content Delivery Manager aracılığıyla sunulan kilit ekranı ipuçlarını, önerileri ve sponsorlu Microsoft Store uygulama tanıtımlarını devre dışı bırakır.",
            broken_features: &[],
        }),

        // === registry_lag.toml (18) ===
        "win32_priority_separation" => Some(LocalizedTweakText {
            title: "Win32PrioritySeparation — Ön Plan Öncelik Artırma",
            description: "Zamanlayıcı kuantumunu kısa, değişken ve yüksek ön plan önceliklendirmeli (0x26 = 38) olarak ayarlar. Oyunlarda ve etkileşimli iş yüklerinde daha düşük giriş gecikmesi için ön plan uygulamalarına öncelik verir.",
            broken_features: &[],
        }),
        "menu_show_delay" => Some(LocalizedTweakText {
            title: "Menü Gösterim Gecikmesini 0 ms'ye İndir",
            description: "Üzerine gelindiğinde menülerin görünmeden önceki yapay gecikmeyi kaldırır. Varsayılan 400 ms'dir; 0 olarak ayarlanması menülerin anında açılmasını sağlar.",
            broken_features: &[],
        }),
        "mmcss_gpu_priority" => Some(LocalizedTweakText {
            title: "MMCSS — Oyun ve Pro Ses Önceliğini Yükselt",
            description: "Multimedya Sınıf Zamanlayıcı Hizmeti (MMCSS) içindeki Oyunlar ve Pro Ses iş parçacığı önceliğini yükselterek ses ve gerçek zamanlı oyun iş parçacıklarının daha yüksek zamanlama önceliği almasını sağlar.",
            broken_features: &[],
        }),
        "mouse_data_queue_size" => Some(LocalizedTweakText {
            title: "Fare Veri Kuyruğu Boyutunu Artır",
            description: "Fare giriş veri kuyruğu boyutunu varsayılan 100 girdiden 256 girdilik daha büyük bir arabelleğe çıkararak yüksek yoklama hızlı oyuncu farelerinde giriş olayı kaybı olasılığını azaltır.",
            broken_features: &[],
        }),
        "keyboard_response" => Some(LocalizedTweakText {
            title: "Klavye Tepkisini İyileştir",
            description: "Kayıt defteri değerleri üzerinden klavye yineleme gecikmesini en kısa, yineleme hızını en hızlı seviyeye getirir ve klavye veri kuyruğu boyutunu artırır.",
            broken_features: &[],
        }),
        "disable_wer_queue" => Some(LocalizedTweakText {
            title: "Windows Hata Bildirimi (WER) Kuyruğunu Devre Dışı Bırak",
            description: "Windows Hata Bildirimi (WER) kuyruğunu kapatarak uygulama çökmelerinden sonra sistemin arka planda çökme dökümü verisi ve rapor yazmasını önler.",
            broken_features: &["Çökme raporları Microsoft'a gönderilmez"],
        }),
        "auto_end_tasks" => Some(LocalizedTweakText {
            title: "Kapanışta Yanıt Vermeyen Görevleri Otomatik Sonlandır",
            description: "Kapatma sırasında yanıt vermeyen uygulamaları kullanıcıya sormadan otomatik olarak sonlandırır. Oturum kapatma ve sistem kapanmasını hızlandırmak için WaitToKillAppTimeout ile HungAppTimeout sürelerini de düşürür.",
            broken_features: &["Yanıt vermeyen uygulamalardaki kaydedilmemiş çalışmalar kapanış sırasında kaybolabilir"],
        }),
        "network_throttling_disable" => Some(LocalizedTweakText {
            title: "Ağ Kısıtlama Dizinini (NetworkThrottlingIndex) Devre Dışı Bırak",
            description: "Windows multimedya ağ kısıtlamasını kapatır (NetworkThrottlingIndex değerini 0xFFFFFFFF yapar); yoğun ağ trafiği veya oyun sırasında paket kaybını ve gecikme sıçramalarını önler.",
            broken_features: &[],
        }),
        "system_responsiveness_gaming" => Some(LocalizedTweakText {
            title: "Ön Plan Sistem Tepkiselliğini Azamiye Çıkar",
            description: "SystemResponsiveness değerini %0 olarak yapılandırır; arka plan işletim sistemi görevleri için %20 ayırmak yerine CPU işlem döngülerinin %100'ünü ön plan etkileşimli oyun ve uygulamalara adar.",
            broken_features: &[],
        }),
        "ntfs_disable_last_access" => Some(LocalizedTweakText {
            title: "NTFS Son Erişim Zaman Damgalarını Devre Dışı Bırak",
            description: "NTFS dosya sisteminin her dosya okuma işleminde disk meta veri zaman damgalarını güncellemesini önleyerek SSD ve NVMe okuma/yazma G/Ç yükünü önemli ölçüde azaltır.",
            broken_features: &["Dosya özellikleri tam son erişim tarihini göstermez"],
        }),
        "disable_paging_executive" => Some(LocalizedTweakText {
            title: "Windows Çekirdeğini ve Sürücüleri RAM'de Tut",
            description: "Windows NT yürütücü çekirdeği ve sürücü yordamlarının diske sayfa dosyası olarak yazılmak yerine fiziksel RAM'de yerleşik kalmasını zorlayarak anlık çekirdek kesme tepkisi sağlar.",
            broken_features: &[],
        }),
        "large_system_cache" => Some(LocalizedTweakText {
            title: "Büyük Sistem Dosya Önbelleğini Etkinleştir",
            description: "Windows'un fiziksel RAM'de daha büyük bir dosya sistemi önbelleği tutmasını yapılandırarak yinelenen dosya erişimini ve oyun doku akışını hızlandırır.",
            broken_features: &[],
        }),
        "disable_fast_startup" => Some(LocalizedTweakText {
            title: "Hızlı Başlatmayı Devre Dışı Bırak (Hibrit Uyku Sızıntı Önleme)",
            description: "Windows Hızlı Başlatmayı (hiberboot) kapatarak her kapanışta gerçek bir temiz önyükleme döngüsü sağlar ve oturumlar arasında çekirdek bellek sızıntıları ile sürücü durumu bozulmalarını önler.",
            broken_features: &["Yavaş mekanik sürücülerde soğuk önyükleme süresi 1-2 saniye artabilir"],
        }),
        "ntfs_disable_8dot3" => Some(LocalizedTweakText {
            title: "NTFS 8.3 Kısa Ad Oluşturmayı Devre Dışı Bırak",
            description: "NTFS birimleri üzerinde eski MS-DOS 8.3 kısa dosya adı üretimini kapatarak yüksek hızlı NVMe/SSD'lerde dizin dosya oluşturma gecikmesini ve yazma yükünü ortadan kaldırır.",
            broken_features: &[],
        }),
        "increase_mft_zone_reservation" => Some(LocalizedTweakText {
            title: "Ana Dosya Tablosu (MFT) Bölge Arabelleğini Artır",
            description: "NTFS Ana Dosya Tablosu için daha büyük bir bitişik disk arabelleği ayırarak MFT parçalanmasını önler ve maksimum disk erişim aktarım hızını korur.",
            broken_features: &[],
        }),
        "tcp_ack_frequency" => Some(LocalizedTweakText {
            title: "TCP Düşük Gecikme (TcpAckFrequency)",
            description: "Windows TCP ağ yığınının zaman aşımı birleştirmesini beklemeden paket ACK yanıtlarını hemen göndermesini zorlayarak çevrimiçi oyunlarda ağ pingini düşürür.",
            broken_features: &[],
        }),
        "tcp_no_delay_global" => Some(LocalizedTweakText {
            title: "Nagle Algoritmasını Devre Dışı Bırak (TCPNoDelay)",
            description: "Etkin ağ arabirimlerinde Nagle paket arabellekleme algoritmasını kapatarak en düşük ağ gidiş-dönüş gecikmesi için küçük paketleri gecikmeden iletir.",
            broken_features: &[],
        }),
        "disable_mouse_acceleration" => Some(LocalizedTweakText {
            title: "İşaretçi Hassasiyetini Artır Seçeneğini Kapat (1:1 Fare Girişi)",
            description: "Doğrusal olmayan Windows imleç ivmelenmesini devre dışı bırakarak rekabetçi hassas nişan alma için gerekli gerçek 1:1 ham donanım fare hareketini sunar.",
            broken_features: &[],
        }),

        // === telemetry_apps.toml (7) ===
        "disable_nvidia_telemetry_service" => Some(LocalizedTweakText {
            title: "NVIDIA Telemetri Kapsayıcı Hizmetini Devre Dışı Bırak",
            description: "NVIDIA sürücü kullanım istatistiklerini ve çökme verilerini toplayıp ileten NvTelemetryContainer hizmetini devre dışı bırakır.",
            broken_features: &["NVIDIA çökme verileri ve kullanım telemetrisi toplanmaz"],
        }),
        "disable_nvidia_telemetry_tasks" => Some(LocalizedTweakText {
            title: "NVIDIA Telemetri Zamanlanmış Görevlerini Devre Dışı Bırak",
            description: "NVIDIA telemetri verilerini düzenli olarak bildiren NvTmMon, NvTmRep ve NvProfile zamanlanmış görevlerini devre dışı bırakır.",
            broken_features: &[],
        }),
        "disable_adobe_update_service" => Some(LocalizedTweakText {
            title: "Adobe Güncelleme Hizmetlerini Devre Dışı Bırak",
            description: "Adobe Acrobat Update Service (AdobeARMservice) ve Adobe Genuine Monitor Service hizmetlerini kapatır. Arka plan güncelleme denetimlerini ve lisans telemetrisini engeller.",
            broken_features: &["Adobe ürünleri otomatik güncellenmez; elle güncelleyin"],
        }),
        "disable_edge_update" => Some(LocalizedTweakText {
            title: "Microsoft Edge Güncelleme Hizmetlerini Devre Dışı Bırak",
            description: "Microsoft Edge Update hizmetlerini (edgeupdate ve edgeupdatem) ve zamanlanmış güncelleme görevlerini kapatır. Edge çalışmaya devam eder ancak otomatik güncellenmez veya güncelleme denetimi için dışarı bağlanmaz.",
            broken_features: &["Microsoft Edge otomatik güncellenmez; güvenlik yamaları elle uygulanmalıdır"],
        }),
        "disable_chrome_update" => Some(LocalizedTweakText {
            title: "Google Chrome Güncelleme Hizmetlerini Devre Dışı Bırak",
            description: "Google Update hizmetlerini (gupdate ve gupdatem) ve bunların zamanlanmış görevlerini kapatır. Chrome çalışmaya devam eder ancak otomatik güncellenmez.",
            broken_features: &["Google Chrome otomatik güncellenmez; güvenlik yamaları elle uygulanmalıdır"],
        }),
        "disable_steam_client_service" => Some(LocalizedTweakText {
            title: "Steam İstemci Hizmeti Otomatik Başlatmasını Devre Dışı Bırak",
            description: "Steam Client Service başlangıç türünü otomatikten elle başlatmaya çevirir. Steam başlatıldığında hizmeti isteğe bağlı olarak yine çalıştırır, ancak Steam kullanılmıyorken arka planda kaynak tüketmez.",
            broken_features: &[],
        }),
        "disable_razer_telemetry" => Some(LocalizedTweakText {
            title: "Razer Telemetri ve Analizlerini Devre Dışı Bırak",
            description: "Razer Game Manager Service ve Razer Chroma SDK hizmetlerinin otomatik başlamasını engelleyerek Razer Synapse kaynaklı arka plan telemetrisini ve analizlerini azaltır.",
            broken_features: &["Razer Synapse otomatik başlatma özellikleri elle başlatma gerektirebilir"],
        }),

        // === telemetry_os.toml (14) ===
        "disable_diagtrack" => Some(LocalizedTweakText {
            title: "DiagTrack Hizmetini Devre Dışı Bırak (Bağlı Kullanıcı Deneyimleri ve Telemetri)",
            description: "Birincil Windows telemetri toplama hizmeti olan Bağlı Kullanıcı Deneyimleri ve Telemetri hizmetini (DiagTrack / diagsvc) devre dışı bırakır.",
            broken_features: &["Tanılama verileri Microsoft'a gönderilmez"],
        }),
        "disable_dmwappushservice" => Some(LocalizedTweakText {
            title: "dmwappushservice Hizmetini Devre Dışı Bırak (WAP Push İleti Yönlendirme)",
            description: "Telemetri veri yönlendirmesi için kullanılan Cihaz Yönetimi Kablosuz Uygulama Protokolü (WAP) Push ileti yönlendirme hizmetini devre dışı bırakır.",
            broken_features: &["MDM anlık iletileri alınmaz"],
        }),
        "disable_telemetry_registry" => Some(LocalizedTweakText {
            title: "AllowTelemetry Kayıt Defteri Üzerinden Telemetriyi Devre Dışı Bırak",
            description: "AllowTelemetry ilkesini en düşük telemetri düzeyi olan 0 (Güvenlik/Kapalı) olarak ayarlar. Enterprise ve Education sürümlerinde telemetriyi tamamen kapatır; Home ve Pro sürümlerinde gereken en düşük düzeye indirir.",
            broken_features: &["Bazı Windows Insider ve tanılama özellikleri çalışmayabilir"],
        }),
        "disable_compat_telrunner" => Some(LocalizedTweakText {
            title: "Uygulama Uyumluluğu Değerlendirici Görevlerini Devre Dışı Bırak",
            description: "Yüklü yazılımları tarayıp uyumluluk telemetrisi gönderen CompatTelRunner.exe dosyasını çalıştıran Microsoft Compatibility Appraiser ve ProgramDataUpdater zamanlanmış görevlerini kapatır.",
            broken_features: &["Windows yükseltme uyumluluk denetimleri otomatik çalışmaz"],
        }),
        "disable_ceip" => Some(LocalizedTweakText {
            title: "Müşteri Deneyimi Geliştirme Programını (CEIP) Devre Dışı Bırak",
            description: "CEIP zamanlanmış görevlerini (Consolidator, UsbCeip, KernelCeipTask) ve CEIP ilke kayıt defteri anahtarını devre dışı bırakır.",
            broken_features: &["CEIP telemetri verileri toplanmaz"],
        }),
        "disable_wer_service" => Some(LocalizedTweakText {
            title: "Windows Hata Bildirimi Hizmetini Devre Dışı Bırak",
            description: "Windows Hata Bildirimi hizmetini (WerSvc) kapatarak çökme raporlarının ve hata telemetrisinin Microsoft'a yüklenmesini engeller.",
            broken_features: &[
                "Çökme raporları Microsoft'a gönderilmez",
                "Sorun Raporları geçmişi güncellenmez",
            ],
        }),
        "disable_connected_user_experiences" => Some(LocalizedTweakText {
            title: "Bağlı Kullanıcı Deneyimlerini ve Önerileri Devre Dışı Bırak",
            description: "Ayarlar uygulamasında, Başlat menüsünde ve kilit ekranında bulut tabanlı içerik ve telemetri geri bildirimleriyle sunulan önerileri kapatır.",
            broken_features: &["Windows önerileri ve ipuçları görünmez"],
        }),
        "disable_diagnostic_data_viewer" => Some(LocalizedTweakText {
            title: "Tanılama Veri Görüntüleyicisini Devre Dışı Bırak",
            description: "Microsoft'a gönderilen tanılama verilerinin yerel bir kopyasını depolayan Tanılama Veri Görüntüleyicisi olay günlüğünü kapatır. Disk G/Ç ve depolama alanı kazandırır.",
            broken_features: &["Tanılama Verileri Görüntüleyicisi uygulaması üzerinden tanılama verileri yerel olarak görüntülenemez"],
        }),
        "disable_delivery_optimization_p2p" => Some(LocalizedTweakText {
            title: "Teslim İyileştirme P2P Karşıya Yükleme Bant Genişliği Paylaşımını Kapat",
            description: "Windows Teslim İyileştirmeyi kısıtlar (DODownloadMode = 0); Windows'un internet üzerindeki rastgele bilgisayarlara güncelleme dağıtmak için karşıya yükleme bant genişliğinizi kullanmasını durdurur.",
            broken_features: &[],
        }),
        "disable_diagnostic_execution_service" => Some(LocalizedTweakText {
            title: "Tanılama Yürütme Hizmetini Devre Dışı Bırak (diagsvc)",
            description: "Zamanlanmış arka plan tanılamalarını ve telemetri paketi yürütmelerini gerçekleştiren Tanılama Yürütme Hizmetini (diagsvc) devre dışı bırakır.",
            broken_features: &[],
        }),
        "wersvc_service_disable" => Some(LocalizedTweakText {
            title: "Tanılama Merkezi Standart Toplayıcısını Devre Dışı Bırak",
            description: "Visual Studio / Windows Diagnostics Hub Standart Toplayıcı Hizmetini kapatarak boştayken arka plan ETW izleme toplayıcılarını durdurur.",
            broken_features: &["VS/Windows Tanılama Merkezi izleme toplaması elle başlatma gerektirir"],
        }),
        "disable_remote_registry" => Some(LocalizedTweakText {
            title: "Uzak Kayıt Defteri Hizmetini Devre Dışı Bırak",
            description: "RemoteRegistry hizmetini devre dışı bırakarak uzak kullanıcıların ve harici ağ varlıklarının yerel kayıt defteri anahtarlarını sorgulamasını veya değiştirmesini engeller.",
            broken_features: &["Yerel LAN üzerinden kayıt defterinin uzaktan yönetimsel idaresi devre dışı bırakılır"],
        }),
        "disable_disk_diagnostic_task" => Some(LocalizedTweakText {
            title: "Disk Tanılama Telemetri Görevini Devre Dışı Bırak",
            description: "Telemetri raporlaması için disk kullanım ölçümlerini ve donanım tanılamalarını toplayan zamanlanmış görevi devre dışı bırakır.",
            broken_features: &[],
        }),
        "disable_winsat_task" => Some(LocalizedTweakText {
            title: "WinSAT Arka Plan Karşılaştırma Görevini Devre Dışı Bırak",
            description: "Zamanlanmış Windows Sistem Değerlendirme Aracı görevinin sistem boştayken kendiliğinden disk ve GPU karşılaştırma stres testleri çalıştırmasını engeller.",
            broken_features: &[],
        }),

        // === ux_customization.toml (13) ===
        "enable_classic_context_menu" => Some(LocalizedTweakText {
            title: "Tam Klasik Windows 10 Sağ Tık Bağlam Menüsünü Geri Yükle",
            description: "HKCU\\Software\\Classes altında CLSID geçersiz kılmasını kaydederek sağ tıklandığında fazladan 'Diğer seçenekleri göster' tıklaması olmadan eksiksiz bağlam menüsünün hemen açılmasını sağlar.",
            broken_features: &["Modern kompakt Win11 bağlam menüsü klasik tam menüyle değiştirilir"],
        }),
        "explorer_power_user_defaults" => Some(LocalizedTweakText {
            title: "Dosya Uzantılarını, Tam Başlık Çubuğu Yolunu, Bu Bilgisayar Açılışını Göster ve Eşitleme Uyarılarını Gizle",
            description: "Bilinen dosya uzantılarını ve gizli dosyaları gösterir, Gezgini Hızlı Erişim yerine doğrudan 'Bu Bilgisayar' görünümünde açar, başlık çubuğu tam yollarını ve kompakt öğe aralığını etkinleştirir ve Gezgindeki OneDrive/eşitleme sağlayıcısı başlıklarını gizler.",
            broken_features: &[],
        }),
        "explorer_hide_win11_home_gallery_nav" => Some(LocalizedTweakText {
            title: "Dosya Gezgini Gezinti Bölmesinden Giriş ve Galeri Sabitlemelerini Gizle",
            description: "HKCU\\Software\\Classes\\CLSID üzerinden Dosya Gezgini sol gezinti bölmesindeki Windows 11 Giriş (Home) ve Galeri (Gallery) öğelerinin sabitlemesini kaldırır.",
            broken_features: &["Giriş ve Galeri hızlı bağlantıları Gezgin sol kenar çubuğundan gizlenir"],
        }),
        "explorer_disable_startup_delay_and_jpeg_compression" => Some(LocalizedTweakText {
            title: "Uygulama Başlangıç Gecikmesini Kaldır ve Duvar Kağıdı JPEG Sıkıştırmasını Kapat",
            description: "Başlangıç programlarının yapay 10 saniyelik kısıtlama olmadan başlaması için Explorer\\Serialize\\StartupDelayInMSec=0 ve kayıpsız masaüstü duvar kağıtları için JPEGImportQuality=100 ayarlar.",
            broken_features: &[],
        }),
        "explorer_desktop_this_pc_and_disable_autoplay" => Some(LocalizedTweakText {
            title: "'Bu Bilgisayar' Masaüstü Simgesini Göster ve Çıkarılabilir Medya Otomatik Kullanmayı Kapat",
            description: "Masaüstüne 'Bu Bilgisayar' kısayol simgesini yerleştirir ve otomatik medya pencereleri ile autorun tehditlerini engellemek için çıkarılabilir USB/optik sürücülerde Otomatik Kullan/Otomatik Çalıştır özelliğini kapatır.",
            broken_features: &["Takılan USB/SD medyalar artık otomatik olarak istem penceresi açmaz"],
        }),
        "disable_visual_effects_userpreferencesmask_bits" => Some(LocalizedTweakText {
            title: "UserPreferencesMask Bit Alanları ve DWM ile Ağır Arayüz Solma/Kayma/Gölge Animasyonlarını Kaldır",
            description: "UserPreferencesMask üzerinde nokta atışı bit maskeleme değişiklikleri (yazı tipi yumuşatma ve temel masaüstü öğelerini koruyarak menü solma/kayma, araç ipucu solması, menü seçim solması, imleç gölgesi, açılan kutu kayması, yumuşak liste kaydırma ve arayüz denetim animasyonlarını kapatır) ile EnableAeroPeek=0 uygular.",
            broken_features: &["Kayma/solma animasyonları veya alt gölgeler olmadan klasik seri arayüz geçişleri"],
        }),
        "taskbar_win11_power_layout_and_end_task" => Some(LocalizedTweakText {
            title: "Win11 Görev Çubuğunu Sola Hizala, Araçlar/Görev Görünümü/Aramayı Gizle, Görev Çubuğu Görevi Sonlandır'ı Aç (Pro/Ent İlkesi)",
            description: "Görev çubuğu simgelerini sola hizalar (TaskbarAl=0), Görev Görünümü ve Araç Takımları kalabalığını kaldırır (TaskbarDa=0), görev çubuğu Arama kutusunu gizler, görev çubuğu geliştirici ayarlarında sağ tık 'Görevi Sonlandır' seçeneğini açar, Aero Shake küçültmeyi kapatır ve Başlat menüsü 'Önerilenler' alanını kaldırır (Windows 11 22H2+ Pro/Enterprise/Education).",
            broken_features: &["Görev çubuğu simgeleri sola hizalanır; Görev Görünümü ve Araç Takımları düğmeleri gizlenir"],
        }),
        "hide_win10_meet_now_tray" => Some(LocalizedTweakText {
            title: "Windows 10 'Şimdi Toplantı Yap' Kamera Tepsi Simgesini Gizle",
            description: "Windows 10 bildirim alanından Skype 'Şimdi Toplantı Yap' (Meet Now) simgesini gizler (yalnızca Windows 10 derlemeleri için).",
            broken_features: &[],
        }),
        "enable_windows_dark_mode_and_clock_seconds" => Some(LocalizedTweakText {
            title: "Windows Sistem ve Uygulama Karanlık Modu + Görev Çubuğu Saat Saniyesini Etkinleştir",
            description: "Windows arayüzünde yerel Karanlık Mod için AppsUseLightTheme=0 ve SystemUsesLightTheme=0 değerlerini değiştirir ve görev çubuğu saatinde canlı saniyeleri gösterir.",
            broken_features: &[],
        }),
        "disable_startup_sound_and_audio_ducking" => Some(LocalizedTweakText {
            title: "Windows Başlangıç Sesini ve Arama Sırasında Ses Kısma Özelliğini Kapat",
            description: "Windows oturum açma sesini susturur ve Discord, Teams veya ses uygulamaları iletişim akışı açtığında Windows'un oyun/müzik sesini %80 düşürmesini durdurur.",
            broken_features: &[],
        }),
        "enable_verbose_logon_status" => Some(LocalizedTweakText {
            title: "Ayrıntılı Windows Önyükleme / Oturum Açma / Kapatma Durum İletilerini Etkinleştir",
            description: "VerboseStatus=1 ayarlayarak Windows'un genel dönen bekleme ekranları yerine kesin hizmet/güncelleme aşamalarını göstermesini sağlar.",
            broken_features: &[],
        }),
        "disable_lock_screen_pro" => Some(LocalizedTweakText {
            title: "Oturum Açma Öncesi Kilit Ekranı Perdesini Atla (Pro/Enterprise İlkesi)",
            description: "Önyükleme veya kilitleme sonrasında önce Kilit Ekranı fotoğrafını kaydırıp kapatmaya gerek kalmadan doğrudan parola/PIN oturum açma istemine geçer.",
            broken_features: &["Pro/Enterprise sürümlerinde yukarı kaydırılan kilit ekranı perdesi atlanır"],
        }),
        "enable_windows_developer_mode" => Some(LocalizedTweakText {
            title: "Windows Geliştirici Modunu Etkinleştir (Yan Yüklemeli Uygulamalar ve Sembolik Bağlar)",
            description: "AllowDevelopmentWithoutDevLicense=1 ayarını etkinleştirerek kullanıcıların UAC olmadan dosya sistemi sembolik bağları oluşturmasını ve MSIX/AppX paketlerini dışarıdan yüklemesini sağlar.",
            broken_features: &["İmzasız MSIX dışarıdan yüklemesine izin verir; geliştiriciler içindir"],
        }),
        // === system_deep_opt.toml (30) ===
        "disable_dwm_animations_per_process" => Some(LocalizedTweakText {
            title: "İşlem Başına DWM Geçiş Animasyonlarını Devre Dışı Bırak",
            description: "DWM anahtarı altında DisallowAnimations=1 ve AnimationsShiftKey=1 değerlerini ayarlayarak işlem başına DWM geçiş animasyonlarını devre dışı bırakır. Pencereler küçültüldüğünde, büyütüldüğünde veya değiştirildiğinde solma ve kayma animasyonlarını kaldırarak GPU bindirmesini azaltır.",
            broken_features: &["Pencere geçiş animasyonları kaldırılacaktır"],
        }),
        "disable_dwm_composition_throttle" => Some(LocalizedTweakText {
            title: "Masaüstü Pencere Yöneticisi Kompozisyon Kısmasını Devre Dışı Bırak",
            description: "DWM anahtarı altında DesktopCompositionThrottle=0 değerini ayarlayarak kare atlandığında kompozitörün işlemeyi yavaşlatmasını önler. GPU yükü altındayken bile DWM'nin maksimum kompozisyon verimini korumasını sağlar.",
            broken_features: &[],
        }),
        "disable_kernel_mitigations_audit" => Some(LocalizedTweakText {
            title: "Çekirdek Azaltma Önlemleri Denetim Modunu Devre Dışı Bırak",
            description: "MitigationAuditOptions değerini tamamen sıfırlayarak çekirdek istismar azaltma denetim günlüğünü devre dışı bırakır. Süreç oluşturma ve özel durum işlemede ETW günlüğü yükünü azaltır.",
            broken_features: &["İstismar azaltma denetim olayları günlüğe kaydedilmeyecektir"],
        }),
        "disable_llmnr_protocol" => Some(LocalizedTweakText {
            title: "LLMNR Ad Çözümleme Protokolünü Devre Dışı Bırak",
            description: "DNS istemci parametrelerinde EnableMulticast=0 ayarlayarak Bağlantı-Yerel Çok Noktaya Yayın Ad Çözümlemesini (LLMNR) devre dışı bırakır. Ağ üzerindeki gereksiz çok noktaya yayın trafiğini ve LLMNR zehirleme saldırı risklerini önler.",
            broken_features: &["DNS olmadan yerel ağ ad çözümlemesi başarısız olabilir", "Yerel ağdaki cihaz algılama işlevi kısıtlanabilir"],
        }),
        "disable_memory_compression" => Some(LocalizedTweakText {
            title: "Bellek Sıkıştırma Motorunu Devre Dışı Bırak",
            description: "DisableCompression=1 ayarlayarak Windows Bellek Yöneticisi sıkıştırma deposunu devre dışı bırakır. 16 GB ve üzeri RAM bulunan sistemlerde Sistem işlemi bellek sıkıştırma iş parçacığından kaynaklanan CPU kullanım artışlarını önler.",
            broken_features: &["Bellek kısıtlı sistemlerde daha yüksek takas dosyası kullanımı", "8 GB sistemlerde bellek yetersizliği durumlarına daha erken yol açabilir"],
        }),
        "disable_netbios_over_tcp" => Some(LocalizedTweakText {
            title: "TCP/IP Üzerinden NetBIOS'u Devre Dışı Bırak",
            description: "NetbiosOptions=2 ayarlayarak ve LMHosts aramasını kapatıp TCP/IP üzerinden NetBIOS protokolünü devre dışı bırakır. NetBIOS, ad çözümleme sorguları yayınlayan ve sistemi gereksiz ağ gürültüsüne ve olası NBNS zehirleme saldırılarına maruz bırakan eski bir LAN protokolüdür.",
            broken_features: &["NetBIOS ad çözümlemesine dayanan eski uygulamalar çalışmayacaktır", "WINS kullanan eski ağ yazıcılarına erişilemeyebilir"],
        }),
        "disable_ntfs_encryption_driver" => Some(LocalizedTweakText {
            title: "NTFS Dosya Sistemi Şifreleme (EFS) Sürücüsünü Devre Dışı Bırak",
            description: "FileSystem anahtarı altında NtfsDisableEncryption=1 ayarlayarak NTFS Şifreleme Dosya Sistemi filtre sürücüsünün dosya G/Ç işlemlerini kesmesini önler. EFS şifrelemesi kullanmayan sistemlerde dosya erişim gecikmesini azaltır.",
            broken_features: &["NTFS Şifreli Dosya Sistemi (EFS) çalışmayacaktır", "Mevcut EFS ile şifrelenmiş dosyalara erişilemez hale gelir"],
        }),
        "disable_page_file_encryption" => Some(LocalizedTweakText {
            title: "Takas Dosyası Şifrelemesini ve Kapanışta Temizlemeyi Devre Dışı Bırak",
            description: "Windows'un takas dosyası içeriklerini şifrelemesini önler ve kapanışta takas dosyasının temizlenmesini devre dışı bırakır (ClearPageFileAtShutdown=0, DisablePagingEncryption=1). Bellek sayfalama verimini artırır ve kapanış süresini kısaltır.",
            broken_features: &["Takaslanan bellek içerikleri kapatma sonrasında diskte kalır", "Hassas veriler ham disk üzerinden kurtarılabilir olabilir"],
        }),
        "disable_processor_idle_demote_threshold" => Some(LocalizedTweakText {
            title: "İşlemci Boşta Durum İndirgemesini Devre Dışı Bırak",
            description: "İşlemci güç yönetimi parametrelerinde IdleDemoteThreshold=100 ve IdlePromoteThreshold=100 ayarlayarak kısa boşta kalma sürelerinde çekirdek güç yöneticisinin işlemci performans durumlarını düşürmesini önler. Oyunlarda ve gerçek zamanlı işlemlerde frekans artırma gecikmelerini ortadan kaldırır.",
            broken_features: &["Boşta daha yüksek güç tüketimi", "Dizüstü bilgisayarlarda azalan pil ömrü"],
        }),
        "disable_rss_base_cpu" => Some(LocalizedTweakText {
            title: "Ağ Yük Dağıtımı İçin RSS Temel CPU Ayarla",
            description: "NDIS parametrelerinde RssBaseCpu=1 ayarlayarak Alma Tarafı Ölçeklendirmesinin (RSS) ağ kesme işlemlerini CPU 0 yerine CPU 1'den dağıtmaya başlamasını sağlar. CPU 0'ı ön plan uygulamaları ve oyunlar için serbest bırakır.",
            broken_features: &[],
        }),
        "disable_spectre_meltdown_mitigations" => Some(LocalizedTweakText {
            title: "Spectre/Meltdown CPU Güvenlik Açığı Önlemlerini Devre Dışı Bırak",
            description: "FeatureSettingsOverride=3 ve FeatureSettingsOverrideMask=3 ayarlayarak donanım düzeyinde Spectre ve Meltdown CPU açık önlemlerini devre dışı bırakır. Ağır hesaplama işlerinde CPU verimini en üst seviyeye çıkarır.",
            broken_features: &["Sistem, Spectre ve Meltdown CPU yan kanal saldırılarına karşı savunmasız kalır"],
        }),
        "disable_threaded_dpc_optimization" => Some(LocalizedTweakText {
            title: "Daha Düşük Kesme Gecikmesi İçin İş Parçacıklı DPC'yi Devre Dışı Bırak",
            description: "Çekirdek anahtarında ThreadDpcEnable=0 ayarlayarak Ertelenmiş Yordam Çağrılarının (DPC) ayrılmış bir iş parçacığı yerine doğrudan DISPATCH_LEVEL seviyesinde yürütülmesini zorlar. Gerçek zamanlı ses üretimi ve rekabetçi oyunlar için DPC gecikmesini en aza indirir.",
            broken_features: &["İş parçacıklı DPC bekleyen bazı sürücüler hatalı davranabilir", "Aşırı çoklu iş parçacığı yükü altında daha yüksek DPC gecikme değişkenliği"],
        }),
        "disable_windows_search_indexer" => Some(LocalizedTweakText {
            title: "Windows Arama Dizin Oluşturucu Hizmetini Devre Dışı Bırak",
            description: "WSearch hizmetini devre dışı bırakarak arka plandaki dizin oluşturucunun tüm sürücülerdeki dosya içeriklerini taramasını durdurur. SSD/NVMe sistemlerde SearchIndexer.exe kaynaklı disk G/Ç, CPU ve bellek tüketimini ortadan kaldırır.",
            broken_features: &["Windows Arama dosya içerikleri için sonuç döndürmeyecektir", "Outlook masaüstü araması daha yavaş olabilir", "Başlat Menüsü araması dosyaları içeriğe göre bulamayacaktır"],
        }),
        "enable_hardware_task_offload" => Some(LocalizedTweakText {
            title: "TCP/IP Donanım Görev Boşaltımını Etkinleştir",
            description: "TCP/IP parametrelerinde DisableTaskOffload=0 ayarlayarak işletim sisteminin sağlama toplamı hesaplamasını ve TCP bölümlemesini ağ kartı donanımına aktarmasını sağlar. CPU döngülerini serbest bırakır.",
            broken_features: &[],
        }),
        "enable_large_ntfs_frs" => Some(LocalizedTweakText {
            title: "Büyük NTFS Dosya Kayıt Segmentlerini Etkinleştir",
            description: "NtfsAllowExtendedCharacterIn8dot3Name=0 ayarlayarak 8.3 kısa adlarındaki genişletilmiş karakter desteğini devre dışı bırakır ve NTFS sürücüsünün daha büyük, verimli Dosya Kayıt Segmentleri ayırmasını sağlar.",
            broken_features: &["Otomatik oluşturulan 8.3 kısa adlarındaki genişletilmiş Unicode karakterleri kaldırılacaktır"],
        }),
        "enable_ultimate_performance_plan" => Some(LocalizedTweakText {
            title: "Nihai Performans Güç Planını Etkinleştir",
            description: "Gizli Nihai Performans güç düzenini etkinleştirir. Çekirdek park etme yok, frekans ölçekleme gecikmeleri yok, USB seçmeli askıya alma yok ve disk durdurma yok. En düşük gecikme süresini sağlar.",
            broken_features: &["Boşta belirgin şekilde daha yüksek güç tüketimi", "Dizüstü bilgisayarlarda azalan pil ömrü", "Boşta daha yüksek CPU ve yonga seti sıcaklıkları"],
        }),
        "increase_gpu_tdr_timeout" => Some(LocalizedTweakText {
            title: "GPU Zaman Aşımı Algılama ve Kurtarma Gecikmesini Artır",
            description: "TdrDelay değerini 2 saniyeden 10 saniyeye yükselterek, Windows ekran sürücüsünü sıfırlamadan önce GPU'ya uzun süren gölgelendirici hesaplamalarını tamamlaması için daha fazla süre tanır.",
            broken_features: &["Gerçek GPU kilitlenmelerinden kurtarma daha uzun sürecektir", "Sürücü sıfırlanmadan önce ekran daha uzun süre donmuş görünebilir"],
        }),
        "increase_irp_stack_size" => Some(LocalizedTweakText {
            title: "Ağ/Disk G/Ç İçin IRP Yığın Boyutunu Artır",
            description: "G/Ç İstek Paketi yığın boyutunu 15'ten 32'ye yükselterek SMB sunucusu ve katmanlı filtre sürücülerine daha fazla yığın alanı sağlar. STATUS_INSUFFICIENT_RESOURCES hatalarını önler ve G/Ç verimini artırır.",
            broken_features: &[],
        }),
        "increase_max_connections_per_server" => Some(LocalizedTweakText {
            title: "Sunucu Başına Maksimum HTTP Bağlantı Sayısını Artır",
            description: "MaxConnectionsPerServer ve MaxConnectionsPer1_0Server değerlerini 16'ya yükselterek WinINet tabanlı uygulamaların aynı ana bilgisayara daha fazla eşzamanlı HTTP bağlantısı açmasını sağlar ve indirmeleri hızlandırır.",
            broken_features: &[],
        }),
        "increase_nonpaged_pool_quota" => Some(LocalizedTweakText {
            title: "Sürücü Kararlılığı İçin Sayfalanmayan Havuz Kotasını Artır",
            description: "NonPagedPoolQuota değerini 0 (sistem tarafından yönetilen maksimum) olarak ayarlayarak çekirdek bellek yöneticisinin sayfalanmayan tahsisler için tüm fiziksel belleği kullanmasına izin verir.",
            broken_features: &[],
        }),
        "optimize_additional_critical_worker_threads" => Some(LocalizedTweakText {
            title: "Ek Çekirdek Çalışma İş Parçacıklarını Artır",
            description: "AdditionalCriticalWorkerThreads ve AdditionalDelayedWorkerThreads değerlerini 2 olarak ayarlayarak kritik ve gecikmeli iş öğelerini işlemek için ek çekirdek modu iş parçacıkları sağlar ve kuyruk duraksamalarını önler.",
            broken_features: &[],
        }),
        "optimize_default_ttl" => Some(LocalizedTweakText {
            title: "En Uygun Varsayılan IP Paketi TTL Değerini Ayarla",
            description: "Varsayılan IP Yaşam Süresi (TTL) değerini Linux/macOS standardıyla eşleşecek şekilde 64 atlama olarak ayarlar. 64 değeri neredeyse tüm internet yönlendirme topolojileri için idealdir.",
            broken_features: &[],
        }),
        "optimize_disk_io_page_lock_limit" => Some(LocalizedTweakText {
            title: "Büyük Aktarımlar İçin G/Ç Sayfa Kilitleme Sınırını Artır",
            description: "IoPageLockLimit değerini 983040 bayt (960 KB) olarak ayarlayarak doğrudan G/Ç aktarımları için kilitlenebilen bellek miktarını artırır ve NVMe sürücülerde büyük dosya kopyalamalarını hızlandırır.",
            broken_features: &[],
        }),
        "optimize_dns_cache_parameters" => Some(LocalizedTweakText {
            title: "DNS İstemci Önbellek Boyutunu ve TTL Süresini Optimize Et",
            description: "MaxCacheEntryTtlLimit=86400 ve MaxNegativeCacheTtl=5 ayarlayarak başarılı aramaların daha uzun süre önbelleğe alınmasını ve başarısız aramaların hızlıca yeniden denenmesini sağlar.",
            broken_features: &[],
        }),
        "optimize_gpu_preemption_level" => Some(LocalizedTweakText {
            title: "GPU Zamanlayıcısını Yalnızca Hesaplama Kesintisine Ayarla",
            description: "Grafik Sürücüleri Zamanlayıcısında GpuPreemptionLevel=0 ayarlayarak GPU zamanlayıcısının uzun süren gölgelendirici iş yüklerini yarıda kesmesini önler ve oyunlardaki mikro takılmaları azaltır.",
            broken_features: &["GPU Zaman Aşımı Algılama ve Kurtarma (TDR) daha yavaş tetiklenebilir"],
        }),
        "optimize_interrupt_steering" => Some(LocalizedTweakText {
            title: "Kesme Yakınlığını ve Yönlendirme İlkesini Optimize Et",
            description: "InterruptSteeringDisabled=0 ve DistributeTimers=1 ayarlayarak donanım kesmelerini ve zamanlayıcı geri çağrılarını yalnızca CPU 0 yerine tüm çekirdeklere dağıtır ve tek çekirdek darboğazlarını önler.",
            broken_features: &[],
        }),
        "optimize_mmcss_proaudio_scheduling" => Some(LocalizedTweakText {
            title: "MMCSS Pro Audio — Maksimum Zamanlama Önceliği",
            description: "Pro Audio MMCSS görevi Scheduling Category ve SFIO Priority değerlerini 'High' olarak ayarlayarak gerçek zamanlı ses iş parçacıklarının en yüksek önceliği almasını sağlar ve ses takılmalarını önler.",
            broken_features: &[],
        }),
        "optimize_ntfs_memory_usage" => Some(LocalizedTweakText {
            title: "Agresif NTFS Sayfalanan Havuz Önbelleğe Alma",
            description: "NtfsMemoryUsage değerini 2 olarak ayarlayarak NTFS sürücüsünün dizin ve meta veri önbelleğe alma için daha büyük bir sayfalanan havuz ayırmasını sağlar ve dosya numaralandırma performansını artırır.",
            broken_features: &[],
        }),
        "optimize_object_manager_quota" => Some(LocalizedTweakText {
            title: "Çekirdek Nesne Yöneticisi Tanıtıcı Kotasını Artır",
            description: "Memory Management altında SessionPoolSize=48 ayarlayarak tanıtıcı tablosu sınırını artırır ve çok sayıda eşzamanlı işlem çalıştıran sistemlerde tanıtıcı tükenmesini önler.",
            broken_features: &[],
        }),
        "optimize_timer_resolution_requests" => Some(LocalizedTweakText {
            title: "Uygulamaların Yüksek Zamanlayıcı Çözünürlüğü İstemesine İzin Ver",
            description: "Çekirdek anahtarında GlobalTimerResolutionRequests=1 ayarlayarak uygulamaların sistem genelinde maksimum platform zamanlayıcı çözünürlüğünü (0,5 ms) istemesine izin verir ve zamanlama gecikmelerini önler.",
            broken_features: &["Artan zamanlayıcı kesme sıklığı nedeniyle boşta daha yüksek güç tüketimi"],
        }),
        // === gaming_latency_ultra.toml (10) ===
        "disable_driver_search_order_probing" => Some(LocalizedTweakText {
            title: "Windows Update Sürücü Arama Yoklamasını Devre Dışı Bırak",
            description: "Windows Update'in sürücü yazılımı ararken arka planda otomatik olarak sunucuları yoklamasını engeller. Aygıt takıldığında oluşan kurulum gecikmesini ve arka plan ağ sorgularını ortadan kaldırır.",
            broken_features: &["Yeni aygıt bağlandığında Windows Update üzerinden otomatik çevrimiçi sürücü araması çalışmayacaktır"],
        }),
        "disable_fast_user_switching" => Some(LocalizedTweakText {
            title: "Hızlı Kullanıcı Geçişini Devre Dışı Bırak",
            description: "Hızlı Kullanıcı Geçişini devre dışı bırakarak etkin olmayan kullanıcı oturumlarının ve arka plan görevlerinin fiziksel RAM'de yüklü kalmasını önler. Bellek bant genişliğini serbest bırakır ve CPU çakışmasını azaltır.",
            broken_features: &["Birden çok yerel kullanıcı oturumu aynı anda açık kalamaz"],
        }),
        "disable_ip_source_routing" => Some(LocalizedTweakText {
            title: "IP Kaynak Yönlendirmesini Devre Dışı Bırak",
            description: "DisableIPSourceRouting=2 ayarlayarak gelen tüm kaynak yönlendirmeli IP paketlerini atar. Paket sahteciliğini, yol ele geçirmeyi ve ağ yığını işleme yükünü önler.",
            broken_features: &["Kaynak yönlendirmeli ağ tanılama araçları (nadir) reddedilecektir"],
        }),
        "disable_netsh_teredo_tunneling" => Some(LocalizedTweakText {
            title: "Teredo IPv6 Geçiş Tünellemesini Devre Dışı Bırak",
            description: "IPv6 DisabledComponents değerini 32 olarak ayarlayarak Teredo tünelleme sahte arabirimini devre dışı bırakır. IPv6'yı IPv4 UDP paketlerine kapsülleyen Teredo'nun NAT geçiş gecikmesini ortadan kaldırır.",
            broken_features: &["Yalnızca Teredo NAT geçişine dayanan eski Xbox Live eşler arası çok oyunculu oyunlar çalışmayacaktır"],
        }),
        "disable_network_adapter_power_saving" => Some(LocalizedTweakText {
            title: "NDIS Ağ Bağdaştırıcısı PnP Güç Tasarrufunu Devre Dışı Bırak",
            description: "NDIS PnPCapabilities değerini 24 olarak ayarlayarak ağ kartlarının uyku durumlarına geçmesini engeller. Oyun duraklamalarında ağ kartının düşük güç moduna girmesini önleyerek paket gecikmelerini sıfırlar.",
            broken_features: &["Ağ arabirimi Ağdan Uyandırma (Wake-on-LAN) ve uyku güç tasarrufu devre dışı kalacaktır"],
        }),
        "disable_remote_desktop_udp" => Some(LocalizedTweakText {
            title: "Uzak Masaüstü İstemcisi UDP Aktarımını Devre Dışı Bırak",
            description: "Uzak Masaüstü (RDP) istemcisini UDP aktarımı yerine güvenilir TCP çerçevelemesi kullanmaya zorlar. UDP soket arabellek tıkanmasını önler ve RDP trafiğinin oyun paketleriyle yarışmasını engeller.",
            broken_features: &["Yüksek gecikmeli WAN bağlantılarındaki Uzak Masaüstü oturumları UDP yerine TCP kullanacaktır"],
        }),
        "disable_smart_card_service" => Some(LocalizedTweakText {
            title: "Akıllı Kart Hizmetini (SCardSvr) Devre Dışı Bırak",
            description: "Akıllı Kart Hizmetini (Start=4) devre dışı bırakır. Kart okuyucular ve kriptografik belirteçler için yapılan arka plan sorgularını durdurarak oyuncu bilgisayarlarında sistem hizmet yükünü azaltır.",
            broken_features: &["Fiziksel akıllı kart kimlik doğrulaması ve kart okuyucu donanımları çalışmayacaktır"],
        }),
        "optimize_kerberos_ticket_cache" => Some(LocalizedTweakText {
            title: "Kerberos Maksimum Belirteç Boyutu Arabelleğini Optimize Et",
            description: "Kerberos kimlik doğrulama MaxTokenSize arabelleğini 65535 bayta ayarlayarak karmaşık güvenlik gruplarında arabellek taşmalarını ve tekrarlanan bilet yeniden anlaşmalarını önler.",
            broken_features: &[],
        }),
        "optimize_tcp_syn_attack_protection" => Some(LocalizedTweakText {
            title: "TCP SYN Saldırı Korumasını Etkinleştir",
            description: "TCP/IP yığınında SYN taşması savunmasını etkinleştirir (SynAttackProtect=1). Bağlantı taşmalarında el sıkışma zaman aşımlarını hızlandırarak oyun soketlerinin tıkanmasını önler.",
            broken_features: &[],
        }),
        "optimize_usb_selective_suspend_hub" => Some(LocalizedTweakText {
            title: "Genel USB Hub Seçmeli Askıya Almayı Devre Dışı Bırak",
            description: "Kök USB sürücü düzeyinde seçmeli askıya almayı devre dışı bırakır (DisableSelectiveSuspend=1). Fare, klavye ve DAC'ların uykuya geçmesini önleyerek giriş uyandırma gecikmesini ortadan kaldırır.",
            broken_features: &["USB veriyolunda boşta biraz daha yüksek güç tüketimi", "Dizüstü bilgisayarlarda azalan pil ömrü"],
        }),
        "remove_webexperience_package" => Some(LocalizedTweakText {
            title: "Windows WebExperienceHost Paketini (Widget Ana Makinesi) Kaldır",
            description: "Görev çubuğu widget'larının arka planda Edge WebView2 belleği tüketmesini önlemek için MicrosoftWindows.Client.WebExperience paketini kaldırır.",
            broken_features: &["Görev Çubuğu Widget'ları ve haber akışları kalıcı olarak kaldırılır."],
        }),
        "disable_wpbt_execution" => Some(LocalizedTweakText {
            title: "WPBT Bellenim İkili Yürütmesini Devre Dışı Bırak",
            description: "Windows Çekirdeğinin anakart UEFI Windows Platform Binary Table (WPBT) içine gömülü ikilileri yürütmesini engeller.",
            broken_features: &[],
        }),
        "service_shutdown_timeout_15s" => Some(LocalizedTweakText {
            title: "Hizmet Kapanış Zaman Aşımını 15 Saniyeye Düşür",
            description: "Windows kapanışı sırasında uzun süren takılmaları önlemek için hizmet sonlandırma zaman aşımını varsayılan 20 saniyeden 15 saniyeye düşürür.",
            broken_features: &[],
        }),
        "start_menu_show_more_pins" => Some(LocalizedTweakText {
            title: "Başlat Menüsü: Daha Fazla Sabitleme Düzeni",
            description: "Windows 11 Başlat Menüsünü önerilen akış içerikleri yerine sabitlenmiş uygulamalara öncelik verecek şekilde yapılandırır.",
            broken_features: &[],
        }),
        "disable_taskbar_people_button" => Some(LocalizedTweakText {
            title: "Görev Çubuğu Kişiler Düğmesini Devre Dışı Bırak",
            description: "Eski Kişiler / Kişilerim simgesini Windows görev çubuğundan gizler ve devre dışı bırakır.",
            broken_features: &[],
        }),
        "repair_ndu_graph" => Some(LocalizedTweakText {
            title: "Görev Yöneticisi Ağ Veri Kullanımı (NDU) Grafiğini Onar",
            description: "Görev Yöneticisi ağ grafiklerinin düzgün çalışması için Network Diagnostic Usage (NDU) çekirdek sürücüsünün Start=2 (Otomatik) olarak ayarlanmasını sağlar.",
            broken_features: &[],
        }),
        "disable_settings_ads" => Some(LocalizedTweakText {
            title: "Ayarlar Uygulamasındaki Önerilen İçerikleri ve Reklamları Kapat",
            description: "Microsoft'un Windows Ayarları içinde önerilen uygulamaları, iş ortağı tanıtımlarını ve ipuçlarını göstermesini engeller.",
            broken_features: &["Ayarlar uygulamasındaki tanıtım kartları ve ipuçları gizlenir."],
        }),
        "disable_start_menu_suggested_apps" => Some(LocalizedTweakText {
            title: "Önerilen Mağaza Uygulamalarının Otomatik Yüklenmesini Engelle",
            description: "Windows Content Delivery Manager'ın önerilen tüketici uygulamalarını ve oyunlarını sessizce indirmesini ve Başlat menüsüne sabitlemesini önler.",
            broken_features: &["Sponsorlu tüketici oyunları Başlat Menüsüne otomatik olarak yüklenmeyecektir."],
        }),
        "disable_hardware_companion_software" => Some(LocalizedTweakText {
            title: "Donanım Yardımcı Yazılımı Önerilerini Devre Dışı Bırak",
            description: "Çevre birimleri bağlandığında Windows'un üçüncü taraf OEM donanım yardımcı yazılımlarını bildirmesini ve otomatik olarak önermesini engeller.",
            broken_features: &[],
        }),
        "disable_ms_office_telemetry" => Some(LocalizedTweakText {
            title: "Microsoft Office Telemetrisini ve Müşteri Deneyimi Günlüğünü Devre Dışı Bırak",
            description: "Microsoft Office tanılama verisi toplamasını, geri bildirim istemlerini ve Müşteri Deneyimi Geliştirme Programı (CEIP) telemetrisini devre dışı bırakır.",
            broken_features: &[],
        }),
        _ => None,
    }
}

/// Localized tweak title for this language.
#[allow(dead_code)]
pub fn tweak_title(id: &str) -> Option<&'static str> {
    match id {
        "disable_dwm_animations_per_process" => Some("İşlem Başına DWM Geçiş Animasyonlarını Devre Dışı Bırak"),
        "disable_dwm_composition_throttle" => Some("Masaüstü Pencere Yöneticisi Kompozisyon Kısmasını Devre Dışı Bırak"),
        "disable_kernel_mitigations_audit" => Some("Çekirdek Azaltma Önlemleri Denetim Modunu Devre Dışı Bırak"),
        "disable_llmnr_protocol" => Some("LLMNR Ad Çözümleme Protokolünü Devre Dışı Bırak"),
        "disable_memory_compression" => Some("Bellek Sıkıştırma Motorunu Devre Dışı Bırak"),
        "disable_netbios_over_tcp" => Some("TCP/IP Üzerinden NetBIOS'u Devre Dışı Bırak"),
        "disable_ntfs_encryption_driver" => Some("NTFS Dosya Sistemi Şifreleme (EFS) Sürücüsünü Devre Dışı Bırak"),
        "disable_page_file_encryption" => Some("Takas Dosyası Şifrelemesini ve Kapanışta Temizlemeyi Devre Dışı Bırak"),
        "disable_processor_idle_demote_threshold" => Some("İşlemci Boşta Durum İndirgemesini Devre Dışı Bırak"),
        "disable_rss_base_cpu" => Some("Ağ Yük Dağıtımı İçin RSS Temel CPU Ayarla"),
        "disable_spectre_meltdown_mitigations" => Some("Spectre/Meltdown CPU Güvenlik Açığı Önlemlerini Devre Dışı Bırak"),
        "disable_threaded_dpc_optimization" => Some("Daha Düşük Kesme Gecikmesi İçin İş Parçacıklı DPC'yi Devre Dışı Bırak"),
        "disable_windows_search_indexer" => Some("Windows Arama Dizin Oluşturucu Hizmetini Devre Dışı Bırak"),
        "enable_hardware_task_offload" => Some("TCP/IP Donanım Görev Boşaltımını Etkinleştir"),
        "enable_large_ntfs_frs" => Some("Büyük NTFS Dosya Kayıt Segmentlerini Etkinleştir"),
        "enable_ultimate_performance_plan" => Some("Nihai Performans Güç Planını Etkinleştir"),
        "increase_gpu_tdr_timeout" => Some("GPU Zaman Aşımı Algılama ve Kurtarma Gecikmesini Artır"),
        "increase_irp_stack_size" => Some("Ağ/Disk G/Ç İçin IRP Yığın Boyutunu Artır"),
        "increase_max_connections_per_server" => Some("Sunucu Başına Maksimum HTTP Bağlantı Sayısını Artır"),
        "increase_nonpaged_pool_quota" => Some("Sürücü Kararlılığı İçin Sayfalanmayan Havuz Kotasını Artır"),
        "optimize_additional_critical_worker_threads" => Some("Ek Çekirdek Çalışma İş Parçacıklarını Artır"),
        "optimize_default_ttl" => Some("En Uygun Varsayılan IP Paketi TTL Değerini Ayarla"),
        "optimize_disk_io_page_lock_limit" => Some("Büyük Aktarımlar İçin G/Ç Sayfa Kilitleme Sınırını Artır"),
        "optimize_dns_cache_parameters" => Some("DNS İstemci Önbellek Boyutunu ve TTL Süresini Optimize Et"),
        "optimize_gpu_preemption_level" => Some("GPU Zamanlayıcısını Yalnızca Hesaplama Kesintisine Ayarla"),
        "optimize_interrupt_steering" => Some("Kesme Yakınlığını ve Yönlendirme İlkesini Optimize Et"),
        "optimize_mmcss_proaudio_scheduling" => Some("MMCSS Pro Audio — Maksimum Zamanlama Önceliği"),
        "optimize_ntfs_memory_usage" => Some("Agresif NTFS Sayfalanan Havuz Önbelleğe Alma"),
        "optimize_object_manager_quota" => Some("Çekirdek Nesne Yöneticisi Tanıtıcı Kotasını Artır"),
        "optimize_timer_resolution_requests" => Some("Uygulamaların Yüksek Zamanlayıcı Çözünürlüğü İstemesine İzin Ver"),
                "disable_driver_search_order_probing" => Some("Windows Update Sürücü Arama Yoklamasını Devre Dışı Bırak"),
        "disable_fast_user_switching" => Some("Hızlı Kullanıcı Geçişini Devre Dışı Bırak"),
        "disable_ip_source_routing" => Some("IP Kaynak Yönlendirmesini Devre Dışı Bırak"),
        "disable_netsh_teredo_tunneling" => Some("Teredo IPv6 Geçiş Tünellemesini Devre Dışı Bırak"),
        "disable_network_adapter_power_saving" => Some("NDIS Ağ Bağdaştırıcısı PnP Güç Tasarrufunu Devre Dışı Bırak"),
        "disable_remote_desktop_udp" => Some("Uzak Masaüstü İstemcisi UDP Aktarımını Devre Dışı Bırak"),
        "disable_smart_card_service" => Some("Akıllı Kart Hizmetini (SCardSvr) Devre Dışı Bırak"),
        "optimize_kerberos_ticket_cache" => Some("Kerberos Maksimum Belirteç Boyutu Arabelleğini Optimize Et"),
        "optimize_tcp_syn_attack_protection" => Some("TCP SYN Saldırı Korumasını Etkinleştir"),
        "optimize_usb_selective_suspend_hub" => Some("Genel USB Hub Seçmeli Askıya Almayı Devre Dışı Bırak"),
        _ => lookup(id).map(|t| t.title),
    }
}

/// Localized tweak description for this language.
#[allow(dead_code)]
pub fn tweak_description(id: &str) -> Option<&'static str> {
    match id {
        "disable_dwm_animations_per_process" => Some("DWM anahtarı altında DisallowAnimations=1 ve AnimationsShiftKey=1 değerlerini ayarlayarak işlem başına DWM geçiş animasyonlarını devre dışı bırakır. Pencereler küçültüldüğünde, büyütüldüğünde veya değiştirildiğinde solma ve kayma animasyonlarını kaldırarak GPU bindirmesini azaltır."),
        "disable_dwm_composition_throttle" => Some("DWM anahtarı altında DesktopCompositionThrottle=0 değerini ayarlayarak kare atlandığında kompozitörün işlemeyi yavaşlatmasını önler. GPU yükü altındayken bile DWM'nin maksimum kompozisyon verimini korumasını sağlar."),
        "disable_kernel_mitigations_audit" => Some("MitigationAuditOptions değerini tamamen sıfırlayarak çekirdek istismar azaltma denetim günlüğünü devre dışı bırakır. Süreç oluşturma ve özel durum işlemede ETW günlüğü yükünü azaltır."),
        "disable_llmnr_protocol" => Some("DNS istemci parametrelerinde EnableMulticast=0 ayarlayarak Bağlantı-Yerel Çok Noktaya Yayın Ad Çözümlemesini (LLMNR) devre dışı bırakır. Ağ üzerindeki gereksiz çok noktaya yayın trafiğini ve LLMNR zehirleme saldırı risklerini önler."),
        "disable_memory_compression" => Some("DisableCompression=1 ayarlayarak Windows Bellek Yöneticisi sıkıştırma deposunu devre dışı bırakır. 16 GB ve üzeri RAM bulunan sistemlerde Sistem işlemi bellek sıkıştırma iş parçacığından kaynaklanan CPU kullanım artışlarını önler."),
        "disable_netbios_over_tcp" => Some("NetbiosOptions=2 ayarlayarak ve LMHosts aramasını kapatıp TCP/IP üzerinden NetBIOS protokolünü devre dışı bırakır. NetBIOS, ad çözümleme sorguları yayınlayan ve sistemi gereksiz ağ gürültüsüne ve olası NBNS zehirleme saldırılarına maruz bırakan eski bir LAN protokolüdür."),
        "disable_ntfs_encryption_driver" => Some("FileSystem anahtarı altında NtfsDisableEncryption=1 ayarlayarak NTFS Şifreleme Dosya Sistemi filtre sürücüsünün dosya G/Ç işlemlerini kesmesini önler. EFS şifrelemesi kullanmayan sistemlerde dosya erişim gecikmesini azaltır."),
        "disable_page_file_encryption" => Some("Windows'un takas dosyası içeriklerini şifrelemesini önler ve kapanışta takas dosyasının temizlenmesini devre dışı bırakır (ClearPageFileAtShutdown=0, DisablePagingEncryption=1). Bellek sayfalama verimini artırır ve kapanış süresini kısaltır."),
        "disable_processor_idle_demote_threshold" => Some("İşlemci güç yönetimi parametrelerinde IdleDemoteThreshold=100 ve IdlePromoteThreshold=100 ayarlayarak kısa boşta kalma sürelerinde çekirdek güç yöneticisinin işlemci performans durumlarını düşürmesini önler. Oyunlarda ve gerçek zamanlı işlemlerde frekans artırma gecikmelerini ortadan kaldırır."),
        "disable_rss_base_cpu" => Some("NDIS parametrelerinde RssBaseCpu=1 ayarlayarak Alma Tarafı Ölçeklendirmesinin (RSS) ağ kesme işlemlerini CPU 0 yerine CPU 1'den dağıtmaya başlamasını sağlar. CPU 0'ı ön plan uygulamaları ve oyunlar için serbest bırakır."),
        "disable_spectre_meltdown_mitigations" => Some("FeatureSettingsOverride=3 ve FeatureSettingsOverrideMask=3 ayarlayarak donanım düzeyinde Spectre ve Meltdown CPU açık önlemlerini devre dışı bırakır. Ağır hesaplama işlerinde CPU verimini en üst seviyeye çıkarır."),
        "disable_threaded_dpc_optimization" => Some("Çekirdek anahtarında ThreadDpcEnable=0 ayarlayarak Ertelenmiş Yordam Çağrılarının (DPC) ayrılmış bir iş parçacığı yerine doğrudan DISPATCH_LEVEL seviyesinde yürütülmesini zorlar. Gerçek zamanlı ses üretimi ve rekabetçi oyunlar için DPC gecikmesini en aza indirir."),
        "disable_windows_search_indexer" => Some("WSearch hizmetini devre dışı bırakarak arka plandaki dizin oluşturucunun tüm sürücülerdeki dosya içeriklerini taramasını durdurur. SSD/NVMe sistemlerde SearchIndexer.exe kaynaklı disk G/Ç, CPU ve bellek tüketimini ortadan kaldırır."),
        "enable_hardware_task_offload" => Some("TCP/IP parametrelerinde DisableTaskOffload=0 ayarlayarak işletim sisteminin sağlama toplamı hesaplamasını ve TCP bölümlemesini ağ kartı donanımına aktarmasını sağlar. CPU döngülerini serbest bırakır."),
        "enable_large_ntfs_frs" => Some("NtfsAllowExtendedCharacterIn8dot3Name=0 ayarlayarak 8.3 kısa adlarındaki genişletilmiş karakter desteğini devre dışı bırakır ve NTFS sürücüsünün daha büyük, verimli Dosya Kayıt Segmentleri ayırmasını sağlar."),
        "enable_ultimate_performance_plan" => Some("Gizli Nihai Performans güç düzenini etkinleştirir. Çekirdek park etme yok, frekans ölçekleme gecikmeleri yok, USB seçmeli askıya alma yok ve disk durdurma yok. En düşük gecikme süresini sağlar."),
        "increase_gpu_tdr_timeout" => Some("TdrDelay değerini 2 saniyeden 10 saniyeye yükselterek, Windows ekran sürücüsünü sıfırlamadan önce GPU'ya uzun süren gölgelendirici hesaplamalarını tamamlaması için daha fazla süre tanır."),
        "increase_irp_stack_size" => Some("G/Ç İstek Paketi yığın boyutunu 15'ten 32'ye yükselterek SMB sunucusu ve katmanlı filtre sürücülerine daha fazla yığın alanı sağlar. STATUS_INSUFFICIENT_RESOURCES hatalarını önler ve G/Ç verimini artırır."),
        "increase_max_connections_per_server" => Some("MaxConnectionsPerServer ve MaxConnectionsPer1_0Server değerlerini 16'ya yükselterek WinINet tabanlı uygulamaların aynı ana bilgisayara daha fazla eşzamanlı HTTP bağlantısı açmasını sağlar ve indirmeleri hızlandırır."),
        "increase_nonpaged_pool_quota" => Some("NonPagedPoolQuota değerini 0 (sistem tarafından yönetilen maksimum) olarak ayarlayarak çekirdek bellek yöneticisinin sayfalanmayan tahsisler için tüm fiziksel belleği kullanmasına izin verir."),
        "optimize_additional_critical_worker_threads" => Some("AdditionalCriticalWorkerThreads ve AdditionalDelayedWorkerThreads değerlerini 2 olarak ayarlayarak kritik ve gecikmeli iş öğelerini işlemek için ek çekirdek modu iş parçacıkları sağlar ve kuyruk duraksamalarını önler."),
        "optimize_default_ttl" => Some("Varsayılan IP Yaşam Süresi (TTL) değerini Linux/macOS standardıyla eşleşecek şekilde 64 atlama olarak ayarlar. 64 değeri neredeyse tüm internet yönlendirme topolojileri için idealdir."),
        "optimize_disk_io_page_lock_limit" => Some("IoPageLockLimit değerini 983040 bayt (960 KB) olarak ayarlayarak doğrudan G/Ç aktarımları için kilitlenebilen bellek miktarını artırır ve NVMe sürücülerde büyük dosya kopyalamalarını hızlandırır."),
        "optimize_dns_cache_parameters" => Some("MaxCacheEntryTtlLimit=86400 ve MaxNegativeCacheTtl=5 ayarlayarak başarılı aramaların daha uzun süre önbelleğe alınmasını ve başarısız aramaların hızlıca yeniden denenmesini sağlar."),
        "optimize_gpu_preemption_level" => Some("Grafik Sürücüleri Zamanlayıcısında GpuPreemptionLevel=0 ayarlayarak GPU zamanlayıcısının uzun süren gölgelendirici iş yüklerini yarıda kesmesini önler ve oyunlardaki mikro takılmaları azaltır."),
        "optimize_interrupt_steering" => Some("InterruptSteeringDisabled=0 ve DistributeTimers=1 ayarlayarak donanım kesmelerini ve zamanlayıcı geri çağrılarını yalnızca CPU 0 yerine tüm çekirdeklere dağıtır ve tek çekirdek darboğazlarını önler."),
        "optimize_mmcss_proaudio_scheduling" => Some("Pro Audio MMCSS görevi Scheduling Category ve SFIO Priority değerlerini 'High' olarak ayarlayarak gerçek zamanlı ses iş parçacıklarının en yüksek önceliği almasını sağlar ve ses takılmalarını önler."),
        "optimize_ntfs_memory_usage" => Some("NtfsMemoryUsage değerini 2 olarak ayarlayarak NTFS sürücüsünün dizin ve meta veri önbelleğe alma için daha büyük bir sayfalanan havuz ayırmasını sağlar ve dosya numaralandırma performansını artırır."),
        "optimize_object_manager_quota" => Some("Memory Management altında SessionPoolSize=48 ayarlayarak tanıtıcı tablosu sınırını artırır ve çok sayıda eşzamanlı işlem çalıştıran sistemlerde tanıtıcı tükenmesini önler."),
        "optimize_timer_resolution_requests" => Some("Çekirdek anahtarında GlobalTimerResolutionRequests=1 ayarlayarak uygulamaların sistem genelinde maksimum platform zamanlayıcı çözünürlüğünü (0,5 ms) istemesine izin verir ve zamanlama gecikmelerini önler."),
                "disable_driver_search_order_probing" => Some("Windows Update'in sürücü yazılımı ararken arka planda otomatik olarak sunucuları yoklamasını engeller. Aygıt takıldığında oluşan kurulum gecikmesini ve arka plan ağ sorgularını ortadan kaldırır."),
        "disable_fast_user_switching" => Some("Hızlı Kullanıcı Geçişini devre dışı bırakarak etkin olmayan kullanıcı oturumlarının ve arka plan görevlerinin fiziksel RAM'de yüklü kalmasını önler. Bellek bant genişliğini serbest bırakır ve CPU çakışmasını azaltır."),
        "disable_ip_source_routing" => Some("DisableIPSourceRouting=2 ayarlayarak gelen tüm kaynak yönlendirmeli IP paketlerini atar. Paket sahteciliğini, yol ele geçirmeyi ve ağ yığını işleme yükünü önler."),
        "disable_netsh_teredo_tunneling" => Some("IPv6 DisabledComponents değerini 32 olarak ayarlayarak Teredo tünelleme sahte arabirimini devre dışı bırakır. IPv6'yı IPv4 UDP paketlerine kapsülleyen Teredo'nun NAT geçiş gecikmesini ortadan kaldırır."),
        "disable_network_adapter_power_saving" => Some("NDIS PnPCapabilities değerini 24 olarak ayarlayarak ağ kartlarının uyku durumlarına geçmesini engeller. Oyun duraklamalarında ağ kartının düşük güç moduna girmesini önleyerek paket gecikmelerini sıfırlar."),
        "disable_remote_desktop_udp" => Some("Uzak Masaüstü (RDP) istemcisini UDP aktarımı yerine güvenilir TCP çerçevelemesi kullanmaya zorlar. UDP soket arabellek tıkanmasını önler ve RDP trafiğinin oyun paketleriyle yarışmasını engeller."),
        "disable_smart_card_service" => Some("Akıllı Kart Hizmetini (Start=4) devre dışı bırakır. Kart okuyucular ve kriptografik belirteçler için yapılan arka plan sorgularını durdurarak oyuncu bilgisayarlarında sistem hizmet yükünü azaltır."),
        "optimize_kerberos_ticket_cache" => Some("Kerberos kimlik doğrulama MaxTokenSize arabelleğini 65535 bayta ayarlayarak karmaşık güvenlik gruplarında arabellek taşmalarını ve tekrarlanan bilet yeniden anlaşmalarını önler."),
        "optimize_tcp_syn_attack_protection" => Some("TCP/IP yığınında SYN taşması savunmasını etkinleştirir (SynAttackProtect=1). Bağlantı taşmalarında el sıkışma zaman aşımlarını hızlandırarak oyun soketlerinin tıkanmasını önler."),
        "optimize_usb_selective_suspend_hub" => Some("Kök USB sürücü düzeyinde seçmeli askıya almayı devre dışı bırakır (DisableSelectiveSuspend=1). Fare, klavye ve DAC'ların uykuya geçmesini önleyerek giriş uyandırma gecikmesini ortadan kaldırır."),
        _ => lookup(id).map(|t| t.description),
    }
}
