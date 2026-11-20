//! Exhaustive localization tables and metadata helpers for tweaks, categories,
//! WinGet apps, Windows optional features, telemetry services, power schemes,
//! and SQLite transaction ledger tokens across all 8 languages.

mod de;
mod es;
mod fr;
mod ja;
mod ru;
mod tr;
mod zh;

use crate::core::i18n::Language;

/// Localized display text for a catalog [`crate::core::risk::TweakDefinition`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalizedTweakText {
    pub title: &'static str,
    pub description: &'static str,
    pub broken_features: &'static [&'static str],
}

#[inline]
const fn non_english_idx(lang: Language) -> Option<usize> {
    match lang {
        Language::English => None,
        Language::Turkish => Some(0),
        Language::German => Some(1),
        Language::French => Some(2),
        Language::Spanish => Some(3),
        Language::Russian => Some(4),
        Language::Chinese => Some(5),
        Language::Japanese => Some(6),
    }
}

fn lookup_text_table<'a>(
    lang: Language,
    raw: &'a str,
    table: &[(&'static str, [&'static str; 7])],
) -> &'a str {
    let Some(idx) = non_english_idx(lang) else {
        return raw;
    };
    let trimmed = raw.trim();
    for (key, vals) in table {
        if key.eq_ignore_ascii_case(trimmed) {
            return vals[idx];
        }
    }
    raw
}

