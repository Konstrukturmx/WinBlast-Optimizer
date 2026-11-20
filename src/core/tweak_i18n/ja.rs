//! Japanese (日本語) localization table for all 168 WinBlast0 catalog tweaks.

use super::LocalizedTweakText;

pub fn lookup(id: &str) -> Option<LocalizedTweakText> {
    match id {
        "disable_windows_ai_recall_extended" => Some(LocalizedTweakText {
            title: "Windows Recall、Click to Do、Copilot+ AI エージェントを無効化",
            description: "Windows 11 24H2 以降で導入されたグループポリシーキーにより、Windows Recall のスナップショット取得、Click to Do、設定 AI エージェント、AI エージェントコネクタ/ワークスペース、Copilot Runtime を遮断します。",
            broken_features: &[
                "Windows Recall のタイムラインスナップショットおよび Click to Do AI オーバーレイ",
            ],
        }),
        "disable_generative_ai_app_access" => Some(LocalizedTweakText {
            title: "生成 AI およびシステム AI モデルへのアプリアクセスを遮断",
            description: "AppPrivacy および CapabilityAccessManager ConsentStore ポリシーにより、バックグラウンド UWP/WinUI アプリによるローカル生成 AI やシステム AI モデル機能へのアクセスを拒否します。",
            broken_features: &[
                "サードパーティ製アプリが組み込みの Windows AI モデルエンドポイントへ照会できなくなります",
            ],
        }),
        "disable_copilot_shell_and_hardware_key" => Some(LocalizedTweakText {
            title: "Copilot タスクバーボタン・通知・専用キー・マイクを無効化",
            description: "Windows Copilot のタスクバーボタンと推奨通知を非表示にし、キーボード専用 Copilot キーの割り当てを無効化、アカウント連動 AI プロモを遮断し、Copilot と OfficeHub へのマイクアクセスを拒否します。",
            broken_features: &[
                "専用 Copilot キーで Copilot が起動しなくなります",
                "Copilot の音声入力が遮断されます",
            ],
        }),
        "disable_paint_ai_features" => Some(LocalizedTweakText {
            title: "Microsoft ペイントのクラウドおよび生成 AI 機能を無効化",
            description: "公式ポリシーキーにより、Windows 11 ペイントの Image Creator、Cocreator、生成塗りつぶし、生成消去、AI 背景削除を無効化します。",
            broken_features: &[
                "Microsoft ペイント内の AI 生成ツールが非表示になります",
            ],
        }),
        "disable_edge_ai_policies" => Some(LocalizedTweakText {
            title: "Microsoft Edge Copilot・サイドバー・組み込み AI API を無効化",
            description: "Copilot ページコンテキスト、Hubs サイドバー、Entra Copilot、インライン作成、AI 履歴検索、ローカル基盤モデル、DevTools GenAI、履歴共有を含む 12 個の Microsoft Edge AI ポリシーを無効化します。",
            broken_features: &[
                "Edge Copilot サイドバーとインライン文章作成支援が無効になります",
            ],
        }),
        "disable_office_ai_copilot" => Some(LocalizedTweakText {
            title: "Microsoft Office AI トレーニングと各アプリの Copilot を無効化",
            description: "Microsoft 365 の AI トレーニング、接続コンテンツ分析、Content Safety Service、および Word・Excel・OneNote の各アプリ向け Copilot スイッチを無効化します。",
            broken_features: &[
                "Word・Excel・OneNote 内の Copilot アシスタントが無効になります",
            ],
        }),
        "disable_wmi_telemetry_autologgers" => Some(LocalizedTweakText {
            title: "10 種の WMI カーネルテレメトリ AutoLogger セッションを無効化",
            description: "Windows が永続的な ETW バックグラウンド AutoLogger トレースセッション（Diagtrack-Listener、SQMLogger、DiagLog、Cellcore、DataMarket、AppModel、CloudExperienceHostOobe、LwtNetLog、WdiContextLog、WiFiSession）を起動しないよう停止します。",
            broken_features: &[
                "カーネル ETW 診断トレースログが無効になります",
            ],
        }),
        "block_silent_preinstalled_apps" => Some(LocalizedTweakText {
            title: "サイレント自動インストールおよびスタート・設定のおすすめを遮断",
            description: "Windows ContentDeliveryManager による OEM や Microsoft Store プロモーションアプリの無断インストールを防ぎ、設定・スタートメニュー・タイムラインのおすすめ表示を非表示にします。",
            broken_features: &[],
        }),
        "disable_extra_telemetry_policies" => Some(LocalizedTweakText {
            title: "フィードバック・AppCompat インベントリ・OOBE 案内・追跡を制限",
            description: "MaxTelemetryAllowed=0 に制限し、Windows フィードバック SIUF 通知を抑制（PeriodInNanoSeconds を削除）、アプリ互換性インベントリ/UAR、デバイスを探す、セットアップ完了促進、アプリ起動追跡、Workplace Join 案内、リモートアシスタンスを無効化します。",
            broken_features: &[
                "リモートアシスタンスの招待機能が無効になります",
                "スタートメニューのよく使う・最近起動したアプリの追跡がオフになります",
            ],
        }),
        "disable_typing_and_inking_telemetry" => Some(LocalizedTweakText {
            title: "手書き・入力の個人設定および Input Insights テレメトリを無効化",
            description: "暗黙的なキー入力・インクデータ収集、カスタム入力辞書向けの連絡先収集、言語診断ログ（TIPC）、および Windows Input Insights を制限します。",
            broken_features: &[
                "入力および手書き予測のパーソナライズ辞書がオフになります",
            ],
        }),
        "disable_search_cloud_and_history" => Some(LocalizedTweakText {
            title: "Windows 検索ハイライト・クラウド検索・デバイス検索履歴を無効化",
            description: "インデックスサービスを停止せずに、Windows 検索ハイライト（動的検索ボックス）、MSA/AAD クラウドコンテンツ検索、およびローカルデバイスの検索履歴記録を無効にします。",
            broken_features: &[
                "検索ハイライトの日替わりイラストと OneDrive クラウド検索結果が非表示になります",
            ],
        }),
        "disable_wifi_sense_and_location_extras" => Some(LocalizedTweakText {
            title: "WiFi-Sense 自動接続・マップ自動更新・位置情報 ConsentStore を無効化",
            description: "WiFi ホットスポットへの自動接続（WcmSvc\\AutoConnectAllowedOEM=0）、Windows マップのオフライン自動更新、および位置情報機能 ConsentStore のグローバル権限を無効にします。",
            broken_features: &[
                "Windows マップの自動ダウンロードとシステム全体の位置情報要求が無効になります",
            ],
        }),
        "block_onedrive_kfm_auto_backup" => Some(LocalizedTweakText {
            title: "OneDrive の既知のフォルダー自動バックアップ（デスクトップ/ドキュメント同期）を阻止",
            description: "KFMBlockOptIn=1 を設定し、OneDrive がデスクトップ・ドキュメント・ピクチャフォルダーを無断でクラウド同期へ移行しないよう阻止します。ローカル保存や手動同期のみ使う場合に安全です。",
            broken_features: &[
                "OneDrive の既知のフォルダー自動移行ウィザードが無効になります",
            ],
        }),
        "disable_bitlocker_auto_encryption" => Some(LocalizedTweakText {
            title: "Windows BitLocker デバイス自動暗号化を無効化",
            description: "PreventDeviceEncryption=1 を設定し、Microsoft アカウントでのサインイン時や初期セットアップ時に回復キーをバックアップしないままドライブが自動暗号化されるのを防ぎます。",
            broken_features: &[
                "ゼロタッチの自動 BitLocker ドライブ暗号化が作動しなくなります",
            ],
        }),
        "disable_extra_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "追加テレメトリ・WER キュー・SQM・電源診断・マップのタスクを無効化",
            description: "QueueReporting、Sqm-Tasks、MareBackup、StartupAppTask、PcaPatchDbTask、MapsUpdateTask、AnalyzeSystem、FamilySafetyMonitor などの追加診断・バックグラウンド予定タスクを無効化します。",
            broken_features: &[
                "電源効率自動レポートとオフラインマップの自動更新が無効になります",
            ],
        }),
        "disable_windows_ai_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Windows AI Recall バックグラウンド予定タスクを無効化",
            description: "Windows 11 24H2 以降で導入された Windows Recall 設定および ML バックグラウンドパイプラインの予定タスクを無効化します。",
            broken_features: &[
                "Windows Recall のバックグラウンドインデックスタスクが無効になります",
            ],
        }),
        "disable_office_actions_scheduled_task" => Some(LocalizedTweakText {
            title: "Microsoft Office AI Actions Server 予定タスクを無効化",
            description: "Microsoft Office Actions Server のバックグラウンド予定タスクを無効化し、Office AI によるバックグラウンドの CPU/メモリ起動を削減します。",
            broken_features: &[],
        }),
        "disable_optional_bloat_services" => Some(LocalizedTweakText {
            title: "不要なオプションサービスを無効化（マップ・FAX・デモ・ウォレット・自動タイムゾーン）",
            description: "OS の基幹サービスや Windows Update に触れず、MapsBroker、RetailDemo、Fax、WalletService、WMPNetworkSvc、AJRouter、shpamsvc、tzautoupdate を無効化します。",
            broken_features: &[
                "Windows FAX、オフラインマップブローカー、店頭デモ、WMP 共有、自動タイムゾーンが無効になります",
            ],
        }),
        "disable_xbox_live_services" => Some(LocalizedTweakText {
            title: "Xbox Live バックグラウンドサービスを無効化（XblAuthManager・GameSave・Networking）",
            description: "Xbox アプリや Game Pass ゲームを利用しないユーザー向けに、Xbox Live 認証マネージャー、ゲームセーブ、ネットワークバックグラウンドサービスを無効化します。",
            broken_features: &[
                "Xbox Game Pass および Microsoft Store での Xbox Live サインイン・クラウドセーブが動作しなくなります",
            ],
        }),
        "disable_ai_fabric_service" => Some(LocalizedTweakText {
            title: "Windows AI Fabric サービス (WSAIFabricSvc) を無効化",
            description: "Windows 11 24H2 以降のローカル AI オーケストレーションバックグラウンドデーモン WSAIFabricSvc を無効化します。",
            broken_features: &[
                "Copilot+ AI オーケストレーションサービスが停止します",
            ],
        }),
        "deny_global_camera_mic_diagnostics" => Some(LocalizedTweakText {
            title: "カメラ・マイク・アカウント情報・診断への全アプリアクセスを遮断 [任意選択]",
            description: "すべてのデスクトップおよび UWP アプリに対し、Web カメラ、マイク、userAccountInformation、appDiagnostics の ConsentStore 権限を一括拒否します。高影響リスク：解除するまで Zoom/Discord/OBS 等のカメラとマイクが使えなくなります。",
            broken_features: &[
                "再度許可するまで、すべてのアプリでカメラとマイクが無効になります",
            ],
        }),
        "remove_bing_weather" => Some(LocalizedTweakText {
            title: "Bing 天気 (Weather) を削除",
            description: "Microsoft Bing 天気 UWP アプリを削除します。天気情報はブラウザーから閲覧できます。",
            broken_features: &[
                "Bing 天気アプリが削除されます",
            ],
        }),
        "remove_bing_news" => Some(LocalizedTweakText {
            title: "Bing ニュース (Microsoft News) を削除",
            description: "Microsoft Bing ニュース (Microsoft News) UWP アプリを削除します。",
            broken_features: &[
                "Bing ニュース / Microsoft News アプリが削除されます",
            ],
        }),
        "remove_get_help" => Some(LocalizedTweakText {
            title: "問い合わせ (Get Help) を削除",
            description: "Microsoft 問い合わせ (Get Help) UWP アプリを削除します。サポートは引き続き Web から利用できます。",
            broken_features: &[
                "問い合わせ (Get Help) アプリが削除されます",
            ],
        }),
        "remove_getstarted_tips" => Some(LocalizedTweakText {
            title: "ヒント (Get Started) を削除",
            description: "Microsoft ヒント / Get Started UWP アプリを削除します。",
            broken_features: &[
                "ヒント / Get Started アプリが削除されます",
            ],
        }),
        "remove_solitaire" => Some(LocalizedTweakText {
            title: "Microsoft Solitaire Collection を削除",
            description: "広告やテレメトリを含む Microsoft Solitaire Collection UWP アプリを削除します。",
            broken_features: &[
                "Solitaire Collection が削除されます",
            ],
        }),
        "remove_people" => Some(LocalizedTweakText {
            title: "People アプリを削除",
            description: "Microsoft People UWP アプリを削除します。",
            broken_features: &[
                "People アプリが削除されます",
            ],
        }),
        "remove_windows_maps" => Some(LocalizedTweakText {
            title: "Windows マップを削除",
            description: "Microsoft Windows マップ UWP アプリを削除します。",
            broken_features: &[
                "Windows マップアプリが削除されます",
            ],
        }),
        "remove_groove_music" => Some(LocalizedTweakText {
            title: "Groove ミュージック (Zune Music) を削除",
            description: "Groove ミュージック (ZuneMusic) UWP アプリを削除します。代わりにメディアプレーヤーやサードパーティ製プレーヤーを利用できます。",
            broken_features: &[
                "Groove ミュージックアプリが削除されます",
            ],
        }),
        "remove_zune_video" => Some(LocalizedTweakText {
            title: "映画 & テレビ (Zune Video) を削除",
            description: "映画 & テレビ (ZuneVideo) UWP アプリを削除します。代わりに VLC などのプレーヤーを利用できます。",
            broken_features: &[
                "映画 & テレビアプリが削除されます",
            ],
        }),
        "remove_clipchamp" => Some(LocalizedTweakText {
            title: "Clipchamp を削除",
            description: "Windows 11 にプリインストールされている Clipchamp 動画編集 UWP アプリを削除します。",
            broken_features: &[
                "Clipchamp 動画エディターが削除されます",
            ],
        }),
        "remove_todos" => Some(LocalizedTweakText {
            title: "Microsoft To Do を削除",
            description: "Microsoft To Do UWP アプリを削除します。",
            broken_features: &[
                "Microsoft To Do アプリが削除されます",
            ],
        }),
        "remove_bing_search_appx" => Some(LocalizedTweakText {
            title: "Microsoft Bing 検索 AppX 拡張を削除",
            description: "Web ウィジェットと Windows 検索の Web 連携を担う Microsoft.BingSearch ストアパッケージを削除します。",
            broken_features: &[
                "Bing 検索 AppX 連携が削除されます",
            ],
        }),
        "remove_feedback_hub" => Some(LocalizedTweakText {
            title: "Microsoft フィードバック Hub を削除",
            description: "プリインストールされている Windows フィードバック Hub テレメトリ・アンケートアプリを削除します。",
            broken_features: &[
                "フィードバック Hub アプリが削除されます",
            ],
        }),
        "remove_copilot_appx" => Some(LocalizedTweakText {
            title: "Microsoft Copilot UWP パッケージを削除",
            description: "単体版の Microsoft.Copilot ストアパッケージをアンインストールします。",
            broken_features: &[
                "Copilot アプリが削除されます",
            ],
        }),
        "remove_edge_game_assist" => Some(LocalizedTweakText {
            title: "Microsoft Edge ゲームアシストオーバーレイを削除",
            description: "ゲーム内ブラウザーオーバーレイを提供する Edge Game Assist AppX パッケージを削除します。",
            broken_features: &[
                "Edge ゲームアシストウィジェットが削除されます",
            ],
        }),
        "remove_dev_home" => Some(LocalizedTweakText {
            title: "Windows Dev Home アプリを削除",
            description: "Windows 11 にプリインストールされている Microsoft.Windows.DevHome ダッシュボードパッケージを削除します。",
            broken_features: &[
                "Dev Home ダッシュボードが削除されます",
            ],
        }),
        "remove_power_automate_desktop" => Some(LocalizedTweakText {
            title: "Microsoft Power Automate Desktop を削除",
            description: "Microsoft Power Automate Desktop RPA パッケージを削除します。",
            broken_features: &[
                "Power Automate デスクトップフローアプリが削除されます",
            ],
        }),
        "remove_office_hub" => Some(LocalizedTweakText {
            title: "Microsoft 365 Copilot (OfficeHub) を削除",
            description: "プリインストールされている Microsoft 365 Office Hub プロモーションランチャーを削除します。",
            broken_features: &[
                "Office プロモランチャーが削除されます（デスクトップ版 Office は削除されません）",
            ],
        }),
        "remove_family_safety" => Some(LocalizedTweakText {
            title: "Microsoft Family Safety アプリを削除",
            description: "一般ユーザー向けの Microsoft Family Safety UWP アプリを削除します。",
            broken_features: &[
                "Family Safety UWP アプリが削除されます",
            ],
        }),
        "remove_3d_viewer_and_mixed_reality" => Some(LocalizedTweakText {
            title: "3D ビューアー & Mixed Reality ポータルを削除",
            description: "レガシの Microsoft 3D ビューアーおよび Mixed Reality ポータルパッケージを削除します。",
            broken_features: &[
                "レガシ 3D ビューアーと Windows Mixed Reality ポータルが削除されます",
            ],
        }),
        "remove_outlook_for_windows" => Some(LocalizedTweakText {
            title: "新しい Outlook for Windows（WebView クライアント）を削除",
            description: "プリインストールされている Microsoft.OutlookForWindows Web ラッパーアプリを削除します。",
            broken_features: &[
                "新しい Outlook UWP クライアントが削除されます",
            ],
        }),
        "remove_your_phone_link" => Some(LocalizedTweakText {
            title: "スマートフォン連携 (YourPhone) を削除",
            description: "スマートフォン同期用コンパニオンアプリ Microsoft.YourPhone をアンインストールします。",
            broken_features: &[
                "Android/iOS スマートフォン連携デスクトップ同期が削除されます",
            ],
        }),
        "remove_sound_recorder_and_sticky_notes" => Some(LocalizedTweakText {
            title: "Windows サウンドレコーダー & 付箋 (Sticky Notes) を削除",
            description: "サウンドレコーダーおよび付箋のストアパッケージを削除します。",
            broken_features: &[
                "組み込みのサウンドレコーダーと付箋アプリが削除されます",
            ],
        }),
        "remove_ms_teams_and_quick_assist" => Some(LocalizedTweakText {
            title: "個人向け Microsoft Teams & クイックアシストを削除",
            description: "個人向け MSTeams チャットアプリとクイックアシストサポートアプリを削除します。",
            broken_features: &[
                "個人向け Teams とクイックアシスト支援ツールが削除されます",
            ],
        }),
        "disable_recall_optional_feature" => Some(LocalizedTweakText {
            title: "Windows Recall オプション機能をアンインストール (DISM)",
            description: "OS サービスレベルでオプションの「Recall」Windows 機能パッケージを無効化します。",
            broken_features: &[
                "Windows Recall 機能バイナリがクリーンに無効化されます",
            ],
        }),
        "remove_legacy_steps_recorder_ie_wordpad" => Some(LocalizedTweakText {
            title: "レガシ ステップ記録ツール & Internet Explorer 11 機能を無効化",
            description: "DISM 経由でレガシのステップ記録ツール (App.StepsRecorder) と Internet Explorer モード機能を削除します。",
            broken_features: &[
                "ステップ記録ツール (psr.exe) と IE11 モードが削除されます",
            ],
        }),
        "enable_hags" => Some(LocalizedTweakText {
            title: "ハードウェアアクセラレータ GPU スケジューリング (HAGS) を有効化",
            description: "HAGS を有効にし、GPU が自身の VRAM スケジューリングを直接管理できるようにします。GPU 負荷の高い処理で遅延と CPU オーバーヘッドを削減します。対応 GPU（NVIDIA GTX 1000 以降、AMD RX 5000 以降）とドライバが必要です。",
            broken_features: &[
                "一部の古いゲームや GPU ツールで動作が不安定になる場合があります",
            ],
        }),
        "disable_mpo" => Some(LocalizedTweakText {
            title: "マルチプレーンオーバーレイ (MPO) を無効化",
            description: "MPO ハードウェアコンポジター機能を無効にします。MPO は特にリフレッシュレートが異なるマルチモニター環境など、多くの GPU/ドライバの組み合わせでカクつき・ブラックスクリーン・フレームペース乱れの原因になります。",
            broken_features: &[
                "ノート PC で GPU コンポジターの待機消費電力がわずかに増加する場合があります",
            ],
        }),
        "disable_core_parking" => Some(LocalizedTweakText {
            title: "CPU コアパーキングを無効化",
            description: "アクティブ維持するプロセッサコアの最小割合を 100% に設定し、全コアを常に稼働可能にします。アイドル時の電力と引き換えにコア復帰遅延を排除します。",
            broken_features: &[
                "アイドル時の消費電力が増加します",
                "ノート PC のバッテリー駆動時間が短くなります",
            ],
        }),
        "disable_cstates" => Some(LocalizedTweakText {
            title: "CPU アイドル C-States を無効化",
            description: "アイドル状態ポリシーをパフォーマンスモードに設定し、プロセッサが深い省電力 C-state（C6/C7/C8）へ移行しないようにします。電力状態遷移時の遅延スパイクを防ぎます。",
            broken_features: &[
                "アイドル時の消費電力が大幅に増加します",
                "ノート PC のバッテリー駆動時間が短くなります",
                "アイドル時の CPU 温度が上昇します",
            ],
        }),
        "gpu_shader_cache_size" => Some(LocalizedTweakText {
            title: "GPU TDR タイムアウトを延長しドライバリセットを防止",
            description: "WDDM のタイムアウト検出・回復 (TdrDelay) 間隔を 2 秒から 10 秒に延長し、重い DirectX シェーダーコンパイル中の誤った GPU ドライバリセットや引っかかりを防ぎます。",
            broken_features: &[
                "実際に GPU ハングが発生した場合、表示ドライバの回復に最大 10 秒待機します",
            ],
        }),
        "enable_game_mode" => Some(LocalizedTweakText {
            title: "Windows ゲームモードを有効化",
            description: "Windows ゲームモード（AutoGameModeEnabled=1、AllowAutoGameMode=1）を有効にし、ゲームプロセスの優先度を引き上げ、プレイ中の Windows Update や再起動通知を抑制します。",
            broken_features: &[],
        }),
        "disable_fullscreen_optimizations" => Some(LocalizedTweakText {
            title: "全画面表示の最適化をグローバルで無効化",
            description: "すべてのアプリに対して Windows 全画面最適化（ボーダーレス全画面ハイブリッド）をグローバルで無効にします。真の排他フルスクリーンを強制し、古いゲームでの入力遅延とフレームタイムのばらつきを抑えます。",
            broken_features: &[
                "フルスクリーンゲームからの Alt-Tab 切り替えが遅くなります",
                "排他フルスクリーンでは Game Bar オーバーレイが機能しません",
            ],
        }),
        "4433ce93-b632-4d67-ae71-0241c31099c0" => Some(LocalizedTweakText {
            title: "AMD ULPS（超低電力状態）を無効化",
            description: "AMD Radeon ドライバスタックの Ultra Low Power State (ULPS) を無効にし、3D ゲーム中にセカンダリ GPU コアや休止状態からの復帰で生じるマイクロスタッターや遅延スパイクを防止します。",
            broken_features: &[
                "AMD マルチ GPU / CrossFire 搭載ノート PC で待機電力がわずかに増加します",
            ],
        }),
        "8a4b4c4d-000e-42db-8bd7-293067f8fae7" => Some(LocalizedTweakText {
            title: "AMD PCIe ASPM を無効化",
            description: "AMD グラフィックスカードの PCIe アクティブステート電源管理 (ASPM) を無効化します。フレーム描画の合間に PCIe リンクが低電力 L0s/L1 へ移行するのを防ぎ、PCIe バス転送ジッタを低減します。",
            broken_features: &[
                "アイドル時の PCIe バス消費電力が増加します",
            ],
        }),
        "a3836587-c461-4beb-9c4c-88493b36c138" => Some(LocalizedTweakText {
            title: "AMD Power Gating を無効化",
            description: "AMD コンピュートユニットとレンダーバックエンドの積極的な動的パワーゲーティングを無効化（DisablePowerGating=1、PP_GPUPowerDownEnabled=0）し、スロットリングのない安定したクロックを維持します。",
            broken_features: &[
                "低負荷時の GPU ダイ温度が高くなります",
            ],
        }),
        "db379bb5-fc8c-4ad3-be32-11be09e76fba" => Some(LocalizedTweakText {
            title: "AMD ビデオクロックゲーティングを無効化",
            description: "AMD VCE/UVD マルチメディアおよびハードウェアディスプレイエンジンのクロック/パワーゲーティング（DisableVCEPowerGating=1、DisableVceClockGating=1、EnableUvdClockGating=0、EnableVceSwClockGating=0）を無効にし、DPC 遅延スパイクを解消します。",
            broken_features: &[],
        }),
        "df8739ff-e322-4958-b369-df584bb9b0c1" => Some(LocalizedTweakText {
            title: "Intel 非同期フリップ (Async Flips) を無効化",
            description: "Intel Graphics ディスプレイアダプタークラスキーに Display1_DisableAsyncFlips=1 を設定し、同期されたディスプレイエンジンフリップを強制してプレゼンテーションキューのジッタを排除します。",
            broken_features: &[],
        }),
        "ebfb0ded-5d1a-4cd1-b5c0-4f7018dd6054" => Some(LocalizedTweakText {
            title: "Intel アダプティブ V-Sync を無効化",
            description: "Intel Graphics ディスプレイアダプタークラスキーに AdaptiveVsyncEnable=0 を設定し、フレームレートがモニターのリフレッシュレートを下回った際のアダプティブリフレッシュ制限を防ぎます。",
            broken_features: &[],
        }),
        "fa58802f-b98c-4e98-aa67-067c5a394296" => Some(LocalizedTweakText {
            title: "NVIDIA 動的 P-State を無効化",
            description: "DisableDynamicPstate=1 を設定して NVIDIA GPU を固定パフォーマンス状態にロックし、クロック周波数の変動を防止します。アイドル時消費電力は増加します。",
            broken_features: &[
                "アイドル時の GPU 消費電力と発熱が増加します",
            ],
        }),
        "9e7702be-9e03-4c96-a316-c4e435cb993e" => Some(LocalizedTweakText {
            title: "NVIDIA 非同期 P-State を無効化",
            description: "DisableAsyncPstates=1 を設定して NVIDIA の非同期電源状態切り替えを防ぎ、VR やリアルタイムアプリで決定論的なパフォーマンスを提供します。",
            broken_features: &[
                "動画再生中に GPU メモリクロックが低下しなくなる場合があります",
            ],
        }),
        "5f1a2b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c" => Some(LocalizedTweakText {
            title: "NVIDIA PowerMizer 最大パフォーマンスモードを有効化",
            description: "AC 電源接続時に PowerMizer 最大パフォーマンスモードを強制し、PCIe バスおよびメモリコントローラーをスロットリングなしのフルスピードに固定します。",
            broken_features: &[
                "GPU 温度と待機時ファン回転数が高くなります",
            ],
        }),
        "disable_animations" => Some(LocalizedTweakText {
            title: "ウィンドウアニメーションを無効化",
            description: "ウィンドウの最小化・最大化時のアニメーション視覚効果を無効にし、体感の UI 遅延を軽減します。",
            broken_features: &[],
        }),
        "disable_transparency" => Some(LocalizedTweakText {
            title: "透明効果を無効化",
            description: "タスクバー、スタートメニュー、アクションセンターのアクリル/透明効果を無効にし、GPU コンポジット負荷を解放します。",
            broken_features: &[
                "タスクバーとスタートメニューの半透明表示が無効になります",
            ],
        }),
        "disable_ntfs_last_access" => Some(LocalizedTweakText {
            title: "Windows アプリのバックグラウンド実行をグローバルで無効化",
            description: "UWP / Microsoft Store アプリが最小化中や終了後にバックグラウンドで勝手に起動・通信して CPU や RAM を消費するのを防ぎます。",
            broken_features: &[
                "ストアアプリを閉じるとバックグラウンド同期が停止します",
            ],
        }),
        "disable_nagle_algorithm" => Some(LocalizedTweakText {
            title: "MSMQ 用 Nagle アルゴリズム (TCPNoDelay) を無効化",
            description: "Microsoft Message Queuing 専用に Nagle アルゴリズム（MSMQ\\Parameters\\TCPNoDelay）を無効化し、MSMQ の小容量メッセージ処理におけるキュー待機遅延を削減します。",
            broken_features: &[
                "小さなパケットを扱う際のネットワーク帯域使用量がわずかに増える場合があります",
            ],
        }),
        "disable_sticky_keys" => Some(LocalizedTweakText {
            title: "固定キー機能およびアクセシビリティホットキーを無効化",
            description: "Shift キー 5 回押しの固定キー確認ダイアログ、フィルターキー、切り替えキーの警告音を無効にし、競技ゲーム中の意図しないポップアップや入力割り込みを防ぎます。",
            broken_features: &[
                "アクセシビリティのショートカット（Shift 5回、右 Shift 8秒長押し）が無効になります",
            ],
        }),
        "disable_mouse_shadow" => Some(LocalizedTweakText {
            title: "エクスプローラーのアイコン影と選択範囲の半透明効果を無効化",
            description: "デスクトップアイコン名のドロップシャドウとエクスプローラーの範囲選択矩形の半透明アルファブレンディングを無効化し、DWM GPU コンポジット負荷を軽減します。",
            broken_features: &[],
        }),
        "high_performance_power_plan" => Some(LocalizedTweakText {
            title: "プロセッサパフォーマンスブーストモードを「積極的」に設定",
            description: "PowerSchemes PerfBoostMode により AC 電源時のプロセッサパフォーマンスブーストモードを「積極的」(Aggressive) に設定し、対話型負荷時のブーストクロック抑制を防ぎます。",
            broken_features: &[
                "アイドル時の消費電力が増加します",
                "ノート PC の発熱が増加します",
            ],
        }),
        "disable_bing_search_start" => Some(LocalizedTweakText {
            title: "スタートメニューの Bing Web 検索を無効化",
            description: "Windows 検索がキー入力ごとに Bing サーバーへ送信するのを防ぎ、スタートメニューでのアプリ・ローカルファイル検索をクラウド遅延なしで瞬時に行います。",
            broken_features: &[
                "スタートメニューに Web 検索結果が表示されなくなります",
            ],
        }),
        "disable_windows_copilot" => Some(LocalizedTweakText {
            title: "Windows Copilot AI 統合を無効化",
            description: "グループポリシーによりシステム全体で Windows Copilot を無効化し、Copilot のバックグラウンド常駐エージェント、タスクバーボタン、AI ネットワーク通信を停止します。",
            broken_features: &[
                "Windows Copilot サイドバーとショートカット (Win+C) が無効になります",
            ],
        }),
        "disable_game_bar_presence_writer" => Some(LocalizedTweakText {
            title: "Game Bar Presence・Nexus ランチャー・起動ヒントを無効化",
            description: "ユーザー書き込み可能な HKCU\\Software\\Microsoft\\GameBar キーによりコントローラー Nexus 起動とスタートアップ案内を無効化し、TrustedInstaller 権限エラーなしでオーバーレイフック負荷を排除します。",
            broken_features: &[
                "Xbox ガイド Nexus ボタン起動と Game Bar 起動パネルが無効になります",
            ],
        }),
        "disable_widgets_news" => Some(LocalizedTweakText {
            title: "Windows ウィジェットと MSN ニュースフィードを無効化",
            description: "タスクバーのウィジェットアイコンを無効化し、MSN ニュースやトレンド・天気ウィジェットのバックグラウンド通信を防ぎます。",
            broken_features: &[
                "タスクバーのウィジェットフライアウトが無効になります",
            ],
        }),
        "gamedvr_config_store" => Some(LocalizedTweakText {
            title: "Xbox GameDVR 録画オーバーレイを無効化",
            description: "バックグラウンドのゲームプレイ録画と Xbox Game Bar フック注入を無効化し、フレームタイム低下やマイクロスタッターを解消します。",
            broken_features: &[
                "Xbox Game Bar のバックグラウンド即時リプレイ録画が無効になります",
            ],
        }),
        "gamedvr_app_capture" => Some(LocalizedTweakText {
            title: "Windows GameDVR アプリキャプチャを無効化",
            description: "Windows GameDVR サブシステム内のバックグラウンド音声/映像録画バッファを無効にします。",
            broken_features: &[],
        }),
        "gamedvr_policy_disable" => Some(LocalizedTweakText {
            title: "システムポリシーで GameDVR を無効化",
            description: "Windows がバックグラウンドで GameDVR キャプチャサービスを勝手に再有効化するのを防ぐシステム全体のグループポリシーロックを適用します。",
            broken_features: &[],
        }),
        "consumer_features_disable" => Some(LocalizedTweakText {
            title: "Windows コンシューマー向けクラウド機能を無効化",
            description: "メンテナンス中に Windows がサードパーティ製スポンサーアプリやゲーム（TikTok、Disney、Candy Crush 等）を自動ダウンロード・インストールするのを防ぎます。",
            broken_features: &[],
        }),
        "disable_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "フィードバック・Autochk・マップのテレメトリタスクを無効化",
            description: "Windows フィードバック Hub の SIUF 診断プロンプト、Autochk テレメトリプロキシ、バックグラウンドのマップ更新トースト通知タスクを無効化します（Sophia Script 準拠）。",
            broken_features: &[
                "フィードバック Hub のアンケート通知が表示されなくなります",
            ],
        }),
        "disable_hpet_synthetic_timer" => Some(LocalizedTweakText {
            title: "動的ティック（合成カーネルタイマー）を無効化",
            description: "Windows NT カーネルの動的ティック省電力を無効化し、合成タイマーのクロックドリフトを防いで高精度な Invariant TSC タイマー照会を強制し、極小 DPC 遅延を実現します。",
            broken_features: &[
                "アイドル時の CPU 消費電力がわずかに増加します",
            ],
        }),
        "optimize_tcp_congestion_tuning" => Some(LocalizedTweakText {
            title: "ネットワーク TCP 輻輳制御とウィンドウ自動チューニングを最適化",
            description: "RFC 1323 高性能 TCP タイムスタンプを有効化し、Web Services Dynamic Discovery (WSD) ブロードキャスト通信を無効にすることで、オンラインゲームのネットワーク遅延とパケット直列化遅延を抑えます。",
            broken_features: &[],
        }),
        "disable_windows_error_reporting" => Some(LocalizedTweakText {
            title: "Windows エラー報告のグループポリシーと送信を無効化",
            description: "Windows エラー報告のグループポリシーロックを適用し、Microsoft サーバーへの追加クラッシュダンプ診断データの送信を禁止します。",
            broken_features: &[
                "自動エラークラッシュレポートとテレメトリ送信が遮断されます",
            ],
        }),
        "disable_bing_search_start_menu" => Some(LocalizedTweakText {
            title: "スタートメニューのおすすめと Iris 広告を無効化",
            description: "Windows 10/11 スタートレイアウト内のスポンサー Web サイト・アカウント推奨、Iris ヒント、最近追加したアプリの不要な表示を削除します。",
            broken_features: &[
                "スタートメニュー内のプロモーション推奨表示が非表示になります",
            ],
        }),
        "disable_cortana_autostart" => Some(LocalizedTweakText {
            title: "ユーザー単位の Cortana 検索とタスクバーボタンを無効化",
            description: "現在のユーザーの Cortana 有効化と検索位置情報権限を無効にし、タスクバー上の Cortana エントリを非表示にします。",
            broken_features: &[
                "ユーザー単位の Cortana タスクバー検索連携が無効になります",
            ],
        }),
        "enable_long_file_paths" => Some(LocalizedTweakText {
            title: "NTFS Win32 長いファイルパス (>260 文字) を有効化",
            description: "LongPathsEnabled=1 により Windows 10/11 の旧来 260 文字 MAX_PATH 制限を解除し、ビルドやアーカイブ展開時のパス切り捨てエラーを防ぎます。",
            broken_features: &[],
        }),
        "disable_background_search_toggle_and_app_policy" => Some(LocalizedTweakText {
            title: "アプリのバックグラウンド実行と検索バックグラウンドトグルを禁止",
            description: "AppPrivacy\\LetAppsRunInBackground=2（強制拒否）を設定し、Search\\BackgroundAppGlobalToggle=0 を無効化して、待機中の UWP/Store アプリがバックグラウンドで CPU や RAM を消費しないようにします。",
            broken_features: &[
                "ストアやメールなどの UWP アプリは閉じている間同期されなくなります",
            ],
        }),
        "extend_webclient_file_size_limit" => Some(LocalizedTweakText {
            title: "WebClient / WebDAV ファイルサイズ上限を最大化 (4 GB)",
            description: "Windows WebClient サービスのダウンロード上限を 50 MB から最大 4 GB (0xFFFFFFFF) へ引き上げ、マウントした WebDAV やネットワーク共有でのコピー失敗を解消します。",
            broken_features: &[],
        }),
        "reduce_services_pipe_timeout" => Some(LocalizedTweakText {
            title: "サービス起動・パイプ終了タイムアウトを 15 秒へ短縮",
            description: "ServicesPipeTimeout を標準の 30000 ms（30 秒）から 15000 ms（15 秒）へ短縮し、応答停止したバックグラウンドサービスが起動時やシャットダウン時に素早く解放されるようにします。",
            broken_features: &[
                "起動に 15 秒以上かかる非常に遅いカスタム DB サービスがタイムアウトする場合があります",
            ],
        }),
        "mmcss_scheduler_full_optimization" => Some(LocalizedTweakText {
            title: "MMCSS マルチメディアスケジューラ完全最適化 (NoLazyMode, AlwaysOn, High SFIO)",
            description: "Multimedia\\SystemProfile の NoLazyMode=1、AlwaysOn=1、および Games タスクの Scheduling Category='High' と SFIO Priority='High' を設定し、途切れないマルチメディア・ゲームスレッド制御を実現します。",
            broken_features: &[],
        }),
        "enable_directx_windowed_game_optimizations" => Some(LocalizedTweakText {
            title: "DirectX ウィンドウ化フリップモデル & VRR 最適化を有効化（無損失マージ）",
            description: "DirectXUserGlobalSettings 内の個別の GPU や AutoHDR 設定を上書きすることなく、SwapEffectUpgradeEnable=1 と VRROptimizeEnable=1 を安全にマージします。",
            broken_features: &[],
        }),
        "disable_hardware_overlays_and_min_fps" => Some(LocalizedTweakText {
            title: "カーネルハードウェアオーバーレイと DWM 最小 FPS 制限を無効化",
            description: "GraphicsDrivers\\DisableOverlays=1 と Dwm\\OverlayMinFPS=0 を設定し、低リフレッシュレート時の DWM ハードウェアオーバーレイ昇格による停止とスタッターを防止します。",
            broken_features: &[
                "ノート PC で待機時の DWM コンポジター電力消費がわずかに増加します",
            ],
        }),
        "nvidia_restore_legacy_sharpening" => Some(LocalizedTweakText {
            title: "従来の NVIDIA ドライバシャープ化ポリシーを復元 (EnableGR535=0)",
            description: "Global\\FTS\\EnableGR535=0 を設定し、新しいオーバーレイフィルター経路を強制せず NVIDIA コントロールパネルに低負荷な従来の「イメージシャープニング」を表示させます。",
            broken_features: &[],
        }),
        "nvidia_dlss_debug_indicator" => Some(LocalizedTweakText {
            title: "NVIDIA DLSS バージョン & プリセットデバッグ透かしを表示",
            description: "Global\\NGXCore\\ShowDlssIndicator=1024 を設定し、DirectX/Vulkan ゲーム画面上に有効な DLSS SDK バージョン・解像度スケーリング・プリセットを表示します。",
            broken_features: &[
                "対応ゲームの左下に DLSS 診断用透かしが表示されます",
            ],
        }),
        "disable_game_bar_tips_and_capture" => Some(LocalizedTweakText {
            title: "Game Bar コントローラー Nexus・起動ヒント・履歴キャプチャを無効化",
            description: "UseNexusForGameBarEnabled（Xbox ガイドボタン起動）、ShowStartupPanel のヒント、およびバックグラウンドの HistoricalCaptureEnabled リングバッファ録画をオフにします。",
            broken_features: &[
                "コントローラーの Nexus ボタンで Game Bar オーバーレイが開かなくなります",
            ],
        }),
        "optimize_mouse_thresholds_and_hover" => Some(LocalizedTweakText {
            title: "マウス加速しきい値をゼロ化し、ホバー応答を 10ms 高速化",
            description: "MouseThreshold1='0'、MouseThreshold2='0'、MouseHoverTime='10' ms を設定し、ポインター加速しきい値を完全にフラット化してタスクバーやツールチップの反応を瞬時にします。",
            broken_features: &[],
        }),
        "disable_extra_accessibility_hotkeys" => Some(LocalizedTweakText {
            title: "マウスキー・ハイコントラスト・ナレーターの誤爆ホットキーを無効化",
            description: "ゲーム中に誤って押しやすい Alt+Shift+NumLock（マウスキー）、Alt+Shift+PrtScn（ハイコントラスト）、Win+Ctrl+Enter（ナレーター）のショートカットポップアップを無効化します。",
            broken_features: &[
                "アクセシビリティのキーボードショートカットは「設定」アプリから切り替える必要があります",
            ],
        }),
        "disable_smart_multi_homed_name_resolution" => Some(LocalizedTweakText {
            title: "Windows スマートマルチホーム DNS 名前解決 (SMHNR) を無効化",
            description: "DNSClient\\DisableSmartNameResolution=1 を設定し、VPN や仮想アダプターへ並列に DNS 問い合わせをブロードキャストせず、プライマリインターフェイスのみに DNS 照会を行います。",
            broken_features: &[
                "スプリットトンネル VPN ではアダプターの DNS メトリックを適切に設定する必要があります",
            ],
        }),
        "harden_windows_update_ux_and_store" => Some(LocalizedTweakText {
            title: "WU のドライバ上書き・意図しない再起動・Store 自動DL を遮断",
            description: "品質更新時の GPU/周辺機器ドライバ配信を除外し、サインイン中の強制再起動を防ぎ、最大 3650 日の更新一時停止を解放し、セキュリティ更新を維持したまま Store のサイレント自動ダウンロードを無効化します。",
            broken_features: &[
                "Microsoft Store アプリの更新は手動で確認する必要があります",
            ],
        }),
        "power_settings_pcie_usb_nvme_epb" => Some(LocalizedTweakText {
            title: "現在の電源プランを超低遅延チューニング (PCIe ASPM, USB 休止, NVMe, EPP, 電力スロットリング)",
            description: "ネイティブ Windows powrprof API を使用して現在の電源プラン（バランス、高パフォーマンス、カスタムプラン対応）に対し、AC 接続時の PCIe ASPM・USB セレクティブサスペンド・USB 3 省電力・NVMe アイドル休止を無効化し、CPU EPP=0 とカーネル PowerThrottling 無効化を適用します。",
            broken_features: &[
                "デスクトップおよび AC 接続時のノート PC でアイドル消費電力が増加します",
            ],
        }),
        "disable_storage_sense_policy" => Some(LocalizedTweakText {
            title: "ストレージセンサーの自動バックグラウンド掃除を無効化",
            description: "対話セッション中に Windows ストレージセンサーがバックグラウンドで自動ディスククリーンアップを実行しないようにします。",
            broken_features: &[
                "一時ファイルとごみ箱は「クリーン&アプリ」から手動で削除する必要があります",
            ],
        }),
        "consolidate_svchost_split_threshold" => Some(LocalizedTweakText {
            title: "搭載物理 RAM に合わせてサービスホストを動的統合 (SvcHostSplitThresholdInKB) [任意]",
            description: "検出された物理 RAM (TotalRamInKb) から SvcHostSplitThresholdInKB を動的に計算し、8GB 以上の環境でプロセス数と仮想メモリ負荷を削減します。注意：サービスごとのプロセス分離は緩和されます。",
            broken_features: &[
                "共有 svchost のクラッシュ分離が低下し、タスクマネージャー上のグループ化サービス表示数が減ります",
            ],
        }),
        "disable_vbs_deviceguard" => Some(LocalizedTweakText {
            title: "仮想化ベースのセキュリティ (VBS) を無効化 [セキュリティとのトレードオフ]",
            description: "EnableVirtualizationBasedSecurity と RequirePlatformSecurityFeatures を無効化します。仮想化オーバーヘッドがある CPU でゲームのフレームレートを 5〜15% 向上させる一方、カーネル仮想化セキュリティ境界を解除します。",
            broken_features: &[
                "VBS セキュリティ分離が無効になり、アンチチートや Windows セキュリティの警告が表示される場合があります",
            ],
        }),
        "disable_hvci_memory_integrity" => Some(LocalizedTweakText {
            title: "コア分離 / メモリ整合性 (HVCI) を無効化 [セキュリティとのトレードオフ]",
            description: "ハイパーバイザー保護されたコード整合性 (HVCI) を無効にします。HVCI がフレームタイムに悪影響を与える環境で競技ゲームの DPC・割り込み遅延を削減する一方、カーネルドライバコード署名の強制検証をオフにします。",
            broken_features: &[
                "Windows セキュリティの「コア分離 / メモリ整合性」にオフ警告が表示されます",
            ],
        }),
        "disable_prefetch_and_sysmain" => Some(LocalizedTweakText {
            title: "高速 NVMe SSD 向けにカーネルプリフェッチと SysMain (Superfetch) を無効化 [任意]",
            description: "RAM キャッシュが不要なバックグラウンドディスク I/O を増やす高速 NVMe ドライブ向けに、アプリ/ブートプリフェッチャーと SysMain メモリ先読みを無効化します。",
            broken_features: &[
                "SATA HDD 環境ではアプリのコールド起動がわずかに遅くなる場合があります",
            ],
        }),
        "disable_advertising_id" => Some(LocalizedTweakText {
            title: "広告識別子を無効化",
            description: "ターゲティング広告やアプリ間トラッキングのために Windows がアプリへ提供するユーザーごとの広告 ID を無効化します。",
            broken_features: &[
                "アプリが固有の広告 ID を取得できなくなります",
            ],
        }),
        "disable_cortana" => Some(LocalizedTweakText {
            title: "Cortana を無効化",
            description: "グループポリシーにより Cortana を無効化し、起動・音声待機・Microsoft クラウドへのデータ送信を停止します。",
            broken_features: &[
                "Cortana 音声アシスタントが無効になります",
            ],
        }),
        "disable_activity_history" => Some(LocalizedTweakText {
            title: "アクティビティの履歴 / タイムラインを無効化",
            description: "Windows アクティビティ履歴（タイムライン）機能を無効にし、操作履歴・閲覧サイト・開いたドキュメントの記録と保存を防止します。",
            broken_features: &[
                "タスクビューのタイムライン/アクティビティ履歴が空になります",
            ],
        }),
        "disable_online_speech_recognition" => Some(LocalizedTweakText {
            title: "オンライン音声認識を無効化",
            description: "クラウドベースのオンライン音声認識サービスを無効化し、音声データが Microsoft サーバーへ送信されないようにします。",
            broken_features: &[
                "クラウドベースの音声認識は機能しなくなります（ローカル/オフライン認識は影響を受けません）",
            ],
        }),
        "disable_tailored_experiences" => Some(LocalizedTweakText {
            title: "カスタマイズされたエクスペリエンスを無効化",
            description: "Microsoft が診断データを使用して Windows 内にパーソナライズされたヒント・広告・おすすめを表示するのを防ぎます。",
            broken_features: &[
                "パーソナライズされた Windows の提案とヒントが無効になります",
            ],
        }),
        "disable_windows_recall" => Some(LocalizedTweakText {
            title: "Windows Recall（AI スナップショット）を無効化",
            description: "Windows 11 24H2 で導入された、作業内容の AI インデックス付きスクリーンショットを定期撮影する Windows Recall 機能を無効にします。ビルド 26100 以降が必要です。",
            broken_features: &[
                "Windows Recall AI スナップショット機能が無効になります",
            ],
        }),
        "disable_cloud_content_spotlight" => Some(LocalizedTweakText {
            title: "クラウドコンテンツとスポットライトのおすすめを無効化",
            description: "Windows スポットライト、設定内のおすすめコンテンツ、およびクラウド経由で配信されるプリインストールアプリの推奨を無効にします。",
            broken_features: &[
                "ロック画面のスポットライト画像が無効になります",
                "スタートメニューのおすすめアプリが表示されなくなります",
            ],
        }),
        "disable_location_tracking" => Some(LocalizedTweakText {
            title: "位置情報の追跡を無効化",
            description: "Windows 位置情報サービスをグローバルで無効化し、アプリや OS による地理的位置情報へのアクセスを遮断します。",
            broken_features: &[
                "位置情報を必要とするアプリ（マップ、天気など）が現在地を取得できなくなります",
            ],
        }),
        "disable_activity_cloud_sync" => Some(LocalizedTweakText {
            title: "クラウドクリップボードとデバイス間同期を無効化",
            description: "グループポリシーで Windows クリップボード履歴の永続化とデバイス間のクラウド同期を無効にし、コピーしたパスワードや機密データをローカルのみに留めます。",
            broken_features: &[
                "Win+V クリップボード履歴とデバイス間の貼り付け同期が無効になります",
            ],
        }),
        "disable_cdp_service" => Some(LocalizedTweakText {
            title: "Connected Devices Platform サービスを無効化",
            description: "Connected Devices Platform Service (CDPUserSvc) を無効化し、デバイス間テレメトリ、アプリ同期ブロードキャスト、バックグラウンド活動ビーコンを停止します。",
            broken_features: &[
                "近距離共有とモバイルデバイスとのクロスデバイス同期が無効になります",
            ],
        }),
        "spotlight_desktop_disable" => Some(LocalizedTweakText {
            title: "Windows スポットライト ロック画面のダウンロードを無効化",
            description: "Windows スポットライトのロック画面壁紙および Microsoft プロモーション画像のバックグラウンド自動取得を無効にします。",
            broken_features: &[],
        }),
        "spotlight_features_disable" => Some(LocalizedTweakText {
            title: "ロック画面のヒントと広告を無効化",
            description: "Content Delivery Manager 経由で配信されるロック画面のヒント・おすすめ・Microsoft Store スポンサーアプリ広告を無効にします。",
            broken_features: &[],
        }),
        "win32_priority_separation" => Some(LocalizedTweakText {
            title: "Win32PrioritySeparation — フォアグラウンド優先ブースト",
            description: "スケジューラクォンタムを「短・可変・高フォアグラウンドブースト」(0x26 = 38) に設定します。前面アプリを優先し、ゲームや対話型処理の入力遅延を低減します。",
            broken_features: &[],
        }),
        "menu_show_delay" => Some(LocalizedTweakText {
            title: "メニュー表示遅延を 0 ms に短縮",
            description: "マウスホバー時にメニューが表示されるまでの人為的な待機時間を削除します。既定は 400 ms で、0 にするとメニューが即座に開きます。",
            broken_features: &[],
        }),
        "mmcss_gpu_priority" => Some(LocalizedTweakText {
            title: "MMCSS — ゲームと Pro Audio の優先度を向上",
            description: "Multimedia Class Scheduler Service (MMCSS) 内で Games および Pro Audio スレッドの優先度を引き上げ、音声とリアルタイムゲームスレッドが確実に高いスケジューリング優先度を得られるようにします。",
            broken_features: &[],
        }),
        "mouse_data_queue_size" => Some(LocalizedTweakText {
            title: "マウスデータキューサイズを拡大",
            description: "マウス入力データキューを既定の 100 件から 256 件バッファへ拡大し、高ポーリングレートのゲーミングマウスで入力イベントが取りこぼされる可能性を低減します。",
            broken_features: &[],
        }),
        "keyboard_response" => Some(LocalizedTweakText {
            title: "キーボード応答速度を最適化",
            description: "レジストリ値によりキーボードのリピート遅延を最短、リピート速度を最速に設定し、キーボードデータキューサイズを拡大します。",
            broken_features: &[],
        }),
        "disable_wer_queue" => Some(LocalizedTweakText {
            title: "Windows エラー報告キューを無効化",
            description: "Windows エラー報告 (WER) キューを無効化し、アプリの異常終了後にバックグラウンドでクラッシュダンプやレポートが書き込まれるのを防ぎます。",
            broken_features: &[
                "クラッシュレポートが Microsoft へ送信されなくなります",
            ],
        }),
        "auto_end_tasks" => Some(LocalizedTweakText {
            title: "シャットダウン時に応答なしタスクを自動終了",
            description: "シャットダウン時に確認画面を出さず応答のないアプリを自動終了します。また WaitToKillAppTimeout と HungAppTimeout を短縮し、サインアウトやシャットダウンを高速化します。",
            broken_features: &[
                "シャットダウン時に応答のないアプリ内の未保存作業が失われる可能性があります",
            ],
        }),
        "network_throttling_disable" => Some(LocalizedTweakText {
            title: "ネットワークスロットリング指数を無効化",
            description: "Windows のマルチメディアネットワーク帯域制限を無効化（NetworkThrottlingIndex を 0xFFFFFFFF に設定）し、高負荷通信やゲーム中のパケットドロップ・遅延スパイクを防ぎます。",
            broken_features: &[],
        }),
        "system_responsiveness_gaming" => Some(LocalizedTweakText {
            title: "フォアグラウンドのシステム応答性を最大化",
            description: "SystemResponsiveness を 0% に設定し、バックグラウンド OS タスク用に 20% を予約する代わりに CPU 処理サイクルの 100% を前面のゲームとアプリへ割り当てます。",
            broken_features: &[],
        }),
        "ntfs_disable_last_access" => Some(LocalizedTweakText {
            title: "NTFS 最終アクセス日時タイムスタンプを無効化",
            description: "ファイル読み取りのたびに NTFS ファイルシステムがディスクメタデータのタイムスタンプを更新しないようにし、SSD や NVMe の読み書き I/O 負荷を大幅に削減します。",
            broken_features: &[
                "ファイルのプロパティに正確な最終アクセス日時が表示されなくなります",
            ],
        }),
        "disable_paging_executive" => Some(LocalizedTweakText {
            title: "Windows カーネルとドライバを RAM に常駐",
            description: "Windows NT エグゼクティブカーネルとドライバルーチンをディスクへページアウトさせず物理 RAM に常駐させ、即時のカーネル割り込み応答を保証します。",
            broken_features: &[],
        }),
        "large_system_cache" => Some(LocalizedTweakText {
            title: "大容量システムファイルキャッシュを有効化",
            description: "物理 RAM 内により大きなファイルシステムキャッシュを維持するよう Windows を構成し、反復的なファイルアクセスとゲームテクスチャストリーミングを高速化します。",
            broken_features: &[],
        }),
        "disable_fast_startup" => Some(LocalizedTweakText {
            title: "高速スタートアップを無効化（ハイブリッドスリープのリーク防止）",
            description: "Windows 高速スタートアップ (hiberboot) を無効化し、毎回のシャットダウンで完全なクリーンブートを実行してセッション間のカーネルメモリリークやドライバ状態破損を防ぎます。",
            broken_features: &[
                "低速な HDD 環境ではコールドブートが 1〜2 秒遅くなる場合があります",
            ],
        }),
        "ntfs_disable_8dot3" => Some(LocalizedTweakText {
            title: "NTFS 8.3 形式の短いファイル名生成を無効化",
            description: "NTFS ボリュームでの古い MS-DOS 8.3 形式ショートファイル名生成を無効にし、高速 NVMe/SSD におけるディレクトリ索引遅延と書き込みオーバーヘッドを排除します。",
            broken_features: &[],
        }),
        "increase_mft_zone_reservation" => Some(LocalizedTweakText {
            title: "マスターファイルテーブル (MFT) ゾーンバッファを拡大",
            description: "NTFS マスターファイルテーブル用の連続ディスクバッファをより大きく確保し、MFT 断片化を防いで最大のディスクアクセススループットを維持します。",
            broken_features: &[],
        }),
        "tcp_ack_frequency" => Some(LocalizedTweakText {
            title: "TCP 低遅延化 (TcpAckFrequency)",
            description: "Windows TCP ネットワークスタックに対しタイムアウト結合を待たず即座に ACK パケットを送信させ、オンラインゲームのネットワーク Ping を削減します。",
            broken_features: &[],
        }),
        "tcp_no_delay_global" => Some(LocalizedTweakText {
            title: "Nagle アルゴリズムをグローバル無効化 (TCPNoDelay)",
            description: "有効なネットワークインターフェイス全体で Nagle のパケットバッファリングアルゴリズムを無効化し、小パケットを即座に送信して往復遅延を最小限に抑えます。",
            broken_features: &[],
        }),
        "disable_mouse_acceleration" => Some(LocalizedTweakText {
            title: "「ポインターの精度を高める」を無効化（1:1 マウス入力）",
            description: "非線形な Windows カーソル加速を無効化し、競技性の高い精密エイムに不可欠な 1:1 のハードウェア生マウス移動を実現します。",
            broken_features: &[],
        }),
        "disable_nvidia_telemetry_service" => Some(LocalizedTweakText {
            title: "NVIDIA テレメトリコンテナサービスを無効化",
            description: "NVIDIA ドライバの使用統計やクラッシュデータを収集・送信する NvTelemetryContainer サービスを無効化します。",
            broken_features: &[
                "NVIDIA クラッシュデータおよび使用状況テレメトリが収集されなくなります",
            ],
        }),
        "disable_nvidia_telemetry_tasks" => Some(LocalizedTweakText {
            title: "NVIDIA テレメトリ予定タスクを無効化",
            description: "NVIDIA テレメトリデータを定期的に送信する NvTmMon、NvTmRep、NvProfile のスケジュールタスクを無効化します。",
            broken_features: &[],
        }),
        "disable_adobe_update_service" => Some(LocalizedTweakText {
            title: "Adobe 更新および正規品監視サービスを無効化",
            description: "Adobe Acrobat Update Service (AdobeARMservice) および Adobe Genuine Monitor Service を無効化し、バックグラウンドの更新確認とライセンス監視テレメトリを停止します。",
            broken_features: &[
                "Adobe 製品の自動更新が停止します。手動で更新してください",
            ],
        }),
        "disable_edge_update" => Some(LocalizedTweakText {
            title: "Microsoft Edge 更新サービスを無効化",
            description: "Microsoft Edge 更新サービス（edgeupdate および edgeupdatem）と関連更新タスクを無効化します。Edge 自体は動作しますが、自動更新や更新確認通信は行われません。",
            broken_features: &[
                "Microsoft Edge が自動更新されなくなります。セキュリティパッチは手動で適用してください",
            ],
        }),
        "disable_chrome_update" => Some(LocalizedTweakText {
            title: "Google Chrome 更新サービスを無効化",
            description: "Google Update サービス（gupdate および gupdatem）と関連スケジュールタスクを無効化します。Chrome 自体は動作しますが、自動更新は行われません。",
            broken_features: &[
                "Google Chrome が自動更新されなくなります。セキュリティパッチは手動で適用してください",
            ],
        }),
        "disable_steam_client_service" => Some(LocalizedTweakText {
            title: "Steam Client Service の自動起動を無効化",
            description: "Steam Client Service を自動起動から手動起動へ変更します。Steam 起動時には必要に応じて開始されますが、未起動時にバックグラウンドで常駐・消費しなくなります。",
            broken_features: &[],
        }),
        "disable_razer_telemetry" => Some(LocalizedTweakText {
            title: "Razer テレメトリおよびアナリティクスを無効化",
            description: "Razer Game Manager Service と Razer Chroma SDK サービスの自動起動を無効化し、Razer Synapse によるバックグラウンドのテレメトリと分析送信を抑制します。",
            broken_features: &[
                "Razer Synapse の自動起動関連機能は手動起動が必要になる場合があります",
            ],
        }),
        "disable_diagtrack" => Some(LocalizedTweakText {
            title: "DiagTrack（接続ユーザーエクスペリエンスとテレメトリ）を無効化",
            description: "Windows の主要なテレメトリ収集サービスである Connected User Experiences and Telemetry サービス (DiagTrack / diagsvc) を無効化します。",
            broken_features: &[
                "診断データが Microsoft へ送信されなくなります",
            ],
        }),
        "disable_dmwappushservice" => Some(LocalizedTweakText {
            title: "dmwappushservice（WAP プッシュメッセージルーティング）を無効化",
            description: "テレメトリデータのルーティングに使用されるデバイス管理ワイヤレスアプリケーションプロトコル (WAP) プッシュルーティングサービスを無効化します。",
            broken_features: &[
                "MDM プッシュメッセージが受信できなくなります",
            ],
        }),
        "disable_telemetry_registry" => Some(LocalizedTweakText {
            title: "AllowTelemetry レジストリ設定でテレメトリを無効化",
            description: "AllowTelemetry ポリシーを最小レベルの 0（セキュリティ/オフ）に設定します。Enterprise および Education では完全に無効化され、Home および Pro では最小限に抑えられます。",
            broken_features: &[
                "一部の Windows Insider および診断機能が動作しない場合があります",
            ],
        }),
        "disable_compat_telrunner" => Some(LocalizedTweakText {
            title: "アプリケーション互換性評価タスク (CompatTelRunner) を無効化",
            description: "インストール済みソフトをスキャンして互換性テレメトリを送信する Microsoft Compatibility Appraiser および ProgramDataUpdater タスク（CompatTelRunner.exe）を無効化します。",
            broken_features: &[
                "Windows アップグレード互換性チェックが自動実行されなくなります",
            ],
        }),
        "disable_ceip" => Some(LocalizedTweakText {
            title: "カスタマーエクスペリエンス向上プログラム (CEIP) を無効化",
            description: "CEIP スケジュールタスク（Consolidator、UsbCeip、KernelCeipTask）および CEIP ポリシーレジストリキーを無効化します。",
            broken_features: &[
                "CEIP テレメトリデータが収集されなくなります",
            ],
        }),
        "disable_wer_service" => Some(LocalizedTweakText {
            title: "Windows エラー報告サービス (WerSvc) を無効化",
            description: "Windows Error Reporting サービス (WerSvc) を無効化し、クラッシュレポートやエラーテレメトリが Microsoft へアップロードされるのを防ぎます。",
            broken_features: &[
                "クラッシュレポートが Microsoft へ送信されなくなります",
                "「問題のレポート」履歴が更新されなくなります",
            ],
        }),
        "disable_connected_user_experiences" => Some(LocalizedTweakText {
            title: "接続ユーザーエクスペリエンスとおすすめ表示を無効化",
            description: "クラウドコンテンツやテレメトリフィードバックに基づく、設定アプリ・スタートメニュー・ロック画面での提案やヒントを無効化します。",
            broken_features: &[
                "Windows の提案やヒントが表示されなくなります",
            ],
        }),
        "disable_diagnostic_data_viewer" => Some(LocalizedTweakText {
            title: "診断データビューアーを無効化",
            description: "Microsoft へ送信される診断データのローカルコピーを保存する診断データビューアーのイベントログ記録を無効化し、ディスク I/O と容量を節約します。",
            broken_features: &[
                "「診断データビューアー」アプリでローカルから診断データを確認できなくなります",
            ],
        }),
        "disable_delivery_optimization_p2p" => Some(LocalizedTweakText {
            title: "配信の最適化の P2P アップロード帯域消費を無効化",
            description: "Windows 配信の最適化を制限（DODownloadMode = 0）し、インターネット上の他 PC へ Windows Update を配信するために上り帯域が使用されるのを停止します。",
            broken_features: &[],
        }),
        "disable_diagnostic_execution_service" => Some(LocalizedTweakText {
            title: "Diagnostic Execution Service (diagsvc) を無効化",
            description: "定期的なバックグラウンド診断およびテレメトリパッケージを実行する Diagnostic Execution Service (diagsvc) を無効化します。",
            broken_features: &[],
        }),
        "wersvc_service_disable" => Some(LocalizedTweakText {
            title: "Diagnostics Hub Standard Collector を無効化",
            description: "Visual Studio / Windows Diagnostics Hub Standard Collector サービスを無効化し、アイドル時のバックグラウンド ETW トレース収集を停止します。",
            broken_features: &[
                "VS / Windows Diagnostics Hub のトレース収集には手動起動が必要になります",
            ],
        }),
        "disable_remote_registry" => Some(LocalizedTweakText {
            title: "Remote Registry サービスを無効化",
            description: "RemoteRegistry サービスを無効化し、リモートユーザーや外部ネットワークからローカルレジストリキーを参照・変更できないようにします。",
            broken_features: &[
                "ローカル LAN 経由のリモートレジストリ管理が無効になります",
            ],
        }),
        "disable_disk_diagnostic_task" => Some(LocalizedTweakText {
            title: "ディスク診断テレメトリタスクを無効化",
            description: "テレメトリ報告用にディスク使用状況メトリクスとハードウェア診断を収集する予定タスクを無効化します。",
            broken_features: &[],
        }),
        "disable_winsat_task" => Some(LocalizedTweakText {
            title: "WinSAT バックグラウンドベンチマークタスクを無効化",
            description: "システムアイドル時に Windows システム評価ツール (WinSAT) タスクが断りなくディスクや GPU の負荷テストを実行しないようにします。",
            broken_features: &[],
        }),
        "enable_classic_context_menu" => Some(LocalizedTweakText {
            title: "従来の Windows 10 フル右クリックコンテキストメニューを復元",
            description: "HKCU\\Software\\Classes 配下に CLSID オーバーライドを登録し、「その他のオプションを表示」をクリックせずに最初から完全な右クリックメニューを開けるようにします。",
            broken_features: &[
                "Windows 11 のコンパクトなコンテキストメニューが従来のフルメニューに置き換わります",
            ],
        }),
        "explorer_power_user_defaults" => Some(LocalizedTweakText {
            title: "拡張子表示・タイトルバーフルパス・PC 既定起動・同期案内非表示を有効化",
            description: "登録済みファイルの拡張子と隠しファイルを表示し、エクスプローラーを「クイックアクセス」ではなく「PC」で開き、タイトルバーのフルパス表示・コンパクト間隔を有効化し、OneDrive 同期バナーを非表示にします。",
            broken_features: &[],
        }),
        "explorer_hide_win11_home_gallery_nav" => Some(LocalizedTweakText {
            title: "Win11 エクスプローラーのナビゲーションから「ホーム」と「ギャラリー」を非表示",
            description: "HKCU\\Software\\Classes\\CLSID 経由でエクスプローラー左ナビゲーションウィンドウから Windows 11 の「ホーム」と「ギャラリー」エントリの固定を解除します。",
            broken_features: &[
                "エクスプローラー左サイドバーから「ホーム」と「ギャラリー」のリンクが非表示になります",
            ],
        }),
        "explorer_disable_startup_delay_and_jpeg_compression" => Some(LocalizedTweakText {
            title: "アプリ起動遅延を排除し、壁紙の JPEG 圧縮を無効化",
            description: "Explorer\\Serialize\\StartupDelayInMSec=0 を設定してスタートアップアプリが人為的な 10 秒待機なしで起動するようにし、JPEGImportQuality=100 で無劣化のデスクトップ壁紙を適用します。",
            broken_features: &[],
        }),
        "explorer_desktop_this_pc_and_disable_autoplay" => Some(LocalizedTweakText {
            title: "デスクトップに「PC」アイコンを表示し、リムーバブルメディア自動再生を無効化",
            description: "デスクトップ上に「PC」ショートカットアイコンを表示し、USB メモリや光学ドライブの Autoplay/AutoRun を無効にして自動ポップアップや自動実行の脅威を遮断します。",
            broken_features: &[
                "USB や SD メディア接続時に自動再生ダイアログが開かなくなります",
            ],
        }),
        "disable_visual_effects_userpreferencesmask_bits" => Some(LocalizedTweakText {
            title: "UserPreferencesMask ビットマスク & DWM で重い UI アニメ/影を削減",
            description: "フォント平滑化とデスクトップ基本表示を保ちつつ UserPreferencesMask の該当ビット（メニューフェード/スライド、ツールチップフェード、選択フェード、カーソル影、コンボボックススライド、リスト滑らかスクロール、コントロールアニメ）を外科的に無効化し、EnableAeroPeek=0 を設定します。",
            broken_features: &[
                "スライド/フェードアニメやドロップシャドウのない軽快なクラシック UI に切り替わります",
            ],
        }),
        "taskbar_win11_power_layout_and_end_task" => Some(LocalizedTweakText {
            title: "Win11 タスクバー左寄せ・ウィジェット/タスクビュー/検索非表示・タスク終了有効化 (Pro/Ent)",
            description: "タスクバーアイコンを左寄せ (TaskbarAl=0) にし、タスクビューとウィジェットボタン (TaskbarDa=0) および検索ボックスを非表示、開発者設定の右クリック「タスクの終了」を有効化、Aero Shake を無効化し、スタートメニューの「おすすめ」を削除します（Win11 22H2+ Pro/Enterprise/Education）。",
            broken_features: &[
                "タスクバーアイコンが左寄せになり、タスクビューとウィジェットボタンが非表示になります",
            ],
        }),
        "hide_win10_meet_now_tray" => Some(LocalizedTweakText {
            title: "Windows 10 トレイの「今すぐ会議 (Meet Now)」カメラアイコンを非表示",
            description: "Windows 10 通知領域から Skype「今すぐ会議」(Meet Now) アイコンを非表示にします（Windows 10 ビルドのみ対象）。",
            broken_features: &[],
        }),
        "enable_windows_dark_mode_and_clock_seconds" => Some(LocalizedTweakText {
            title: "Windows システム & アプリのダークモード + タスクバー時計の秒表示を有効化",
            description: "AppsUseLightTheme=0 と SystemUsesLightTheme=0 を設定して Windows UI 全体にネイティブダークモードを適用し、タスクバーの時計に秒を表示します。",
            broken_features: &[],
        }),
        "disable_startup_sound_and_audio_ducking" => Some(LocalizedTweakText {
            title: "Windows 起動サウンドと通話時の自動音量低下を無効化",
            description: "Windows サインイン時の起動音を消音し、Discord・Teams・音声通話アプリ利用時にゲームや音楽の音量が 80% 自動で下げられるのを防ぎます。",
            broken_features: &[],
        }),
        "enable_verbose_logon_status" => Some(LocalizedTweakText {
            title: "Windows 起動 / サインイン / シャットダウンの詳細ステータスメッセージを有効化",
            description: "VerboseStatus=1 を設定し、汎用スピナー画面の代わりに具体的なサービスや更新の進行段階を表示します。",
            broken_features: &[],
        }),
        "disable_lock_screen_pro" => Some(LocalizedTweakText {
            title: "サインイン前のロック画面カーテンをバイパス (Pro/Enterprise ポリシー)",
            description: "起動後やロック解除時にロック画面の写真をスワイプして解除する手間を省き、直接パスワード/PIN 入力画面へ移動します。",
            broken_features: &[
                "Pro / Enterprise エディションでロック画面のスライドアップカーテンがスキップされます",
            ],
        }),
        "enable_windows_developer_mode" => Some(LocalizedTweakText {
            title: "Windows 開発者モードを有効化（サイドロードアプリとシンボリックリンク）",
            description: "AllowDevelopmentWithoutDevLicense=1 を有効にし、UAC なしでシンボリックリンクの作成や署名なし MSIX/AppX パッケージのサイドロードを可能にします。",
            broken_features: &[
                "署名なし MSIX サイドロードが許可されます（開発者向け機能）",
            ],
        }),
        // === system_deep_opt.toml (30) ===
        "disable_dwm_animations_per_process" => Some(LocalizedTweakText {
            title: "プロセスごとの DWM トランジションアニメーションを無効化",
            description: "DWM キー下の DisallowAnimations=1 および AnimationsShiftKey=1 を設定して、プロセスごとの DWM トランジションアニメーションを無効化します。ウィンドウの最小化、最大化、切り替え時のフェードやスライド効果を排除し、GPU コンポジターの負荷を軽減してウィンドウ操作を瞬時にします。",
            broken_features: &["ウィンドウのトランジションアニメーションが削除されます"],
        }),
        "disable_dwm_composition_throttle" => Some(LocalizedTweakText {
            title: "デスクトップウィンドウマネージャーのコンポジション調整を無効化",
            description: "DWM キー下の DesktopCompositionThrottle=0 を設定し、フレームドロップ発生時にコンポジターがレンダリングを抑制するのを防止します。GPU 負荷時でも DWM が最大のコンポジションスループットを維持できるようにします。",
            broken_features: &[],
        }),
        "disable_kernel_mitigations_audit" => Some(LocalizedTweakText {
            title: "カーネル緩和策の監査モードを無効化",
            description: "MitigationAuditOptions をすべてゼロに設定して、カーネルエクスプロイト緩和策の監査ログを無効化します。プロセス生成や例外処理時の ETW イベント発生を抑制し、パフォーマンスを向上させます。",
            broken_features: &["エクスプロイト緩和策の監査イベントはログに記録されなくなります"],
        }),
        "disable_llmnr_protocol" => Some(LocalizedTweakText {
            title: "LLMNR 名前解決プロトコルを無効化",
            description: "DNS クライアント設定で EnableMulticast=0 を指定し、リンクローカルマルチキャスト名前解決 (LLMNR) を無効化します。ローカルサブネットへのブロードキャストを停止し、セキュリティリスク (LLMNR ポイズニング) と不要なネットワークトラフィックを削減します。",
            broken_features: &["DNS のないローカルネットワークでの名前解決に失敗する可能性があります", "ローカルデバイスの自動検出が機能しなくなる可能性があります"],
        }),
        "disable_memory_compression" => Some(LocalizedTweakText {
            title: "メモリ圧縮エンジンを無効化",
            description: "DisableCompression=1 を設定して Windows メモリマネージャーの圧縮ストアを無効化します。16GB 以上の RAM を搭載した環境では、System プロセスのメモリ圧縮スレッドによる CPU 使用率スパイクを防止できます。",
            broken_features: &["メモリ容量が少ない環境でのページファイル使用量が増加します", "8GB システムでは早期にメモリ不足が発生する可能性があります"],
        }),
        "disable_netbios_over_tcp" => Some(LocalizedTweakText {
            title: "NetBIOS over TCP/IP を無効化",
            description: "NetbiosOptions=2 を設定し LMHosts 参照を無効化することで NetBIOS over TCP/IP を無効化します。レガシーなブロードキャスト名前解決を停止し、不要なネットワーク負荷と NBNS ポイズニング攻撃のリスクを排除します。",
            broken_features: &["NetBIOS 名前解決に依存するレガシーアプリケーションが失敗します", "WINS を使用する古いネットワークプリンターにアクセスできなくなる可能性があります"],
        }),
        "disable_ntfs_encryption_driver" => Some(LocalizedTweakText {
            title: "NTFS 暗号化ファイルシステム (EFS) ドライバを無効化",
            description: "FileSystem キー下で NtfsDisableEncryption=1 を設定し、EFS フィルタードライバがファイル IO 処理に割り込むのを防止します。ファイル単位の EFS 暗号化を使用しない環境でファイルアクセス遅延を低減します。",
            broken_features: &["NTFS 暗号化ファイルシステム (EFS) が動作しなくなります", "既存の EFS 暗号化ファイルにアクセスできなくなります"],
        }),
        "disable_page_file_encryption" => Some(LocalizedTweakText {
            title: "ページファイルの暗号化およびシャットダウン時の消去を無効化",
            description: "ページファイル内容の暗号化を防止し、シャットダウン時のページファイル消去を無効化します (ClearPageFileAtShutdown=0, DisablePagingEncryption=1)。ページングスループットを向上させ、シャットダウン所要時間を大幅に短縮します。",
            broken_features: &["シャットダウン後もページングされたメモリ内容がディスクに残ります", "物理ディスクから機密データが復元可能になるリスクがあります"],
        }),
        "disable_processor_idle_demote_threshold" => Some(LocalizedTweakText {
            title: "プロセッサのアイドル状態降格を無効化",
            description: "プロセッサ電源管理で IdleDemoteThreshold=100 および IdlePromoteThreshold=100 を設定し、瞬間的なアイドル時に CPU パフォーマンス状態が降格するのを防止します。ゲームやリアルタイム処理での周波数引き上げ遅延を排除します。",
            broken_features: &["アイドル時の消費電力が増加します", "ノート PC のバッテリー持続時間が短縮されます"],
        }),
        "disable_rss_base_cpu" => Some(LocalizedTweakText {
            title: "ネットワーク負荷分散のための RSS ベース CPU を設定",
            description: "NDIS パラメータで RssBaseCpu=1 を設定し、Receive Side Scaling によるネットワーク割り込み処理を CPU 0 ではなく CPU 1 から分散開始するように指示します。ブート CPU (CPU 0) の負荷を解放し、ゲームのマイクロスタッターを抑制します。",
            broken_features: &[],
        }),
        "disable_spectre_meltdown_mitigations" => Some(LocalizedTweakText {
            title: "Spectre / Meltdown CPU 脆弱性緩和策を無効化",
            description: "FeatureSettingsOverride=3 および FeatureSettingsOverrideMask=3 を設定し、ハードウェアレベルの Spectre および Meltdown CPU 緩和策を無効化します。投機的実行攻撃対策による性能低下を解消し、計算負荷の高いワークロードで大幅なスループット向上を実現します。",
            broken_features: &["システムが Spectre および Meltdown CPU サイドチャネル攻撃に対して脆弱になります"],
        }),
        "disable_threaded_dpc_optimization" => Some(LocalizedTweakText {
            title: "スレッド化 DPC を無効化して割り込みレイテンシを短縮",
            description: "ThreadDpcEnable=0 を設定し、遅延プロシージャコール (DPC) を専用スレッドではなく DISPATCH_LEVEL で直接実行するように強制します。リアルタイムオーディオ制作や競技ゲームで重要なサブミリ秒レベルの割り込み処理レイテンシを短縮します。",
            broken_features: &["スレッド化 DPC を想定している一部のドライバが予期せぬ動作をする可能性があります", "極端なマルチスレッド負荷時に DPC レイテンシのばらつきが増大する可能性があります"],
        }),
        "disable_windows_search_indexer" => Some(LocalizedTweakText {
            title: "Windows 検索インデクサーサービスを無効化",
            description: "WSearch サービスを無効化し、バックグラウンドでのファイルクロールとインデックス作成を停止します。SSD/NVMe 環境ではインデックスなしでも高速に検索でき、SearchIndexer.exe によるディスク IO、CPU、メモリ消費を削減します。",
            broken_features: &["Windows 検索でファイル内容の検索結果が表示されなくなります", "Outlook デスクトップのメール検索速度が低下する場合があります", "スタートメニューの検索でファイル内容による検索ができなくなります"],
        }),
        "enable_hardware_task_offload" => Some(LocalizedTweakText {
            title: "TCP/IP ハードウェアタスクオフロードを有効化",
            description: "TCP/IP パラメータで DisableTaskOffload=0 を設定し、チェックサム計算や TCP セグメンテーション処理を NIC ハードウェアにオフロードします。CPU のネットワーク処理負荷を低減します。",
            broken_features: &[],
        }),
        "enable_large_ntfs_frs" => Some(LocalizedTweakText {
            title: "大容量 NTFS ファイルレコードセグメントを有効化",
            description: "NtfsAllowExtendedCharacterIn8dot3Name=0 を設定して 8.3 形式の短い名前の拡張文字を無効化し、NTFS ドライバがより大きく効率的なファイルレコードセグメントを割り当てられるようにして MFT 断片化を低減します。",
            broken_features: &["自動生成される 8.3 形式の短いファイル名で拡張 Unicode 文字が削除されます"],
        }),
        "enable_ultimate_performance_plan" => Some(LocalizedTweakText {
            title: "究極のパフォーマンス電源プランを有効化",
            description: "隠された「究極のパフォーマンス」電源スキームを有効化します。コアパーキングの無効化、周波数スケーリング遅延の排除、USB セレクティブサスペンドの停止により、極限の低遅延を実現します。",
            broken_features: &["アイドル時の消費電力が大幅に増加します", "ノート PC のバッテリー持続時間が短縮されます", "アイドル時の CPU およびチップセット温度が上昇します"],
        }),
        "increase_gpu_tdr_timeout" => Some(LocalizedTweakText {
            title: "GPU タイムアウト検出・回復 (TDR) 遅延時間を延長",
            description: "TdrDelay をデフォルトの 2 秒から 10 秒に延長し、長時間のシェーダー計算やレイトレーシング処理中に Windows が GPU フリーズと誤判定してドライバをリセットするのを防止します。",
            broken_features: &["実際の GPU フリーズ発生時にリカバリまでの時間が長くなります", "ドライバ再起動前に画面が長時間静止したように見える場合があります"],
        }),
        "increase_irp_stack_size" => Some(LocalizedTweakText {
            title: "ネットワーク / ディスク IO 用 IRP スタックサイズを増加",
            description: "I/O 要求パケット (IRP) スタックサイズをデフォルトの 15 から 32 に拡張し、SMB サーバーや多層フィルタードライバに十分なスタックフレームを提供してリソース不足エラーを回避します。",
            broken_features: &[],
        }),
        "increase_max_connections_per_server" => Some(LocalizedTweakText {
            title: "サーバーあたりの最大 HTTP 接続数を増加",
            description: "MaxConnectionsPerServer および MaxConnectionsPer1_0Server を 16 に引き上げ、WinINet ベースのアプリが同一ホストに対してより多くの並列接続を開けるようにし、ダウンロードや Web 読み込みを高速化します。",
            broken_features: &[],
        }),
        "increase_nonpaged_pool_quota" => Some(LocalizedTweakText {
            title: "ドライバの安定性向上のため非ページプールクォータを拡張",
            description: "NonPagedPoolQuota を 0 (システム管理の無制限) に設定し、カーネルメモリマネージャーが非ページ割り当てに利用可能なすべての物理メモリを使用できるようにしてプール枯渇を防ぎます。",
            broken_features: &[],
        }),
        "optimize_additional_critical_worker_threads" => Some(LocalizedTweakText {
            title: "追加のカーネルワーカーワーカースレッドを増強",
            description: "AdditionalCriticalWorkerThreads および AdditionalDelayedWorkerThreads を 2 に設定し、重大および遅延作業項目を処理するカーネルモードワーカースレッドを追加確保してキューの停滞を防ぎます。",
            broken_features: &[],
        }),
        "optimize_default_ttl" => Some(LocalizedTweakText {
            title: "最適なデフォルト IP パケット TTL を設定",
            description: "デフォルトの IP Time-To-Live (TTL) 値を Linux や macOS の標準に合わせた 64 ホップに設定します。不要に大きなホップ数を適正化し、ネットワークルーティングを最適化します。",
            broken_features: &[],
        }),
        "optimize_disk_io_page_lock_limit" => Some(LocalizedTweakText {
            title: "大容量転送向け IO ページロック制限を拡張",
            description: "IoPageLockLimit を 983040 バイト (960 KB) に設定し、直接 IO 転送のために IO マネージャーがロックできるメモリ量を増加させ、高速 NVMe SSD での大容量ファイル転送を効率化します。",
            broken_features: &[],
        }),
        "optimize_dns_cache_parameters" => Some(LocalizedTweakText {
            title: "DNS クライアントのキャッシュサイズと TTL を最適化",
            description: "MaxCacheEntryTtlLimit=86400 および MaxNegativeCacheTtl=5 を設定して DNS キャッシュの保持期間を延長しつつ、失敗した名前解決の再試行を短縮して Web 応答性を向上させます。",
            broken_features: &[],
        }),
        "optimize_gpu_preemption_level" => Some(LocalizedTweakText {
            title: "GPU スケジューラをコンピュート専用プリエンプションに設定",
            description: "グラフィックスドライバスケジューラで GpuPreemptionLevel=0 を設定し、シェーダー実行途中の不必要なコンテキストスイッチを防止してゲーム中のマイクロスタッターを抑制します。",
            broken_features: &["GPU のタイムアウト検出とリカバリ (TDR) のトリガーが遅くなる可能性があります"],
        }),
        "optimize_interrupt_steering" => Some(LocalizedTweakText {
            title: "割り込みアフィニティおよびステアリングポリシーを最適化",
            description: "InterruptSteeringDisabled=0 および DistributeTimers=1 を設定し、ハードウェア割り込みとタイマーコールバックを CPU 0 に集中させずすべてのコアに均等分散してボトルネックを解消します。",
            broken_features: &[],
        }),
        "optimize_mmcss_proaudio_scheduling" => Some(LocalizedTweakText {
            title: "MMCSS Pro Audio — 最大スケジューリング優先度",
            description: "Pro Audio MMCSS タスクの Scheduling Category および SFIO Priority を「High」に設定し、DAW や音楽制作アプリのリアルタイムオーディオ処理に最高レベルのマルチメディア優先度を与えて音飛びを防止します。",
            broken_features: &[],
        }),
        "optimize_ntfs_memory_usage" => Some(LocalizedTweakText {
            title: "NTFS ページプールの積極的キャッシュを有効化",
            description: "NtfsMemoryUsage を 2 に設定し、NTFS ドライバがディレクトリとメタデータのキャッシュにより大きなページプール領域を割り当てるようにして、大量の小ファイル読み取りやビルド処理を高速化します。",
            broken_features: &[],
        }),
        "optimize_object_manager_quota" => Some(LocalizedTweakText {
            title: "カーネルオブジェクトマネージャーのハンドルクォータを拡張",
            description: "SessionPoolSize=48 を設定してハンドルテーブルの制限を拡張し、多数の並行プロセスや IDE、ビルドシステムを同時に実行する環境でのハンドル枯渇を防止します。",
            broken_features: &[],
        }),
        "optimize_timer_resolution_requests" => Some(LocalizedTweakText {
            title: "アプリケーションによる高解像度タイマー要求を許可",
            description: "GlobalTimerResolutionRequests=1 を設定し、アプリケーションがシステム全体で最大プラットフォームタイマー解像度 (0.5ms) を要求できるようにして、バックグラウンドオーディオやゲームのジッターを排除します。",
            broken_features: &["タイマー割り込み頻度の増加により、アイドル時の消費電力が増加します"],
        }),
        // === gaming_latency_ultra.toml (10) ===
        "disable_driver_search_order_probing" => Some(LocalizedTweakText {
            title: "Windows Update によるドライバー検索プローブを無効化",
            description: "デバイス接続時に Windows Update がバックグラウンドでドライバーを自動照会するのを停止します。デバイス接続時のインストール遅延やバックグラウンドネットワーク負荷を排除します。",
            broken_features: &["新規デバイス接続時に Windows Update による自動オンラインドライバー検索が実行されなくなります"],
        }),
        "disable_fast_user_switching" => Some(LocalizedTweakText {
            title: "ユーザーの簡易切り替えを無効化",
            description: "ユーザーの簡易切り替えを無効化し、非アクティブなセッションやバックグラウンドタスクが物理 RAM に常駐し続けるのを防止します。メモリ帯域を解放し CPU 競合を低減します。",
            broken_features: &["複数のローカルユーザーセッションに同時にサインインした状態を維持できなくなります"],
        }),
        "disable_ip_source_routing" => Some(LocalizedTweakText {
            title: "IP ソースルーティングを無効化",
            description: "DisableIPSourceRouting=2 を設定してすべての着信ソースルーティング IP パケットを破棄します。パケットスプーフィングや経路乗っ取り、ネットワークスタック処理オーバーヘッドを防止します。",
            broken_features: &["ソースルーティングを使用するネットワーク診断ツール（極めて稀）が拒否されます"],
        }),
        "disable_netsh_teredo_tunneling" => Some(LocalizedTweakText {
            title: "Teredo IPv6 移行トンネリングを無効化",
            description: "IPv6 DisabledComponents を 32 に設定して Teredo トンネリング仮想インターフェイスを無効化します。IPv4 UDP 内に IPv6 をカプセル化する不要な NAT 越え遅延を排除します。",
            broken_features: &["Teredo NAT 越えに厳密に依存する一部のレガシー Xbox Live P2P マルチプレイヤーゲームが動作しなくなります"],
        }),
        "disable_network_adapter_power_saving" => Some(LocalizedTweakText {
            title: "NDIS ネットワークアダプターの PnP 省電力を無効化",
            description: "NDIS PnPCapabilities を 24 に設定し、ネットワークカードの省電力スリープ機能を停止します。ゲームプレイの一時停止中に NIC が低電力状態に遷移するのを防ぎ、パケット遅延を排除します。",
            broken_features: &["ネットワークインターフェイスの Wake-on-LAN およびスリープ省電力機能が無効化されます"],
        }),
        "disable_remote_desktop_udp" => Some(LocalizedTweakText {
            title: "リモートデスクトップクライアントの UDP トランスポートを無効化",
            description: "リモートデスクトップ (RDP) クライアントに UDP の代わりに信頼性の高い TCP フレームの使用を強制します。UDP バッファ枯渇を防ぎ、RDP 通信がゲームの UDP パケットと競合するのを防止します。",
            broken_features: &["高遅延 WAN 接続経由のリモートデスクトップセッションで UDP の代わりに TCP が使用されます"],
        }),
        "disable_smart_card_service" => Some(LocalizedTweakText {
            title: "スマートカードサービス (SCardSvr) を無効化",
            description: "Smart Card サービスのスタートアップの種類を無効 (Start=4) に設定します。カードリーダーや暗号化トークンのバックグラウンドポーリングを停止し、ゲーミング PC のサービス負荷を軽減します。",
            broken_features: &["物理スマートカード認証およびスマートカードリーダー機器が動作しなくなります"],
        }),
        "optimize_kerberos_ticket_cache" => Some(LocalizedTweakText {
            title: "Kerberos 最大トークンサイズバッファを最適化",
            description: "Kerberos 認証の MaxTokenSize バッファを 65535 バイトに設定し、複雑なセキュリティグループ環境でのバッファオーバーフローや頻繁なチケット再ネゴシエーションを防止します。",
            broken_features: &[],
        }),
        "optimize_tcp_syn_attack_protection" => Some(LocalizedTweakText {
            title: "TCP SYN アタック保護を有効化",
            description: "TCP/IP スタックで SYN フラッド防御 (SynAttackProtect=1) を有効化します。接続バースト発生時にハンドシェイクタイムアウトを短縮し、ゲーム用ネットワークソケットの枯渇を防ぎます。",
            broken_features: &[],
        }),
        "optimize_usb_selective_suspend_hub" => Some(LocalizedTweakText {
            title: "グローバル USB ハブのセレクティブサスペンドを無効化",
            description: "ルート USB ドライバレベルでセレクティブサスペンドを無効化 (DisableSelectiveSuspend=1) します。アイドル状態のゲーミングマウスやキーボード、DAC の電源オフを防ぎ、入力復帰遅延をゼロにします。",
            broken_features: &["USB バスのアイドル時消費電力がわずかに増加します", "ノート PC のバッテリー持続時間が短縮されます"],
        }),
        "remove_webexperience_package" => Some(LocalizedTweakText {
            title: "Windows WebExperienceHost パッケージ (ウィジェット ホスト) の削除",
            description: "MicrosoftWindows.Client.WebExperience パッケージを削除し、タスクバーのウィジェットによるバックグラウンド Edge WebView2 メモリ消費を防ぎます。",
            broken_features: &["タスクバーのウィジェットとニュース フィードが完全に削除されます。"],
        }),
        "disable_wpbt_execution" => Some(LocalizedTweakText {
            title: "WPBT ファームウェア バイナリ実行の無効化",
            description: "マザーボードの UEFI Windows Platform Binary Table (WPBT) に埋め込まれたバイナリの実行をブロックします。",
            broken_features: &[],
        }),
        "service_shutdown_timeout_15s" => Some(LocalizedTweakText {
            title: "サービスのシャットダウン タイムアウトを 15 秒に短縮",
            description: "シャットダウン時のサービス終了タイムアウトを 20 秒から 15 秒に短縮し、シャットダウンの遅延を防ぎます。",
            broken_features: &[],
        }),
        "start_menu_show_more_pins" => Some(LocalizedTweakText {
            title: "スタート メニュー: ピン留めを増やすレイアウト",
            description: "Windows 11 スタート メニューでおすすめの候補よりもピン留めされたアプリを優先するように構成します。",
            broken_features: &[],
        }),
        "disable_taskbar_people_button" => Some(LocalizedTweakText {
            title: "タスクバーの「People」ボタンの無効化",
            description: "Windows タスクバーからレガシーな「People」アイコンを非表示にして無効化します。",
            broken_features: &[],
        }),
        "repair_ndu_graph" => Some(LocalizedTweakText {
            title: "タスク マネージャー ネットワーク データ使用量 (NDU) グラフの修復",
            description: "タスク マネージャーのネットワーク グラフが正常に機能するよう、Network Diagnostic Usage (NDU) カーネル ドライバーを Start=2 (自動) に設定します。",
            broken_features: &[],
        }),
        "disable_settings_ads" => Some(LocalizedTweakText {
            title: "「設定」内の推奨コンテンツと広告の無効化",
            description: "Windows の「設定」内に Microsoft が推奨アプリ、パートナーのプロモーション、ヒントを表示するのを防ぎます。",
            broken_features: &["「設定」アプリ内のプロモーション カードとヒントが非表示になります。"],
        }),
        "disable_start_menu_suggested_apps" => Some(LocalizedTweakText {
            title: "おすすめストア アプリの自動インストールの無効化",
            description: "Windows Content Delivery Manager によるおすすめアプリやゲームのサイレント ダウンロードとピン留めを防ぎます。",
            broken_features: &["スポンサー提供のゲームがスタート メニューに自動インストールされなくなります。"],
        }),
        "disable_hardware_companion_software" => Some(LocalizedTweakText {
            title: "ハードウェア コンパニオン ソフトウェアの推奨を無効化",
            description: "周辺機器を接続した際に、サードパーティ製 OEM ハードウェア コンパニオン ソフトウェアを通知・推奨するのを防ぎます。",
            broken_features: &[],
        }),
        "disable_ms_office_telemetry" => Some(LocalizedTweakText {
            title: "Microsoft Office テレメトリおよびカスタマー エクスペリエンス ログの無効化",
            description: "Microsoft Office の診断数据收集、フィードバック プロンプト、カスタマー エクスペリエンス向上プログラム (CEIP) テレメトリを無効にします。",
            broken_features: &[],
        }),
        _ => None,
    }
}

