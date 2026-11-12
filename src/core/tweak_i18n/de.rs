use super::LocalizedTweakText;

pub fn lookup(id: &str) -> Option<LocalizedTweakText> {
    match id {
        // === ai_privacy_ext.toml (21) ===
        "disable_windows_ai_recall_extended" => Some(LocalizedTweakText {
            title: "Windows Recall, Click to Do & Copilot+ KI-Agenten deaktivieren",
            description: "Blockiert die Snapshot-Erfassung von Windows Recall, Click to Do, den Einstellungs-KI-Agenten, KI-Agent-Konnektoren/Arbeitsbereiche sowie die Copilot Runtime über Gruppenrichtlinien-Schlüssel ab Windows 11 24H2+.",
            broken_features: &["Windows Recall Zeitleisten-Snapshots & Click to Do KI-Overlay"],
        }),
        "disable_generative_ai_app_access" => Some(LocalizedTweakText {
            title: "App-Zugriff auf generative & System-KI-Modelle blockieren",
            description: "Verweigert UWP-/WinUI-Hintergrund-Apps den Zugriff auf lokale generative KI- und System-KI-Modellfunktionen über AppPrivacy- und CapabilityAccessManager-ConsentStore-Richtlinien.",
            broken_features: &["Drittanbieter-Apps können keine integrierten Windows-KI-Modellendpunkte abfragen"],
        }),
        "disable_copilot_shell_and_hardware_key" => Some(LocalizedTweakText {
            title: "Copilot-Taskleistenschaltfläche, Hinweise, Hardware-Taste & Mikrofon deaktivieren",
            description: "Blendet die Windows-Copilot-Schaltfläche und Hinweise in der Taskleiste aus, neutralisiert die dedizierte Copilot-Tastaturtaste, blockiert kontobezogene KI-Werbung und verweigert Copilot & OfficeHub den Mikrofonzugriff.",
            broken_features: &[
                "Dedizierte Copilot-Taste startet Copilot nicht",
                "Copilot-Spracheingabe blockiert",
            ],
        }),
        "disable_paint_ai_features" => Some(LocalizedTweakText {
            title: "Cloud- & generative KI-Funktionen in Microsoft Paint deaktivieren",
            description: "Deaktiviert Image Creator, Cocreator, Generatives Füllen, Generatives Löschen und KI-Hintergrundentfernung in Windows 11 Paint über offizielle Richtlinienschlüssel.",
            broken_features: &["KI-Erzeugungswerkzeuge in Microsoft Paint ausgeblendet"],
        }),
        "disable_edge_ai_policies" => Some(LocalizedTweakText {
            title: "Microsoft Edge Copilot, Seitenleiste & integrierte KI-APIs deaktivieren",
            description: "Deaktiviert 12 KI-Richtlinien in Microsoft Edge, darunter Copilot-Seitenkontext, Hubs-Seitenleiste, Entra Copilot, Inline-Verfassen, KI-Verlaufssuche, lokale Basismodelle, DevTools GenAI und Verlaufsfreigabe.",
            broken_features: &["Edge-Copilot-Seitenleiste und Inline-Schreibhilfe deaktiviert"],
        }),
        "disable_office_ai_copilot" => Some(LocalizedTweakText {
            title: "Microsoft Office KI-Training & App-Copilots deaktivieren",
            description: "Deaktiviert Microsoft 365 KI-Training, Analyse verbundener Inhalte, Content Safety Service sowie App-spezifische Copilot-Schalter in Word, Excel und OneNote.",
            broken_features: &["Copilot-Assistent in Word, Excel und OneNote deaktiviert"],
        }),
        "disable_wmi_telemetry_autologgers" => Some(LocalizedTweakText {
            title: "10 WMI-Kernel-Telemetrie-AutoLogger-Sitzungen deaktivieren",
            description: "Verhindert, dass Windows persistente ETW-Hintergrund-AutoLogger-Ablaufverfolgungssitzungen startet (Diagtrack-Listener, SQMLogger, DiagLog, Cellcore, DataMarket, AppModel, CloudExperienceHostOobe, LwtNetLog, WdiContextLog, WiFiSession).",
            broken_features: &["Kernel-ETW-Diagnoseablaufverfolgungsprotokolle deaktiviert"],
        }),
        "block_silent_preinstalled_apps" => Some(LocalizedTweakText {
            title: "Stille App-Vorinstallationen sowie Start- & Einstellungsvorschläge blockieren",
            description: "Verhindert, dass der Windows ContentDeliveryManager stillschweigend OEM- oder Microsoft-Store-Werbe-Apps installiert, und blendet empfohlene Inhalte in Einstellungen, Startmenü und Zeitachse aus.",
            broken_features: &[],
        }),
        "disable_extra_telemetry_policies" => Some(LocalizedTweakText {
            title: "Feedback, AppCompat-Inventar, OOBE-Hinweise & Tracking absichern",
            description: "Begrenzt MaxTelemetryAllowed=0, unterdrückt Windows-Feedback-SIUF-Abfragen (löscht PeriodInNanoSeconds), deaktiviert Anwendungskompatibilitäts-Inventar/UAR, Mein Gerät suchen, Willkommens-/Einrichtungshinweise, App-Startverfolgung, Workplace Join-Abfragen und Remoteunterstützung.",
            broken_features: &[
                "Einladungen zur Remoteunterstützung deaktiviert",
                "Verfolgung häufiger/zuletzt gestarteter Apps im Startmenü ausgeschaltet",
            ],
        }),
        "disable_typing_and_inking_telemetry" => Some(LocalizedTweakText {
            title: "Freihand-, Eingabe-Personalisierung & Eingabe-Einblicke deaktivieren",
            description: "Beschränkt implizite Tastenanschlag-/Freihanddatenerfassung, Kontakterfassung für benutzerdefinierte Eingabewörterbücher, linguistische Diagnoseprotokollierung (TIPC) sowie Windows-Eingabe-Einblicke.",
            broken_features: &["Personalisiertes Eingabe- & Handschriftvorhersage-Wörterbuch deaktiviert"],
        }),
        "disable_search_cloud_and_history" => Some(LocalizedTweakText {
            title: "Windows-Such-Highlights, Cloud-Suche & Geräteverlauf deaktivieren",
            description: "Schaltet Windows-Such-Highlights (dynamisches Suchfeld), MSA/AAD-Cloud-Inhaltssuche und lokale Gerätesuchverlaufs-Protokollierung ab, ohne den Indexdienst zu deaktivieren.",
            broken_features: &["Tägliche Such-Highlights-Grafiken & OneDrive-Cloud-Ergebnisse ausgeblendet"],
        }),
        "disable_wifi_sense_and_location_extras" => Some(LocalizedTweakText {
            title: "WLAN-Optimierung (WiFi-Sense), Karten-Auto-Update & Standort-ConsentStore deaktivieren",
            description: "Deaktiviert automatische WLAN-Hotspot-Verbindungen (WcmSvc\\AutoConnectAllowedOEM=0) sowie automatische Offline-Karten-Updates und die globale Standortberechtigung im ConsentStore.",
            broken_features: &["Automatische Windows-Karten-Downloads und systemweite App-Standortabfragen"],
        }),
        "block_onedrive_kfm_auto_backup" => Some(LocalizedTweakText {
            title: "Automatische OneDrive-Ordnersicherung (Desktop/Dokumente) blockieren",
            description: "Setzt KFMBlockOptIn=1, damit OneDrive Ordner wie Desktop, Dokumente und Bilder nicht stillschweigend in die Cloud-Sicherung übernimmt. Sicher für rein lokale Speicherung oder manuelle Synchronisierung.",
            broken_features: &["Automatischer OneDrive-Assistent zum Verschieben bekannter Ordner deaktiviert"],
        }),
        "disable_bitlocker_auto_encryption" => Some(LocalizedTweakText {
            title: "Automatische Windows BitLocker-Geräteverschlüsselung deaktivieren",
            description: "Setzt PreventDeviceEncryption=1, damit Windows Laufwerke bei Anmeldung mit einem Microsoft-Konto oder bei Neueinrichtungen nicht automatisch ohne vorherige Sicherung der Wiederherstellungsschlüssel verschlüsselt.",
            broken_features: &["Automatische BitLocker-Laufwerksverschlüsselung ohne Rückfrage wird nicht aktiviert"],
        }),
        "disable_extra_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Zusätzliche Telemetrie-, WER-Warteschlangen-, SQM-, Energie- & Kartenaufgaben deaktivieren",
            description: "Deaktiviert weitere Diagnose- und Hintergrundaufgaben: QueueReporting, Sqm-Tasks, MareBackup, StartupAppTask, PcaPatchDbTask, MapsUpdateTask, AnalyzeSystem und FamilySafetyMonitor.",
            broken_features: &["Automatisierter Energieeffizienzbericht & Offline-Kartenupdates deaktiviert"],
        }),
        "disable_windows_ai_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Geplante Hintergrundaufgaben für Windows AI Recall deaktivieren",
            description: "Deaktiviert die ab Windows 11 24H2+ eingeführten geplanten Aufgaben für die Windows Recall-Konfiguration und ML-Hintergrundverarbeitung.",
            broken_features: &["Windows Recall-Hintergrundindizierungsaufgaben deaktiviert"],
        }),
        "disable_office_actions_scheduled_task" => Some(LocalizedTweakText {
            title: "Geplante Aufgabe für Microsoft Office KI-Aktionen deaktivieren",
            description: "Deaktiviert die geplante Hintergrundaufgabe Microsoft Office Actions Server, um CPU- und Speicheraufwachvorgänge durch Office-KI zu reduzieren.",
            broken_features: &[],
        }),
        "disable_optional_bloat_services" => Some(LocalizedTweakText {
            title: "Ungenutzte optionale Windows-Dienste deaktivieren (Karten, Fax, Demo, Wallet, tzautoupdate)",
            description: "Deaktiviert MapsBroker, RetailDemo, Fax, WalletService, WMPNetworkSvc, AJRouter, shpamsvc und tzautoupdate, ohne zentrale OS- oder Windows-Update-Dienste anzutasten.",
            broken_features: &["Windows-Fax, Offline-Karten-Broker, Einzelhandelsdemo, WMP-Freigabe & automatische Zeitzone deaktiviert"],
        }),
        "disable_xbox_live_services" => Some(LocalizedTweakText {
            title: "Xbox Live-Hintergrunddienste deaktivieren (XblAuthManager, GameSave, Networking)",
            description: "Deaktiviert die Hintergrunddienste Xbox Live Auth Manager, Game Save und Networking für Nutzer, die keine Xbox-App- oder Game-Pass-Titel spielen.",
            broken_features: &["Xbox Game Pass & Microsoft Store Xbox Live-Anmeldung / Cloud-Spielstände funktionieren nicht mehr"],
        }),
        "disable_ai_fabric_service" => Some(LocalizedTweakText {
            title: "Windows AI Fabric-Dienst deaktivieren (WSAIFabricSvc)",
            description: "Deaktiviert den lokalen Hintergrunddienst zur KI-Orchestrierung WSAIFabricSvc unter Windows 11 24H2+.",
            broken_features: &["Copilot+ KI-Orchestrierungsdienst gestoppt"],
        }),
        "deny_global_camera_mic_diagnostics" => Some(LocalizedTweakText {
            title: "Globalen App-Zugriff auf Kamera, Mikrofon, Kontoinfos & Diagnose sperren [Opt-In]",
            description: "Verweigert global ConsentStore-Berechtigungen für Webcam, Mikrofon, userAccountInformation und appDiagnostics für alle Desktop- und UWP-Apps. Hohes Störungsrisiko: deaktiviert Kameras & Mikrofone in Zoom/Discord/OBS bis zur Rücknahme.",
            broken_features: &["Deaktiviert Kamera und Mikrofon in ALLEN Anwendungen bis zur erneuten Freigabe"],
        }),

        // === bloatware_appx.toml (26) ===
        "remove_bing_weather" => Some(LocalizedTweakText {
            title: "Bing Wetter entfernen",
            description: "Entfernt die Microsoft Bing Wetter UWP-App. Wetterdaten können stattdessen über den Browser abgerufen werden.",
            broken_features: &["Die Bing Wetter-App wird entfernt"],
        }),
        "remove_bing_news" => Some(LocalizedTweakText {
            title: "Bing Nachrichten entfernen",
            description: "Entfernt die Microsoft Bing Nachrichten (Microsoft News) UWP-App.",
            broken_features: &["Die Bing Nachrichten / Microsoft News-App wird entfernt"],
        }),
        "remove_get_help" => Some(LocalizedTweakText {
            title: "Hilfe anfordern entfernen",
            description: "Entfernt die Microsoft Hilfe anfordern (Get Help) UWP-App. Support bleibt weiterhin über das Web erreichbar.",
            broken_features: &["Die Hilfe anfordern-App wird entfernt"],
        }),
        "remove_getstarted_tips" => Some(LocalizedTweakText {
            title: "Tipps (Erste Schritte) entfernen",
            description: "Entfernt die Microsoft Tipps / Erste Schritte UWP-App.",
            broken_features: &["Die Tipps / Erste Schritte-App wird entfernt"],
        }),
        "remove_solitaire" => Some(LocalizedTweakText {
            title: "Microsoft Solitaire Collection entfernen",
            description: "Entfernt die Microsoft Solitaire Collection UWP-App, die Werbung und Telemetrie enthält.",
            broken_features: &["Solitaire Collection wird entfernt"],
        }),
        "remove_people" => Some(LocalizedTweakText {
            title: "Kontakte-App (People) entfernen",
            description: "Entfernt die Microsoft Kontakte (People) UWP-App.",
            broken_features: &["Die Kontakte-App wird entfernt"],
        }),
        "remove_windows_maps" => Some(LocalizedTweakText {
            title: "Windows-Karten entfernen",
            description: "Entfernt die Microsoft Windows-Karten UWP-App.",
            broken_features: &["Die Windows-Karten-App wird entfernt"],
        }),
        "remove_groove_music" => Some(LocalizedTweakText {
            title: "Groove-Musik (Zune Music) entfernen",
            description: "Entfernt die Groove-Musik (ZuneMusic) UWP-App. Stattdessen können der Media Player oder Drittanbieter-Player genutzt werden.",
            broken_features: &["Die Groove-Musik-App wird entfernt"],
        }),
        "remove_zune_video" => Some(LocalizedTweakText {
            title: "Filme & TV (Zune Video) entfernen",
            description: "Entfernt die Filme & TV (ZuneVideo) UWP-App. Stattdessen können VLC oder andere Player verwendet werden.",
            broken_features: &["Die Filme & TV-App wird entfernt"],
        }),
        "remove_clipchamp" => Some(LocalizedTweakText {
            title: "Clipchamp entfernen",
            description: "Entfernt den unter Windows 11 vorinstallierten Video-Editor Clipchamp (UWP-App).",
            broken_features: &["Der Clipchamp Video-Editor wird entfernt"],
        }),
        "remove_todos" => Some(LocalizedTweakText {
            title: "Microsoft To Do entfernen",
            description: "Entfernt die Microsoft To Do UWP-App.",
            broken_features: &["Die Microsoft To Do-App wird entfernt"],
        }),
        "remove_bing_search_appx" => Some(LocalizedTweakText {
            title: "Microsoft Bing Suche AppX-Erweiterung entfernen",
            description: "Entfernt das Store-Paket Microsoft.BingSearch, das Web-Widgets und Webintegrationen der Windows-Suche bereitstellt.",
            broken_features: &["Bing-Suche AppX-Integration entfernt"],
        }),
        "remove_feedback_hub" => Some(LocalizedTweakText {
            title: "Microsoft Feedback-Hub entfernen",
            description: "Entfernt die vorinstallierte Telemetrie- und Umfrage-App Windows Feedback-Hub.",
            broken_features: &["Feedback-Hub-App entfernt"],
        }),
        "remove_copilot_appx" => Some(LocalizedTweakText {
            title: "Microsoft Copilot UWP-Paket entfernen",
            description: "Deinstalliert das eigenständige Microsoft.Copilot-Store-Paket.",
            broken_features: &["Copilot-App entfernt"],
        }),
        "remove_edge_game_assist" => Some(LocalizedTweakText {
            title: "Microsoft Edge Game Assist-Overlay entfernen",
            description: "Entfernt das AppX-Paket des Edge Game Assist In-Game-Browser-Overlays.",
            broken_features: &["Edge Game Assist-Widget entfernt"],
        }),
        "remove_dev_home" => Some(LocalizedTweakText {
            title: "Windows Dev Home-App entfernen",
            description: "Entfernt das unter Windows 11 vorinstallierte Dashboard-Paket Microsoft.Windows.DevHome.",
            broken_features: &["Dev Home-Dashboard entfernt"],
        }),
        "remove_power_automate_desktop" => Some(LocalizedTweakText {
            title: "Microsoft Power Automate Desktop entfernen",
            description: "Entfernt das RPA-Paket Microsoft Power Automate Desktop.",
            broken_features: &["Power Automate Desktop-Flows-App entfernt"],
        }),
        "remove_office_hub" => Some(LocalizedTweakText {
            title: "Microsoft 365 Copilot (OfficeHub) entfernen",
            description: "Entfernt die vorinstallierte Microsoft 365 Office Hub Werbe-Starter-App.",
            broken_features: &["Office-Werbestarter entfernt (deinstalliert nicht Desktop-Office)"],
        }),
        "remove_family_safety" => Some(LocalizedTweakText {
            title: "Microsoft Family Safety-App entfernen",
            description: "Entfernt die Microsoft Family Safety UWP-App für Endverbraucher.",
            broken_features: &["Family Safety UWP-App entfernt"],
        }),
        "remove_3d_viewer_and_mixed_reality" => Some(LocalizedTweakText {
            title: "3D-Viewer & Mixed Reality-Portal entfernen",
            description: "Entfernt veraltete Pakete von Microsoft 3D-Viewer und Mixed Reality-Portal.",
            broken_features: &["Veralteter 3D-Viewer & Windows Mixed Reality-Portal entfernt"],
        }),
        "remove_outlook_for_windows" => Some(LocalizedTweakText {
            title: "Neues Outlook für Windows entfernen (WebView-Client)",
            description: "Entfernt die vorinstallierte Web-Wrapper-App Microsoft.OutlookForWindows.",
            broken_features: &["Neuer Outlook UWP-Client entfernt"],
        }),
        "remove_your_phone_link" => Some(LocalizedTweakText {
            title: "Smartphone-Link (YourPhone) entfernen",
            description: "Deinstalliert die Begleit-App Microsoft.YourPhone zur Mobilgerätesynchronisierung.",
            broken_features: &["Android/iOS Smartphone-Link Desktop-Synchronisierung entfernt"],
        }),
        "remove_sound_recorder_and_sticky_notes" => Some(LocalizedTweakText {
            title: "Windows Audiorekorder & Kurznotizen entfernen",
            description: "Entfernt die Store-Pakete für Audiorekorder und Kurznotizen (Sticky Notes).",
            broken_features: &["Integrierte Apps Audiorekorder & Kurznotizen entfernt"],
        }),
        "remove_ms_teams_and_quick_assist" => Some(LocalizedTweakText {
            title: "Microsoft Teams (Privat) & Remotehilfe entfernen",
            description: "Entfernt die private Chat-App MSTeams und das Support-Werkzeug Remotehilfe (Quick Assist).",
            broken_features: &["Privates Teams & Remotehilfe-Supportwerkzeug entfernt"],
        }),
        "disable_recall_optional_feature" => Some(LocalizedTweakText {
            title: "Optionales Windows Recall-Feature deinstallieren (DISM)",
            description: "Deaktiviert das optionale Windows-Feature-Paket 'Recall' sauber auf Betriebssystem-Wartungsebene.",
            broken_features: &["Windows Recall-Feature-Binärdateien sauber deaktiviert"],
        }),
        "remove_legacy_steps_recorder_ie_wordpad" => Some(LocalizedTweakText {
            title: "Veraltete Schrittaufzeichnung & Internet Explorer 11-Funktion deaktivieren",
            description: "Entfernt die veraltete Schrittaufzeichnung (App.StepsRecorder) und die Internet Explorer-Modus-Funktion über DISM.",
            broken_features: &["Schrittaufzeichnung (psr.exe) und IE11-Modus entfernt"],
        }),

        // === hardware_tuning.toml (16) ===
        "enable_hags" => Some(LocalizedTweakText {
            title: "Hardwarebeschleunigte GPU-Planung (HAGS) aktivieren",
            description: "Aktiviert die hardwarebeschleunigte GPU-Planung, sodass die GPU ihre VRAM-Zuweisung direkt verwaltet. Reduziert Latenz und CPU-Last bei GPU-limitierten Workloads. Erfordert eine kompatible GPU (NVIDIA GTX 1000+, AMD RX 5000+) und Treiberunterstützung.",
            broken_features: &["Einige ältere Spiele oder GPU-Tools können Instabilitäten aufweisen"],
        }),
        "disable_mpo" => Some(LocalizedTweakText {
            title: "Multi-Plane Overlay (MPO) deaktivieren",
            description: "Deaktiviert die Hardware-Compositor-Funktion Multi-Plane Overlay. MPO verursacht bei vielen GPU-/Treiberkombinationen bekanntermaßen Ruckler, schwarze Bildschirme und Frame-Pacing-Probleme, insbesondere bei Multi-Monitor-Setups mit unterschiedlichen Bildwiederholraten.",
            broken_features: &["Leicht erhöhter Stromverbrauch des GPU-Compositors auf Laptops"],
        }),
        "disable_core_parking" => Some(LocalizedTweakText {
            title: "CPU-Kernparken (Core Parking) deaktivieren",
            description: "Setzt den Mindestprozentsatz der Prozessorkerne, die ungeparkt bleiben müssen, auf 100 %, sodass stets alle Kerne für Workloads bereitstehen. Beseitigt Aufwecklatenzen auf Kosten eines höheren Leerlaufstroms.",
            broken_features: &[
                "Höherer Stromverbrauch im Leerlauf",
                "Verkürzte Akkulaufzeit bei Laptops",
            ],
        }),
        "disable_cstates" => Some(LocalizedTweakText {
            title: "CPU-Leerlauf-C-States deaktivieren",
            description: "Verhindert, dass der Prozessor in tiefe Leerlauf-C-States (C6/C7/C8) wechselt, indem die Leerlauf-Richtlinie auf Leistung gestellt wird. Reduziert Latenzspitzen bei Energiezustandswechseln auf Kosten des Leerlaufverbrauchs.",
            broken_features: &[
                "Deutlich höherer Stromverbrauch im Leerlauf",
                "Verkürzte Akkulaufzeit bei Laptops",
                "Höhere CPU-Temperaturen im Leerlauf",
            ],
        }),
        "gpu_shader_cache_size" => Some(LocalizedTweakText {
            title: "GPU-TDR-Zeitlimit verlängern & Treiber-Resets verhindern",
            description: "Erhöht das WDDM-Zeitlimit für Timeout Detection and Recovery (TdrDelay) von 2s auf 10s, um fälschliche GPU-Treiberzurücksetzungen und Ruckler bei intensiver DirectX-Shader-Kompilierung zu verhindern.",
            broken_features: &["Bei einem echten GPU-Absturz wartet die Anzeigetreiber-Wiederherstellung bis zu 10s"],
        }),
        "enable_game_mode" => Some(LocalizedTweakText {
            title: "Windows-Spielmodus aktivieren",
            description: "Stellt sicher, dass der Windows-Spielmodus aktiviert ist (AutoGameModeEnabled=1, AllowAutoGameMode=1), priorisiert Spielprozesse und unterdrückt Windows-Update- sowie Neustartbenachrichtigungen während des Spielens.",
            broken_features: &[],
        }),
        "disable_fullscreen_optimizations" => Some(LocalizedTweakText {
            title: "Vollbildoptimierungen global deaktivieren",
            description: "Deaktiviert die Windows-Vollbildoptimierungen (Hybrid aus randlosem Vollbild) global für alle Anwendungen. Dies erzwingt einen echten exklusiven Vollbildmodus, was Eingabelatenz und Frame-Time-Schwankungen in älteren Spielen verringern kann.",
            broken_features: &[
                "Alt-Tab aus Vollbildspielen wird langsamer",
                "Game Bar-Overlay funktioniert im exklusiven Vollbildmodus nicht",
            ],
        }),
        "4433ce93-b632-4d67-ae71-0241c31099c0" => Some(LocalizedTweakText {
            title: "AMD ULPS deaktivieren",
            description: "Deaktiviert Ultra Low Power State (ULPS) im AMD Radeon-Treiberstack. Verhindert, dass sekundäre GPU-Kerne oder Schlafzustände im 3D-Gaming mit Mikrorucklern und Frame-Latenzspitzen aufwachen.",
            broken_features: &["Leicht höherer Leerlaufstromverbrauch auf AMD Multi-GPU-/Crossfire-Laptops"],
        }),
        "8a4b4c4d-000e-42db-8bd7-293067f8fae7" => Some(LocalizedTweakText {
            title: "AMD ASPM deaktivieren",
            description: "Deaktiviert PCIe Active State Power Management für AMD-Grafikkarten. Verhindert, dass die PCIe-Busverbindung in Renderpausen in stromsparende L0s/L1-Zustände wechselt, und verringert PCIe-Übertragungsjitter.",
            broken_features: &["Erhöhter Leerlaufstromverbrauch des PCIe-Busses"],
        }),
        "a3836587-c461-4beb-9c4c-88493b36c138" => Some(LocalizedTweakText {
            title: "AMD Power Gating deaktivieren",
            description: "Deaktiviert aggressives dynamisches Power Gating bei AMD-Recheneinheiten und Render-Backends (DisablePowerGating=1, PP_GPUPowerDownEnabled=0). Hält Taktraten konstant ohne Drosselung.",
            broken_features: &["Höhere GPU-Chip-Temperaturen bei geringer Auslastung"],
        }),
        "db379bb5-fc8c-4ad3-be32-11be09e76fba" => Some(LocalizedTweakText {
            title: "AMD Video Clock Gating deaktivieren",
            description: "Verhindert Clock- und Power-Gating bei AMD VCE/UVD-Multimedia- und Hardware-Display-Engines (DisableVCEPowerGating=1, DisableVceClockGating=1, EnableUvdClockGating=0, EnableVceSwClockGating=0). Beseitigt DPC-Latenzspitzen.",
            broken_features: &[],
        }),
        "df8739ff-e322-4958-b369-df584bb9b0c1" => Some(LocalizedTweakText {
            title: "Intel Async Flips deaktivieren",
            description: "Setzt Display1_DisableAsyncFlips=1 in den Intel-Grafiktreiberschlüsseln, um synchronisierte Display-Engine-Flips zu erzwingen und Jitter in der Darstellungswarteschlange zu beseitigen.",
            broken_features: &[],
        }),
        "ebfb0ded-5d1a-4cd1-b5c0-4f7018dd6054" => Some(LocalizedTweakText {
            title: "Intel Adaptive V-Sync deaktivieren",
            description: "Setzt AdaptiveVsyncEnable=0 in den Intel-Grafiktreiberschlüsseln, um eine adaptive Bildwiederholratendrosselung zu verhindern, wenn die Bildrate unter die Monitorfrequenz fällt.",
            broken_features: &[],
        }),
        "fa58802f-b98c-4e98-aa67-067c5a394296" => Some(LocalizedTweakText {
            title: "NVIDIA Dynamic P-State deaktivieren",
            description: "Setzt DisableDynamicPstate=1, um die NVIDIA-GPU in einen festen Leistungszustand zu sperren und Taktschwankungen zu verhindern. Erhöht die Leistungsaufnahme im Leerlauf.",
            broken_features: &["Erhöht Leistungsaufnahme und Wärmeentwicklung der GPU im Leerlauf"],
        }),
        "9e7702be-9e03-4c96-a316-c4e435cb993e" => Some(LocalizedTweakText {
            title: "NVIDIA Async P-States deaktivieren",
            description: "Setzt DisableAsyncPstates=1, um asynchrone Leistungszustandswechsel bei NVIDIA zu unterbinden und deterministische Leistung für VR-/Echtzeit-Apps zu liefern.",
            broken_features: &["Kann das Heruntertakten des GPU-Speichers bei der Videowiedergabe verhindern"],
        }),
        "5f1a2b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c" => Some(LocalizedTweakText {
            title: "NVIDIA PowerMizer auf maximale Leistung stellen",
            description: "Erzwingt den PowerMizer-Modus 'Maximale Leistung' im Netzbetrieb und sperrt PCIe-Bus sowie Speichercontroller ohne Drosselung auf volle Geschwindigkeit.",
            broken_features: &["Höhere GPU-Temperaturen und Lüfterdrehzahl im Leerlauf"],
        }),

        // === os_settings.toml (21) ===
        "disable_animations" => Some(LocalizedTweakText {
            title: "Fensteranimationen deaktivieren",
            description: "Deaktiviert den visuellen Effekt beim Minimieren und Maximieren von Fenstern und reduziert so wahrgenommene UI-Verzögerungen.",
            broken_features: &[],
        }),
        "disable_transparency" => Some(LocalizedTweakText {
            title: "Transparenzeffekte deaktivieren",
            description: "Deaktiviert den Acryl-/Transparenzeffekt auf Taskleiste, Startmenü und Info-Center und entlastet das GPU-Compositing.",
            broken_features: &["Transparentes Erscheinungsbild von Taskleiste und Startmenü"],
        }),
        "disable_ntfs_last_access" => Some(LocalizedTweakText {
            title: "Windows-Hintergrund-Apps global deaktivieren",
            description: "Verhindert, dass UWP-/Microsoft-Store-Apps im minimierten oder geschlossenen Zustand aufwachen, Netzwerke abfragen und im Hintergrund still CPU/RAM verbrauchen.",
            broken_features: &["Store-Apps synchronisieren im geschlossenen Zustand nicht mehr im Hintergrund"],
        }),
        "disable_nagle_algorithm" => Some(LocalizedTweakText {
            title: "MSMQ Nagle-Algorithmus deaktivieren (TCPNoDelay)",
            description: "Deaktiviert den Nagle-Algorithmus speziell für Microsoft Message Queuing (MSMQ\\Parameters\\TCPNoDelay) und reduziert die Warteschlangenlatenz bei MSMQ-Workloads mit kleinen Nachrichten.",
            broken_features: &["Kann die Netzwerkbandbreitennutzung bei kleinen Paketen leicht erhöhen"],
        }),
        "disable_sticky_keys" => Some(LocalizedTweakText {
            title: "Einrastfunktion & Eingabehilfen-Tastenkombinationen deaktivieren",
            description: "Deaktiviert das 5x-Umschalt-Dialogfeld der Einrastfunktion, die Anschlagverzögerung und Statussignaltöne, um störende Pop-ups und Eingabeunterbrechungen beim kompetitiven Spielen zu verhindern.",
            broken_features: &["Barrierefreiheits-Tastenkürzel (5x Umschalttaste, 8s rechte Umschalttaste) deaktiviert"],
        }),
        "disable_mouse_shadow" => Some(LocalizedTweakText {
            title: "Explorer-Schlagschatten & Auswahl-Alphatransparenz deaktivieren",
            description: "Deaktiviert Schlagschatten von Desktop-Symbolbeschriftungen und die halbtransparente Alpha-Überblendung des Explorer-Auswahlrechtecks, was die DWM-GPU-Compositing-Last reduziert.",
            broken_features: &[],
        }),
        "high_performance_power_plan" => Some(LocalizedTweakText {
            title: "Aggressiven Prozessor-Leistungsboost festlegen",
            description: "Setzt den Prozessor-Leistungsboost-Modus im Netzbetrieb über PowerSchemes PerfBoostMode auf 'Aggressiv', um Boost-Taktdrosselungen unter interaktiver Last zu verhindern.",
            broken_features: &[
                "Höherer Stromverbrauch im Leerlauf",
                "Erhöhte Wärmeentwicklung bei Laptops",
            ],
        }),
        "disable_bing_search_start" => Some(LocalizedTweakText {
            title: "Bing-Websuche im Startmenü deaktivieren",
            description: "Verhindert, dass die Windows-Suche jeden Tastenanschlag an Bing-Server sendet, sodass App- und lokale Dateisuchen im Startmenü verzögerungsfrei ohne Cloud-Latenz erfolgen.",
            broken_features: &["Websuchergebnisse werden im Startmenü nicht angezeigt"],
        }),
        "disable_windows_copilot" => Some(LocalizedTweakText {
            title: "Windows Copilot KI-Integration deaktivieren",
            description: "Deaktiviert Windows Copilot systemweit per Gruppenrichtlinie und stoppt residente Copilot-Hintergrundagenten, Taskleistenschaltflächen sowie KI-Netzwerkanfragen.",
            broken_features: &["Windows Copilot-Seitenleiste und Tastenkürzel (Win+C) werden deaktiviert"],
        }),
        "disable_game_bar_presence_writer" => Some(LocalizedTweakText {
            title: "Game Bar-Präsenz, Nexus-Starter & Starthinweise deaktivieren",
            description: "Deaktiviert den Start der Game Bar über die Controller-Nexus-Taste sowie Start-Hinweisfenster über benutzerbeschreibbare HKCU\\Software\\Microsoft\\GameBar-Schlüssel – ohne TrustedInstaller-ACL-Fehler.",
            broken_features: &["Start über Xbox Guide Nexus-Taste und Game Bar-Startpanel deaktiviert"],
        }),
        "disable_widgets_news" => Some(LocalizedTweakText {
            title: "Windows-Widgets & MSN-Newsfeed deaktivieren",
            description: "Deaktiviert das Widgets-Symbol in der Taskleiste und verhindert das Abrufen von MSN-Nachrichten, Trendthemen und Wetter-Widgets im Hintergrund.",
            broken_features: &["Widgets-Flyout in der Taskleiste wird deaktiviert"],
        }),
        "gamedvr_config_store" => Some(LocalizedTweakText {
            title: "Xbox GameDVR-Aufnahme-Overlay deaktivieren",
            description: "Deaktiviert Hintergrund-Videoaufnahmen des Spielgeschehens sowie Xbox Game Bar Hook-Injektionen und beseitigt Frametime-Einbrüche und Mikroruckler.",
            broken_features: &["Sofortige Hintergrund-Wiederholungsaufnahme der Xbox Game Bar wird deaktiviert"],
        }),
        "gamedvr_app_capture" => Some(LocalizedTweakText {
            title: "Windows GameDVR-App-Aufzeichnung deaktivieren",
            description: "Deaktiviert Hintergrund-Audio-/Video-Aufnahmepuffer im Windows GameDVR-Subsystem.",
            broken_features: &[],
        }),
        "gamedvr_policy_disable" => Some(LocalizedTweakText {
            title: "GameDVR-Systemrichtlinie deaktivieren",
            description: "Systemweite Gruppenrichtliniensperre, die verhindert, dass Windows GameDVR-Aufnahmedienste im Hintergrund wieder aktiviert.",
            broken_features: &[],
        }),
        "consumer_features_disable" => Some(LocalizedTweakText {
            title: "Windows-Consumer-Cloud-Funktionen deaktivieren",
            description: "Verhindert, dass Windows während der Wartung automatisch gesponserte Drittanbieter-Apps und Spiele (z. B. TikTok, Disney, Candy Crush) herunterlädt und installiert.",
            broken_features: &[],
        }),
        "disable_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Feedback-, Autochk- & Karten-Telemetrieaufgaben deaktivieren",
            description: "Deaktiviert SIUF-Diagnoseabfragen des Windows Feedback-Hubs, den Autochk-Telemetrie-Proxy und geplante Karten-Benachrichtigungsupdates (übernommen aus Sophia Script).",
            broken_features: &["Proaktive Umfragebenachrichtigungen des Feedback-Hubs werden deaktiviert"],
        }),
        "disable_hpet_synthetic_timer" => Some(LocalizedTweakText {
            title: "Dynamic Tick (synthetischer Kernel-Zeitgeber) deaktivieren",
            description: "Deaktiviert das Dynamic-Tick-Stromsparen im Windows-NT-Kernel, verhindert Zeitgeberdrift und erzwingt zuverlässige hochfrequente Invariant-TSC-Timerabfragen für extrem niedrige DPC-Latenz.",
            broken_features: &["Leicht höherer CPU-Stromverbrauch im Leerlauf"],
        }),
        "optimize_tcp_congestion_tuning" => Some(LocalizedTweakText {
            title: "TCP-Überlastungssteuerung & Fenster-Auto-Tuning optimieren",
            description: "Aktiviert leistungsstarke RFC 1323 TCP-Zeitstempel und deaktiviert Broadcast-Verkehr für Web Services Dynamic Discovery (WSD), um Latenz beim Online-Gaming und Paket-Serialisierungsverzögerungen zu minimieren.",
            broken_features: &[],
        }),
        "disable_windows_error_reporting" => Some(LocalizedTweakText {
            title: "Windows-Fehlerberichterstattungs-Richtlinie & Uploads deaktivieren",
            description: "Erzwingt die Gruppenrichtliniensperre der Windows-Fehlerberichterstattung und unterbindet das Senden zusätzlicher Absturzabbilddaten an Microsoft-Server.",
            broken_features: &["Automatische Absturzberichte und Telemetrie-Uploads sind blockiert"],
        }),
        "disable_bing_search_start_menu" => Some(LocalizedTweakText {
            title: "Startmenü-Empfehlungen & Iris-Werbung deaktivieren",
            description: "Entfernt gesponserte Website-/Kontoempfehlungen, Iris-Tipps und überflüssige Einträge unter 'Zuletzt hinzugefügt' in Windows 10/11-Startmenülayouts.",
            broken_features: &["Werbeempfehlungen im Startmenü sind ausgeblendet"],
        }),
        "disable_cortana_autostart" => Some(LocalizedTweakText {
            title: "Benutzerbezogene Cortana-Suche & Taskleistenschaltfläche deaktivieren",
            description: "Deaktiviert die benutzerbezogene Cortana-Aktivierung, Suchstandortberechtigungen und blendet den Cortana-Taskleisteneintrag aus.",
            broken_features: &["Benutzerbezogene Cortana-Taskleistensuchintegration wird deaktiviert"],
        }),
        // === perf_power_ext.toml (20) ===
        "enable_long_file_paths" => Some(LocalizedTweakText {
            title: "Lange NTFS-Win32-Dateipfade aktivieren (>260 Zeichen)",
            description: "Entfernt die veraltete MAX_PATH-Begrenzung auf 260 Zeichen in Windows 10/11 per LongPathsEnabled=1 und verhindert Pfadabschneidefehler beim Kompilieren und Entpacken von Archiven.",
            broken_features: &[],
        }),
        "disable_background_search_toggle_and_app_policy" => Some(LocalizedTweakText {
            title: "Ausführung von Hintergrund-Apps & Such-Hintergrundschalter verweigern",
            description: "Setzt AppPrivacy\\LetAppsRunInBackground=2 (Erzwungen verweigern) und deaktiviert Search\\BackgroundAppGlobalToggle=0, damit ruhende UWP-/Store-Apps im Hintergrund weder CPU noch RAM verbrauchen.",
            broken_features: &["UWP-Apps wie Store/Mail synchronisieren nicht im geschlossenen Zustand"],
        }),
        "extend_webclient_file_size_limit" => Some(LocalizedTweakText {
            title: "WebClient- / WebDAV-Dateigrößenlimit maximieren (4 GB)",
            description: "Erhöht das Downloadlimit des Windows-WebClient-Dienstes von 50 MB auf das Maximum von 4 GB (0xFFFFFFFF) und behebt Kopierfehler auf eingebundenen WebDAV-/Netzwerkfreigaben.",
            broken_features: &[],
        }),
        "reduce_services_pipe_timeout" => Some(LocalizedTweakText {
            title: "Dienststart- & Pipe-Shutdown-Zeitlimit auf 15s verkürzen",
            description: "Senkt ServicesPipeTimeout vom Standardwert 30000 ms (30s) auf 15000 ms (15s), damit hängende Hintergrunddienste beim Systemstart und Herunterfahren schneller beendet werden.",
            broken_features: &["Sehr langsame benutzerdefinierte Datenbankdienste mit >15s Startzeit können ein Zeitlimit überschreiten"],
        }),
        "mmcss_scheduler_full_optimization" => Some(LocalizedTweakText {
            title: "Vollständige MMCSS-Multimedia-Scheduler-Optimierung (NoLazyMode, AlwaysOn, Hohe SFIO)",
            description: "Konfiguriert Multimedia\\SystemProfile NoLazyMode=1, AlwaysOn=1 sowie Games Tasks Scheduling Category='High' und SFIO Priority='High' für unterbrechungsfreie Multimedia- und Spiel-Thread-Planung.",
            broken_features: &[],
        }),
        "enable_directx_windowed_game_optimizations" => Some(LocalizedTweakText {
            title: "DirectX-Optimierungen für Fenstermodus-Flip-Model & VRR aktivieren (verlustfreie Zusammenführung)",
            description: "Führt SwapEffectUpgradeEnable=1 und VRROptimizeEnable=1 sicher in DirectXUserGlobalSettings zusammen, ohne Ihre spielspezifischen GPU- oder AutoHDR-Einstellungen zu überschreiben.",
            broken_features: &[],
        }),
        "disable_hardware_overlays_and_min_fps" => Some(LocalizedTweakText {
            title: "Kernel-Hardware-Overlays & DWM-Mindest-FPS-Drosselung deaktivieren",
            description: "Setzt GraphicsDrivers\\DisableOverlays=1 und Dwm\\OverlayMinFPS=0, um DWM-Hardware-Overlay-Umschaltverzögerungen und Ruckler bei niedrigen Bildwiederholraten zu vermeiden.",
            broken_features: &["Leicht höherer Stromverbrauch des DWM-Compositors im Leerlauf auf Laptops"],
        }),
        "nvidia_restore_legacy_sharpening" => Some(LocalizedTweakText {
            title: "Klassische NVIDIA-Treiber-Schärfungsrichtlinie wiederherstellen (EnableGR535=0)",
            description: "Setzt Global\\FTS\\EnableGR535=0, sodass die NVIDIA Systemsteuerung wieder die klassische ressourcenschonende Bildschärfung anzeigt, statt den neueren Overlay-Filterpfad zu erzwingen.",
            broken_features: &[],
        }),
        "nvidia_dlss_debug_indicator" => Some(LocalizedTweakText {
            title: "NVIDIA DLSS-Versions- & Preset-Debug-Wasserzeichen anzeigen",
            description: "Setzt Global\\NGXCore\\ShowDlssIndicator=1024, um die aktive DLSS-SDK-Version, Auflösungsskalierung und das Preset in DirectX-/Vulkan-Spielen auf dem Bildschirm anzuzeigen.",
            broken_features: &["Zeigt unten links in unterstützten Spielen ein DLSS-Diagnose-Wasserzeichen an"],
        }),
        "disable_game_bar_tips_and_capture" => Some(LocalizedTweakText {
            title: "Game Bar Controller-Nexus, Starthinweise & Verlaufsaufzeichnung deaktivieren",
            description: "Schaltet UseNexusForGameBarEnabled (Xbox-Guide-Tastenstarter), ShowStartupPanel-Hinweise und die Hintergrund-Ringpufferaufzeichnung HistoricalCaptureEnabled aus.",
            broken_features: &["Die Controller-Nexus-Taste öffnet das Game Bar-Overlay nicht mehr"],
        }),
        "optimize_mouse_thresholds_and_hover" => Some(LocalizedTweakText {
            title: "Mausbeschleunigungsschwellen nullen & 10ms Sofort-Hover-Reaktion aktivieren",
            description: "Setzt MouseThreshold1='0', MouseThreshold2='0' und MouseHoverTime='10' ms, sodass Zeigerbeschleunigungsschwellen komplett flach sind und Taskleisten-/Tooltip-Vorschauen sofort reagieren.",
            broken_features: &[],
        }),
        "disable_extra_accessibility_hotkeys" => Some(LocalizedTweakText {
            title: "Versehentliche Tastenkürzel für Tastaturmaus, Hohen Kontrast & Sprachausgabe deaktivieren",
            description: "Deaktiviert die Pop-ups für Alt+Umschalt+NumLock (Tastaturmaus), Alt+Umschalt+Druck (Hoher Kontrast) und Win+Strg+Eingabe (Sprachausgabe) beim Spielen.",
            broken_features: &["Tastenkürzel für Barrierefreiheit müssen über die Einstellungen-App umgeschaltet werden"],
        }),
        "disable_smart_multi_homed_name_resolution" => Some(LocalizedTweakText {
            title: "Intelligente Multi-Homed-DNS-Namensauflösung unter Windows deaktivieren",
            description: "Setzt DNSClient\\DisableSmartNameResolution=1, damit Windows DNS-Abfragen strikt über die primäre Schnittstelle sendet, statt parallele Anfragen über VPN-/virtuelle Adapter zu verteilen.",
            broken_features: &["Split-Tunnel-VPNs müssen Adapter-DNS-Metriken sauber konfigurieren"],
        }),
        "harden_windows_update_ux_and_store" => Some(LocalizedTweakText {
            title: "WU-Treiberüberschreibungen, unerwartete Update-Neustarts & Store-Auto-Downloads blockieren",
            description: "Schließt GPU-/Peripherietreiber bei Qualitätsupdates aus, verhindert erzwungene Neustarts während aktiver Anmeldung, schaltet eine erweiterte Update-Pause (3650 Tage) frei und deaktiviert stille Store-Hintergrunddownloads bei aktiv bleibenden Sicherheitsupdates.",
            broken_features: &["Microsoft Store-App-Updates müssen manuell geprüft werden"],
        }),
        "power_settings_pcie_usb_nvme_epb" => Some(LocalizedTweakText {
            title: "Ultra-Low-Latency-Tuning für aktiven Energieplan (PCIe ASPM, USB-Energiesparen, NVMe, EPP, PowerThrottling)",
            description: "Nutzt native Windows-powrprof-APIs im aktiven Energieschema (funktioniert mit Ausbalanciert, Höchstleistung und benutzerdefinierten Plänen), um im Netzbetrieb PCIe ASPM, selektives USB-Energiesparen, USB-3-Link-Stromsparen, primären/sekundären NVMe-Leerlauf und CPU-EPP (EPP=0) sowie Kernel-PowerThrottling zu deaktivieren.",
            broken_features: &["Höherer Leerlaufstromverbrauch im Netzbetrieb auf Desktops und angeschlossenen Laptops"],
        }),
        "disable_storage_sense_policy" => Some(LocalizedTweakText {
            title: "Automatische Hintergrundbereinigung der Speicheroptimierung (Storage Sense) deaktivieren",
            description: "Verhindert, dass die Windows-Speicheroptimierung während interaktiver Sitzungen automatische Datenträgerbereinigungen im Hintergrund ausführt.",
            broken_features: &["Temporäre Dateien & Papierkorb müssen manuell über Bereinigung & Apps geleert werden"],
        }),
        "consolidate_svchost_split_threshold" => Some(LocalizedTweakText {
            title: "Diensthosts dynamisch am physischen RAM konsolidieren (SvcHostSplitThresholdInKB) [Opt-In]",
            description: "Berechnet SvcHostSplitThresholdInKB dynamisch aus dem erkannten System-RAM (TotalRamInKb) und verringert Prozessanzahl sowie virtuellen Speicheroverhead auf Systemen ab 8 GB. Hinweis: verringert die Prozessisolation pro Dienst.",
            broken_features: &["Gemeinsame svchost-Absturzisolation reduziert; Task-Manager zeigt weniger gruppierte Dienste"],
        }),
        "disable_vbs_deviceguard" => Some(LocalizedTweakText {
            title: "Virtualisierungsbasierte Sicherheit (VBS) deaktivieren [Sicherheitskompromiss Opt-In]",
            description: "Deaktiviert EnableVirtualizationBasedSecurity und RequirePlatformSecurityFeatures. Kann die Bildrate in Spielen auf CPUs mit Virtualisierungsoverhead um 5–15 % steigern, schwächt jedoch Kernel-Virtualisierungsschutzgrenzen.",
            broken_features: &["Deaktiviert VBS-Sicherheitsisolation; Anti-Cheat- / Windows-Sicherheitswarnung kann erscheinen"],
        }),
        "disable_hvci_memory_integrity" => Some(LocalizedTweakText {
            title: "Kernisolierung / Speicher-Integrität (HVCI) deaktivieren [Sicherheitskompromiss Opt-In]",
            description: "Deaktiviert die Hypervisor-gestützte Code-Integrität (HVCI). Senkt DPC-/Interrupt-Latenzen in kompetitiven Spielen auf Systemen, wo HVCI Frame-Times verschlechtert, verzichtet aber auf die Erzwingung der Kernel-Treibercodesignatur.",
            broken_features: &["Windows-Sicherheit zeigt Warnung 'Kernisolierung / Speicher-Integrität Aus' an"],
        }),
        "disable_prefetch_and_sysmain" => Some(LocalizedTweakText {
            title: "Kernel-Prefetcher & SysMain (Superfetch) für schnelle NVMe-SSDs deaktivieren [Opt-In]",
            description: "Deaktiviert Anwendungs-/Boot-Prefetcher und stoppt das SysMain-Speicher-Preloading auf schnellen NVMe-Laufwerken, auf denen RAM-Caching unnötige Hintergrund-Datenträger-I/O verursacht.",
            broken_features: &["Kaltstartgeschwindigkeit von Apps kann auf SATA-HDDs leicht abnehmen"],
        }),

        // === privacy.toml (12) ===
        "disable_advertising_id" => Some(LocalizedTweakText {
            title: "Werbe-ID deaktivieren",
            description: "Deaktiviert die benutzerspezifische Werbe-ID, die Windows Apps für personalisierte Werbung und App-übergreifendes Tracking bereitstellt.",
            broken_features: &["Apps erhalten keine eindeutige Werbe-ID mehr"],
        }),
        "disable_cortana" => Some(LocalizedTweakText {
            title: "Cortana deaktivieren",
            description: "Deaktiviert Cortana per Gruppenrichtlinie und verhindert, dass der Assistent läuft, zuhört und Daten an Microsoft-Cloud-Dienste sendet.",
            broken_features: &["Cortana-Sprachassistent wird deaktiviert"],
        }),
        "disable_activity_history" => Some(LocalizedTweakText {
            title: "Aktivitätsverlauf / Zeitachse deaktivieren",
            description: "Deaktiviert den Windows-Aktivitätsverlauf (Zeitachse) und verhindert, dass Windows eine Chronik Ihrer Aktivitäten, besuchten Websites und geöffneten Dokumente sammelt und speichert.",
            broken_features: &["Zeitachse/Aktivitätsverlauf in der Aufgabenansicht bleibt leer"],
        }),
        "disable_online_speech_recognition" => Some(LocalizedTweakText {
            title: "Online-Spracherkennung deaktivieren",
            description: "Deaktiviert den cloudbasierten Dienst für Online-Spracherkennung. Sprachdaten werden nicht mehr zur Verarbeitung an Microsoft-Server gesendet.",
            broken_features: &["Cloudbasierte Spracherkennung funktioniert nicht; lokale/Offline-Erkennung bleibt unberührt"],
        }),
        "disable_tailored_experiences" => Some(LocalizedTweakText {
            title: "Individuelle Benutzererfahrungen deaktivieren",
            description: "Verhindert, dass Microsoft Diagnosedaten nutzt, um personalisierte Tipps, Werbung und Empfehlungen in Windows anzuzeigen.",
            broken_features: &["Personalisierte Windows-Vorschläge und Tipps werden deaktiviert"],
        }),
        "disable_windows_recall" => Some(LocalizedTweakText {
            title: "Windows Recall (KI-Schnappschüsse) deaktivieren",
            description: "Deaktiviert die mit Windows 11 24H2 eingeführte Funktion Windows Recall, die regelmäßig KI-indizierte Bildschirmfotos Ihrer Aktivität erstellt. Erfordert Build 26100+.",
            broken_features: &["Windows Recall KI-Schnappschussfunktion wird deaktiviert"],
        }),
        "disable_cloud_content_spotlight" => Some(LocalizedTweakText {
            title: "Cloud-Inhalte & Windows-Blickpunkt-Vorschläge deaktivieren",
            description: "Deaktiviert Windows-Blickpunkt (Spotlight), vorgeschlagene Inhalte in den Einstellungen sowie über die Cloud gelieferte Vorschläge für vorinstallierte Apps.",
            broken_features: &[
                "Sperrbildschirm-Blickpunktbilder werden deaktiviert",
                "Vorgeschlagene Apps erscheinen nicht mehr im Startmenü",
            ],
        }),
        "disable_location_tracking" => Some(LocalizedTweakText {
            title: "Standortverfolgung deaktivieren",
            description: "Deaktiviert den Windows-Standortdienst global und verhindert, dass Apps und das Betriebssystem auf Ihren geografischen Standort zugreifen.",
            broken_features: &["Apps mit Standortbedarf (Karten, Wetter) haben keinen Zugriff auf Ihre Position"],
        }),
        "disable_activity_cloud_sync" => Some(LocalizedTweakText {
            title: "Cloud-Zwischenablage & geräteübergreifende Synchronisierung deaktivieren",
            description: "Deaktiviert die Speicherung des Zwischenablageverlaufs sowie die geräteübergreifende Cloud-Synchronisierung per Gruppenrichtlinie, damit kopierte Passwörter und sensible Daten streng lokal bleiben.",
            broken_features: &["Win+V Zwischenablageverlauf und geräteübergreifendes Einfügen werden deaktiviert"],
        }),
        "disable_cdp_service" => Some(LocalizedTweakText {
            title: "Plattformdienst für verbundene Geräte deaktivieren",
            description: "Deaktiviert den Dienst für verbundene Geräte (CDPUserSvc) und unterbindet geräteübergreifende Telemetrie, App-Synchronisierungs-Broadcasts und Hintergrund-Beacons.",
            broken_features: &["Umgebungsfreigabe und geräteübergreifende Synchronisierung mit Mobilgeräten werden deaktiviert"],
        }),
        "spotlight_desktop_disable" => Some(LocalizedTweakText {
            title: "Windows-Blickpunkt-Downloads für den Sperrbildschirm deaktivieren",
            description: "Deaktiviert den automatischen Hintergrundabruf von Windows-Blickpunkt-Sperrbildschirm-Hintergründen und Microsoft-Werbebildern.",
            broken_features: &[],
        }),
        "spotlight_features_disable" => Some(LocalizedTweakText {
            title: "Tipps & Werbung auf dem Sperrbildschirm deaktivieren",
            description: "Deaktiviert Sperrbildschirm-Tipps, Vorschläge und gesponserte Microsoft-Store-App-Werbung über den Content Delivery Manager.",
            broken_features: &[],
        }),

        // === registry_lag.toml (18) ===
        "win32_priority_separation" => Some(LocalizedTweakText {
            title: "Win32PrioritySeparation — Vordergrund-Boost",
            description: "Stellt das Scheduler-Quantum auf kurz, variabel und mit hohem Vordergrund-Boost (0x26 = 38) ein. Dies priorisiert Vordergrundanwendungen für geringere Eingabelatenz in Spielen und interaktiven Workloads.",
            broken_features: &[],
        }),
        "menu_show_delay" => Some(LocalizedTweakText {
            title: "Menü-Anzeigeverzögerung auf 0 ms reduzieren",
            description: "Entfernt die künstliche Verzögerung, bevor Menüs beim Zeigen erscheinen. Standard ist 400 ms; der Wert 0 öffnet Menüs augenblicklich.",
            broken_features: &[],
        }),
        "mmcss_gpu_priority" => Some(LocalizedTweakText {
            title: "MMCSS — Priorität für Spiele und Pro-Audio erhöhen",
            description: "Erhöht die Thread-Priorität für Spiele und Pro-Audio im Multimedia Class Scheduler Service (MMCSS), damit Audio- und Echtzeit-Spiel-Threads höhere Planungs-Priorität erhalten.",
            broken_features: &[],
        }),
        "mouse_data_queue_size" => Some(LocalizedTweakText {
            title: "Maus-Datenwarteschlangengröße erhöhen",
            description: "Erhöht die Puffergröße der Mauseingabe-Warteschlange vom Standardwert 100 auf 256 Einträge und reduziert die Gefahr verworfener Mauseingaben bei Gaming-Mäusen mit hoher Abfragerate.",
            broken_features: &[],
        }),
        "keyboard_response" => Some(LocalizedTweakText {
            title: "Tastaturreaktion optimieren",
            description: "Stellt die Wiederholungsverzögerung der Tastatur per Registrierung auf den kürzesten und die Wiederholungsrate auf den schnellsten Wert ein und vergrößert die Tastatur-Datenwarteschlange.",
            broken_features: &[],
        }),
        "disable_wer_queue" => Some(LocalizedTweakText {
            title: "Warteschlange der Windows-Fehlerberichterstattung deaktivieren",
            description: "Deaktiviert die Warteschlange der Windows-Fehlerberichterstattung (WER) und verhindert, dass das System nach Programmabstürzen im Hintergrund Absturzabbilder und Berichte schreibt.",
            broken_features: &["Absturzberichte werden nicht an Microsoft gesendet"],
        }),
        "auto_end_tasks" => Some(LocalizedTweakText {
            title: "Hängende Aufgaben beim Herunterfahren automatisch beenden",
            description: "Beendet beim Herunterfahren hängende Anwendungen automatisch ohne Rückfrage. Verkürzt zudem WaitToKillAppTimeout und HungAppTimeout für schnelleres Abmelden und Herunterfahren.",
            broken_features: &["Ungespeicherte Arbeit in hängenden Anwendungen kann beim Herunterfahren verloren gehen"],
        }),
        "network_throttling_disable" => Some(LocalizedTweakText {
            title: "Netzwerk-Drosselungsindex deaktivieren",
            description: "Deaktiviert die Windows-Multimedia-Netzwerkdrosselung (setzt NetworkThrottlingIndex auf 0xFFFFFFFF) und verhindert Paketverluste sowie Latenzspitzen bei hoher Netzwerklast oder beim Spielen.",
            broken_features: &[],
        }),
        "system_responsiveness_gaming" => Some(LocalizedTweakText {
            title: "Systemreaktionsfähigkeit im Vordergrund maximieren",
            description: "Konfiguriert SystemResponsiveness auf 0 % und widmet 100 % der CPU-Rechenzyklen interaktiven Spielen und Anwendungen im Vordergrund, statt 20 % für Hintergrundaufgaben zu reservieren.",
            broken_features: &[],
        }),
        "ntfs_disable_last_access" => Some(LocalizedTweakText {
            title: "NTFS-Zeitstempel für letzten Zugriff deaktivieren",
            description: "Verhindert, dass das NTFS-Dateisystem bei jedem Lesezugriff die Datenträger-Metadaten-Zeitstempel aktualisiert, und reduziert SSD-/NVMe-Lese-/Schreib-I/O deutlich.",
            broken_features: &["Dateieigenschaften zeigen nicht mehr das exakte Datum des letzten Zugriffs an"],
        }),
        "disable_paging_executive" => Some(LocalizedTweakText {
            title: "Windows-Kernel & Treiber im RAM halten",
            description: "Zwingt den Windows-NT-Executive-Kernel und Treiberroutinen, im physischen RAM zu verbleiben statt auf die Festplatte ausgelagert zu werden, für sofortige Kernel-Interrupt-Reaktionen.",
            broken_features: &[],
        }),
        "large_system_cache" => Some(LocalizedTweakText {
            title: "Großen Systemdateicache aktivieren",
            description: "Konfiguriert Windows für einen größeren Dateisystemcache im physischen Arbeitsspeicher und beschleunigt wiederholte Dateizugriffe sowie das Streaming von Spieltexturen.",
            broken_features: &[],
        }),
        "disable_fast_startup" => Some(LocalizedTweakText {
            title: "Schnellstart deaktivieren (Schutz vor Hybrid-Standby-Lecks)",
            description: "Deaktiviert den Windows-Schnellstart (Hiberboot), sorgt bei jedem Herunterfahren für einen echten sauberen Neustartzyklus und verhindert Kernel-Speicherlecks sowie Treiberzustandsfehler über Sitzungen hinweg.",
            broken_features: &["Kaltstartzeit kann sich auf langsamen mechanischen Festplatten um 1–2 Sekunden verlängern"],
        }),
        "ntfs_disable_8dot3" => Some(LocalizedTweakText {
            title: "NTFS 8.3-Kurznamen-Erstellung deaktivieren",
            description: "Deaktiviert die veraltete MS-DOS 8.3-Kurznamengenerierung auf NTFS-Volumes und beseitigt Verzeichnisindizierungslatenz sowie Schreiboverhead auf schnellen NVMe-/SSDs.",
            broken_features: &[],
        }),
        "increase_mft_zone_reservation" => Some(LocalizedTweakText {
            title: "Puffer der Master File Table (MFT)-Zone vergrößern",
            description: "Reserviert einen größeren zusammenhängenden Datenträgerpuffer für die NTFS Master File Table, verhindert MFT-Fragmentierung und erhält maximalen Datenträgerdurchsatz.",
            broken_features: &[],
        }),
        "tcp_ack_frequency" => Some(LocalizedTweakText {
            title: "Niedrige TCP-Latenz (TcpAckFrequency)",
            description: "Zwingt den Windows-TCP-Netzwerkstack, Paket-ACKs sofort ohne Wartezeit für Timeout-Zusammenfassung zu senden, und senkt so den Ping in Online-Spielen.",
            broken_features: &[],
        }),
        "tcp_no_delay_global" => Some(LocalizedTweakText {
            title: "Nagle-Algorithmus deaktivieren (TCPNoDelay)",
            description: "Deaktiviert Nagles Paketpufferungs-Algorithmus über aktive Netzwerkschnittstellen hinweg und überträgt kleine Pakete sofort für minimale Round-Trip-Latenz.",
            broken_features: &[],
        }),
        "disable_mouse_acceleration" => Some(LocalizedTweakText {
            title: "Zeigerbeschleunigung deaktivieren (1:1-Mauseingabe)",
            description: "Deaktiviert die nichtlineare Windows-Mauszeigerbeschleunigung ('Zeigerbeschleunigung verbessern') für echte 1:1-Rohmausbewegung – essenziell für präzises kompetitives Zielen.",
            broken_features: &[],
        }),

        // === telemetry_apps.toml (7) ===
        "disable_nvidia_telemetry_service" => Some(LocalizedTweakText {
            title: "NVIDIA-Telemetrie-Containerdienst deaktivieren",
            description: "Deaktiviert den Dienst NvTelemetryContainer, der Nutzungsstatistiken und Absturzdaten des NVIDIA-Treibers sammelt und überträgt.",
            broken_features: &["NVIDIA-Absturzdaten und Nutzungstelemetrie werden nicht erfasst"],
        }),
        "disable_nvidia_telemetry_tasks" => Some(LocalizedTweakText {
            title: "Geplante Aufgaben für NVIDIA-Telemetrie deaktivieren",
            description: "Deaktiviert die geplanten Aufgaben NvTmMon, NvTmRep und NvProfile, die regelmäßig NVIDIA-Telemetriedaten melden.",
            broken_features: &[],
        }),
        "disable_adobe_update_service" => Some(LocalizedTweakText {
            title: "Adobe-Updatedienste deaktivieren",
            description: "Deaktiviert den Adobe Acrobat Update Service (AdobeARMservice) und den Adobe Genuine Monitor Service. Verhindert Hintergrund-Updateprüfungen und Lizenztelemetrie.",
            broken_features: &["Adobe-Produkte werden nicht automatisch aktualisiert; manuell aktualisieren"],
        }),
        "disable_edge_update" => Some(LocalizedTweakText {
            title: "Microsoft Edge-Updatedienste deaktivieren",
            description: "Deaktiviert die Microsoft Edge Update-Dienste (edgeupdate und edgeupdatem) sowie geplante Update-Aufgaben. Edge funktioniert weiterhin, aktualisiert sich jedoch nicht automatisch und fragt keine Updates im Hintergrund ab.",
            broken_features: &["Microsoft Edge wird nicht automatisch aktualisiert; Sicherheitsupdates müssen manuell installiert werden"],
        }),
        "disable_chrome_update" => Some(LocalizedTweakText {
            title: "Google Chrome-Updatedienste deaktivieren",
            description: "Deaktiviert die Google Update-Dienste (gupdate und gupdatem) und deren geplante Aufgaben. Chrome funktioniert weiterhin, aktualisiert sich jedoch nicht automatisch.",
            broken_features: &["Google Chrome wird nicht automatisch aktualisiert; Sicherheitsupdates müssen manuell installiert werden"],
        }),
        "disable_steam_client_service" => Some(LocalizedTweakText {
            title: "Automatischen Start des Steam Client Service deaktivieren",
            description: "Ändert den Starttyp des Steam Client Service von 'Automatisch' auf 'Manuell'. Steam startet den Dienst bei Bedarf beim Öffnen weiterhin, verbraucht jedoch keine Ressourcen im Hintergrund, wenn Steam nicht genutzt wird.",
            broken_features: &[],
        }),
        "disable_razer_telemetry" => Some(LocalizedTweakText {
            title: "Razer-Telemetrie und -Analysen deaktivieren",
            description: "Verhindert den automatischen Start des Razer Game Manager Service und der Razer Chroma SDK-Dienste und reduziert so Hintergrundtelemetrie und Analysen von Razer Synapse.",
            broken_features: &["Autostart-Funktionen von Razer Synapse erfordern möglicherweise einen manuellen Start"],
        }),

        // === telemetry_os.toml (14) ===
        "disable_diagtrack" => Some(LocalizedTweakText {
            title: "DiagTrack deaktivieren (Benutzererfahrung und Telemetrie im verbundenen Modus)",
            description: "Deaktiviert den Dienst 'Benutzererfahrung und Telemetrie im verbundenen Modus' (DiagTrack / diagsvc), den primären Windows-Telemetriedienst.",
            broken_features: &["Diagnosedaten werden nicht an Microsoft gesendet"],
        }),
        "disable_dmwappushservice" => Some(LocalizedTweakText {
            title: "dmwappushservice deaktivieren (WAP-Push-Nachrichtenrouting)",
            description: "Deaktiviert den Dienst für das Geräteverwaltungs-WAP-Push-Nachrichtenrouting (Wireless Application Protocol), der für das Telemetriedaten-Routing genutzt wird.",
            broken_features: &["MDM-Push-Nachrichten werden nicht empfangen"],
        }),
        "disable_telemetry_registry" => Some(LocalizedTweakText {
            title: "Telemetrie über die Registrierung (AllowTelemetry) deaktivieren",
            description: "Setzt die Richtlinie AllowTelemetry auf 0 (Sicherheit/Aus), die niedrigste Telemetriestufe. Bei Enterprise und Education deaktiviert dies Telemetrie vollständig; bei Home und Pro reduziert es sie auf das erforderliche Minimum.",
            broken_features: &["Einige Windows Insider- und Diagnosefunktionen funktionieren möglicherweise nicht"],
        }),
        "disable_compat_telrunner" => Some(LocalizedTweakText {
            title: "Aufgaben des Anwendungskompatibilitäts-Bewerters deaktivieren",
            description: "Deaktiviert die geplanten Aufgaben Microsoft Compatibility Appraiser und ProgramDataUpdater, die CompatTelRunner.exe ausführen, installierte Software scannen und Kompatibilitätstelemetrie senden.",
            broken_features: &["Kompatibilitätsprüfungen für Windows-Upgrades laufen nicht automatisch"],
        }),
        "disable_ceip" => Some(LocalizedTweakText {
            title: "Programm zur Verbesserung der Benutzerfreundlichkeit (CEIP) deaktivieren",
            description: "Deaktiviert die geplanten CEIP-Aufgaben (Consolidator, UsbCeip, KernelCeipTask) sowie den CEIP-Richtlinienschlüssel in der Registrierung.",
            broken_features: &["CEIP-Telemetriedaten werden nicht erfasst"],
        }),
        "disable_wer_service" => Some(LocalizedTweakText {
            title: "Windows-Fehlerberichterstattungsdienst deaktivieren",
            description: "Deaktiviert den Windows-Fehlerberichterstattungsdienst (WerSvc) und verhindert das Hochladen von Absturzberichten und Fehlertelemetrie an Microsoft.",
            broken_features: &[
                "Absturzberichte werden nicht an Microsoft gesendet",
                "Verlauf der Problemberichte wird nicht aktualisiert",
            ],
        }),
        "disable_connected_user_experiences" => Some(LocalizedTweakText {
            title: "Verbundene Benutzererfahrungen und Vorschläge deaktivieren",
            description: "Deaktiviert Vorschläge in der Einstellungen-App, im Startmenü und auf dem Sperrbildschirm, die über cloudbasierte Inhalte und Telemetrie-Feedback gesteuert werden.",
            broken_features: &["Windows-Vorschläge und Tipps erscheinen nicht"],
        }),
        "disable_diagnostic_data_viewer" => Some(LocalizedTweakText {
            title: "Diagnosedaten-Viewer deaktivieren",
            description: "Deaktiviert die Ereignisprotokollierung des Diagnosedaten-Viewers, die eine lokale Kopie der an Microsoft gesendeten Diagnosedaten speichert. Spart Datenträger-I/O und Speicherplatz.",
            broken_features: &["Diagnosedaten können nicht lokal über die Diagnosedaten-Viewer-App eingesehen werden"],
        }),
        "disable_delivery_optimization_p2p" => Some(LocalizedTweakText {
            title: "P2P-Upload-Bandbreitennutzung der Übermittlungsoptimierung deaktivieren",
            description: "Schränkt die Windows-Übermittlungsoptimierung ein (DODownloadMode = 0) und hindert Windows daran, Ihre Upload-Bandbreite zum Verteilen von Windows-Updates an fremde PCs im Internet zu nutzen.",
            broken_features: &[],
        }),
        "disable_diagnostic_execution_service" => Some(LocalizedTweakText {
            title: "Diagnoseausführungsdienst deaktivieren (diagsvc)",
            description: "Deaktiviert den Diagnoseausführungsdienst (diagsvc), der geplante Hintergrunddiagnosen und Telemetriepaket-Ausführungen durchführt.",
            broken_features: &[],
        }),
        "wersvc_service_disable" => Some(LocalizedTweakText {
            title: "Standardsammlerdienst des Diagnosehubs deaktivieren",
            description: "Deaktiviert den Visual Studio / Windows Diagnostics Hub Standard Collector Service und stoppt ETW-Ablaufverfolgungssammler im Leerlauf.",
            broken_features: &["Ablaufverfolgungssammlung des VS/Windows-Diagnosehubs erfordert manuellen Start"],
        }),
        "disable_remote_registry" => Some(LocalizedTweakText {
            title: "Remoteregistrierungsdienst deaktivieren",
            description: "Deaktiviert den RemoteRegistry-Dienst und verhindert, dass Remotebenutzer oder externe Netzwerkteilnehmer lokale Registrierungsschlüssel abfragen oder ändern.",
            broken_features: &["Remote-Verwaltung der Registrierung über das lokale LAN wird deaktiviert"],
        }),
        "disable_disk_diagnostic_task" => Some(LocalizedTweakText {
            title: "Telemetrieaufgabe für Datenträgerdiagnose deaktivieren",
            description: "Deaktiviert die geplante Aufgabe, die Datenträgernutzungsmetriken und Hardwarediagnosen für Telemetrieberichte sammelt.",
            broken_features: &[],
        }),
        "disable_winsat_task" => Some(LocalizedTweakText {
            title: "WinSAT-Hintergrundbenchmark-Aufgabe deaktivieren",
            description: "Verhindert, dass die geplante Aufgabe des Windows System Assessment Tool im Leerlauf ungefragt Datenträger- und GPU-Stresstests ausführt.",
            broken_features: &[],
        }),

        // === ux_customization.toml (13) ===
        "enable_classic_context_menu" => Some(LocalizedTweakText {
            title: "Vollständiges klassisches Windows-10-Kontextmenü wiederherstellen",
            description: "Registriert die CLSID-Überschreibung unter HKCU\\Software\\Classes, sodass ein Rechtsklick sofort das vollständige Kontextmenü ohne den Zwischenklick auf 'Weitere Optionen anzeigen' öffnet.",
            broken_features: &["Modernes kompaktes Win11-Kontextmenü durch klassisches Vollmenü ersetzt"],
        }),
        "explorer_power_user_defaults" => Some(LocalizedTweakText {
            title: "Dateierweiterungen, vollständigen Titelleistenpfad & 'Dieser PC'-Start zeigen, Sync-Hinweise ausblenden",
            description: "Zeigt bekannte Dateierweiterungen und versteckte Dateien an, öffnet den Explorer direkt mit 'Dieser PC' statt Schnellzugriff, aktiviert vollständige Pfade in der Titelleiste sowie kompakte Abstände und blendet OneDrive-/Sync-Banner im Explorer aus.",
            broken_features: &[],
        }),
        "explorer_hide_win11_home_gallery_nav" => Some(LocalizedTweakText {
            title: "Einträge 'Start' & 'Katalog' aus der Datei-Explorer-Navigation ausblenden",
            description: "Entfernt die angehefteten Einträge Windows 11 'Start' (Home) und 'Katalog' (Gallery) über HKCU\\Software\\Classes\\CLSID aus dem linken Navigationsbereich des Datei-Explorers.",
            broken_features: &["Schnelllinks für Start & Katalog in der linken Explorer-Seitenleiste ausgeblendet"],
        }),
        "explorer_disable_startup_delay_and_jpeg_compression" => Some(LocalizedTweakText {
            title: "App-Startverzögerung beseitigen & JPEG-Komprimierung für Hintergrundbilder deaktivieren",
            description: "Setzt Explorer\\Serialize\\StartupDelayInMSec=0, damit Autostart-Programme ohne künstliche 10s-Drosselung starten, und JPEGImportQuality=100 für verlustfreie Desktop-Hintergründe.",
            broken_features: &[],
        }),
        "explorer_desktop_this_pc_and_disable_autoplay" => Some(LocalizedTweakText {
            title: "Desktopsymbol 'Dieser PC' anzeigen & automatische Wiedergabe für Wechselmedien deaktivieren",
            description: "Platziert die Verknüpfung 'Dieser PC' auf dem Desktop und deaktiviert AutoPlay/AutoRun für USB- und optische Wechseldatenträger, um automatische Medien-Pop-ups und Autorun-Risiken zu blockieren.",
            broken_features: &["Eingesteckte USB-/SD-Medien öffnen keine automatischen Eingabeaufforderungen mehr"],
        }),
        "disable_visual_effects_userpreferencesmask_bits" => Some(LocalizedTweakText {
            title: "Schwere UI-Einblend-/Gleit-/Schattenanimationen via UserPreferencesMask-Bitfelder & DWM entfernen",
            description: "Nutzt gezielte Bitmasken-Änderungen an UserPreferencesMask (deaktiviert Menü-Einblenden/-Gleiten, Tooltip-Überblendung, Menüauswahl-Effekt, Zeigerschatten, Kombinationsfeld-Gleiten, weiches Listenfeld-Scrollen und Steuerelement-Animationen bei Erhalt von Schriftglättung & Desktop-Grundlagen) sowie EnableAeroPeek=0.",
            broken_features: &["Klassisch reaktionsschnelle UI-Übergänge ohne Gleit-/Überblendeffekte oder Schlagschatten"],
        }),
        "taskbar_win11_power_layout_and_end_task" => Some(LocalizedTweakText {
            title: "Win11-Taskleiste links ausrichten, Widgets/Taskansicht/Suche ausblenden, 'Task beenden' aktivieren (Pro/Ent-Richtlinie)",
            description: "Richtet Taskleistensymbole links aus (TaskbarAl=0), entfernt Taskansicht und Widgets (TaskbarDa=0), blendet das Suchfeld aus, aktiviert Rechtsklick 'Task beenden' in den Taskleisten-Entwicklereinstellungen, deaktiviert Aero Shake und entfernt Startmenü-'Empfohlen'-Einträge (Windows 11 22H2+ Pro/Enterprise/Education).",
            broken_features: &["Taskleistensymbole links ausgerichtet; Schaltflächen für Taskansicht & Widgets ausgeblendet"],
        }),
        "hide_win10_meet_now_tray" => Some(LocalizedTweakText {
            title: "Windows 10 'Jetzt besprechen'-Kamerasymbol im Infobereich ausblenden",
            description: "Blendet das Skype-Symbol 'Jetzt besprechen' (Meet Now) aus dem Windows-10-Infobereich aus (nur für Windows 10-Builds).",
            broken_features: &[],
        }),
        "enable_windows_dark_mode_and_clock_seconds" => Some(LocalizedTweakText {
            title: "Dunklen Modus für System & Apps + Sekunden in der Taskleistenuhr aktivieren",
            description: "Stellt AppsUseLightTheme=0 und SystemUsesLightTheme=0 für nativen Dunkelmodus in der Windows-Oberfläche ein und zeigt Live-Sekunden in der Taskleistenuhr an.",
            broken_features: &[],
        }),
        "disable_startup_sound_and_audio_ducking" => Some(LocalizedTweakText {
            title: "Windows-Startsound & Lautstärkeabsenkung bei Anrufen deaktivieren",
            description: "Schaltet den Windows-Anmeldeklang stumm und verhindert, dass Windows die Spiel-/Musiklautstärke um 80 % absenkt, sobald Discord, Teams oder Sprach-Apps einen Kommunikationsstream öffnen.",
            broken_features: &[],
        }),
        "enable_verbose_logon_status" => Some(LocalizedTweakText {
            title: "Ausführliche Statusmeldungen beim Starten, Anmelden & Herunterfahren aktivieren",
            description: "Setzt VerboseStatus=1, sodass Windows genaue Dienst- und Update-Phasen statt generischer Ladekreise anzeigt.",
            broken_features: &[],
        }),
        "disable_lock_screen_pro" => Some(LocalizedTweakText {
            title: "Sperrbildschirm-Vorhang vor der Anmeldung überspringen (Pro/Enterprise-Richtlinie)",
            description: "Springt nach Systemstart oder Sperrung direkt zur Passwort-/PIN-Eingabe, ohne dass zuerst das Sperrbildschirmfoto weggeschoben werden muss.",
            broken_features: &["Sperrbildschirm-Vorhang wird in Pro/Enterprise-Editionen übersprungen"],
        }),
        "enable_windows_developer_mode" => Some(LocalizedTweakText {
            title: "Windows-Entwicklermodus aktivieren (Sideloading von Apps & Symlinks)",
            description: "Aktiviert AllowDevelopmentWithoutDevLicense=1, sodass Benutzer symbolische Dateisystemlinks ohne UAC erstellen und MSIX-/AppX-Pakete sideloaden können.",
            broken_features: &["Ermöglicht unsigniertes MSIX-Sideloading; für Entwickler vorgesehen"],
        }),
        _ => None,
    }
}