fn lookup_pair_table(
    lang: Language,
    key: &str,
    table: &[(&'static str, [(&'static str, &'static str); 7])],
) -> Option<(&'static str, &'static str)> {
    let idx = non_english_idx(lang)?;
    let trimmed = key.trim();
    for (entry_key, vals) in table {
        if entry_key.eq_ignore_ascii_case(trimmed) {
            return Some(vals[idx]);
        }
    }
    None
}

fn contains_ascii_ci(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let h = haystack.as_bytes();
    let n = needle.as_bytes();
    if n.len() > h.len() {
        return false;
    }
    h.windows(n.len()).any(|w| w.eq_ignore_ascii_case(n))
}

/// Look up localized title, description, and `broken_features` for a catalog tweak ID.
///
/// Returns `None` for [`Language::English`] so callers can use the canonical TOML strings directly.
pub fn localize_tweak(lang: Language, tweak_id: &str) -> Option<LocalizedTweakText> {
    match lang {
        Language::English => None,
        Language::Turkish => tr::lookup(tweak_id),
        Language::German => de::lookup(tweak_id),
        Language::French => fr::lookup(tweak_id),
        Language::Spanish => es::lookup(tweak_id),
        Language::Russian => ru::lookup(tweak_id),
        Language::Chinese => zh::lookup(tweak_id),
        Language::Japanese => ja::lookup(tweak_id),
    }
}

/// Localized tweak title (English base catalog).
#[allow(dead_code)]
pub fn tweak_title(id: &str) -> Option<&'static str> {
    match id {
        "disable_driver_search_order_probing" => Some("Disable Windows Update Driver Search Probing"),
        "disable_fast_user_switching" => Some("Disable Fast User Switching"),
        "disable_ip_source_routing" => Some("Disable IP Source Routing"),
        "disable_netsh_teredo_tunneling" => Some("Disable Teredo IPv6 Transition Tunneling"),
        "disable_network_adapter_power_saving" => Some("Disable NDIS Network Adapter PnP Power Saving"),
        "disable_remote_desktop_udp" => Some("Disable Remote Desktop Client UDP Transport"),
        "disable_smart_card_service" => Some("Disable Smart Card Service (SCardSvr)"),
        "optimize_kerberos_ticket_cache" => Some("Optimize Kerberos Max Token Size Buffer"),
        "optimize_tcp_syn_attack_protection" => Some("Enable TCP SYN Attack Protection"),
        "optimize_usb_selective_suspend_hub" => Some("Disable Global USB Hub Selective Suspend"),
        "disable_dwm_animations_per_process" => Some("Disable DWM Per-Process Transition Animations"),
        "disable_dwm_composition_throttle" => Some("Disable Desktop Window Manager Composition Throttle"),
        "disable_kernel_mitigations_audit" => Some("Disable Kernel Mitigations Audit Mode"),
        "disable_llmnr_protocol" => Some("Disable LLMNR Name Resolution Protocol"),
        "disable_memory_compression" => Some("Disable Memory Compression Engine"),
        "disable_netbios_over_tcp" => Some("Disable NetBIOS over TCP/IP"),
        "disable_ntfs_encryption_driver" => Some("Disable NTFS Filesystem Encryption (EFS) Driver"),
        "disable_page_file_encryption" => Some("Disable Page File Encryption & Clear-on-Shutdown"),
        "disable_processor_idle_demote_threshold" => Some("Disable Processor Idle State Demotion"),
        "disable_rss_base_cpu" => Some("Set RSS Base CPU for Network Load Distribution"),
        "disable_spectre_meltdown_mitigations" => Some("Disable Spectre/Meltdown CPU Vulnerability Mitigations"),
        "disable_threaded_dpc_optimization" => Some("Disable Threaded DPC for Lower Interrupt Latency"),
        "disable_windows_search_indexer" => Some("Disable Windows Search Indexer Service"),
        "enable_hardware_task_offload" => Some("Enable TCP/IP Hardware Task Offloading"),
        "enable_large_ntfs_frs" => Some("Enable Large NTFS File Record Segments"),
        "enable_ultimate_performance_plan" => Some("Activate Ultimate Performance Power Plan"),
        "increase_gpu_tdr_timeout" => Some("Increase GPU Timeout Detection and Recovery Delay"),
        "increase_irp_stack_size" => Some("Increase IRP Stack Size for Network/Disk IO"),
        "increase_max_connections_per_server" => Some("Increase Maximum HTTP Connections Per Server"),
        "increase_nonpaged_pool_quota" => Some("Increase Non-Paged Pool Quota for Driver Stability"),
        "optimize_additional_critical_worker_threads" => Some("Increase Additional Kernel Worker Threads"),
        "optimize_default_ttl" => Some("Set Optimal Default IP Packet TTL"),
        "optimize_disk_io_page_lock_limit" => Some("Increase IO Page Lock Limit for Larger Transfers"),
        "optimize_dns_cache_parameters" => Some("Optimize DNS Client Cache Size and TTL"),
        "optimize_gpu_preemption_level" => Some("Set GPU Scheduler to Compute-Only Preemption"),
        "optimize_interrupt_steering" => Some("Optimize Interrupt Affinity and Steering Policy"),
        "optimize_mmcss_proaudio_scheduling" => Some("MMCSS Pro Audio — Maximum Scheduling Priority"),
        "optimize_ntfs_memory_usage" => Some("Aggressive NTFS Paged-Pool Caching"),
        "optimize_object_manager_quota" => Some("Increase Kernel Object Manager Handle Quota"),
        "optimize_timer_resolution_requests" => Some("Allow Applications to Request High Timer Resolution"),
        "disable_wpbt_execution" => Some("Disable WPBT Firmware Binary Execution"),
        "service_shutdown_timeout_15s" => Some("Reduce Service Shutdown Timeout to 15s"),
        _ => None,
    }
}

/// Localized tweak description (English base catalog).
#[allow(dead_code)]
pub fn tweak_description(id: &str) -> Option<&'static str> {
    match id {
        "disable_driver_search_order_probing" => Some("Prevents Windows Update from automatically querying Windows Update servers in the background when searching for driver software. Eliminates device install latency and background network polling during device insertion."),
        "disable_fast_user_switching" => Some("Disables Fast User Switching to prevent inactive user sessions and their background tasks from remaining loaded in physical RAM. Frees memory bandwidth, reduces scheduler contention, and prevents background session CPU usage."),
        "disable_ip_source_routing" => Some("Sets DisableIPSourceRouting=2 (highest protection) to discard all incoming source-routed IP packets. Prevents packet spoofing, path hijacking, and network stack processing overhead associated with loose/strict source-routed packets."),
        "disable_netsh_teredo_tunneling" => Some("Disables the Teredo tunneling pseudo-interface by setting IPv6 DisabledComponents to 0x20 (32). Teredo encapsulates IPv6 in IPv4 UDP packets, introducing NAT traversal latency and packet overhead in network stacks without native IPv6."),
        "disable_network_adapter_power_saving" => Some("Sets NDIS PnPCapabilities to 24 (disabling power management wake/sleep capabilities on network interfaces). Prevents network cards from entering low-power D-states during gameplay pauses, eliminating packet serialization delays."),
        "disable_remote_desktop_udp" => Some("Forces the Remote Desktop (RDP) client to use reliable TCP framing instead of UDP transport. Eliminates UDP socket buffer starvation and prevents RDP background traffic from competing with UDP gaming packet queues."),
        "disable_smart_card_service" => Some("Disables the Smart Card Service (SCardSvr) by setting its startup type to Disabled (4). Prevents background polling for smart card readers and cryptographic token devices, reducing system service overhead on consumer gaming PCs."),
        "optimize_kerberos_ticket_cache" => Some("Sets the Kerberos authentication MaxTokenSize buffer to 65535 bytes, preventing authentication buffer overflows and repeated round-trip ticket renegotiations in domains or environments with complex security group memberships."),
        "optimize_tcp_syn_attack_protection" => Some("Enables TCP SYN flood defense in the TCP/IP stack. Under SYN flood or micro-burst connection floods, Windows accelerates TCP handshake timeouts and reduces SYN-ACK retransmission counts, protecting network gaming sockets from starvation."),
        "optimize_usb_selective_suspend_hub" => Some("Disables USB selective suspend at the root USB driver stack level. Prevents USB controllers and hubs from powering down idle gaming mice, keyboards, and DACs, eliminating input wake-up latency and dropped polling packets."),
        "disable_dwm_animations_per_process" => Some("Disables the per-process DWM transition animations by setting DisallowAnimations=1 and AnimationsShiftKey=1 under the DWM key. This eliminates the fade and slide transitions when windows are minimized, maximized, or switched, reducing GPU compositor overhead and making window management feel instantaneous."),
        "disable_dwm_composition_throttle" => Some("Sets DesktopCompositionThrottle=0 under the DWM (Desktop Window Manager) key, preventing the compositor from throttling rendering when frames are being dropped. This allows DWM to maintain maximum composition throughput even under GPU load, reducing perceived desktop lag when switching between a fullscreen game and windowed applications."),
        "disable_kernel_mitigations_audit" => Some("Disables the kernel mitigation options audit logging by setting MitigationAuditOptions to all zeros. The kernel audit for exploit mitigations generates ETW events for every mitigation trigger, adding overhead to process creation and exception handling. Disabling the audit reduces kernel logging noise in performance-critical environments."),
        "disable_llmnr_protocol" => Some("Disables Link-Local Multicast Name Resolution (LLMNR) by setting EnableMulticast=0 in DNS client parameters. LLMNR broadcasts name queries on the local subnet when DNS fails, which is both a security risk (LLMNR poisoning/relay attacks) and a source of unnecessary multicast traffic on managed networks."),
        "disable_memory_compression" => Some("Disables the Windows Memory Manager compression store by setting DisableCompression=1. Windows 10+ compresses infrequently-used memory pages instead of paging them out; this saves disk IO but adds CPU overhead. On systems with 16 GB+ RAM, disabling compression reduces CPU usage spikes from the System process memory compression thread."),
        "disable_netbios_over_tcp" => Some("Disables NetBIOS over TCP/IP protocol by setting NetbiosOptions=2 and disabling the LMHosts lookup. NetBIOS is a legacy LAN protocol that broadcasts name resolution queries, exposing the system to unnecessary network noise and potential NBNS poisoning attacks. Disabling it reduces attack surface and network overhead on modern networks using DNS."),
        "disable_ntfs_encryption_driver" => Some("Sets NtfsDisableEncryption=1 under the FileSystem key, preventing the NTFS Encrypting File System filter driver from intercepting file IO operations. On systems that do not use per-file EFS encryption (most consumer and gaming PCs), this removes an unnecessary filter driver from the IO path, reducing file access latency."),
        "disable_page_file_encryption" => Some("Prevents Windows from encrypting the page file contents and disables clearing the page file at shutdown. Page file encryption imposes CPU overhead on every page-out operation, and clearing adds seconds to shutdown time. Disabling both improves paging throughput and shutdown speed, but leaves paged-out memory readable on the raw disk."),
        "disable_processor_idle_demote_threshold" => Some("Sets IdleDemoteThreshold=100 and IdlePromoteThreshold=100 in the processor power management parameters, preventing the kernel power manager from demoting processor performance states during brief idle periods. This keeps the CPU at its current performance level even during short idle gaps between work bursts, eliminating latency spikes from frequency ramp-up delays in bursty workloads like gaming and real-time processing."),
        "disable_rss_base_cpu" => Some("Sets RssBaseCpu=1 in the NDIS parameters, directing Receive Side Scaling to start distributing network interrupt processing from CPU 1 instead of CPU 0. This offloads network interrupt processing from the boot CPU, freeing CPU 0 for foreground application threads and reducing gaming micro-stutters caused by NIC interrupt contention."),
        "disable_spectre_meltdown_mitigations" => Some("Disables hardware-level Spectre and Meltdown CPU vulnerability mitigations by setting FeatureSettingsOverride=3 and FeatureSettingsOverrideMask=3. This removes the performance penalty imposed by speculative execution side-channel attack mitigations, yielding significant CPU throughput gains in compute-heavy workloads at the cost of leaving the system vulnerable to these hardware-level exploits."),
        "disable_threaded_dpc_optimization" => Some("Sets ThreadDpcEnable=0 in the Session Manager kernel key, forcing Deferred Procedure Calls to execute at DISPATCH_LEVEL instead of in a dedicated thread at PASSIVE_LEVEL. Threaded DPCs add scheduling overhead; disabling them reduces DPC latency at the cost of potentially blocking other PASSIVE_LEVEL work. Ideal for real-time audio production and competitive gaming where sub-millisecond interrupt handling is critical."),
        "disable_windows_search_indexer" => Some("Disables the WSearch (Windows Search) service, stopping the background indexer from continuously crawling and indexing file contents on all volumes. On SSD/NVMe systems, file search is fast enough without an index, and eliminating the indexer removes significant background disk IO, CPU usage, and memory consumption from the SearchIndexer.exe process."),
        "enable_hardware_task_offload" => Some("Sets DisableTaskOffload=0 in the TCP/IP parameters, ensuring the OS offloads checksum calculation, TCP segmentation, and large send operations to the NIC hardware. Modern NICs handle these operations faster than the CPU, reducing network processing overhead and freeing CPU cycles for application workloads."),
        "enable_large_ntfs_frs" => Some("Sets NtfsAllowExtendedCharacterIn8dot3Name=0, disabling extended character support in 8.3 short names and allowing the NTFS driver to allocate larger, more efficient File Record Segments. Combined with the existing 8.3 name disable, this further reduces MFT fragmentation and directory enumeration overhead on volumes with millions of files."),
        "enable_ultimate_performance_plan" => Some("Activates the hidden Ultimate Performance power scheme (e9a42b02-d5df-448d-aa00-03f14749eb61) via PowerCfgSetting. This plan eliminates micro-latencies from power management policies: no core parking, no frequency scaling delays, no USB selective suspend, and no hard disk spindown. Designed for workstation-class workloads demanding the lowest possible latency."),
        "increase_gpu_tdr_timeout" => Some("Raises the TdrDelay from the default 2 seconds to 10 seconds, giving the GPU more time to complete long-running shaders before the Windows Display Driver Model (WDDM) considers the GPU hung and resets the driver. This prevents false TDR resets during heavy compute or ray-tracing workloads that legitimately require extended GPU execution time."),
        "increase_irp_stack_size" => Some("Raises the I/O Request Packet stack size from the default 15 to 32 entries, giving the SMB server and layered filter drivers more stack frames for complex IO operations. Helps avoid STATUS_INSUFFICIENT_RESOURCES errors and improves throughput on systems running antivirus, backup agents, or deep storage filter stacks."),
        "increase_max_connections_per_server" => Some("Raises MaxConnectionsPerServer and MaxConnectionsPer1_0Server from the default 2-4 to 16, allowing WinINet-based applications (Edge, Explorer, Windows Update, Store) to open more simultaneous HTTP connections to a single host. This accelerates parallel downloads, large file transfers, and web application loading."),
        "increase_nonpaged_pool_quota" => Some("Raises NonPagedPoolQuota to 0 (unlimited/system-managed maximum) allowing the kernel memory manager to use all available physical memory for non-paged allocations. Prevents pool exhaustion on systems running many kernel-mode drivers, heavy network IO, or virtualization stacks."),
        "optimize_additional_critical_worker_threads" => Some("Sets AdditionalCriticalWorkerThreads to 2 and AdditionalDelayedWorkerThreads to 2, provisioning extra kernel-mode worker threads for handling critical and delayed work items. This reduces work-item queue stalls on systems with heavy IO and many concurrent processes."),
        "optimize_default_ttl" => Some("Sets the default IP Time-To-Live value to 64 hops, matching the Linux/macOS standard. The Windows default of 128 is unnecessarily high for most internet paths and can confuse network fingerprinting-based optimizations. A TTL of 64 is sufficient for virtually all internet routing topologies."),
        "optimize_disk_io_page_lock_limit" => Some("Sets IoPageLockLimit to 983040 bytes (960 KB), increasing the number of bytes the IO manager can lock for direct IO transfers. The default limit is too conservative for modern NVMe drives capable of multi-gigabyte-per-second throughput; raising it allows larger scatter-gather DMA transfers and reduces the number of IO operations needed for large file copies."),
        "optimize_dns_cache_parameters" => Some("Increases the DNS client cache maximum entry count and negative cache TTL floor. By setting MaxCacheEntryTtlLimit=86400 and MaxNegativeCacheTtl=5, the resolver caches successful lookups longer (reducing repeated DNS round-trips) while quickly retrying failed lookups. This improves web browsing responsiveness and reduces DNS query volume."),
        "optimize_gpu_preemption_level" => Some("Sets GpuPreemptionLevel=0 in the Graphics Drivers Scheduler, forcing compute-only GPU preemption. This prevents the GPU scheduler from preempting long-running shader workloads mid-execution, reducing GPU context-switch overhead and micro-stutters in graphically intensive games at the cost of potentially longer GPU hangs if a shader stalls."),
        "optimize_interrupt_steering" => Some("Configures the kernel interrupt steering policy to IrqPolicySpreadMessagesAcrossAllProcessors by setting InterruptSteeringDisabled=0 and DistributeTimers=1 under the Session Manager kernel key. This distributes hardware interrupts and deferred timer callbacks across all available cores rather than concentrating them on CPU 0, reducing single-core bottlenecks from interrupt storms."),
        "optimize_mmcss_proaudio_scheduling" => Some("Sets the Pro Audio MMCSS task SchedulingCategory to 'High' and SFIO Priority to 'High', ensuring real-time audio threads receive the highest available multimedia scheduling priority. This minimizes audio glitches, buffer underruns, and latency in DAWs, music production software, and VoIP applications during heavy system load."),
        "optimize_ntfs_memory_usage" => Some("Sets NtfsMemoryUsage to 2, instructing the NTFS driver to use a larger paged-pool allocation for directory and metadata caching. This reduces repeated disk reads on volumes with many small files, improving file enumeration and build system performance on systems with ample RAM."),
        "optimize_object_manager_quota" => Some("Raises the Session Manager ObjectDirectories additional kernel object quota threshold, and increases the handle table size limit via SessionPoolSize=48. This prevents handle table exhaustion on systems running many concurrent processes with heavy file handle usage (build systems, IDEs, game launchers with many child processes)."),
        "optimize_timer_resolution_requests" => Some("Sets GlobalTimerResolutionRequests=1 in the Session Manager kernel key, allowing applications to request the maximum platform timer resolution (0.5 ms) system-wide. Without this, Windows may ignore high-resolution timer requests from non-foreground processes, causing timing jitter in background audio engines, game servers, and real-time applications."),
        "disable_wpbt_execution" => Some("Blocks the Windows Kernel from executing binaries bundled into the motherboard UEFI Windows Platform Binary Table (WPBT)."),
        "service_shutdown_timeout_15s" => Some("Reduces the OS service termination timeout during shutdown from default 20s to 15s to prevent long shutdown hangs."),
        _ => None,
    }
}