/// Localized tweak title for this language.
#[allow(dead_code)]
pub fn tweak_title(id: &str) -> Option<&'static str> {
    match id {
        "disable_dwm_animations_per_process" => Some("プロセスごとの DWM トランジションアニメーションを無効化"),
        "disable_dwm_composition_throttle" => Some("デスクトップウィンドウマネージャーのコンポジション調整を無効化"),
        "disable_kernel_mitigations_audit" => Some("カーネル緩和策の監査モードを無効化"),
        "disable_llmnr_protocol" => Some("LLMNR 名前解決プロトコルを無効化"),
        "disable_memory_compression" => Some("メモリ圧縮エンジンを無効化"),
        "disable_netbios_over_tcp" => Some("NetBIOS over TCP/IP を無効化"),
        "disable_ntfs_encryption_driver" => Some("NTFS 暗号化ファイルシステム (EFS) ドライバを無効化"),
        "disable_page_file_encryption" => Some("ページファイルの暗号化およびシャットダウン時の消去を無効化"),
        "disable_processor_idle_demote_threshold" => Some("プロセッサのアイドル状態降格を無効化"),
        "disable_rss_base_cpu" => Some("ネットワーク負荷分散のための RSS ベース CPU を設定"),
        "disable_spectre_meltdown_mitigations" => Some("Spectre / Meltdown CPU 脆弱性緩和策を無効化"),
        "disable_threaded_dpc_optimization" => Some("スレッド化 DPC を無効化して割り込みレイテンシを短縮"),
        "disable_windows_search_indexer" => Some("Windows 検索インデクサーサービスを無効化"),
        "enable_hardware_task_offload" => Some("TCP/IP ハードウェアタスクオフロードを有効化"),
        "enable_large_ntfs_frs" => Some("大容量 NTFS ファイルレコードセグメントを有効化"),
        "enable_ultimate_performance_plan" => Some("究極のパフォーマンス電源プランを有効化"),
        "increase_gpu_tdr_timeout" => Some("GPU タイムアウト検出・回復 (TDR) 遅延時間を延長"),
        "increase_irp_stack_size" => Some("ネットワーク / ディスク IO 用 IRP スタックサイズを増加"),
        "increase_max_connections_per_server" => Some("サーバーあたりの最大 HTTP 接続数を増加"),
        "increase_nonpaged_pool_quota" => Some("ドライバの安定性向上のため非ページプールクォータを拡張"),
        "optimize_additional_critical_worker_threads" => Some("追加のカーネルワーカーワーカースレッドを増強"),
        "optimize_default_ttl" => Some("最適なデフォルト IP パケット TTL を設定"),
        "optimize_disk_io_page_lock_limit" => Some("大容量転送向け IO ページロック制限を拡張"),
        "optimize_dns_cache_parameters" => Some("DNS クライアントのキャッシュサイズと TTL を最適化"),
        "optimize_gpu_preemption_level" => Some("GPU スケジューラをコンピュート専用プリエンプションに設定"),
        "optimize_interrupt_steering" => Some("割り込みアフィニティおよびステアリングポリシーを最適化"),
        "optimize_mmcss_proaudio_scheduling" => Some("MMCSS Pro Audio — 最大スケジューリング優先度"),
        "optimize_ntfs_memory_usage" => Some("NTFS ページプールの積極的キャッシュを有効化"),
        "optimize_object_manager_quota" => Some("カーネルオブジェクトマネージャーのハンドルクォータを拡張"),
        "optimize_timer_resolution_requests" => Some("アプリケーションによる高解像度タイマー要求を許可"),
                "disable_driver_search_order_probing" => Some("Windows Update によるドライバー検索プローブを無効化"),
        "disable_fast_user_switching" => Some("ユーザーの簡易切り替えを無効化"),
        "disable_ip_source_routing" => Some("IP ソースルーティングを無効化"),
        "disable_netsh_teredo_tunneling" => Some("Teredo IPv6 移行トンネリングを無効化"),
        "disable_network_adapter_power_saving" => Some("NDIS ネットワークアダプターの PnP 省電力を無効化"),
        "disable_remote_desktop_udp" => Some("リモートデスクトップクライアントの UDP トランスポートを無効化"),
        "disable_smart_card_service" => Some("スマートカードサービス (SCardSvr) を無効化"),
        "optimize_kerberos_ticket_cache" => Some("Kerberos 最大トークンサイズバッファを最適化"),
        "optimize_tcp_syn_attack_protection" => Some("TCP SYN アタック保護を有効化"),
        "optimize_usb_selective_suspend_hub" => Some("グローバル USB ハブのセレクティブサスペンドを無効化"),
        _ => lookup(id).map(|t| t.title),
    }
}

