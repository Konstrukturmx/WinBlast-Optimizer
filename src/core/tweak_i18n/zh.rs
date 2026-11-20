//! Simplified Chinese (简体中文) localization table for all 168 WinBlast0 catalog tweaks.

use super::LocalizedTweakText;

pub fn lookup(id: &str) -> Option<LocalizedTweakText> {
    match id {
        "disable_windows_ai_recall_extended" => Some(LocalizedTweakText {
            title: "禁用 Windows Recall、Click to Do 与 Copilot+ AI 智能体",
            description: "通过 Windows 11 24H2+ 引入的组策略键值，屏蔽 Windows Recall 快照捕获、Click to Do、设置 AI 智能体、AI 智能体连接器/工作区及 Copilot Runtime。",
            broken_features: &[
                "Windows Recall 时间线快照与 Click to Do AI 浮层",
            ],
        }),
        "disable_generative_ai_app_access" => Some(LocalizedTweakText {
            title: "阻止应用访问生成式 AI 与系统 AI 模型",
            description: "通过 AppPrivacy 和 CapabilityAccessManager ConsentStore 策略，禁止后台 UWP/WinUI 应用访问本地生成式 AI 和系统 AI 模型能力。",
            broken_features: &[
                "第三方应用无法查询内置 Windows AI 模型接口",
            ],
        }),
        "disable_copilot_shell_and_hardware_key" => Some(LocalizedTweakText {
            title: "禁用 Copilot 任务栏按钮、提示、专用硬件键与麦克风",
            description: "隐藏 Windows Copilot 任务栏按钮与引导提示，禁用键盘专用 Copilot 快捷键映射，屏蔽账户状态 AI 推广，并拒绝 Copilot 和 OfficeHub 访问麦克风。",
            broken_features: &[
                "专用 Copilot 快捷键不会启动 Copilot",
                "Copilot 语音输入已被阻止",
            ],
        }),
        "disable_paint_ai_features" => Some(LocalizedTweakText {
            title: "禁用 Microsoft 画图云端与生成式 AI 功能",
            description: "通过官方策略键值禁用 Windows 11 画图中的 Image Creator、Cocreator、生成式填充、生成式擦除和 AI 移除背景功能。",
            broken_features: &[
                "Microsoft 画图中的 AI 生成工具将被隐藏",
            ],
        }),
        "disable_edge_ai_policies" => Some(LocalizedTweakText {
            title: "禁用 Microsoft Edge Copilot、侧边栏及内置 AI API",
            description: "禁用 12 项 Microsoft Edge AI 策略，包括 Copilot 页面上下文、Hubs 侧边栏、Entra Copilot、内联撰写、AI 历史记录搜索、本地基础模型、DevTools GenAI 与历史记录共享。",
            broken_features: &[
                "Edge Copilot 侧边栏与内联写作辅助已禁用",
            ],
        }),
        "disable_office_ai_copilot" => Some(LocalizedTweakText {
            title: "禁用 Microsoft Office AI 训练与各应用 Copilot",
            description: "禁用 Microsoft 365 AI 训练、联网内容分析、内容安全服务以及 Word、Excel 和 OneNote 中的各应用 Copilot 开关。",
            broken_features: &[
                "Word、Excel 和 OneNote 中的 Copilot 助手已禁用",
            ],
        }),
        "disable_wmi_telemetry_autologgers" => Some(LocalizedTweakText {
            title: "禁用 10 个 WMI 内核遥测自动记录器会话",
            description: "阻止 Windows 启动常驻 ETW 后台 AutoLogger 跟踪会话（Diagtrack-Listener、SQMLogger、DiagLog、Cellcore、DataMarket、AppModel、CloudExperienceHostOobe、LwtNetLog、WdiContextLog、WiFiSession）。",
            broken_features: &[
                "内核 ETW 诊断跟踪日志已禁用",
            ],
        }),
        "block_silent_preinstalled_apps" => Some(LocalizedTweakText {
            title: "阻止静默预装应用以及开始菜单与设置中的推荐内容",
            description: "阻止 Windows ContentDeliveryManager 静默安装 OEM 或 Microsoft Store 推广应用，并隐藏设置、开始菜单和时间线中的推荐内容。",
            broken_features: &[],
        }),
        "disable_extra_telemetry_policies" => Some(LocalizedTweakText {
            title: "强化反馈、应用兼容性清单、OOBE 提示与追踪限制",
            description: "将 MaxTelemetryAllowed 限制为 0，抑制 Windows 反馈 SIUF 提示（删除 PeriodInNanoSeconds），禁用应用兼容性清单/UAR、查找我的设备、欢迎/完成设置弹窗、应用启动追踪、Workplace Join 提示与远程协助。",
            broken_features: &[
                "远程协助邀请功能已禁用",
                "开始菜单的常用/最近启动应用追踪已关闭",
            ],
        }),
        "disable_typing_and_inking_telemetry" => Some(LocalizedTweakText {
            title: "禁用墨迹书写、输入个性化与输入见解遥测",
            description: "限制隐式按键/墨迹数据收集、自定义输入词典的联系人采集、语言诊断日志（TIPC）以及 Windows 输入见解。",
            broken_features: &[
                "个性化输入与手写预测词典已关闭",
            ],
        }),
        "disable_search_cloud_and_history" => Some(LocalizedTweakText {
            title: "禁用 Windows 搜索精选、云端搜索与设备历史记录",
            description: "在不禁用索引服务的前提下，关闭 Windows 搜索精选（动态搜索框）、MSA/AAD 云端内容搜索以及本地设备搜索历史日志记录。",
            broken_features: &[
                "搜索精选每日插图与 OneDrive 云端搜索结果已隐藏",
            ],
        }),
        "disable_wifi_sense_and_location_extras" => Some(LocalizedTweakText {
            title: "禁用 WiFi-Sense 自动连接、地图自动更新与位置权限存储",
            description: "禁用 WiFi 热点自动连接（WcmSvc\\AutoConnectAllowedOEM=0）、Windows 地图离线自动更新以及全局位置功能 ConsentStore 权限。",
            broken_features: &[
                "Windows 地图自动下载与系统级应用位置请求已禁用",
            ],
        }),
        "block_onedrive_kfm_auto_backup" => Some(LocalizedTweakText {
            title: "阻止 OneDrive 自动移动已知文件夹（桌面/文档云同步）",
            description: "设置 KFMBlockOptIn=1，防止 OneDrive 在未征得同意的情况下将桌面、文档和图片文件夹接管并备份至云端。若您仅使用本地存储或手动同步则非常安全。",
            broken_features: &[
                "OneDrive 自动已知文件夹移动向导已禁用",
            ],
        }),
        "disable_bitlocker_auto_encryption" => Some(LocalizedTweakText {
            title: "禁用 Windows BitLocker 自动设备加密",
            description: "设置 PreventDeviceEncryption=1，防止 Windows 在登录 Microsoft 账户或初始化系统时未先备份恢复密钥便静默加密磁盘。",
            broken_features: &[
                "零干预自动 BitLocker 驱动器加密将不会激活",
            ],
        }),
        "disable_extra_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "禁用额外遥测、WER 队列、SQM、电源诊断与地图计划任务",
            description: "禁用附加诊断与后台计划任务：QueueReporting、Sqm-Tasks、MareBackup、StartupAppTask、PcaPatchDbTask、MapsUpdateTask、AnalyzeSystem 和 FamilySafetyMonitor。",
            broken_features: &[
                "自动能效报告与离线地图更新已禁用",
            ],
        }),
        "disable_windows_ai_scheduled_tasks" => Some(LocalizedTweakText {
            title: "禁用 Windows AI Recall 后台计划任务",
            description: "禁用 Windows 11 24H2+ 中引入的 Windows Recall 配置和机器学习后台管线计划任务。",
            broken_features: &[
                "Windows Recall 后台索引任务已禁用",
            ],
        }),
        "disable_office_actions_scheduled_task" => Some(LocalizedTweakText {
            title: "禁用 Microsoft Office AI Actions Server 计划任务",
            description: "禁用 Microsoft Office Actions Server 后台计划任务，减少 Office AI 在后台唤醒 CPU 和占用内存。",
            broken_features: &[],
        }),
        "disable_optional_bloat_services" => Some(LocalizedTweakText {
            title: "禁用闲置可选 Windows 服务（地图、传真、零售演示、钱包、自动时区）",
            description: "禁用 MapsBroker、RetailDemo、Fax、WalletService、WMPNetworkSvc、AJRouter、shpamsvc 和 tzautoupdate，不触及核心系统与 Windows 更新服务。",
            broken_features: &[
                "Windows 传真、离线地图代理、零售演示、WMP 共享与自动时区已禁用",
            ],
        }),
        "disable_xbox_live_services" => Some(LocalizedTweakText {
            title: "禁用 Xbox Live 后台服务（XblAuthManager、GameSave、Networking）",
            description: "为不游玩 Xbox 应用或 Game Pass 游戏的用户禁用 Xbox Live 身份验证管理器、游戏存档与网络后台服务。",
            broken_features: &[
                "Xbox Game Pass 与 Microsoft Store 的 Xbox Live 登录和云存档将停止工作",
            ],
        }),
        "disable_ai_fabric_service" => Some(LocalizedTweakText {
            title: "禁用 Windows AI Fabric 服务 (WSAIFabricSvc)",
            description: "在 Windows 11 24H2+ 上禁用 WSAIFabricSvc 后台本地 AI 编排守护服务。",
            broken_features: &[
                "Copilot+ AI 编排服务已停止",
            ],
        }),
        "deny_global_camera_mic_diagnostics" => Some(LocalizedTweakText {
            title: "全局阻止应用访问摄像头、麦克风、账户信息与诊断数据 [谨慎自选]",
            description: "通过 ConsentStore 全局拒绝所有桌面和 UWP 应用访问摄像头、麦克风、userAccountInformation 和 appDiagnostics。高影响风险：在还原前将导致 Zoom/Discord/OBS 等无法使用摄像头和麦克风。",
            broken_features: &[
                "在重新允许前，将禁用所有应用的摄像头与麦克风权限",
            ],
        }),
        "remove_bing_weather" => Some(LocalizedTweakText {
            title: "卸载 MSN 天气 (Bing Weather)",
            description: "移除 Microsoft Bing 天气 UWP 应用，天气信息可改用浏览器查看。",
            broken_features: &[
                "Bing 天气应用将被卸载",
            ],
        }),
        "remove_bing_news" => Some(LocalizedTweakText {
            title: "卸载 MSN 资讯 (Bing News)",
            description: "移除 Microsoft Bing 资讯 (Microsoft News) UWP 应用。",
            broken_features: &[
                "Bing 资讯 / Microsoft News 应用将被卸载",
            ],
        }),
        "remove_get_help" => Some(LocalizedTweakText {
            title: "卸载获取帮助 (Get Help)",
            description: "移除 Microsoft 获取帮助 UWP 应用，仍可通过网页访问技术支持。",
            broken_features: &[
                "获取帮助应用将被卸载",
            ],
        }),
        "remove_getstarted_tips" => Some(LocalizedTweakText {
            title: "卸载提示 (Get Started)",
            description: "移除 Microsoft 提示 / 入门 (Get Started) UWP 应用。",
            broken_features: &[
                "提示 / 入门应用将被卸载",
            ],
        }),
        "remove_solitaire" => Some(LocalizedTweakText {
            title: "卸载 Microsoft Solitaire Collection 纸牌合集",
            description: "移除内置广告与遥测的 Microsoft Solitaire Collection UWP 应用。",
            broken_features: &[
                "Solitaire Collection 纸牌合集将被卸载",
            ],
        }),
        "remove_people" => Some(LocalizedTweakText {
            title: "卸载人脉 (People) 应用",
            description: "移除 Microsoft 人脉 (People) UWP 应用。",
            broken_features: &[
                "人脉 (People) 应用将被卸载",
            ],
        }),
        "remove_windows_maps" => Some(LocalizedTweakText {
            title: "卸载 Windows 地图",
            description: "移除 Microsoft Windows 地图 UWP 应用。",
            broken_features: &[
                "Windows 地图应用将被卸载",
            ],
        }),
        "remove_groove_music" => Some(LocalizedTweakText {
            title: "卸载 Groove 音乐 (Zune Music)",
            description: "移除 Groove 音乐 (ZuneMusic) UWP 应用，可改用新版媒体播放器或第三方播放器。",
            broken_features: &[
                "Groove 音乐应用将被卸载",
            ],
        }),
        "remove_zune_video" => Some(LocalizedTweakText {
            title: "卸载电影和电视 (Zune Video)",
            description: "移除电影和电视 (ZuneVideo) UWP 应用，可改用 VLC 或其他播放器。",
            broken_features: &[
                "电影和电视应用将被卸载",
            ],
        }),
        "remove_clipchamp" => Some(LocalizedTweakText {
            title: "卸载 Clipchamp 视频编辑器",
            description: "移除 Windows 11 预装的 Clipchamp 视频编辑 UWP 应用。",
            broken_features: &[
                "Clipchamp 视频编辑器将被卸载",
            ],
        }),
        "remove_todos" => Some(LocalizedTweakText {
            title: "卸载 Microsoft To Do",
            description: "移除 Microsoft To Do 待办事项 UWP 应用。",
            broken_features: &[
                "Microsoft To Do 应用将被卸载",
            ],
        }),
        "remove_bing_search_appx" => Some(LocalizedTweakText {
            title: "卸载 Microsoft Bing 搜索 AppX 扩展",
            description: "移除驱动桌面网页小组件及 Windows 搜索网页集成的 Microsoft.BingSearch 商店包。",
            broken_features: &[
                "Bing 搜索 AppX 集成将被移除",
            ],
        }),
        "remove_feedback_hub" => Some(LocalizedTweakText {
            title: "卸载 Microsoft 反馈中心 (Feedback Hub)",
            description: "移除预装的 Windows 反馈中心遥测与问卷调查应用。",
            broken_features: &[
                "反馈中心应用将被移除",
            ],
        }),
        "remove_copilot_appx" => Some(LocalizedTweakText {
            title: "卸载 Microsoft Copilot UWP 应用包",
            description: "卸载独立的 Microsoft.Copilot Microsoft Store 应用包。",
            broken_features: &[
                "Copilot 应用将被移除",
            ],
        }),
        "remove_edge_game_assist" => Some(LocalizedTweakText {
            title: "卸载 Microsoft Edge 游戏助手浮层",
            description: "移除 Edge Game Assist 游戏内浏览器浮层 AppX 应用包。",
            broken_features: &[
                "Edge 游戏助手小组件将被移除",
            ],
        }),
        "remove_dev_home" => Some(LocalizedTweakText {
            title: "卸载 Windows Dev Home 应用",
            description: "移除 Windows 11 预装的 Microsoft.Windows.DevHome 开发者仪表盘应用包。",
            broken_features: &[
                "Dev Home 仪表盘将被移除",
            ],
        }),
        "remove_power_automate_desktop" => Some(LocalizedTweakText {
            title: "卸载 Microsoft Power Automate Desktop",
            description: "移除 Microsoft Power Automate Desktop 自动化流程 (RPA) 应用包。",
            broken_features: &[
                "Power Automate 桌面流应用将被移除",
            ],
        }),
        "remove_office_hub" => Some(LocalizedTweakText {
            title: "卸载 Microsoft 365 Copilot (OfficeHub)",
            description: "移除预装的 Microsoft 365 Office Hub 推广启动器应用。",
            broken_features: &[
                "Office 推广启动器将被移除（不会卸载桌面版 Office）",
            ],
        }),
        "remove_family_safety" => Some(LocalizedTweakText {
            title: "卸载 Microsoft Family Safety 家庭安全应用",
            description: "移除面向个人用户的 Microsoft Family Safety UWP 应用。",
            broken_features: &[
                "Family Safety UWP 应用将被移除",
            ],
        }),
        "remove_3d_viewer_and_mixed_reality" => Some(LocalizedTweakText {
            title: "卸载 3D 查看器与混合现实门户",
            description: "移除旧版 Microsoft 3D Viewer 和 Mixed Reality Portal 应用包。",
            broken_features: &[
                "旧版 3D 查看器与 Windows 混合现实门户将被移除",
            ],
        }),
        "remove_outlook_for_windows" => Some(LocalizedTweakText {
            title: "卸载新版 Outlook for Windows（WebView 客户端）",
            description: "移除预装的 Microsoft.OutlookForWindows 网页封装应用。",
            broken_features: &[
                "新版 Outlook UWP 客户端将被移除",
            ],
        }),
        "remove_your_phone_link" => Some(LocalizedTweakText {
            title: "卸载手机连接 (YourPhone / Phone Link)",
            description: "卸载 Microsoft.YourPhone 手机桌面同步配套应用。",
            broken_features: &[
                "Android/iOS 手机连接桌面同步功能将被移除",
            ],
        }),
        "remove_sound_recorder_and_sticky_notes" => Some(LocalizedTweakText {
            title: "卸载 Windows 录音机与便笺 (Sticky Notes)",
            description: "移除录音机与便笺 Microsoft Store 应用包。",
            broken_features: &[
                "内置录音机与便笺应用将被移除",
            ],
        }),
        "remove_ms_teams_and_quick_assist" => Some(LocalizedTweakText {
            title: "卸载个人版 Microsoft Teams 与快速助手",
            description: "移除个人版 MSTeams 聊天应用与快速助手 (Quick Assist) 远程支持应用。",
            broken_features: &[
                "个人版 Teams 与快速助手支持工具将被移除",
            ],
        }),
        "disable_recall_optional_feature" => Some(LocalizedTweakText {
            title: "卸载 Windows Recall 可选功能 (DISM)",
            description: "在系统维护层级禁用可选的“Recall”Windows 功能包。",
            broken_features: &[
                "Windows Recall 功能二进制组件已干净禁用",
            ],
        }),
        "remove_legacy_steps_recorder_ie_wordpad" => Some(LocalizedTweakText {
            title: "禁用旧版步骤记录器与 Internet Explorer 11 功能",
            description: "通过 DISM 移除旧版步骤记录器 (App.StepsRecorder) 和 Internet Explorer 模式功能。",
            broken_features: &[
                "步骤记录器 (psr.exe) 与 IE11 模式将被移除",
            ],
        }),
        "enable_hags" => Some(LocalizedTweakText {
            title: "开启硬件加速 GPU 调度 (HAGS)",
            description: "开启硬件加速 GPU 调度，允许显卡直接管理自身的显存 (VRAM) 调度。可降低 GPU 密集型负载的延迟和 CPU 开销。需要受支持的显卡（NVIDIA GTX 1000+、AMD RX 5000+）及对应驱动。",
            broken_features: &[
                "部分较老的游戏或 GPU 工具可能出现不稳定现象",
            ],
        }),
        "disable_mpo" => Some(LocalizedTweakText {
            title: "禁用多平面叠加 (MPO)",
            description: "禁用多平面叠加硬件合成器功能。在多种显卡/驱动组合（尤其是混合刷新率多显示器环境）下，MPO 常导致画面卡顿、黑屏和帧生成时间异常。",
            broken_features: &[
                "笔记本电脑上 GPU 合成器待机功耗略微增加",
            ],
        }),
        "disable_core_parking" => Some(LocalizedTweakText {
            title: "禁用 CPU 核心休眠 (Core Parking)",
            description: "将必须保持唤醒的处理器核心最小百分比设为 100%，确保所有核心随时待命，彻底消除核心唤醒延迟，但会增加待机功耗。",
            broken_features: &[
                "待机功耗增加",
                "笔记本电脑电池续航缩短",
            ],
        }),
        "disable_cstates" => Some(LocalizedTweakText {
            title: "禁用 CPU 空闲 C-States 深度节能状态",
            description: "通过将空闲状态策略设为性能模式，阻止处理器进入深度空闲 C-States (C6/C7/C8)。可消除 CPU 电源状态切换时的延迟尖峰，但会增加空闲功耗。",
            broken_features: &[
                "待机功耗显著升高",
                "笔记本电脑电池续航缩短",
                "待机状态下 CPU 温度升高",
            ],
        }),
        "gpu_shader_cache_size" => Some(LocalizedTweakText {
            title: "延长 GPU TDR 超时时间并防止驱动重置",
            description: "将 WDDM 超时检测与恢复 (TdrDelay) 间隔从 2 秒延长至 10 秒，防止大型 DirectX 着色器编译期间发生误判性显卡驱动重置与卡顿。",
            broken_features: &[
                "当 GPU 真正发生挂起时，显示驱动恢复最多等待 10 秒",
            ],
        }),
        "enable_game_mode" => Some(LocalizedTweakText {
            title: "启用 Windows 游戏模式 (Game Mode)",
            description: "确保启用 Windows 游戏模式（AutoGameModeEnabled=1、AllowAutoGameMode=1），在游戏期间优先分配资源并抑制 Windows 更新与重启通知。",
            broken_features: &[],
        }),
        "disable_fullscreen_optimizations" => Some(LocalizedTweakText {
            title: "全局禁用全屏优化",
            description: "为所有应用程序全局禁用 Windows 全屏优化（无边框全屏混合模式），强制使用真正的独占全屏模式，可降低旧游戏的输入延迟与帧时间波动。",
            broken_features: &[
                "全屏游戏中按 Alt-Tab 切换窗口会变慢",
                "独占全屏模式下 Xbox Game Bar 浮层将无法显示",
            ],
        }),
        "4433ce93-b632-4d67-ae71-0241c31099c0" => Some(LocalizedTweakText {
            title: "禁用 AMD ULPS 超低功耗状态",
            description: "禁用 AMD Radeon 驱动栈中的超低功耗状态 (ULPS)，防止副 GPU 核心或休眠状态唤醒时在 3D 游戏中引发微卡顿与帧延迟尖峰。",
            broken_features: &[
                "AMD 双显卡 / CrossFire 笔记本在空闲时功耗略微上升",
            ],
        }),
        "8a4b4c4d-000e-42db-8bd7-293067f8fae7" => Some(LocalizedTweakText {
            title: "禁用 AMD PCIe ASPM 节能",
            description: "禁用 AMD 显卡的 PCIe 活动状态电源管理 (ASPM)，防止帧渲染间隙 PCIe 总线链路切入低功耗 L0s/L1 状态，降低 PCIe 传输抖动。",
            broken_features: &[
                "PCIe 总线空闲功耗增加",
            ],
        }),
        "a3836587-c461-4beb-9c4c-88493b36c138" => Some(LocalizedTweakText {
            title: "禁用 AMD Power Gating 电源门控",
            description: "禁用针对 AMD 计算单元和渲染后端的激进动态电源门控（DisablePowerGating=1、PP_GPUPowerDownEnabled=0），保持持续稳定的核心频率不降频。",
            broken_features: &[
                "低负载时 GPU 核心温度更高",
            ],
        }),
        "db379bb5-fc8c-4ad3-be32-11be09e76fba" => Some(LocalizedTweakText {
            title: "禁用 AMD 视频时钟门控 (Video Clock Gating)",
            description: "阻止对 AMD VCE/UVD 多媒体和硬件显示引擎进行时钟和电源门控（DisableVCEPowerGating=1、DisableVceClockGating=1、EnableUvdClockGating=0、EnableVceSwClockGating=0），消除 DPC 延迟尖峰。",
            broken_features: &[],
        }),
        "df8739ff-e322-4958-b369-df584bb9b0c1" => Some(LocalizedTweakText {
            title: "禁用 Intel 异步画面翻转 (Async Flips)",
            description: "在 Intel 核显适配器类注册表项中设置 Display1_DisableAsyncFlips=1，强制同步显示引擎翻转，消除画面呈现队列抖动。",
            broken_features: &[],
        }),
        "ebfb0ded-5d1a-4cd1-b5c0-4f7018dd6054" => Some(LocalizedTweakText {
            title: "禁用 Intel 自适应垂直同步 (Adaptive V-Sync)",
            description: "在 Intel 核显适配器类注册表项中设置 AdaptiveVsyncEnable=0，防止帧率低于显示器刷新率时触发自适应刷新节流。",
            broken_features: &[],
        }),
        "fa58802f-b98c-4e98-aa67-067c5a394296" => Some(LocalizedTweakText {
            title: "禁用 NVIDIA 动态 P-State 性能状态切换",
            description: "设置 DisableDynamicPstate=1 将 NVIDIA GPU 锁定在固定性能状态，防止核心频率频繁波动，代价是待机功耗上升。",
            broken_features: &[
                "增加 GPU 待机功耗与发热量",
            ],
        }),
        "9e7702be-9e03-4c96-a316-c4e435cb993e" => Some(LocalizedTweakText {
            title: "禁用 NVIDIA 异步 P-State 切换",
            description: "设置 DisableAsyncPstates=1 禁用 NVIDIA 异步电源状态切换，为 VR 和实时应用提供确定性性能。",
            broken_features: &[
                "可能阻止视频播放期间 GPU 显存降频",
            ],
        }),
        "5f1a2b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c" => Some(LocalizedTweakText {
            title: "开启 NVIDIA PowerMizer 最高性能模式",
            description: "在接通交流电源时强制启用 PowerMizer 最高性能模式，将 PCIe 总线与显存控制器锁定在全速状态，避免降频。",
            broken_features: &[
                "GPU 温度和待机风扇转速升高",
            ],
        }),
        "disable_animations" => Some(LocalizedTweakText {
            title: "禁用窗口动画效果",
            description: "禁用窗口最小化和最大化时的动画效果，降低界面操作的主观延迟感。",
            broken_features: &[],
        }),
        "disable_transparency" => Some(LocalizedTweakText {
            title: "禁用系统透明效果",
            description: "禁用任务栏、开始菜单和操作中心的亚克力/透明特效，释放 GPU 合成性能开销。",
            broken_features: &[
                "任务栏和开始菜单将不再显示半透明磨砂效果",
            ],
        }),
        "disable_ntfs_last_access" => Some(LocalizedTweakText {
            title: "全局禁止 Windows 应用在后台运行",
            description: "阻止 UWP / Microsoft Store 应用在最小化或关闭后静默唤醒、轮询网络并占用 CPU 与内存。",
            broken_features: &[
                "商店应用在关闭后将停止后台同步",
            ],
        }),
        "disable_nagle_algorithm" => Some(LocalizedTweakText {
            title: "禁用 MSMQ Nagle 算法 (TCPNoDelay)",
            description: "专门为 Microsoft 消息队列（MSMQ\\Parameters\\TCPNoDelay）禁用 Nagle 算法，降低 MSMQ 小消息负载的排队延迟。",
            broken_features: &[
                "小数据包场景下的网络带宽占用可能略微增加",
            ],
        }),
        "disable_sticky_keys" => Some(LocalizedTweakText {
            title: "禁用粘滞键与辅助功能快捷键",
            description: "禁用连按 5 次 Shift 触发的粘滞键弹窗、筛选键以及切换键提示音，防止竞技游戏中意外弹窗或打断输入。",
            broken_features: &[
                "辅助功能快捷键（连按 5 次 Shift、长按右 Shift 8 秒）已禁用",
            ],
        }),
        "disable_mouse_shadow" => Some(LocalizedTweakText {
            title: "禁用桌面图标标签阴影与资源管理器半透明框选",
            description: "禁用桌面图标文字投影以及资源管理器拖拽框选矩形的半透明 Alpha 混合，减轻 DWM GPU 合成工作量。",
            broken_features: &[],
        }),
        "high_performance_power_plan" => Some(LocalizedTweakText {
            title: "将处理器性能提升模式设为“积极”",
            description: "通过 PowerSchemes PerfBoostMode 将交流电源下的处理器性能提升模式设为“积极”(Aggressive)，防止交互负载下睿频受限。",
            broken_features: &[
                "待机功耗增加",
                "笔记本电脑发热量上升",
            ],
        }),
        "disable_bing_search_start" => Some(LocalizedTweakText {
            title: "禁用开始菜单中的 Bing 网页搜索",
            description: "阻止 Windows 搜索将每次按键发送至 Bing 服务器，使开始菜单搜索应用和本地文件即时响应且零云端网络延迟。",
            broken_features: &[
                "开始菜单中将不再显示网页搜索结果",
            ],
        }),
        "disable_windows_copilot" => Some(LocalizedTweakText {
            title: "禁用 Windows Copilot AI 集成",
            description: "通过组策略在全系统禁用 Windows Copilot，停止 Copilot 后台常驻代理、隐藏任务栏按钮并拦截 AI 网络请求。",
            broken_features: &[
                "Windows Copilot 侧边栏与快捷键 (Win+C) 将被禁用",
            ],
        }),
        "disable_game_bar_presence_writer" => Some(LocalizedTweakText {
            title: "禁用 Game Bar Presence、Nexus 启动器与启动提示",
            description: "通过用户可写的 HKCU\\Software\\Microsoft\\GameBar 键值禁用手柄 Nexus 按钮启动和 Game Bar 启动提示面板，消除浮层挂钩开销且不会触发 TrustedInstaller ACL 权限错误。",
            broken_features: &[
                "Xbox Guide Nexus 按钮启动与 Game Bar 启动面板已禁用",
            ],
        }),
        "disable_widgets_news" => Some(LocalizedTweakText {
            title: "禁用 Windows 小组件与 MSN 新闻资讯流",
            description: "禁用任务栏小组件图标，阻止后台联网拉取 MSN 新闻资讯、热门话题和天气小组件。",
            broken_features: &[
                "任务栏小组件弹出面板将被禁用",
            ],
        }),
        "gamedvr_config_store" => Some(LocalizedTweakText {
            title: "禁用 Xbox GameDVR 录制浮层",
            description: "禁用后台游戏视频录制和 Xbox Game Bar 挂钩注入，消除帧时间下跌与微卡顿。",
            broken_features: &[
                "Xbox Game Bar 后台即时精彩回放录制功能将被禁用",
            ],
        }),
        "gamedvr_app_capture" => Some(LocalizedTweakText {
            title: "禁用 Windows GameDVR 应用捕获",
            description: "禁用 Windows GameDVR 子系统中的后台音视频录制缓冲区。",
            broken_features: &[],
        }),
        "gamedvr_policy_disable" => Some(LocalizedTweakText {
            title: "通过系统策略禁用 GameDVR",
            description: "系统级组策略锁定，防止 Windows 在后台重新启用 GameDVR 捕获服务。",
            broken_features: &[],
        }),
        "consumer_features_disable" => Some(LocalizedTweakText {
            title: "禁用 Windows 消费者云功能（自动推广应用）",
            description: "防止 Windows 在系统维护期间自动下载并安装第三方赞助应用和游戏（例如 TikTok、Disney、Candy Crush）。",
            broken_features: &[],
        }),
        "disable_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "禁用反馈、Autochk 与地图遥测计划任务",
            description: "禁用 Windows 反馈中心 SIUF 诊断提示、Autochk 遥测代理以及后台地图更新通知任务（整理自 Sophia Script）。",
            broken_features: &[
                "反馈中心主动问卷弹窗通知将被禁用",
            ],
        }),
        "disable_hpet_synthetic_timer" => Some(LocalizedTweakText {
            title: "禁用动态时钟滴答（合成内核定时器）",
            description: "禁用 Windows NT 内核中的动态滴答省电模式，防止合成定时器时钟漂移，强制使用高频率 Invariant TSC 定时器以实现超低 DPC 延迟。",
            broken_features: &[
                "待机时 CPU 功耗略微增加",
            ],
        }),
        "optimize_tcp_congestion_tuning" => Some(LocalizedTweakText {
            title: "优化网络 TCP 拥塞控制与窗口自动调谐",
            description: "启用 RFC 1323 高性能 TCP 时间戳并禁用 Web 服务动态发现 (WSD) 广播通信，最小化联机游戏延迟与数据包序列化卡顿。",
            broken_features: &[],
        }),
        "disable_windows_error_reporting" => Some(LocalizedTweakText {
            title: "禁用 Windows 错误报告组策略与数据上传",
            description: "启用 Windows 错误报告组策略锁定，禁止向 Microsoft 服务器发送额外的崩溃转储诊断数据。",
            broken_features: &[
                "自动错误崩溃报告与遥测上传已被阻止",
            ],
        }),
        "disable_bing_search_start_menu" => Some(LocalizedTweakText {
            title: "禁用开始菜单推荐与 Iris 推广广告",
            description: "移除 Windows 10/11 开始菜单中的赞助网页/账户推荐、Iris 提示以及“最近添加”杂乱项目。",
            broken_features: &[
                "开始菜单中的推广推荐内容已隐藏",
            ],
        }),
        "disable_cortana_autostart" => Some(LocalizedTweakText {
            title: "禁用当前用户 Cortana 搜索与任务栏按钮",
            description: "禁用当前用户的 Cortana 激活与搜索位置权限，并隐藏任务栏上的 Cortana 入口。",
            broken_features: &[
                "当前用户的 Cortana 任务栏搜索集成将被禁用",
            ],
        }),
        "enable_long_file_paths" => Some(LocalizedTweakText {
            title: "启用 NTFS Win32 长文件路径（>260 字符）",
            description: "通过 LongPathsEnabled=1 解除 Windows 10/11 传统的 260 字符 MAX_PATH 路径限制，防止编译构建与解压归档时出现路径截断错误。",
            broken_features: &[],
        }),
        "disable_background_search_toggle_and_app_policy" => Some(LocalizedTweakText {
            title: "禁止应用后台执行并关闭搜索后台开关",
            description: "设置 AppPrivacy\\LetAppsRunInBackground=2（强制拒绝）并关闭 Search\\BackgroundAppGlobalToggle=0，使闲置的 UWP/Store 应用无法在后台消耗 CPU 或内存。",
            broken_features: &[
                "应用商店和邮件等 UWP 应用关闭后将不会后台同步",
            ],
        }),
        "extend_webclient_file_size_limit" => Some(LocalizedTweakText {
            title: "将 WebClient / WebDAV 文件大小上限提升至最大 (4 GB)",
            description: "将 Windows WebClient 服务的下载限制从 50 MB 提高至 4 GB 上限 (0xFFFFFFFF)，解决挂载 WebDAV 或网络共享时的复制失败问题。",
            broken_features: &[],
        }),
        "reduce_services_pipe_timeout" => Some(LocalizedTweakText {
            title: "将服务启动与管道关闭超时缩短至 15 秒",
            description: "将 ServicesPipeTimeout 从默认的 30000 毫秒（30 秒）降至 15000 毫秒（15 秒），使卡死的后台服务在开机和关机时更快释放。",
            broken_features: &[
                "启动耗时超过 15 秒的极慢自定义数据库服务可能会超时",
            ],
        }),
        "mmcss_scheduler_full_optimization" => Some(LocalizedTweakText {
            title: "MMCSS 多媒体调度器全套优化 (NoLazyMode、AlwaysOn、高 SFIO)",
            description: "配置 Multimedia\\SystemProfile NoLazyMode=1、AlwaysOn=1，并将 Games Tasks 调度类别与 SFIO 优先级设为 'High'，实现无中断的多媒体与游戏线程调度。",
            broken_features: &[],
        }),
        "enable_directx_windowed_game_optimizations" => Some(LocalizedTweakText {
            title: "启用 DirectX 窗口化翻转模型与 VRR 优化（无损合并）",
            description: "在 DirectXUserGlobalSettings 中安全合并 SwapEffectUpgradeEnable=1 与 VRROptimizeEnable=1，不会覆盖您原有的单游戏 GPU 或 AutoHDR 配置。",
            broken_features: &[],
        }),
        "disable_hardware_overlays_and_min_fps" => Some(LocalizedTweakText {
            title: "禁用内核硬件叠加与 DWM 最低帧率节流",
            description: "设置 GraphicsDrivers\\DisableOverlays=1 和 Dwm\\OverlayMinFPS=0，防止低刷新率下 DWM 硬件叠加层提升停顿与卡顿。",
            broken_features: &[
                "笔记本电脑待机时的 DWM 合成器功耗略微升高",
            ],
        }),
        "nvidia_restore_legacy_sharpening" => Some(LocalizedTweakText {
            title: "恢复传统 NVIDIA 驱动图像锐化策略 (EnableGR535=0)",
            description: "设置 Global\\FTS\\EnableGR535=0，使 NVIDIA 控制面板显示经典低开销的“图像锐化”选项，而不强制启用新版浮层滤镜路径。",
            broken_features: &[],
        }),
        "nvidia_dlss_debug_indicator" => Some(LocalizedTweakText {
            title: "显示 NVIDIA DLSS 版本与预设调试水印叠加",
            description: "设置 Global\\NGXCore\\ShowDlssIndicator=1024，在 DirectX/Vulkan 游戏画面上实时显示当前启用的 DLSS SDK 版本、分辨率缩放与预设。",
            broken_features: &[
                "在受支持的游戏中左下角将显示 DLSS 诊断水印",
            ],
        }),
        "disable_game_bar_tips_and_capture" => Some(LocalizedTweakText {
            title: "禁用 Game Bar 手柄 Nexus 启动、提示与历史后台录制",
            description: "关闭 UseNexusForGameBarEnabled（Xbox 导航键启动器）、ShowStartupPanel 提示以及后台 HistoricalCaptureEnabled 环形缓冲区录制。",
            broken_features: &[
                "手柄上的 Nexus 按钮将不会触发 Game Bar 浮层",
            ],
        }),
        "optimize_mouse_thresholds_and_hover" => Some(LocalizedTweakText {
            title: "清零鼠标加速阈值并启用 10ms 极速悬停响应",
            description: "设置 MouseThreshold1='0'、MouseThreshold2='0' 与 MouseHoverTime='10' ms，让指针加速曲线完全平直且任务栏与提示框悬停即刻响应。",
            broken_features: &[],
        }),
        "disable_extra_accessibility_hotkeys" => Some(LocalizedTweakText {
            title: "禁用鼠标键、高对比度与讲述人误触快捷键",
            description: "禁用游戏过程中极易误触的 Alt+Shift+NumLock（鼠标键）、Alt+Shift+PrtScn（高对比度）以及 Win+Ctrl+Enter（讲述人）快捷键弹窗。",
            broken_features: &[
                "辅助功能键盘快捷键需前往“设置”应用中手动切换",
            ],
        }),
        "disable_smart_multi_homed_name_resolution" => Some(LocalizedTweakText {
            title: "禁用 Windows 智能多宿主 DNS 名称解析 (SMHNR)",
            description: "设置 DNSClient\\DisableSmartNameResolution=1，让 Windows 仅通过主网络接口查询 DNS，不再向 VPN 和虚拟网卡广播并行 DNS 请求。",
            broken_features: &[
                "分流隧道 (Split-tunnel) VPN 需正确配置网卡 DNS 跃点指标",
            ],
        }),
        "harden_windows_update_ux_and_store" => Some(LocalizedTweakText {
            title: "阻止 WU 覆盖驱动、意外更新重启及商店自动下载",
            description: "在质量更新期间排除显卡/外设驱动推送，阻止已登录状态下强制重启，解锁长达 3650 天的更新暂停时长，并关闭商店后台静默下载（同时保留安全更新功能）。",
            broken_features: &[
                "Microsoft Store 应用更新需要手动检查",
            ],
        }),
        "power_settings_pcie_usb_nvme_epb" => Some(LocalizedTweakText {
            title: "当前电源计划超低延迟调优 (PCIe ASPM、USB 挂起、NVMe、EPP、电源节流)",
            description: "通过原生 Windows powrprof API 作用于当前活动电源方案（兼容平衡、高性能及自定义方案），关闭交流电下的 PCIe ASPM、USB 选择性暂停、USB 3 链路节能、主/副 NVMe 空闲休眠、将 CPU 能效偏好 EPP 设为 0，并禁用内核电源节流 (PowerThrottling)。",
            broken_features: &[
                "台式机及插电状态下的笔记本电脑交流待机功耗升高",
            ],
        }),
        "disable_storage_sense_policy" => Some(LocalizedTweakText {
            title: "禁用存储感知自动后台清理扫描",
            description: "阻止 Windows 存储感知在用户交互会话期间自动在后台执行磁盘清理扫描。",
            broken_features: &[
                "临时文件与回收站需要通过“清理与应用”手动清理",
            ],
        }),
        "consolidate_svchost_split_threshold" => Some(LocalizedTweakText {
            title: "按物理内存动态合并服务宿主进程 (SvcHostSplitThresholdInKB) [自选]",
            description: "根据检测到的系统内存 (TotalRamInKb) 动态计算 SvcHostSplitThresholdInKB，减少 8GB+ 内存设备上的进程总数和虚拟内存开销。注意：会降低单服务的进程崩溃隔离性。",
            broken_features: &[
                "共享 svchost 的崩溃隔离性降低；任务管理器中显示的分组服务进程减少",
            ],
        }),
        "disable_vbs_deviceguard" => Some(LocalizedTweakText {
            title: "禁用基于虚拟化的安全性 (VBS) [安全权衡自选]",
            description: "禁用 EnableVirtualizationBasedSecurity 与 RequirePlatformSecurityFeatures。在受虚拟化开销影响的 CPU 上可提升 5–15% 游戏帧率，代价是削弱内核虚拟化安全边界。",
            broken_features: &[
                "禁用 VBS 安全隔离；部分反作弊系统或 Windows 安全中心可能会提示警告",
            ],
        }),
        "disable_hvci_memory_integrity" => Some(LocalizedTweakText {
            title: "禁用内核隔离 / 内存完整性 (HVCI) [安全权衡自选]",
            description: "禁用虚拟机监控程序保护的代码完整性 (HVCI)。在 HVCI 影响帧时间的竞技游戏中可降低 DPC 与中断延迟，代价是关闭内核驱动代码签名强制检查。",
            broken_features: &[
                "Windows 安全中心的内核隔离 / 内存完整性将显示“已关闭”警告",
            ],
        }),
        "disable_prefetch_and_sysmain" => Some(LocalizedTweakText {
            title: "针对高速 NVMe SSD 禁用内核预读与 SysMain (Superfetch) [自选]",
            description: "在高速 NVMe 固态硬盘上禁用应用/开机预读器 (Prefetcher) 并停止 SysMain 内存预加载，避免多余的后台磁盘 I/O。",
            broken_features: &[
                "使用传统 SATA 机械硬盘时，程序冷启动速度可能略微变慢",
            ],
        }),
        "disable_advertising_id" => Some(LocalizedTweakText {
            title: "禁用广告 ID",
            description: "禁用 Windows 提供给应用用于定向广告与跨应用追踪的每用户广告 ID。",
            broken_features: &[
                "应用将无法获取唯一广告 ID",
            ],
        }),
        "disable_cortana" => Some(LocalizedTweakText {
            title: "禁用 Cortana",
            description: "通过组策略禁用 Cortana，阻止其运行、监听并将数据发送至 Microsoft 云服务。",
            broken_features: &[
                "Cortana 语音助手将被禁用",
            ],
        }),
        "disable_activity_history" => Some(LocalizedTweakText {
            title: "禁用活动历史记录 / 时间线 (Timeline)",
            description: "禁用 Windows 活动历史记录（时间线）功能，阻止 Windows 收集和存储您的操作历史、访问的网站及打开的文档记录。",
            broken_features: &[
                "任务视图中的时间线/活动历史记录将保持为空",
            ],
        }),
        "disable_online_speech_recognition" => Some(LocalizedTweakText {
            title: "禁用联机语音识别",
            description: "禁用基于云端的联机语音识别服务，语音数据不会上传至 Microsoft 服务器进行处理。",
            broken_features: &[
                "基于云端的语音识别将无法使用；本地/离线语音识别不受影响",
            ],
        }),
        "disable_tailored_experiences" => Some(LocalizedTweakText {
            title: "禁用定制体验",
            description: "阻止 Microsoft 利用诊断数据在 Windows 中推送个性化提示、广告与推荐内容。",
            broken_features: &[
                "个性化的 Windows 建议与提示将被禁用",
            ],
        }),
        "disable_windows_recall" => Some(LocalizedTweakText {
            title: "禁用 Windows Recall（AI 快照回顾）",
            description: "禁用 Windows 11 24H2 引入的 Windows Recall 功能，阻止其定期截取并由 AI 索引您的屏幕活动快照。需要系统版本 26100+。",
            broken_features: &[
                "Windows Recall AI 快照功能将被禁用",
            ],
        }),
        "disable_cloud_content_spotlight" => Some(LocalizedTweakText {
            title: "禁用云端内容与聚焦 (Spotlight) 推荐",
            description: "禁用 Windows 聚焦、设置中的推荐内容以及通过云端推送的预装应用建议。",
            broken_features: &[
                "锁屏界面 Windows 聚焦图片将被禁用",
                "开始菜单中将不再出现推荐应用",
            ],
        }),
        "disable_location_tracking" => Some(LocalizedTweakText {
            title: "禁用位置跟踪",
            description: "全局禁用 Windows 定位服务，阻止应用程序与操作系统访问您的地理位置信息。",
            broken_features: &[
                "需要定位权限的应用（如地图、天气）将无法获取您的位置",
            ],
        }),
        "disable_activity_cloud_sync" => Some(LocalizedTweakText {
            title: "禁用云剪贴板与跨设备同步",
            description: "通过组策略禁用 Windows 剪贴板历史记录持久化与跨设备云剪贴板同步，确保复制的密码和敏感数据仅保留在本地。",
            broken_features: &[
                "Win+V 剪贴板历史记录与跨设备粘贴同步将被禁用",
            ],
        }),
        "disable_cdp_service" => Some(LocalizedTweakText {
            title: "禁用连接设备平台服务 (CDPUserSvc)",
            description: "禁用连接设备平台服务 (CDPUserSvc)，阻止跨设备遥测、应用同步广播以及后台活动信标。",
            broken_features: &[
                "就近共享及与移动设备的跨设备同步将被禁用",
            ],
        }),
        "spotlight_desktop_disable" => Some(LocalizedTweakText {
            title: "禁用 Windows 聚焦锁屏壁纸下载",
            description: "禁用后台自动下载 Windows 聚焦锁屏壁纸与 Microsoft 推广图片。",
            broken_features: &[],
        }),
        "spotlight_features_disable" => Some(LocalizedTweakText {
            title: "禁用锁屏提示与广告推送",
            description: "禁用通过 Content Delivery Manager 推送的锁屏提示、建议与 Microsoft Store 赞助应用广告。",
            broken_features: &[],
        }),
        "win32_priority_separation" => Some(LocalizedTweakText {
            title: "Win32PrioritySeparation — 前台窗口响应增强",
            description: "将调度器时间片设为短可变并高优先响应前台（0x26 = 38），优先保障前台应用程序，降低游戏与交互任务中的输入延迟。",
            broken_features: &[],
        }),
        "menu_show_delay" => Some(LocalizedTweakText {
            title: "将菜单显示延迟降至 0 ms",
            description: "移除鼠标悬停打开菜单前的人为等待延迟。默认值为 400 ms；设为 0 可实现菜单瞬间弹出。",
            broken_features: &[],
        }),
        "mmcss_gpu_priority" => Some(LocalizedTweakText {
            title: "MMCSS — 提升游戏与专业音频线程优先级",
            description: "提高多媒体类调度服务 (MMCSS) 中 Games 和 Pro Audio 任务的线程优先级，确保音频与实时游戏线程获得更高调度权重。",
            broken_features: &[],
        }),
        "mouse_data_queue_size" => Some(LocalizedTweakText {
            title: "增大鼠标数据队列缓冲区大小",
            description: "将鼠标输入数据队列从默认的 100 项扩容至 256 项缓冲，降低高回报率游戏鼠标在高负载下丢帧或丢输入事件的概率。",
            broken_features: &[],
        }),
        "keyboard_response" => Some(LocalizedTweakText {
            title: "优化键盘输入响应速度",
            description: "通过注册表将键盘重复延迟设为最短、重复速率设为最快，同时扩大键盘数据队列缓冲区大小。",
            broken_features: &[],
        }),
        "disable_wer_queue" => Some(LocalizedTweakText {
            title: "禁用 Windows 错误报告队列 (WER)",
            description: "禁用 Windows 错误报告 (WER) 队列，防止程序崩溃后系统在后台写入转储数据与报告。",
            broken_features: &[
                "崩溃报告将不会发送给 Microsoft",
            ],
        }),
        "auto_end_tasks" => Some(LocalizedTweakText {
            title: "关机时自动结束无响应任务",
            description: "关机或注销时自动终止无响应程序而不再弹窗等待用户确认，并缩短 WaitToKillAppTimeout 与 HungAppTimeout 以加快关机注销速度。",
            broken_features: &[
                "关机时无响应应用中未保存的工作可能会丢失",
            ],
        }),
        "network_throttling_disable" => Some(LocalizedTweakText {
            title: "禁用多媒体网络节流指数 (NetworkThrottlingIndex)",
            description: "禁用 Windows 多媒体网络限流（将 NetworkThrottlingIndex 设为 0xFFFFFFFF），防止高网络流量或游戏过程中出现丢包与延迟飙升。",
            broken_features: &[],
        }),
        "system_responsiveness_gaming" => Some(LocalizedTweakText {
            title: "最大化游戏与前台系统响应度",
            description: "将 SystemResponsiveness 配置为 0%，把 100% 的 CPU 处理周期分配给前台游戏与应用，不再为后台系统任务强制预留 20%。",
            broken_features: &[],
        }),
        "ntfs_disable_last_access" => Some(LocalizedTweakText {
            title: "禁用 NTFS 最后访问时间戳更新",
            description: "阻止 NTFS 文件系统在每次读取文件时更新磁盘元数据时间戳，显著减少 SSD 和 NVMe 的冗余读写 I/O 开销。",
            broken_features: &[
                "文件属性中将不再显示精确的最后访问日期",
            ],
        }),
        "disable_paging_executive" => Some(LocalizedTweakText {
            title: "将 Windows 内核与驱动常驻物理内存 (RAM)",
            description: "强制 Windows NT 执行内核和驱动程序例程常驻物理内存而不被分页换出到磁盘，保障内核中断响应瞬间完成。",
            broken_features: &[],
        }),
        "large_system_cache" => Some(LocalizedTweakText {
            title: "启用大系统文件缓存",
            description: "配置 Windows 在物理内存中保留更大的文件系统缓存，加速高频重复文件读取与游戏贴图流式加载。",
            broken_features: &[],
        }),
        "disable_fast_startup" => Some(LocalizedTweakText {
            title: "禁用快速启动（防止休眠内核状态残留与内存泄漏）",
            description: "禁用 Windows 快速启动 (hiberboot)，确保每次关机后都执行彻底的干净冷启动，避免跨会话内核内存泄漏与驱动状态异常。",
            broken_features: &[
                "在较慢的机械硬盘 (HDD) 上，冷启动时间可能增加 1–2 秒",
            ],
        }),
        "ntfs_disable_8dot3" => Some(LocalizedTweakText {
            title: "禁用 NTFS 8.3 短文件名生成",
            description: "禁用 NTFS 卷上传统 MS-DOS 8.3 格式短文件名生成，消除高速 NVMe/SSD 上的目录索引延迟与额外写入开销。",
            broken_features: &[],
        }),
        "increase_mft_zone_reservation" => Some(LocalizedTweakText {
            title: "增大主文件表 (MFT) 保留缓冲区",
            description: "为 NTFS 主文件表 (MFT) 预留更大的连续磁盘缓冲空间，防止 MFT 碎片化并维持最高磁盘访问吞吐量。",
            broken_features: &[],
        }),
        "tcp_ack_frequency" => Some(LocalizedTweakText {
            title: "TCP 低延迟确认 (TcpAckFrequency)",
            description: "强制 Windows TCP 网络栈立即发送 ACK 确认包而不等待超时合并，显著降低联机游戏的网络延迟。",
            broken_features: &[],
        }),
        "tcp_no_delay_global" => Some(LocalizedTweakText {
            title: "全局禁用 Nagle 算法 (TCPNoDelay)",
            description: "在活动网卡接口上禁用 Nagle 数据包缓冲算法，立即发送小数据包以获得最低网络往返延迟。",
            broken_features: &[],
        }),
        "disable_mouse_acceleration" => Some(LocalizedTweakText {
            title: "禁用“提高指针精确度”（1:1 原生鼠标输入）",
            description: "禁用非线性的 Windows 鼠标光标加速，提供竞技精准瞄准所需的真正 1:1 硬件原生鼠标移动。",
            broken_features: &[],
        }),
        "disable_nvidia_telemetry_service" => Some(LocalizedTweakText {
            title: "禁用 NVIDIA 遥测容器服务 (NvTelemetryContainer)",
            description: "禁用收集并传输 NVIDIA 驱动使用统计和崩溃数据的 NvTelemetryContainer 服务。",
            broken_features: &[
                "NVIDIA 崩溃数据与使用情况遥测将不再被收集",
            ],
        }),
        "disable_nvidia_telemetry_tasks" => Some(LocalizedTweakText {
            title: "禁用 NVIDIA 遥测计划任务",
            description: "禁用定期上报 NVIDIA 遥测数据的 NvTmMon、NvTmRep 和 NvProfile 计划任务。",
            broken_features: &[],
        }),
        "disable_adobe_update_service" => Some(LocalizedTweakText {
            title: "禁用 Adobe 更新与正版校验服务",
            description: "禁用 Adobe Acrobat Update Service (AdobeARMservice) 和 Adobe Genuine Monitor Service，阻止后台检查更新与许可遥测。",
            broken_features: &[
                "Adobe 产品将不会自动更新；请手动检查更新",
            ],
        }),
        "disable_edge_update" => Some(LocalizedTweakText {
            title: "禁用 Microsoft Edge 更新服务",
            description: "禁用 Microsoft Edge 更新服务（edgeupdate 和 edgeupdatem）及相关更新计划任务。Edge 仍可正常使用，但不会自动更新或后台联网检查更新。",
            broken_features: &[
                "Microsoft Edge 将不会自动更新；安全补丁需手动安装",
            ],
        }),
        "disable_chrome_update" => Some(LocalizedTweakText {
            title: "禁用 Google Chrome 更新服务",
            description: "禁用 Google Update 服务（gupdate 和 gupdatem）及相关计划任务。Chrome 仍可正常运行，但不会自动更新。",
            broken_features: &[
                "Google Chrome 将不会自动更新；安全补丁需手动安装",
            ],
        }),
        "disable_steam_client_service" => Some(LocalizedTweakText {
            title: "禁用 Steam Client Service 开机自启",
            description: "将 Steam Client Service 从自动启动更改为手动启动。启动 Steam 时仍会按需拉起该服务，但未运行 Steam 时不会在后台占用资源。",
            broken_features: &[],
        }),
        "disable_razer_telemetry" => Some(LocalizedTweakText {
            title: "禁用 Razer 雷蛇遥测与分析服务",
            description: "禁止 Razer Game Manager Service 和 Razer Chroma SDK 服务自动启动，减少 Razer Synapse 雷云在后台的遥测与分析行为。",
            broken_features: &[
                "Razer Synapse 自启动相关功能可能需要手动开启",
            ],
        }),
        "disable_diagtrack" => Some(LocalizedTweakText {
            title: "禁用 DiagTrack（连接用户体验和遥测服务）",
            description: "禁用连接用户体验和遥测服务 (DiagTrack / diagsvc)，该服务是 Windows 主要的遥测数据收集组件。",
            broken_features: &[
                "诊断数据将不会发送至 Microsoft",
            ],
        }),
        "disable_dmwappushservice" => Some(LocalizedTweakText {
            title: "禁用 dmwappushservice（WAP 推送消息路由服务）",
            description: "禁用用于设备管理与遥测数据路由的无线应用协议 (WAP) 推送消息路由服务。",
            broken_features: &[
                "将无法接收 MDM 推送消息",
            ],
        }),
        "disable_telemetry_registry" => Some(LocalizedTweakText {
            title: "通过 AllowTelemetry 注册表策略禁用遥测",
            description: "将 AllowTelemetry 策略设为 0（安全/关闭），即最低遥测级别。在企业版和教育版上可完全关闭遥测；在家庭版和专业版上可降至允许的最低级别。",
            broken_features: &[
                "部分 Windows 预览体验计划与诊断功能可能无法工作",
            ],
        }),
        "disable_compat_telrunner" => Some(LocalizedTweakText {
            title: "禁用应用程序兼容性评估器任务 (CompatTelRunner)",
            description: "禁用运行 CompatTelRunner.exe 扫描已安装软件并发送兼容性遥测的 Microsoft Compatibility Appraiser 和 ProgramDataUpdater 计划任务。",
            broken_features: &[
                "Windows 升级兼容性检查将不会自动运行",
            ],
        }),
        "disable_ceip" => Some(LocalizedTweakText {
            title: "禁用客户体验改善计划 (CEIP)",
            description: "禁用 CEIP 计划任务（Consolidator、UsbCeip、KernelCeipTask）以及 CEIP 策略注册表项。",
            broken_features: &[
                "CEIP 遥测数据将不会被收集",
            ],
        }),
        "disable_wer_service" => Some(LocalizedTweakText {
            title: "禁用 Windows 错误报告服务 (WerSvc)",
            description: "禁用 Windows 错误报告服务 (WerSvc)，防止系统将崩溃报告和错误遥测上传至 Microsoft。",
            broken_features: &[
                "崩溃报告将不会发送给 Microsoft",
                "“问题报告”历史记录将不会更新",
            ],
        }),
        "disable_connected_user_experiences" => Some(LocalizedTweakText {
            title: "禁用连接用户体验与系统建议",
            description: "禁用设置应用、开始菜单和锁屏界面中由云端内容和遥测反馈驱动的各类建议与提示。",
            broken_features: &[
                "Windows 建议与提示将不再显示",
            ],
        }),
        "disable_diagnostic_data_viewer" => Some(LocalizedTweakText {
            title: "禁用诊断数据查看器记录",
            description: "禁用诊断数据查看器的本地事件日志记录（该日志会保存一份发送至 Microsoft 的诊断数据副本），减少磁盘 I/O 与空间占用。",
            broken_features: &[
                "无法通过“诊断数据查看器”应用在本地查看诊断数据",
            ],
        }),
        "disable_delivery_optimization_p2p" => Some(LocalizedTweakText {
            title: "禁用传递优化 P2P 后台上传共享带宽占用",
            description: "限制 Windows 传递优化（DODownloadMode = 0），阻止系统占用您的上行带宽向互联网上的随机电脑分发 Windows 更新。",
            broken_features: &[],
        }),
        "disable_diagnostic_execution_service" => Some(LocalizedTweakText {
            title: "禁用诊断执行服务 (diagsvc)",
            description: "禁用负责按计划执行后台诊断和遥测诊断包的 Diagnostic Execution Service (diagsvc)。",
            broken_features: &[],
        }),
        "wersvc_service_disable" => Some(LocalizedTweakText {
            title: "禁用诊断中心标准收集器服务 (Diagnostics Hub)",
            description: "禁用 Visual Studio / Windows 诊断中心标准收集器服务，在空闲时停止后台 ETW 跟踪收集器。",
            broken_features: &[
                "VS / Windows 诊断中心跟踪收集需要手动启动服务",
            ],
        }),
        "disable_remote_registry" => Some(LocalizedTweakText {
            title: "禁用远程注册表服务 (RemoteRegistry)",
            description: "禁用 RemoteRegistry 服务，防止远程用户和外部网络实体查询或修改本地注册表键值。",
            broken_features: &[
                "通过局域网进行的远程注册表管理将被禁用",
            ],
        }),
        "disable_disk_diagnostic_task" => Some(LocalizedTweakText {
            title: "禁用磁盘诊断遥测计划任务",
            description: "禁用收集磁盘使用指标与硬件诊断信息并用于遥测上报的计划任务。",
            broken_features: &[],
        }),
        "disable_winsat_task" => Some(LocalizedTweakText {
            title: "禁用 WinSAT 后台性能基准测试任务",
            description: "禁止 Windows 系统评估工具计划任务在系统空闲时自动运行磁盘和显卡性能压力测试。",
            broken_features: &[],
        }),
        "enable_classic_context_menu" => Some(LocalizedTweakText {
            title: "恢复完整经典 Windows 10 右键上下文菜单",
            description: "在 HKCU\\Software\\Classes 下注册 CLSID 覆盖项，右键点击时立即打开完整经典菜单，无需再点击一次“显示更多选项”。",
            broken_features: &[
                "Windows 11 精简现代右键菜单已替换为完整经典菜单",
            ],
        }),
        "explorer_power_user_defaults" => Some(LocalizedTweakText {
            title: "显示文件扩展名、标题栏完整路径、默认打开“此电脑”并隐藏同步提示",
            description: "显示已知文件扩展名与隐藏文件，打开文件资源管理器时直接进入“此电脑”而非“快速访问”，开启标题栏完整路径与紧凑视图间距，并隐藏 OneDrive/同步提供程序横幅。",
            broken_features: &[],
        }),
        "explorer_hide_win11_home_gallery_nav" => Some(LocalizedTweakText {
            title: "隐藏 Win11 文件资源管理器导航窗格中的“主页”与“图库”",
            description: "通过 HKCU\\Software\\Classes\\CLSID 从文件资源管理器左侧导航窗格取消固定 Windows 11 的“主页”和“图库”入口。",
            broken_features: &[
                "资源管理器左侧栏中的“主页”与“图库”快捷入口已隐藏",
            ],
        }),
        "explorer_disable_startup_delay_and_jpeg_compression" => Some(LocalizedTweakText {
            title: "消除开机自启延迟并禁用桌面壁纸 JPEG 压缩",
            description: "设置 Explorer\\Serialize\\StartupDelayInMSec=0 使开机自启程序不再被人为延迟 10 秒，同时设置 JPEGImportQuality=100 实现无损桌面壁纸显示。",
            broken_features: &[],
        }),
        "explorer_desktop_this_pc_and_disable_autoplay" => Some(LocalizedTweakText {
            title: "桌面显示“此电脑”图标并禁用可移动媒体自动播放",
            description: "在桌面上显示“此电脑”快捷图标，并关闭可移动 USB/光盘驱动器的 Autoplay/AutoRun，拦截自动弹窗及自启动安全威胁。",
            broken_features: &[
                "插入 USB/SD 存储介质后将不再自动弹出自动播放提示",
            ],
        }),
        "disable_visual_effects_userpreferencesmask_bits" => Some(LocalizedTweakText {
            title: "通过 UserPreferencesMask 位掩码与 DWM 精简繁重 UI 淡入/滑动/阴影动画",
            description: "精确修改 UserPreferencesMask 位字段（禁用菜单淡入滑动、工具提示淡入、菜单选中淡入、光标阴影、组合框滑动、列表平滑滚动与控件动画，同时保留字体平滑与基础桌面效果）并设置 EnableAeroPeek=0。",
            broken_features: &[
                "启用无滑动/淡入动画或阴影的经典极速界面过渡效果",
            ],
        }),
        "taskbar_win11_power_layout_and_end_task" => Some(LocalizedTweakText {
            title: "Win11 任务栏左对齐、隐藏小组件/任务视图/搜索框并开启“结束任务” (Pro/Ent 策略)",
            description: "将任务栏图标左对齐 (TaskbarAl=0)，移除任务视图与小组件按钮 (TaskbarDa=0)，隐藏任务栏搜索框，开启右键“结束任务”开发者选项，禁用 Aero Shake 摇晃最小化，并移除开始菜单“推荐的项目”板块（Windows 11 22H2+ Pro/Enterprise/Education）。",
            broken_features: &[
                "任务栏图标左对齐；任务视图与小组件按钮已隐藏",
            ],
        }),
        "hide_win10_meet_now_tray" => Some(LocalizedTweakText {
            title: "隐藏 Windows 10 托盘“立即开会 (Meet Now)”摄像头图标",
            description: "隐藏 Windows 10 通知区域中的 Skype“立即开会”(Meet Now) 图标（仅适用于 Windows 10 版本）。",
            broken_features: &[],
        }),
        "enable_windows_dark_mode_and_clock_seconds" => Some(LocalizedTweakText {
            title: "启用 Windows 系统与应用深色模式 + 任务栏时钟显示秒数",
            description: "切换 AppsUseLightTheme=0 和 SystemUsesLightTheme=0 以开启原生 Windows 深色模式，并在任务栏时钟中显示实时秒数。",
            broken_features: &[],
        }),
        "disable_startup_sound_and_audio_ducking" => Some(LocalizedTweakText {
            title: "禁用 Windows 开机启动音效与通话自动降音",
            description: "关闭 Windows 登录提示音，并阻止系统在 Discord、Teams 或语音应用开启通话流时将游戏/音乐音量自动降低 80%。",
            broken_features: &[],
        }),
        "enable_verbose_logon_status" => Some(LocalizedTweakText {
            title: "启用详细的 Windows 启动/登录/关机状态信息",
            description: "设置 VerboseStatus=1，使 Windows 在开机、登录和关机界面显示具体的服务与更新阶段，而不仅仅是转圈动画。",
            broken_features: &[],
        }),
        "disable_lock_screen_pro" => Some(LocalizedTweakText {
            title: "跳过登录前锁屏界面幕布 (Pro/Enterprise 策略)",
            description: "开机或锁屏唤醒后直接进入密码/PIN 登录界面，无需先上滑或点击关闭锁屏壁纸。",
            broken_features: &[
                "在 Pro/Enterprise 版本中将直接跳过锁屏滑屏幕布",
            ],
        }),
        "enable_windows_developer_mode" => Some(LocalizedTweakText {
            title: "启用 Windows 开发人员模式（支持侧载应用与无 UAC 符号链接）",
            description: "启用 AllowDevelopmentWithoutDevLicense=1，允许用户无需 UAC 提权即可创建文件系统符号链接，并允许侧载 MSIX/AppX 开发包。",
            broken_features: &[
                "允许侧载未签名的 MSIX 应用包；主要面向开发人员",
            ],
        }),
        // === system_deep_opt.toml (30) ===
        "disable_dwm_animations_per_process" => Some(LocalizedTweakText {
            title: "禁用基于进程的 DWM 窗口过渡动画",
            description: "通过在 DWM 注册表项下设置 DisallowAnimations=1 和 AnimationsShiftKey=1 禁用基于每个进程的 DWM 过渡动画。消除窗口最小化、最大化或切换时的淡入淡出与滑动效果，降低 GPU 合成器开销，让窗口管理响应更迅捷。",
            broken_features: &["将移除窗口过渡切换动画"],
        }),
        "disable_dwm_composition_throttle" => Some(LocalizedTweakText {
            title: "禁用桌面窗口管理器 (DWM) 合成限流",
            description: "在 DWM 注册表项下设置 DesktopCompositionThrottle=0，防止合成器在掉帧时主动降低渲染吞吐量。允许 DWM 在高 GPU 负载下仍保持最大合成吞吐，减少在全屏游戏与窗口程序间切换时的掉帧卡顿。",
            broken_features: &[],
        }),
        "disable_kernel_mitigations_audit" => Some(LocalizedTweakText {
            title: "禁用内核漏洞缓解措施审计模式",
            description: "通过将 MitigationAuditOptions 清零禁用内核漏洞缓解选项审计日志。缓解措施审计在每次触发时均会产生 ETW 事件，增加进程创建和异常处理开销。禁用审计可显著降低内核日志开销。",
            broken_features: &["将不再记录漏洞利用缓解审计事件"],
        }),
        "disable_llmnr_protocol" => Some(LocalizedTweakText {
            title: "禁用 LLMNR 本地链路多播名称解析协议",
            description: "通过在 DNS 客户端参数中设置 EnableMulticast=0 禁用链路本地多播名称解析 (LLMNR)。LLMNR 会在 DNS 失败时向本地子网广播查询，既存在中继中毒安全风险，又会产生不必要的多播广播流量。",
            broken_features: &["无 DNS 环境下的局域网计算机名称解析可能失败", "类似家庭组的本地网络设备自动发现可能受阻"],
        }),
        "disable_memory_compression" => Some(LocalizedTweakText {
            title: "禁用内存压缩引擎",
            description: "通过设置 DisableCompression=1 禁用 Windows 内存管理器的压缩存储。在配备 16 GB 以上内存的高性能 PC 上，禁用压缩可消除 System 进程内存压缩工作线程引起的 CPU 瞬时占用与微卡顿。",
            broken_features: &["在内存受限系统上会增加页面文件的使用量", "在 8 GB 或更小内存系统上可能更早触发内存不足状况"],
        }),
        "disable_netbios_over_tcp" => Some(LocalizedTweakText {
            title: "禁用基于 TCP/IP 的 NetBIOS 协议",
            description: "通过设置 NetbiosOptions=2 并禁用 LMHosts 查询来禁用 NetBIOS over TCP/IP。NetBIOS 是广播名称解析的陈旧局域网协议，会产生无谓的局域网广播风暴并暴露在 NBNS 欺骗投毒攻击风险下。现代 DNS 环境下建议禁用。",
            broken_features: &["依赖 NetBIOS 名称解析的陈旧软件将无法正常寻址", "使用 WINS 的老旧网络打印机可能无法连接"],
        }),
        "disable_ntfs_encryption_driver" => Some(LocalizedTweakText {
            title: "禁用 NTFS 文件系统加密 (EFS) 驱动过滤",
            description: "在 FileSystem 项下设置 NtfsDisableEncryption=1，阻止 NTFS 加密文件系统微过滤驱动介入常规文件 I/O 操作路径。对于不使用单文件 EFS 加密的日常及游戏系统，可有效减少文件读写访问延迟。",
            broken_features: &["NTFS 加密文件系统 (EFS) 将停止工作", "现有已使用 EFS 加密的文件将无法访问"],
        }),
        "disable_page_file_encryption" => Some(LocalizedTweakText {
            title: "禁用页面文件磁盘加密及关机覆写清除",
            description: "阻止 Windows 对虚拟内存交换页面文件进行实时加密，并禁用关机时清空页面文件的耗时流程 (ClearPageFileAtShutdown=0, DisablePagingEncryption=1)。消除换页 CPU 开销并大幅缩短关机耗时。",
            broken_features: &["关机后页面文件中的交换内存内容仍将残留在物理磁盘中", "敏感数据可能通过对原始裸盘的扫描被恢复"],
        }),
        "disable_processor_idle_demote_threshold" => Some(LocalizedTweakText {
            title: "禁用处理器短暂空闲状态降级降频",
            description: "在处理器电源设置中配置 IdleDemoteThreshold=100 与 IdlePromoteThreshold=100，防止内核电源管理器在工作突发间隙的微小空闲期过度降级 CPU 运行状态。消除竞技游戏及低延迟音频工作负载中的升频唤醒延迟。",
            broken_features: &["系统空闲状态下的功耗会有所上升", "便携式笔记本电脑的电池续航时间将有所减少"],
        }),
        "disable_rss_base_cpu" => Some(LocalizedTweakText {
            title: "将网络接收端缩放 (RSS) 基准 CPU 迁移至 CPU 1",
            description: "在 NDIS 参数中设置 RssBaseCpu=1，指令接收端缩放 (RSS) 从 CPU 1 而非默认的 CPU 0 开始分配网络中断处理。将网络中断与 DPC 从首个引导 CPU 上分流，避免网卡中断风暴争夺主线程算力导致的电竞微卡顿。",
            broken_features: &[],
        }),
        "disable_spectre_meltdown_mitigations" => Some(LocalizedTweakText {
            title: "禁用 Spectre / Meltdown CPU 投机执行漏洞安全缓解",
            description: "通过设置 FeatureSettingsOverride=3 和 FeatureSettingsOverrideMask=3 禁用硬件层 Spectre 和 Meltdown CPU 侧信道攻击缓解。彻底消除推测执行分支预测屏障带来的 CPU 性能惩罚，为密集计算工作负载释放最大原始算力。",
            broken_features: &["系统将暴露于 Spectre 和 Meltdown CPU 投机执行侧信道攻击风险中"],
        }),
        "disable_threaded_dpc_optimization" => Some(LocalizedTweakText {
            title: "禁用线程化 DPC 以压低硬件中断响应延迟",
            description: "在内核参数中设置 ThreadDpcEnable=0，强制延迟过程调用 (DPC) 直接在 DISPATCH_LEVEL 级执行而非在 PASSIVE_LEVEL 线程中排队。消除线程上下文调度开销，为专业音频制作和电竞追求亚毫秒级超低中断抖动。",
            broken_features: &["某些强依赖线程化 DPC 的特定硬件驱动程序可能会出现异常", "在极重度的多线程负载下 DPC 延迟波动方差可能会有所增大"],
        }),
        "disable_windows_search_indexer" => Some(LocalizedTweakText {
            title: "完全禁用 Windows 搜索全文索引服务 (WSearch)",
            description: "禁用 WSearch 后台服务，彻底制止后台索引引擎在固态硬盘上持续爬取和遍历所有卷中文件的行为。在 NVMe SSD 上免除 SearchIndexer.exe 造成的后台磁盘读写、CPU 占用和内存消耗。",
            broken_features: &["Windows 全局搜索将不再返回文件具体文本内容匹配结果", "Outlook 桌面客户端邮件与附件检索速度可能会变慢", "开始菜单搜索将无法按文档全文内容定位文件"],
        }),
        "enable_hardware_task_offload" => Some(LocalizedTweakText {
            title: "启用 TCP/IP 网卡硬件卸载加速 (Task Offloading)",
            description: "在 TCP/IP 参数中配置 DisableTaskOffload=0，确保操作系统将数据包校验和计算、TCP 分段以及大型发送卸载 (LSO) 交由网卡专用硬件芯片处理。大幅降低网络高负荷吞吐时的 CPU 中断占用。",
            broken_features: &[],
        }),
        "enable_large_ntfs_frs" => Some(LocalizedTweakText {
            title: "启用 NTFS 大容量文件记录段 (FRS) 空间分配",
            description: "通过设置 NtfsAllowExtendedCharacterIn8dot3Name=0 禁用 8.3 短文件名中的拓展字符支持，允许 NTFS 驱动为 MFT 分配更大、更高效的文件记录段。有效降低海量小文件存储卷上的主文件表碎片与目录枚举开销。",
            broken_features: &["自动生成的 8.3 短文件名中的扩展 Unicode 字符将被去除"],
        }),
        "enable_ultimate_performance_plan" => Some(LocalizedTweakText {
            title: "激活并启用 Windows 隐藏的卓越性能电源方案",
            description: "解锁并应用工作站级专属卓越性能 (Ultimate Performance) 电源方案。完全禁用 CPU 核心休眠挂起 (Core Parking)、消除动态降频响应延迟、关闭 USB 选择性挂起及磁盘停转，为整机提供极致低延迟响应。",
            broken_features: &["系统空闲挂起时的整机待机功耗显著上升", "笔记本电脑等便携设备的电池续航时间明显缩短", "CPU 与主板芯片组在轻载或空闲时的温度会偏高"],
        }),
        "increase_gpu_tdr_timeout" => Some(LocalizedTweakText {
            title: "增加 GPU 超时检测与恢复 (TDR) 响应延迟容限",
            description: "将显卡 TdrDelay 容限时间从默认的 2 秒延长至 10 秒。为耗时漫长的大型计算着色器或光线追踪重载留出充足运算时间，避免 Windows 误判显卡挂起并强制闪屏重启图形驱动。",
            broken_features: &["GPU 发生真正死锁崩溃时的检测与恢复重置用时会变长", "在显卡驱动重置前显示器画面可能出现数秒钟的冻结等待"],
        }),
        "increase_irp_stack_size" => Some(LocalizedTweakText {
            title: "扩充网络与磁盘 I/O 请求包 (IRP) 堆栈大小",
            description: "将 SMB 共享服务 IRPStackSize 堆栈条目从默认的 15 提升至 32。为深层防病毒过滤驱动和高速网络文件传输提供充足的 I/O 堆栈帧，彻底杜绝 STATUS_INSUFFICIENT_RESOURCES 资源不足读写报错。",
            broken_features: &[],
        }),
        "increase_max_connections_per_server" => Some(LocalizedTweakText {
            title: "提高 WinINet 每台服务器并发 HTTP 最大连接数",
            description: "将 MaxConnectionsPerServer 和 MaxConnectionsPer1_0Server 从默认的 2-4 提升至 16。允许 Edge、应用商店、更新服务及系统组件对同一主机并发发起更多 HTTP 管道连接，加速多线程下载与网页加载。",
            broken_features: &[],
        }),
        "increase_nonpaged_pool_quota" => Some(LocalizedTweakText {
            title: "解除内核非分页内存池 (Non-Paged Pool) 配额限制",
            description: "将 NonPagedPoolQuota 配置为 0（无限制/系统最大化管理），允许内核内存管理器在重度网络 I/O、虚拟机多实例或加载大量驱动时充分调动物理内存，避免非分页池耗尽蓝屏。",
            broken_features: &[],
        }),
        "optimize_additional_critical_worker_threads" => Some(LocalizedTweakText {
            title: "预置额外关键与延迟内核工作线程 (Worker Threads)",
            description: "将 AdditionalCriticalWorkerThreads 与 AdditionalDelayedWorkerThreads 均设为 2。为内核执行体调度常驻专用关键与延迟工作线程池，显著缓解高频 I/O 读写与多进程并发时的内核工作项排队等待阻塞。",
            broken_features: &[],
        }),
        "optimize_default_ttl" => Some(LocalizedTweakText {
            title: "将 IP 数据包默认生存时间 (TTL) 设定为标准 64 跃点",
            description: "将 DefaultTTL 设定为 64，对齐 Linux/macOS/UNIX 国际网络协议规范标准。Windows 原生 128 跃点不仅在现代路由拓扑中过度冗长，且易暴露 OS 指纹特征。64 跃点兼顾最佳路由与标准兼容。",
            broken_features: &[],
        }),
        "optimize_disk_io_page_lock_limit" => Some(LocalizedTweakText {
            title: "扩充直接 I/O 内存页锁定上限 (IoPageLockLimit)",
            description: "将内存管理器 IoPageLockLimit 扩增至 983040 字节 (960 KB)。Windows 原生保守限制早已滞后于数 GB/s 级的高速 NVMe 固态硬盘，扩充后允许更大的分散/聚集 DMA 内存锁定并显著加速大文件读写拷贝。",
            broken_features: &[],
        }),
        "optimize_dns_cache_parameters" => Some(LocalizedTweakText {
            title: "优化本地 DNS 客户端解析缓存条目与 TTL 存活周期",
            description: "配置 MaxCacheEntryTtlLimit=86400 与 MaxNegativeCacheTtl=5。大幅延长成功域名解析在本地的驻留缓存以减少向外轮询往返，同时对临时失败的解析在 5 秒内迅速触发重试，显著提升网页浏览和网络请求的灵敏度。",
            broken_features: &[],
        }),
        "optimize_gpu_preemption_level" => Some(LocalizedTweakText {
            title: "将显卡调度器配置为仅计算抢占模式 (Compute-Only)",
            description: "在图形驱动调度器中设置 GpuPreemptionLevel=0。阻止 GPU 任务调度器在长时运行的复杂像素/计算着色器管线中途进行无谓抢占打断，大幅缩减 GPU 线程上下文切换开销，消除 3A 游戏中因管线争用造成的瞬时掉帧。",
            broken_features: &["GPU 超时检测与恢复机制 (TDR) 在特定图形死锁时的响应速度可能有所减缓"],
        }),
        "optimize_interrupt_steering" => Some(LocalizedTweakText {
            title: "优化硬件中断亲和性与多核路由分发策略 (IRQ Steering)",
            description: "在内核中配置 InterruptSteeringDisabled=0 与 DistributeTimers=1。使内核将所有外部硬件中断和高精度定时器回调均匀轮转分发至各个空闲物理核心，避免全部堆积在 CPU 0 引起的单核饱和与中断风暴。",
            broken_features: &[],
        }),
        "optimize_mmcss_proaudio_scheduling" => Some(LocalizedTweakText {
            title: "为多媒体类调度器 (MMCSS) 专业音频任务赋予最高实时优先级",
            description: "将 MMCSS 系统任务配置文件中的 Pro Audio 调度分类与 SFIO 优先级均提升至最高级别 'High'。确保专业宿主编曲软件 (DAW)、ASIO 声卡与连麦通信中的实时音频线程拥有极高调度特权，杜绝爆音与欠载。",
            broken_features: &[],
        }),
        "optimize_ntfs_memory_usage" => Some(LocalizedTweakText {
            title: "启用 NTFS 文件系统分页池侵略性元数据高速缓存",
            description: "将文件系统参数 NtfsMemoryUsage 配置为 2，指令 NTFS 文件系统驱动为目录树与元数据分配更大比例的分页池内存作为热缓存。在大内存设备上成倍提升代码工程构建、Git 版本检索和海量小文件遍历效率。",
            broken_features: &[],
        }),
        "optimize_object_manager_quota" => Some(LocalizedTweakText {
            title: "扩充内核对象管理器句柄池配额上限 (SessionPoolSize)",
            description: "在内存管理会话中设置 SessionPoolSize=48，扩展系统内核对象句柄表分配上限。彻底避免在高并发子进程调用、大型 IDE、本地构建集群以及多微服务运行时因句柄表耗尽引起的程序崩溃。",
            broken_features: &[],
        }),
        "optimize_timer_resolution_requests" => Some(LocalizedTweakText {
            title: "全局允许应用程序请求 0.5ms 超高精度系统时钟定时器",
            description: "在内核中配置 GlobalTimerResolutionRequests=1。允许应用程序向系统申请最高达 0.5 毫秒的硬件平台时钟中断精度，彻底根除后台音频合成引擎、低延迟游戏与实时模拟计算中的计时抖动。",
            broken_features: &["高频定时器中断调度会导致系统待机闲置时的基础功耗小幅增加"],
        }),
        // === gaming_latency_ultra.toml (10) ===
        "disable_driver_search_order_probing" => Some(LocalizedTweakText {
            title: "禁用 Windows Update 后台联网检索设备驱动",
            description: "阻止 Windows Update 在插入新硬件时于后台自动向微软服务器轮询驱动程序。彻底消除新设备接入时的系统识别延迟与后台网络占用。",
            broken_features: &["接入新硬件时将不再通过 Windows Update 自动联网搜索驱动"],
        }),
        "disable_fast_user_switching" => Some(LocalizedTweakText {
            title: "禁用多账户快速用户切换",
            description: "禁用快速用户切换功能，防止非活动登录账户及其后台驻留任务持续占用物理 RAM 内存。释放内存带宽并减少调度器对 CPU 算力的争抢。",
            broken_features: &["多个本地用户账户将无法保持同时登录处于后台驻留状态"],
        }),
        "disable_ip_source_routing" => Some(LocalizedTweakText {
            title: "禁用 IP 源路由选择 (Source Routing)",
            description: "设置 DisableIPSourceRouting=2 以丢弃所有包含源路由指令的入站数据包。有效防范 IP 地址伪造嗅探、路径劫持以及不必要的网络协议栈开销。",
            broken_features: &["依赖源路由机制的网络诊断分析工具（极罕见）将被拦截拒收"],
        }),
        "disable_netsh_teredo_tunneling" => Some(LocalizedTweakText {
            title: "禁用 Teredo IPv6 移行隧道适配器",
            description: "通过设置 DisabledComponents=32 彻底停用 Teredo 隧道伪接口。Teredo 将 IPv6 封包在 IPv4 UDP 中传输，在无原生 IPv6 环境下会产生额外 NAT 穿透延迟。",
            broken_features: &["严格依赖 Teredo NAT 穿透的老旧 Xbox Live P2P 联机游戏将无法直连"],
        }),
        "disable_network_adapter_power_saving" => Some(LocalizedTweakText {
            title: "禁用 NDIS 物理网卡 PnP 节能与休眠",
            description: "将 NDIS PnPCapabilities 设置为 24，彻底禁止网卡硬件进入低功耗节能 D 状态。防止网卡在对局间歇进入休眠造成的首包唤醒延迟与发包抖动。",
            broken_features: &["网卡唤醒 (Wake-on-LAN) 及网卡空闲休眠节能功能将被禁用"],
        }),
        "disable_remote_desktop_udp" => Some(LocalizedTweakText {
            title: "禁用远程桌面客户端 UDP 传输协议",
            description: "强制远程桌面 (RDP) 客户端回退至可靠的 TCP 分帧协议，杜绝 UDP 套接字缓冲区饥饿，防止后台远程桌面流量与实时游戏 UDP 抢占带宽。",
            broken_features: &["高延迟 WAN 网络环境下的远程桌面连接将强制改用 TCP 代替 UDP"],
        }),
        "disable_smart_card_service" => Some(LocalizedTweakText {
            title: "禁用 Windows 智能卡服务 (SCardSvr)",
            description: "将智能卡服务启动类型设置为禁用 (Start=4)。彻底停止后台对智能卡读卡器和加密狗设备的无谓轮询扫描，减少游戏电脑上的多余后台服务。",
            broken_features: &["物理智能卡身份验证及智能卡读卡器硬件将停止工作"],
        }),
        "optimize_kerberos_ticket_cache" => Some(LocalizedTweakText {
            title: "优化 Kerberos 身份认证最大令牌缓存 (MaxTokenSize)",
            description: "将 Kerberos 认证 MaxTokenSize 提升至 65535 字节。避免在复杂局域网、多域环境或复杂安全组策略下发生身份验证缓冲区溢出与反复握手协商。",
            broken_features: &[],
        }),
        "optimize_tcp_syn_attack_protection" => Some(LocalizedTweakText {
            title: "开启 TCP SYN 洪水攻击防御与连接保护",
            description: "在 TCP/IP 协议栈中开启 SYN 洪水防御 (SynAttackProtect=1)。在突发高连接洪峰下加速握手超时判定，保护网络游戏在线套接字免受耗尽假死。",
            broken_features: &[],
        }),
        "optimize_usb_selective_suspend_hub" => Some(LocalizedTweakText {
            title: "全局禁用 USB 集线器选择性暂停挂起",
            description: "在根驱动层彻底禁用 USB 选择性暂停 (DisableSelectiveSuspend=1)。防止主板控制器休眠空闲的电竞鼠标、键盘与专业 USB DAC 声卡，实现零唤醒输入延迟。",
            broken_features: &["USB 总线在空闲待机时的功耗会有微小增加", "笔记本电脑的电池续航时间将有所减少"],
        }),
        "remove_webexperience_package" => Some(LocalizedTweakText {
            title: "删除 Windows WebExperienceHost 程序包（小组件宿主）",
            description: "卸载 MicrosoftWindows.Client.WebExperience 程序包，防止任务栏小组件在后台占用 Edge WebView2 内存。",
            broken_features: &["任务栏小组件和新闻资讯流将被永久移除。"],
        }),
        "disable_wpbt_execution" => Some(LocalizedTweakText {
            title: "禁用 WPBT 固件二进制文件执行",
            description: "阻止 Windows 内核执行主板 UEFI Windows 平台二进制表 (WPBT) 中嵌入的捆绑程序。",
            broken_features: &[],
        }),
        "service_shutdown_timeout_15s" => Some(LocalizedTweakText {
            title: "将服务关机超时缩减至 15 秒",
            description: "将关机期间的服务终止等待超时从默认的 20 秒减少到 15 秒，防止关机卡死。",
            broken_features: &[],
        }),
        "start_menu_show_more_pins" => Some(LocalizedTweakText {
            title: "开始菜单：显示更多固定项布局",
            description: "配置 Windows 11 开始菜单布局，优先显示固定应用程序而非推荐源建议。",
            broken_features: &[],
        }),
        "disable_taskbar_people_button" => Some(LocalizedTweakText {
            title: "禁用任务栏“人脉”按钮",
            description: "隐藏并禁用 Windows 任务栏上的旧版“人脉”图标。",
            broken_features: &[],
        }),
        "repair_ndu_graph" => Some(LocalizedTweakText {
            title: "修复任务管理器网络数据使用量 (NDU) 图表",
            description: "确保网络诊断使用量 (NDU) 内核驱动程序设置为 Start=2（自动），以便任务管理器网络图表正常工作。",
            broken_features: &[],
        }),
        "disable_settings_ads" => Some(LocalizedTweakText {
            title: "禁用“设置”中的推荐内容与广告",
            description: "阻止 Microsoft 在 Windows“设置”中显示推荐应用程序、合作伙伴促销和提示。",
            broken_features: &["“设置”应用中的推广卡片和提示将被抑制。"],
        }),
        "disable_start_menu_suggested_apps" => Some(LocalizedTweakText {
            title: "禁用推荐应用商店应用的自动安装",
            description: "阻止 Windows Content Delivery Manager 静默下载并固定推荐的消费类应用和赞助游戏。",
            broken_features: &["赞助商游戏将不会自动安装到开始菜单中。"],
        }),
        "disable_hardware_companion_software" => Some(LocalizedTweakText {
            title: "禁用硬件配套软件推荐",
            description: "阻止 Windows 在插入外设时自动推荐并通知安装第三方 OEM 硬件配套软件。",
            broken_features: &[],
        }),
        "disable_ms_office_telemetry" => Some(LocalizedTweakText {
            title: "禁用 Microsoft Office 遥测与客户体验日志记录",
            description: "禁用 Microsoft Office 诊断数据收集、反馈提示和客户体验改善计划 (CEIP) 遥测。",
            broken_features: &[],
        }),
        "set_this_pc_as_default_explorer" => Some(LocalizedTweakText {
            title: "将“此电脑”设为文件资源管理器默认打开视图",
            description: "配置文件资源管理器启动时直接打开“此电脑”而非“主页/快速访问”，避免网络驱动器扫描和云同步带来的打开延迟。",
            broken_features: &[],
        }),
        "enable_taskbar_end_task" => Some(LocalizedTweakText {
            title: "在任务栏右键菜单中启用“结束任务”选项",
            description: "启用 Windows 11 原生任务栏右键“结束任务”功能，无需打开任务管理器即可瞬间强行终止进程。",
            broken_features: &[],
        }),
        "disable_oobe_privacy_experience" => Some(LocalizedTweakText {
            title: "禁用 Windows OOBE 隐私设置体验弹窗",
            description: "在创建新用户账户或完成系统功能更新时，通过组策略彻底抑制多页 OOBE 隐私设置向导提示。",
            broken_features: &[],
        }),
        "disable_explorer_sync_provider_ads" => Some(LocalizedTweakText {
            title: "禁用文件资源管理器同步提供程序横幅广告",
            description: "禁止 Windows 文件资源管理器在文件夹顶部弹出 OneDrive 及 Microsoft 365 订阅促销横幅。",
            broken_features: &[],
        }),
        "disable_taskbar_task_view" => Some(LocalizedTweakText {
            title: "隐藏任务栏上的“任务视图”按钮",
            description: "隐藏任务栏中多余的任务视图图标以释放宝贵的屏幕空间，同时不影响快捷键 Alt+Tab 切换。",
            broken_features: &[],
        }),
        "hide_taskbar_meet_now" => Some(LocalizedTweakText {
            title: "从任务栏中隐藏“立即开会”及聊天按钮",
            description: "从任务栏和系统托盘中彻底隐藏 Microsoft Teams 聊天及“立即开会”图标。",
            broken_features: &[],
        }),
        "disable_background_apps_global" => Some(LocalizedTweakText {
            title: "全局禁止后台应用程序运行",
            description: "阻止所有 Windows 商店与 UWP 应用在关闭后仍悄悄驻留后台运行、拉取通知及占用 CPU 与内存资源。",
            broken_features: &[],
        }),
        "disable_finish_setup_ads" => Some(LocalizedTweakText {
            title: "禁用“完成设备设置”全屏营销提示",
            description: "彻底关闭重启后反复弹出的全屏“让我们完成设备设置”以推销 Microsoft 365 和 OneDrive 的烦人引导。",
            broken_features: &[],
        }),
        "disable_lockscreen_spotlight_ads" => Some(LocalizedTweakText {
            title: "禁用锁屏界面趣味知识与商业推广广告",
            description: "禁止 Windows 锁屏界面上推送的各类提示、链接推荐以及全屏动态商业广告覆盖层。",
            broken_features: &[],
        }),
        "disable_welcome_experience_ads" => Some(LocalizedTweakText {
            title: "禁用重大更新后的 Windows 欢迎与促销体验弹窗",
            description: "禁止系统更新后强行弹出的“最新功能介绍”以及推销 Microsoft Edge 和 Copilot 服务的欢迎界面。",
            broken_features: &[],
        }),
        "disable_general_tips_ads" => Some(LocalizedTweakText {
            title: "禁用 Windows 系统日常使用提示与技巧横幅",
            description: "全面关闭由 ContentDeliveryManager 周期性向桌面及通知栏推送的系统技巧、建议和推广通知。",
            broken_features: &[],
        }),
        "enable_windowed_game_optimizations" => Some(LocalizedTweakText {
            title: "启用窗口化游戏专项低延迟优化",
            description: "为窗口化和无边框游戏强制启用 Windows 11 现代翻转呈现模型 (Flip Model)，显著削减输入延迟并解锁自动 HDR 与可变刷新率 (VRR)。",
            broken_features: &[],
        }),
        _ => None,
    }
}