static CATEGORY_TABLE: &[(&str, [&str; 7])] = &[
    (
        "Bloatware",
        [
            "Gereksiz Uygulamalar",
            "Bloatware-Apps",
            "Applications superflues",
            "Apps innecesarias",
            "Ненужные приложения",
            "预装冗余应用",
            "不要アプリ・ブロートウェア",
        ],
    ),
    (
        "GPU Tuning",
        [
            "GPU Ayarları",
            "GPU-Tuning",
            "Optimisation GPU",
            "Ajustes de GPU",
            "Настройка GPU",
            "GPU 专项调优",
            "GPU チューニング",
        ],
    ),
    (
        "Hardware Tuning",
        [
            "Donanım Ayarları",
            "Hardware-Tuning",
            "Optimisation matérielle",
            "Ajustes de hardware",
            "Настройка оборудования",
            "硬件调优",
            "ハードウェア最適化",
        ],
    ),
    (
        "OS Settings",
        [
            "İşletim Sistemi Ayarları",
            "Systemeinstellungen",
            "Paramètres système",
            "Ajustes del sistema",
            "Настройки ОС",
            "系统设置",
            "OS 設定",
        ],
    ),
    (
        "Privacy",
        [
            "Gizlilik",
            "Datenschutz",
            "Confidentialité",
            "Privacidad",
            "Приватность",
            "隐私保护",
            "プライバシー",
        ],
    ),
    (
        "Registry Lag",
        [
            "Kayıt Defteri Gecikmesi",
            "Registry-Latenz",
            "Latence du registre",
            "Latencia del registro",
            "Задержки реестра",
            "注册表延迟优化",
            "レジストリ遅延最適化",
        ],
    ),
    (
        "Telemetry (Apps)",
        [
            "Telemetri (Uygulamalar)",
            "Telemetrie (Apps)",
            "Télémétrie (Applications)",
            "Telemetría (Apps)",
            "Телеметрия (Приложения)",
            "遥测 (第三方应用)",
            "テレメトリ (アプリ)",
        ],
    ),
    (
        "Telemetry (OS)",
        [
            "Telemetri (OS)",
            "Telemetrie (Betriebssystem)",
            "Télémétrie (Système)",
            "Telemetría (SO)",
            "Телеметрия (ОС)",
            "遥测 (Windows 系统)",
            "テレメトリ (OS)",
        ],
    ),
    (
        "Kernel & Memory Optimization",
        [
            "Çekirdek ve Bellek Optimizasyonu",
            "Kernel- & Speicheroptimierung",
            "Optimisation du noyau et de la mémoire",
            "Optimización de kernel y memoria",
            "Оптимизация ядра и памяти",
            "内核与内存优化",
            "カーネルとメモリの最適化",
        ],
    ),
    (
        "Network Stack Optimization",
        [
            "Ağ Yığını Optimizasyonu",
            "Netzwerk-Stack-Optimierung",
            "Optimisation de la pile réseau",
            "Optimización de la pila de red",
            "Оптимизация сетевого стека",
            "网络协议栈优化",
            "ネットワークスタックの最適化",
        ],
    ),
    (
        "GPU & Display Pipeline",
        [
            "GPU ve Ekran İşlem Hattı",
            "GPU- & Anzeige-Pipeline",
            "Pipeline GPU et affichage",
            "Canalización de GPU y pantalla",
            "Конвейер GPU и отображения",
            "GPU 与显示管线",
            "GPU・ディスプレイパイプライン",
        ],
    ),
    (
        "Storage & IO Optimization",
        [
            "Depolama ve G/Ç Optimizasyonu",
            "Speicher- & E/A-Optimierung",
            "Optimisation du stockage et des E/S",
            "Optimización de almacenamiento y E/S",
            "Оптимизация хранилища и ввода-вывода",
            "存储与 I/O 优化",
            "ストレージ・IO 最適化",
        ],
    ),
    (
        "Scheduler & Latency",
        [
            "Zamanlayıcı ve Gecikme",
            "Scheduler & Latenz",
            "Planificateur et latence",
            "Planificador y latencia",
            "Планировщик и задержки",
            "调度器与延迟优化",
            "スケジューラ・レイテンシ",
        ],
    ),
];

/// Localize a raw catalog category name across all 8 languages, falling back to `raw_category`.
pub fn localize_category(lang: Language, raw_category: &str) -> &str {
    lookup_text_table(lang, raw_category, CATEGORY_TABLE)
}