/// Localized tweak description for this language.
#[allow(dead_code)]
pub fn tweak_description(id: &str) -> Option<&'static str> {
    match id {
        "disable_dwm_animations_per_process" => Some("DWM キー下の DisallowAnimations=1 および AnimationsShiftKey=1 を設定して、プロセスごとの DWM トランジションアニメーションを無効化します。ウィンドウの最小化、最大化、切り替え時のフェードやスライド効果を排除し、GPU コンポジターの負荷を軽減してウィンドウ操作を瞬時にします。"),
        "disable_dwm_composition_throttle" => Some("DWM キー下の DesktopCompositionThrottle=0 を設定し、フレームドロップ発生時にコンポジターがレンダリングを抑制するのを防止します。GPU 負荷時でも DWM が最大のコンポジションスループットを維持できるようにします。"),
        "disable_kernel_mitigations_audit" => Some("MitigationAuditOptions をすべてゼロに設定して、カーネルエクスプロイト緩和策の監査ログを無効化します。プロセス生成や例外処理時の ETW イベント発生を抑制し、パフォーマンスを向上させます。"),
        "disable_llmnr_protocol" => Some("DNS クライアント設定で EnableMulticast=0 を指定し、リンクローカルマルチキャスト名前解決 (LLMNR) を無効化します。ローカルサブネットへのブロードキャストを停止し、セキュリティリスク (LLMNR ポイズニング) と不要なネットワークトラフィックを削減します。"),
        "disable_memory_compression" => Some("DisableCompression=1 を設定して Windows メモリマネージャーの圧縮ストアを無効化します。16GB 以上の RAM を搭載した環境では、System プロセスのメモリ圧縮スレッドによる CPU 使用率スパイクを防止できます。"),
        "disable_netbios_over_tcp" => Some("NetbiosOptions=2 を設定し LMHosts 参照を無効化することで NetBIOS over TCP/IP を無効化します。レガシーなブロードキャスト名前解決を停止し、不要なネットワーク負荷と NBNS ポイズニング攻撃のリスクを排除します。"),
        "disable_ntfs_encryption_driver" => Some("FileSystem キー下で NtfsDisableEncryption=1 を設定し、EFS フィルタードライバがファイル IO 処理に割り込むのを防止します。ファイル単位の EFS 暗号化を使用しない環境でファイルアクセス遅延を低減します。"),
        "disable_page_file_encryption" => Some("ページファイル内容の暗号化を防止し、シャットダウン時のページファイル消去を無効化します (ClearPageFileAtShutdown=0, DisablePagingEncryption=1)。ページングスループットを向上させ、シャットダウン所要時間を大幅に短縮します。"),
        "disable_processor_idle_demote_threshold" => Some("プロセッサ電源管理で IdleDemoteThreshold=100 および IdlePromoteThreshold=100 を設定し、瞬間的なアイドル時に CPU パフォーマンス状態が降格するのを防止します。ゲームやリアルタイム処理での周波数引き上げ遅延を排除します。"),
        "disable_rss_base_cpu" => Some("NDIS パラメータで RssBaseCpu=1 を設定し、Receive Side Scaling によるネットワーク割り込み処理を CPU 0 ではなく CPU 1 から分散開始するように指示します。ブート CPU (CPU 0) の負荷を解放し、ゲームのマイクロスタッターを抑制します。"),
        "disable_spectre_meltdown_mitigations" => Some("FeatureSettingsOverride=3 および FeatureSettingsOverrideMask=3 を設定し、ハードウェアレベルの Spectre および Meltdown CPU 緩和策を無効化します。投機的実行攻撃対策による性能低下を解消し、計算負荷の高いワークロードで大幅なスループット向上を実現します。"),
        "disable_threaded_dpc_optimization" => Some("ThreadDpcEnable=0 を設定し、遅延プロシージャコール (DPC) を専用スレッドではなく DISPATCH_LEVEL で直接実行するように強制します。リアルタイムオーディオ制作や競技ゲームで重要なサブミリ秒レベルの割り込み処理レイテンシを短縮します。"),
        "disable_windows_search_indexer" => Some("WSearch サービスを無効化し、バックグラウンドでのファイルクロールとインデックス作成を停止します。SSD/NVMe 環境ではインデックスなしでも高速に検索でき、SearchIndexer.exe によるディスク IO、CPU、メモリ消費を削減します。"),
        "enable_hardware_task_offload" => Some("TCP/IP パラメータで DisableTaskOffload=0 を設定し、チェックサム計算や TCP セグメンテーション処理を NIC ハードウェアにオフロードします。CPU のネットワーク処理負荷を低減します。"),
        "enable_large_ntfs_frs" => Some("NtfsAllowExtendedCharacterIn8dot3Name=0 を設定して 8.3 形式の短い名前の拡張文字を無効化し、NTFS ドライバがより大きく効率的なファイルレコードセグメントを割り当てられるようにして MFT 断片化を低減します。"),
        "enable_ultimate_performance_plan" => Some("隠された「究極のパフォーマンス」電源スキームを有効化します。コアパーキングの無効化、周波数スケーリング遅延の排除、USB セレクティブサスペンドの停止により、極限の低遅延を実現します。"),
        "increase_gpu_tdr_timeout" => Some("TdrDelay をデフォルトの 2 秒から 10 秒に延長し、長時間のシェーダー計算やレイトレーシング処理中に Windows が GPU フリーズと誤判定してドライバをリセットするのを防止します。"),
        "increase_irp_stack_size" => Some("I/O 要求パケット (IRP) スタックサイズをデフォルトの 15 から 32 に拡張し、SMB サーバーや多層フィルタードライバに十分なスタックフレームを提供してリソース不足エラーを回避します。"),
        "increase_max_connections_per_server" => Some("MaxConnectionsPerServer および MaxConnectionsPer1_0Server を 16 に引き上げ、WinINet ベースのアプリが同一ホストに対してより多くの並列接続を開けるようにし、ダウンロードや Web 読み込みを高速化します。"),
        "increase_nonpaged_pool_quota" => Some("NonPagedPoolQuota を 0 (システム管理の無制限) に設定し、カーネルメモリマネージャーが非ページ割り当てに利用可能なすべての物理メモリを使用できるようにしてプール枯渇を防ぎます。"),
        "optimize_additional_critical_worker_threads" => Some("AdditionalCriticalWorkerThreads および AdditionalDelayedWorkerThreads を 2 に設定し、重大および遅延作業項目を処理するカーネルモードワーカースレッドを追加確保してキューの停滞を防ぎます。"),
        "optimize_default_ttl" => Some("デフォルトの IP Time-To-Live (TTL) 値を Linux や macOS の標準に合わせた 64 ホップに設定します。不要に大きなホップ数を適正化し、ネットワークルーティングを最適化します。"),
        "optimize_disk_io_page_lock_limit" => Some("IoPageLockLimit を 983040 バイト (960 KB) に設定し、直接 IO 転送のために IO マネージャーがロックできるメモリ量を増加させ、高速 NVMe SSD での大容量ファイル転送を効率化します。"),
        "optimize_dns_cache_parameters" => Some("MaxCacheEntryTtlLimit=86400 および MaxNegativeCacheTtl=5 を設定して DNS キャッシュの保持期間を延長しつつ、失敗した名前解決の再試行を短縮して Web 応答性を向上させます。"),
        "optimize_gpu_preemption_level" => Some("グラフィックスドライバスケジューラで GpuPreemptionLevel=0 を設定し、シェーダー実行途中の不必要なコンテキストスイッチを防止してゲーム中のマイクロスタッターを抑制します。"),
        "optimize_interrupt_steering" => Some("InterruptSteeringDisabled=0 および DistributeTimers=1 を設定し、ハードウェア割り込みとタイマーコールバックを CPU 0 に集中させずすべてのコアに均等分散してボトルネックを解消します。"),
        "optimize_mmcss_proaudio_scheduling" => Some("Pro Audio MMCSS タスクの Scheduling Category および SFIO Priority を「High」に設定し、DAW や音楽制作アプリのリアルタイムオーディオ処理に最高レベルのマルチメディア優先度を与えて音飛びを防止します。"),
        "optimize_ntfs_memory_usage" => Some("NtfsMemoryUsage を 2 に設定し、NTFS ドライバがディレクトリとメタデータのキャッシュにより大きなページプール領域を割り当てるようにして、大量の小ファイル読み取りやビルド処理を高速化します。"),
        "optimize_object_manager_quota" => Some("SessionPoolSize=48 を設定してハンドルテーブルの制限を拡張し、多数の並行プロセスや IDE、ビルドシステムを同時に実行する環境でのハンドル枯渇を防止します。"),
        "optimize_timer_resolution_requests" => Some("GlobalTimerResolutionRequests=1 を設定し、アプリケーションがシステム全体で最大プラットフォームタイマー解像度 (0.5ms) を要求できるようにして、バックグラウンドオーディオやゲームのジッターを排除します。"),
                "disable_driver_search_order_probing" => Some("デバイス接続時に Windows Update がバックグラウンドでドライバーを自動照会するのを停止します。デバイス接続時のインストール遅延やバックグラウンドネットワーク負荷を排除します。"),
        "disable_fast_user_switching" => Some("ユーザーの簡易切り替えを無効化し、非アクティブなセッションやバックグラウンドタスクが物理 RAM に常駐し続けるのを防止します。メモリ帯域を解放し CPU 競合を低減します。"),
        "disable_ip_source_routing" => Some("DisableIPSourceRouting=2 を設定してすべての着信ソースルーティング IP パケットを破棄します。パケットスプーフィングや経路乗っ取り、ネットワークスタック処理オーバーヘッドを防止します。"),
        "disable_netsh_teredo_tunneling" => Some("IPv6 DisabledComponents を 32 に設定して Teredo トンネリング仮想インターフェイスを無効化します。IPv4 UDP 内に IPv6 をカプセル化する不要な NAT 越え遅延を排除します。"),
        "disable_network_adapter_power_saving" => Some("NDIS PnPCapabilities を 24 に設定し、ネットワークカードの省電力スリープ機能を停止します。ゲームプレイの一時停止中に NIC が低電力状態に遷移するのを防ぎ、パケット遅延を排除します。"),
        "disable_remote_desktop_udp" => Some("リモートデスクトップ (RDP) クライアントに UDP の代わりに信頼性の高い TCP フレームの使用を強制します。UDP バッファ枯渇を防ぎ、RDP 通信がゲームの UDP パケットと競合するのを防止します。"),
        "disable_smart_card_service" => Some("Smart Card サービスのスタートアップの種類を無効 (Start=4) に設定します。カードリーダーや暗号化トークンのバックグラウンドポーリングを停止し、ゲーミング PC のサービス負荷を軽減します。"),
        "optimize_kerberos_ticket_cache" => Some("Kerberos 認証の MaxTokenSize バッファを 65535 バイトに設定し、複雑なセキュリティグループ環境でのバッファオーバーフローや頻繁なチケット再ネゴシエーションを防止します。"),
        "optimize_tcp_syn_attack_protection" => Some("TCP/IP スタックで SYN フラッド防御 (SynAttackProtect=1) を有効化します。接続バースト発生時にハンドシェイクタイムアウトを短縮し、ゲーム用ネットワークソケットの枯渇を防ぎます。"),
        "optimize_usb_selective_suspend_hub" => Some("ルート USB ドライバレベルでセレクティブサスペンドを無効化 (DisableSelectiveSuspend=1) します。アイドル状態のゲーミングマウスやキーボード、DAC の電源オフを防ぎ、入力復帰遅延をゼロにします。"),
        _ => lookup(id).map(|t| t.description),
    }
}