/// Localized tweak title for this language.
#[allow(dead_code)]
pub fn tweak_title(id: &str) -> Option<&'static str> {
    match id {
        "disable_dwm_animations_per_process" => Some("禁用基于进程的 DWM 窗口过渡动画"),
        "disable_dwm_composition_throttle" => Some("禁用桌面窗口管理器 (DWM) 合成限流"),
        "disable_kernel_mitigations_audit" => Some("禁用内核漏洞缓解措施审计模式"),
        "disable_llmnr_protocol" => Some("禁用 LLMNR 本地链路多播名称解析协议"),
        "disable_memory_compression" => Some("禁用内存压缩引擎"),
        "disable_netbios_over_tcp" => Some("禁用基于 TCP/IP 的 NetBIOS 协议"),
        "disable_ntfs_encryption_driver" => Some("禁用 NTFS 文件系统加密 (EFS) 驱动过滤"),
        "disable_page_file_encryption" => Some("禁用页面文件磁盘加密及关机覆写清除"),
        "disable_processor_idle_demote_threshold" => Some("禁用处理器短暂空闲状态降级降频"),
        "disable_rss_base_cpu" => Some("将网络接收端缩放 (RSS) 基准 CPU 迁移至 CPU 1"),
        "disable_spectre_meltdown_mitigations" => Some("禁用 Spectre / Meltdown CPU 投机执行漏洞安全缓解"),
        "disable_threaded_dpc_optimization" => Some("禁用线程化 DPC 以压低硬件中断响应延迟"),
        "disable_windows_search_indexer" => Some("完全禁用 Windows 搜索全文索引服务 (WSearch)"),
        "enable_hardware_task_offload" => Some("启用 TCP/IP 网卡硬件卸载加速 (Task Offloading)"),
        "enable_large_ntfs_frs" => Some("启用 NTFS 大容量文件记录段 (FRS) 空间分配"),
        "enable_ultimate_performance_plan" => Some("激活并启用 Windows 隐藏的卓越性能电源方案"),
        "increase_gpu_tdr_timeout" => Some("增加 GPU 超时检测与恢复 (TDR) 响应延迟容限"),
        "increase_irp_stack_size" => Some("扩充网络与磁盘 I/O 请求包 (IRP) 堆栈大小"),
        "increase_max_connections_per_server" => Some("提高 WinINet 每台服务器并发 HTTP 最大连接数"),
        "increase_nonpaged_pool_quota" => Some("解除内核非分页内存池 (Non-Paged Pool) 配额限制"),
        "optimize_additional_critical_worker_threads" => Some("预置额外关键与延迟内核工作线程 (Worker Threads)"),
        "optimize_default_ttl" => Some("将 IP 数据包默认生存时间 (TTL) 设定为标准 64 跃点"),
        "optimize_disk_io_page_lock_limit" => Some("扩充直接 I/O 内存页锁定上限 (IoPageLockLimit)"),
        "optimize_dns_cache_parameters" => Some("优化本地 DNS 客户端解析缓存条目与 TTL 存活周期"),
        "optimize_gpu_preemption_level" => Some("将显卡调度器配置为仅计算抢占模式 (Compute-Only)"),
        "optimize_interrupt_steering" => Some("优化硬件中断亲和性与多核路由分发策略 (IRQ Steering)"),
        "optimize_mmcss_proaudio_scheduling" => Some("为多媒体类调度器 (MMCSS) 专业音频任务赋予最高实时优先级"),
        "optimize_ntfs_memory_usage" => Some("启用 NTFS 文件系统分页池侵略性元数据高速缓存"),
        "optimize_object_manager_quota" => Some("扩充内核对象管理器句柄池配额上限 (SessionPoolSize)"),
        "optimize_timer_resolution_requests" => Some("全局允许应用程序请求 0.5ms 超高精度系统时钟定时器"),
                "disable_driver_search_order_probing" => Some("禁用 Windows Update 后台联网检索设备驱动"),
        "disable_fast_user_switching" => Some("禁用多账户快速用户切换"),
        "disable_ip_source_routing" => Some("禁用 IP 源路由选择 (Source Routing)"),
        "disable_netsh_teredo_tunneling" => Some("禁用 Teredo IPv6 移行隧道适配器"),
        "disable_network_adapter_power_saving" => Some("禁用 NDIS 物理网卡 PnP 节能与休眠"),
        "disable_remote_desktop_udp" => Some("禁用远程桌面客户端 UDP 传输协议"),
        "disable_smart_card_service" => Some("禁用 Windows 智能卡服务 (SCardSvr)"),
        "optimize_kerberos_ticket_cache" => Some("优化 Kerberos 身份认证最大令牌缓存 (MaxTokenSize)"),
        "optimize_tcp_syn_attack_protection" => Some("开启 TCP SYN 洪水攻击防御与连接保护"),
        "optimize_usb_selective_suspend_hub" => Some("全局禁用 USB 集线器选择性暂停挂起"),
        _ => lookup(id).map(|t| t.title),
    }
}