static SUB_CATEGORY_TABLE: &[(&str, [&str; 7])] = &[
    (
        "Power",
        [
            "Güç",
            "Energie",
            "Alimentation",
            "Energía",
            "Питание",
            "电源",
            "電源",
        ],
    ),
    (
        "Performance",
        [
            "Performans",
            "Leistung",
            "Performance",
            "Rendimiento",
            "Производительность",
            "性能",
            "パフォーマンス",
        ],
    ),
    (
        "Power & Clocks",
        [
            "Güç ve Saat Hızları",
            "Energie & Taktraten",
            "Alimentation et fréquences",
            "Energía y frecuencias",
            "Питание и частоты",
            "功耗与频率",
            "電力・クロック",
        ],
    ),
    (
        "Latency",
        [
            "Gecikme",
            "Latenz",
            "Latence",
            "Latencia",
            "Задержка",
            "延迟",
            "レイテンシ",
        ],
    ),
    (
        "Compositor",
        [
            "Görüntü Oluşturucu",
            "Desktop-Compositor",
            "Compositeur",
            "Compositor de ventana",
            "Композитор",
            "合成器",
            "コンポジター",
        ],
    ),
    (
        "Driver & Clarity",
        [
            "Sürücü ve Netlik",
            "Treiber & Bildschärfe",
            "Pilote et netteté",
            "Controlador y nitidez",
            "Драйвер и чёткость",
            "驱动与清晰度",
            "ドライバ・鮮明度",
        ],
    ),
    (
        "Diagnostics",
        [
            "Tanılama",
            "Diagnose",
            "Diagnostics",
            "Diagnóstico",
            "Диагностика",
            "诊断",
            "診断",
        ],
    ),
];

/// Localize a tweak sub-category badge across all 8 languages, falling back to `raw_sub`.
pub fn localize_sub_category(lang: Language, raw_sub: &str) -> &str {
    lookup_text_table(lang, raw_sub, SUB_CATEGORY_TABLE)
}

/// Canonical check indicating whether a raw catalog category belongs to Privacy or Telemetry.
pub fn is_privacy_or_telemetry_category(raw_category: &str) -> bool {
    contains_ascii_ci(raw_category, "privacy") || contains_ascii_ci(raw_category, "telemetry")
}

const CAT_UTILITIES: [&str; 7] = [
    "Yardımcı Araçlar",
    "Dienstprogramme",
    "Utilitaires",
    "Utilidades",
    "Утилиты",
    "实用工具",
    "ユーティリティ",
];
const CAT_DEVELOPMENT: [&str; 7] = [
    "Geliştirme",
    "Entwicklung",
    "Développement",
    "Desarrollo",
    "Разработка",
    "开发工具",
    "開発",
];
const CAT_MEDIA: [&str; 7] = [
    "Medya",
    "Medien",
    "Médias",
    "Multimedia",
    "Медиа",
    "多媒体",
    "メディア",
];
const CAT_BROWSERS: [&str; 7] = [
    "Tarayıcılar",
    "Browser",
    "Navigateurs",
    "Navegadores",
    "Браузеры",
    "浏览器",
    "ブラウザー",
];
const CAT_DIAGNOSTICS: [&str; 7] = [
    "Tanılama",
    "Diagnose",
    "Diagnostics",
    "Diagnóstico",
    "Диагностика",
    "硬件诊断",
    "診断",
];

static WINGET_TABLE: &[(&str, [(&str, &str); 7])] = &[
    (
        "7zip.7zip",
        [
            (CAT_UTILITIES[0], "Hızlı ve açık kaynaklı dosya arşivleyici"),
            (CAT_UTILITIES[1], "Schneller Open-Source-Dateiarchivierer"),
            (CAT_UTILITIES[2], "Archiveur de fichiers open-source rapide"),
            (CAT_UTILITIES[3], "Compresor y archivador de código abierto rápido"),
            (CAT_UTILITIES[4], "Быстрый архиватор с открытым исходным кодом"),
            (CAT_UTILITIES[5], "快速、开源的文件压缩与归档工具"),
            (CAT_UTILITIES[6], "高速なオープンソースのファイル圧縮・解凍ツール"),
        ],
    ),
    (
        "Microsoft.PowerToys",
        [
            (CAT_UTILITIES[0], "Microsoft'un resmi uzman kullanıcı araç paketi"),
            (CAT_UTILITIES[1], "Offizielle Microsoft-Tool-Sammlung für Power-User"),
            (CAT_UTILITIES[2], "Suite officielle d'utilitaires Microsoft pour utilisateurs avancés"),
            (CAT_UTILITIES[3], "Suite oficial de utilidades de Microsoft para usuarios avanzados"),
            (CAT_UTILITIES[4], "Официальный набор утилит Microsoft для опытных пользователей"),
            (CAT_UTILITIES[5], "微软官方高阶用户系统增强工具套件"),
            (CAT_UTILITIES[6], "Microsoft 公式のパワーユーザー向けユーティリティ群"),
        ],
    ),
    (
        "voidtools.Everything",
        [
            (CAT_UTILITIES[0], "Anlık NTFS dosya adı arama motoru"),
            (CAT_UTILITIES[1], "Sofortige NTFS-Dateinamensuche"),
            (CAT_UTILITIES[2], "Moteur de recherche instantanée de fichiers NTFS"),
            (CAT_UTILITIES[3], "Motor de búsqueda instantánea de archivos NTFS"),
            (CAT_UTILITIES[4], "Мгновенный поиск файлов на томах NTFS по имени"),
            (CAT_UTILITIES[5], "毫秒级响应的 NTFS 文件名即时搜索引擎"),
            (CAT_UTILITIES[6], "NTFS ファイル名を瞬時に検索する高速検索エンジン"),
        ],
    ),
    (
        "Notepad++.Notepad++",
        [
            (CAT_DEVELOPMENT[0], "Hafif metin ve kaynak kodu düzenleyicisi"),
            (CAT_DEVELOPMENT[1], "Leichtgewichtiger Text- und Quellcode-Editor"),
            (CAT_DEVELOPMENT[2], "Éditeur léger de texte et de code source"),
            (CAT_DEVELOPMENT[3], "Editor ligero de texto y código fuente"),
            (CAT_DEVELOPMENT[4], "Лёгкий текстовый редактор и редактор исходного кода"),
            (CAT_DEVELOPMENT[5], "轻量级文本与源代码编辑器"),
            (CAT_DEVELOPMENT[6], "軽量なテキストおよびソースコードエディター"),
        ],
    ),
    (
        "VideoLAN.VLC",
        [
            (CAT_MEDIA[0], "Açık kaynaklı çoklu ortam oynatıcısı"),
            (CAT_MEDIA[1], "Open-Source-Mediaplayer für nahezu alle Formate"),
            (CAT_MEDIA[2], "Lecteur multimédia open-source"),
            (CAT_MEDIA[3], "Reproductor multimedia de código abierto"),
            (CAT_MEDIA[4], "Мультимедиа-проигрыватель с открытым исходным кодом"),
            (CAT_MEDIA[5], "开源全能多媒体播放器"),
            (CAT_MEDIA[6], "オープンソースのマルチメディアプレーヤー"),
        ],
    ),
    (
        "ShareX.ShareX",
        [
            (CAT_UTILITIES[0], "Hafif ekran görüntüsü alma ve açıklama aracı"),
            (CAT_UTILITIES[1], "Leichtgewichtiges Werkzeug für Screenshots und Anmerkungen"),
            (CAT_UTILITIES[2], "Outil léger de capture d'écran et d'annotation"),
            (CAT_UTILITIES[3], "Herramienta ligera de captura de pantalla y anotación"),
            (CAT_UTILITIES[4], "Лёгкий инструмент для снимков экрана и аннотаций"),
            (CAT_UTILITIES[5], "轻量级屏幕截图与标注工具"),
            (CAT_UTILITIES[6], "軽量なスクリーンショット撮影・注釈ツール"),
        ],
    ),
    (
        "OBSProject.OBSStudio",
        [
            (CAT_MEDIA[0], "Ekran kaydı ve canlı yayın stüdyosu"),
            (CAT_MEDIA[1], "Studio für Bildschirmaufnahme und Livestreaming"),
            (CAT_MEDIA[2], "Studio d'enregistrement d'écran et de diffusion en direct"),
            (CAT_MEDIA[3], "Estudio de grabación de pantalla y transmisión en vivo"),
            (CAT_MEDIA[4], "Студия для записи экрана и прямых трансляций"),
            (CAT_MEDIA[5], "开源屏幕录制与实时直播工作室"),
            (CAT_MEDIA[6], "画面録画およびライブ配信スタジオ"),
        ],
    ),
    (
        "Mozilla.Firefox",
        [
            (CAT_BROWSERS[0], "Gizlilik odaklı açık kaynaklı web tarayıcısı"),
            (CAT_BROWSERS[1], "Datenschutzorientierter Open-Source-Webbrowser"),
            (CAT_BROWSERS[2], "Navigateur web open-source axé sur la confidentialité"),
            (CAT_BROWSERS[3], "Navegador web de código abierto centrado en la privacidad"),
            (CAT_BROWSERS[4], "Веб-браузер с открытым исходным кодом и фокусом на приватность"),
            (CAT_BROWSERS[5], "注重隐私保护的开源网页浏览器"),
            (CAT_BROWSERS[6], "プライバシー重視のオープンソース Web ブラウザー"),
        ],
    ),
    (
        "Brave.Brave",
        [
            (CAT_BROWSERS[0], "Yerleşik reklam ve izleyici korumalı Chromium tarayıcısı"),
            (CAT_BROWSERS[1], "Chromium-Browser mit integriertem Werbe- und Tracker-Blocker"),
            (CAT_BROWSERS[2], "Navigateur Chromium avec bloqueur de pubs et traqueurs intégré"),
            (CAT_BROWSERS[3], "Navegador Chromium con bloqueo integrado de anuncios y rastreadores"),
            (CAT_BROWSERS[4], "Браузер на базе Chromium со встроенной защитой от рекламы и трекеров"),
            (CAT_BROWSERS[5], "内置广告与追踪器拦截盾的 Chromium 浏览器"),
            (CAT_BROWSERS[6], "広告・トラッカー遮断機能内蔵の Chromium ブラウザー"),
        ],
    ),
    (
        "Microsoft.VisualStudioCode",
        [
            (CAT_DEVELOPMENT[0], "Genişletilebilir kod düzenleyicisi"),
            (CAT_DEVELOPMENT[1], "Erweiterbarer Code-Editor"),
            (CAT_DEVELOPMENT[2], "Éditeur de code extensible"),
            (CAT_DEVELOPMENT[3], "Editor de código extensible"),
            (CAT_DEVELOPMENT[4], "Расширяемый редактор кода"),
            (CAT_DEVELOPMENT[5], "可扩展的现代化代码编辑器"),
            (CAT_DEVELOPMENT[6], "拡張性の高いコードエディター"),
        ],
    ),
    (
        "Git.Git",
        [
            (CAT_DEVELOPMENT[0], "Dağıtılmış sürüm kontrol sistemi"),
            (CAT_DEVELOPMENT[1], "Verteiltes Versionskontrollsystem"),
            (CAT_DEVELOPMENT[2], "Système de gestion de versions décentralisé"),
            (CAT_DEVELOPMENT[3], "Sistema de control de versiones distribuido"),
            (CAT_DEVELOPMENT[4], "Распределённая система контроля версий"),
            (CAT_DEVELOPMENT[5], "分布式版本控制系统"),
            (CAT_DEVELOPMENT[6], "分散型バージョン管理システム"),
        ],
    ),
    (
        "CPUID.HWMonitor",
        [
            (CAT_DIAGNOSTICS[0], "Donanım voltajı, fan ve sıcaklık sensörü izleyicisi"),
            (CAT_DIAGNOSTICS[1], "Monitor für Spannungen, Lüfterdrehzahlen und Temperatursensoren"),
            (CAT_DIAGNOSTICS[2], "Moniteur de tensions, ventilateurs et sondes de température"),
            (CAT_DIAGNOSTICS[3], "Monitor de voltaje, ventiladores y sensores de temperatura"),
            (CAT_DIAGNOSTICS[4], "Монитор напряжений, вентиляторов и датчиков температуры"),
            (CAT_DIAGNOSTICS[5], "硬件电压、风扇转速与温度传感器监测工具"),
            (CAT_DIAGNOSTICS[6], "ハードウェアの電圧・ファン回転数・温度センサー監視ツール"),
        ],
    ),
];

