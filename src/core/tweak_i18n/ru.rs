//! Russian (Русский) localization table for all 168 WinBlast0 catalog tweaks.

use super::LocalizedTweakText;

pub fn lookup(id: &str) -> Option<LocalizedTweakText> {
    match id {
        "disable_windows_ai_recall_extended" => Some(LocalizedTweakText {
            title: "Отключить Windows Recall, Click to Do и ИИ-агентов Copilot+",
            description: "Блокирует захват снимков Windows Recall, Click to Do, ИИ-агента параметров, коннекторов/рабочих пространств ИИ-агентов и Copilot Runtime через ключи групповой политики, появившиеся в Windows 11 24H2+.",
            broken_features: &[
                "Снимки ленты времени Windows Recall и ИИ-наложение Click to Do",
            ],
        }),
        "disable_generative_ai_app_access" => Some(LocalizedTweakText {
            title: "Заблокировать доступ приложений к генеративным и системным ИИ-моделям",
            description: "Запрещает фоновый доступ приложений UWP/WinUI к локальному генеративному ИИ и системным ИИ-моделям через политики AppPrivacy и CapabilityAccessManager ConsentStore.",
            broken_features: &[
                "Сторонние приложения не смогут обращаться к встроенным конечным точкам ИИ-моделей Windows",
            ],
        }),
        "disable_copilot_shell_and_hardware_key" => Some(LocalizedTweakText {
            title: "Отключить кнопку Copilot на панели задач, подсказки, клавишу и микрофон",
            description: "Скрывает кнопку и подсказки Windows Copilot на панели задач, отключает назначение выделенной клавиши Copilot на клавиатуре, блокирует рекламные предложения ИИ для учётной записи и запрещает доступ к микрофону для Copilot и OfficeHub.",
            broken_features: &[
                "Выделенная клавиша Copilot не запускает Copilot",
                "Голосовой ввод в Copilot заблокирован",
            ],
        }),
        "disable_paint_ai_features" => Some(LocalizedTweakText {
            title: "Отключить облачные и генеративные ИИ-функции Microsoft Paint",
            description: "Отключает Image Creator, Cocreator, генеративную заливку, генеративное стирание и ИИ-удаление фона в Microsoft Paint для Windows 11 через официальные ключи политик.",
            broken_features: &[
                "Инструменты ИИ-генерации скрыты в Microsoft Paint",
            ],
        }),
        "disable_edge_ai_policies" => Some(LocalizedTweakText {
            title: "Отключить Copilot, боковую панель и встроенные ИИ-API в Microsoft Edge",
            description: "Отключает 12 политик ИИ Microsoft Edge, включая Copilot Page Context, боковую панель Hubs, Entra Copilot, встроенный редактор (Inline Compose), ИИ-поиск по истории, локальные базовые модели, DevTools GenAI и передачу истории.",
            broken_features: &[
                "Боковая панель Copilot и встроенный помощник ввода текста в Edge отключены",
            ],
        }),
        "disable_office_ai_copilot" => Some(LocalizedTweakText {
            title: "Отключить обучение ИИ и встроенных помощников Copilot в Microsoft Office",
            description: "Отключает обучение ИИ в Microsoft 365, анализ содержимого подключённых служб, службу безопасности контента и переключатели Copilot в Word, Excel и OneNote.",
            broken_features: &[
                "Помощник Copilot в Word, Excel и OneNote отключён",
            ],
        }),
        "disable_wmi_telemetry_autologgers" => Some(LocalizedTweakText {
            title: "Отключить 10 сеансов автологирования телеметрии ядра WMI",
            description: "Запрещает Windows запускать постоянные фоновые сеансы трассировки ETW AutoLogger (Diagtrack-Listener, SQMLogger, DiagLog, Cellcore, DataMarket, AppModel, CloudExperienceHostOobe, LwtNetLog, WdiContextLog, WiFiSession).",
            broken_features: &[
                "Диагностические журналы трассировки ETW ядра отключены",
            ],
        }),
        "block_silent_preinstalled_apps" => Some(LocalizedTweakText {
            title: "Заблокировать тихую предустановку приложений и советы в Пуске и Параметрах",
            description: "Запрещает ContentDeliveryManager незаметно устанавливать рекламные приложения OEM или Microsoft Store и скрывает рекомендуемый контент в Параметрах, меню «Пуск» и ленте времени.",
            broken_features: &[],
        }),
        "disable_extra_telemetry_policies" => Some(LocalizedTweakText {
            title: "Ограничить отзывы, инвентаризацию AppCompat, подсказки OOBE и отслеживание",
            description: "Устанавливает MaxTelemetryAllowed=0, отключает запросы отзывов Windows SIUF (удаляет PeriodInNanoSeconds), блокирует инвентаризацию совместимости приложений (UAR), «Поиск устройства», напоминания о завершении настройки, отслеживание запуска приложений, предложения Workplace Join и Удалённого помощника.",
            broken_features: &[
                "Приглашения Удалённого помощника отключены",
                "Отслеживание часто и недавно запускаемых приложений в меню «Пуск» выключено",
            ],
        }),
        "disable_typing_and_inking_telemetry" => Some(LocalizedTweakText {
            title: "Отключить сбор данных рукописного ввода, персонализации печати и Input Insights",
            description: "Ограничивает скрытый сбор данных нажатий клавиш и рукописного ввода, сбор контактов для пользовательских словарей, лингвистическое диагностическое журналирование (TIPC) и Windows Input Insights.",
            broken_features: &[
                "Персонализированный словарь подсказок ввода текста и рукописного ввода отключён",
            ],
        }),
        "disable_search_cloud_and_history" => Some(LocalizedTweakText {
            title: "Отключить «Главное в поиске», облачный поиск и историю поиска устройства",
            description: "Выключает «Главное в поиске» (динамическое поле поиска), облачный поиск учётных записей MSA/AAD и локальное ведение истории поиска без отключения службы индексирования.",
            broken_features: &[
                "Ежедневные иллюстрации в поиске и облачные результаты OneDrive скрыты",
            ],
        }),
        "disable_wifi_sense_and_location_extras" => Some(LocalizedTweakText {
            title: "Отключить автоподключение WiFi-Sense, автообновление Карт и доступ к геопозиции",
            description: "Отключает автоматическое подключение к точкам доступа WiFi (WcmSvc\\AutoConnectAllowedOEM=0), фоновое обновление автономных карт Windows и глобальное разрешение ConsentStore для службы определения местоположения.",
            broken_features: &[
                "Автоматическая загрузка карт Windows и системные запросы геопозиции для приложений отключены",
            ],
        }),
        "block_onedrive_kfm_auto_backup" => Some(LocalizedTweakText {
            title: "Заблокировать автоматический перенос известных папок в OneDrive (Рабочий стол/Документы)",
            description: "Устанавливает KFMBlockOptIn=1, чтобы OneDrive не мог незаметно перенести папки «Рабочий стол», «Документы» и «Изображения» в облачное хранилище. Безопасно, если вы используете только локальное хранение или ручную синхронизацию.",
            broken_features: &[
                "Мастер автоматического переноса известных папок OneDrive отключён",
            ],
        }),
        "disable_bitlocker_auto_encryption" => Some(LocalizedTweakText {
            title: "Отключить автоматическое шифрование устройства BitLocker в Windows",
            description: "Устанавливает PreventDeviceEncryption=1, чтобы Windows не шифровала диски автоматически при входе с учётной записью Microsoft или новой настройке без предварительного сохранения ключей восстановления.",
            broken_features: &[
                "Автоматическое шифрование дисков BitLocker без участия пользователя не активируется",
            ],
        }),
        "disable_extra_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Отключить дополнительные задачи телеметрии, WER, SQM, диагностики питания и Карт",
            description: "Отключает дополнительные диагностические и фоновые запланированные задачи: QueueReporting, Sqm-Tasks, MareBackup, StartupAppTask, PcaPatchDbTask, MapsUpdateTask, AnalyzeSystem и FamilySafetyMonitor.",
            broken_features: &[
                "Автоматические отчёты об энергоэффективности и обновления автономных карт отключены",
            ],
        }),
        "disable_windows_ai_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Отключить фоновые задачи планировщика Windows AI Recall",
            description: "Отключает запланированные задачи конфигурации Windows Recall и фонового конвейера машинного обучения, появившиеся в Windows 11 24H2+.",
            broken_features: &[
                "Фоновые задачи индексирования Windows Recall отключены",
            ],
        }),
        "disable_office_actions_scheduled_task" => Some(LocalizedTweakText {
            title: "Отключить задачу планировщика Microsoft Office AI Actions Server",
            description: "Отключает фоновую задачу планировщика Microsoft Office Actions Server, снижая фоновые пробуждения ЦП и потребление памяти со стороны ИИ Office.",
            broken_features: &[],
        }),
        "disable_optional_bloat_services" => Some(LocalizedTweakText {
            title: "Отключить неиспользуемые службы (Карты, Факс, Демо, Кошелёк, tzautoupdate)",
            description: "Отключает службы MapsBroker, RetailDemo, Fax, WalletService, WMPNetworkSvc, AJRouter, shpamsvc и tzautoupdate, не затрагивая базовые службы ОС и Центра обновления Windows.",
            broken_features: &[
                "Службы Windows Факс, диспетчер автономных карт, режим витрины, общий доступ WMP и авточасовой пояс отключены",
            ],
        }),
        "disable_xbox_live_services" => Some(LocalizedTweakText {
            title: "Отключить фоновые службы Xbox Live (XblAuthManager, GameSave, Networking)",
            description: "Отключает фоновые службы диспетчера проверки подлинности Xbox Live, сохранения игр и сетевых служб Xbox для пользователей, которые не играют в игры из приложения Xbox / Game Pass.",
            broken_features: &[
                "Вход в Xbox Live и облачные сохранения в Xbox Game Pass и Microsoft Store перестанут работать",
            ],
        }),
        "disable_ai_fabric_service" => Some(LocalizedTweakText {
            title: "Отключить службу Windows AI Fabric Service (WSAIFabricSvc)",
            description: "Отключает фоновый демон локальной оркестрации ИИ WSAIFabricSvc в Windows 11 24H2+.",
            broken_features: &[
                "Служба оркестрации ИИ Copilot+ остановлена",
            ],
        }),
        "deny_global_camera_mic_diagnostics" => Some(LocalizedTweakText {
            title: "Заблокировать глобальный доступ к камере, микрофону, данным учётной записи и диагностике [Опционально]",
            description: "Глобально запрещает разрешения ConsentStore для веб-камеры, микрофона, userAccountInformation и appDiagnostics для всех классических и UWP-приложений. Высокий риск нарушений: отключает камеру и микрофон в Zoom/Discord/OBS до отмены твика.",
            broken_features: &[
                "Отключает камеру и микрофон во ВСЕХ приложениях до повторного разрешения",
            ],
        }),
        "remove_bing_weather" => Some(LocalizedTweakText {
            title: "Удалить «Погода» (Bing Weather)",
            description: "Удаляет UWP-приложение Microsoft Bing Weather. Прогноз погоды можно открывать в браузере.",
            broken_features: &[
                "Приложение «Погода» (Bing Weather) будет удалено",
            ],
        }),
        "remove_bing_news" => Some(LocalizedTweakText {
            title: "Удалить «Новости» (Bing News)",
            description: "Удаляет UWP-приложение Microsoft Bing News (Microsoft News).",
            broken_features: &[
                "Приложение «Новости» (Bing News / Microsoft News) будет удалено",
            ],
        }),
        "remove_get_help" => Some(LocalizedTweakText {
            title: "Удалить «Техническая поддержка» (Get Help)",
            description: "Удаляет UWP-приложение Microsoft Get Help. Обратиться в поддержку по-прежнему можно через веб-сайт.",
            broken_features: &[
                "Приложение «Техническая поддержка» (Get Help) будет удалено",
            ],
        }),
        "remove_getstarted_tips" => Some(LocalizedTweakText {
            title: "Удалить «Советы» (Get Started)",
            description: "Удаляет UWP-приложение Microsoft Tips / Get Started.",
            broken_features: &[
                "Приложение «Советы» (Tips / Get Started) будет удалено",
            ],
        }),
        "remove_solitaire" => Some(LocalizedTweakText {
            title: "Удалить Microsoft Solitaire Collection",
            description: "Удаляет UWP-приложение Microsoft Solitaire Collection, содержащее рекламу и телеметрию.",
            broken_features: &[
                "Приложение Solitaire Collection будет удалено",
            ],
        }),
        "remove_people" => Some(LocalizedTweakText {
            title: "Удалить приложение «Люди» (People)",
            description: "Удаляет UWP-приложение Microsoft People («Люди»).",
            broken_features: &[
                "Приложение «Люди» (People) будет удалено",
            ],
        }),
        "remove_windows_maps" => Some(LocalizedTweakText {
            title: "Удалить «Карты Windows» (Windows Maps)",
            description: "Удаляет UWP-приложение Microsoft Windows Maps.",
            broken_features: &[
                "Приложение «Карты Windows» будет удалено",
            ],
        }),
        "remove_groove_music" => Some(LocalizedTweakText {
            title: "Удалить Groove Music (Zune Music)",
            description: "Удаляет UWP-приложение Groove Music (ZuneMusic). Вместо него можно использовать Медиаплеер или сторонние плееры.",
            broken_features: &[
                "Приложение Groove Music будет удалено",
            ],
        }),
        "remove_zune_video" => Some(LocalizedTweakText {
            title: "Удалить «Кино и ТВ» (Zune Video)",
            description: "Удаляет UWP-приложение «Кино и ТВ» (ZuneVideo). Вместо него можно использовать VLC или другие видеоплееры.",
            broken_features: &[
                "Приложение «Кино и ТВ» будет удалено",
            ],
        }),
        "remove_clipchamp" => Some(LocalizedTweakText {
            title: "Удалить Clipchamp",
            description: "Удаляет предустановленный в Windows 11 UWP-видеоредактор Clipchamp.",
            broken_features: &[
                "Видеоредактор Clipchamp будет удалён",
            ],
        }),
        "remove_todos" => Some(LocalizedTweakText {
            title: "Удалить Microsoft To Do",
            description: "Удаляет UWP-приложение Microsoft To Do.",
            broken_features: &[
                "Приложение Microsoft To Do будет удалено",
            ],
        }),
        "remove_bing_search_appx" => Some(LocalizedTweakText {
            title: "Удалить расширение Microsoft Bing Search AppX",
            description: "Удаляет пакет Microsoft.BingSearch из Store, обеспечивающий веб-виджеты и веб-интеграцию поиска Windows.",
            broken_features: &[
                "Интеграция Bing Search AppX удалена",
            ],
        }),
        "remove_feedback_hub" => Some(LocalizedTweakText {
            title: "Удалить «Центр отзывов» (Feedback Hub)",
            description: "Удаляет предустановленное приложение телеметрии и опросов Windows Feedback Hub.",
            broken_features: &[
                "Приложение «Центр отзывов» удалено",
            ],
        }),
        "remove_copilot_appx" => Some(LocalizedTweakText {
            title: "Удалить UWP-пакет Microsoft Copilot",
            description: "Удаляет отдельный пакет Microsoft.Copilot из Microsoft Store.",
            broken_features: &[
                "Приложение Copilot удалено",
            ],
        }),
        "remove_edge_game_assist" => Some(LocalizedTweakText {
            title: "Удалить игровой оверлей Microsoft Edge Game Assist",
            description: "Удаляет AppX-пакет внутриигрового браузерного оверлея Edge Game Assist.",
            broken_features: &[
                "Виджет Edge Game Assist удалён",
            ],
        }),
        "remove_dev_home" => Some(LocalizedTweakText {
            title: "Удалить приложение Windows Dev Home",
            description: "Удаляет предустановленный в Windows 11 пакет панели разработчика Microsoft.Windows.DevHome.",
            broken_features: &[
                "Панель Dev Home удалена",
            ],
        }),
        "remove_power_automate_desktop" => Some(LocalizedTweakText {
            title: "Удалить Microsoft Power Automate Desktop",
            description: "Удаляет RPA-пакет автоматизации рабочих процессов Microsoft Power Automate Desktop.",
            broken_features: &[
                "Приложение потоков автоматизации Power Automate Desktop удалено",
            ],
        }),
        "remove_office_hub" => Some(LocalizedTweakText {
            title: "Удалить Microsoft 365 Copilot (OfficeHub)",
            description: "Удаляет предустановленное рекламное приложение запуска Microsoft 365 Office Hub.",
            broken_features: &[
                "Рекламный лаунчер Office удалён (настольный пакет Office не затрагивается)",
            ],
        }),
        "remove_family_safety" => Some(LocalizedTweakText {
            title: "Удалить приложение Microsoft Family Safety",
            description: "Удаляет пользовательское UWP-приложение родительского контроля Microsoft Family Safety.",
            broken_features: &[
                "UWP-приложение Family Safety удалено",
            ],
        }),
        "remove_3d_viewer_and_mixed_reality" => Some(LocalizedTweakText {
            title: "Удалить «Средство 3D-просмотра» и «Портал смешанной реальности»",
            description: "Удаляет устаревшие пакеты Microsoft 3D Viewer и Mixed Reality Portal.",
            broken_features: &[
                "Устаревшие 3D Viewer и Windows Mixed Reality Portal удалены",
            ],
        }),
        "remove_outlook_for_windows" => Some(LocalizedTweakText {
            title: "Удалить новый Outlook для Windows (WebView-клиент)",
            description: "Удаляет предустановленную веб-обёртку Microsoft.OutlookForWindows.",
            broken_features: &[
                "Новый UWP-клиент Outlook удалён",
            ],
        }),
        "remove_your_phone_link" => Some(LocalizedTweakText {
            title: "Удалить «Связь с телефоном» (YourPhone)",
            description: "Удаляет приложение-компаньон для синхронизации со смартфоном Microsoft.YourPhone.",
            broken_features: &[
                "Синхронизация ПК со смартфоном Android/iOS через «Связь с телефоном» удалена",
            ],
        }),
        "remove_sound_recorder_and_sticky_notes" => Some(LocalizedTweakText {
            title: "Удалить «Запись голоса» и «Записки» (Sticky Notes)",
            description: "Удаляет пакеты Store для «Записи голоса» и «Записок».",
            broken_features: &[
                "Встроенные приложения «Запись голоса» и «Записки» удалены",
            ],
        }),
        "remove_ms_teams_and_quick_assist" => Some(LocalizedTweakText {
            title: "Удалить пользовательский Microsoft Teams и «Быструю помощь»",
            description: "Удаляет личный чат MSTeams и инструмент удалённой поддержки Quick Assist.",
            broken_features: &[
                "Личный клиент Teams и инструмент поддержки «Быстрая помощь» удалены",
            ],
        }),
        "disable_recall_optional_feature" => Some(LocalizedTweakText {
            title: "Удалить дополнительный компонент Windows Recall (DISM)",
            description: "Отключает дополнительный пакет компонента Windows «Recall» на уровне обслуживания ОС.",
            broken_features: &[
                "Исполняемые компоненты Windows Recall корректно отключены",
            ],
        }),
        "remove_legacy_steps_recorder_ie_wordpad" => Some(LocalizedTweakText {
            title: "Отключить устаревшее «Средство записи действий» и компонент IE11",
            description: "Удаляет устаревшее «Средство записи действий» (App.StepsRecorder) и компонент режима Internet Explorer через DISM.",
            broken_features: &[
                "«Средство записи действий» (psr.exe) и режим IE11 удалены",
            ],
        }),
        "enable_hags" => Some(LocalizedTweakText {
            title: "Включить аппаратное планирование GPU (HAGS)",
            description: "Включает Hardware Accelerated GPU Scheduling, позволяя видеокарте напрямую управлять очередью видеопамяти VRAM. Снижает задержки и нагрузку на ЦП в GPU-зависимых задачах. Требует совместимый GPU (NVIDIA GTX 1000+, AMD RX 5000+) и поддержку драйвера.",
            broken_features: &[
                "Некоторые старые игры или утилиты для GPU могут работать нестабильно",
            ],
        }),
        "disable_mpo" => Some(LocalizedTweakText {
            title: "Отключить Multi-Plane Overlay (MPO)",
            description: "Отключает аппаратный композитор Multi-Plane Overlay. MPO часто вызывает статтеры, чёрные экраны и проблемы со временем кадра на многих связках GPU и драйверов, особенно при использовании мониторов с разной частотой обновления.",
            broken_features: &[
                "Незначительно более высокое энергопотребление композитора GPU на ноутбуках",
            ],
        }),
        "disable_core_parking" => Some(LocalizedTweakText {
            title: "Отключить парковку ядер процессора (Core Parking)",
            description: "Устанавливает минимальный процент активных ядер процессора на 100%, чтобы все ядра всегда оставались в готовности. Устраняет задержку пробуждения ядер ценой чуть более высокого энергопотребления в простое.",
            broken_features: &[
                "Повышенное энергопотребление в простое",
                "Сокращение времени работы от батареи на ноутбуках",
            ],
        }),
        "disable_cstates" => Some(LocalizedTweakText {
            title: "Отключить состояния глубокого простоя ЦП (C-States)",
            description: "Запрещает процессору переходить в глубокие энергосберегающие состояния C-states (C6/C7/C8), переводя политику простоя в режим производительности. Убирает всплески задержки при переключении состояний питания ценой повышенного энергопотребления в простое.",
            broken_features: &[
                "Заметно более высокое энергопотребление в простое",
                "Сокращение времени автономной работы на ноутбуках",
                "Повышенные температуры процессора в простое",
            ],
        }),
        "gpu_shader_cache_size" => Some(LocalizedTweakText {
            title: "Увеличить таймаут TDR GPU и предотвратить сбросы драйвера",
            description: "Увеличивает интервал WDDM Timeout Detection and Recovery (TdrDelay) с 2 до 10 секунд, предотвращая ложные сбросы видеодрайвера и подёргивания при тяжёлой компиляции шейдеров DirectX.",
            broken_features: &[
                "Восстановление видеодрайвера при реальном зависании GPU займёт до 10 секунд",
            ],
        }),
        "enable_game_mode" => Some(LocalizedTweakText {
            title: "Включить игровой режим Windows (Game Mode)",
            description: "Гарантирует включение Windows Game Mode (AutoGameModeEnabled=1, AllowAutoGameMode=1), повышая приоритет игровых процессов и подавляя уведомления Центра обновления Windows и перезагрузки во время игры.",
            broken_features: &[],
        }),
        "disable_fullscreen_optimizations" => Some(LocalizedTweakText {
            title: "Глобально отключить полноэкранную оптимизацию",
            description: "Глобально отключает полноэкранную оптимизацию Windows (гибридный полноэкранный режим без рамок) для всех приложений. Это принудительно включает истинный эксклюзивный полноэкранный режим, снижая задержку ввода и разброс времени кадра в старых играх.",
            broken_features: &[
                "Переключение по Alt-Tab из полноэкранных игр станет медленнее",
                "Оверлей Xbox Game Bar не будет отображаться в эксклюзивном полноэкранном режиме",
            ],
        }),
        "4433ce93-b632-4d67-ae71-0241c31099c0" => Some(LocalizedTweakText {
            title: "Отключить AMD ULPS",
            description: "Отключает режим сверхнизкого энергопотребления (ULPS) в драйвере AMD Radeon. Предотвращает пробуждение вторичных ядер GPU или выход из сна с микрофризами и скачками задержки кадра в 3D-играх.",
            broken_features: &[
                "Незначительно более высокое энергопотребление в простое на ноутбуках с несколькими GPU AMD / CrossFire",
            ],
        }),
        "8a4b4c4d-000e-42db-8bd7-293067f8fae7" => Some(LocalizedTweakText {
            title: "Отключить AMD ASPM",
            description: "Отключает управление питанием активного состояния PCIe (ASPM) для видеокарт AMD. Не даёт шине PCIe переходить в энергосберегающие состояния L0s/L1 в паузах между кадрами, снижая джиттер передачи по шине PCIe.",
            broken_features: &[
                "Повышенное энергопотребление шины PCIe в простое",
            ],
        }),
        "a3836587-c461-4beb-9c4c-88493b36c138" => Some(LocalizedTweakText {
            title: "Отключить AMD Power Gating",
            description: "Отключает агрессивное динамическое отключение питания вычислительных блоков и бэкендов рендеринга AMD (DisablePowerGating=1, PP_GPUPowerDownEnabled=0). Поддерживает стабильные частоты без просадок.",
            broken_features: &[
                "Более высокая температура кристалла GPU при низкой нагрузке",
            ],
        }),
        "db379bb5-fc8c-4ad3-be32-11be09e76fba" => Some(LocalizedTweakText {
            title: "Отключить AMD Video Clock Gating",
            description: "Отключает тактовое и силовое отключение мультимедийных блоков AMD VCE/UVD и аппаратных движков вывода изображения (DisableVCEPowerGating=1, DisableVceClockGating=1, EnableUvdClockGating=0, EnableVceSwClockGating=0). Устраняет всплески задержки DPC.",
            broken_features: &[],
        }),
        "df8739ff-e322-4958-b369-df584bb9b0c1" => Some(LocalizedTweakText {
            title: "Отключить Intel Async Flips",
            description: "Устанавливает Display1_DisableAsyncFlips=1 в ключах класса видеоадаптеров Intel Graphics для принудительной синхронизации переключения буферов дисплея и устранения джиттера очереди вывода.",
            broken_features: &[],
        }),
        "ebfb0ded-5d1a-4cd1-b5c0-4f7018dd6054" => Some(LocalizedTweakText {
            title: "Отключить адаптивную вертикальную синхронизацию Intel",
            description: "Устанавливает AdaptiveVsyncEnable=0 в ключах класса видеоадаптеров Intel Graphics, предотвращая адаптивное ограничение частоты обновления при падении FPS ниже частоты монитора.",
            broken_features: &[],
        }),
        "fa58802f-b98c-4e98-aa67-067c5a394296" => Some(LocalizedTweakText {
            title: "Отключить динамические P-состояния NVIDIA",
            description: "Устанавливает DisableDynamicPstate=1 для фиксации видеокарты NVIDIA в стабильном производительном состоянии, предотвращая колебания тактовой частоты. Увеличивает энергопотребление в простое.",
            broken_features: &[
                "Увеличивает энергопотребление и нагрев GPU в простое",
            ],
        }),
        "9e7702be-9e03-4c96-a316-c4e435cb993e" => Some(LocalizedTweakText {
            title: "Отключить асинхронные P-состояния NVIDIA",
            description: "Устанавливает DisableAsyncPstates=1 для запрета асинхронного переключения состояний питания NVIDIA, обеспечивая предсказуемую производительность в VR и приложениях реального времени.",
            broken_features: &[
                "Может препятствовать снижению частоты памяти GPU при воспроизведении видео",
            ],
        }),
        "5f1a2b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c" => Some(LocalizedTweakText {
            title: "Включить режим максимальной производительности NVIDIA PowerMizer",
            description: "Принудительно включает режим максимальной производительности PowerMizer при питании от сети, фиксируя шину PCIe и контроллеры памяти на полной скорости без троттлинга.",
            broken_features: &[
                "Повышенные температуры GPU и скорость вентиляторов в простое",
            ],
        }),
        "disable_animations" => Some(LocalizedTweakText {
            title: "Отключить анимацию окон",
            description: "Отключает визуальный эффект анимации окон при сворачивании и разворачивании, снижая ощущаемые задержки интерфейса.",
            broken_features: &[],
        }),
        "disable_transparency" => Some(LocalizedTweakText {
            title: "Отключить эффекты прозрачности",
            description: "Отключает эффект акрила и прозрачности на панели задач, в меню «Пуск» и Центре уведомлений, снижая нагрузку на композитор GPU.",
            broken_features: &[
                "Прозрачный внешний вид панели задач и меню «Пуск»",
            ],
        }),
        "disable_ntfs_last_access" => Some(LocalizedTweakText {
            title: "Глобально отключить фоновые приложения Windows",
            description: "Запрещает приложениям UWP / Microsoft Store просыпаться, опрашивать сеть и незаметно расходовать ресурсы ЦП и ОЗУ в фоновом режиме в свёрнутом или закрытом состоянии.",
            broken_features: &[
                "Приложения Store перестанут синхронизироваться в фоне после закрытия",
            ],
        }),
        "disable_nagle_algorithm" => Some(LocalizedTweakText {
            title: "Отключить алгоритм Нейгла для MSMQ (TCPNoDelay)",
            description: "Отключает алгоритм Нейгла специально для очереди сообщений Microsoft (MSMQ\\Parameters\\TCPNoDelay), снижая задержки буферизации при передаче небольших сообщений MSMQ.",
            broken_features: &[
                "Может незначительно увеличить сетевой трафик при передаче мелких пакетов",
            ],
        }),
        "disable_sticky_keys" => Some(LocalizedTweakText {
            title: "Отключить залипание клавиш и горячие клавиши спецвозможностей",
            description: "Отключает окно залипания клавиш при 5 нажатиях Shift, фильтрацию ввода и звуковые сигналы озвучивания переключения клавиш, предотвращая внезапные всплывающие окна и перехват ввода в играх.",
            broken_features: &[
                "Горячие клавиши специальных возможностей (5× Shift, удержание правого Shift 8 с) отключены",
            ],
        }),
        "disable_mouse_shadow" => Some(LocalizedTweakText {
            title: "Отключить тени подписей значков и полупрозрачное выделение в Проводнике",
            description: "Отключает отбрасывание теней подписями значков рабочего стола и альфа-смешивание полупрозрачной рамки выделения в Проводнике, снижая нагрузку на композитор DWM GPU.",
            broken_features: &[],
        }),
        "high_performance_power_plan" => Some(LocalizedTweakText {
            title: "Включить агрессивный режим усиления частоты процессора",
            description: "Устанавливает режим усиления производительности процессора при питании от сети (PerfBoostMode) в значение «Агрессивный» через PowerSchemes, предотвращая снижение турбо-частот при интерактивных нагрузках.",
            broken_features: &[
                "Повышенное энергопотребление в простое",
                "Повышенное тепловыделение на ноутбуках",
            ],
        }),
        "disable_bing_search_start" => Some(LocalizedTweakText {
            title: "Отключить веб-поиск Bing в меню «Пуск»",
            description: "Предотвращает отправку каждого нажатия клавиши из Поиска Windows на серверы Bing, делая поиск программ и локальных файлов в меню «Пуск» мгновенным и без сетевых задержек.",
            broken_features: &[
                "Результаты веб-поиска не будут отображаться в меню «Пуск»",
            ],
        }),
        "disable_windows_copilot" => Some(LocalizedTweakText {
            title: "Отключить интеграцию ИИ Windows Copilot",
            description: "Отключает Windows Copilot в системе через групповую политику, останавливая фоновых резидентных агентов Copilot, скрывая кнопки на панели задач и блокируя ИИ-запросы в сеть.",
            broken_features: &[
                "Боковая панель Windows Copilot и горячая клавиша (Win+C) будут отключены",
            ],
        }),
        "disable_game_bar_presence_writer" => Some(LocalizedTweakText {
            title: "Отключить Game Bar Presence, кнопку Nexus и стартовые подсказки",
            description: "Отключает запуск контроллером Nexus и стартовые панели подсказок Game Bar через доступные для записи пользователем ключи HKCU\\Software\\Microsoft\\GameBar, убирая нагрузку хуков оверлея без ошибок прав TrustedInstaller.",
            broken_features: &[
                "Запуск кнопкой Xbox Guide Nexus и стартовая панель Game Bar отключены",
            ],
        }),
        "disable_widgets_news" => Some(LocalizedTweakText {
            title: "Отключить виджеты Windows и ленту новостей MSN",
            description: "Отключает значок виджетов на панели задач и блокирует фоновую загрузку новостных лент MSN, трендов и виджетов погоды.",
            broken_features: &[
                "Всплывающая панель виджетов на панели задач будет отключена",
            ],
        }),
        "gamedvr_config_store" => Some(LocalizedTweakText {
            title: "Отключить оверлей записи Xbox GameDVR",
            description: "Отключает фоновую запись игрового процесса и внедрение хуков Xbox Game Bar, устраняя просадки времени кадра и микрофризы.",
            broken_features: &[
                "Фоновая запись мгновенных повторов через Xbox Game Bar будет отключена",
            ],
        }),
        "gamedvr_app_capture" => Some(LocalizedTweakText {
            title: "Отключить захват приложений Windows GameDVR",
            description: "Отключает фоновые буферы записи аудио и видео в подсистеме Windows GameDVR.",
            broken_features: &[],
        }),
        "gamedvr_policy_disable" => Some(LocalizedTweakText {
            title: "Отключить GameDVR через системную политику",
            description: "Общесистемная блокировка групповой политикой, не позволяющая Windows повторно включать службы захвата GameDVR в фоновом режиме.",
            broken_features: &[],
        }),
        "consumer_features_disable" => Some(LocalizedTweakText {
            title: "Отключить потребительские облачные функции Windows",
            description: "Предотвращает автоматическую загрузку и установку спонсируемых сторонних приложений и игр (например, TikTok, Disney, Candy Crush) во время обслуживания системы.",
            broken_features: &[],
        }),
        "disable_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Отключить задачи телеметрии отзывов, Autochk и Карт",
            description: "Отключает запросы диагностики Центра отзывов SIUF, прокси телеметрии Autochk и фоновые задачи уведомлений обновления Карт (адаптировано из Sophia Script).",
            broken_features: &[
                "Проактивные уведомления с опросами Центра отзывов будут отключены",
            ],
        }),
        "disable_hpet_synthetic_timer" => Some(LocalizedTweakText {
            title: "Отключить динамический тик (синтетический таймер ядра)",
            description: "Отключает энергосберегающий динамический тик в ядре Windows NT, предотвращая дрейф синтетического таймера и обеспечивая стабильные высокочастотные запросы таймера Invariant TSC для минимальной задержки DPC.",
            broken_features: &[
                "Незначительно более высокое энергопотребление ЦП в простое",
            ],
        }),
        "optimize_tcp_congestion_tuning" => Some(LocalizedTweakText {
            title: "Оптимизировать управление перегрузками TCP и автонастройку окна",
            description: "Включает высокопроизводительные метки времени TCP по стандарту RFC 1323 и отключает широковещательный трафик Web Services Dynamic Discovery (WSD) для минимизации сетевой задержки в играх и задержек сериализации пакетов.",
            broken_features: &[],
        }),
        "disable_windows_error_reporting" => Some(LocalizedTweakText {
            title: "Отключить политику и отправку отчётов об ошибках Windows",
            description: "Устанавливает блокировку групповой политикой для службы отчётов об ошибках Windows и запрещает отправку дополнительных дампов сбоев на серверы Microsoft.",
            broken_features: &[
                "Автоматическая отправка отчётов о сбоях и телеметрии заблокирована",
            ],
        }),
        "disable_bing_search_start_menu" => Some(LocalizedTweakText {
            title: "Отключить рекомендации в меню «Пуск» и рекламу Iris",
            description: "Убирает спонсируемые рекомендации сайтов и учётной записи, советы Iris и блок недавно добавленных приложений в макетах меню «Пуск» Windows 10/11.",
            broken_features: &[
                "Рекламные рекомендации в меню «Пуск» скрыты",
            ],
        }),
        "disable_cortana_autostart" => Some(LocalizedTweakText {
            title: "Отключить поиск Cortana для пользователя и кнопку на панели задач",
            description: "Отключает активацию Cortana для текущего пользователя, разрешения геолокации поиска и скрывает кнопку Cortana на панели задач.",
            broken_features: &[
                "Пользовательская интеграция поиска Cortana на панели задач будет отключена",
            ],
        }),
        "enable_long_file_paths" => Some(LocalizedTweakText {
            title: "Включить поддержку длинных путей NTFS Win32 (>260 символов)",
            description: "Снимает устаревшее ограничение MAX_PATH в 260 символов в Windows 10/11 через LongPathsEnabled=1, предотвращая ошибки обрезки путей при сборке проектов и распаковке архивов.",
            broken_features: &[],
        }),
        "disable_background_search_toggle_and_app_policy" => Some(LocalizedTweakText {
            title: "Запретить фоновую работу приложений и фоновый переключатель поиска",
            description: "Устанавливает AppPrivacy\\LetAppsRunInBackground=2 (Принудительный запрет) и отключает Search\\BackgroundAppGlobalToggle=0, чтобы неактивные UWP-приложения не расходовали ЦП и ОЗУ в фоне.",
            broken_features: &[
                "UWP-приложения вроде Store и Почты не будут синхронизироваться в закрытом состоянии",
            ],
        }),
        "extend_webclient_file_size_limit" => Some(LocalizedTweakText {
            title: "Увеличить лимит размера файлов WebClient / WebDAV до максимума (4 ГБ)",
            description: "Повышает лимит загрузки службы Windows WebClient с 50 МБ до максимальных 4 ГБ (0xFFFFFFFF), устраняя сбои копирования с подключённых сетевых дисков WebDAV.",
            broken_features: &[],
        }),
        "reduce_services_pipe_timeout" => Some(LocalizedTweakText {
            title: "Сократить таймаут запуска и завершения служб до 15 секунд",
            description: "Уменьшает ServicesPipeTimeout со стандартных 30000 мс (30 с) до 15000 мс (15 с), чтобы зависшие фоновые службы быстрее завершались при загрузке и выключении.",
            broken_features: &[
                "Очень медленные пользовательские службы БД, запускающиеся дольше 15 с, могут завершиться по таймауту",
            ],
        }),
        "mmcss_scheduler_full_optimization" => Some(LocalizedTweakText {
            title: "Полная оптимизация планировщика мультимедиа MMCSS (NoLazyMode, AlwaysOn, High SFIO)",
            description: "Настраивает Multimedia\\SystemProfile NoLazyMode=1, AlwaysOn=1, а также Games Tasks Scheduling Category='High' и SFIO Priority='High' для бесперебойного планирования потоков игр и мультимедиа.",
            broken_features: &[],
        }),
        "enable_directx_windowed_game_optimizations" => Some(LocalizedTweakText {
            title: "Включить оптимизацию оконного режима DirectX Flip-Model и VRR (безопасное слияние)",
            description: "Безопасно объединяет параметры SwapEffectUpgradeEnable=1 и VRROptimizeEnable=1 в DirectXUserGlobalSettings, не перезаписывая ваши индивидуальные настройки GPU и AutoHDR для игр.",
            broken_features: &[],
        }),
        "disable_hardware_overlays_and_min_fps" => Some(LocalizedTweakText {
            title: "Отключить аппаратные оверлеи ядра и ограничение минимального FPS DWM",
            description: "Устанавливает GraphicsDrivers\\DisableOverlays=1 и Dwm\\OverlayMinFPS=0, устраняя задержки продвижения плоскостей аппаратного оверлея DWM и статтеры при низкой частоте обновления.",
            broken_features: &[
                "Незначительно более высокое энергопотребление композитора DWM в простое на ноутбуках",
            ],
        }),
        "nvidia_restore_legacy_sharpening" => Some(LocalizedTweakText {
            title: "Вернуть классическую настройку резкости драйвера NVIDIA (EnableGR535=0)",
            description: "Устанавливает Global\\FTS\\EnableGR535=0, чтобы панель управления NVIDIA отображала классическую «Резкость изображения» с низкими накладными расходами вместо принудительного нового фильтра оверлея.",
            broken_features: &[],
        }),
        "nvidia_dlss_debug_indicator" => Some(LocalizedTweakText {
            title: "Отображать отладочный индикатор версии и пресета NVIDIA DLSS",
            description: "Устанавливает Global\\NGXCore\\ShowDlssIndicator=1024 для отображения активной версии SDK DLSS, масштабирования разрешения и пресета поверх экрана в играх DirectX и Vulkan.",
            broken_features: &[
                "Отображает диагностический водяной знак DLSS в левом нижнем углу поддерживаемых игр",
            ],
        }),
        "disable_game_bar_tips_and_capture" => Some(LocalizedTweakText {
            title: "Отключить кнопку Nexus контроллера, советы запуска и буфер записи Game Bar",
            description: "Выключает UseNexusForGameBarEnabled (запуск кнопкой Xbox Guide), всплывающие советы ShowStartupPanel и фоновую кольцевую запись HistoricalCaptureEnabled.",
            broken_features: &[
                "Кнопка Nexus на геймпаде не вызывает оверлей Game Bar",
            ],
        }),
        "optimize_mouse_thresholds_and_hover" => Some(LocalizedTweakText {
            title: "Обнулить пороги ускорения мыши и ускорить отклик наведения до 10 мс",
            description: "Устанавливает MouseThreshold1='0', MouseThreshold2='0' и MouseHoverTime='10' мс, чтобы кривая ускорения указателя была абсолютно линейной, а подсказки и панель задач реагировали мгновенно.",
            broken_features: &[],
        }),
        "disable_extra_accessibility_hotkeys" => Some(LocalizedTweakText {
            title: "Отключить случайные горячие клавиши управления указателем, контрастности и Экранного диктора",
            description: "Отключает комбинации Alt+Shift+NumLock (Управление указателем), Alt+Shift+PrtScn (Высокая контрастность) и Win+Ctrl+Enter (Экранный диктор) во время игр.",
            broken_features: &[
                "Горячие клавиши специальных возможностей нужно будет включать через приложение «Параметры»",
            ],
        }),
        "disable_smart_multi_homed_name_resolution" => Some(LocalizedTweakText {
            title: "Отключить многоканальное разрешение имён DNS (SMHNR)",
            description: "Устанавливает DNSClient\\DisableSmartNameResolution=1, чтобы Windows отправляла DNS-запросы строго через основной интерфейс вместо параллельной рассылки по VPN и виртуальным адаптерам.",
            broken_features: &[
                "VPN с раздельным туннелированием (split-tunnel) должны корректно настраивать DNS-метрики адаптера",
            ],
        }),
        "harden_windows_update_ux_and_store" => Some(LocalizedTweakText {
            title: "Запретить перезапись драйверов WU, внезапные перезагрузки и автозагрузку Store",
            description: "Исключает обновление драйверов GPU и периферии через Центр обновления, запрещает принудительные перезагрузки при активном пользователе, открывает паузу обновлений до 3650 дней и отключает тихие фоновые загрузки Microsoft Store, сохраняя обновления безопасности.",
            broken_features: &[
                "Обновления приложений Microsoft Store нужно проверять вручную",
            ],
        }),
        "power_settings_pcie_usb_nvme_epb" => Some(LocalizedTweakText {
            title: "Сверхнизкая задержка активной схемы питания (PCIe ASPM, USB, NVMe, EPP, PowerThrottling)",
            description: "Использует нативный API Windows powrprof в текущей активной схеме питания (работает для Сбалансированной, Высокой производительности и пользовательских схем) для отключения от сети PCIe ASPM, выборочного отключения USB, энергосбережения USB 3, сна основного/вторичного NVMe и установки EPP=0, а также отключает PowerThrottling ядра.",
            broken_features: &[
                "Повышенное энергопотребление от сети в простое на ПК и подключённых к розетке ноутбуках",
            ],
        }),
        "disable_storage_sense_policy" => Some(LocalizedTweakText {
            title: "Отключить автоматические фоновые проверки «Контроля памяти» (Storage Sense)",
            description: "Запрещает «Контролю памяти» Windows запускать автоматическую фоновую очистку диска во время интерактивных сеансов.",
            broken_features: &[
                "Временные файлы и Корзину потребуется очищать вручную через раздел «Очистка и AppX»",
            ],
        }),
        "consolidate_svchost_split_threshold" => Some(LocalizedTweakText {
            title: "Динамически объединить хосты служб по объёму ОЗУ (SvcHostSplitThresholdInKB) [Опционально]",
            description: "Динамически вычисляет SvcHostSplitThresholdInKB на основе физического объёма ОЗУ (TotalRamInKb), сокращая число процессов и накладные расходы виртуальной памяти на ПК с 8 ГБ+ ОЗУ. Внимание: снижает изоляцию сбоев между службами.",
            broken_features: &[
                "Изоляция сбоев общих процессов svchost снижается; в Диспетчере задач отображается меньше сгруппированных служб",
            ],
        }),
        "disable_vbs_deviceguard" => Some(LocalizedTweakText {
            title: "Отключить безопасность на основе виртуализации (VBS) [Компромисс безопасности]",
            description: "Отключает EnableVirtualizationBasedSecurity и RequirePlatformSecurityFeatures. Может повысить частоту кадров в играх на 5–15% на процессорах с накладными расходами виртуализации ценой отключения границ безопасности ядра через гипервизор.",
            broken_features: &[
                "Отключает изоляцию безопасности VBS; может появиться предупреждение античита или Безопасности Windows",
            ],
        }),
        "disable_hvci_memory_integrity" => Some(LocalizedTweakText {
            title: "Отключить изоляцию ядра / целостность памяти (HVCI) [Компромисс безопасности]",
            description: "Отключает целостность кода под управлением гипервизора (HVCI). Снижает задержку DPC и прерываний в соревновательных играх на системах, где HVCI ухудшает время кадра, ценой отключения проверки подписи драйверов гипервизором.",
            broken_features: &[
                "В разделе «Безопасность Windows → Изоляция ядра» отобразится предупреждение о выключенной целостности памяти",
            ],
        }),
        "disable_prefetch_and_sysmain" => Some(LocalizedTweakText {
            title: "Отключить Prefetcher ядра и SysMain (Superfetch) для быстрых NVMe SSD [Опционально]",
            description: "Отключает упреждающую выборку приложений/загрузки (Prefetcher) и останавливает предзагрузку SysMain в ОЗУ на быстрых накопителях NVMe, где кэширование создаёт лишний фоновый дисковый ввод-вывод.",
            broken_features: &[
                "Холодный запуск программ может немного замедлиться на механических SATA HDD",
            ],
        }),
        "disable_advertising_id" => Some(LocalizedTweakText {
            title: "Отключить рекламный идентификатор",
            description: "Отключает персональный рекламный идентификатор, который Windows передаёт приложениям для таргетированной рекламы и межпрограммного отслеживания.",
            broken_features: &[
                "Приложения не будут получать уникальный рекламный идентификатор",
            ],
        }),
        "disable_cortana" => Some(LocalizedTweakText {
            title: "Отключить Cortana",
            description: "Отключает Cortana через групповую политику, запрещая её запуск, прослушивание и отправку данных в облачные службы Microsoft.",
            broken_features: &[
                "Голосовой помощник Cortana будет отключён",
            ],
        }),
        "disable_activity_history" => Some(LocalizedTweakText {
            title: "Отключить журнал действий / временную шкалу (Timeline)",
            description: "Отключает функцию «Журнал действий» Windows (Timeline), не позволяя системе собирать и хранить историю ваших действий, посещённых сайтов и открытых документов.",
            broken_features: &[
                "Временная шкала / журнал действий в представлении задач будет пустым",
            ],
        }),
        "disable_online_speech_recognition" => Some(LocalizedTweakText {
            title: "Отключить распознавание речи в сети",
            description: "Отключает облачную службу распознавания речи. Голосовые данные не будут отправляться на серверы Microsoft для обработки.",
            broken_features: &[
                "Облачное распознавание речи не будет работать; локальное офлайн-распознавание не затронуто",
            ],
        }),
        "disable_tailored_experiences" => Some(LocalizedTweakText {
            title: "Отключить персонализированные возможности",
            description: "Запрещает Microsoft использовать диагностические данные для показа персонализированных советов, рекламы и рекомендаций в Windows.",
            broken_features: &[
                "Персонализированные подсказки и советы Windows будут отключены",
            ],
        }),
        "disable_windows_recall" => Some(LocalizedTweakText {
            title: "Отключить Windows Recall (ИИ-снимки экрана)",
            description: "Отключает функцию Windows Recall из Windows 11 24H2, которая периодически делает индексируемые ИИ снимки экрана с вашей активностью. Требуется сборка 26100+.",
            broken_features: &[
                "Функция ИИ-снимков Windows Recall будет отключена",
            ],
        }),
        "disable_cloud_content_spotlight" => Some(LocalizedTweakText {
            title: "Отключить облачный контент и рекомендации «Интересного» (Spotlight)",
            description: "Отключает «Windows: интересное», рекомендуемый контент в Параметрах и предложения предустановленных приложений из облака.",
            broken_features: &[
                "Изображения «Windows: интересное» на экране блокировки будут отключены",
                "Рекомендуемые приложения в меню «Пуск» не будут появляться",
            ],
        }),
        "disable_location_tracking" => Some(LocalizedTweakText {
            title: "Отключить отслеживание местоположения",
            description: "Глобально отключает службу определения местоположения Windows, запрещая приложениям и ОС доступ к вашим географическим координатам.",
            broken_features: &[
                "Приложениям, требующим геопозицию (Карты, Погода), доступ к местоположению будет закрыт",
            ],
        }),
        "disable_activity_cloud_sync" => Some(LocalizedTweakText {
            title: "Отключить облачный буфер обмена и синхронизацию между устройствами",
            description: "Отключает сохранение журнала буфера обмена Windows и облачную синхронизацию между устройствами через групповую политику, чтобы скопированные пароли и конфиденциальные данные оставались строго локальными.",
            broken_features: &[
                "Журнал буфера обмена Win+V и синхронизация вставки между устройствами будут отключены",
            ],
        }),
        "disable_cdp_service" => Some(LocalizedTweakText {
            title: "Отключить службу платформы подключённых устройств (CDPUserSvc)",
            description: "Отключает службу Connected Devices Platform Service (CDPUserSvc), блокируя межустройственную телеметрию, широковещательную синхронизацию приложений и фоновые маячки активности.",
            broken_features: &[
                "Обмен с устройствами поблизости и синхронизация со смартфонами будут отключены",
            ],
        }),
        "spotlight_desktop_disable" => Some(LocalizedTweakText {
            title: "Отключить загрузку фонов «Windows: интересное» для экрана блокировки",
            description: "Отключает автоматическую фоновую загрузку обоев «Windows: интересное» для экрана блокировки и рекламных изображений Microsoft.",
            broken_features: &[],
        }),
        "spotlight_features_disable" => Some(LocalizedTweakText {
            title: "Отключить советы и рекламу на экране блокировки",
            description: "Отключает советы, подсказки и рекламу приложений Microsoft Store на экране блокировки, доставляемые через Content Delivery Manager.",
            broken_features: &[],
        }),
        "win32_priority_separation" => Some(LocalizedTweakText {
            title: "Win32PrioritySeparation — приоритет активного окна",
            description: "Настраивает короткие переменные кванты планировщика с высоким приоритетом активного окна (0x26 = 38). Отдаёт приоритет активным приложениям для снижения задержки ввода в играх и интерактивных задачах.",
            broken_features: &[],
        }),
        "menu_show_delay" => Some(LocalizedTweakText {
            title: "Сократить задержку показа меню до 0 мс",
            description: "Убирает искусственную задержку перед открытием меню при наведении. По умолчанию составляет 400 мс; значение 0 делает открытие меню мгновенным.",
            broken_features: &[],
        }),
        "mmcss_gpu_priority" => Some(LocalizedTweakText {
            title: "MMCSS — повысить приоритет игр и Pro Audio",
            description: "Повышает приоритет потоков для категорий Games и Pro Audio в службе планировщика мультимедиа (MMCSS), обеспечивая звуковым и игровым потокам реального времени более высокий приоритет планирования.",
            broken_features: &[],
        }),
        "mouse_data_queue_size" => Some(LocalizedTweakText {
            title: "Увеличить размер буфера очереди данных мыши",
            description: "Увеличивает размер буфера очереди событий ввода мыши со стандартных 100 до 256 записей, снижая вероятность потери событий на игровых мышах с высокой частотой опроса.",
            broken_features: &[],
        }),
        "keyboard_response" => Some(LocalizedTweakText {
            title: "Оптимизировать отклик клавиатуры",
            description: "Устанавливает минимальную задержку перед повтором клавиш и максимальную скорость повтора через реестр, а также увеличивает буфер очереди данных клавиатуры.",
            broken_features: &[],
        }),
        "disable_wer_queue" => Some(LocalizedTweakText {
            title: "Отключить очередь отчётов об ошибках Windows (WER)",
            description: "Отключает очередь Windows Error Reporting (WER), не позволяя системе записывать дампы сбоев и отчёты в фоновом режиме после падения программ.",
            broken_features: &[
                "Отчёты о сбоях не будут отправляться в Microsoft",
            ],
        }),
        "auto_end_tasks" => Some(LocalizedTweakText {
            title: "Автоматически завершать зависшие задачи при выключении",
            description: "Автоматически закрывает зависшие программы при завершении работы вместо вывода окна ожидания. Также сокращает WaitToKillAppTimeout и HungAppTimeout для ускорения выхода и выключения ПК.",
            broken_features: &[
                "Несохранённые данные в зависших приложениях могут быть потеряны при выключении",
            ],
        }),
        "network_throttling_disable" => Some(LocalizedTweakText {
            title: "Отключить индекс ограничения сети (NetworkThrottlingIndex)",
            description: "Отключает мультимедийное ограничение сетевого трафика Windows (устанавливает NetworkThrottlingIndex в 0xFFFFFFFF), предотвращая потери пакетов и скачки пинга при высокой нагрузке на сеть или в играх.",
            broken_features: &[],
        }),
        "system_responsiveness_gaming" => Some(LocalizedTweakText {
            title: "Максимизировать отзывчивость системы для активного окна",
            description: "Устанавливает SystemResponsiveness в 0%, выделяя 100% циклов процессора интерактивным играм и приложениям на переднем плане вместо резервирования 20% под фоновые задачи ОС.",
            broken_features: &[],
        }),
        "ntfs_disable_last_access" => Some(LocalizedTweakText {
            title: "Отключить обновление меток последнего доступа NTFS",
            description: "Запрещает файловой системе NTFS обновлять метаданные времени последнего доступа при каждом чтении файла, заметно снижая лишнюю нагрузку ввода-вывода на SSD и NVMe.",
            broken_features: &[
                "Свойства файлов не будут показывать точную дату последнего доступа",
            ],
        }),
        "disable_paging_executive" => Some(LocalizedTweakText {
            title: "Удерживать ядро и драйверы Windows в оперативной памяти (RAM)",
            description: "Заставляет исполнительное ядро Windows NT и процедуры драйверов постоянно находиться в физической памяти ОЗУ без выгрузки в файл подкачки, обеспечивая мгновенный отклик на прерывания ядра.",
            broken_features: &[],
        }),
        "large_system_cache" => Some(LocalizedTweakText {
            title: "Включить большой системный файловый кэш",
            description: "Настраивает Windows на поддержание увеличенного кэша файловой системы в физической ОЗУ, ускоряя повторный доступ к файлам и подгрузку игровых текстур.",
            broken_features: &[],
        }),
        "disable_fast_startup" => Some(LocalizedTweakText {
            title: "Отключить «Быстрый запуск» (защита от утечек гибернации)",
            description: "Отключает быстрый запуск Windows (hiberboot), гарантируя полноценную чистую загрузку ядра при каждом выключении и предотвращая утечки памяти и сбои состояний драйверов между сеансами.",
            broken_features: &[
                "Холодная загрузка на медленных механических дисках HDD может замедлиться на 1–2 секунды",
            ],
        }),
        "ntfs_disable_8dot3" => Some(LocalizedTweakText {
            title: "Отключить создание коротких имён NTFS 8.3",
            description: "Отключает генерацию устаревших коротких имён MS-DOS формата 8.3 на томах NTFS, устраняя задержки индексации каталогов и лишние операции записи на быстрых NVMe и SSD.",
            broken_features: &[],
        }),
        "increase_mft_zone_reservation" => Some(LocalizedTweakText {
            title: "Увеличить резерв зоны главной таблицы файлов (MFT)",
            description: "Резервирует увеличенный непрерывный буфер на диске для главной таблицы файлов NTFS (MFT), предотвращая её фрагментацию и сохраняя максимальную скорость работы с диском.",
            broken_features: &[],
        }),
        "tcp_ack_frequency" => Some(LocalizedTweakText {
            title: "Низкая задержка TCP (TcpAckFrequency)",
            description: "Заставляет сетевой стек Windows TCP отправлять подтверждения ACK немедленно без ожидания объединения по таймеру, снижая сетевой пинг в онлайн-играх.",
            broken_features: &[],
        }),
        "tcp_no_delay_global" => Some(LocalizedTweakText {
            title: "Отключить алгоритм Нейгла глобально (TCPNoDelay)",
            description: "Отключает алгоритм буферизации пакетов Нейгла на активных сетевых интерфейсах, отправляя мелкие пакеты без задержки для минимального времени отклика сети.",
            broken_features: &[],
        }),
        "disable_mouse_acceleration" => Some(LocalizedTweakText {
            title: "Отключить повышенную точность установки указателя (ввод мыши 1:1)",
            description: "Отключает нелинейное программное ускорение курсора Windows, обеспечивая точное аппаратное перемещение 1:1, необходимое для прицеливания в играх.",
            broken_features: &[],
        }),
        "disable_nvidia_telemetry_service" => Some(LocalizedTweakText {
            title: "Отключить службу контейнера телеметрии NVIDIA",
            description: "Отключает службу NvTelemetryContainer, которая собирает и отправляет статистику использования драйверов NVIDIA и данные о сбоях.",
            broken_features: &[
                "Данные о сбоях и телеметрия использования NVIDIA не будут собираться",
            ],
        }),
        "disable_nvidia_telemetry_tasks" => Some(LocalizedTweakText {
            title: "Отключить задачи телеметрии NVIDIA в планировщике",
            description: "Отключает запланированные задачи NvTmMon, NvTmRep и NvProfile, периодически отправляющие данные телеметрии NVIDIA.",
            broken_features: &[],
        }),
        "disable_adobe_update_service" => Some(LocalizedTweakText {
            title: "Отключить службы обновления и проверки подлинности Adobe",
            description: "Отключает службы Adobe Acrobat Update Service (AdobeARMservice) и Adobe Genuine Monitor Service, предотвращая фоновые проверки обновлений и телеметрию лицензий.",
            broken_features: &[
                "Продукты Adobe не будут обновляться автоматически; обновляйте их вручную",
            ],
        }),
        "disable_edge_update" => Some(LocalizedTweakText {
            title: "Отключить службы обновления Microsoft Edge",
            description: "Отключает службы обновления Microsoft Edge (edgeupdate и edgeupdatem) и их задачи планировщика. Браузер продолжит работать, но не будет автоматически обновляться и отправлять фоновые запросы.",
            broken_features: &[
                "Microsoft Edge не будет обновляться автоматически; патчи безопасности нужно ставить вручную",
            ],
        }),
        "disable_chrome_update" => Some(LocalizedTweakText {
            title: "Отключить службы обновления Google Chrome",
            description: "Отключает службы Google Update (gupdate и gupdatem) и связанные задачи планировщика. Chrome продолжит работать, но не будет автоматически обновляться.",
            broken_features: &[
                "Google Chrome не будет обновляться автоматически; патчи безопасности нужно ставить вручную",
            ],
        }),
        "disable_steam_client_service" => Some(LocalizedTweakText {
            title: "Отключить автозапуск службы Steam Client Service",
            description: "Переводит службу Steam Client Service из автоматического запуска в ручной режим. Steam по-прежнему запустит службу при открытии клиента, но она не будет постоянно висеть в фоне, когда Steam закрыт.",
            broken_features: &[],
        }),
        "disable_razer_telemetry" => Some(LocalizedTweakText {
            title: "Отключить телеметрию и аналитику Razer",
            description: "Отключает автозапуск служб Razer Game Manager Service и Razer Chroma SDK, снижая фоновую телеметрию и сбор аналитики со стороны Razer Synapse.",
            broken_features: &[
                "Функции автозапуска Razer Synapse могут потребовать ручного включения",
            ],
        }),
        "disable_diagtrack" => Some(LocalizedTweakText {
            title: "Отключить DiagTrack (Функциональные возможности и телеметрия)",
            description: "Отключает службу «Функциональные возможности для подключённых пользователей и телеметрия» (DiagTrack / diagsvc) — основную службу сбора телеметрии Windows.",
            broken_features: &[
                "Диагностические данные не будут отправляться в Microsoft",
            ],
        }),
        "disable_dmwappushservice" => Some(LocalizedTweakText {
            title: "Отключить dmwappushservice (Маршрутизация push-сообщений WAP)",
            description: "Отключает службу маршрутизации push-сообщений WAP для управления устройствами, используемую для передачи телеметрии.",
            broken_features: &[
                "Push-сообщения MDM не будут приниматься",
            ],
        }),
        "disable_telemetry_registry" => Some(LocalizedTweakText {
            title: "Отключить телеметрию через политику реестра AllowTelemetry",
            description: "Устанавливает политику AllowTelemetry в 0 (Безопасность/Выкл) — минимально возможный уровень телеметрии. В редакциях Enterprise и Education полностью отключает телеметрию; в Home и Pro снижает её до минимума.",
            broken_features: &[
                "Некоторые функции программы Windows Insider и диагностики могут не работать",
            ],
        }),
        "disable_compat_telrunner" => Some(LocalizedTweakText {
            title: "Отключить задачи оценки совместимости приложений (CompatTelRunner)",
            description: "Отключает задачи планировщика Microsoft Compatibility Appraiser и ProgramDataUpdater, запускающие CompatTelRunner.exe для сканирования установленного ПО и отправки отчётов совместимости.",
            broken_features: &[
                "Автоматическая проверка совместимости обновлений Windows не будет запускаться",
            ],
        }),
        "disable_ceip" => Some(LocalizedTweakText {
            title: "Отключить программу улучшения качества ПО (CEIP)",
            description: "Отключает запланированные задачи CEIP (Consolidator, UsbCeip, KernelCeipTask) и соответствующий ключ политики реестра CEIP.",
            broken_features: &[
                "Данные телеметрии программы CEIP не будут собираться",
            ],
        }),
        "disable_wer_service" => Some(LocalizedTweakText {
            title: "Отключить службу отчётов об ошибках Windows (WerSvc)",
            description: "Отключает службу Windows Error Reporting (WerSvc), предотвращая загрузку отчётов о сбоях и телеметрии ошибок на серверы Microsoft.",
            broken_features: &[
                "Отчёты о сбоях не будут отправляться в Microsoft",
                "Журнал отчётов о проблемах не будет обновляться",
            ],
        }),
        "disable_connected_user_experiences" => Some(LocalizedTweakText {
            title: "Отключить функции взаимодействия и рекомендации Windows",
            description: "Отключает подсказки и предложения в приложении «Параметры», меню «Пуск» и на экране блокировки, формируемые на основе облачного контента и телеметрии.",
            broken_features: &[
                "Предложения и подсказки Windows не будут отображаться",
            ],
        }),
        "disable_diagnostic_data_viewer" => Some(LocalizedTweakText {
            title: "Отключить средство просмотра диагностических данных",
            description: "Отключает ведение локального журнала средства просмотра диагностических данных, хранящего копию отправляемых в Microsoft сведений. Экономит операции диска и место.",
            broken_features: &[
                "Локальный просмотр диагностических данных в приложении «Средство просмотра диагностических данных» будет недоступен",
            ],
        }),
        "disable_delivery_optimization_p2p" => Some(LocalizedTweakText {
            title: "Отключить P2P-раздачу обновлений в Оптимизации доставки",
            description: "Ограничивает службу оптимизации доставки Windows (DODownloadMode = 0), запрещая системе использовать ваш исходящий канал интернета для раздачи обновлений Windows посторонним ПК.",
            broken_features: &[],
        }),
        "disable_diagnostic_execution_service" => Some(LocalizedTweakText {
            title: "Отключить службу выполнения диагностики (diagsvc)",
            description: "Отключает Diagnostic Execution Service (diagsvc), выполняющую фоновую диагностику и запуск диагностических пакетов телеметрии по расписанию.",
            broken_features: &[],
        }),
        "wersvc_service_disable" => Some(LocalizedTweakText {
            title: "Отключить стандартный сборщик концентратора диагностики (Diagnostics Hub)",
            description: "Отключает службу Diagnostics Hub Standard Collector Service (Visual Studio / Windows), останавливая фоновых сборщиков трассировки ETW в простое.",
            broken_features: &[
                "Сбор трассировки Diagnostics Hub в VS/Windows потребует ручного запуска службы",
            ],
        }),
        "disable_remote_registry" => Some(LocalizedTweakText {
            title: "Отключить службу удалённого реестра (RemoteRegistry)",
            description: "Отключает службу RemoteRegistry, запрещая удалённым пользователям и внешним сетевым узлам считывать или изменять ключи локального реестра.",
            broken_features: &[
                "Удалённое администрирование реестра по локальной сети будет отключено",
            ],
        }),
        "disable_disk_diagnostic_task" => Some(LocalizedTweakText {
            title: "Отключить задачу диагностической телеметрии дисков",
            description: "Отключает запланированную задачу, собирающую метрики использования накопителей и аппаратную диагностику для отчётов телеметрии.",
            broken_features: &[],
        }),
        "disable_winsat_task" => Some(LocalizedTweakText {
            title: "Отключить фоновый тест производительности WinSAT",
            description: "Отключает запланированную задачу Windows System Assessment Tool от запуска фоновых нагрузочных тестов диска и GPU во время простоя системы.",
            broken_features: &[],
        }),
        "enable_classic_context_menu" => Some(LocalizedTweakText {
            title: "Вернуть полное классическое контекстное меню Windows 10",
            description: "Регистрирует переопределение CLSID в HKCU\\Software\\Classes, чтобы правый клик сразу открывал полное контекстное меню без лишнего нажатия «Показать дополнительные параметры».",
            broken_features: &[
                "Компактное современное контекстное меню Windows 11 заменено полным классическим",
            ],
        }),
        "explorer_power_user_defaults" => Some(LocalizedTweakText {
            title: "Показывать расширения, полный путь в заголовке, «Этот компьютер» и скрыть баннеры OneDrive",
            description: "Включает отображение расширений файлов и скрытых элементов, открывает Проводник сразу на «Этот компьютер» вместо «Быстрого доступа», выводит полный путь в заголовке окна, включает компактный вид и скрывает уведомления синхронизации OneDrive.",
            broken_features: &[],
        }),
        "explorer_hide_win11_home_gallery_nav" => Some(LocalizedTweakText {
            title: "Скрыть «Главная» и «Галерея» из панели навигации Проводника Win11",
            description: "Открепляет разделы «Главная» (Home) и «Галерея» (Gallery) в левой панели навигации Проводника Windows 11 через HKCU\\Software\\Classes\\CLSID.",
            broken_features: &[
                "Быстрые ссылки «Главная» и «Галерея» скрыты из боковой панели Проводника",
            ],
        }),
        "explorer_disable_startup_delay_and_jpeg_compression" => Some(LocalizedTweakText {
            title: "Убрать задержку автозагрузки программ и сжатие JPEG для обоев",
            description: "Устанавливает Explorer\\Serialize\\StartupDelayInMSec=0, чтобы программы автозапуска стартовали без искусственной 10-секундной задержки, и задаёт JPEGImportQuality=100 для обоев без потерь качества.",
            broken_features: &[],
        }),
        "explorer_desktop_this_pc_and_disable_autoplay" => Some(LocalizedTweakText {
            title: "Показать значок «Этот компьютер» на рабочем столе и отключить автозапуск носителей",
            description: "Размещает значок «Этот компьютер» на рабочем столе и отключает Autoplay/AutoRun для съёмных USB-накопителей и оптических дисков, блокируя всплывающие окна и угрозы автозапуска.",
            broken_features: &[
                "Подключаемые USB-накопители и SD-карты больше не открывают окно автозапуска",
            ],
        }),
        "disable_visual_effects_userpreferencesmask_bits" => Some(LocalizedTweakText {
            title: "Убрать тяжёлую анимацию затухания, скольжения и теней через битовые маски UserPreferencesMask и DWM",
            description: "Точечно изменяет биты UserPreferencesMask (отключая затухание и скольжение меню, подсказок, тени курсора, анимацию списков и элементов управления при сохранении сглаживания шрифтов) и задаёт EnableAeroPeek=0.",
            broken_features: &[
                "Классический быстрый отклик интерфейса без плавного затухания, скольжения и теней",
            ],
        }),
        "taskbar_win11_power_layout_and_end_task" => Some(LocalizedTweakText {
            title: "Выровнять панель задач Win11 слева, скрыть виджеты/поиск и включить «Завершить задачу» (Pro/Ent)",
            description: "Выравнивает значки панели задач слева (TaskbarAl=0), скрывает кнопки «Представление задач», «Виджеты» (TaskbarDa=0) и поле поиска, включает пункт «Завершить задачу» по правому клику, отключает сворачивание встряхиванием (Aero Shake) и убирает блок «Рекомендуем» в меню «Пуск» (Win11 22H2+ Pro/Enterprise/Education).",
            broken_features: &[
                "Значки панели задач выровнены по левому краю; кнопки «Представление задач» и «Виджеты» скрыты",
            ],
        }),
        "hide_win10_meet_now_tray" => Some(LocalizedTweakText {
            title: "Скрыть значок камеры «Собрание» (Meet Now) в трее Windows 10",
            description: "Скрывает значок Skype «Собрание» (Meet Now) из области уведомлений Windows 10 (только для сборок Windows 10).",
            broken_features: &[],
        }),
        "enable_windows_dark_mode_and_clock_seconds" => Some(LocalizedTweakText {
            title: "Включить тёмную тему Windows и системных приложений + секунды на часах",
            description: "Переключает AppsUseLightTheme=0 и SystemUsesLightTheme=0 для нативной тёмной темы интерфейса Windows, а также включает отображение секунд в часах на панели задач.",
            broken_features: &[],
        }),
        "disable_startup_sound_and_audio_ducking" => Some(LocalizedTweakText {
            title: "Отключить звук запуска Windows и приглушение громкости при звонках",
            description: "Отключает звуковой сигнал входа в Windows и запрещает системе снижать громкость игр и музыки на 80%, когда Discord, Teams или голосовые приложения открывают канал связи.",
            broken_features: &[],
        }),
        "enable_verbose_logon_status" => Some(LocalizedTweakText {
            title: "Включить подробные сообщения состояния загрузки, входа и выключения Windows",
            description: "Устанавливает VerboseStatus=1, чтобы Windows отображала точные этапы запуска служб и обновлений вместо общих экранов загрузки.",
            broken_features: &[],
        }),
        "disable_lock_screen_pro" => Some(LocalizedTweakText {
            title: "Пропускать занавес экрана блокировки перед входом (Политика Pro/Enterprise)",
            description: "Сразу переходит к вводу пароля или ПИН-кода после включения или блокировки ПК без необходимости смахивать заставку экрана блокировки.",
            broken_features: &[
                "Шторка экрана блокировки отключена в редакциях Pro и Enterprise",
            ],
        }),
        "enable_windows_developer_mode" => Some(LocalizedTweakText {
            title: "Включить режим разработчика Windows (установка MSIX/AppX и символические ссылки)",
            description: "Включает AllowDevelopmentWithoutDevLicense=1, позволяя создавать символические ссылки без запроса UAC и устанавливать неподписанные пакеты MSIX/AppX.",
            broken_features: &[
                "Разрешает неограниченную установку неподписанных пакетов MSIX; предназначено для разработчиков",
            ],
        }),
        _ => None,
    }
}
