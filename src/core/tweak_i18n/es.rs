use super::LocalizedTweakText;

pub fn lookup(id: &str) -> Option<LocalizedTweakText> {
    match id {
        // === ai_privacy_ext.toml (21) ===
        "disable_windows_ai_recall_extended" => Some(LocalizedTweakText {
            title: "Desactivar Windows Recall, Click to Do y agentes de IA de Copilot+",
            description: "Bloquea la captura de instantáneas de Windows Recall, Click to Do, el agente de IA de Configuración, los conectores/espacios de trabajo de agentes de IA y Copilot Runtime mediante claves de Directiva de grupo introducidas en Windows 11 24H2+.",
            broken_features: &["Instantáneas de la línea de tiempo de Windows Recall y superposición de IA Click to Do"],
        }),
        "disable_generative_ai_app_access" => Some(LocalizedTweakText {
            title: "Bloquear el acceso de aplicaciones a modelos de IA generativa y del sistema",
            description: "Deniega a las aplicaciones UWP/WinUI en segundo plano el acceso a las capacidades locales de IA generativa y modelos de IA del sistema mediante las directivas AppPrivacy y CapabilityAccessManager ConsentStore.",
            broken_features: &["Las aplicaciones de terceros no pueden consultar los puntos de conexión de modelos de IA integrados de Windows"],
        }),
        "disable_copilot_shell_and_hardware_key" => Some(LocalizedTweakText {
            title: "Desactivar botón de Copilot en la barra de tareas, avisos, tecla dedicada y micrófono",
            description: "Oculta el botón y los avisos de Windows Copilot en la barra de tareas, neutraliza la asignación de la tecla dedicada de Copilot, bloquea promociones de IA ligadas a la cuenta y deniega el acceso al micrófono a Copilot y OfficeHub.",
            broken_features: &[
                "La tecla dedicada de Copilot no inicia Copilot",
                "Entrada de voz de Copilot bloqueada",
            ],
        }),
        "disable_paint_ai_features" => Some(LocalizedTweakText {
            title: "Desactivar funciones en la nube y de IA generativa en Microsoft Paint",
            description: "Desactiva Image Creator, Cocreator, Relleno generativo, Borrado generativo y Quitar fondo por IA dentro de Windows 11 Paint mediante claves oficiales de Directiva.",
            broken_features: &["Herramientas de generación por IA ocultas en Microsoft Paint"],
        }),
        "disable_edge_ai_policies" => Some(LocalizedTweakText {
            title: "Desactivar Copilot, barra lateral y APIs de IA integradas en Microsoft Edge",
            description: "Desactiva 12 directivas de IA en Microsoft Edge, incluidas Contexto de página de Copilot, barra lateral Hubs, Entra Copilot, redacción en línea, búsqueda de historial por IA, modelos locales, DevTools GenAI y uso compartido del historial.",
            broken_features: &["Barra lateral de Edge Copilot y asistencia de redacción en línea desactivadas"],
        }),
        "disable_office_ai_copilot" => Some(LocalizedTweakText {
            title: "Desactivar entrenamiento de IA de Microsoft Office y Copilots por aplicación",
            description: "Desactiva el entrenamiento de IA de Microsoft 365, el análisis de contenido conectado, el Servicio de seguridad de contenido y los interruptores de Copilot por aplicación en Word, Excel y OneNote.",
            broken_features: &["Asistente Copilot desactivado en Word, Excel y OneNote"],
        }),
        "disable_wmi_telemetry_autologgers" => Some(LocalizedTweakText {
            title: "Desactivar 10 sesiones AutoLogger de telemetría del núcleo WMI",
            description: "Impide que Windows inicie sesiones persistentes de seguimiento ETW AutoLogger en segundo plano (Diagtrack-Listener, SQMLogger, DiagLog, Cellcore, DataMarket, AppModel, CloudExperienceHostOobe, LwtNetLog, WdiContextLog, WiFiSession).",
            broken_features: &["Registros de seguimiento diagnóstico ETW del núcleo desactivados"],
        }),
        "block_silent_preinstalled_apps" => Some(LocalizedTweakText {
            title: "Bloquear aplicaciones preinstaladas en silencio y sugerencias de Inicio y Configuración",
            description: "Impide que Windows ContentDeliveryManager instale silenciosamente aplicaciones promocionales de OEM o Microsoft Store y oculta el contenido sugerido en Configuración, el menú Inicio y la Línea de tiempo.",
            broken_features: &[],
        }),
        "disable_extra_telemetry_policies" => Some(LocalizedTweakText {
            title: "Reforzar bloqueo de comentarios, inventario AppCompat, avisos OOBE y seguimiento",
            description: "Limita MaxTelemetryAllowed=0, suprime los avisos de comentarios de Windows SIUF (elimina PeriodInNanoSeconds), desactiva el inventario de compatibilidad de aplicaciones/UAR, Buscar mi dispositivo, avisos de bienvenida/finalizar configuración, seguimiento de inicio de apps, Workplace Join y Asistencia rápida.",
            broken_features: &[
                "Invitaciones de Asistencia remota desactivadas",
                "Seguimiento de aplicaciones frecuentes/recientes en Inicio desactivado",
            ],
        }),
        "disable_typing_and_inking_telemetry" => Some(LocalizedTweakText {
            title: "Desactivar telemetría de entrada manuscrita, personalización de escritura e información de entrada",
            description: "Restringe la recopilación implícita de pulsaciones de teclas/lápiz, la obtención de contactos para diccionarios personalizados, el registro diagnóstico lingüístico (TIPC) y Windows Input Insights.",
            broken_features: &["Diccionario personalizado de predicción de escritura y entrada manuscrita desactivado"],
        }),
        "disable_search_cloud_and_history" => Some(LocalizedTweakText {
            title: "Desactivar aspectos destacados de búsqueda de Windows, búsqueda en la nube e historial local",
            description: "Desactiva los aspectos destacados de búsqueda (cuadro de búsqueda dinámico), la búsqueda de contenido en la nube MSA/AAD y el registro local del historial de búsqueda sin desactivar el servicio de indexación.",
            broken_features: &["Ilustración diaria de búsqueda destacada y resultados en la nube de OneDrive ocultos"],
        }),
        "disable_wifi_sense_and_location_extras" => Some(LocalizedTweakText {
            title: "Desactivar conexión automática WiFi-Sense, actualización de Mapas y ConsentStore de ubicación",
            description: "Desactiva la conexión automática a puntos de acceso Wi-Fi (WcmSvc\\AutoConnectAllowedOEM=0), las actualizaciones automáticas sin conexión de Mapas de Windows y el permiso global de ubicación ConsentStore.",
            broken_features: &["Descargas automáticas de Mapas de Windows y solicitudes de ubicación de apps en todo el sistema"],
        }),
        "block_onedrive_kfm_auto_backup" => Some(LocalizedTweakText {
            title: "Bloquear el movimiento automático de carpetas conocidas de OneDrive (Escritorio/Documentos)",
            description: "Establece KFMBlockOptIn=1 para que OneDrive no pueda redirigir silenciosamente las carpetas Escritorio, Documentos e Imágenes a la copia de seguridad en la nube. Seguro si solo deseas almacenamiento local o sincronización manual.",
            broken_features: &["Asistente automático de movimiento de carpetas conocidas de OneDrive desactivado"],
        }),
        "disable_bitlocker_auto_encryption" => Some(LocalizedTweakText {
            title: "Desactivar el cifrado automático de dispositivo BitLocker de Windows",
            description: "Establece PreventDeviceEncryption=1 para que Windows no cifre unidades silenciosamente al iniciar sesión con una cuenta de Microsoft o en instalaciones nuevas sin respaldar primero las claves de recuperación.",
            broken_features: &["El cifrado automático desatendido de unidades con BitLocker no se activará"],
        }),
        "disable_extra_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Desactivar tareas programadas adicionales de telemetría, cola WER, SQM, energía y Mapas",
            description: "Desactiva tareas programadas adicionales de diagnóstico y segundo plano: QueueReporting, Sqm-Tasks, MareBackup, StartupAppTask, PcaPatchDbTask, MapsUpdateTask, AnalyzeSystem y FamilySafetyMonitor.",
            broken_features: &["Informe automático de eficiencia energética y actualizaciones de mapas sin conexión desactivados"],
        }),
        "disable_windows_ai_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Desactivar tareas programadas en segundo plano de Windows AI Recall",
            description: "Desactiva las tareas programadas de configuración de Windows Recall y de canalización de aprendizaje automático en segundo plano introducidas en Windows 11 24H2+.",
            broken_features: &["Tareas de indexación en segundo plano de Windows Recall desactivadas"],
        }),
        "disable_office_actions_scheduled_task" => Some(LocalizedTweakText {
            title: "Desactivar la tarea del servidor de acciones de IA de Microsoft Office",
            description: "Desactiva la tarea programada en segundo plano Microsoft Office Actions Server para reducir las activaciones de CPU/memoria en segundo plano de la IA de Office.",
            broken_features: &[],
        }),
        "disable_optional_bloat_services" => Some(LocalizedTweakText {
            title: "Desactivar servicios opcionales de Windows sin uso (Mapas, Fax, Demo, Cartera, tzautoupdate)",
            description: "Desactiva MapsBroker, RetailDemo, Fax, WalletService, WMPNetworkSvc, AJRouter, shpamsvc y tzautoupdate sin tocar servicios esenciales del sistema ni de Windows Update.",
            broken_features: &["Fax de Windows, agente de Mapas sin conexión, Demo comercial, uso compartido WMP y zona horaria automática desactivados"],
        }),
        "disable_xbox_live_services" => Some(LocalizedTweakText {
            title: "Desactivar servicios en segundo plano de Xbox Live (XblAuthManager, GameSave, Networking)",
            description: "Desactiva los servicios en segundo plano Xbox Live Auth Manager, Game Save y Networking para usuarios que no juegan a títulos de Xbox App / Game Pass.",
            broken_features: &["El inicio de sesión de Xbox Live y guardados en la nube de Xbox Game Pass / Microsoft Store dejan de funcionar"],
        }),
        "disable_ai_fabric_service" => Some(LocalizedTweakText {
            title: "Desactivar el servicio Windows AI Fabric (WSAIFabricSvc)",
            description: "Desactiva el servicio en segundo plano de orquestación local de IA WSAIFabricSvc en Windows 11 24H2+.",
            broken_features: &["Servicio de orquestación de IA Copilot+ detenido"],
        }),
        "deny_global_camera_mic_diagnostics" => Some(LocalizedTweakText {
            title: "Bloquear el acceso global de aplicaciones a cámara, micrófono, datos de cuenta y diagnósticos [Opt-In]",
            description: "Deniega globalmente los permisos ConsentStore para cámara web, micrófono, userAccountInformation y appDiagnostics en todas las aplicaciones de escritorio y UWP. Alto riesgo de interrupción: desactiva cámaras y micrófonos en Zoom/Discord/OBS hasta revertir.",
            broken_features: &["Desactiva la cámara y el micrófono en TODAS las aplicaciones hasta volver a permitirlos"],
        }),

        // === bloatware_appx.toml (26) ===
        "remove_bing_weather" => Some(LocalizedTweakText {
            title: "Eliminar El Tiempo de Bing",
            description: "Elimina la aplicación UWP Microsoft Bing Weather. El clima se puede consultar desde el navegador.",
            broken_features: &["Se eliminará la aplicación El Tiempo de Bing"],
        }),
        "remove_bing_news" => Some(LocalizedTweakText {
            title: "Eliminar Noticias de Bing",
            description: "Elimina la aplicación UWP Microsoft Bing News (Microsoft News).",
            broken_features: &["Se eliminará la aplicación Noticias de Bing / Microsoft News"],
        }),
        "remove_get_help" => Some(LocalizedTweakText {
            title: "Eliminar Obtener ayuda",
            description: "Elimina la aplicación UWP Microsoft Obtener ayuda (Get Help). Los usuarios pueden seguir accediendo al soporte por la web.",
            broken_features: &["Se eliminará la aplicación Obtener ayuda"],
        }),
        "remove_getstarted_tips" => Some(LocalizedTweakText {
            title: "Eliminar Sugerencias (Primeros pasos)",
            description: "Elimina la aplicación UWP Microsoft Sugerencias / Primeros pasos.",
            broken_features: &["Se eliminará la aplicación Sugerencias / Primeros pasos"],
        }),
        "remove_solitaire" => Some(LocalizedTweakText {
            title: "Eliminar Microsoft Solitaire Collection",
            description: "Elimina la aplicación UWP Microsoft Solitaire Collection, que incluye anuncios y telemetría.",
            broken_features: &["Se eliminará Solitaire Collection"],
        }),
        "remove_people" => Some(LocalizedTweakText {
            title: "Eliminar la aplicación Contactos (People)",
            description: "Elimina la aplicación UWP Microsoft Contactos (People).",
            broken_features: &["Se eliminará la aplicación Contactos"],
        }),
        "remove_windows_maps" => Some(LocalizedTweakText {
            title: "Eliminar Mapas de Windows",
            description: "Elimina la aplicación UWP Mapas de Windows de Microsoft.",
            broken_features: &["Se eliminará la aplicación Mapas de Windows"],
        }),
        "remove_groove_music" => Some(LocalizedTweakText {
            title: "Eliminar Groove Música (Zune Music)",
            description: "Elimina la aplicación UWP Groove Música (ZuneMusic). Se puede usar el Reproductor multimedia o reproductores de terceros en su lugar.",
            broken_features: &["Se eliminará la aplicación Groove Música"],
        }),
        "remove_zune_video" => Some(LocalizedTweakText {
            title: "Eliminar Películas y TV (Zune Video)",
            description: "Elimina la aplicación UWP Películas y TV (ZuneVideo). Se puede usar VLC u otros reproductores en su lugar.",
            broken_features: &["Se eliminará la aplicación Películas y TV"],
        }),
        "remove_clipchamp" => Some(LocalizedTweakText {
            title: "Eliminar Clipchamp",
            description: "Elimina el editor de vídeo UWP Clipchamp preinstalado en Windows 11.",
            broken_features: &["Se eliminará el editor de vídeo Clipchamp"],
        }),
        "remove_todos" => Some(LocalizedTweakText {
            title: "Eliminar Microsoft To Do",
            description: "Elimina la aplicación UWP Microsoft To Do.",
            broken_features: &["Se eliminará la aplicación Microsoft To Do"],
        }),
        "remove_bing_search_appx" => Some(LocalizedTweakText {
            title: "Eliminar la extensión AppX de búsqueda de Microsoft Bing",
            description: "Elimina el paquete de Store Microsoft.BingSearch que impulsa los widgets web y las integraciones web de Búsqueda de Windows.",
            broken_features: &["Integración AppX de Búsqueda de Bing eliminada"],
        }),
        "remove_feedback_hub" => Some(LocalizedTweakText {
            title: "Eliminar Centro de opiniones de Microsoft",
            description: "Elimina la aplicación preinstalada de telemetría y encuestas Centro de opiniones de Windows.",
            broken_features: &["Aplicación Centro de opiniones eliminada"],
        }),
        "remove_copilot_appx" => Some(LocalizedTweakText {
            title: "Eliminar paquete UWP de Microsoft Copilot",
            description: "Desinstala el paquete independiente de Microsoft Store Microsoft.Copilot.",
            broken_features: &["Aplicación Copilot eliminada"],
        }),
        "remove_edge_game_assist" => Some(LocalizedTweakText {
            title: "Eliminar la superposición Microsoft Edge Game Assist",
            description: "Elimina el paquete AppX de superposición de navegador en juegos Edge Game Assist.",
            broken_features: &["Widget Edge Game Assist eliminado"],
        }),
        "remove_dev_home" => Some(LocalizedTweakText {
            title: "Eliminar la aplicación Windows Dev Home",
            description: "Elimina el paquete de panel Microsoft.Windows.DevHome preinstalado en Windows 11.",
            broken_features: &["Panel Dev Home eliminado"],
        }),
        "remove_power_automate_desktop" => Some(LocalizedTweakText {
            title: "Eliminar Microsoft Power Automate Desktop",
            description: "Elimina el paquete RPA Microsoft Power Automate Desktop.",
            broken_features: &["Aplicación de flujos de escritorio Power Automate eliminada"],
        }),
        "remove_office_hub" => Some(LocalizedTweakText {
            title: "Eliminar Microsoft 365 Copilot (OfficeHub)",
            description: "Elimina la aplicación promocional preinstalada Microsoft 365 Office Hub.",
            broken_features: &["Iniciador promocional de Office eliminado (no desinstala Office de escritorio)"],
        }),
        "remove_family_safety" => Some(LocalizedTweakText {
            title: "Eliminar la aplicación Microsoft Family Safety",
            description: "Elimina la aplicación UWP de consumo Microsoft Family Safety.",
            broken_features: &["Aplicación UWP Family Safety eliminada"],
        }),
        "remove_3d_viewer_and_mixed_reality" => Some(LocalizedTweakText {
            title: "Eliminar Visor 3D y Portal de realidad mixta",
            description: "Elimina los paquetes heredados Microsoft Visor 3D y Portal de realidad mixta.",
            broken_features: &["Visor 3D heredado y Portal de realidad mixta de Windows eliminados"],
        }),
        "remove_outlook_for_windows" => Some(LocalizedTweakText {
            title: "Eliminar el nuevo Outlook para Windows (cliente WebView)",
            description: "Elimina la aplicación envoltorio web preinstalada Microsoft.OutlookForWindows.",
            broken_features: &["Nuevo cliente UWP de Outlook eliminado"],
        }),
        "remove_your_phone_link" => Some(LocalizedTweakText {
            title: "Eliminar Enlace móvil (YourPhone)",
            description: "Desinstala la aplicación complementaria de sincronización móvil Microsoft.YourPhone.",
            broken_features: &["Sincronización de escritorio Android/iOS de Enlace móvil eliminada"],
        }),
        "remove_sound_recorder_and_sticky_notes" => Some(LocalizedTweakText {
            title: "Eliminar Grabadora de sonidos y Notas rápidas de Windows",
            description: "Elimina los paquetes de Store de Grabadora de sonidos y Notas rápidas (Sticky Notes).",
            broken_features: &["Aplicaciones integradas Grabadora de sonidos y Notas rápidas eliminadas"],
        }),
        "remove_ms_teams_and_quick_assist" => Some(LocalizedTweakText {
            title: "Eliminar Microsoft Teams personal y Asistencia rápida",
            description: "Elimina la aplicación personal de chat MSTeams y la herramienta de soporte Asistencia rápida.",
            broken_features: &["Teams personal y herramienta de soporte Asistencia rápida eliminados"],
        }),
        "disable_recall_optional_feature" => Some(LocalizedTweakText {
            title: "Desinstalar la característica opcional Windows Recall (DISM)",
            description: "Desactiva de forma limpia el paquete de característica opcional 'Recall' de Windows a nivel de mantenimiento del sistema operativo.",
            broken_features: &["Binarios de la característica Windows Recall desactivados de forma limpia"],
        }),
        "remove_legacy_steps_recorder_ie_wordpad" => Some(LocalizedTweakText {
            title: "Desactivar Grabación de acciones heredada y capacidad Internet Explorer 11",
            description: "Elimina la herramienta heredada Grabación de acciones (App.StepsRecorder) y la capacidad de modo Internet Explorer mediante DISM.",
            broken_features: &["Grabación de acciones (psr.exe) y modo IE11 eliminados"],
        }),

        // === hardware_tuning.toml (16) ===
        "enable_hags" => Some(LocalizedTweakText {
            title: "Activar programación de GPU acelerada por hardware (HAGS)",
            description: "Activa la programación de GPU acelerada por hardware, permitiendo que la GPU gestione directamente su propia planificación de VRAM. Reduce la latencia y la sobrecarga de CPU en cargas limitadas por GPU. Requiere una GPU compatible (NVIDIA GTX 1000+, AMD RX 5000+) y soporte de controlador.",
            broken_features: &["Algunos juegos antiguos o herramientas de GPU pueden mostrar inestabilidad"],
        }),
        "disable_mpo" => Some(LocalizedTweakText {
            title: "Desactivar superposición multiplano (MPO)",
            description: "Desactiva la función de composición por hardware Multi-Plane Overlay. MPO es conocido por provocar tirones, pantallas negras y fallos de ritmo de fotogramas en muchas combinaciones de GPU/controlador, sobre todo en configuraciones multimonitor con distintas frecuencias de refresco.",
            broken_features: &["Consumo energético del compositor GPU ligeramente mayor en portátiles"],
        }),
        "disable_core_parking" => Some(LocalizedTweakText {
            title: "Desactivar el aparcamiento de núcleos de CPU (Core Parking)",
            description: "Fija al 100 % el porcentaje mínimo de núcleos del procesador que deben permanecer activos para que todos los núcleos estén siempre disponibles. Elimina la latencia de activación de núcleos a costa de mayor consumo en reposo.",
            broken_features: &[
                "Mayor consumo de energía en reposo",
                "Menor duración de batería en portátiles",
            ],
        }),
        "disable_cstates" => Some(LocalizedTweakText {
            title: "Desactivar estados de inactividad C-States de la CPU",
            description: "Impide que el procesador entre en estados profundos de reposo C-States (C6/C7/C8) estableciendo la directiva de inactividad en modo rendimiento. Reduce picos de latencia al cambiar de estado de energía a costa del consumo en reposo.",
            broken_features: &[
                "Consumo de energía en reposo significativamente mayor",
                "Menor duración de batería en portátiles",
                "Temperaturas de CPU más altas en reposo",
            ],
        }),
        "gpu_shader_cache_size" => Some(LocalizedTweakText {
            title: "Ampliar tiempo de espera TDR de GPU y evitar reinicios del controlador",
            description: "Aumenta el intervalo WDDM Timeout Detection and Recovery (TdrDelay) de 2s a 10s, evitando reinicios falsos del controlador gráfico y tirones durante compilaciones pesadas de sombreadores DirectX.",
            broken_features: &["La recuperación del controlador de pantalla espera hasta 10s si ocurre un bloqueo real de GPU"],
        }),
        "enable_game_mode" => Some(LocalizedTweakText {
            title: "Activar Modo de juego de Windows",
            description: "Garantiza que el Modo de juego de Windows esté activado (AutoGameModeEnabled=1, AllowAutoGameMode=1), útil si fue desactivado previamente, priorizando procesos de juegos y suprimiendo notificaciones de Windows Update y reinicios durante las partidas.",
            broken_features: &[],
        }),
        "disable_fullscreen_optimizations" => Some(LocalizedTweakText {
            title: "Desactivar optimizaciones de pantalla completa globalmente",
            description: "Desactiva globalmente las optimizaciones de pantalla completa de Windows (híbrido sin bordes) para todas las aplicaciones. Esto fuerza el modo de pantalla completa exclusivo real, lo que puede reducir la latencia de entrada y la varianza del tiempo de fotograma en juegos antiguos.",
            broken_features: &[
                "Alt-Tab desde juegos en pantalla completa será más lento",
                "La superposición de Game Bar no funcionará en pantalla completa exclusiva",
            ],
        }),
        "4433ce93-b632-4d67-ae71-0241c31099c0" => Some(LocalizedTweakText {
            title: "Desactivar AMD ULPS",
            description: "Desactiva Ultra Low Power State (ULPS) en la pila de controladores AMD Radeon. Evita que núcleos GPU secundarios o estados de suspensión despierten con micro-tirones y picos de latencia en juegos 3D.",
            broken_features: &["Consumo en reposo ligeramente mayor en portátiles AMD multi-GPU / Crossfire"],
        }),
        "8a4b4c4d-000e-42db-8bd7-293067f8fae7" => Some(LocalizedTweakText {
            title: "Desactivar AMD ASPM",
            description: "Desactiva la gestión activa de energía PCIe (ASPM) para tarjetas gráficas AMD. Impide que el enlace PCIe pase a estados de bajo consumo L0s/L1 entre fotogramas, reduciendo la fluctuación de transmisión PCIe.",
            broken_features: &["Mayor consumo energético del bus PCIe en reposo"],
        }),
        "a3836587-c461-4beb-9c4c-88493b36c138" => Some(LocalizedTweakText {
            title: "Desactivar AMD Power Gating",
            description: "Desactiva el Power Gating dinámico agresivo en unidades de cómputo y motores de renderizado AMD (DisablePowerGating=1, PP_GPUPowerDownEnabled=0). Mantiene frecuencias de reloj continuas sin estrangulamiento.",
            broken_features: &["Temperaturas del chip GPU más altas con baja carga"],
        }),
        "db379bb5-fc8c-4ad3-be32-11be09e76fba" => Some(LocalizedTweakText {
            title: "Desactivar AMD Video Clock Gating",
            description: "Evita la desconexión de reloj y energía en los motores multimedia y de pantalla por hardware AMD VCE/UVD (DisableVCEPowerGating=1, DisableVceClockGating=1, EnableUvdClockGating=0, EnableVceSwClockGating=0). Elimina picos de latencia DPC.",
            broken_features: &[],
        }),
        "df8739ff-e322-4958-b369-df584bb9b0c1" => Some(LocalizedTweakText {
            title: "Desactivar Intel Async Flips",
            description: "Establece Display1_DisableAsyncFlips=1 en las claves de clase del adaptador gráfico Intel para imponer cambios de pantalla sincronizados y eliminar el jitter de la cola de presentación.",
            broken_features: &[],
        }),
        "ebfb0ded-5d1a-4cd1-b5c0-4f7018dd6054" => Some(LocalizedTweakText {
            title: "Desactivar Intel Adaptive V-Sync",
            description: "Establece AdaptiveVsyncEnable=0 en las claves de clase del adaptador gráfico Intel para evitar el recorte adaptativo de refresco cuando los FPS caen por debajo de la tasa del monitor.",
            broken_features: &[],
        }),
        "fa58802f-b98c-4e98-aa67-067c5a394296" => Some(LocalizedTweakText {
            title: "Desactivar NVIDIA Dynamic P-State",
            description: "Establece DisableDynamicPstate=1 para fijar la GPU NVIDIA en un estado de rendimiento constante y evitar fluctuaciones de frecuencia. Aumenta el consumo en reposo.",
            broken_features: &["Aumenta el consumo de energía y el calor de la GPU en reposo"],
        }),
        "9e7702be-9e03-4c96-a316-c4e435cb993e" => Some(LocalizedTweakText {
            title: "Desactivar NVIDIA Async P-States",
            description: "Establece DisableAsyncPstates=1 para evitar transiciones asíncronas de estado de energía en NVIDIA, brindando rendimiento determinista en VR y aplicaciones en tiempo real.",
            broken_features: &["Puede impedir que la memoria de la GPU reduzca su reloj durante la reproducción de vídeo"],
        }),
        "5f1a2b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c" => Some(LocalizedTweakText {
            title: "Activar modo de rendimiento máximo PowerMizer en NVIDIA",
            description: "Fuerza el modo de máximo rendimiento PowerMizer conectado a corriente alterna, fijando el bus PCIe y los controladores de memoria a máxima velocidad sin reducción.",
            broken_features: &["Temperaturas de GPU y velocidad de ventiladores en reposo más elevadas"],
        }),

        // === os_settings.toml (21) ===
        "disable_animations" => Some(LocalizedTweakText {
            title: "Desactivar animaciones de ventanas",
            description: "Desactiva el efecto visual al minimizar y maximizar ventanas, reduciendo la sensación de retardo en la interfaz.",
            broken_features: &[],
        }),
        "disable_transparency" => Some(LocalizedTweakText {
            title: "Desactivar efectos de transparencia",
            description: "Desactiva el efecto acrílico/transparencia en la barra de tareas, el menú Inicio y el Centro de actividades, liberando carga de composición de GPU.",
            broken_features: &["Apariencia transparente de la barra de tareas y del menú Inicio"],
        }),
        "disable_ntfs_last_access" => Some(LocalizedTweakText {
            title: "Desactivar aplicaciones en segundo plano de Windows globalmente",
            description: "Impide que las aplicaciones UWP / Microsoft Store se reactiven, consulten redes y consuman CPU/RAM silenciosamente en segundo plano al estar minimizadas o cerradas.",
            broken_features: &["Las apps de Store dejan de sincronizarse en segundo plano cuando están cerradas"],
        }),
        "disable_nagle_algorithm" => Some(LocalizedTweakText {
            title: "Desactivar algoritmo de Nagle para MSMQ (TCPNoDelay)",
            description: "Desactiva el algoritmo de Nagle específicamente para Microsoft Message Queuing (MSMQ\\Parameters\\TCPNoDelay), reduciendo la latencia de cola en cargas de mensajes pequeños MSMQ.",
            broken_features: &["Puede aumentar ligeramente el uso de ancho de banda de red para paquetes pequeños"],
        }),
        "disable_sticky_keys" => Some(LocalizedTweakText {
            title: "Desactivar Teclas especiales y atajos de accesibilidad",
            description: "Desactiva el cuadro de diálogo de Teclas especiales (5x Shift), Teclas filtro y pitidos de Teclas de alternancia, evitando ventanas emergentes molestas e interrupciones en juegos competitivos.",
            broken_features: &["Atajos de accesibilidad (5x Shift, 8s Shift derecho) desactivados"],
        }),
        "disable_mouse_shadow" => Some(LocalizedTweakText {
            title: "Desactivar sombras del Explorador y transparencia alfa de selección",
            description: "Desactiva las sombras bajo etiquetas de iconos del escritorio y la mezcla alfa translúcida del rectángulo de selección en el Explorador, reduciendo el trabajo de composición GPU del DWM.",
            broken_features: &[],
        }),
        "high_performance_power_plan" => Some(LocalizedTweakText {
            title: "Establecer modo agresivo de impulso de rendimiento del procesador",
            description: "Configura el modo de impulso de rendimiento del procesador con corriente alterna en Agresivo mediante PowerSchemes PerfBoostMode, evitando caídas de frecuencia turbo bajo carga interactiva.",
            broken_features: &[
                "Mayor consumo de energía en reposo",
                "Mayor generación de calor en portátiles",
            ],
        }),
        "disable_bing_search_start" => Some(LocalizedTweakText {
            title: "Desactivar búsqueda web de Bing en el menú Inicio",
            description: "Impide que Búsqueda de Windows envíe cada pulsación a los servidores de Bing, logrando que la búsqueda local de aplicaciones y archivos en el menú Inicio sea instantánea y sin retraso de red.",
            broken_features: &["No aparecerán resultados de búsqueda web en el menú Inicio"],
        }),
        "disable_windows_copilot" => Some(LocalizedTweakText {
            title: "Desactivar integración de IA Windows Copilot",
            description: "Desactiva Windows Copilot en todo el sistema mediante Directiva de grupo, deteniendo agentes residentes en segundo plano, botones de barra de tareas y consultas de IA en red.",
            broken_features: &["La barra lateral de Windows Copilot y el atajo (Win+C) quedarán desactivados"],
        }),
        "disable_game_bar_presence_writer" => Some(LocalizedTweakText {
            title: "Desactivar presencia de Game Bar, iniciador Nexus y sugerencias iniciales",
            description: "Desactiva el lanzamiento de Game Bar con el botón Nexus del mando y los paneles de inicio mediante claves HKCU\\Software\\Microsoft\\GameBar editables por el usuario, eliminando hooks sin errores de ACL TrustedInstaller.",
            broken_features: &["Inicio con el botón Xbox Guide Nexus y panel inicial de Game Bar desactivados"],
        }),
        "disable_widgets_news" => Some(LocalizedTweakText {
            title: "Desactivar Widgets de Windows y noticias de MSN",
            description: "Desactiva el icono de Widgets en la barra de tareas e impide la descarga en segundo plano de noticias MSN, temas de tendencia y widgets meteorológicos.",
            broken_features: &["El panel desplegable de Widgets en la barra de tareas quedará desactivado"],
        }),
        "gamedvr_config_store" => Some(LocalizedTweakText {
            title: "Desactivar superposición de grabación Xbox GameDVR",
            description: "Desactiva la grabación de vídeo de partidas en segundo plano y la inyección de hooks de Xbox Game Bar, eliminando caídas de frametime y micro-tirones.",
            broken_features: &["La grabación instantánea de repeticiones en segundo plano de Xbox Game Bar quedará desactivada"],
        }),
        "gamedvr_app_capture" => Some(LocalizedTweakText {
            title: "Desactivar captura de aplicaciones de Windows GameDVR",
            description: "Desactiva los búferes de grabación de audio/vídeo en segundo plano dentro del subsistema Windows GameDVR.",
            broken_features: &[],
        }),
        "gamedvr_policy_disable" => Some(LocalizedTweakText {
            title: "Desactivar directiva del sistema GameDVR",
            description: "Bloqueo por Directiva de grupo en todo el sistema que impide a Windows volver a activar los servicios de captura GameDVR en segundo plano.",
            broken_features: &[],
        }),
        "consumer_features_disable" => Some(LocalizedTweakText {
            title: "Desactivar funciones en la nube para el consumidor de Windows",
            description: "Impide que Windows descargue e instale automáticamente aplicaciones y juegos patrocinados de terceros (p. ej. TikTok, Disney, Candy Crush) durante el mantenimiento.",
            broken_features: &[],
        }),
        "disable_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Desactivar tareas de telemetría de Comentarios, Autochk y Mapas",
            description: "Desactiva avisos diagnósticos SIUF del Centro de opiniones de Windows, el proxy de telemetría Autochk y tareas de notificaciones de actualización de Mapas en segundo plano (adaptado de Sophia Script).",
            broken_features: &["Las notificaciones proactivas de encuestas del Centro de opiniones quedarán desactivadas"],
        }),
        "disable_hpet_synthetic_timer" => Some(LocalizedTweakText {
            title: "Desactivar Dynamic Tick (temporizador sintético del núcleo)",
            description: "Desactiva el ahorro de energía Dynamic Tick en el núcleo Windows NT, evitando desviaciones del reloj sintético y forzando consultas fiables de alta frecuencia Invariant TSC para latencia DPC ultrabaja.",
            broken_features: &["Consumo energético de la CPU en reposo ligeramente mayor"],
        }),
        "optimize_tcp_congestion_tuning" => Some(LocalizedTweakText {
            title: "Optimizar congestión TCP de red y ajuste automático de ventana",
            description: "Activa marcas de tiempo TCP de alto rendimiento RFC 1323 y desactiva la difusión Web Services Dynamic Discovery (WSD) para minimizar la latencia en juegos en línea y el retardo de serialización de paquetes.",
            broken_features: &[],
        }),
        "disable_windows_error_reporting" => Some(LocalizedTweakText {
            title: "Desactivar directiva y envíos de Informes de errores de Windows",
            description: "Aplica el bloqueo de Directiva de grupo de Informes de errores de Windows y desactiva el envío adicional de volcados de fallos a servidores de Microsoft.",
            broken_features: &["Los informes automáticos de fallos y cargas de telemetría quedan bloqueados"],
        }),
        "disable_bing_search_start_menu" => Some(LocalizedTweakText {
            title: "Desactivar recomendaciones del menú Inicio y anuncios Iris",
            description: "Elimina recomendaciones patrocinadas de sitios web/cuentas, consejos Iris y elementos de aplicaciones agregadas recientemente en los diseños de Inicio de Windows 10/11.",
            broken_features: &["Las recomendaciones promocionales del menú Inicio quedan ocultas"],
        }),
        "disable_cortana_autostart" => Some(LocalizedTweakText {
            title: "Desactivar búsqueda de Cortana por usuario y botón de barra de tareas",
            description: "Desactiva la activación de Cortana por usuario, los permisos de ubicación de búsqueda y oculta el acceso de Cortana en la barra de tareas.",
            broken_features: &["La integración de búsqueda de Cortana en la barra de tareas por usuario quedará desactivada"],
        }),
        // === perf_power_ext.toml (20) ===
        "enable_long_file_paths" => Some(LocalizedTweakText {
            title: "Activar rutas de archivo largas Win32 NTFS (>260 caracteres)",
            description: "Elimina la antigua limitación MAX_PATH de 260 caracteres en Windows 10/11 mediante LongPathsEnabled=1, evitando errores de truncado de rutas al compilar o extraer archivos.",
            broken_features: &[],
        }),
        "disable_background_search_toggle_and_app_policy" => Some(LocalizedTweakText {
            title: "Denegar ejecución de aplicaciones en segundo plano y opción de Búsqueda en segundo plano",
            description: "Establece AppPrivacy\\LetAppsRunInBackground=2 (Denegación forzada) y desactiva Search\\BackgroundAppGlobalToggle=0 para que las apps UWP/Store inactivas no consuman CPU ni RAM en segundo plano.",
            broken_features: &["Las apps UWP como Store/Correo no se sincronizarán mientras estén cerradas"],
        }),
        "extend_webclient_file_size_limit" => Some(LocalizedTweakText {
            title: "Maximizar límite de tamaño de archivo WebClient / WebDAV (4 GB)",
            description: "Eleva el límite de descarga del servicio Windows WebClient de 50 MB al máximo de 4 GB (0xFFFFFFFF), corrigiendo fallos al copiar en recursos compartidos WebDAV o de red montados.",
            broken_features: &[],
        }),
        "reduce_services_pipe_timeout" => Some(LocalizedTweakText {
            title: "Reducir tiempo de espera de inicio y cierre de canalización de servicios a 15s",
            description: "Reduce ServicesPipeTimeout del valor predeterminado de 30000 ms (30s) a 15000 ms (15s) para que los servicios bloqueados en segundo plano se liberen antes al arrancar y apagar.",
            broken_features: &["Servicios de bases de datos personalizados muy lentos que tarden >15s al arrancar pueden agotar el tiempo de espera"],
        }),
        "mmcss_scheduler_full_optimization" => Some(LocalizedTweakText {
            title: "Optimización completa del programador multimedia MMCSS (NoLazyMode, AlwaysOn, SFIO alto)",
            description: "Configura Multimedia\\SystemProfile NoLazyMode=1, AlwaysOn=1, junto con Games Tasks Scheduling Category='High' y SFIO Priority='High' para una programación ininterrumpida de hilos multimedia y de juegos.",
            broken_features: &[],
        }),
        "enable_directx_windowed_game_optimizations" => Some(LocalizedTweakText {
            title: "Activar optimizaciones DirectX Flip-Model y VRR en modo ventana (fusión sin pérdida)",
            description: "Combina de forma segura SwapEffectUpgradeEnable=1 y VRROptimizeEnable=1 dentro de DirectXUserGlobalSettings sin sobrescribir tus preferencias de GPU o AutoHDR por juego.",
            broken_features: &[],
        }),
        "disable_hardware_overlays_and_min_fps" => Some(LocalizedTweakText {
            title: "Desactivar superposiciones de hardware del núcleo y limitación FPS mínima de DWM",
            description: "Establece GraphicsDrivers\\DisableOverlays=1 y Dwm\\OverlayMinFPS=0 para evitar bloqueos al promover planos de superposición de hardware DWM y tirones a bajas tasas de refresco.",
            broken_features: &["Consumo eléctrico del compositor DWM en reposo ligeramente mayor en portátiles"],
        }),
        "nvidia_restore_legacy_sharpening" => Some(LocalizedTweakText {
            title: "Restaurar directiva clásica de nitidez del controlador NVIDIA (EnableGR535=0)",
            description: "Establece Global\\FTS\\EnableGR535=0 para que el Panel de control de NVIDIA muestre la Nitidez de imagen clásica de baja carga en lugar de forzar la ruta del filtro de superposición más reciente.",
            broken_features: &[],
        }),
        "nvidia_dlss_debug_indicator" => Some(LocalizedTweakText {
            title: "Mostrar marca de agua de depuración de versión y perfil NVIDIA DLSS",
            description: "Establece Global\\NGXCore\\ShowDlssIndicator=1024 para mostrar en pantalla la versión activa del SDK de DLSS, el escalado de resolución y el preajuste en juegos DirectX/Vulkan.",
            broken_features: &["Muestra una marca de agua de diagnóstico DLSS abajo a la izquierda en juegos compatibles"],
        }),
        "disable_game_bar_tips_and_capture" => Some(LocalizedTweakText {
            title: "Desactivar Nexus del mando, consejos de inicio y captura histórica en Game Bar",
            description: "Desactiva UseNexusForGameBarEnabled (iniciador con el botón Guía Xbox), los avisos ShowStartupPanel y la grabación en búfer circular en segundo plano HistoricalCaptureEnabled.",
            broken_features: &["El botón Nexus del mando ya no activa la superposición de Game Bar"],
        }),
        "optimize_mouse_thresholds_and_hover" => Some(LocalizedTweakText {
            title: "Poner a cero los umbrales de aceleración del ratón y respuesta instantánea al señalar de 10ms",
            description: "Establece MouseThreshold1='0', MouseThreshold2='0' y MouseHoverTime='10' ms para que los umbrales de aceleración del puntero queden completamente planos y la vista previa al pasar el cursor reaccione al instante.",
            broken_features: &[],
        }),
        "disable_extra_accessibility_hotkeys" => Some(LocalizedTweakText {
            title: "Desactivar atajos accidentales de Teclas de ratón, Contraste alto y Narrador",
            description: "Desactiva las ventanas emergentes de Alt+Shift+BloqNum (Teclas de ratón), Alt+Shift+ImprPant (Contraste alto) y Win+Ctrl+Entrar (Narrador) durante las partidas.",
            broken_features: &["Los atajos de teclado de accesibilidad deben activarse desde la aplicación Configuración"],
        }),
        "disable_smart_multi_homed_name_resolution" => Some(LocalizedTweakText {
            title: "Desactivar resolución inteligente de nombres DNS multired de Windows",
            description: "Establece DNSClient\\DisableSmartNameResolution=1 para que Windows consulte el DNS estrictamente en la interfaz principal en lugar de difundir solicitudes paralelas en adaptadores VPN/virtuales.",
            broken_features: &["Las VPN de túnel dividido (split-tunnel) deben configurar correctamente las métricas DNS del adaptador"],
        }),
        "harden_windows_update_ux_and_store" => Some(LocalizedTweakText {
            title: "Bloquear sobrescritura de controladores en WU, reinicios inesperados y descargas de Store",
            description: "Excluye envíos de controladores de GPU/periféricos en actualizaciones de calidad, impide reinicios forzados con sesión iniciada, desbloquea una pausa extendida (3650 días) y desactiva descargas silenciosas de Store manteniendo las actualizaciones de seguridad.",
            broken_features: &["Las actualizaciones de aplicaciones de Microsoft Store deben buscarse manualmente"],
        }),
        "power_settings_pcie_usb_nvme_epb" => Some(LocalizedTweakText {
            title: "Ajuste de latencia ultrabaja del plan de energía activo (PCIe ASPM, suspensión USB, NVMe, EPP, PowerThrottling)",
            description: "Usa las API nativas de Windows powrprof sobre el esquema de energía activo (funciona en Equilibrado, Alto rendimiento y planes personalizados) para desactivar con corriente alterna PCIe ASPM, suspensión selectiva USB, ahorro de enlace USB 3, reposo NVMe y EPP de CPU (EPP=0), apagando además PowerThrottling del núcleo.",
            broken_features: &["Mayor consumo energético en reposo conectado a corriente en equipos de escritorio y portátiles enchufados"],
        }),
        "disable_storage_sense_policy" => Some(LocalizedTweakText {
            title: "Desactivar limpiezas automáticas en segundo plano del Sensor de almacenamiento",
            description: "Impide que el Sensor de almacenamiento de Windows ejecute análisis automáticos de disco en segundo plano durante sesiones interactivas.",
            broken_features: &["Los archivos temporales y la Papelera deben limpiarse manualmente desde Limpieza y Apps"],
        }),
        "consolidate_svchost_split_threshold" => Some(LocalizedTweakText {
            title: "Consolidar hosts de servicio dinámicamente según la RAM física (SvcHostSplitThresholdInKB) [Opt-In]",
            description: "Calcula dinámicamente SvcHostSplitThresholdInKB a partir de la RAM del sistema detectada (TotalRamInKb), reduciendo el número de procesos y la sobrecarga de memoria virtual en equipos de 8 GB+. Precaución: reduce el aislamiento de procesos por servicio.",
            broken_features: &["Aislamiento de fallos en svchost compartido reducido; el Administrador de tareas muestra menos servicios agrupados"],
        }),
        "disable_vbs_deviceguard" => Some(LocalizedTweakText {
            title: "Desactivar Seguridad basada en virtualización (VBS) [Compromiso de seguridad Opt-In]",
            description: "Desactiva EnableVirtualizationBasedSecurity y RequirePlatformSecurityFeatures. Puede aumentar los FPS entre un 5 y un 15 % en CPUs con sobrecarga de virtualización, a costa de reducir los límites de seguridad de virtualización del núcleo.",
            broken_features: &["Desactiva el aislamiento de seguridad VBS; puede aparecer advertencia de antitrampas / Seguridad de Windows"],
        }),
        "disable_hvci_memory_integrity" => Some(LocalizedTweakText {
            title: "Desactivar Aislamiento del núcleo / Integridad de memoria (HVCI) [Compromiso de seguridad Opt-In]",
            description: "Desactiva la Integridad de código protegida por hipervisor (HVCI). Reduce la latencia de DPC/interrupciones en juegos competitivos donde HVCI afecta al tiempo de fotograma, a costa de la exigencia de firma de controladores del núcleo.",
            broken_features: &["Seguridad de Windows muestra la advertencia de Aislamiento del núcleo / Integridad de memoria desactivada"],
        }),
        "disable_prefetch_and_sysmain" => Some(LocalizedTweakText {
            title: "Desactivar Prefetcher del núcleo y SysMain (Superfetch) para SSD NVMe rápidos [Opt-In]",
            description: "Desactiva el Prefetcher de aplicaciones/arranque y detiene la precarga en memoria de SysMain en unidades NVMe rápidas donde el almacenamiento en caché RAM añade E/S en segundo plano innecesaria.",
            broken_features: &["La velocidad de arranque en frío puede reducirse ligeramente en discos duros mecánicos SATA"],
        }),

        // === privacy.toml (12) ===
        "disable_advertising_id" => Some(LocalizedTweakText {
            title: "Desactivar identificador de publicidad",
            description: "Desactiva el identificador de publicidad por usuario que Windows proporciona a las aplicaciones para publicidad dirigida y seguimiento entre aplicaciones.",
            broken_features: &["Las aplicaciones no recibirán un identificador de publicidad único"],
        }),
        "disable_cortana" => Some(LocalizedTweakText {
            title: "Desactivar Cortana",
            description: "Desactiva Cortana mediante Directiva de grupo, impidiendo que se ejecute, escuche y envíe datos a los servicios en la nube de Microsoft.",
            broken_features: &["El asistente de voz Cortana quedará desactivado"],
        }),
        "disable_activity_history" => Some(LocalizedTweakText {
            title: "Desactivar Historial de actividades / Línea de tiempo",
            description: "Desactiva la función Historial de actividades de Windows (Línea de tiempo), impidiendo que Windows recopile y almacene un registro de tus actividades, sitios web visitados y documentos abiertos.",
            broken_features: &["La Línea de tiempo/Historial de actividades en Vista de tareas estará vacía"],
        }),
        "disable_online_speech_recognition" => Some(LocalizedTweakText {
            title: "Desactivar reconocimiento de voz en línea",
            description: "Desactiva el servicio de reconocimiento de voz basado en la nube. Los datos de voz no se enviarán a los servidores de Microsoft para su procesamiento.",
            broken_features: &["El reconocimiento de voz en la nube no funcionará; el reconocimiento local/sin conexión no se ve afectado"],
        }),
        "disable_tailored_experiences" => Some(LocalizedTweakText {
            title: "Desactivar experiencias personalizadas",
            description: "Impide que Microsoft utilice los datos de diagnóstico para ofrecer sugerencias, anuncios y recomendaciones personalizadas en Windows.",
            broken_features: &["Se desactivarán las sugerencias y consejos personalizados de Windows"],
        }),
        "disable_windows_recall" => Some(LocalizedTweakText {
            title: "Desactivar Windows Recall (instantánea de IA)",
            description: "Desactiva la función Windows Recall introducida en Windows 11 24H2, que toma periódicamente capturas de pantalla indexadas por IA de tu actividad. Requiere compilación 26100+.",
            broken_features: &["La función de instantáneas de IA Windows Recall quedará desactivada"],
        }),
        "disable_cloud_content_spotlight" => Some(LocalizedTweakText {
            title: "Desactivar contenido en la nube y sugerencias de Contenido destacado",
            description: "Desactiva Contenido destacado de Windows (Spotlight), contenido sugerido en Configuración y sugerencias de aplicaciones preinstaladas entregadas desde la nube.",
            broken_features: &[
                "Las imágenes de Contenido destacado en la pantalla de bloqueo quedarán desactivadas",
                "No aparecerán aplicaciones sugeridas en el menú Inicio",
            ],
        }),
        "disable_location_tracking" => Some(LocalizedTweakText {
            title: "Desactivar seguimiento de ubicación",
            description: "Desactiva globalmente el servicio de ubicación de Windows, impidiendo que las aplicaciones y el sistema operativo accedan a tu ubicación geográfica.",
            broken_features: &["Las aplicaciones que requieren ubicación (Mapas, Tiempo) no tendrán acceso a tu posición"],
        }),
        "disable_activity_cloud_sync" => Some(LocalizedTweakText {
            title: "Desactivar Portapapeles en la nube y sincronización entre dispositivos",
            description: "Desactiva la persistencia del Historial del portapapeles de Windows y la sincronización en la nube entre dispositivos mediante Directiva de grupo para que contraseñas copiadas y datos sensibles permanezcan estrictamente locales.",
            broken_features: &["El Historial del portapapeles Win+V y la sincronización de pegado entre dispositivos quedarán desactivados"],
        }),
        "disable_cdp_service" => Some(LocalizedTweakText {
            title: "Desactivar servicio de Plataforma de dispositivos conectados",
            description: "Desactiva el servicio de Plataforma de dispositivos conectados (CDPUserSvc), impidiendo telemetría entre dispositivos, difusiones de sincronización de aplicaciones y balizas de actividad en segundo plano.",
            broken_features: &["Compartir en proximidad y la sincronización entre dispositivos con móviles quedarán desactivados"],
        }),
        "spotlight_desktop_disable" => Some(LocalizedTweakText {
            title: "Desactivar descargas de Contenido destacado en la pantalla de bloqueo",
            description: "Desactiva la obtención automática en segundo plano de fondos de pantalla de bloqueo de Windows Spotlight e imágenes promocionales de Microsoft.",
            broken_features: &[],
        }),
        "spotlight_features_disable" => Some(LocalizedTweakText {
            title: "Desactivar consejos y anuncios en la pantalla de bloqueo",
            description: "Desactiva consejos, sugerencias y promociones patrocinadas de Microsoft Store en la pantalla de bloqueo distribuidas mediante Content Delivery Manager.",
            broken_features: &[],
        }),

        // === registry_lag.toml (18) ===
        "win32_priority_separation" => Some(LocalizedTweakText {
            title: "Win32PrioritySeparation — Prioridad de primer plano",
            description: "Configura el cuanto del planificador como corto, variable y con alto impulso de primer plano (0x26 = 38). Prioriza las aplicaciones activas para lograr menor latencia de entrada en juegos y cargas interactivas.",
            broken_features: &[],
        }),
        "menu_show_delay" => Some(LocalizedTweakText {
            title: "Reducir el retardo al mostrar menús a 0 ms",
            description: "Elimina el retardo artificial antes de que aparezcan los menús al pasar el cursor. El valor predeterminado es 400 ms; fijarlo en 0 hace que los menús se abran al instante.",
            broken_features: &[],
        }),
        "mmcss_gpu_priority" => Some(LocalizedTweakText {
            title: "MMCSS — Elevar la prioridad de juegos y audio profesional",
            description: "Aumenta la prioridad de los hilos de Juegos y Audio profesional dentro del Servicio programador de clases multimedia (MMCSS), asegurando mayor prioridad de planificación para audio y juegos en tiempo real.",
            broken_features: &[],
        }),
        "mouse_data_queue_size" => Some(LocalizedTweakText {
            title: "Aumentar el tamaño de cola de datos del ratón",
            description: "Incrementa el tamaño de la cola de datos de entrada del ratón desde las 100 entradas predeterminadas a un búfer mayor de 256 entradas, reduciendo el riesgo de perder eventos en ratones gaming con alta tasa de sondeo.",
            broken_features: &[],
        }),
        "keyboard_response" => Some(LocalizedTweakText {
            title: "Optimizar la respuesta del teclado",
            description: "Configura mediante el Registro el retardo de repetición del teclado al mínimo y la velocidad de repetición al máximo, y aumenta el tamaño de la cola de datos del teclado.",
            broken_features: &[],
        }),
        "disable_wer_queue" => Some(LocalizedTweakText {
            title: "Desactivar la cola de Informes de errores de Windows (WER)",
            description: "Desactiva la cola de Windows Error Reporting (WER), evitando que el sistema escriba volcados de memoria e informes en segundo plano tras fallos de aplicaciones.",
            broken_features: &["Los informes de fallos no se enviarán a Microsoft"],
        }),
        "auto_end_tasks" => Some(LocalizedTweakText {
            title: "Finalizar automáticamente tareas bloqueadas al apagar",
            description: "Cierra automáticamente aplicaciones bloqueadas durante el apagado sin preguntar al usuario. También reduce WaitToKillAppTimeout y HungAppTimeout para acelerar el cierre de sesión y apagado.",
            broken_features: &["El trabajo sin guardar en aplicaciones bloqueadas puede perderse al apagar"],
        }),
        "network_throttling_disable" => Some(LocalizedTweakText {
            title: "Desactivar el índice de limitación de red (NetworkThrottlingIndex)",
            description: "Desactiva la limitación multimedia de red de Windows (estableciendo NetworkThrottlingIndex en 0xFFFFFFFF), evitando pérdida de paquetes y picos de latencia durante tráfico intenso o juegos.",
            broken_features: &[],
        }),
        "system_responsiveness_gaming" => Some(LocalizedTweakText {
            title: "Maximizar capacidad de respuesta del sistema en primer plano",
            description: "Configura SystemResponsiveness al 0 %, dedicando el 100 % de los ciclos de CPU a juegos y aplicaciones interactivas en primer plano en lugar de reservar el 20 % para tareas en segundo plano.",
            broken_features: &[],
        }),
        "ntfs_disable_last_access" => Some(LocalizedTweakText {
            title: "Desactivar marcas de tiempo de último acceso NTFS",
            description: "Impide que el sistema de archivos NTFS actualice las marcas de tiempo de metadatos en disco en cada lectura de archivo, reduciendo notablemente la carga de E/S en SSD y NVMe.",
            broken_features: &["Las propiedades del archivo no mostrarán la fecha exacta del último acceso"],
        }),
        "disable_paging_executive" => Some(LocalizedTweakText {
            title: "Mantener el núcleo de Windows y controladores en RAM",
            description: "Fuerza a que el núcleo ejecutivo de Windows NT y las rutinas de controladores permanezcan residentes en RAM física en lugar de paginarse a disco, garantizando respuesta instantánea a interrupciones.",
            broken_features: &[],
        }),
        "large_system_cache" => Some(LocalizedTweakText {
            title: "Activar caché grande de archivos del sistema",
            description: "Configura Windows para mantener una caché de sistema de archivos mayor en la RAM física, acelerando accesos repetitivos a archivos y la transmisión de texturas en juegos.",
            broken_features: &[],
        }),
        "disable_fast_startup" => Some(LocalizedTweakText {
            title: "Desactivar Inicio rápido (prevención de fugas de suspensión híbrida)",
            description: "Desactiva el Inicio rápido de Windows (hiberboot), asegurando un ciclo de arranque realmente limpio en cada apagado y evitando fugas de memoria del núcleo y corrupción de estado de controladores.",
            broken_features: &["El tiempo de arranque en frío puede aumentar 1-2 segundos en discos mecánicos lentos"],
        }),
        "ntfs_disable_8dot3" => Some(LocalizedTweakText {
            title: "Desactivar creación de nombres cortos NTFS 8.3",
            description: "Desactiva la generación heredada de nombres cortos MS-DOS 8.3 en volúmenes NTFS, eliminando la latencia de indexación de directorios y sobrecarga de escritura en unidades NVMe/SSD rápidas.",
            broken_features: &[],
        }),
        "increase_mft_zone_reservation" => Some(LocalizedTweakText {
            title: "Aumentar el búfer de zona de la Tabla maestra de archivos (MFT)",
            description: "Reserva un búfer contiguo mayor en disco para la Master File Table de NTFS, evitando la fragmentación de la MFT y manteniendo el máximo rendimiento de acceso al disco.",
            broken_features: &[],
        }),
        "tcp_ack_frequency" => Some(LocalizedTweakText {
            title: "TCP de baja latencia (TcpAckFrequency)",
            description: "Obliga a la pila de red TCP de Windows a enviar confirmaciones ACK de paquetes inmediatamente sin esperar a la fusión por tiempo de espera, reduciendo el ping en juegos en línea.",
            broken_features: &[],
        }),
        "tcp_no_delay_global" => Some(LocalizedTweakText {
            title: "Desactivar algoritmo de Nagle (TCPNoDelay)",
            description: "Desactiva el algoritmo de almacenamiento en búfer de paquetes de Nagle en interfaces de red activas, transmitiendo paquetes pequeños al instante para lograr la mínima latencia de ida y vuelta.",
            broken_features: &[],
        }),
        "disable_mouse_acceleration" => Some(LocalizedTweakText {
            title: "Desactivar Mejorar la precisión del puntero (entrada de ratón 1:1)",
            description: "Desactiva la aceleración no lineal del cursor en Windows, ofreciendo un movimiento físico puro 1:1 del hardware esencial para la precisión competitiva.",
            broken_features: &[],
        }),

        // === telemetry_apps.toml (7) ===
        "disable_nvidia_telemetry_service" => Some(LocalizedTweakText {
            title: "Desactivar el servicio Contenedor de telemetría de NVIDIA",
            description: "Desactiva el servicio NvTelemetryContainer, que recopila y transmite estadísticas de uso y datos de fallos del controlador NVIDIA.",
            broken_features: &["No se recopilarán datos de fallos ni telemetría de uso de NVIDIA"],
        }),
        "disable_nvidia_telemetry_tasks" => Some(LocalizedTweakText {
            title: "Desactivar tareas programadas de telemetría de NVIDIA",
            description: "Desactiva las tareas programadas NvTmMon, NvTmRep y NvProfile que informan periódicamente datos de telemetría de NVIDIA.",
            broken_features: &[],
        }),
        "disable_adobe_update_service" => Some(LocalizedTweakText {
            title: "Desactivar servicios de actualización de Adobe",
            description: "Desactiva Adobe Acrobat Update Service (AdobeARMservice) y Adobe Genuine Monitor Service. Evita comprobaciones de actualización en segundo plano y telemetría de licencias.",
            broken_features: &["Los productos de Adobe no se actualizarán automáticamente; actualiza manualmente"],
        }),
        "disable_edge_update" => Some(LocalizedTweakText {
            title: "Desactivar servicios de actualización de Microsoft Edge",
            description: "Desactiva los servicios Microsoft Edge Update (edgeupdate y edgeupdatem) y sus tareas programadas. Edge seguirá funcionando pero no se actualizará automáticamente ni consultará actualizaciones.",
            broken_features: &["Microsoft Edge no se actualizará automáticamente; los parches de seguridad deben aplicarse manualmente"],
        }),
        "disable_chrome_update" => Some(LocalizedTweakText {
            title: "Desactivar servicios de actualización de Google Chrome",
            description: "Desactiva los servicios Google Update (gupdate y gupdatem) y sus tareas programadas. Chrome seguirá funcionando pero no se actualizará automáticamente.",
            broken_features: &["Google Chrome no se actualizará automáticamente; los parches de seguridad deben aplicarse manualmente"],
        }),
        "disable_steam_client_service" => Some(LocalizedTweakText {
            title: "Desactivar inicio automático de Steam Client Service",
            description: "Cambia Steam Client Service de inicio automático a manual. Steam seguirá iniciando el servicio bajo demanda al abrirse, pero no consumirá recursos en segundo plano cuando Steam no se use.",
            broken_features: &[],
        }),
        "disable_razer_telemetry" => Some(LocalizedTweakText {
            title: "Desactivar telemetría y análisis de Razer",
            description: "Impide el inicio automático del Razer Game Manager Service y los servicios de Razer Chroma SDK, reduciendo la telemetría y los análisis en segundo plano de Razer Synapse.",
            broken_features: &["Las funciones de inicio automático de Razer Synapse pueden requerir inicio manual"],
        }),

        // === telemetry_os.toml (14) ===
        "disable_diagtrack" => Some(LocalizedTweakText {
            title: "Desactivar DiagTrack (Experiencias del usuario y telemetría asociadas)",
            description: "Desactiva el servicio Experiencias del usuario y telemetría asociadas (DiagTrack / diagsvc), que es el servicio principal de recopilación de telemetría en Windows.",
            broken_features: &["Los datos de diagnóstico no se enviarán a Microsoft"],
        }),
        "disable_dmwappushservice" => Some(LocalizedTweakText {
            title: "Desactivar dmwappushservice (Enrutamiento de mensajes Push WAP)",
            description: "Desactiva el servicio de enrutamiento de mensajes Push de Protocolo de aplicación inalámbrica (WAP) para administración de dispositivos, usado para enrutar datos de telemetría.",
            broken_features: &["No se recibirán mensajes Push de MDM"],
        }),
        "disable_telemetry_registry" => Some(LocalizedTweakText {
            title: "Desactivar telemetría mediante el Registro AllowTelemetry",
            description: "Establece la directiva AllowTelemetry en 0 (Seguridad/Desactivado), el nivel mínimo de telemetría. En ediciones Enterprise y Education la desactiva por completo; en Home y Pro la reduce al mínimo exigido.",
            broken_features: &["Es posible que algunas funciones de Windows Insider y diagnóstico no funcionen"],
        }),
        "disable_compat_telrunner" => Some(LocalizedTweakText {
            title: "Desactivar tareas del evaluador de compatibilidad de aplicaciones",
            description: "Desactiva las tareas programadas Microsoft Compatibility Appraiser y ProgramDataUpdater que ejecutan CompatTelRunner.exe para analizar el software instalado y enviar telemetría de compatibilidad.",
            broken_features: &["Las comprobaciones de compatibilidad de actualización de Windows no se ejecutarán automáticamente"],
        }),
        "disable_ceip" => Some(LocalizedTweakText {
            title: "Desactivar Programa para la mejora de la experiencia del usuario (CEIP)",
            description: "Desactiva las tareas programadas de CEIP (Consolidator, UsbCeip, KernelCeipTask) y la clave de directiva CEIP en el Registro.",
            broken_features: &["No se recopilarán datos de telemetría del programa CEIP"],
        }),
        "disable_wer_service" => Some(LocalizedTweakText {
            title: "Desactivar el servicio de Informes de errores de Windows",
            description: "Desactiva el servicio Informes de errores de Windows (WerSvc), impidiendo el envío de informes de fallos y telemetría de errores a Microsoft.",
            broken_features: &[
                "Los informes de fallos no se enviarán a Microsoft",
                "El historial de Informes de problemas no se actualizará",
            ],
        }),
        "disable_connected_user_experiences" => Some(LocalizedTweakText {
            title: "Desactivar experiencias de usuario conectadas y sugerencias",
            description: "Desactiva sugerencias en la aplicación Configuración, el menú Inicio y la pantalla de bloqueo impulsadas por contenido en la nube y comentarios telemétricos.",
            broken_features: &["No aparecerán sugerencias ni consejos de Windows"],
        }),
        "disable_diagnostic_data_viewer" => Some(LocalizedTweakText {
            title: "Desactivar Visor de datos de diagnóstico",
            description: "Desactiva el registro de eventos del Visor de datos de diagnóstico, que guarda una copia local de los datos enviados a Microsoft. Libera E/S de disco y almacenamiento.",
            broken_features: &["No se podrán ver los datos de diagnóstico localmente mediante la aplicación Visor de datos de diagnóstico"],
        }),
        "disable_delivery_optimization_p2p" => Some(LocalizedTweakText {
            title: "Desactivar consumo P2P de subida de Optimización de distribución",
            description: "Restringe la Optimización de distribución de Windows (DODownloadMode = 0), evitando que Windows utilice tu ancho de banda de subida para compartir actualizaciones con otros PC por Internet.",
            broken_features: &[],
        }),
        "disable_diagnostic_execution_service" => Some(LocalizedTweakText {
            title: "Desactivar el servicio de ejecución de diagnóstico (diagsvc)",
            description: "Desactiva el servicio de ejecución de diagnóstico (diagsvc), encargado de ejecutar diagnósticos programados en segundo plano y paquetes de telemetría.",
            broken_features: &[],
        }),
        "wersvc_service_disable" => Some(LocalizedTweakText {
            title: "Desactivar el servicio recopilador estándar del Centro de diagnósticos",
            description: "Desactiva el servicio Visual Studio / Windows Diagnostics Hub Standard Collector, deteniendo recopiladores de trazas ETW en segundo plano cuando está inactivo.",
            broken_features: &["La recopilación de trazas del Centro de diagnósticos VS/Windows requiere inicio manual"],
        }),
        "disable_remote_registry" => Some(LocalizedTweakText {
            title: "Desactivar el servicio Registro remoto",
            description: "Desactiva el servicio RemoteRegistry, impidiendo que usuarios remotos y entidades de red externas consulten o modifiquen claves locales del Registro.",
            broken_features: &["Se desactivará la administración remota del Registro a través de la red LAN local"],
        }),
        "disable_disk_diagnostic_task" => Some(LocalizedTweakText {
            title: "Desactivar tarea telemétrica de diagnóstico de disco",
            description: "Desactiva la tarea programada que recopila métricas de uso del disco y diagnósticos de hardware para informes de telemetría.",
            broken_features: &[],
        }),
        "disable_winsat_task" => Some(LocalizedTweakText {
            title: "Desactivar tarea de evaluación comparativa WinSAT en segundo plano",
            description: "Impide que la tarea programada de la Herramienta de evaluación del sistema Windows ejecute pruebas de esfuerzo de disco y GPU sin aviso durante la inactividad del sistema.",
            broken_features: &[],
        }),

        // === ux_customization.toml (13) ===
        "enable_classic_context_menu" => Some(LocalizedTweakText {
            title: "Restaurar el menú contextual clásico completo de Windows 10",
            description: "Registra la anulación CLSID bajo HKCU\\Software\\Classes para que al hacer clic derecho se abra inmediatamente el menú contextual completo sin un clic adicional en 'Mostrar más opciones'.",
            broken_features: &["Menú contextual compacto moderno de Win11 sustituido por el menú completo clásico"],
        }),
        "explorer_power_user_defaults" => Some(LocalizedTweakText {
            title: "Mostrar extensiones de archivo, ruta completa, abrir en Este equipo y ocultar avisos de sincronización",
            description: "Muestra extensiones de archivo conocidas y archivos ocultos, abre el Explorador directamente en 'Este equipo' en lugar de Acceso rápido, activa rutas completas en la barra de título y vista compacta, y oculta banners de OneDrive/sincronización.",
            broken_features: &[],
        }),
        "explorer_hide_win11_home_gallery_nav" => Some(LocalizedTweakText {
            title: "Ocultar anclajes de Inicio y Galería del panel de navegación del Explorador",
            description: "Desancla las entradas Inicio (Home) y Galería (Gallery) de Windows 11 del panel izquierdo del Explorador de archivos mediante HKCU\\Software\\Classes\\CLSID.",
            broken_features: &["Accesos rápidos de Inicio y Galería ocultos en la barra lateral izquierda del Explorador"],
        }),
        "explorer_disable_startup_delay_and_jpeg_compression" => Some(LocalizedTweakText {
            title: "Eliminar retraso de inicio de aplicaciones y desactivar compresión JPEG de fondos",
            description: "Establece Explorer\\Serialize\\StartupDelayInMSec=0 para que los programas de inicio arranquen sin espera artificial de 10s, y JPEGImportQuality=100 para fondos de escritorio sin pérdida.",
            broken_features: &[],
        }),
        "explorer_desktop_this_pc_and_disable_autoplay" => Some(LocalizedTweakText {
            title: "Mostrar icono 'Este equipo' en el escritorio y desactivar Reproducción automática",
            description: "Coloca el acceso directo 'Este equipo' en el escritorio y desactiva AutoPlay/AutoRun en unidades USB y ópticas extraíbles para bloquear avisos automáticos y riesgos de ejecución automática.",
            broken_features: &["Los medios USB/SD conectados ya no abren ventanas emergentes automáticas"],
        }),
        "disable_visual_effects_userpreferencesmask_bits" => Some(LocalizedTweakText {
            title: "Quitar animaciones pesadas de fundido/deslizamiento/sombra mediante bits UserPreferencesMask y DWM",
            description: "Aplica mutaciones precisas con máscara de bits en UserPreferencesMask (desactivando fundido/deslizamiento de menús, fundido de descripciones, selección, sombra del puntero, listas desplegables, desplazamiento suave y animaciones de controles mientras conserva suavizado de fuentes) más EnableAeroPeek=0.",
            broken_features: &["Transiciones de interfaz clásicas e instantáneas sin animaciones de deslizamiento/fundido ni sombras"],
        }),
        "taskbar_win11_power_layout_and_end_task" => Some(LocalizedTweakText {
            title: "Alinear barra de tareas Win11 a la izquierda, ocultar Widgets/Vista de tareas/Búsqueda, activar Finalizar tarea (Directiva Pro/Ent)",
            description: "Alinea iconos a la izquierda (TaskbarAl=0), quita Vista de tareas y Widgets (TaskbarDa=0), oculta el cuadro de Búsqueda, habilita 'Finalizar tarea' con clic derecho en opciones de desarrollador, desactiva Aero Shake y quita la sección 'Recomendado' del menú Inicio (Windows 11 22H2+ Pro/Enterprise/Education).",
            broken_features: &["Iconos de la barra de tareas alineados a la izquierda; botones Vista de tareas y Widgets ocultos"],
        }),
        "hide_win10_meet_now_tray" => Some(LocalizedTweakText {
            title: "Ocultar icono de cámara 'Reunirse ahora' (Meet Now) en la bandeja de Windows 10",
            description: "Oculta el icono de Skype 'Reunirse ahora' del área de notificación de Windows 10 (solo para compilaciones de Windows 10).",
            broken_features: &[],
        }),
        "enable_windows_dark_mode_and_clock_seconds" => Some(LocalizedTweakText {
            title: "Activar modo oscuro del sistema y aplicaciones + segundos en el reloj de la barra",
            description: "Cambia AppsUseLightTheme=0 y SystemUsesLightTheme=0 para activar el Modo oscuro nativo en la interfaz de Windows y muestra los segundos en tiempo real en el reloj de la barra de tareas.",
            broken_features: &[],
        }),
        "disable_startup_sound_and_audio_ducking" => Some(LocalizedTweakText {
            title: "Desactivar sonido de arranque de Windows y atenuación de volumen en llamadas",
            description: "Silencia el sonido de inicio de sesión de Windows y evita que Windows baje el volumen de juegos/música en un 80 % cuando Discord, Teams o apps de voz abren un flujo de comunicaciones.",
            broken_features: &[],
        }),
        "enable_verbose_logon_status" => Some(LocalizedTweakText {
            title: "Activar mensajes detallados de estado al arrancar, iniciar sesión y apagar",
            description: "Establece VerboseStatus=1 para que Windows muestre las etapas exactas de servicios y actualizaciones en lugar de pantallas genéricas con círculo de carga.",
            broken_features: &[],
        }),
        "disable_lock_screen_pro" => Some(LocalizedTweakText {
            title: "Omitir la cortina de pantalla de bloqueo previa al inicio de sesión (Directiva Pro/Enterprise)",
            description: "Salta directamente al cuadro de contraseña/PIN tras encender o bloquear el equipo sin necesidad de deslizar primero la imagen de la pantalla de bloqueo.",
            broken_features: &["Se omite la cortina deslizable de la pantalla de bloqueo en ediciones Pro/Enterprise"],
        }),
        "enable_windows_developer_mode" => Some(LocalizedTweakText {
            title: "Activar Modo para desarrolladores de Windows (instalación local de apps y enlaces simbólicos)",
            description: "Activa AllowDevelopmentWithoutDevLicense=1 para permitir crear enlaces simbólicos en el sistema de archivos sin UAC e instalar paquetes MSIX/AppX sin firmar.",
            broken_features: &["Permite la instalación local de paquetes MSIX sin firmar; pensado para desarrolladores"],
        }),
        // === system_deep_opt.toml (30) ===
        "disable_dwm_animations_per_process" => Some(LocalizedTweakText {
            title: "Desactivar animaciones de transición por proceso de DWM",
            description: "Desactiva las animaciones de transición de DWM por proceso estableciendo DisallowAnimations=1 y AnimationsShiftKey=1 en la clave DWM. Elimina transiciones de desvanecimiento y deslizamiento al minimizar, maximizar o cambiar ventanas, reduciendo la sobrecarga del compositor de GPU.",
            broken_features: &["Se eliminarán las animaciones de transición de ventanas"],
        }),
        "disable_dwm_composition_throttle" => Some(LocalizedTweakText {
            title: "Desactivar limitación de composición del Administrador de ventanas de escritorio",
            description: "Establece DesktopCompositionThrottle=0 en la clave DWM, evitando que el compositor limite la representación cuando se pierden fotogramas. Permite que DWM mantenga el máximo rendimiento de composición bajo carga de GPU.",
            broken_features: &[],
        }),
        "disable_kernel_mitigations_audit" => Some(LocalizedTweakText {
            title: "Desactivar modo de auditoría de mitigaciones del kernel",
            description: "Desactiva el registro de auditoría de opciones de mitigación del kernel estableciendo MitigationAuditOptions en ceros. Reduce los eventos ETW en la creación de procesos y manejo de excepciones.",
            broken_features: &["No se registrarán eventos de auditoría de mitigación de vulnerabilidades"],
        }),
        "disable_llmnr_protocol" => Some(LocalizedTweakText {
            title: "Desactivar protocolo de resolución de nombres LLMNR",
            description: "Desactiva la resolución de nombres de multidifusión local de vínculo (LLMNR) estableciendo EnableMulticast=0 en los parámetros del cliente DNS. Reduce el tráfico de multidifusión innecesario y previene ataques de envenenamiento LLMNR.",
            broken_features: &["La resolución de nombres en redes locales sin DNS puede fallar", "La detección de dispositivos locales tipo grupo de hogar puede verse afectada"],
        }),
        "disable_memory_compression" => Some(LocalizedTweakText {
            title: "Desactivar motor de compresión de memoria",
            description: "Desactiva el almacén de compresión del Administrador de memoria de Windows estableciendo DisableCompression=1. En equipos con 16 GB o más de RAM, reduce los picos de uso de CPU causados por el subproceso de compresión del sistema.",
            broken_features: &["Mayor uso del archivo de paginación en sistemas con memoria limitada", "Puede causar condiciones de falta de memoria antes en equipos con 8 GB"],
        }),
        "disable_netbios_over_tcp" => Some(LocalizedTweakText {
            title: "Desactivar NetBIOS a través de TCP/IP",
            description: "Desactiva el protocolo NetBIOS a través de TCP/IP estableciendo NetbiosOptions=2 y deshabilitando la búsqueda LMHosts. NetBIOS es un protocolo LAN heredado que transmite consultas de resolución de nombres, exponiendo el sistema a ataques de envenenamiento NBNS.",
            broken_features: &["Las aplicaciones heredadas que dependen de la resolución NetBIOS fallarán", "Las impresoras de red antiguas que usan WINS pueden quedar inaccesibles"],
        }),
        "disable_ntfs_encryption_driver" => Some(LocalizedTweakText {
            title: "Desactivar controlador de cifrado del sistema de archivos NTFS (EFS)",
            description: "Establece NtfsDisableEncryption=1 en la clave FileSystem, evitando que el controlador de filtro EFS intercepte operaciones de E/S. En equipos que no usan cifrado EFS por archivo, reduce la latencia de acceso a archivos.",
            broken_features: &["El sistema de cifrado de archivos NTFS (EFS) no funcionará", "Los archivos existentes cifrados con EFS quedarán inaccesibles"],
        }),
        "disable_page_file_encryption" => Some(LocalizedTweakText {
            title: "Desactivar cifrado de archivo de paginación y borrado al apagar",
            description: "Evita que Windows cifre el archivo de paginación y desactiva su borrado al apagar (ClearPageFileAtShutdown=0, DisablePagingEncryption=1). Mejora el rendimiento de paginación y acelera notablemente el tiempo de apagado.",
            broken_features: &["Los contenidos de memoria paginada permanecen en el disco tras el apagado", "Los datos confidenciales pueden ser recuperables del disco sin procesar"],
        }),
        "disable_processor_idle_demote_threshold" => Some(LocalizedTweakText {
            title: "Desactivar degradación de estados de inactividad del procesador",
            description: "Establece IdleDemoteThreshold=100 e IdlePromoteThreshold=100 en la administración de energía del procesador. Evita que el administrador de energía degrade el rendimiento de la CPU en pausas breves de actividad, eliminando retrasos de escalado de frecuencia.",
            broken_features: &["Mayor consumo de energía en estado inactivo", "Menor duración de la batería en ordenadores portátiles"],
        }),
        "disable_rss_base_cpu" => Some(LocalizedTweakText {
            title: "Establecer CPU base RSS para distribución de carga de red",
            description: "Establece RssBaseCpu=1 en los parámetros NDIS, indicando al escalado en el lado de recepción que distribuya las interrupciones de red desde la CPU 1 en lugar de la CPU 0. Libera la CPU de arranque para subprocesos en primer plano y juegos.",
            broken_features: &[],
        }),
        "disable_spectre_meltdown_mitigations" => Some(LocalizedTweakText {
            title: "Desactivar mitigaciones de vulnerabilidad de CPU Spectre/Meltdown",
            description: "Desactiva las mitigaciones de hardware para Spectre y Meltdown estableciendo FeatureSettingsOverride=3 y FeatureSettingsOverrideMask=3. Elimina la penalización de rendimiento en cargas de trabajo de computación pesada.",
            broken_features: &["El sistema es vulnerable a ataques de canal lateral de CPU Spectre y Meltdown"],
        }),
        "disable_threaded_dpc_optimization" => Some(LocalizedTweakText {
            title: "Desactivar DPC en subprocesos para menor latencia de interrupciones",
            description: "Establece ThreadDpcEnable=0 en el kernel, forzando la ejecución de llamadas a procedimientos diferidos en DISPATCH_LEVEL en lugar de subprocesos PASSIVE_LEVEL. Reduce la latencia DPC para producción de audio en tiempo real y juegos competitivos.",
            broken_features: &["Algunos controladores que esperan DPC en subprocesos pueden comportarse mal", "Mayor variación de latencia DPC bajo carga extrema de múltiples subprocesos"],
        }),
        "disable_windows_search_indexer" => Some(LocalizedTweakText {
            title: "Desactivar servicio de indización de Windows Search",
            description: "Desactiva el servicio WSearch, impidiendo que el indizador rastree continuamente los archivos en segundo plano. En sistemas SSD/NVMe, elimina el uso constante de disco, CPU y memoria de SearchIndexer.exe.",
            broken_features: &["Windows Search no devolverá resultados para el contenido de los archivos", "La búsqueda de escritorio de Outlook puede ser más lenta", "La búsqueda del menú Inicio no encontrará archivos por contenido"],
        }),
        "enable_hardware_task_offload" => Some(LocalizedTweakText {
            title: "Habilitar descarga de tareas de hardware TCP/IP",
            description: "Establece DisableTaskOffload=0 en TCP/IP, garantizando que el sistema operativo descargue el cálculo de suma de comprobación y la segmentación TCP en el hardware de la tarjeta de red. Reduce el uso de CPU.",
            broken_features: &[],
        }),
        "enable_large_ntfs_frs" => Some(LocalizedTweakText {
            title: "Habilitar segmentos grandes de registro de archivos NTFS",
            description: "Establece NtfsAllowExtendedCharacterIn8dot3Name=0, permitiendo que el controlador NTFS asigne segmentos de registro de archivos más grandes y eficientes, reduciendo la fragmentación de la MFT.",
            broken_features: &["Se eliminarán los caracteres Unicode extendidos en nombres cortos 8.3 autogenerados"],
        }),
        "enable_ultimate_performance_plan" => Some(LocalizedTweakText {
            title: "Activar plan de energía de máximo rendimiento",
            description: "Activa el plan de energía oculto de Máximo Rendimiento. Elimina micro-latencias de ahorro de energía: sin estacionamiento de núcleos, sin demoras de escala de frecuencia, sin suspensión selectiva USB ni apagado de discos.",
            broken_features: &["Consumo de energía significativamente mayor en estado inactivo", "Menor duración de la batería del portátil", "Temperaturas más altas de CPU y chipset en reposo"],
        }),
        "increase_gpu_tdr_timeout" => Some(LocalizedTweakText {
            title: "Aumentar retraso de detección y recuperación de tiempo de espera de GPU",
            description: "Aumenta TdrDelay de 2 a 10 segundos, otorgando a la GPU más tiempo para completar sombreadores complejos antes de que Windows reinicie el controlador. Evita reinicios TDR falsos en computación gráfica pesada.",
            broken_features: &["Los bloqueos reales de la GPU tardarán más en recuperarse", "La pantalla puede parecer congelada durante más tiempo antes de reiniciar el controlador"],
        }),
        "increase_irp_stack_size" => Some(LocalizedTweakText {
            title: "Aumentar tamaño de pila IRP para E/S de red y disco",
            description: "Aumenta el tamaño de la pila de paquetes de solicitud de E/S de 15 a 32 entradas en LanmanServer. Evita errores STATUS_INSUFFICIENT_RESOURCES y mejora el rendimiento con antivirus o filtros de almacenamiento.",
            broken_features: &[],
        }),
        "increase_max_connections_per_server" => Some(LocalizedTweakText {
            title: "Aumentar el máximo de conexiones HTTP por servidor",
            description: "Aumenta MaxConnectionsPerServer y MaxConnectionsPer1_0Server a 16, permitiendo que las aplicaciones basadas en WinINet abran más conexiones HTTP simultáneas por host, acelerando descargas y transferencias.",
            broken_features: &[],
        }),
        "increase_nonpaged_pool_quota" => Some(LocalizedTweakText {
            title: "Aumentar cuota del grupo no paginado para estabilidad de controladores",
            description: "Establece NonPagedPoolQuota en 0 (máximo administrado por el sistema), permitiendo al administrador de memoria usar toda la memoria física para asignaciones no paginadas, evitando el agotamiento de recursos.",
            broken_features: &[],
        }),
        "optimize_additional_critical_worker_threads" => Some(LocalizedTweakText {
            title: "Aumentar subprocesos de trabajo adicionales del kernel",
            description: "Establece AdditionalCriticalWorkerThreads en 2 y AdditionalDelayedWorkerThreads en 2, aprovisionando subprocesos de trabajo adicionales del kernel para evitar bloqueos en colas de trabajo con alta E/S.",
            broken_features: &[],
        }),
        "optimize_default_ttl" => Some(LocalizedTweakText {
            title: "Establecer TTL óptimo predeterminado de paquetes IP",
            description: "Establece el valor predeterminado de tiempo de vida (TTL) de IP en 64 saltos, coincidiendo con el estándar de Linux/macOS. 64 saltos es suficiente para prácticamente todas las topologías de enrutamiento.",
            broken_features: &[],
        }),
        "optimize_disk_io_page_lock_limit" => Some(LocalizedTweakText {
            title: "Aumentar límite de bloqueo de páginas de E/S para transferencias grandes",
            description: "Establece IoPageLockLimit en 983040 bytes (960 KB), aumentando la cantidad de memoria que el administrador de E/S puede bloquear para transferencias directas en unidades NVMe rápidas.",
            broken_features: &[],
        }),
        "optimize_dns_cache_parameters" => Some(LocalizedTweakText {
            title: "Optimizar tamaño de caché y TTL del cliente DNS",
            description: "Establece MaxCacheEntryTtlLimit=86400 y MaxNegativeCacheTtl=5 en Dnscache, almacenando en caché búsquedas exitosas por más tiempo y reintentando rápidamente las fallidas.",
            broken_features: &[],
        }),
        "optimize_gpu_preemption_level" => Some(LocalizedTweakText {
            title: "Configurar programador de GPU para preferencia solo de cómputo",
            description: "Establece GpuPreemptionLevel=0 en el programador de controladores gráficos. Evita que el programador interrumpa sombreadores en ejecución, reduciendo microtirones y sobrecarga de contexto en juegos.",
            broken_features: &["La detección y recuperación de tiempo de espera de GPU (TDR) puede activarse más lentamente"],
        }),
        "optimize_interrupt_steering" => Some(LocalizedTweakText {
            title: "Optimizar afinidad de interrupciones y directiva de direccionamiento",
            description: "Configura la directiva de direccionamiento de interrupciones del kernel estableciendo InterruptSteeringDisabled=0 y DistributeTimers=1, distribuyendo interrupciones de hardware entre todos los núcleos.",
            broken_features: &[],
        }),
        "optimize_mmcss_proaudio_scheduling" => Some(LocalizedTweakText {
            title: "MMCSS Pro Audio — Máxima prioridad de programación",
            description: "Establece la categoría de programación y prioridad SFIO de la tarea Pro Audio de MMCSS en 'High', garantizando la máxima prioridad para subprocesos de audio en DAWs y aplicaciones de comunicación.",
            broken_features: &[],
        }),
        "optimize_ntfs_memory_usage" => Some(LocalizedTweakText {
            title: "Caché agresiva de grupo paginado NTFS",
            description: "Establece NtfsMemoryUsage en 2, indicando al controlador NTFS que use una asignación de grupo paginado más grande para almacenar en caché directorios y metadatos, mejorando el rendimiento de compilación.",
            broken_features: &[],
        }),
        "optimize_object_manager_quota" => Some(LocalizedTweakText {
            title: "Aumentar cuota de identificadores del Administrador de objetos del kernel",
            description: "Aumenta el límite de la tabla de identificadores mediante SessionPoolSize=48 en Memory Management. Evita el agotamiento de identificadores en sistemas con numerosos procesos concurrentes e IDEs.",
            broken_features: &[],
        }),
        "optimize_timer_resolution_requests" => Some(LocalizedTweakText {
            title: "Permitir que las aplicaciones soliciten alta resolución de temporizador",
            description: "Establece GlobalTimerResolutionRequests=1 en el kernel, permitiendo a las aplicaciones solicitar la máxima resolución de temporizador (0,5 ms) en todo el sistema, eliminando fluctuaciones de sincronización.",
            broken_features: &["Mayor consumo de energía en reposo debido a una mayor frecuencia de interrupciones del temporizador"],
        }),
        // === gaming_latency_ultra.toml (10) ===
        "disable_driver_search_order_probing" => Some(LocalizedTweakText {
            title: "Desactivar sondeo de búsqueda de controladores en Windows Update",
            description: "Evita que Windows Update consulte automáticamente los servidores de actualización en segundo plano al buscar controladores. Elimina la latencia de instalación de dispositivos y sondeos de red.",
            broken_features: &["La búsqueda automática de controladores en línea mediante Windows Update no se ejecutará al conectar dispositivos nuevos"],
        }),
        "disable_fast_user_switching" => Some(LocalizedTweakText {
            title: "Desactivar cambio rápido de usuario",
            description: "Desactiva el cambio rápido de usuario para evitar que las sesiones inactivas y sus tareas en segundo plano permanezcan cargadas en la RAM física, liberando ancho de banda y CPU.",
            broken_features: &["No se podrán mantener varias sesiones de usuario locales abiertas simultáneamente"],
        }),
        "disable_ip_source_routing" => Some(LocalizedTweakText {
            title: "Desactivar enrutamiento de origen IP",
            description: "Establece DisableIPSourceRouting=2 para descartar paquetes IP con enrutamiento de origen entrantes. Evita la suplantación de paquetes, el secuestro de rutas y la sobrecarga de la pila de red.",
            broken_features: &["Se rechazarán las herramientas de diagnóstico de red basadas en enrutamiento de origen (poco comunes)"],
        }),
        "disable_netsh_teredo_tunneling" => Some(LocalizedTweakText {
            title: "Desactivar tunelización de transición Teredo IPv6",
            description: "Desactiva la interfaz de túnel Teredo configurando DisabledComponents en 32 en IPv6. Teredo encapsula IPv6 en UDP IPv4, generando latencia adicional de cruce NAT.",
            broken_features: &["Los juegos multijugador P2P antiguos de Xbox Live que dependen estrictamente del cruce NAT Teredo no funcionarán"],
        }),
        "disable_network_adapter_power_saving" => Some(LocalizedTweakText {
            title: "Desactivar ahorro de energía PnP del adaptador de red NDIS",
            description: "Configura NDIS PnPCapabilities en 24, desactivando estados de bajo consumo en adaptadores de red. Evita que las tarjetas de red entren en reposo en pausas de juego, eliminando retrasos.",
            broken_features: &["Se desactivarán el encendido por red (Wake-on-LAN) y el ahorro de energía en reposo de la interfaz de red"],
        }),
        "disable_remote_desktop_udp" => Some(LocalizedTweakText {
            title: "Desactivar transporte UDP del cliente de Escritorio remoto",
            description: "Obliga al cliente RDP a utilizar TCP en lugar de UDP. Elimina la congestión del búfer de sockets UDP y evita que el tráfico RDP compita con los paquetes de juegos UDP.",
            broken_features: &["Las sesiones de Escritorio remoto en conexiones WAN de alta latencia utilizarán TCP en lugar de UDP"],
        }),
        "disable_smart_card_service" => Some(LocalizedTweakText {
            title: "Desactivar servicio de tarjeta inteligente (SCardSvr)",
            description: "Desactiva el servicio de tarjetas inteligentes (SCardSvr) estableciendo Start=4. Elimina el sondeo en segundo plano de lectores de tarjetas y tokens en equipos de juego.",
            broken_features: &["La autenticación mediante tarjeta inteligente física y el hardware lector de tarjetas no funcionarán"],
        }),
        "optimize_kerberos_ticket_cache" => Some(LocalizedTweakText {
            title: "Optimizar búfer de tamaño máximo de token Kerberos",
            description: "Establece el búfer MaxTokenSize de Kerberos en 65535 bytes, evitando desbordamientos de búfer y renegociaciones continuas de tickets en redes con grupos de seguridad complejos.",
            broken_features: &[],
        }),
        "optimize_tcp_syn_attack_protection" => Some(LocalizedTweakText {
            title: "Habilitar protección contra ataques TCP SYN",
            description: "Habilita la defensa contra inundaciones TCP SYN en la pila TCP/IP (SynAttackProtect=1). Acelera los tiempos de espera de protocolo de enlace protegiendo los sockets de juegos.",
            broken_features: &[],
        }),
        "optimize_usb_selective_suspend_hub" => Some(LocalizedTweakText {
            title: "Desactivar suspensión selectiva global de concentradores USB",
            description: "Desactiva la suspensión selectiva USB a nivel de controlador raíz (DisableSelectiveSuspend=1). Evita que ratones, teclados y DACs entren en reposo, garantizando latencia cero de entrada.",
            broken_features: &["Consumo de energía ligeramente mayor en el bus USB en reposo", "Menor duración de la batería del portátil"],
        }),
        "remove_webexperience_package" => Some(LocalizedTweakText {
            title: "Eliminar paquete Windows WebExperienceHost (Host de widgets)",
            description: "Elimina el paquete MicrosoftWindows.Client.WebExperience para evitar que los widgets de la barra de tareas consuman memoria de Edge WebView2 en segundo plano.",
            broken_features: &["Los widgets de la barra de tareas y las noticias se eliminarán permanentemente."],
        }),
        "disable_wpbt_execution" => Some(LocalizedTweakText {
            title: "Deshabilitar ejecución de binarios de firmware WPBT",
            description: "Evita que el kernel de Windows ejecute binarios incrustados en la tabla WPBT de la UEFI de la placa base.",
            broken_features: &[],
        }),
        "service_shutdown_timeout_15s" => Some(LocalizedTweakText {
            title: "Reducir tiempo de espera de apagado de servicios a 15 segundos",
            description: "Reduce el tiempo de espera de terminación de servicios durante el apagado de 20 s a 15 s para evitar bloqueos.",
            broken_features: &[],
        }),
        "start_menu_show_more_pins" => Some(LocalizedTweakText {
            title: "Menú Inicio: diseño con más anclajes",
            description: "Configura el menú Inicio de Windows 11 para priorizar las aplicaciones ancladas sobre las sugerencias recomendadas.",
            broken_features: &[],
        }),
        "disable_taskbar_people_button" => Some(LocalizedTweakText {
            title: "Deshabilitar botón Contactos de la barre de tareas",
            description: "Oculta y deshabilita el icono Contactos de la barra de tareas de Windows.",
            broken_features: &[],
        }),
        "repair_ndu_graph" => Some(LocalizedTweakText {
            title: "Reparar gráfico de uso de datos de red (NDU) del Administrador de tareas",
            description: "Asegura que el controlador del kernel NDU esté configurado en Start=2 (Automático) para que los gráficos de red del Administrador de tareas funcionen correctamente.",
            broken_features: &[],
        }),
        "disable_settings_ads" => Some(LocalizedTweakText {
            title: "Deshabilitar contenido sugerido y anuncios en Configuración",
            description: "Evita que Microsoft muestre aplicaciones sugeridas, promociones de socios y sugerencias dentro de la Configuración de Windows.",
            broken_features: &["Se suprimen las tarjetas promocionales y sugerencias en Configuración."],
        }),
        "disable_start_menu_suggested_apps" => Some(LocalizedTweakText {
            title: "Deshabilitar instalación automática de aplicaciones sugeridas",
            description: "Evita que Windows Content Delivery Manager descargue y ancle silenciosamente aplicaciones y juegos sugeridos.",
            broken_features: &["Los juegos patrocinados no se instalarán automáticamente en el menú Inicio."],
        }),
        "disable_hardware_companion_software" => Some(LocalizedTweakText {
            title: "Deshabilitar recomendaciones de software complementario de hardware",
            description: "Evita que Windows notifique y recomiende automáticamente software complementario de OEM al conectar periféricos.",
            broken_features: &[],
        }),
        "disable_ms_office_telemetry" => Some(LocalizedTweakText {
            title: "Deshabilitar telemetría y registro de experiencia de Microsoft Office",
            description: "Deshabilita la recopilación de datos de diagnóstico de Microsoft Office, las solicitudes de comentarios y la telemetría del Programa para la mejora de la experiencia del cliente (CEIP).",
            broken_features: &[],
        }),
        "set_this_pc_as_default_explorer" => Some(LocalizedTweakText {
            title: "Establecer 'Este equipo' como vista predeterminada del Explorador",
            description: "Configura el Explorador de archivos para que se abra en 'Este equipo' en lugar de 'Inicio', evitando retrasos de sincronización al inicio.",
            broken_features: &[],
        }),
        "enable_taskbar_end_task" => Some(LocalizedTweakText {
            title: "Habilitar 'Finalizar tarea' con clic derecho en la barra de tareas",
            description: "Habilita la opción nativa de Windows 11 'Finalizar tarea' directamente en la barra de tareas sin abrir el Administrador de tareas.",
            broken_features: &[],
        }),
        "disable_oobe_privacy_experience" => Some(LocalizedTweakText {
            title: "Desactivar avisos de configuración de privacidad OOBE de Windows",
            description: "Suprime los avisos multipágina de configuración de privacidad OOBE durante la creación de cuentas de usuario y actualizaciones.",
            broken_features: &[],
        }),
        "disable_explorer_sync_provider_ads" => Some(LocalizedTweakText {
            title: "Desactivar anuncios del proveedor de sincronización en el Explorador",
            description: "Evita que el Explorador muestre anuncios y promociones de suscripción a OneDrive y Microsoft 365.",
            broken_features: &[],
        }),
        "disable_taskbar_task_view" => Some(LocalizedTweakText {
            title: "Desactivar botón de Vista de tareas en la barra de tareas",
            description: "Oculta el icono de Vista de tareas de la barra de tareas para ahorrar espacio, manteniendo los atajos Alt+Tab.",
            broken_features: &[],
        }),
        "hide_taskbar_meet_now" => Some(LocalizedTweakText {
            title: "Ocultar el botón 'Reunirse ahora' y Chat de la barra de tareas",
            description: "Oculta el botón integrado de Reunirse ahora y Microsoft Teams de la barra de tareas.",
            broken_features: &[],
        }),
        "disable_background_apps_global" => Some(LocalizedTweakText {
            title: "Desactivar aplicaciones en segundo plano de forma global",
            description: "Evita que las aplicaciones de la Tienda y UWP se ejecuten en segundo plano y consuman CPU/RAM cuando están cerradas.",
            broken_features: &[],
        }),
        "disable_finish_setup_ads" => Some(LocalizedTweakText {
            title: "Desactivar avisos de pantalla completa 'Terminar de configurar'",
            description: "Desactiva las pantallas emergentes que sugieren suscribirse a Microsoft 365 o configurar OneDrive tras reiniciar.",
            broken_features: &[],
        }),
        "disable_lockscreen_spotlight_ads" => Some(LocalizedTweakText {
            title: "Desactivar consejos y publicidad en la pantalla de bloqueo",
            description: "Elimina los consejos promocionales y enlaces publicitarios que aparecen en la pantalla de bloqueo de Windows.",
            broken_features: &[],
        }),
        "disable_welcome_experience_ads" => Some(LocalizedTweakText {
            title: "Desactivar pantallas de bienvenida promocionales de Windows",
            description: "Bloquea las pantallas emergentes que muestran novedades de Microsoft Edge y Copilot tras las actualizaciones.",
            broken_features: &[],
        }),
        "disable_general_tips_ads" => Some(LocalizedTweakText {
            title: "Desactivar sugerencias, consejos y trucos de Windows",
            description: "Desactiva los consejos generales del sistema y las sugerencias en las notificaciones del escritorio.",
            broken_features: &[],
        }),
        "enable_windowed_game_optimizations" => Some(LocalizedTweakText {
            title: "Habilitar optimizaciones para juegos en ventana",
            description: "Habilita el modelo de presentación moderno de Windows 11 (flip model) para juegos en ventana, reduciendo la latencia de entrada y activando VRR.",
            broken_features: &[],
        }),
        _ => None,
    }
}