/// Return `(localized_category, localized_description)` for a curated WinGet app ID.
///
/// Returns `None` for [`Language::English`] or unknown IDs.
pub fn localize_winget(lang: Language, app_id: &str) -> Option<(&'static str, &'static str)> {
    lookup_pair_table(lang, app_id, WINGET_TABLE)
}

static OPTIONAL_FEATURES_TABLE: &[(&str, [(&str, &str); 7])] = &[
    (
        "Recall",
        [
            ("Windows Recall Yapay Zeka (İsteğe Bağlı Özellik)", "Copilot+ bilgisayarlarda ekran görüntüsü bellek dizinleyicisi (24H2+)."),
            ("Windows Recall KI (Optionales Feature)", "Screenshot-Speicherindexer auf Copilot+ PCs (24H2+)."),
            ("Windows Recall IA (Fonctionnalité facultative)", "Indexeur de mémoire par captures d'écran sur PC Copilot+ (24H2+)."),
            ("Windows Recall IA (Característica opcional)", "Indexador de memoria por capturas de pantalla en PC Copilot+ (24H2+)."),
            ("Windows Recall ИИ (Необязательный компонент)", "Индексатор памяти снимков экрана на ПК Copilot+ (24H2+)."),
            ("Windows Recall AI（可选功能）", "Copilot+ PC 上的屏幕截图记忆索引器 (24H2+)。"),
            ("Windows Recall AI（オプション機能）", "Copilot+ PC 向けのスクリーンショット記憶インデクサー (24H2+)。"),
        ],
    ),
    (
        "Containers-DisposableClientVM",
        [
            ("Windows Korumalı Alanı (Sandbox)", "Güvenilmeyen uygulamaları test etmek için tek kullanımlık yalıtılmış masaüstü sanal makinesi."),
            ("Windows-Sandbox", "Isolierte Einweg-Desktop-VM zum Testen nicht vertrauenswürdiger Apps."),
            ("Bac à sable Windows (Sandbox)", "Machine virtuelle isolée jetable pour tester des applications non fiables."),
            ("Espacio aislado de Windows (Sandbox)", "Máquina virtual de escritorio aislada y desechable para probar apps no confiables."),
            ("Песочница Windows (Sandbox)", "Одноразовая изолированная виртуальная машина для безопасного тестирования программ."),
            ("Windows 沙盒 (Sandbox)", "用于测试不受信任应用的一次性隔离桌面虚拟机。"),
            ("Windows サンドボックス", "信頼できないアプリを検証するための使い捨て隔離デスクトップ VM。"),
        ],
    ),
    (
        "Microsoft-Windows-Subsystem-Linux",
        [
            ("Linux için Windows Alt Sistemi (WSL)", "Windows içinde yerel Linux ortamlarını çalıştırın."),
            ("Windows-Subsystem für Linux (WSL)", "Native Linux-Umgebungen direkt unter Windows ausführen."),
            ("Sous-système Windows pour Linux (WSL)", "Exécutez des environnements Linux natifs dans Windows."),
            ("Subsistema de Windows para Linux (WSL)", "Ejecute entornos Linux nativos dentro de Windows."),
            ("Подсистема Windows для Linux (WSL)", "Запуск нативных сред Linux внутри Windows."),
            ("适用于 Linux 的 Windows 子系统 (WSL)", "在 Windows 内部直接运行原生 Linux 环境。"),
            ("Linux 用 Windows サブシステム (WSL)", "Windows 内でネイティブ Linux 環境を実行します。"),
        ],
    ),
    (
        "Microsoft-Hyper-V-All",
        [
            ("Hyper-V Sanallaştırma", "Tip-1 hipervizör platformu ve sanal makine yönetim araçları."),
            ("Hyper-V-Virtualisierung", "Typ-1-Hypervisor-Plattform und VM-Verwaltungswerkzeuge."),
            ("Virtualisation Hyper-V", "Plateforme d'hyperviseur de type 1 et outils de gestion de VM."),
            ("Virtualización Hyper-V", "Plataforma de hipervisor de tipo 1 y herramientas de gestión de VM."),
            ("Виртуализация Hyper-V", "Платформа гипервизора 1-го типа и средства управления ВМ."),
            ("Hyper-V 虚拟化平台", "Type-1 虚拟机监控程序平台与虚拟机管理工具。"),
            ("Hyper-V 仮想化", "Type-1 ハイパーバイザー基盤と仮想マシン管理ツール。"),
        ],
    ),
    (
        "NetFx3",
        [
            (".NET Framework 3.5", "Eski .NET 2.0/3.0/3.5 çalışma zamanı uyumluluğu."),
            (".NET Framework 3.5", "Kompatibilität für ältere .NET 2.0/3.0/3.5-Laufzeitumgebungen."),
            (".NET Framework 3.5", "Compatibilité avec les anciens environnements d'exécution .NET 2.0/3.0/3.5."),
            (".NET Framework 3.5", "Compatibilidad con entornos de ejecución heredados .NET 2.0/3.0/3.5."),
            (".NET Framework 3.5", "Совместимость со старой средой выполнения .NET 2.0/3.0/3.5."),
            (".NET Framework 3.5", "旧版 .NET 2.0/3.0/3.5 运行时兼容支持。"),
            (".NET Framework 3.5", "レガシ .NET 2.0/3.0/3.5 ランタイム互換機能。"),
        ],
    ),
    (
        "Browser.InternetExplorer",
        [
            ("Internet Explorer (Eski Bileşen)", "Eski Trident web tarayıcısı kabuğu."),
            ("Internet Explorer (Veraltete Funktion)", "Veraltete Trident-Webbrowser-Shell."),
            ("Internet Explorer (Fonctionnalité héritée)", "Ancienne coquille du navigateur web Trident."),
            ("Internet Explorer (Capacidad heredada)", "Motor heredado del navegador web Trident."),
            ("Internet Explorer (Устаревший компонент)", "Устаревшая оболочка веб-браузера на движке Trident."),
            ("Internet Explorer（旧版功能）", "旧版 Trident 引擎网页浏览器外壳。"),
            ("Internet Explorer（レガシ機能）", "旧式 Trident Web ブラウザーシェル。"),
        ],
    ),
    (
        "App.StepsRecorder",
        [
            ("Adım Kaydedici (Steps Recorder)", "Eski sorun adımı ekran görüntüsü kaydedicisi."),
            ("Schrittaufzeichnung (Steps Recorder)", "Veraltete Screenshot-Aufzeichnung für Problembeschreibungen."),
            ("Enregistreur d'actions (Steps Recorder)", "Ancien enregistreur d'étapes et de captures d'écran."),
            ("Grabación de acciones de usuario", "Grabador heredado de capturas paso a paso."),
            ("Средство записи действий", "Устаревший инструмент пошаговой записи снимков экрана."),
            ("步骤记录器 (Steps Recorder)", "旧版问题操作步骤截屏记录工具。"),
            ("ステップ記録ツール", "レガシの問題再現手順スクリーンショット記録ツール。"),
        ],
    ),
    (
        "Microsoft.Windows.WordPad",
        [
            ("WordPad (Eski)", "Eski RTF/DOC belge düzenleyici özelliği."),
            ("WordPad (Veraltet)", "Veraltete RTF/DOC-Editor-Funktion."),
            ("WordPad (Hérité)", "Ancien éditeur de documents RTF/DOC."),
            ("WordPad (Heredado)", "Editor heredado de documentos RTF/DOC."),
            ("WordPad (Устаревший)", "Устаревший встроенный редактор документов RTF/DOC."),
            ("写字板 WordPad（旧版）", "旧版内置 RTF/DOC 文档编辑器功能。"),
            ("ワードパッド（レガシ）", "レガシ RTF/DOC ドキュメント編集機能。"),
        ],
    ),
    (
        "Microsoft.Windows.PowerShell.ISE",
        [
            ("PowerShell ISE", "Eski PowerShell betik geliştirme ortamı (IDE)."),
            ("PowerShell ISE", "Veraltete PowerShell-Skript-IDE."),
            ("PowerShell ISE", "Ancien environnement de développement de scripts PowerShell."),
            ("PowerShell ISE", "IDE heredado para scripts de PowerShell."),
            ("PowerShell ISE", "Устаревшая среда разработки сценариев PowerShell."),
            ("PowerShell ISE", "旧版 PowerShell 脚本集成开发环境。"),
            ("PowerShell ISE", "レガシ PowerShell スクリプト統合開発環境。"),
        ],
    ),
    (
        "OpenSSH.Client",
        [
            ("OpenSSH İstemcisi", "Yerleşik SSH komut satırı istemcisi."),
            ("OpenSSH-Client", "Integrierter SSH-Befehlszeilenclient."),
            ("Client OpenSSH", "Client SSH en ligne de commande intégré."),
            ("Cliente OpenSSH", "Cliente CLI de SSH integrado."),
            ("Клиент OpenSSH", "Встроенный консольный SSH-клиент."),
            ("OpenSSH 客户端", "内置 OpenSSH 命令行客户端。"),
            ("OpenSSH クライアント", "組み込みの SSH コマンドラインクライアント。"),
        ],
    ),
];