/// Localized tweak description for this language.
#[allow(dead_code)]
pub fn tweak_description(id: &str) -> Option<&'static str> {
    match id {
        "disable_dwm_animations_per_process" => Some("通过在 DWM 注册表项下设置 DisallowAnimations=1 和 AnimationsShiftKey=1 禁用基于每个进程的 DWM 过渡动画。消除窗口最小化、最大化或切换时的淡入淡出与滑动效果，降低 GPU 合成器开销，让窗口管理响应更迅捷。"),
        "disable_dwm_composition_throttle" => Some("在 DWM 注册表项下设置 DesktopCompositionThrottle=0，防止合成器在掉帧时主动降低渲染吞吐量。允许 DWM 在高 GPU 负载下仍保持最大合成吞吐，减少在全屏游戏与窗口程序间切换时的掉帧卡顿。"),
        "disable_kernel_mitigations_audit" => Some("通过将 MitigationAuditOptions 清零禁用内核漏洞缓解选项审计日志。缓解措施审计在每次触发时均会产生 ETW 事件，增加进程创建和异常处理开销。禁用审计可显著降低内核日志开销。"),
        "disable_llmnr_protocol" => Some("通过在 DNS 客户端参数中设置 EnableMulticast=0 禁用链路本地多播名称解析 (LLMNR)。LLMNR 会在 DNS 失败时向本地子网广播查询，既存在中继中毒安全风险，又会产生不必要的多播广播流量。"),
        "disable_memory_compression" => Some("通过设置 DisableCompression=1 禁用 Windows 内存管理器的压缩存储。在配备 16 GB 以上内存的高性能 PC 上，禁用压缩可消除 System 进程内存压缩工作线程引起的 CPU 瞬时占用与微卡顿。"),
        "disable_netbios_over_tcp" => Some("通过设置 NetbiosOptions=2 并禁用 LMHosts 查询来禁用 NetBIOS over TCP/IP。NetBIOS 是广播名称解析的陈旧局域网协议，会产生无谓的局域网广播风暴并暴露在 NBNS 欺骗投毒攻击风险下。现代 DNS 环境下建议禁用。"),
        "disable_ntfs_encryption_driver" => Some("在 FileSystem 项下设置 NtfsDisableEncryption=1，阻止 NTFS 加密文件系统微过滤驱动介入常规文件 I/O 操作路径。对于不使用单文件 EFS 加密的日常及游戏系统，可有效减少文件读写访问延迟。"),
        "disable_page_file_encryption" => Some("阻止 Windows 对虚拟内存交换页面文件进行实时加密，并禁用关机时清空页面文件的耗时流程 (ClearPageFileAtShutdown=0, DisablePagingEncryption=1)。消除换页 CPU 开销并大幅缩短关机耗时。"),
        "disable_processor_idle_demote_threshold" => Some("在处理器电源设置中配置 IdleDemoteThreshold=100 与 IdlePromoteThreshold=100，防止内核电源管理器在工作突发间隙的微小空闲期过度降级 CPU 运行状态。消除竞技游戏及低延迟音频工作负载中的升频唤醒延迟。"),
        "disable_rss_base_cpu" => Some("在 NDIS 参数中设置 RssBaseCpu=1，指令接收端缩放 (RSS) 从 CPU 1 而非默认的 CPU 0 开始分配网络中断处理。将网络中断与 DPC 从首个引导 CPU 上分流，避免网卡中断风暴争夺主线程算力导致的电竞微卡顿。"),
        "disable_spectre_meltdown_mitigations" => Some("通过设置 FeatureSettingsOverride=3 和 FeatureSettingsOverrideMask=3 禁用硬件层 Spectre 和 Meltdown CPU 侧信道攻击缓解。彻底消除推测执行分支预测屏障带来的 CPU 性能惩罚，为密集计算工作负载释放最大原始算力。"),
        "disable_threaded_dpc_optimization" => Some("在内核参数中设置 ThreadDpcEnable=0，强制延迟过程调用 (DPC) 直接在 DISPATCH_LEVEL 级执行而非在 PASSIVE_LEVEL 线程中排队。消除线程上下文调度开销，为专业音频制作和电竞追求亚毫秒级超低中断抖动。"),
        "disable_windows_search_indexer" => Some("禁用 WSearch 后台服务，彻底制止后台索引引擎在固态硬盘上持续爬取和遍历所有卷中文件的行为。在 NVMe SSD 上免除 SearchIndexer.exe 造成的后台磁盘读写、CPU 占用和内存消耗。"),
        "enable_hardware_task_offload" => Some("在 TCP/IP 参数中配置 DisableTaskOffload=0，确保操作系统将数据包校验和计算、TCP 分段以及大型发送卸载 (LSO) 交由网卡专用硬件芯片处理。大幅降低网络高负荷吞吐时的 CPU 中断占用。"),
        "enable_large_ntfs_frs" => Some("通过设置 NtfsAllowExtendedCharacterIn8dot3Name=0 禁用 8.3 短文件名中的拓展字符支持，允许 NTFS 驱动为 MFT 分配更大、更高效的文件记录段。有效降低海量小文件存储卷上的主文件表碎片与目录枚举开销。"),
        "enable_ultimate_performance_plan" => Some("解锁并应用工作站级专属卓越性能 (Ultimate Performance) 电源方案。完全禁用 CPU 核心休眠挂起 (Core Parking)、消除动态降频响应延迟、关闭 USB 选择性挂起及磁盘停转，为整机提供极致低延迟响应。"),
        "increase_gpu_tdr_timeout" => Some("将显卡 TdrDelay 容限时间从默认的 2 秒延长至 10 秒。为耗时漫长的大型计算着色器或光线追踪重载留出充足运算时间，避免 Windows 误判显卡挂起并强制闪屏重启图形驱动。"),
        "increase_irp_stack_size" => Some("将 SMB 共享服务 IRPStackSize 堆栈条目从默认的 15 提升至 32。为深层防病毒过滤驱动和高速网络文件传输提供充足的 I/O 堆栈帧，彻底杜绝 STATUS_INSUFFICIENT_RESOURCES 资源不足读写报错。"),
        "increase_max_connections_per_server" => Some("将 MaxConnectionsPerServer 和 MaxConnectionsPer1_0Server 从默认的 2-4 提升至 16。允许 Edge、应用商店、更新服务及系统组件对同一主机并发发起更多 HTTP 管道连接，加速多线程下载与网页加载。"),
        "increase_nonpaged_pool_quota" => Some("将 NonPagedPoolQuota 配置为 0（无限制/系统最大化管理），允许内核内存管理器在重度网络 I/O、虚拟机多实例或加载大量驱动时充分调动物理内存，避免非分页池耗尽蓝屏。"),
        "optimize_additional_critical_worker_threads" => Some("将 AdditionalCriticalWorkerThreads 与 AdditionalDelayedWorkerThreads 均设为 2。为内核执行体调度常驻专用关键与延迟工作线程池，显著缓解高频 I/O 读写与多进程并发时的内核工作项排队等待阻塞。"),
        "optimize_default_ttl" => Some("将 DefaultTTL 设定为 64，对齐 Linux/macOS/UNIX 国际网络协议规范标准。Windows 原生 128 跃点不仅在现代路由拓扑中过度冗长，且易暴露 OS 指纹特征。64 跃点兼顾最佳路由与标准兼容。"),
        "optimize_disk_io_page_lock_limit" => Some("将内存管理器 IoPageLockLimit 扩增至 983040 字节 (960 KB)。Windows 原生保守限制早已滞后于数 GB/s 级的高速 NVMe 固态硬盘，扩充后允许更大的分散/聚集 DMA 内存锁定并显著加速大文件读写拷贝。"),
        "optimize_dns_cache_parameters" => Some("配置 MaxCacheEntryTtlLimit=86400 与 MaxNegativeCacheTtl=5。大幅延长成功域名解析在本地的驻留缓存以减少向外轮询往返，同时对临时失败的解析在 5 秒内迅速触发重试，显著提升网页浏览和网络请求的灵敏度。"),
        "optimize_gpu_preemption_level" => Some("在图形驱动调度器中设置 GpuPreemptionLevel=0。阻止 GPU 任务调度器在长时运行的复杂像素/计算着色器管线中途进行无谓抢占打断，大幅缩减 GPU 线程上下文切换开销，消除 3A 游戏中因管线争用造成的瞬时掉帧。"),
        "optimize_interrupt_steering" => Some("在内核中配置 InterruptSteeringDisabled=0 与 DistributeTimers=1。使内核将所有外部硬件中断和高精度定时器回调均匀轮转分发至各个空闲物理核心，避免全部堆积在 CPU 0 引起的单核饱和与中断风暴。"),
        "optimize_mmcss_proaudio_scheduling" => Some("将 MMCSS 系统任务配置文件中的 Pro Audio 调度分类与 SFIO 优先级均提升至最高级别 'High'。确保专业宿主编曲软件 (DAW)、ASIO 声卡与连麦通信中的实时音频线程拥有极高调度特权，杜绝爆音与欠载。"),
        "optimize_ntfs_memory_usage" => Some("将文件系统参数 NtfsMemoryUsage 配置为 2，指令 NTFS 文件系统驱动为目录树与元数据分配更大比例的分页池内存作为热缓存。在大内存设备上成倍提升代码工程构建、Git 版本检索和海量小文件遍历效率。"),
        "optimize_object_manager_quota" => Some("在内存管理会话中设置 SessionPoolSize=48，扩展系统内核对象句柄表分配上限。彻底避免在高并发子进程调用、大型 IDE、本地构建集群以及多微服务运行时因句柄表耗尽引起的程序崩溃。"),
        "optimize_timer_resolution_requests" => Some("在内核中配置 GlobalTimerResolutionRequests=1。允许应用程序向系统申请最高达 0.5 毫秒的硬件平台时钟中断精度，彻底根除后台音频合成引擎、低延迟游戏与实时模拟计算中的计时抖动。"),
                "disable_driver_search_order_probing" => Some("阻止 Windows Update 在插入新硬件时于后台自动向微软服务器轮询驱动程序。彻底消除新设备接入时的系统识别延迟与后台网络占用。"),
        "disable_fast_user_switching" => Some("禁用快速用户切换功能，防止非活动登录账户及其后台驻留任务持续占用物理 RAM 内存。释放内存带宽并减少调度器对 CPU 算力的争抢。"),
        "disable_ip_source_routing" => Some("设置 DisableIPSourceRouting=2 以丢弃所有包含源路由指令的入站数据包。有效防范 IP 地址伪造嗅探、路径劫持以及不必要的网络协议栈开销。"),
        "disable_netsh_teredo_tunneling" => Some("通过设置 DisabledComponents=32 彻底停用 Teredo 隧道伪接口。Teredo 将 IPv6 封包在 IPv4 UDP 中传输，在无原生 IPv6 环境下会产生额外 NAT 穿透延迟。"),
        "disable_network_adapter_power_saving" => Some("将 NDIS PnPCapabilities 设置为 24，彻底禁止网卡硬件进入低功耗节能 D 状态。防止网卡在对局间歇进入休眠造成的首包唤醒延迟与发包抖动。"),
        "disable_remote_desktop_udp" => Some("强制远程桌面 (RDP) 客户端回退至可靠的 TCP 分帧协议，杜绝 UDP 套接字缓冲区饥饿，防止后台远程桌面流量与实时游戏 UDP 抢占带宽。"),
        "disable_smart_card_service" => Some("将智能卡服务启动类型设置为禁用 (Start=4)。彻底停止后台对智能卡读卡器和加密狗设备的无谓轮询扫描，减少游戏电脑上的多余后台服务。"),
        "optimize_kerberos_ticket_cache" => Some("将 Kerberos 认证 MaxTokenSize 提升至 65535 字节。避免在复杂局域网、多域环境或复杂安全组策略下发生身份验证缓冲区溢出与反复握手协商。"),
        "optimize_tcp_syn_attack_protection" => Some("在 TCP/IP 协议栈中开启 SYN 洪水防御 (SynAttackProtect=1)。在突发高连接洪峰下加速握手超时判定，保护网络游戏在线套接字免受耗尽假死。"),
        "optimize_usb_selective_suspend_hub" => Some("在根驱动层彻底禁用 USB 选择性暂停 (DisableSelectiveSuspend=1)。防止主板控制器休眠空闲的电竞鼠标、键盘与专业 USB DAC 声卡，实现零唤醒输入延迟。"),
        _ => lookup(id).map(|t| t.description),
    }
}