/// Localized tweak title for this language.
#[allow(dead_code)]
pub fn tweak_title(id: &str) -> Option<&'static str> {
    match id {
        "disable_dwm_animations_per_process" => Some("Desactivar animaciones de transición por proceso de DWM"),
        "disable_dwm_composition_throttle" => Some("Desactivar limitación de composición del Administrador de ventanas de escritorio"),
        "disable_kernel_mitigations_audit" => Some("Desactivar modo de auditoría de mitigaciones del kernel"),
        "disable_llmnr_protocol" => Some("Desactivar protocolo de resolución de nombres LLMNR"),
        "disable_memory_compression" => Some("Desactivar motor de compresión de memoria"),
        "disable_netbios_over_tcp" => Some("Desactivar NetBIOS a través de TCP/IP"),
        "disable_ntfs_encryption_driver" => Some("Desactivar controlador de cifrado del sistema de archivos NTFS (EFS)"),
        "disable_page_file_encryption" => Some("Desactivar cifrado de archivo de paginación y borrado al apagar"),
        "disable_processor_idle_demote_threshold" => Some("Desactivar degradación de estados de inactividad del procesador"),
        "disable_rss_base_cpu" => Some("Establecer CPU base RSS para distribución de carga de red"),
        "disable_spectre_meltdown_mitigations" => Some("Desactivar mitigaciones de vulnerabilidad de CPU Spectre/Meltdown"),
        "disable_threaded_dpc_optimization" => Some("Desactivar DPC en subprocesos para menor latencia de interrupciones"),
        "disable_windows_search_indexer" => Some("Desactivar servicio de indización de Windows Search"),
        "enable_hardware_task_offload" => Some("Habilitar descarga de tareas de hardware TCP/IP"),
        "enable_large_ntfs_frs" => Some("Habilitar segmentos grandes de registro de archivos NTFS"),
        "enable_ultimate_performance_plan" => Some("Activar plan de energía de máximo rendimiento"),
        "increase_gpu_tdr_timeout" => Some("Aumentar retraso de detección y recuperación de tiempo de espera de GPU"),
        "increase_irp_stack_size" => Some("Aumentar tamaño de pila IRP para E/S de red y disco"),
        "increase_max_connections_per_server" => Some("Aumentar el máximo de conexiones HTTP por servidor"),
        "increase_nonpaged_pool_quota" => Some("Aumentar cuota del grupo no paginado para estabilidad de controladores"),
        "optimize_additional_critical_worker_threads" => Some("Aumentar subprocesos de trabajo adicionales del kernel"),
        "optimize_default_ttl" => Some("Establecer TTL óptimo predeterminado de paquetes IP"),
        "optimize_disk_io_page_lock_limit" => Some("Aumentar límite de bloqueo de páginas de E/S para transferencias grandes"),
        "optimize_dns_cache_parameters" => Some("Optimizar tamaño de caché y TTL del cliente DNS"),
        "optimize_gpu_preemption_level" => Some("Configurar programador de GPU para preferencia solo de cómputo"),
        "optimize_interrupt_steering" => Some("Optimizar afinidad de interrupciones y directiva de direccionamiento"),
        "optimize_mmcss_proaudio_scheduling" => Some("MMCSS Pro Audio — Máxima prioridad de programación"),
        "optimize_ntfs_memory_usage" => Some("Caché agresiva de grupo paginado NTFS"),
        "optimize_object_manager_quota" => Some("Aumentar cuota de identificadores del Administrador de objetos del kernel"),
        "optimize_timer_resolution_requests" => Some("Permitir que las aplicaciones soliciten alta resolución de temporizador"),
                "disable_driver_search_order_probing" => Some("Desactivar sondeo de búsqueda de controladores en Windows Update"),
        "disable_fast_user_switching" => Some("Desactivar cambio rápido de usuario"),
        "disable_ip_source_routing" => Some("Desactivar enrutamiento de origen IP"),
        "disable_netsh_teredo_tunneling" => Some("Desactivar tunelización de transición Teredo IPv6"),
        "disable_network_adapter_power_saving" => Some("Desactivar ahorro de energía PnP del adaptador de red NDIS"),
        "disable_remote_desktop_udp" => Some("Desactivar transporte UDP del cliente de Escritorio remoto"),
        "disable_smart_card_service" => Some("Desactivar servicio de tarjeta inteligente (SCardSvr)"),
        "optimize_kerberos_ticket_cache" => Some("Optimizar búfer de tamaño máximo de token Kerberos"),
        "optimize_tcp_syn_attack_protection" => Some("Habilitar protección contra ataques TCP SYN"),
        "optimize_usb_selective_suspend_hub" => Some("Desactivar suspensión selectiva global de concentradores USB"),
        _ => lookup(id).map(|t| t.title),
    }
}