/// Return `(localized_display_name, localized_description)` for a curated Windows optional
/// feature or capability (matching exact keys or base prefixes before `~~~~`/`~~~`).
///
/// Returns `None` for [`Language::English`] or unknown features.
pub fn localize_optional_feature(
    lang: Language,
    feature_name: &str,
) -> Option<(&'static str, &'static str)> {
    let base = feature_name
        .trim()
        .split("~~~~")
        .next()
        .unwrap_or(feature_name)
        .split("~~~")
        .next()
        .unwrap_or(feature_name);
    lookup_pair_table(lang, base, OPTIONAL_FEATURES_TABLE)
}

const RAZER_SYNAPSE_LABELS: [&str; 7] = [
    "Razer Synapse Telemetrisi",
    "Razer Synapse-Telemetrie",
    "Télémétrie Razer Synapse",
    "Telemetría de Razer Synapse",
    "Телеметрия Razer Synapse",
    "Razer Synapse 雷云遥测服务",
    "Razer Synapse テレメトリ",
];

static TELEMETRY_SERVICE_TABLE: &[(&str, [&str; 7])] = &[
    (
        "DiagTrack",
        [
            "Bağlı Kullanıcı Deneyimleri ve Telemetri",
            "Benutzererfahrung und Telemetrie im verbundenen Modus",
            "Expériences des utilisateurs connectés et télémétrie",
            "Experiencias del usuario conectado y telemetría",
            "Функциональные возможности для подключённых пользователей и телеметрия",
            "连接的用户体验和遥测",
            "接続ユーザーエクスペリエンスとテレメトリ",
        ],
    ),
    (
        "dmwappushservice",
        [
            "WAP Anlık İleti Yönlendirme Hizmeti",
            "WAP-Push-Nachrichten-Routingdienst",
            "Service de routage de messages Push WAP",
            "Servicio de enrutamiento de mensajes Push WAP",
            "Служба маршрутизации push-сообщений WAP",
            "WAP 推送消息路由服务",
            "WAP プッシュメッセージルーティングサービス",
        ],
    ),
    (
        "WerSvc",
        [
            "Windows Hata Raporlama Hizmeti",
            "Windows-Fehlerberichterstattungsdienst",
            "Service de rapport d'erreurs Windows",
            "Servicio de informe de errores de Windows",
            "Служба отчётов об ошибках Windows",
            "Windows 错误报告服务",
            "Windows エラー報告サービス",
        ],
    ),
    (
        "NvTelemetryContainer",
        [
            "NVIDIA Telemetri Kapsayıcısı",
            "NVIDIA-Telemetrie-Container",
            "Conteneur de télémétrie NVIDIA",
            "Contenedor de telemetría de NVIDIA",
            "Контейнер телеметрии NVIDIA",
            "NVIDIA 遥测容器服务",
            "NVIDIA テレメトリコンテナ",
        ],
    ),
    (
        "AdobeARMservice",
        [
            "Adobe Acrobat Güncelleme Hizmeti",
            "Adobe Acrobat-Aktualisierungsdienst",
            "Service de mise à jour Adobe Acrobat",
            "Servicio de actualización de Adobe Acrobat",
            "Служба обновления Adobe Acrobat",
            "Adobe Acrobat 更新服务",
            "Adobe Acrobat 更新サービス",
        ],
    ),
    (
        "AGMService",
        [
            "Adobe Orijinal Yazılım Bütünlük Hizmeti",
            "Adobe Genuine Software-Integritätsdienst",
            "Service d'intégrité logicielle Adobe Genuine",
            "Servicio de integridad de software original de Adobe",
            "Служба проверки подлинности ПО Adobe",
            "Adobe 正版软件完整性验证服务",
            "Adobe 正規ソフトウェア整合性サービス",
        ],
    ),
    ("Razer Game Manager Service", RAZER_SYNAPSE_LABELS),
    ("Razer Synapse Service", RAZER_SYNAPSE_LABELS),
    (
        "edgeupdate",
        [
            "Microsoft Edge Güncelleme Hizmeti",
            "Microsoft Edge-Aktualisierungsdienst",
            "Service de mise à jour Microsoft Edge",
            "Servicio de actualización de Microsoft Edge",
            "Служба обновления Microsoft Edge",
            "Microsoft Edge 更新服务",
            "Microsoft Edge 更新サービス",
        ],
    ),
    (
        "gupdate",
        [
            "Google Güncelleme Hizmeti",
            "Google-Aktualisierungsdienst",
            "Service de mise à jour Google",
            "Servicio de actualización de Google",
            "Служба обновления Google",
            "Google 更新服务",
            "Google 更新サービス",
        ],
    ),
];