/// Localized tweak description for this language.
#[allow(dead_code)]
pub fn tweak_description(id: &str) -> Option<&'static str> {
    match id {
        "disable_dwm_animations_per_process" => Some("Desactiva las animaciones de transición de DWM por proceso estableciendo DisallowAnimations=1 y AnimationsShiftKey=1 en la clave DWM. Elimina transiciones de desvanecimiento y deslizamiento al minimizar, maximizar o cambiar ventanas, reduciendo la sobrecarga del compositor de GPU."),
        "disable_dwm_composition_throttle" => Some("Establece DesktopCompositionThrottle=0 en la clave DWM, evitando que el compositor limite la representación cuando se pierden fotogramas. Permite que DWM mantenga el máximo rendimiento de composición bajo carga de GPU."),
        "disable_kernel_mitigations_audit" => Some("Desactiva el registro de auditoría de opciones de mitigación del kernel estableciendo MitigationAuditOptions en ceros. Reduce los eventos ETW en la creación de procesos y manejo de excepciones."),
        "disable_llmnr_protocol" => Some("Desactiva la resolución de nombres de multidifusión local de vínculo (LLMNR) estableciendo EnableMulticast=0 en los parámetros del cliente DNS. Reduce el tráfico de multidifusión innecesario y previene ataques de envenenamiento LLMNR."),
        "disable_memory_compression" => Some("Desactiva el almacén de compresión del Administrador de memoria de Windows estableciendo DisableCompression=1. En equipos con 16 GB o más de RAM, reduce los picos de uso de CPU causados por el subproceso de compresión del sistema."),
        "disable_netbios_over_tcp" => Some("Desactiva el protocolo NetBIOS a través de TCP/IP estableciendo NetbiosOptions=2 y deshabilitando la búsqueda LMHosts. NetBIOS es un protocolo LAN heredado que transmite consultas de resolución de nombres, exponiendo el sistema a ataques de envenenamiento NBNS."),
        "disable_ntfs_encryption_driver" => Some("Establece NtfsDisableEncryption=1 en la clave FileSystem, evitando que el controlador de filtro EFS intercepte operaciones de E/S. En equipos que no usan cifrado EFS por archivo, reduce la latencia de acceso a archivos."),
        "disable_page_file_encryption" => Some("Evita que Windows cifre el archivo de paginación y desactiva su borrado al apagar (ClearPageFileAtShutdown=0, DisablePagingEncryption=1). Mejora el rendimiento de paginación y acelera notablemente el tiempo de apagado."),
        "disable_processor_idle_demote_threshold" => Some("Establece IdleDemoteThreshold=100 e IdlePromoteThreshold=100 en la administración de energía del procesador. Evita que el administrador de energía degrade el rendimiento de la CPU en pausas breves de actividad, eliminando retrasos de escalado de frecuencia."),
        "disable_rss_base_cpu" => Some("Establece RssBaseCpu=1 en los parámetros NDIS, indicando al escalado en el lado de recepción que distribuya las interrupciones de red desde la CPU 1 en lugar de la CPU 0. Libera la CPU de arranque para subprocesos en primer plano y juegos."),
        "disable_spectre_meltdown_mitigations" => Some("Desactiva las mitigaciones de hardware para Spectre y Meltdown estableciendo FeatureSettingsOverride=3 y FeatureSettingsOverrideMask=3. Elimina la penalización de rendimiento en cargas de trabajo de computación pesada."),
        "disable_threaded_dpc_optimization" => Some("Establece ThreadDpcEnable=0 en el kernel, forzando la ejecución de llamadas a procedimientos diferidos en DISPATCH_LEVEL en lugar de subprocesos PASSIVE_LEVEL. Reduce la latencia DPC para producción de audio en tiempo real y juegos competitivos."),
        "disable_windows_search_indexer" => Some("Desactiva el servicio WSearch, impidiendo que el indizador rastree continuamente los archivos en segundo plano. En sistemas SSD/NVMe, elimina el uso constante de disco, CPU y memoria de SearchIndexer.exe."),
        "enable_hardware_task_offload" => Some("Establece DisableTaskOffload=0 en TCP/IP, garantizando que el sistema operativo descargue el cálculo de suma de comprobación y la segmentación TCP en el hardware de la tarjeta de red. Reduce el uso de CPU."),
        "enable_large_ntfs_frs" => Some("Establece NtfsAllowExtendedCharacterIn8dot3Name=0, permitiendo que el controlador NTFS asigne segmentos de registro de archivos más grandes y eficientes, reduciendo la fragmentación de la MFT."),
        "enable_ultimate_performance_plan" => Some("Activa el plan de energía oculto de Máximo Rendimiento. Elimina micro-latencias de ahorro de energía: sin estacionamiento de núcleos, sin demoras de escala de frecuencia, sin suspensión selectiva USB ni apagado de discos."),
        "increase_gpu_tdr_timeout" => Some("Aumenta TdrDelay de 2 a 10 segundos, otorgando a la GPU más tiempo para completar sombreadores complejos antes de que Windows reinicie el controlador. Evita reinicios TDR falsos en computación gráfica pesada."),
        "increase_irp_stack_size" => Some("Aumenta el tamaño de la pila de paquetes de solicitud de E/S de 15 a 32 entradas en LanmanServer. Evita errores STATUS_INSUFFICIENT_RESOURCES y mejora el rendimiento con antivirus o filtros de almacenamiento."),
        "increase_max_connections_per_server" => Some("Aumenta MaxConnectionsPerServer y MaxConnectionsPer1_0Server a 16, permitiendo que las aplicaciones basadas en WinINet abran más conexiones HTTP simultáneas por host, acelerando descargas y transferencias."),
        "increase_nonpaged_pool_quota" => Some("Establece NonPagedPoolQuota en 0 (máximo administrado por el sistema), permitiendo al administrador de memoria usar toda la memoria física para asignaciones no paginadas, evitando el agotamiento de recursos."),
        "optimize_additional_critical_worker_threads" => Some("Establece AdditionalCriticalWorkerThreads en 2 y AdditionalDelayedWorkerThreads en 2, aprovisionando subprocesos de trabajo adicionales del kernel para evitar bloqueos en colas de trabajo con alta E/S."),
        "optimize_default_ttl" => Some("Establece el valor predeterminado de tiempo de vida (TTL) de IP en 64 saltos, coincidiendo con el estándar de Linux/macOS. 64 saltos es suficiente para prácticamente todas las topologías de enrutamiento."),
        "optimize_disk_io_page_lock_limit" => Some("Establece IoPageLockLimit en 983040 bytes (960 KB), aumentando la cantidad de memoria que el administrador de E/S puede bloquear para transferencias directas en unidades NVMe rápidas."),
        "optimize_dns_cache_parameters" => Some("Establece MaxCacheEntryTtlLimit=86400 y MaxNegativeCacheTtl=5 en Dnscache, almacenando en caché búsquedas exitosas por más tiempo y reintentando rápidamente las fallidas."),
        "optimize_gpu_preemption_level" => Some("Establece GpuPreemptionLevel=0 en el programador de controladores gráficos. Evita que el programador interrumpa sombreadores en ejecución, reduciendo microtirones y sobrecarga de contexto en juegos."),
        "optimize_interrupt_steering" => Some("Configura la directiva de direccionamiento de interrupciones del kernel estableciendo InterruptSteeringDisabled=0 y DistributeTimers=1, distribuyendo interrupciones de hardware entre todos los núcleos."),
        "optimize_mmcss_proaudio_scheduling" => Some("Establece la categoría de programación y prioridad SFIO de la tarea Pro Audio de MMCSS en 'High', garantizando la máxima prioridad para subprocesos de audio en DAWs y aplicaciones de comunicación."),
        "optimize_ntfs_memory_usage" => Some("Establece NtfsMemoryUsage en 2, indicando al controlador NTFS que use una asignación de grupo paginado más grande para almacenar en caché directorios y metadatos, mejorando el rendimiento de compilación."),
        "optimize_object_manager_quota" => Some("Aumenta el límite de la tabla de identificadores mediante SessionPoolSize=48 en Memory Management. Evita el agotamiento de identificadores en sistemas con numerosos procesos concurrentes e IDEs."),
        "optimize_timer_resolution_requests" => Some("Establece GlobalTimerResolutionRequests=1 en el kernel, permitiendo a las aplicaciones solicitar la máxima resolución de temporizador (0,5 ms) en todo el sistema, eliminando fluctuaciones de sincronización."),
                "disable_driver_search_order_probing" => Some("Evita que Windows Update consulte automáticamente los servidores de actualización en segundo plano al buscar controladores. Elimina la latencia de instalación de dispositivos y sondeos de red."),
        "disable_fast_user_switching" => Some("Desactiva el cambio rápido de usuario para evitar que las sesiones inactivas y sus tareas en segundo plano permanezcan cargadas en la RAM física, liberando ancho de banda y CPU."),
        "disable_ip_source_routing" => Some("Establece DisableIPSourceRouting=2 para descartar paquetes IP con enrutamiento de origen entrantes. Evita la suplantación de paquetes, el secuestro de rutas y la sobrecarga de la pila de red."),
        "disable_netsh_teredo_tunneling" => Some("Desactiva la interfaz de túnel Teredo configurando DisabledComponents en 32 en IPv6. Teredo encapsula IPv6 en UDP IPv4, generando latencia adicional de cruce NAT."),
        "disable_network_adapter_power_saving" => Some("Configura NDIS PnPCapabilities en 24, desactivando estados de bajo consumo en adaptadores de red. Evita que las tarjetas de red entren en reposo en pausas de juego, eliminando retrasos."),
        "disable_remote_desktop_udp" => Some("Obliga al cliente RDP a utilizar TCP en lugar de UDP. Elimina la congestión del búfer de sockets UDP y evita que el tráfico RDP compita con los paquetes de juegos UDP."),
        "disable_smart_card_service" => Some("Desactiva el servicio de tarjetas inteligentes (SCardSvr) estableciendo Start=4. Elimina el sondeo en segundo plano de lectores de tarjetas y tokens en equipos de juego."),
        "optimize_kerberos_ticket_cache" => Some("Establece el búfer MaxTokenSize de Kerberos en 65535 bytes, evitando desbordamientos de búfer y renegociaciones continuas de tickets en redes con grupos de seguridad complejos."),
        "optimize_tcp_syn_attack_protection" => Some("Habilita la defensa contra inundaciones TCP SYN en la pila TCP/IP (SynAttackProtect=1). Acelera los tiempos de espera de protocolo de enlace protegiendo los sockets de juegos."),
        "optimize_usb_selective_suspend_hub" => Some("Desactiva la suspensión selectiva USB a nivel de controlador raíz (DisableSelectiveSuspend=1). Evita que ratones, teclados y DACs entren en reposo, garantizando latencia cero de entrada."),
        _ => lookup(id).map(|t| t.description),
    }
}