/// Return localized human-readable description for a known telemetry service name.
///
/// Returns `None` for [`Language::English`] or unknown service names.
pub fn localize_telemetry_service(lang: Language, service_name: &str) -> Option<&'static str> {
    let idx = non_english_idx(lang)?;
    let trimmed = service_name.trim();
    for (svc, vals) in TELEMETRY_SERVICE_TABLE {
        if svc.eq_ignore_ascii_case(trimmed) {
            return Some(vals[idx]);
        }
    }
    None
}

static POWER_SCHEME_TABLE: &[(&str, [&str; 7])] = &[
    (
        "Balanced",
        [
            "Dengeli",
            "Ausbalanciert",
            "Équilibré",
            "Equilibrado",
            "Сбалансированная",
            "平衡",
            "バランス",
        ],
    ),
    (
        "High performance",
        [
            "Yüksek Performans",
            "Höchstleistung",
            "Performances élevées",
            "Alto rendimiento",
            "Высокая производительность",
            "高性能",
            "高パフォーマンス",
        ],
    ),
    (
        "Power saver",
        [
            "Güç Tasarrufu",
            "Energiesparmodus",
            "Économie d'énergie",
            "Economizador",
            "Экономия энергии",
            "节能",
            "省電力",
        ],
    ),
    (
        "Ultimate Performance",
        [
            "Nihai Performans",
            "Ultimative Leistung",
            "Performances optimales",
            "Máximo rendimiento",
            "Максимальная производительность",
            "卓越性能",
            "究極のパフォーマンス",
        ],
    ),
    (
        "WinBlast0 Ultimate Performance",
        [
            "WinBlast0 Nihai Performans",
            "WinBlast0 Ultimative Leistung",
            "WinBlast0 Performances optimales",
            "WinBlast0 Máximo rendimiento",
            "WinBlast0 Максимальная производительность",
            "WinBlast0 卓越性能",
            "WinBlast0 究極のパフォーマンス",
        ],
    ),
];

/// Localize canonical Windows and WinBlast0 power scheme names case-insensitively across all 8 languages.
pub fn localize_power_scheme_name(lang: Language, raw_name: &str) -> &str {
    lookup_text_table(lang, raw_name, POWER_SCHEME_TABLE)
}

static LEDGER_TOKEN_TABLE: &[(&str, [&str; 8])] = &[
    (
        "__ABSENT__",
        [
            "Absent",
            "Yok",
            "Nicht vorhanden",
            "Absent",
            "Ausente",
            "Отсутствует",
            "不存在",
            "未設定",
        ],
    ),
    (
        "__VAL_ABSENT__",
        [
            "Value Absent",
            "Değer Yok",
            "Wert nicht vorhanden",
            "Valeur absente",
            "Valor ausente",
            "Параметр отсутствует",
            "值不存在",
            "値なし",
        ],
    ),
    (
        "installed",
        [
            "Installed",
            "Yüklü",
            "Installiert",
            "Installé",
            "Instalado",
            "Установлено",
            "已安装",
            "インストール済み",
        ],
    ),
    (
        "removed",
        [
            "Removed",
            "Kaldırıldı",
            "Entfernt",
            "Supprimé",
            "Eliminado",
            "Удалено",
            "已移除",
            "削除済み",
        ],
    ),
    (
        "enabled",
        [
            "Enabled",
            "Etkin",
            "Aktiviert",
            "Activé",
            "Activado",
            "Включено",
            "已启用",
            "有効",
        ],
    ),
    (
        "disabled",
        [
            "Disabled",
            "Devre Dışı",
            "Deaktiviert",
            "Désactivé",
            "Desactivado",
            "Отключено",
            "已禁用",
            "無効",
        ],
    ),
    (
        "unparked",
        [
            "Unparked",
            "Park Dışı (Aktif)",
            "Entparkt (Aktiv)",
            "Actif (Déparqué)",
            "Desaparcado (Activo)",
            "Разпарковано",
            "已唤醒 (未休眠)",
            "パーキング解除",
        ],
    ),
    (
        "parked",
        [
            "Parked",
            "Park Edildi",
            "Geparkt",
            "Parqué",
            "Aparcado",
            "Припарковано",
            "已休眠",
            "パーキング中",
        ],
    ),
    (
        "true",
        [
            "True",
            "Doğru",
            "Wahr",
            "Vrai",
            "Verdadero",
            "Истина",
            "真",
            "真",
        ],
    ),
    (
        "false",
        [
            "False",
            "Yanlış",
            "Falsch",
            "Faux",
            "Falso",
            "Ложь",
            "假",
            "偽",
        ],
    ),
];

/// Translate common SQLite ledger value tokens across all 8 languages while preserving
/// numeric, hex, or custom values verbatim.
pub fn localize_ledger_token(lang: Language, token: &str) -> String {
    let trimmed = token.trim();
    for (key, vals) in LEDGER_TOKEN_TABLE {
        if key.eq_ignore_ascii_case(trimmed) {
            return vals.get(lang as usize).copied().unwrap_or(vals[0]).to_string();
        }
    }
    token.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::risk::load_embedded_rules;
    use crate::scanner::telemetry_scanner::KNOWN_TELEMETRY_SERVICES;
    use crate::tools::features_apps::{curated_windows_features_meta, CURATED_WINGET_APPS};

    const NON_ENGLISH_LANGS: [Language; 7] = [
        Language::Turkish,
        Language::German,
        Language::French,
        Language::Spanish,
        Language::Russian,
        Language::Chinese,
        Language::Japanese,
    ];

    #[test]
    fn test_all_218_tweaks_localized_all_7_languages() {
        let tweaks = load_embedded_rules();
        assert_eq!(tweaks.len(), 218, "Expected 218 embedded catalog tweaks");

        for tweak in &tweaks {
            assert!(
                localize_tweak(Language::English, &tweak.id).is_none(),
                "English should return None for {}",
                tweak.id
            );

            for lang in NON_ENGLISH_LANGS {
                let loc = localize_tweak(lang, &tweak.id).unwrap_or_else(|| {
                    panic!("Missing localization for tweak '{}' in {:?}", tweak.id, lang)
                });
                assert!(
                    !loc.title.trim().is_empty(),
                    "Empty localized title for '{}' in {:?}",
                    tweak.id,
                    lang
                );
                assert!(
                    !loc.description.trim().is_empty(),
                    "Empty localized description for '{}' in {:?}",
                    tweak.id,
                    lang
                );
                assert_eq!(
                    loc.broken_features.len(),
                    tweak.valuation.broken_features.len(),
                    "broken_features count mismatch for '{}' in {:?}",
                    tweak.id,
                    lang
                );
                for bf in loc.broken_features {
                    assert!(
                        !bf.trim().is_empty(),
                        "Empty broken_features entry for '{}' in {:?}",
                        tweak.id,
                        lang
                    );
                }
            }
        }
    }

    #[test]
    fn test_categories_and_sub_categories() {
        let categories = [
            "Bloatware",
            "GPU Tuning",
            "Hardware Tuning",
            "OS Settings",
            "Privacy",
            "Registry Lag",
            "Telemetry (Apps)",
            "Telemetry (OS)",
            "Kernel & Memory Optimization",
            "Network Stack Optimization",
            "GPU & Display Pipeline",
            "Storage & IO Optimization",
            "Scheduler & Latency",
        ];
        let sub_categories = [
            "Power",
            "Performance",
            "Power & Clocks",
            "Latency",
            "Compositor",
            "Driver & Clarity",
            "Diagnostics",
        ];

        for lang in Language::ALL {
            for cat in categories {
                let loc = localize_category(lang, cat);
                assert!(!loc.trim().is_empty());
                if lang == Language::English {
                    assert_eq!(loc, cat);
                } else {
                    assert_ne!(loc, cat, "Category '{}' untranslated for {:?}", cat, lang);
                }
            }
            assert_eq!(localize_category(lang, "Custom Category"), "Custom Category");

            for sub in sub_categories {
                let loc = localize_sub_category(lang, sub);
                assert!(!loc.trim().is_empty());
                if lang == Language::English {
                    assert_eq!(loc, sub);
                }
            }
            assert_eq!(localize_sub_category(lang, "Custom Sub"), "Custom Sub");
        }

        assert!(is_privacy_or_telemetry_category("Privacy"));
        assert!(is_privacy_or_telemetry_category("Telemetry (OS)"));
        assert!(is_privacy_or_telemetry_category("Telemetry (Apps)"));
        assert!(!is_privacy_or_telemetry_category("Bloatware"));
        assert!(!is_privacy_or_telemetry_category("GPU Tuning"));
        assert!(!is_privacy_or_telemetry_category("Hardware Tuning"));
        assert!(!is_privacy_or_telemetry_category("OS Settings"));
        assert!(!is_privacy_or_telemetry_category("Registry Lag"));
    }

    #[test]
    fn test_winget_optional_features_and_telemetry_services() {
        assert_eq!(CURATED_WINGET_APPS.len(), 12);
        for app in CURATED_WINGET_APPS {
            assert!(localize_winget(Language::English, app.id).is_none());
            for lang in NON_ENGLISH_LANGS {
                let (cat, desc) = localize_winget(lang, app.id).unwrap_or_else(|| {
                    panic!("Missing WinGet localization for '{}' in {:?}", app.id, lang)
                });
                assert!(!cat.trim().is_empty());
                assert!(!desc.trim().is_empty());
            }
        }

        let feats = curated_windows_features_meta();
        assert_eq!(feats.len(), 10);
        for (name, _, _, _, _) in &feats {
            assert!(localize_optional_feature(Language::English, name).is_none());
            for lang in NON_ENGLISH_LANGS {
                let (disp, desc) = localize_optional_feature(lang, name).unwrap_or_else(|| {
                    panic!("Missing feature localization for '{}' in {:?}", name, lang)
                });
                assert!(!disp.trim().is_empty());
                assert!(!desc.trim().is_empty());
            }
        }
        // Prefix matching when DISM version suffix differs or is omitted
        assert!(localize_optional_feature(Language::Russian, "Browser.InternetExplorer~~~~0.0.99.0").is_some());
        assert!(localize_optional_feature(Language::Japanese, "App.StepsRecorder").is_some());

        assert_eq!(KNOWN_TELEMETRY_SERVICES.len(), 9);
        for (svc, _) in KNOWN_TELEMETRY_SERVICES {
            assert!(localize_telemetry_service(Language::English, svc).is_none());
            for lang in NON_ENGLISH_LANGS {
                let text = localize_telemetry_service(lang, svc).unwrap_or_else(|| {
                    panic!("Missing telemetry service localization for '{}' in {:?}", svc, lang)
                });
                assert!(!text.trim().is_empty());
            }
        }
        for lang in NON_ENGLISH_LANGS {
            assert!(localize_telemetry_service(lang, "Razer Synapse Service").is_some());
        }
    }

    #[test]
    fn test_power_schemes_and_ledger_tokens() {
        let schemes = [
            "balanced",
            "HIGH PERFORMANCE",
            "Power Saver",
            "ultimate performance",
            "WinBlast0 Ultimate Performance",
        ];
        for lang in Language::ALL {
            for s in schemes {
                let loc = localize_power_scheme_name(lang, s);
                assert!(!loc.trim().is_empty());
            }
            assert_eq!(localize_power_scheme_name(lang, "My Custom Plan"), "My Custom Plan");
        }

        let tokens = [
            "__ABSENT__",
            "__VAL_ABSENT__",
            "installed",
            "removed",
            "enabled",
            "disabled",
            "unparked",
            "parked",
            "true",
            "false",
        ];
        for lang in Language::ALL {
            for tok in tokens {
                let loc = localize_ledger_token(lang, tok);
                assert!(!loc.trim().is_empty());
                assert_ne!(loc, "__ABSENT__");
                assert_ne!(loc, "__VAL_ABSENT__");
            }
            assert_eq!(localize_ledger_token(lang, "0x26"), "0x26");
            assert_eq!(localize_ledger_token(lang, "15000"), "15000");
            assert_eq!(localize_ledger_token(lang, "2|delayed=1"), "2|delayed=1");
        }
    }

    #[test]
    fn test_all_32_deep_opt_tweaks_localized_all_7_languages() {
        let deep_opt_toml = include_str!("../../../assets/rules/system_deep_opt.toml");
        let catalog = toml::from_str::<crate::core::risk::TweakCatalog>(deep_opt_toml)
            .expect("system_deep_opt.toml should parse cleanly");
        assert_eq!(catalog.tweaks.len(), 32, "Expected 32 deep optimization tweaks");

        for tweak in &catalog.tweaks {
            // English base lookup
            let en_title = tweak_title(&tweak.id);
            assert!(en_title.is_some(), "Missing English tweak_title for {}", tweak.id);
            let en_desc = tweak_description(&tweak.id);
            assert!(en_desc.is_some(), "Missing English tweak_description for {}", tweak.id);

            for lang in NON_ENGLISH_LANGS {
                let loc = localize_tweak(lang, &tweak.id).unwrap_or_else(|| {
                    panic!("Missing localization for tweak '{}' in {:?}", tweak.id, lang)
                });
                assert!(
                    !loc.title.trim().is_empty(),
                    "Empty localized title for '{}' in {:?}",
                    tweak.id,
                    lang
                );
                assert!(
                    !loc.description.trim().is_empty(),
                    "Empty localized description for '{}' in {:?}",
                    tweak.id,
                    lang
                );
                assert_eq!(
                    loc.broken_features.len(),
                    tweak.valuation.broken_features.len(),
                    "broken_features count mismatch for '{}' in {:?}",
                    tweak.id,
                    lang
                );
                for bf in loc.broken_features {
                    assert!(
                        !bf.trim().is_empty(),
                        "Empty broken_features entry for '{}' in {:?}",
                        tweak.id,
                        lang
                    );
                }
            }
        }
    }


    #[test]
    fn test_all_10_gaming_latency_ultra_tweaks_localized_all_7_languages() {
        let ultra_toml = include_str!("../../../assets/rules/gaming_latency_ultra.toml");
        let catalog = toml::from_str::<crate::core::risk::TweakCatalog>(ultra_toml)
            .expect("gaming_latency_ultra.toml should parse cleanly");
        assert_eq!(catalog.tweaks.len(), 10, "Expected 10 gaming latency ultra tweaks");

        for tweak in &catalog.tweaks {
            // English base lookup
            let en_title = tweak_title(&tweak.id);
            assert!(en_title.is_some(), "Missing English tweak_title for {}", tweak.id);
            let en_desc = tweak_description(&tweak.id);
            assert!(en_desc.is_some(), "Missing English tweak_description for {}", tweak.id);

            for lang in NON_ENGLISH_LANGS {
                let loc = localize_tweak(lang, &tweak.id).unwrap_or_else(|| {
                    panic!("Missing localization for tweak '{}' in {:?}", tweak.id, lang)
                });
                assert!(
                    !loc.title.trim().is_empty(),
                    "Empty localized title for '{}' in {:?}",
                    tweak.id,
                    lang
                );
                assert!(
                    !loc.description.trim().is_empty(),
                    "Empty localized description for '{}' in {:?}",
                    tweak.id,
                    lang
                );
                assert_eq!(
                    loc.broken_features.len(),
                    tweak.valuation.broken_features.len(),
                    "broken_features count mismatch for '{}' in {:?}",
                    tweak.id,
                    lang
                );
                for bf in loc.broken_features {
                    assert!(
                        !bf.trim().is_empty(),
                        "Empty broken_features entry for '{}' in {:?}",
                        tweak.id,
                        lang
                    );
                }
            }
        }
    }

}
