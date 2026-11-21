use super::LocalizedTweakText;

pub fn lookup(id: &str) -> Option<LocalizedTweakText> {
    match id {
        // === ai_privacy_ext.toml (21) ===
        "disable_windows_ai_recall_extended" => Some(LocalizedTweakText {
            title: "Désactiver Windows Recall, Click to Do et les agents IA Copilot+",
            description: "Bloque la capture d'instantanés Windows Recall, Click to Do, l'agent IA des Paramètres, les connecteurs/espaces de travail d'agents IA et Copilot Runtime via les clés de stratégie de groupe introduites dans Windows 11 24H2+.",
            broken_features: &["Instantanés de chronologie Windows Recall et superposition IA Click to Do"],
        }),
        "disable_generative_ai_app_access" => Some(LocalizedTweakText {
            title: "Bloquer l'accès des applications aux modèles d'IA générative et système",
            description: "Refuse aux applications UWP/WinUI en arrière-plan l'accès aux capacités locales d'IA générative et aux modèles d'IA système via les stratégies AppPrivacy et CapabilityAccessManager ConsentStore.",
            broken_features: &["Les applications tierces ne peuvent plus interroger les points de terminaison des modèles d'IA Windows intégrés"],
        }),
        "disable_copilot_shell_and_hardware_key" => Some(LocalizedTweakText {
            title: "Désactiver le bouton Copilot de la barre des tâches, les incitations, la touche dédiée et le micro",
            description: "Masque le bouton et les notifications Copilot dans la barre des tâches, neutralise l'affectation de la touche clavier dédiée Copilot, bloque les promotions IA liées au compte et refuse l'accès au microphone pour Copilot et OfficeHub.",
            broken_features: &[
                "La touche dédiée Copilot ne lance plus Copilot",
                "Entrée vocale de Copilot bloquée",
            ],
        }),
        "disable_paint_ai_features" => Some(LocalizedTweakText {
            title: "Désactiver les fonctions cloud et d'IA générative de Microsoft Paint",
            description: "Désactive Image Creator, Cocreator, Remplissage génératif, Effacement génératif et Suppression d'arrière-plan par IA dans Microsoft Paint sous Windows 11 via les clés de stratégie officielles.",
            broken_features: &["Outils de génération par IA masqués dans Microsoft Paint"],
        }),
        "disable_edge_ai_policies" => Some(LocalizedTweakText {
            title: "Désactiver Copilot, la barre latérale et les API d'IA intégrées de Microsoft Edge",
            description: "Désactive 12 stratégies d'IA dans Microsoft Edge, dont le contexte de page Copilot, la barre latérale Hubs, Entra Copilot, la rédaction en ligne, la recherche IA dans l'historique, les modèles locaux, DevTools GenAI et le partage d'historique.",
            broken_features: &["Barre latérale Copilot d'Edge et aide à la rédaction en ligne désactivées"],
        }),
        "disable_office_ai_copilot" => Some(LocalizedTweakText {
            title: "Désactiver l'entraînement IA de Microsoft Office et les Copilots d'applications",
            description: "Désactive l'entraînement IA de Microsoft 365, l'analyse du contenu connecté, le service de sécurité du contenu et les commutateurs Copilot par application dans Word, Excel et OneNote.",
            broken_features: &["Assistant Copilot dans Word, Excel et OneNote désactivé"],
        }),
        "disable_wmi_telemetry_autologgers" => Some(LocalizedTweakText {
            title: "Désactiver 10 sessions AutoLogger de télémétrie noyau WMI",
            description: "Empêche Windows de lancer des sessions persistantes de suivi ETW AutoLogger en arrière-plan (Diagtrack-Listener, SQMLogger, DiagLog, Cellcore, DataMarket, AppModel, CloudExperienceHostOobe, LwtNetLog, WdiContextLog, WiFiSession).",
            broken_features: &["Journaux de trace diagnostique noyau ETW désactivés"],
        }),
        "block_silent_preinstalled_apps" => Some(LocalizedTweakText {
            title: "Bloquer les applications préinstallées silencieusement et les suggestions Démarrer/Paramètres",
            description: "Empêche ContentDeliveryManager d'installer silencieusement des applications promotionnelles OEM ou Microsoft Store et masque le contenu suggéré dans Paramètres, le menu Démarrer et la Chronologie.",
            broken_features: &[],
        }),
        "disable_extra_telemetry_policies" => Some(LocalizedTweakText {
            title: "Renforcer la protection contre les commentaires, l'inventaire AppCompat, les invites OOBE et le suivi",
            description: "Plafonne MaxTelemetryAllowed=0, supprime les invites de commentaires Windows SIUF (supprime PeriodInNanoSeconds), désactive l'inventaire AppCompat/UAR, Localiser mon appareil, les écrans de bienvenue/fin de configuration, le suivi des lancements d'applications, Workplace Join et l'Assistance à distance.",
            broken_features: &[
                "Invitations d'Assistance à distance désactivées",
                "Suivi des applications fréquentes/récentes dans le menu Démarrer désactivé",
            ],
        }),
        "disable_typing_and_inking_telemetry" => Some(LocalizedTweakText {
            title: "Désactiver la télémétrie d'écriture manuscrite, de saisie et d'analyse d'entrée",
            description: "Restreint la collecte implicite des frappes clavier et du stylet, la collecte de contacts pour les dictionnaires de saisie personnalisés, la journalisation diagnostique linguistique (TIPC) et Windows Input Insights.",
            broken_features: &["Dictionnaire personnalisé de prédiction de saisie et d'écriture manuscrite désactivé"],
        }),
        "disable_search_cloud_and_history" => Some(LocalizedTweakText {
            title: "Désactiver les temps forts de recherche Windows, la recherche cloud et l'historique de l'appareil",
            description: "Désactive les temps forts de recherche Windows (zone de recherche dynamique), la recherche de contenu cloud MSA/AAD et l'historique local des recherches sans désactiver le service d'indexation.",
            broken_features: &["Illustration quotidienne des temps forts de recherche et résultats cloud OneDrive masqués"],
        }),
        "disable_wifi_sense_and_location_extras" => Some(LocalizedTweakText {
            title: "Désactiver la connexion automatique WiFi-Sense, les mises à jour Cartes et ConsentStore de localisation",
            description: "Désactive la connexion automatique aux points d'accès Wi-Fi (WcmSvc\\AutoConnectAllowedOEM=0), les mises à jour automatiques hors ligne de Cartes Windows et l'autorisation globale de localisation ConsentStore.",
            broken_features: &["Téléchargements automatiques de Cartes Windows et invites de localisation système des applications"],
        }),
        "block_onedrive_kfm_auto_backup" => Some(LocalizedTweakText {
            title: "Bloquer le déplacement automatique des dossiers connus OneDrive (Bureau/Documents)",
            description: "Définit KFMBlockOptIn=1 afin qu'OneDrive ne puisse pas détourner silencieusement les dossiers Bureau, Documents et Images vers le cloud. Sûr si vous souhaitez un stockage local ou une synchronisation manuelle.",
            broken_features: &["Assistant automatique de déplacement des dossiers connus OneDrive désactivé"],
        }),
        "disable_bitlocker_auto_encryption" => Some(LocalizedTweakText {
            title: "Désactiver le chiffrement automatique d'appareil BitLocker de Windows",
            description: "Définit PreventDeviceEncryption=1 afin que Windows ne chiffre pas silencieusement les lecteurs lors de la connexion à un compte Microsoft ou d'une nouvelle installation sans sauvegarde préalable des clés de récupération.",
            broken_features: &["Le chiffrement automatique silencieux des lecteurs BitLocker ne s'activera pas"],
        }),
        "disable_extra_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Désactiver les tâches planifiées supplémentaires de télémétrie, file WER, SQM, énergie et Cartes",
            description: "Désactive des tâches planifiées supplémentaires de diagnostic et d'arrière-plan : QueueReporting, Sqm-Tasks, MareBackup, StartupAppTask, PcaPatchDbTask, MapsUpdateTask, AnalyzeSystem et FamilySafetyMonitor.",
            broken_features: &["Rapport automatique d'efficacité énergétique et mises à jour de cartes hors ligne désactivés"],
        }),
        "disable_windows_ai_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Désactiver les tâches planifiées en arrière-plan de Windows AI Recall",
            description: "Désactive les tâches planifiées de configuration Windows Recall et de pipeline d'apprentissage automatique en arrière-plan introduites dans Windows 11 24H2+.",
            broken_features: &["Tâches d'indexation en arrière-plan de Windows Recall désactivées"],
        }),
        "disable_office_actions_scheduled_task" => Some(LocalizedTweakText {
            title: "Désactiver la tâche du serveur d'actions IA Microsoft Office",
            description: "Désactive la tâche planifiée en arrière-plan Microsoft Office Actions Server pour réduire les réveils CPU/mémoire liés à l'IA d'Office.",
            broken_features: &[],
        }),
        "disable_optional_bloat_services" => Some(LocalizedTweakText {
            title: "Désactiver les services Windows optionnels inutilisés (Cartes, Fax, Démo, Portefeuille, tzautoupdate)",
            description: "Désactive MapsBroker, RetailDemo, Fax, WalletService, WMPNetworkSvc, AJRouter, shpamsvc et tzautoupdate sans toucher aux services essentiels du système ou de Windows Update.",
            broken_features: &["Télécopie Windows, courtier de cartes hors ligne, démo revendeur, partage WMP et fuseau horaire auto désactivés"],
        }),
        "disable_xbox_live_services" => Some(LocalizedTweakText {
            title: "Désactiver les services d'arrière-plan Xbox Live (XblAuthManager, GameSave, Networking)",
            description: "Désactive les services d'arrière-plan Xbox Live Auth Manager, Game Save et Networking pour les utilisateurs ne jouant pas aux titres de l'application Xbox ou du Game Pass.",
            broken_features: &["La connexion Xbox Live et les sauvegardes cloud Xbox Game Pass / Microsoft Store cessent de fonctionner"],
        }),
        "disable_ai_fabric_service" => Some(LocalizedTweakText {
            title: "Désactiver le service Windows AI Fabric (WSAIFabricSvc)",
            description: "Désactive le service d'arrière-plan d'orchestration IA locale WSAIFabricSvc sur Windows 11 24H2+.",
            broken_features: &["Service d'orchestration IA Copilot+ arrêté"],
        }),
        "deny_global_camera_mic_diagnostics" => Some(LocalizedTweakText {
            title: "Bloquer l'accès global des applications à la caméra, au micro, aux infos de compte et aux diagnostics [Opt-In]",
            description: "Refuse globalement les autorisations ConsentStore pour la webcam, le microphone, userAccountInformation et appDiagnostics sur toutes les applications bureau et UWP. Risque élevé d'interruption : désactive caméra et micro dans Zoom/Discord/OBS jusqu'à annulation.",
            broken_features: &["Désactive la caméra et le microphone dans TOUTES les applications jusqu'à réautorisation"],
        }),

        // === bloatware_appx.toml (26) ===
        "remove_bing_weather" => Some(LocalizedTweakText {
            title: "Supprimer Météo Bing",
            description: "Supprime l'application UWP Microsoft Bing Météo. La météo reste accessible via un navigateur.",
            broken_features: &["L'application Météo Bing sera supprimée"],
        }),
        "remove_bing_news" => Some(LocalizedTweakText {
            title: "Supprimer Bing Actualités",
            description: "Supprime l'application UWP Microsoft Bing Actualités (Microsoft News).",
            broken_features: &["L'application Bing Actualités / Microsoft News sera supprimée"],
        }),
        "remove_get_help" => Some(LocalizedTweakText {
            title: "Supprimer Obtenir de l'aide",
            description: "Supprime l'application UWP Microsoft Obtenir de l'aide (Get Help). L'assistance reste accessible sur le Web.",
            broken_features: &["L'application Obtenir de l'aide sera supprimée"],
        }),
        "remove_getstarted_tips" => Some(LocalizedTweakText {
            title: "Supprimer Astuces (Bien démarrer)",
            description: "Supprime l'application UWP Microsoft Astuces / Bien démarrer.",
            broken_features: &["L'application Astuces / Bien démarrer sera supprimée"],
        }),
        "remove_solitaire" => Some(LocalizedTweakText {
            title: "Supprimer Microsoft Solitaire Collection",
            description: "Supprime l'application UWP Microsoft Solitaire Collection, qui contient des publicités et de la télémétrie.",
            broken_features: &["Solitaire Collection sera supprimée"],
        }),
        "remove_people" => Some(LocalizedTweakText {
            title: "Supprimer l'application Contacts (People)",
            description: "Supprime l'application UWP Microsoft Contacts (People).",
            broken_features: &["L'application Contacts sera supprimée"],
        }),
        "remove_windows_maps" => Some(LocalizedTweakText {
            title: "Supprimer Cartes Windows",
            description: "Supprime l'application UWP Microsoft Cartes Windows.",
            broken_features: &["L'application Cartes Windows sera supprimée"],
        }),
        "remove_groove_music" => Some(LocalizedTweakText {
            title: "Supprimer Groove Musique (Zune Music)",
            description: "Supprime l'application UWP Groove Musique (ZuneMusic). Le Lecteur multimédia ou des lecteurs tiers peuvent être utilisés à la place.",
            broken_features: &["L'application Groove Musique sera supprimée"],
        }),
        "remove_zune_video" => Some(LocalizedTweakText {
            title: "Supprimer Films et TV (Zune Video)",
            description: "Supprime l'application UWP Films et TV (ZuneVideo). VLC ou d'autres lecteurs peuvent être utilisés à la place.",
            broken_features: &["L'application Films et TV sera supprimée"],
        }),
        "remove_clipchamp" => Some(LocalizedTweakText {
            title: "Supprimer Clipchamp",
            description: "Supprime l'éditeur vidéo UWP Clipchamp préinstallé sur Windows 11.",
            broken_features: &["L'éditeur vidéo Clipchamp sera supprimé"],
        }),
        "remove_todos" => Some(LocalizedTweakText {
            title: "Supprimer Microsoft To Do",
            description: "Supprime l'application UWP Microsoft To Do.",
            broken_features: &["L'application Microsoft To Do sera supprimée"],
        }),
        "remove_bing_search_appx" => Some(LocalizedTweakText {
            title: "Supprimer l'extension AppX Recherche Microsoft Bing",
            description: "Supprime le paquet Store Microsoft.BingSearch qui alimente les widgets Web et les intégrations Web de la recherche Windows.",
            broken_features: &["Intégration AppX Recherche Bing supprimée"],
        }),
        "remove_feedback_hub" => Some(LocalizedTweakText {
            title: "Supprimer le Hub de commentaires Microsoft",
            description: "Supprime l'application préinstallée de télémétrie et de sondages Hub de commentaires Windows.",
            broken_features: &["Application Hub de commentaires supprimée"],
        }),
        "remove_copilot_appx" => Some(LocalizedTweakText {
            title: "Supprimer le paquet UWP Microsoft Copilot",
            description: "Désinstalle le paquet autonome Microsoft Store Microsoft.Copilot.",
            broken_features: &["Application Copilot supprimée"],
        }),
        "remove_edge_game_assist" => Some(LocalizedTweakText {
            title: "Supprimer la superposition Microsoft Edge Game Assist",
            description: "Supprime le paquet AppX de superposition de navigateur en jeu Edge Game Assist.",
            broken_features: &["Widget Edge Game Assist supprimé"],
        }),
        "remove_dev_home" => Some(LocalizedTweakText {
            title: "Supprimer l'application Windows Dev Home",
            description: "Supprime le paquet de tableau de bord Microsoft.Windows.DevHome préinstallé sur Windows 11.",
            broken_features: &["Tableau de bord Dev Home supprimé"],
        }),
        "remove_power_automate_desktop" => Some(LocalizedTweakText {
            title: "Supprimer Microsoft Power Automate Desktop",
            description: "Supprime le paquet d'automatisation RPA Microsoft Power Automate Desktop.",
            broken_features: &["Application de flux de bureau Power Automate supprimée"],
        }),
        "remove_office_hub" => Some(LocalizedTweakText {
            title: "Supprimer Microsoft 365 Copilot (OfficeHub)",
            description: "Supprime le lanceur promotionnel préinstallé Microsoft 365 Office Hub.",
            broken_features: &["Lanceur promotionnel Office supprimé (ne désinstalle pas Office bureau)"],
        }),
        "remove_family_safety" => Some(LocalizedTweakText {
            title: "Supprimer l'application Microsoft Family Safety",
            description: "Supprime l'application UWP grand public Microsoft Family Safety.",
            broken_features: &["Application UWP Family Safety supprimée"],
        }),
        "remove_3d_viewer_and_mixed_reality" => Some(LocalizedTweakText {
            title: "Supprimer Visionneuse 3D et Portail de réalité mixte",
            description: "Supprime les paquets hérités Microsoft Visionneuse 3D et Portail de réalité mixte.",
            broken_features: &["Ancienne Visionneuse 3D et Portail de réalité mixte Windows supprimés"],
        }),
        "remove_outlook_for_windows" => Some(LocalizedTweakText {
            title: "Supprimer le nouvel Outlook pour Windows (client WebView)",
            description: "Supprime l'application conteneur Web préinstallée Microsoft.OutlookForWindows.",
            broken_features: &["Nouveau client UWP Outlook supprimé"],
        }),
        "remove_your_phone_link" => Some(LocalizedTweakText {
            title: "Supprimer Mobile connecté (YourPhone)",
            description: "Désinstalle l'application compagnon de synchronisation mobile Microsoft.YourPhone.",
            broken_features: &["Synchronisation bureau Android/iOS Mobile connecté supprimée"],
        }),
        "remove_sound_recorder_and_sticky_notes" => Some(LocalizedTweakText {
            title: "Supprimer Enregistreur vocal Windows et Pense-bêtes",
            description: "Supprime les paquets Microsoft Store Enregistreur vocal et Pense-bêtes (Sticky Notes).",
            broken_features: &["Applications intégrées Enregistreur vocal et Pense-bêtes supprimées"],
        }),
        "remove_ms_teams_and_quick_assist" => Some(LocalizedTweakText {
            title: "Supprimer Microsoft Teams grand public et Assistance rapide",
            description: "Supprime l'application de messagerie personnelle MSTeams grand public et l'outil d'assistance Assistance rapide.",
            broken_features: &["Teams personnel et outil d'assistance Assistance rapide supprimés"],
        }),
        "disable_recall_optional_feature" => Some(LocalizedTweakText {
            title: "Désinstaller la fonctionnalité facultative Windows Recall (DISM)",
            description: "Désactive proprement le paquet de fonctionnalité Windows facultative 'Recall' au niveau de la maintenance du système d'exploitation.",
            broken_features: &["Binaires de la fonctionnalité Windows Recall désactivés proprement"],
        }),
        "remove_legacy_steps_recorder_ie_wordpad" => Some(LocalizedTweakText {
            title: "Désactiver l'Enregistreur d'actions utilisateur hérité et Internet Explorer 11",
            description: "Supprime l'Enregistreur d'actions utilisateur hérité (App.StepsRecorder) et la capacité mode Internet Explorer via DISM.",
            broken_features: &["Enregistreur d'actions (psr.exe) et mode IE11 supprimés"],
        }),

        // === hardware_tuning.toml (16) ===
        "enable_hags" => Some(LocalizedTweakText {
            title: "Activer la planification GPU à accélération matérielle (HAGS)",
            description: "Active la planification GPU à accélération matérielle, permettant au GPU de gérer directement sa propre VRAM. Réduit la latence et la charge CPU sur les tâches liées au GPU. Nécessite un GPU compatible (NVIDIA GTX 1000+, AMD RX 5000+) et un pilote adapté.",
            broken_features: &["Certains jeux ou utilitaires GPU anciens peuvent présenter une instabilité"],
        }),
        "disable_mpo" => Some(LocalizedTweakText {
            title: "Désactiver la superposition multi-plan (MPO)",
            description: "Désactive la fonction matérielle de composition Multi-Plane Overlay. Le MPO est connu pour provoquer des saccades, des écrans noirs et des irrégularités d'images sur de nombreuses combinaisons GPU/pilotes, notamment avec plusieurs écrans à taux de rafraîchissement mixtes.",
            broken_features: &["Consommation électrique du compositeur GPU légèrement plus élevée sur ordinateur portable"],
        }),
        "disable_core_parking" => Some(LocalizedTweakText {
            title: "Désactiver le stationnement des cœurs CPU (Core Parking)",
            description: "Fixe à 100 % le pourcentage minimal de cœurs processeur devant rester actifs afin que tous les cœurs soient toujours disponibles. Élimine la latence de réveil des cœurs au prix d'une consommation au repos plus élevée.",
            broken_features: &[
                "Consommation électrique plus élevée au repos",
                "Autonomie réduite sur les ordinateurs portables",
            ],
        }),
        "disable_cstates" => Some(LocalizedTweakText {
            title: "Désactiver les états d'inactivité C-States du CPU",
            description: "Empêche le processeur d'entrer dans des états de veille profonde C-States (C6/C7/C8) en plaçant la stratégie d'inactivité en mode performance. Réduit les pics de latence lors des transitions d'état d'alimentation, au prix d'une consommation au repos accrue.",
            broken_features: &[
                "Consommation électrique nettement plus élevée au repos",
                "Autonomie réduite sur les ordinateurs portables",
                "Températures CPU plus élevées au repos",
            ],
        }),
        "gpu_shader_cache_size" => Some(LocalizedTweakText {
            title: "Allonger le délai TDR du GPU et prévenir les réinitialisations du pilote",
            description: "Augmente l'intervalle WDDM Timeout Detection and Recovery (TdrDelay) de 2s à 10s, évitant les réinitialisations intempestives du pilote GPU et les saccades lors de compilations lourdes de shaders DirectX.",
            broken_features: &["La récupération du pilote d'affichage attend jusqu'à 10s si un véritable blocage GPU survient"],
        }),
        "enable_game_mode" => Some(LocalizedTweakText {
            title: "Activer le mode Jeu de Windows",
            description: "Garantit l'activation du mode Jeu de Windows (AutoGameModeEnabled=1, AllowAutoGameMode=1), utile s'il a été désactivé, en priorisant les processus de jeu et en bloquant les notifications Windows Update et de redémarrage pendant les parties.",
            broken_features: &[],
        }),
        "disable_fullscreen_optimizations" => Some(LocalizedTweakText {
            title: "Désactiver globalement les optimisations plein écran",
            description: "Désactive globalement les optimisations plein écran de Windows (mode hybride sans bordure) pour toutes les applications. Force le véritable plein écran exclusif, pouvant réduire la latence d'entrée et la variance des temps d'image dans les anciens jeux.",
            broken_features: &[
                "Le retour Alt-Tab depuis les jeux en plein écran sera plus lent",
                "La superposition Game Bar ne fonctionnera pas en plein écran exclusif",
            ],
        }),
        "4433ce93-b632-4d67-ae71-0241c31099c0" => Some(LocalizedTweakText {
            title: "Désactiver AMD ULPS",
            description: "Désactive Ultra Low Power State (ULPS) dans la pile de pilotes AMD Radeon. Empêche les cœurs GPU secondaires ou états de veille de se réveiller avec des micro-saccades et pics de latence d'image en jeu 3D.",
            broken_features: &["Consommation au repos légèrement plus élevée sur les portables AMD multi-GPU / Crossfire"],
        }),
        "8a4b4c4d-000e-42db-8bd7-293067f8fae7" => Some(LocalizedTweakText {
            title: "Désactiver AMD ASPM",
            description: "Désactive la gestion active de l'alimentation PCIe (ASPM) pour les cartes graphiques AMD. Empêche le lien PCIe de passer en états basse consommation L0s/L1 entre les rendus, réduisant la gigue de transmission PCIe.",
            broken_features: &["Consommation électrique accrue du bus PCIe au repos"],
        }),
        "a3836587-c461-4beb-9c4c-88493b36c138" => Some(LocalizedTweakText {
            title: "Désactiver AMD Power Gating",
            description: "Désactive le Power Gating dynamique agressif sur les unités de calcul et moteurs de rendu AMD (DisablePowerGating=1, PP_GPUPowerDownEnabled=0). Maintient des fréquences d'horloge stables sans bridage.",
            broken_features: &["Températures de la puce GPU plus élevées à faible charge"],
        }),
        "db379bb5-fc8c-4ad3-be32-11be09e76fba" => Some(LocalizedTweakText {
            title: "Désactiver AMD Video Clock Gating",
            description: "Empêche le découpage d'horloge et d'alimentation sur les moteurs multimédias et d'affichage matériel AMD VCE/UVD (DisableVCEPowerGating=1, DisableVceClockGating=1, EnableUvdClockGating=0, EnableVceSwClockGating=0). Élimine les pics de latence DPC.",
            broken_features: &[],
        }),
        "df8739ff-e322-4958-b369-df584bb9b0c1" => Some(LocalizedTweakText {
            title: "Désactiver Intel Async Flips",
            description: "Définit Display1_DisableAsyncFlips=1 dans les clés de classe d'adaptateur graphique Intel pour imposer des basculements d'affichage synchronisés et éliminer la gigue de file de présentation.",
            broken_features: &[],
        }),
        "ebfb0ded-5d1a-4cd1-b5c0-4f7018dd6054" => Some(LocalizedTweakText {
            title: "Désactiver Intel Adaptive V-Sync",
            description: "Définit AdaptiveVsyncEnable=0 dans les clés de classe d'adaptateur graphique Intel pour éviter le bridage adaptatif de la fréquence lorsque le framerate passe sous le taux du moniteur.",
            broken_features: &[],
        }),
        "fa58802f-b98c-4e98-aa67-067c5a394296" => Some(LocalizedTweakText {
            title: "Désactiver NVIDIA Dynamic P-State",
            description: "Définit DisableDynamicPstate=1 pour verrouiller le GPU NVIDIA sur un état de performance fixe et éviter les fluctuations de fréquence. Augmente la consommation au repos.",
            broken_features: &["Augmente la consommation et la chaleur du GPU au repos"],
        }),
        "9e7702be-9e03-4c96-a316-c4e435cb993e" => Some(LocalizedTweakText {
            title: "Désactiver NVIDIA Async P-States",
            description: "Définit DisableAsyncPstates=1 pour empêcher les transitions asynchrones d'état d'alimentation NVIDIA, offrant des performances déterministes pour la VR et les applications temps réel.",
            broken_features: &["Peut empêcher la baisse de fréquence de la mémoire GPU pendant la lecture vidéo"],
        }),
        "5f1a2b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c" => Some(LocalizedTweakText {
            title: "Activer les performances maximales NVIDIA PowerMizer",
            description: "Force le mode Performances maximales PowerMizer sur secteur, verrouillant le bus PCIe et les contrôleurs mémoire à pleine vitesse sans limitation.",
            broken_features: &["Températures GPU et vitesse des ventilateurs au repos plus élevées"],
        }),

        // === os_settings.toml (21) ===
        "disable_animations" => Some(LocalizedTweakText {
            title: "Désactiver les animations des fenêtres",
            description: "Désactive l'effet visuel d'animation lors de la réduction et de l'agrandissement des fenêtres, diminuant la latence perçue de l'interface.",
            broken_features: &[],
        }),
        "disable_transparency" => Some(LocalizedTweakText {
            title: "Désactiver les effets de transparence",
            description: "Désactive l'effet acrylique/transparence sur la barre des tâches, le menu Démarrer et le centre de notifications, libérant la charge de composition GPU.",
            broken_features: &["Apparence transparente de la barre des tâches et du menu Démarrer"],
        }),
        "disable_ntfs_last_access" => Some(LocalizedTweakText {
            title: "Désactiver globalement les applications Windows en arrière-plan",
            description: "Empêche les applications UWP / Microsoft Store de se réveiller, d'interroger le réseau et de consommer silencieusement CPU/RAM en arrière-plan lorsqu'elles sont réduites ou fermées.",
            broken_features: &["Les applications du Store cessent de se synchroniser en arrière-plan une fois fermées"],
        }),
        "disable_nagle_algorithm" => Some(LocalizedTweakText {
            title: "Désactiver l'algorithme de Nagle pour MSMQ (TCPNoDelay)",
            description: "Désactive l'algorithme de Nagle spécifiquement pour Microsoft Message Queuing (MSMQ\\Parameters\\TCPNoDelay), réduisant la latence de file d'attente pour les petits messages MSMQ.",
            broken_features: &["Peut augmenter légèrement l'utilisation de la bande passante réseau pour les petits paquets"],
        }),
        "disable_sticky_keys" => Some(LocalizedTweakText {
            title: "Désactiver les touches rémanentes et raccourcis d'accessibilité",
            description: "Désactive la boîte de dialogue des touches rémanentes (5x Maj), les touches filtres et les bips des touches bascules afin d'éviter les fenêtres intempestives et interruptions en jeu compétitif.",
            broken_features: &["Raccourcis d'accessibilité (5x Maj, 8s Maj droit) désactivés"],
        }),
        "disable_mouse_shadow" => Some(LocalizedTweakText {
            title: "Désactiver les ombres portées de l'Explorateur et la transparence alpha de sélection",
            description: "Désactive les ombres portées des étiquettes d'icônes du bureau et le mélange alpha translucide du rectangle de sélection dans l'Explorateur, réduisant le travail de composition GPU du DWM.",
            broken_features: &[],
        }),
        "high_performance_power_plan" => Some(LocalizedTweakText {
            title: "Définir le mode d'accélération processeur agressif",
            description: "Définit le mode d'accélération des performances processeur sur secteur sur Agressif via PowerSchemes PerfBoostMode, évitant la baisse des fréquences turbo sous charge interactive.",
            broken_features: &[
                "Consommation électrique plus élevée au repos",
                "Dégagement de chaleur accru sur les ordinateurs portables",
            ],
        }),
        "disable_bing_search_start" => Some(LocalizedTweakText {
            title: "Désactiver la recherche Web Bing dans le menu Démarrer",
            description: "Empêche la recherche Windows d'envoyer chaque frappe aux serveurs Bing, rendant la recherche locale d'applications et de fichiers dans le menu Démarrer instantanée sans délai cloud.",
            broken_features: &["Les résultats de recherche Web n'apparaîtront plus dans le menu Démarrer"],
        }),
        "disable_windows_copilot" => Some(LocalizedTweakText {
            title: "Désactiver l'intégration IA de Windows Copilot",
            description: "Désactive Windows Copilot à l'échelle du système via stratégie de groupe, arrêtant les agents résidents en arrière-plan, les boutons de la barre des tâches et les requêtes réseau IA.",
            broken_features: &["La barre latérale Windows Copilot et le raccourci (Win+C) seront désactivés"],
        }),
        "disable_game_bar_presence_writer" => Some(LocalizedTweakText {
            title: "Désactiver la présence Game Bar, le lanceur Nexus et les astuces de démarrage",
            description: "Désactive le lancement Game Bar par la touche Nexus de manette et les panneaux d'invite de démarrage via les clés HKCU\\Software\\Microsoft\\GameBar modifiables par l'utilisateur, éliminant la surcharge des hooks sans erreur ACL TrustedInstaller.",
            broken_features: &["Lancement par touche Xbox Guide Nexus et panneau de démarrage Game Bar désactivés"],
        }),
        "disable_widgets_news" => Some(LocalizedTweakText {
            title: "Désactiver les widgets Windows et le flux d'actualités MSN",
            description: "Désactive l'icône Widgets de la barre des tâches et empêche la récupération en arrière-plan des actualités MSN, sujets tendance et widgets météo.",
            broken_features: &["Le volet Widgets de la barre des tâches sera désactivé"],
        }),
        "gamedvr_config_store" => Some(LocalizedTweakText {
            title: "Désactiver la superposition d'enregistrement Xbox GameDVR",
            description: "Désactive l'enregistrement vidéo de jeu en arrière-plan et l'injection de hooks Xbox Game Bar, éliminant les chutes de temps d'image et micro-saccades.",
            broken_features: &["L'enregistrement instantané de clips en arrière-plan Xbox Game Bar sera désactivé"],
        }),
        "gamedvr_app_capture" => Some(LocalizedTweakText {
            title: "Désactiver la capture d'applications Windows GameDVR",
            description: "Désactive les tampons d'enregistrement audio/vidéo en arrière-plan dans le sous-système Windows GameDVR.",
            broken_features: &[],
        }),
        "gamedvr_policy_disable" => Some(LocalizedTweakText {
            title: "Désactiver la stratégie système GameDVR",
            description: "Verrouillage par stratégie de groupe à l'échelle du système empêchant Windows de réactiver les services de capture GameDVR en arrière-plan.",
            broken_features: &[],
        }),
        "consumer_features_disable" => Some(LocalizedTweakText {
            title: "Désactiver les fonctionnalités cloud grand public de Windows",
            description: "Empêche Windows de télécharger et d'installer automatiquement des applications et jeux sponsorisés tiers (ex. TikTok, Disney, Candy Crush) lors de la maintenance.",
            broken_features: &[],
        }),
        "disable_telemetry_scheduled_tasks" => Some(LocalizedTweakText {
            title: "Désactiver les tâches de télémétrie Commentaires, Autochk et Cartes",
            description: "Désactive les invites diagnostiques SIUF du Hub de commentaires Windows, le proxy de télémétrie Autochk et les tâches de mise à jour des notifications Cartes en arrière-plan (adapté de Sophia Script).",
            broken_features: &["Les notifications de sondages proactifs du Hub de commentaires seront désactivées"],
        }),
        "disable_hpet_synthetic_timer" => Some(LocalizedTweakText {
            title: "Désactiver Dynamic Tick (minuteur noyau synthétique)",
            description: "Désactive l'économie d'énergie Dynamic Tick dans le noyau Windows NT, évitant la dérive d'horloge du minuteur synthétique et imposant des requêtes fiables haute fréquence Invariant TSC pour une latence DPC ultra-basse.",
            broken_features: &["Consommation électrique du CPU au repos légèrement plus élevée"],
        }),
        "optimize_tcp_congestion_tuning" => Some(LocalizedTweakText {
            title: "Optimiser la congestion TCP réseau et le réglage automatique de fenêtre",
            description: "Active les horodatages TCP haute performance RFC 1323 et désactive les diffusions Web Services Dynamic Discovery (WSD) pour minimiser la latence en jeu en ligne et les délais de sérialisation des paquets.",
            broken_features: &[],
        }),
        "disable_windows_error_reporting" => Some(LocalizedTweakText {
            title: "Désactiver la stratégie et les envois de rapports d'erreurs Windows",
            description: "Applique le verrou de stratégie de groupe de rapport d'erreurs Windows et désactive l'envoi de données supplémentaires de vidage sur incident aux serveurs Microsoft.",
            broken_features: &["Les rapports d'erreurs automatiques et envois de télémétrie sont bloqués"],
        }),
        "disable_bing_search_start_menu" => Some(LocalizedTweakText {
            title: "Désactiver les recommandations du menu Démarrer et les publicités Iris",
            description: "Supprime les recommandations sponsorisées de sites Web/comptes, les astuces Iris et l'encombrement des applications récemment ajoutées dans les menus Démarrer de Windows 10/11.",
            broken_features: &["Les recommandations promotionnelles du menu Démarrer sont masquées"],
        }),
        "disable_cortana_autostart" => Some(LocalizedTweakText {
            title: "Désactiver la recherche Cortana par utilisateur et le bouton de barre des tâches",
            description: "Désactive l'activation de Cortana par utilisateur, les autorisations de localisation de recherche et masque l'entrée Cortana de la barre des tâches.",
            broken_features: &["L'intégration de recherche Cortana dans la barre des tâches par utilisateur sera désactivée"],
        }),
        // === perf_power_ext.toml (20) ===
        "enable_long_file_paths" => Some(LocalizedTweakText {
            title: "Activer les chemins de fichiers longs Win32 NTFS (>260 caractères)",
            description: "Supprime l'ancienne limite MAX_PATH de 260 caractères dans Windows 10/11 via LongPathsEnabled=1, évitant les erreurs de troncature de chemin lors des compilations et extractions d'archives.",
            broken_features: &[],
        }),
        "disable_background_search_toggle_and_app_policy" => Some(LocalizedTweakText {
            title: "Refuser l'exécution en arrière-plan et l'option d'arrière-plan de Recherche",
            description: "Définit AppPrivacy\\LetAppsRunInBackground=2 (Refus forcé) et Search\\BackgroundAppGlobalToggle=0 afin que les applications UWP/Store inactives ne puissent pas consommer de CPU ni de RAM en arrière-plan.",
            broken_features: &["Les applications UWP comme Store/Courrier ne se synchronisent pas lorsqu'elles sont fermées"],
        }),
        "extend_webclient_file_size_limit" => Some(LocalizedTweakText {
            title: "Maximiser la limite de taille de fichier WebClient / WebDAV (4 Go)",
            description: "Relève la limite de téléchargement du service Windows WebClient de 50 Mo au maximum de 4 Go (0xFFFFFFFF), corrigeant les échecs de copie sur partages WebDAV / réseau montés.",
            broken_features: &[],
        }),
        "reduce_services_pipe_timeout" => Some(LocalizedTweakText {
            title: "Réduire le délai d'expiration de démarrage/arrêt des services à 15 s",
            description: "Abaisse ServicesPipeTimeout de la valeur par défaut de 30000 ms (30 s) à 15000 ms (15 s) pour que les services d'arrière-plan bloqués se libèrent plus vite au démarrage et à l'arrêt.",
            broken_features: &["Des services de base de données personnalisés très lents mettant >15 s au démarrage peuvent expirer"],
        }),
        "mmcss_scheduler_full_optimization" => Some(LocalizedTweakText {
            title: "Optimisation complète du planificateur multimédia MMCSS (NoLazyMode, AlwaysOn, SFIO élevé)",
            description: "Configure Multimedia\\SystemProfile NoLazyMode=1, AlwaysOn=1, ainsi que Games Tasks Scheduling Category='High' et SFIO Priority='High' pour une planification ininterrompue des threads multimédias et de jeu.",
            broken_features: &[],
        }),
        "enable_directx_windowed_game_optimizations" => Some(LocalizedTweakText {
            title: "Activer les optimisations DirectX Flip-Model et VRR en mode fenêtré (fusion sans perte)",
            description: "Fusionne en toute sécurité SwapEffectUpgradeEnable=1 et VRROptimizeEnable=1 dans DirectXUserGlobalSettings sans écraser vos préférences GPU ou AutoHDR par jeu.",
            broken_features: &[],
        }),
        "disable_hardware_overlays_and_min_fps" => Some(LocalizedTweakText {
            title: "Désactiver les superpositions matérielles du noyau et la limitation FPS minimale du DWM",
            description: "Définit GraphicsDrivers\\DisableOverlays=1 et Dwm\\OverlayMinFPS=0 afin d'éviter les blocages de promotion des plans de superposition matérielle DWM et les saccades à faible fréquence de rafraîchissement.",
            broken_features: &["Consommation électrique du compositeur DWM au repos légèrement plus élevée sur ordinateur portable"],
        }),
        "nvidia_restore_legacy_sharpening" => Some(LocalizedTweakText {
            title: "Restaurer l'ancienne politique de netteté du pilote NVIDIA (EnableGR535=0)",
            description: "Définit Global\\FTS\\EnableGR535=0 afin que le Panneau de configuration NVIDIA affiche la netteté d'image classique à faible surcharge au lieu d'imposer le nouveau filtre de superposition.",
            broken_features: &[],
        }),
        "nvidia_dlss_debug_indicator" => Some(LocalizedTweakText {
            title: "Afficher le filigrane de débogage version & préréglage NVIDIA DLSS",
            description: "Définit Global\\NGXCore\\ShowDlssIndicator=1024 pour afficher à l'écran la version active du SDK DLSS, la mise à l'échelle de résolution et le préréglage dans les jeux DirectX/Vulkan.",
            broken_features: &["Affiche un filigrane de diagnostic DLSS en bas à gauche dans les jeux compatibles"],
        }),
        "disable_game_bar_tips_and_capture" => Some(LocalizedTweakText {
            title: "Désactiver le bouton Nexus, les astuces de démarrage et la capture historique de Game Bar",
            description: "Désactive UseNexusForGameBarEnabled (lanceur du bouton Guide Xbox), les astuces ShowStartupPanel et l'enregistrement en mémoire tampon circulaire HistoricalCaptureEnabled.",
            broken_features: &["Le bouton Nexus de la manette ne déclenche plus la superposition Game Bar"],
        }),
        "optimize_mouse_thresholds_and_hover" => Some(LocalizedTweakText {
            title: "Mettre à zéro les seuils d'accélération souris et réponse au survol à 10 ms",
            description: "Définit MouseThreshold1='0', MouseThreshold2='0' et MouseHoverTime='10' ms afin que les seuils d'accélération du pointeur soient parfaitement linéaires et que le survol réagisse instantanément.",
            broken_features: &[],
        }),
        "disable_extra_accessibility_hotkeys" => Some(LocalizedTweakText {
            title: "Désactiver les raccourcis accidentels Touches souris, Contraste élevé et Narrateur",
            description: "Désactive les fenêtres contextuelles de raccourcis Alt+Maj+Verr.Num (Touches souris), Alt+Maj+Impr.Écran (Contraste élevé) et Win+Ctrl+Entrée (Narrateur) pendant les sessions de jeu.",
            broken_features: &["Les raccourcis clavier d'accessibilité doivent être activés via l'application Paramètres"],
        }),
        "disable_smart_multi_homed_name_resolution" => Some(LocalizedTweakText {
            title: "Désactiver la résolution de noms DNS multi-réseaux intelligente de Windows",
            description: "Définit DNSClient\\DisableSmartNameResolution=1 afin que Windows interroge le DNS uniquement sur l'interface principale au lieu de diffuser des requêtes parallèles sur les adaptateurs VPN/virtuels.",
            broken_features: &["Les VPN en tunnel fractionné (split-tunnel) doivent configurer proprement leurs métriques DNS d'adaptateur"],
        }),
        "harden_windows_update_ux_and_store" => Some(LocalizedTweakText {
            title: "Bloquer l'écrasement de pilotes par WU, les redémarrages imprévus et les téléchargements Store",
            description: "Exclut l'envoi de pilotes GPU/périphériques lors des mises à jour qualité, empêche les redémarrages forcés en session active, débloque une pause étendue des mises à jour (3650 jours) et coupe les téléchargements silencieux du Store tout en gardant les correctifs de sécurité.",
            broken_features: &["Les mises à jour des applications Microsoft Store doivent être vérifiées manuellement"],
        }),
        "power_settings_pcie_usb_nvme_epb" => Some(LocalizedTweakText {
            title: "Réglage ultra-basse latence du mode d'alimentation actif (PCIe ASPM, veille USB, NVMe, EPP, PowerThrottling)",
            description: "Utilise les API natives Windows powrprof sur le mode d'alimentation actif (compatible Équilibré, Performances élevées et plans personnalisés) pour désactiver sur secteur PCIe ASPM, la suspension sélective USB, l'économie d'énergie USB 3, la veille NVMe et l'EPP CPU (EPP=0), tout en coupant PowerThrottling du noyau.",
            broken_features: &["Consommation électrique au repos sur secteur plus élevée sur PC de bureau et portables branchés"],
        }),
        "disable_storage_sense_policy" => Some(LocalizedTweakText {
            title: "Désactiver les nettoyages automatiques en arrière-plan de l'Assistant Stockage",
            description: "Empêche l'Assistant Stockage de Windows d'exécuter des nettoyages disque automatiques en arrière-plan pendant les sessions interactives.",
            broken_features: &["Les fichiers temporaires et la Corbeille doivent être vidés manuellement via Nettoyage & Apps"],
        }),
        "consolidate_svchost_split_threshold" => Some(LocalizedTweakText {
            title: "Consolider dynamiquement les hôtes de services selon la RAM physique (SvcHostSplitThresholdInKB) [Opt-In]",
            description: "Calcule dynamiquement SvcHostSplitThresholdInKB d'après la RAM système détectée (TotalRamInKb), réduisant le nombre de processus et la mémoire virtuelle sur les machines 8 Go+. Attention : réduit l'isolation des processus par service.",
            broken_features: &["Isolation des plantages svchost réduite ; le Gestionnaire des tâches affiche moins de services groupés"],
        }),
        "disable_vbs_deviceguard" => Some(LocalizedTweakText {
            title: "Désactiver la sécurité basée sur la virtualisation (VBS) [Compromis sécurité Opt-In]",
            description: "Désactive EnableVirtualizationBasedSecurity et RequirePlatformSecurityFeatures. Peut augmenter les FPS de 5 à 15 % sur les CPU ayant une surcharge de virtualisation, au détriment des limites de sécurité par virtualisation du noyau.",
            broken_features: &["Désactive l'isolation de sécurité VBS ; un avertissement anti-triche / Sécurité Windows peut apparaître"],
        }),
        "disable_hvci_memory_integrity" => Some(LocalizedTweakText {
            title: "Désactiver l'isolation du noyau / intégrité de la mémoire (HVCI) [Compromis sécurité Opt-In]",
            description: "Désactive l'intégrité du code protégée par hyperviseur (HVCI). Réduit la latence DPC/interruptions en jeu compétitif là où HVCI pénalise les temps d'image, au prix de l'application stricte de signature de pilotes noyau.",
            broken_features: &["Sécurité Windows affiche un avertissement d'isolation du noyau / intégrité de la mémoire désactivée"],
        }),
        "disable_prefetch_and_sysmain" => Some(LocalizedTweakText {
            title: "Désactiver le Prefetcher noyau et SysMain (Superfetch) pour SSD NVMe rapides [Opt-In]",
            description: "Désactive le Prefetcher d'applications/démarrage et stoppe le préchargement mémoire SysMain sur les disques NVMe rapides où la mise en cache RAM génère des E/S disque inutiles en arrière-plan.",
            broken_features: &["Le lancement à froid peut ralentir légèrement sur les disques durs mécaniques SATA"],
        }),

        // === privacy.toml (12) ===
        "disable_advertising_id" => Some(LocalizedTweakText {
            title: "Désactiver l'identifiant publicitaire",
            description: "Désactive l'identifiant publicitaire par utilisateur fourni par Windows aux applications pour le ciblage publicitaire et le suivi inter-applications.",
            broken_features: &["Les applications ne recevront plus d'identifiant publicitaire unique"],
        }),
        "disable_cortana" => Some(LocalizedTweakText {
            title: "Désactiver Cortana",
            description: "Désactive Cortana via la stratégie de groupe, l'empêchant de s'exécuter, d'écouter et d'envoyer des données aux services cloud de Microsoft.",
            broken_features: &["L'assistant vocal Cortana sera désactivé"],
        }),
        "disable_activity_history" => Some(LocalizedTweakText {
            title: "Désactiver l'historique des activités / Chronologie",
            description: "Désactive la fonction d'historique des activités Windows (Chronologie), empêchant Windows de collecter et stocker l'historique de vos activités, sites visités et documents ouverts.",
            broken_features: &["La Chronologie/Historique des activités dans la Vue des tâches sera vide"],
        }),
        "disable_online_speech_recognition" => Some(LocalizedTweakText {
            title: "Désactiver la reconnaissance vocale en ligne",
            description: "Désactive le service de reconnaissance vocale basé sur le cloud. Aucune donnée vocale ne sera envoyée aux serveurs Microsoft pour traitement.",
            broken_features: &["La reconnaissance vocale cloud ne fonctionnera pas ; la reconnaissance locale/hors ligne reste intacte"],
        }),
        "disable_tailored_experiences" => Some(LocalizedTweakText {
            title: "Désactiver les expériences personnalisées",
            description: "Empêche Microsoft d'utiliser les données de diagnostic pour proposer des astuces, publicités et recommandations personnalisées dans Windows.",
            broken_features: &["Les suggestions et astuces Windows personnalisées seront désactivées"],
        }),
        "disable_windows_recall" => Some(LocalizedTweakText {
            title: "Désactiver Windows Recall (instantané IA)",
            description: "Désactive la fonction Windows Recall introduite dans Windows 11 24H2, qui réalise périodiquement des captures d'écran indexées par IA de votre activité. Nécessite la build 26100+.",
            broken_features: &["La fonction d'instantané IA Windows Recall sera désactivée"],
        }),
        "disable_cloud_content_spotlight" => Some(LocalizedTweakText {
            title: "Désactiver le contenu cloud et les suggestions Windows à la une",
            description: "Désactive Windows à la une (Spotlight), le contenu suggéré dans les Paramètres et les suggestions d'applications préinstallées diffusées via le cloud.",
            broken_features: &[
                "Les images Windows à la une de l'écran de verrouillage seront désactivées",
                "Les applications suggérées n'apparaîtront plus dans le menu Démarrer",
            ],
        }),
        "disable_location_tracking" => Some(LocalizedTweakText {
            title: "Désactiver le suivi de localisation",
            description: "Désactive globalement le service de localisation Windows, empêchant les applications et le système d'accéder à votre position géographique.",
            broken_features: &["Les applications nécessitant la localisation (Cartes, Météo) n'auront plus accès à votre position"],
        }),
        "disable_activity_cloud_sync" => Some(LocalizedTweakText {
            title: "Désactiver le presse-papiers cloud et la synchronisation multi-appareils",
            description: "Désactive la persistance de l'historique du presse-papiers Windows et la synchronisation cloud multi-appareils via stratégie de groupe afin que mots de passe copiés et données sensibles restent strictement locaux.",
            broken_features: &["L'historique du presse-papiers Win+V et la synchronisation de collage multi-appareils seront désactivés"],
        }),
        "disable_cdp_service" => Some(LocalizedTweakText {
            title: "Désactiver le service Plateforme des appareils connectés",
            description: "Désactive le service Plateforme des appareils connectés (CDPUserSvc), bloquant la télémétrie inter-appareils, les diffusions de synchronisation d'applications et les balises d'activité en arrière-plan.",
            broken_features: &["Le partage de proximité et la synchronisation multi-appareils avec les mobiles seront désactivés"],
        }),
        "spotlight_desktop_disable" => Some(LocalizedTweakText {
            title: "Désactiver les téléchargements Windows à la une sur l'écran de verrouillage",
            description: "Désactive la récupération automatique en arrière-plan des fonds d'écran de verrouillage Windows à la une et des images promotionnelles Microsoft.",
            broken_features: &[],
        }),
        "spotlight_features_disable" => Some(LocalizedTweakText {
            title: "Désactiver les astuces et publicités de l'écran de verrouillage",
            description: "Désactive les astuces, suggestions et promotions sponsorisées du Microsoft Store sur l'écran de verrouillage diffusées via Content Delivery Manager.",
            broken_features: &[],
        }),

        // === registry_lag.toml (18) ===
        "win32_priority_separation" => Some(LocalizedTweakText {
            title: "Win32PrioritySeparation — Priorité au premier plan",
            description: "Règle le quantum du planificateur sur court, variable, avec forte priorité au premier plan (0x26 = 38). Priorise les applications actives pour réduire la latence d'entrée dans les jeux et charges interactives.",
            broken_features: &[],
        }),
        "menu_show_delay" => Some(LocalizedTweakText {
            title: "Réduire le délai d'affichage des menus à 0 ms",
            description: "Supprime le délai artificiel avant l'apparition des menus au survol. La valeur par défaut est de 400 ms ; passer à 0 rend l'ouverture instantanée.",
            broken_features: &[],
        }),
        "mmcss_gpu_priority" => Some(LocalizedTweakText {
            title: "MMCSS — Élever la priorité des jeux et de l'audio pro",
            description: "Augmente la priorité des threads Jeux et Audio Pro au sein du service Multimedia Class Scheduler (MMCSS), garantissant une priorité d'ordonnancement supérieure aux flux audio et de jeu temps réel.",
            broken_features: &[],
        }),
        "mouse_data_queue_size" => Some(LocalizedTweakText {
            title: "Augmenter la taille de file de données de la souris",
            description: "Augmente la file tampon des données d'entrée souris de 100 entrées par défaut à 256 entrées, réduisant le risque de perte d'événements sur les souris gaming à fréquence d'interrogation élevée.",
            broken_features: &[],
        }),
        "keyboard_response" => Some(LocalizedTweakText {
            title: "Optimiser la réactivité du clavier",
            description: "Règle via le Registre le délai de répétition clavier au plus court et la fréquence au plus rapide, et augmente la taille de la file d'attente des données clavier.",
            broken_features: &[],
        }),
        "disable_wer_queue" => Some(LocalizedTweakText {
            title: "Désactiver la file d'attente des rapports d'erreurs Windows (WER)",
            description: "Désactive la file d'attente Windows Error Reporting (WER), empêchant le système d'écrire des vidages sur incident et des rapports en arrière-plan après un plantage d'application.",
            broken_features: &["Les rapports de plantage ne seront pas envoyés à Microsoft"],
        }),
        "auto_end_tasks" => Some(LocalizedTweakText {
            title: "Terminer automatiquement les tâches bloquées à l'arrêt",
            description: "Met automatiquement fin aux applications bloquées lors de l'arrêt du système sans demander confirmation. Réduit également WaitToKillAppTimeout et HungAppTimeout pour accélérer la déconnexion et l'extinction.",
            broken_features: &["Le travail non enregistré dans des applications bloquées peut être perdu lors de l'arrêt"],
        }),
        "network_throttling_disable" => Some(LocalizedTweakText {
            title: "Désactiver l'index de limitation réseau (NetworkThrottlingIndex)",
            description: "Désactive la limitation réseau multimédia de Windows (NetworkThrottlingIndex = 0xFFFFFFFF), évitant les pertes de paquets et pics de latence lors d'un fort trafic réseau ou en jeu.",
            broken_features: &[],
        }),
        "system_responsiveness_gaming" => Some(LocalizedTweakText {
            title: "Maximiser la réactivité système au premier plan",
            description: "Configure SystemResponsiveness à 0 %, consacrant 100 % des cycles CPU aux jeux et applications interactives au premier plan au lieu d'en réserver 20 % aux tâches d'arrière-plan.",
            broken_features: &[],
        }),
        "ntfs_disable_last_access" => Some(LocalizedTweakText {
            title: "Désactiver l'horodatage du dernier accès NTFS",
            description: "Empêche le système de fichiers NTFS de mettre à jour les horodatages de métadonnées à chaque lecture de fichier, réduisant sensiblement la charge d'E/S sur SSD et NVMe.",
            broken_features: &["Les propriétés des fichiers n'afficheront plus la date exacte du dernier accès"],
        }),
        "disable_paging_executive" => Some(LocalizedTweakText {
            title: "Conserver le noyau Windows et les pilotes en RAM",
            description: "Force le noyau exécutif Windows NT et les routines des pilotes à rester résidents dans la RAM physique au lieu d'être paginés sur disque, assurant une réponse instantanée aux interruptions.",
            broken_features: &[],
        }),
        "large_system_cache" => Some(LocalizedTweakText {
            title: "Activer le grand cache système de fichiers",
            description: "Configure Windows pour maintenir un cache du système de fichiers plus vaste en RAM physique, accélérant les accès répétés et le streaming des textures de jeux.",
            broken_features: &[],
        }),
        "disable_fast_startup" => Some(LocalizedTweakText {
            title: "Désactiver le démarrage rapide (prévention des fuites de veille hybride)",
            description: "Désactive le démarrage rapide Windows (hiberboot), garantissant un véritable cycle d'amorçage propre à chaque extinction et prévenant les fuites mémoire du noyau et corruptions d'état des pilotes.",
            broken_features: &["Le temps de démarrage à froid peut augmenter de 1 à 2 secondes sur les disques mécaniques lents"],
        }),
        "ntfs_disable_8dot3" => Some(LocalizedTweakText {
            title: "Désactiver la création de noms courts NTFS 8.3",
            description: "Désactive l'ancienne génération de noms de fichiers courts MS-DOS 8.3 sur les volumes NTFS, éliminant la latence d'indexation de répertoires et les écritures superflues sur NVMe/SSD rapides.",
            broken_features: &[],
        }),
        "increase_mft_zone_reservation" => Some(LocalizedTweakText {
            title: "Augmenter le tampon de zone de la table de fichiers maîtres (MFT)",
            description: "Réserve un tampon disque contigu plus important pour la Master File Table NTFS, prévenant la fragmentation de la MFT et maintenant le débit d'accès disque maximal.",
            broken_features: &[],
        }),
        "tcp_ack_frequency" => Some(LocalizedTweakText {
            title: "TCP faible latence (TcpAckFrequency)",
            description: "Force la pile réseau TCP Windows à envoyer immédiatement les acquittements ACK sans attendre le regroupement par délai d'expiration, réduisant le ping dans les jeux en ligne.",
            broken_features: &[],
        }),
        "tcp_no_delay_global" => Some(LocalizedTweakText {
            title: "Désactiver l'algorithme de Nagle (TCPNoDelay)",
            description: "Désactive l'algorithme de mise en tampon de paquets de Nagle sur les interfaces réseau actives, transmettant les petits paquets immédiatement pour une latence aller-retour minimale.",
            broken_features: &[],
        }),
        "disable_mouse_acceleration" => Some(LocalizedTweakText {
            title: "Désactiver l'amélioration de la précision du pointeur (entrée souris 1:1)",
            description: "Désactive l'accélération non linéaire du curseur Windows, offrant un véritable mouvement matériel brut 1:1 indispensable pour la précision compétitive.",
            broken_features: &[],
        }),

        // === telemetry_apps.toml (7) ===
        "disable_nvidia_telemetry_service" => Some(LocalizedTweakText {
            title: "Désactiver le service NVIDIA Telemetry Container",
            description: "Désactive le service NvTelemetryContainer, qui collecte et transmet les statistiques d'utilisation et les données de plantage des pilotes NVIDIA.",
            broken_features: &["Les données de plantage et la télémétrie d'utilisation NVIDIA ne seront pas collectées"],
        }),
        "disable_nvidia_telemetry_tasks" => Some(LocalizedTweakText {
            title: "Désactiver les tâches planifiées de télémétrie NVIDIA",
            description: "Désactive les tâches planifiées NvTmMon, NvTmRep et NvProfile qui envoient périodiquement les données de télémétrie NVIDIA.",
            broken_features: &[],
        }),
        "disable_adobe_update_service" => Some(LocalizedTweakText {
            title: "Désactiver les services de mise à jour Adobe",
            description: "Désactive Adobe Acrobat Update Service (AdobeARMservice) et Adobe Genuine Monitor Service. Empêche les vérifications de mise à jour en arrière-plan et la télémétrie de licence.",
            broken_features: &["Les produits Adobe ne se mettront plus à jour automatiquement ; mettez-les à jour manuellement"],
        }),
        "disable_edge_update" => Some(LocalizedTweakText {
            title: "Désactiver les services de mise à jour Microsoft Edge",
            description: "Désactive les services Microsoft Edge Update (edgeupdate et edgeupdatem) ainsi que les tâches planifiées associées. Edge continue de fonctionner mais ne se mettra plus à jour automatiquement ni ne contactera les serveurs de mise à jour.",
            broken_features: &["Microsoft Edge ne se mettra plus à jour automatiquement ; les correctifs de sécurité doivent être appliqués manuellement"],
        }),
        "disable_chrome_update" => Some(LocalizedTweakText {
            title: "Désactiver les services de mise à jour Google Chrome",
            description: "Désactive les services Google Update (gupdate et gupdatem) et leurs tâches planifiées. Chrome continue de fonctionner mais ne se mettra plus à jour automatiquement.",
            broken_features: &["Google Chrome ne se mettra plus à jour automatiquement ; les correctifs de sécurité doivent être appliqués manuellement"],
        }),
        "disable_steam_client_service" => Some(LocalizedTweakText {
            title: "Désactiver le démarrage automatique du Steam Client Service",
            description: "Passe le Steam Client Service du démarrage automatique au démarrage manuel. Steam le lancera à la demande lors de l'ouverture, mais il ne consommera plus de ressources en arrière-plan lorsque Steam est fermé.",
            broken_features: &[],
        }),
        "disable_razer_telemetry" => Some(LocalizedTweakText {
            title: "Désactiver la télémétrie et les analyses Razer",
            description: "Empêche le démarrage automatique du Razer Game Manager Service et des services Razer Chroma SDK, réduisant la télémétrie et les analyses en arrière-plan de Razer Synapse.",
            broken_features: &["Les fonctions de démarrage automatique de Razer Synapse peuvent nécessiter un lancement manuel"],
        }),

        // === telemetry_os.toml (14) ===
        "disable_diagtrack" => Some(LocalizedTweakText {
            title: "Désactiver DiagTrack (Expériences des utilisateurs connectés et télémétrie)",
            description: "Désactive le service Expériences des utilisateurs connectés et télémétrie (DiagTrack / diagsvc), principal service de collecte télémétrique de Windows.",
            broken_features: &["Les données de diagnostic ne seront pas envoyées à Microsoft"],
        }),
        "disable_dmwappushservice" => Some(LocalizedTweakText {
            title: "Désactiver dmwappushservice (routage de messages Push WAP)",
            description: "Désactive le service de routage de messages Push WAP (Wireless Application Protocol) de gestion des appareils, utilisé pour acheminer les données de télémétrie.",
            broken_features: &["Les messages Push MDM ne seront pas reçus"],
        }),
        "disable_telemetry_registry" => Some(LocalizedTweakText {
            title: "Désactiver la télémétrie via le Registre AllowTelemetry",
            description: "Définit la stratégie AllowTelemetry sur 0 (Sécurité/Désactivé), le niveau le plus bas. Sous Enterprise et Education cela désactive entièrement la télémétrie ; sous Home et Pro cela la réduit au minimum requis.",
            broken_features: &["Certaines fonctionnalités Windows Insider et de diagnostic peuvent ne pas fonctionner"],
        }),
        "disable_compat_telrunner" => Some(LocalizedTweakText {
            title: "Désactiver les tâches de l'évaluateur de compatibilité des applications",
            description: "Désactive les tâches planifiées Microsoft Compatibility Appraiser et ProgramDataUpdater exécutant CompatTelRunner.exe, qui analyse les logiciels installés et transmet la télémétrie de compatibilité.",
            broken_features: &["Les vérifications de compatibilité de mise à niveau Windows ne s'exécuteront plus automatiquement"],
        }),
        "disable_ceip" => Some(LocalizedTweakText {
            title: "Désactiver le Programme d'amélioration du produit (CEIP)",
            description: "Désactive les tâches planifiées CEIP (Consolidator, UsbCeip, KernelCeipTask) et la clé de stratégie CEIP dans le Registre.",
            broken_features: &["Les données télémétriques du programme CEIP ne seront pas collectées"],
        }),
        "disable_wer_service" => Some(LocalizedTweakText {
            title: "Désactiver le service de rapport d'erreurs Windows",
            description: "Désactive le service de rapport d'erreurs Windows (WerSvc), empêchant l'envoi de rapports de plantage et de télémétrie d'erreurs à Microsoft.",
            broken_features: &[
                "Les rapports de plantage ne seront pas envoyés à Microsoft",
                "L'historique des rapports de problèmes ne sera pas mis à jour",
            ],
        }),
        "disable_connected_user_experiences" => Some(LocalizedTweakText {
            title: "Désactiver les expériences utilisateur connectées et les suggestions",
            description: "Désactive les suggestions dans Paramètres, le menu Démarrer et l'écran de verrouillage alimentées par les contenus cloud et les retours télémétriques.",
            broken_features: &["Les suggestions et astuces Windows n'apparaîtront plus"],
        }),
        "disable_diagnostic_data_viewer" => Some(LocalizedTweakText {
            title: "Désactiver la Visionneuse de données de diagnostic",
            description: "Désactive la journalisation des événements de la Visionneuse de données de diagnostic, qui stocke une copie locale des données envoyées à Microsoft. Libère des E/S disque et de l'espace.",
            broken_features: &["Impossible de consulter localement les données via l'application Visionneuse de données de diagnostic"],
        }),
        "disable_delivery_optimization_p2p" => Some(LocalizedTweakText {
            title: "Désactiver l'utilisation de bande passante montante P2P de l'Optimisation de livraison",
            description: "Restreint l'Optimisation de livraison Windows (DODownloadMode = 0), empêchant Windows d'utiliser votre débit montant pour diffuser des mises à jour à d'autres PC sur Internet.",
            broken_features: &[],
        }),
        "disable_diagnostic_execution_service" => Some(LocalizedTweakText {
            title: "Désactiver le service d'exécution de diagnostic (diagsvc)",
            description: "Désactive le service d'exécution de diagnostic (diagsvc), qui exécute des diagnostics planifiés en arrière-plan et des paquets de télémétrie.",
            broken_features: &[],
        }),
        "wersvc_service_disable" => Some(LocalizedTweakText {
            title: "Désactiver le service collecteur standard du Hub de diagnostics",
            description: "Désactive le service Visual Studio / Windows Diagnostics Hub Standard Collector, stoppant les collecteurs de traces ETW en arrière-plan au repos.",
            broken_features: &["La collecte de traces du Hub de diagnostics VS/Windows requiert un démarrage manuel"],
        }),
        "disable_remote_registry" => Some(LocalizedTweakText {
            title: "Désactiver le service Registre à distance",
            description: "Désactive le service RemoteRegistry, empêchant les utilisateurs distants et entités réseau externes d'interroger ou de modifier les clés de Registre locales.",
            broken_features: &["L'administration à distance du Registre sur le réseau local LAN sera désactivée"],
        }),
        "disable_disk_diagnostic_task" => Some(LocalizedTweakText {
            title: "Désactiver la tâche télémétrique de diagnostic disque",
            description: "Désactive la tâche planifiée qui collecte les métriques d'utilisation disque et les diagnostics matériels pour les rapports de télémétrie.",
            broken_features: &[],
        }),
        "disable_winsat_task" => Some(LocalizedTweakText {
            title: "Désactiver la tâche de test de performances WinSAT en arrière-plan",
            description: "Empêche la tâche planifiée de l'outil d'évaluation système Windows de lancer sans préavis des tests de stress disque et GPU pendant l'inactivité.",
            broken_features: &[],
        }),

        // === ux_customization.toml (13) ===
        "enable_classic_context_menu" => Some(LocalizedTweakText {
            title: "Restaurer le menu contextuel complet classique de Windows 10",
            description: "Enregistre la substitution CLSID sous HKCU\\Software\\Classes afin qu'un clic droit ouvre immédiatement le menu contextuel complet sans clic supplémentaire sur 'Afficher plus d'options'.",
            broken_features: &["Menu contextuel moderne compact de Win11 remplacé par le menu complet classique"],
        }),
        "explorer_power_user_defaults" => Some(LocalizedTweakText {
            title: "Afficher les extensions, le chemin complet, ouvrir sur Ce PC et masquer les bannières de synchro",
            description: "Affiche les extensions connues et les fichiers cachés, ouvre l'Explorateur directement sur 'Ce PC' plutôt que l'Accès rapide, active le chemin complet dans la barre de titre, l'espacement compact et masque les bannières OneDrive/synchro.",
            broken_features: &[],
        }),
        "explorer_hide_win11_home_gallery_nav" => Some(LocalizedTweakText {
            title: "Masquer les raccourcis Accueil et Galerie du volet de navigation de l'Explorateur",
            description: "Désépingle les entrées Accueil (Home) et Galerie (Gallery) de Windows 11 du volet de navigation gauche de l'Explorateur de fichiers via HKCU\\Software\\Classes\\CLSID.",
            broken_features: &["Liens rapides Accueil et Galerie masqués dans la barre latérale gauche de l'Explorateur"],
        }),
        "explorer_disable_startup_delay_and_jpeg_compression" => Some(LocalizedTweakText {
            title: "Supprimer le délai de démarrage des applications et la compression JPEG du fond d'écran",
            description: "Définit Explorer\\Serialize\\StartupDelayInMSec=0 pour lancer les programmes au démarrage sans bridage artificiel de 10 s, et JPEGImportQuality=100 pour des fonds d'écran sans perte.",
            broken_features: &[],
        }),
        "explorer_desktop_this_pc_and_disable_autoplay" => Some(LocalizedTweakText {
            title: "Afficher l'icône Bureau 'Ce PC' et désactiver l'exécution automatique des médias amovibles",
            description: "Place l'icône de raccourci 'Ce PC' sur le Bureau et désactive AutoPlay/AutoRun sur les lecteurs USB et optiques amovibles pour bloquer les fenêtres automatiques et menaces autorun.",
            broken_features: &["Les médias USB/SD insérés n'ouvrent plus de fenêtre d'action automatique"],
        }),
        "disable_visual_effects_userpreferencesmask_bits" => Some(LocalizedTweakText {
            title: "Retirer les lourdes animations de fondu/glissement/ombre via les bits UserPreferencesMask et DWM",
            description: "Applique des masques de bits ciblés sur UserPreferencesMask (désactivant fondu/glissement des menus, fondu des infobulles, fondu de sélection, ombre du curseur, glissement des listes déroulantes, défilement doux et animations des contrôles tout en gardant le lissage des polices) plus EnableAeroPeek=0.",
            broken_features: &["Transitions d'interface classiques et nerveuses sans animations de glissement/fondu ni ombres portées"],
        }),
        "taskbar_win11_power_layout_and_end_task" => Some(LocalizedTweakText {
            title: "Aligner la barre des tâches Win11 à gauche, masquer Widgets/Vue tâches/Recherche, activer Fin de tâche (stratégie Pro/Ent)",
            description: "Aligne les icônes à gauche (TaskbarAl=0), retire la Vue des tâches et les Widgets (TaskbarDa=0), masque la case Recherche, active le clic droit 'Fin de tâche' dans les paramètres développeur, désactive Aero Shake et retire la section 'Recommandations' du menu Démarrer (Windows 11 22H2+ Pro/Enterprise/Education).",
            broken_features: &["Icônes de barre des tâches alignées à gauche ; boutons Vue des tâches et Widgets masqués"],
        }),
        "hide_win10_meet_now_tray" => Some(LocalizedTweakText {
            title: "Masquer l'icône caméra 'Démarrer une réunion' (Meet Now) sous Windows 10",
            description: "Masque l'icône Skype 'Démarrer une réunion' de la zone de notification Windows 10 (versions Windows 10 uniquement).",
            broken_features: &[],
        }),
        "enable_windows_dark_mode_and_clock_seconds" => Some(LocalizedTweakText {
            title: "Activer le mode sombre système et applications + secondes dans l'horloge",
            description: "Bascule AppsUseLightTheme=0 et SystemUsesLightTheme=0 pour le mode sombre natif dans toute l'interface Windows, et affiche les secondes en direct dans l'horloge de la barre des tâches.",
            broken_features: &[],
        }),
        "disable_startup_sound_and_audio_ducking" => Some(LocalizedTweakText {
            title: "Désactiver le son de démarrage Windows et l'atténuation du volume lors des appels",
            description: "Coupe le carillon d'ouverture de session Windows et empêche Windows de baisser de 80 % le volume des jeux/musiques lorsque Discord, Teams ou des applications vocales ouvrent un flux de communication.",
            broken_features: &[],
        }),
        "enable_verbose_logon_status" => Some(LocalizedTweakText {
            title: "Activer les messages d'état détaillés de démarrage, connexion et arrêt Windows",
            description: "Définit VerboseStatus=1 afin que Windows affiche les étapes exactes des services et mises à jour plutôt que des écrans d'attente génériques.",
            broken_features: &[],
        }),
        "disable_lock_screen_pro" => Some(LocalizedTweakText {
            title: "Ignorer le rideau d'écran de verrouillage avant connexion (stratégie Pro/Enterprise)",
            description: "Passe directement à l'invite mot de passe/PIN après le démarrage ou le verrouillage sans devoir d'abord faire glisser l'image de l'écran de verrouillage.",
            broken_features: &["Le rideau de l'écran de verrouillage est ignoré sur les éditions Pro/Enterprise"],
        }),
        "enable_windows_developer_mode" => Some(LocalizedTweakText {
            title: "Activer le mode Développeur Windows (installation indépendante d'apps et liens symboliques)",
            description: "Active AllowDevelopmentWithoutDevLicense=1 afin que les utilisateurs puissent créer des liens symboliques sans UAC et installer des paquets MSIX/AppX non signés.",
            broken_features: &["Autorise le chargement indépendant de MSIX non signés ; destiné aux développeurs"],
        }),
        // === system_deep_opt.toml (30) ===
        "disable_dwm_animations_per_process" => Some(LocalizedTweakText {
            title: "Désactiver les animations de transition DWM par processus",
            description: "Désactive les animations de transition DWM par processus en définissant DisallowAnimations=1 et AnimationsShiftKey=1 dans la clé DWM. Supprime les effets de fondu et de glissement lors de la réduction, l'agrandissement ou le basculement des fenêtres, réduisant la charge du compositeur GPU.",
            broken_features: &["Les animations de transition des fenêtres seront supprimées"],
        }),
        "disable_dwm_composition_throttle" => Some(LocalizedTweakText {
            title: "Désactiver la limitation de composition du Gestionnaire de fenêtres du Bureau",
            description: "Définit DesktopCompositionThrottle=0 dans la clé DWM, empêchant le compositeur de brider le rendu lors de pertes d'images. Permet à DWM de maintenir un débit de composition maximal même sous charge GPU.",
            broken_features: &[],
        }),
        "disable_kernel_mitigations_audit" => Some(LocalizedTweakText {
            title: "Désactiver le mode d'audit des atténuations du noyau",
            description: "Désactive la journalisation d'audit des atténuations d'exploits du noyau en réinitialisant MitigationAuditOptions à zéro. Réduit les événements ETW lors de la création de processus et du traitement des exceptions.",
            broken_features: &["Les événements d'audit d'atténuation d'exploits ne seront pas journalisés"],
        }),
        "disable_llmnr_protocol" => Some(LocalizedTweakText {
            title: "Désactiver le protocole de résolution de noms LLMNR",
            description: "Désactive le protocole LLMNR en définissant EnableMulticast=0 dans les paramètres du client DNS. Élimine le trafic de multidiffusion inutile et protège contre les attaques de relais et d'empoisonnement LLMNR.",
            broken_features: &["La résolution de noms sur le réseau local sans DNS peut échouer", "La découverte d'appareils locaux de type groupe résidentiel peut être altérée"],
        }),
        "disable_memory_compression" => Some(LocalizedTweakText {
            title: "Désactiver le moteur de compression de mémoire",
            description: "Désactive le magasin de compression du Gestionnaire de mémoire Windows en définissant DisableCompression=1. Sur les systèmes dotés de 16 Go de RAM ou plus, cela supprime les pics d'utilisation processeur causés par le thread de compression.",
            broken_features: &["Utilisation accrue du fichier d'échange sur les systèmes à mémoire restreinte", "Peut provoquer des saturations de mémoire plus rapides sur les systèmes de 8 Go"],
        }),
        "disable_netbios_over_tcp" => Some(LocalizedTweakText {
            title: "Désactiver NetBIOS sur TCP/IP",
            description: "Désactive le protocole NetBIOS sur TCP/IP en définissant NetbiosOptions=2 et en désactivant la recherche LMHosts. NetBIOS est un protocole LAN obsolète qui diffuse des requêtes de résolution de noms, exposant le système à du trafic parasite et aux attaques d'empoisonnement NBNS.",
            broken_features: &["Les applications héritées dépendant de la résolution NetBIOS échoueront", "Les anciennes imprimantes réseau utilisant WINS peuvent devenir inaccessibles"],
        }),
        "disable_ntfs_encryption_driver" => Some(LocalizedTweakText {
            title: "Désactiver le pilote de chiffrement de fichiers NTFS (EFS)",
            description: "Définit NtfsDisableEncryption=1 dans la clé FileSystem, empêchant le pilote de filtre EFS d'intercepter les opérations d'E/S de fichiers. Réduit la latence d'accès aux fichiers sur les systèmes n'utilisant pas le chiffrement EFS.",
            broken_features: &["Le système de fichiers chiffrés NTFS (EFS) ne fonctionnera pas", "Les fichiers existants chiffrés avec EFS deviendront inaccessibles"],
        }),
        "disable_page_file_encryption" => Some(LocalizedTweakText {
            title: "Désactiver le chiffrement du fichier d'échange et l'effacement à l'arrêt",
            description: "Empêche Windows de chiffrer le fichier d'échange et désactive son effacement à l'arrêt (ClearPageFileAtShutdown=0, DisablePagingEncryption=1). Améliore le débit de pagination et réduit le temps d'extinction du système.",
            broken_features: &["Le contenu de la mémoire paginée reste sur le disque après l'arrêt", "Les données sensibles peuvent être récupérées sur le disque brut"],
        }),
        "disable_processor_idle_demote_threshold" => Some(LocalizedTweakText {
            title: "Désactiver la rétrogradation d'état d'inactivité du processeur",
            description: "Définit IdleDemoteThreshold=100 et IdlePromoteThreshold=100 dans la gestion d'alimentation du processeur. Maintient le processeur à son niveau de performance maximal même pendant de courts intervalles d'inactivité, éliminant les pics de latence liés à la montée en fréquence.",
            broken_features: &["Consommation d'énergie au repos plus élevée", "Autonomie de la batterie réduite sur les ordinateurs portables"],
        }),
        "disable_rss_base_cpu" => Some(LocalizedTweakText {
            title: "Définir le processeur de base RSS pour la répartition réseau",
            description: "Définit RssBaseCpu=1 dans les paramètres NDIS, demandant au Receive Side Scaling de démarrer le traitement des interruptions réseau à partir du processeur 1 au lieu du processeur 0. Libère le processeur d'amorçage pour les applications de premier plan.",
            broken_features: &[],
        }),
        "disable_spectre_meltdown_mitigations" => Some(LocalizedTweakText {
            title: "Désactiver les atténuations de failles processeur Spectre/Meltdown",
            description: "Désactive les atténuations matérielles des vulnérabilités processeur Spectre et Meltdown en définissant FeatureSettingsOverride=3 et FeatureSettingsOverrideMask=3. Supprime la pénalité de performances sur les charges de calcul intensives.",
            broken_features: &["Le système est vulnérable aux attaques par canal auxiliaire de processeur Spectre et Meltdown"],
        }),
        "disable_threaded_dpc_optimization" => Some(LocalizedTweakText {
            title: "Désactiver les DPC threadés pour une latence d'interruption réduite",
            description: "Définit ThreadDpcEnable=0 dans la clé du noyau, forçant l'exécution des appels de procédure différée à DISPATCH_LEVEL au lieu d'un thread dédié à PASSIVE_LEVEL. Réduit la latence DPC pour la production audio en temps réel et le jeu compétitif.",
            broken_features: &["Certains pilotes attendant des DPC threadés peuvent mal se comporter", "Variance de latence DPC plus élevée sous une charge multithread extrême"],
        }),
        "disable_windows_search_indexer" => Some(LocalizedTweakText {
            title: "Désactiver le service d'indexation Windows Search",
            description: "Désactive le service WSearch, empêchant l'indexeur d'analyser en permanence les fichiers en arrière-plan. Sur les SSD/NVMe, cela supprime les E/S disque, l'utilisation CPU et la mémoire consommées par SearchIndexer.exe.",
            broken_features: &["La recherche Windows ne renverra aucun résultat pour le contenu des fichiers", "La recherche sur le bureau Outlook peut être plus lente", "La recherche du menu Démarrer ne trouvera pas les fichiers par contenu"],
        }),
        "enable_hardware_task_offload" => Some(LocalizedTweakText {
            title: "Activer le déchargement matériel des tâches TCP/IP",
            description: "Définit DisableTaskOffload=0 dans les paramètres TCP/IP, garantissant que l'OS décharge le calcul des sommes de contrôle et la segmentation TCP sur la carte réseau, libérant des cycles CPU.",
            broken_features: &[],
        }),
        "enable_large_ntfs_frs" => Some(LocalizedTweakText {
            title: "Activer les grands segments d'enregistrement de fichiers NTFS",
            description: "Définit NtfsAllowExtendedCharacterIn8dot3Name=0, permettant au pilote NTFS d'allouer des segments d'enregistrement plus volumineux et efficaces, réduisant la fragmentation de la MFT.",
            broken_features: &["Les caractères Unicode étendus dans les noms courts 8.3 générés automatiquement seront supprimés"],
        }),
        "enable_ultimate_performance_plan" => Some(LocalizedTweakText {
            title: "Activer le mode d'alimentation Performances ultimes",
            description: "Active le schéma d'alimentation masqué 'Performances ultimes'. Élimine les micro-latences d'économie d'énergie : pas de stationnement de cœurs, pas de délai d'échelonnement de fréquence, ni suspension sélective USB.",
            broken_features: &["Consommation d'énergie en veille nettement plus élevée", "Autonomie de batterie d'ordinateur portable réduite", "Températures du processeur et du chipset plus élevées au repos"],
        }),
        "increase_gpu_tdr_timeout" => Some(LocalizedTweakText {
            title: "Augmenter le délai de détection et récupération de délai d'attente GPU",
            description: "Augmente TdrDelay de 2 à 10 secondes, donnant au processeur graphique plus de temps pour achever les shaders lourds avant que Windows ne réinitialise le pilote. Évite les faux blocages TDR lors de calculs intenses.",
            broken_features: &["Les véritables blocages GPU mettront plus de temps à être récupérés", "L'écran peut sembler figé plus longtemps avant la réinitialisation du pilote"],
        }),
        "increase_irp_stack_size" => Some(LocalizedTweakText {
            title: "Augmenter la taille de la pile IRP pour les E/S réseau et disque",
            description: "Augmente la taille de la pile des paquets de requêtes d'E/S de 15 à 32 entrées pour LanmanServer. Évite les erreurs STATUS_INSUFFICIENT_RESOURCES et améliore le débit lors de l'utilisation de filtres antivirus.",
            broken_features: &[],
        }),
        "increase_max_connections_per_server" => Some(LocalizedTweakText {
            title: "Augmenter les connexions HTTP maximales par serveur",
            description: "Porte MaxConnectionsPerServer et MaxConnectionsPer1_0Server à 16, permettant aux applications WinINet (Edge, Explorer, Store) d'ouvrir plus de connexions simultanées, accélérant téléchargements et transferts.",
            broken_features: &[],
        }),
        "increase_nonpaged_pool_quota" => Some(LocalizedTweakText {
            title: "Augmenter le quota du pool non paginé pour la stabilité des pilotes",
            description: "Définit NonPagedPoolQuota à 0 (maximum géré par le système), permettant au gestionnaire de mémoire d'utiliser toute la mémoire physique disponible pour les allocations du pool non paginé.",
            broken_features: &[],
        }),
        "optimize_additional_critical_worker_threads" => Some(LocalizedTweakText {
            title: "Augmenter les threads de travail supplémentaires du noyau",
            description: "Définit AdditionalCriticalWorkerThreads à 2 et AdditionalDelayedWorkerThreads à 2, provisionnant des threads de travail supplémentaires en mode noyau pour éviter les blocages de files d'attente d'E/S.",
            broken_features: &[],
        }),
        "optimize_default_ttl" => Some(LocalizedTweakText {
            title: "Définir la durée de vie (TTL) optimale des paquets IP",
            description: "Définit la valeur par défaut du Time-To-Live (TTL) IP à 64 sauts, correspondant à la norme Linux/macOS. Une valeur de 64 est idéale pour l'ensemble des topologies de routage actuelles.",
            broken_features: &[],
        }),
        "optimize_disk_io_page_lock_limit" => Some(LocalizedTweakText {
            title: "Augmenter la limite de verrouillage de pages d'E/S",
            description: "Définit IoPageLockLimit à 983040 octets (960 Ko), augmentant la mémoire verrouillable pour les transferts directs d'E/S, améliorant les transferts DMA volumineux sur SSD NVMe.",
            broken_features: &[],
        }),
        "optimize_dns_cache_parameters" => Some(LocalizedTweakText {
            title: "Optimiser la taille du cache et le TTL du client DNS",
            description: "Définit MaxCacheEntryTtlLimit=86400 et MaxNegativeCacheTtl=5, permettant au résolveur DNS de conserver les résolutions réussies plus longtemps tout en réessayant rapidement les échecs.",
            broken_features: &[],
        }),
        "optimize_gpu_preemption_level" => Some(LocalizedTweakText {
            title: "Régler le planificateur GPU sur la préemption de calcul uniquement",
            description: "Définit GpuPreemptionLevel=0 dans le planificateur de pilotes graphiques. Empêche l'interruption des shaders lourds en cours d'exécution, réduisant les micro-saccades et la latence en jeu.",
            broken_features: &["La détection et la récupération de délai d'attente GPU (TDR) peuvent se déclencher plus lentement"],
        }),
        "optimize_interrupt_steering" => Some(LocalizedTweakText {
            title: "Optimiser l'affinité des interruptions et la politique de routage",
            description: "Configure la politique de routage des interruptions en définissant InterruptSteeringDisabled=0 et DistributeTimers=1, répartissant les interruptions matérielles sur tous les cœurs plutôt que sur le CPU 0.",
            broken_features: &[],
        }),
        "optimize_mmcss_proaudio_scheduling" => Some(LocalizedTweakText {
            title: "MMCSS Pro Audio — Priorité maximale de planification",
            description: "Définit la catégorie de planification et la priorité SFIO de la tâche Pro Audio de MMCSS sur 'High', garantissant aux threads audio temps réel la priorité multimédia la plus élevée dans les STAN et logiciels de musique.",
            broken_features: &[],
        }),
        "optimize_ntfs_memory_usage" => Some(LocalizedTweakText {
            title: "Mise en cache agressive du pool paginé NTFS",
            description: "Définit NtfsMemoryUsage à 2, demandant au pilote NTFS d'utiliser une plus grande allocation du pool paginé pour la mise en cache des répertoires et métadonnées, améliorant les performances de compilation.",
            broken_features: &[],
        }),
        "optimize_object_manager_quota" => Some(LocalizedTweakText {
            title: "Augmenter le quota de descripteurs du Gestionnaire d'objets du noyau",
            description: "Augmente la limite de table de descripteurs via SessionPoolSize=48 dans Memory Management, évitant la saturation lors de l'exécution simultanée de nombreux processus, EDI et compilateurs.",
            broken_features: &[],
        }),
        "optimize_timer_resolution_requests" => Some(LocalizedTweakText {
            title: "Autoriser les applications à demander une haute résolution de minuterie",
            description: "Définit GlobalTimerResolutionRequests=1 dans la clé du noyau, permettant aux applications de demander la résolution de minuterie maximale (0,5 ms) sur l'ensemble du système pour éliminer le jitter audio.",
            broken_features: &["Consommation d'énergie en veille plus élevée due à une fréquence d'interruption de minuterie accrue"],
        }),
        // === gaming_latency_ultra.toml (10) ===
        "disable_driver_search_order_probing" => Some(LocalizedTweakText {
            title: "Désactiver la recherche de pilotes en ligne Windows Update",
            description: "Empêche Windows Update d'interroger automatiquement ses serveurs en arrière-plan lors de la recherche de pilotes. Élimine les délais d'installation de périphériques et les requêtes réseau.",
            broken_features: &["La recherche automatique de pilotes en ligne via Windows Update ne s'exécutera pas lors de la connexion d'un nouveau périphérique"],
        }),
        "disable_fast_user_switching" => Some(LocalizedTweakText {
            title: "Désactiver le basculement rapide d'utilisateur",
            description: "Désactive le basculement rapide d'utilisateur pour éviter que les sessions inactives et leurs tâches ne restent en RAM physique, libérant la bande passante mémoire et le processeur.",
            broken_features: &["Plusieurs sessions utilisateur locales ne peuvent plus rester ouvertes simultanément"],
        }),
        "disable_ip_source_routing" => Some(LocalizedTweakText {
            title: "Désactiver le routage par la source IP",
            description: "Définit DisableIPSourceRouting=2 pour rejeter tous les paquets IP routés par la source. Empêche l'usurpation de paquets, le détournement d'itinéraire et la surcharge de la pile réseau.",
            broken_features: &["Les outils de diagnostic réseau utilisant le routage par la source (rares) seront rejetés"],
        }),
        "disable_netsh_teredo_tunneling" => Some(LocalizedTweakText {
            title: "Désactiver le tunneling de transition IPv6 Teredo",
            description: "Désactive l'interface de tunneling Teredo en définissant DisabledComponents à 32 dans IPv6. Teredo encapsule IPv6 dans des paquets UDP IPv4, introduisant une latence de traversée NAT.",
            broken_features: &["Les anciens jeux multijoueurs P2P Xbox Live dépendant strictement de la traversée NAT Teredo ne fonctionneront pas"],
        }),
        "disable_network_adapter_power_saving" => Some(LocalizedTweakText {
            title: "Désactiver l'économie d'énergie PnP de la carte réseau NDIS",
            description: "Configure NDIS PnPCapabilities à 24 pour désactiver la mise en veille des cartes réseau. Empêche la carte réseau de passer en mode veille durant les pauses de jeu, éliminant les latences de paquets.",
            broken_features: &["Le réveil par le réseau (Wake-on-LAN) et la mise en veille de l'interface réseau seront désactivés"],
        }),
        "disable_remote_desktop_udp" => Some(LocalizedTweakText {
            title: "Désactiver le transport UDP du client Bureau à distance",
            description: "Force le client Bureau à distance (RDP) à utiliser TCP au lieu de UDP. Élimine la saturation de mémoire tampon UDP et empêche le trafic RDP d'interférer avec les paquets de jeu UDP.",
            broken_features: &["Les sessions Bureau à distance sur les connexions WAN à latence élevée utiliseront TCP au lieu d'UDP"],
        }),
        "disable_smart_card_service" => Some(LocalizedTweakText {
            title: "Désactiver le service de carte à puce (SCardSvr)",
            description: "Désactive le service de carte à puce en définissant Start=4. Élimine les interrogations en arrière-plan des lecteurs de carte à puce et des jetons cryptographiques sur les PC de jeu.",
            broken_features: &["L'authentification physique par carte à puce et le matériel de lecture de carte ne fonctionneront pas"],
        }),
        "optimize_kerberos_ticket_cache" => Some(LocalizedTweakText {
            title: "Optimiser le tampon MaxTokenSize de Kerberos",
            description: "Fixe le tampon MaxTokenSize de Kerberos à 65535 octets, évitant les dépassements de mémoire tampon et les renégociations de tickets répétées dans les environnements à groupes complexes.",
            broken_features: &[],
        }),
        "optimize_tcp_syn_attack_protection" => Some(LocalizedTweakText {
            title: "Activer la protection contre les attaques TCP SYN",
            description: "Active la défense contre les inondations TCP SYN dans la pile TCP/IP (SynAttackProtect=1). Accélère les délais de négociation lors d'afflux de connexions, protégeant les sockets de jeu.",
            broken_features: &[],
        }),
        "optimize_usb_selective_suspend_hub" => Some(LocalizedTweakText {
            title: "Désactiver la suspension sélective globale des hubs USB",
            description: "Désactive la suspension sélective USB au niveau du pilote racine (DisableSelectiveSuspend=1). Empêche les souris et claviers de jeu d'entrer en veille, éliminant la latence de réveil.",
            broken_features: &["Consommation d'énergie au repos légèrement plus élevée sur le bus USB", "Autonomie de batterie réduite sur les ordinateurs portables"],
        }),
        "remove_webexperience_package" => Some(LocalizedTweakText {
            title: "Supprimer le package Windows WebExperienceHost (Hôte des widgets)",
            description: "Supprime le package MicrosoftWindows.Client.WebExperience pour empêcher les widgets de la barre des tâches de consommer de la mémoire Edge WebView2 en arrière-plan.",
            broken_features: &["Les widgets de la barre des tâches et les flux d'actualités seront définitivement supprimés."],
        }),
        "disable_wpbt_execution" => Some(LocalizedTweakText {
            title: "Désactiver l'exécution des binaires du micrologiciel WPBT",
            description: "Empêche le noyau Windows d'exécuter des binaires intégrés dans la table WPBT de l'UEFI de la carte mère.",
            broken_features: &[],
        }),
        "service_shutdown_timeout_15s" => Some(LocalizedTweakText {
            title: "Réduire le délai d'arrêt des services à 15 secondes",
            description: "Réduit le délai d'arrêt des services de 20 s à 15 s pour éviter les blocages lors de l'extinction de Windows.",
            broken_features: &[],
        }),
        "start_menu_show_more_pins" => Some(LocalizedTweakText {
            title: "Menu Démarrer : afficher plus d'épingles",
            description: "Configure le menu Démarrer de Windows 11 pour donner la priorité aux applications épinglées sur les recommandations.",
            broken_features: &[],
        }),
        "disable_taskbar_people_button" => Some(LocalizedTweakText {
            title: "Désactiver le bouton Contacts de la barre des tâches",
            description: "Masque et désactive l'icône Contacts de la barre des tâches Windows.",
            broken_features: &[],
        }),
        "repair_ndu_graph" => Some(LocalizedTweakText {
            title: "Réparer le graphique d'utilisation réseau (NDU) du Gestionnaire des tâches",
            description: "Garantit que le pilote de noyau NDU est défini sur Start=2 (Automatique) pour que les graphiques réseau du Gestionnaire des tâches fonctionnent correctement.",
            broken_features: &[],
        }),
        "disable_settings_ads" => Some(LocalizedTweakText {
            title: "Désactiver les contenus suggérés et les publicités dans les Paramètres",
            description: "Empêche Microsoft d'afficher des applications suggérées, des promotions de partenaires et des conseils dans les Paramètres Windows.",
            broken_features: &["Les cartes promotionnelles et les conseils dans Paramètres sont masqués."],
        }),
        "disable_start_menu_suggested_apps" => Some(LocalizedTweakText {
            title: "Désactiver l'installation automatique des applications suggérées",
            description: "Empêche Windows Content Delivery Manager de télécharger et d'épingler silencieusement des applications et jeux suggérés.",
            broken_features: &["Les jeux sponsorisés ne seront pas installés automatiquement dans le menu Démarrer."],
        }),
        "disable_hardware_companion_software" => Some(LocalizedTweakText {
            title: "Désactiver les recommandations de logiciels compagnons matériels",
            description: "Empêche Windows de notifier et recommander automatiquement des logiciels compagnons OEM lors du branchement de périphériques.",
            broken_features: &[],
        }),
        "disable_ms_office_telemetry" => Some(LocalizedTweakText {
            title: "Désactiver la télémétrie Microsoft Office et les journaux CEIP",
            description: "Désactive la collecte des données de diagnostic, les demandes de commentaires et la télémétrie du Programme d'amélioration de l'expérience utilisateur (CEIP) d'Office.",
            broken_features: &[],
        }),
        "set_this_pc_as_default_explorer" => Some(LocalizedTweakText {
            title: "Définir 'Ce PC' comme vue par défaut de l'Explorateur de fichiers",
            description: "Configure l'Explorateur de fichiers pour qu'il s'ouvre sur 'Ce PC' plutôt que sur 'Accueil', évitant ainsi les ralentissements au lancement.",
            broken_features: &[],
        }),
        "enable_taskbar_end_task" => Some(LocalizedTweakText {
            title: "Activer 'Fin de tâche' par clic droit dans la barre des tâches",
            description: "Active l'option native Windows 11 'Fin de tâche' directement dans le menu contextuel de la barre des tâches sans ouvrir le Gestionnaire des tâches.",
            broken_features: &[],
        }),
        "disable_oobe_privacy_experience" => Some(LocalizedTweakText {
            title: "Désactiver les invites de configuration de confidentialité OOBE",
            description: "Supprime les invites multipages de paramètres de confidentialité OOBE lors de la création d'utilisateurs et après les mises à jour.",
            broken_features: &[],
        }),
        "disable_explorer_sync_provider_ads" => Some(LocalizedTweakText {
            title: "Désactiver les publicités de synchronisation dans l'Explorateur",
            description: "Empêche l'Explorateur d'afficher des bannières promotionnelles pour OneDrive et Microsoft 365.",
            broken_features: &[],
        }),
        "disable_taskbar_task_view" => Some(LocalizedTweakText {
            title: "Désactiver le bouton Affichage des tâches dans la barre des tâches",
            description: "Masque l'icône Affichage des tâches de la barre des tâches pour libérer de l'espace tout en conservant Alt+Tab.",
            broken_features: &[],
        }),
        "hide_taskbar_meet_now" => Some(LocalizedTweakText {
            title: "Masquer le bouton 'Réunion immédiate' et Chat de la barre des tâches",
            description: "Masque le bouton de conversation Microsoft Teams et Réunion immédiate de la barre des tâches.",
            broken_features: &[],
        }),
        "disable_background_apps_global" => Some(LocalizedTweakText {
            title: "Désactiver l'exécution des applications en arrière-plan globalement",
            description: "Empêche les applications du Windows Store et UWP de s'exécuter en arrière-plan et de consommer des ressources lorsqu'elles sont fermées.",
            broken_features: &[],
        }),
        "disable_finish_setup_ads" => Some(LocalizedTweakText {
            title: "Désactiver les invites plein écran 'Terminer la configuration'",
            description: "Désactive les écrans de rappel récurrents incitant à s'abonner à Microsoft 365 ou OneDrive après le démarrage.",
            broken_features: &[],
        }),
        "disable_lockscreen_spotlight_ads" => Some(LocalizedTweakText {
            title: "Désactiver les conseils et publicités sur l'écran de verrouillage",
            description: "Supprime les bannières publicitaires, astuces et promotions affichées sur l'écran de verrouillage de Windows.",
            broken_features: &[],
        }),
        "disable_welcome_experience_ads" => Some(LocalizedTweakText {
            title: "Désactiver les écrans de bienvenue promotionnels après mise à jour",
            description: "Bloque les fenêtres 'Nouveautés' et d'expérience d'accueil affichant du contenu marketing pour Edge ou Copilot.",
            broken_features: &[],
        }),
        "disable_general_tips_ads" => Some(LocalizedTweakText {
            title: "Désactiver les suggestions, astuces et conseils de Windows",
            description: "Désactive les conseils génériques et recommandations de fonctionnalités poussés par ContentDeliveryManager.",
            broken_features: &[],
        }),
        "enable_windowed_game_optimizations" => Some(LocalizedTweakText {
            title: "Activer les optimisations pour les jeux en mode fenêtré",
            description: "Active le modèle de présentation moderne de Windows 11 (flip model) pour les jeux fenêtrés, réduisant la latence et activant VRR.",
            broken_features: &[],
        }),
        _ => None,
    }
}

/// Localized tweak title for this language.
#[allow(dead_code)]
pub fn tweak_title(id: &str) -> Option<&'static str> {
    match id {
        "disable_dwm_animations_per_process" => {
            Some("Désactiver les animations de transition DWM par processus")
        }
        "disable_dwm_composition_throttle" => {
            Some("Désactiver la limitation de composition du Gestionnaire de fenêtres du Bureau")
        }
        "disable_kernel_mitigations_audit" => {
            Some("Désactiver le mode d'audit des atténuations du noyau")
        }
        "disable_llmnr_protocol" => Some("Désactiver le protocole de résolution de noms LLMNR"),
        "disable_memory_compression" => Some("Désactiver le moteur de compression de mémoire"),
        "disable_netbios_over_tcp" => Some("Désactiver NetBIOS sur TCP/IP"),
        "disable_ntfs_encryption_driver" => {
            Some("Désactiver le pilote de chiffrement de fichiers NTFS (EFS)")
        }
        "disable_page_file_encryption" => {
            Some("Désactiver le chiffrement du fichier d'échange et l'effacement à l'arrêt")
        }
        "disable_processor_idle_demote_threshold" => {
            Some("Désactiver la rétrogradation d'état d'inactivité du processeur")
        }
        "disable_rss_base_cpu" => {
            Some("Définir le processeur de base RSS pour la répartition réseau")
        }
        "disable_spectre_meltdown_mitigations" => {
            Some("Désactiver les atténuations de failles processeur Spectre/Meltdown")
        }
        "disable_threaded_dpc_optimization" => {
            Some("Désactiver les DPC threadés pour une latence d'interruption réduite")
        }
        "disable_windows_search_indexer" => {
            Some("Désactiver le service d'indexation Windows Search")
        }
        "enable_hardware_task_offload" => {
            Some("Activer le déchargement matériel des tâches TCP/IP")
        }
        "enable_large_ntfs_frs" => {
            Some("Activer les grands segments d'enregistrement de fichiers NTFS")
        }
        "enable_ultimate_performance_plan" => {
            Some("Activer le mode d'alimentation Performances ultimes")
        }
        "increase_gpu_tdr_timeout" => {
            Some("Augmenter le délai de détection et récupération de délai d'attente GPU")
        }
        "increase_irp_stack_size" => {
            Some("Augmenter la taille de la pile IRP pour les E/S réseau et disque")
        }
        "increase_max_connections_per_server" => {
            Some("Augmenter les connexions HTTP maximales par serveur")
        }
        "increase_nonpaged_pool_quota" => {
            Some("Augmenter le quota du pool non paginé pour la stabilité des pilotes")
        }
        "optimize_additional_critical_worker_threads" => {
            Some("Augmenter les threads de travail supplémentaires du noyau")
        }
        "optimize_default_ttl" => Some("Définir la durée de vie (TTL) optimale des paquets IP"),
        "optimize_disk_io_page_lock_limit" => {
            Some("Augmenter la limite de verrouillage de pages d'E/S")
        }
        "optimize_dns_cache_parameters" => {
            Some("Optimiser la taille du cache et le TTL du client DNS")
        }
        "optimize_gpu_preemption_level" => {
            Some("Régler le planificateur GPU sur la préemption de calcul uniquement")
        }
        "optimize_interrupt_steering" => {
            Some("Optimiser l'affinité des interruptions et la politique de routage")
        }
        "optimize_mmcss_proaudio_scheduling" => {
            Some("MMCSS Pro Audio — Priorité maximale de planification")
        }
        "optimize_ntfs_memory_usage" => Some("Mise en cache agressive du pool paginé NTFS"),
        "optimize_object_manager_quota" => {
            Some("Augmenter le quota de descripteurs du Gestionnaire d'objets du noyau")
        }
        "optimize_timer_resolution_requests" => {
            Some("Autoriser les applications à demander une haute résolution de minuterie")
        }
        "disable_driver_search_order_probing" => {
            Some("Désactiver la recherche de pilotes en ligne Windows Update")
        }
        "disable_fast_user_switching" => Some("Désactiver le basculement rapide d'utilisateur"),
        "disable_ip_source_routing" => Some("Désactiver le routage par la source IP"),
        "disable_netsh_teredo_tunneling" => {
            Some("Désactiver le tunneling de transition IPv6 Teredo")
        }
        "disable_network_adapter_power_saving" => {
            Some("Désactiver l'économie d'énergie PnP de la carte réseau NDIS")
        }
        "disable_remote_desktop_udp" => {
            Some("Désactiver le transport UDP du client Bureau à distance")
        }
        "disable_smart_card_service" => Some("Désactiver le service de carte à puce (SCardSvr)"),
        "optimize_kerberos_ticket_cache" => Some("Optimiser le tampon MaxTokenSize de Kerberos"),
        "optimize_tcp_syn_attack_protection" => {
            Some("Activer la protection contre les attaques TCP SYN")
        }
        "optimize_usb_selective_suspend_hub" => {
            Some("Désactiver la suspension sélective globale des hubs USB")
        }
        _ => lookup(id).map(|t| t.title),
    }
}

/// Localized tweak description for this language.
#[allow(dead_code)]
pub fn tweak_description(id: &str) -> Option<&'static str> {
    match id {
        "disable_dwm_animations_per_process" => Some("Désactive les animations de transition DWM par processus en définissant DisallowAnimations=1 et AnimationsShiftKey=1 dans la clé DWM. Supprime les effets de fondu et de glissement lors de la réduction, l'agrandissement ou le basculement des fenêtres, réduisant la charge du compositeur GPU."),
        "disable_dwm_composition_throttle" => Some("Définit DesktopCompositionThrottle=0 dans la clé DWM, empêchant le compositeur de brider le rendu lors de pertes d'images. Permet à DWM de maintenir un débit de composition maximal même sous charge GPU."),
        "disable_kernel_mitigations_audit" => Some("Désactive la journalisation d'audit des atténuations d'exploits du noyau en réinitialisant MitigationAuditOptions à zéro. Réduit les événements ETW lors de la création de processus et du traitement des exceptions."),
        "disable_llmnr_protocol" => Some("Désactive le protocole LLMNR en définissant EnableMulticast=0 dans les paramètres du client DNS. Élimine le trafic de multidiffusion inutile et protège contre les attaques de relais et d'empoisonnement LLMNR."),
        "disable_memory_compression" => Some("Désactive le magasin de compression du Gestionnaire de mémoire Windows en définissant DisableCompression=1. Sur les systèmes dotés de 16 Go de RAM ou plus, cela supprime les pics d'utilisation processeur causés par le thread de compression."),
        "disable_netbios_over_tcp" => Some("Désactive le protocole NetBIOS sur TCP/IP en définissant NetbiosOptions=2 et en désactivant la recherche LMHosts. NetBIOS est un protocole LAN obsolète qui diffuse des requêtes de résolution de noms, exposant le système à du trafic parasite et aux attaques d'empoisonnement NBNS."),
        "disable_ntfs_encryption_driver" => Some("Définit NtfsDisableEncryption=1 dans la clé FileSystem, empêchant le pilote de filtre EFS d'intercepter les opérations d'E/S de fichiers. Réduit la latence d'accès aux fichiers sur les systèmes n'utilisant pas le chiffrement EFS."),
        "disable_page_file_encryption" => Some("Empêche Windows de chiffrer le fichier d'échange et désactive son effacement à l'arrêt (ClearPageFileAtShutdown=0, DisablePagingEncryption=1). Améliore le débit de pagination et réduit le temps d'extinction du système."),
        "disable_processor_idle_demote_threshold" => Some("Définit IdleDemoteThreshold=100 et IdlePromoteThreshold=100 dans la gestion d'alimentation du processeur. Maintient le processeur à son niveau de performance maximal même pendant de courts intervalles d'inactivité, éliminant les pics de latence liés à la montée en fréquence."),
        "disable_rss_base_cpu" => Some("Définit RssBaseCpu=1 dans les paramètres NDIS, demandant au Receive Side Scaling de démarrer le traitement des interruptions réseau à partir du processeur 1 au lieu du processeur 0. Libère le processeur d'amorçage pour les applications de premier plan."),
        "disable_spectre_meltdown_mitigations" => Some("Désactive les atténuations matérielles des vulnérabilités processeur Spectre et Meltdown en définissant FeatureSettingsOverride=3 et FeatureSettingsOverrideMask=3. Supprime la pénalité de performances sur les charges de calcul intensives."),
        "disable_threaded_dpc_optimization" => Some("Définit ThreadDpcEnable=0 dans la clé du noyau, forçant l'exécution des appels de procédure différée à DISPATCH_LEVEL au lieu d'un thread dédié à PASSIVE_LEVEL. Réduit la latence DPC pour la production audio en temps réel et le jeu compétitif."),
        "disable_windows_search_indexer" => Some("Désactive le service WSearch, empêchant l'indexeur d'analyser en permanence les fichiers en arrière-plan. Sur les SSD/NVMe, cela supprime les E/S disque, l'utilisation CPU et la mémoire consommées par SearchIndexer.exe."),
        "enable_hardware_task_offload" => Some("Définit DisableTaskOffload=0 dans les paramètres TCP/IP, garantissant que l'OS décharge le calcul des sommes de contrôle et la segmentation TCP sur la carte réseau, libérant des cycles CPU."),
        "enable_large_ntfs_frs" => Some("Définit NtfsAllowExtendedCharacterIn8dot3Name=0, permettant au pilote NTFS d'allouer des segments d'enregistrement plus volumineux et efficaces, réduisant la fragmentation de la MFT."),
        "enable_ultimate_performance_plan" => Some("Active le schéma d'alimentation masqué 'Performances ultimes'. Élimine les micro-latences d'économie d'énergie : pas de stationnement de cœurs, pas de délai d'échelonnement de fréquence, ni suspension sélective USB."),
        "increase_gpu_tdr_timeout" => Some("Augmente TdrDelay de 2 à 10 secondes, donnant au processeur graphique plus de temps pour achever les shaders lourds avant que Windows ne réinitialise le pilote. Évite les faux blocages TDR lors de calculs intenses."),
        "increase_irp_stack_size" => Some("Augmente la taille de la pile des paquets de requêtes d'E/S de 15 à 32 entrées pour LanmanServer. Évite les erreurs STATUS_INSUFFICIENT_RESOURCES et améliore le débit lors de l'utilisation de filtres antivirus."),
        "increase_max_connections_per_server" => Some("Porte MaxConnectionsPerServer et MaxConnectionsPer1_0Server à 16, permettant aux applications WinINet (Edge, Explorer, Store) d'ouvrir plus de connexions simultanées, accélérant téléchargements et transferts."),
        "increase_nonpaged_pool_quota" => Some("Définit NonPagedPoolQuota à 0 (maximum géré par le système), permettant au gestionnaire de mémoire d'utiliser toute la mémoire physique disponible pour les allocations du pool non paginé."),
        "optimize_additional_critical_worker_threads" => Some("Définit AdditionalCriticalWorkerThreads à 2 et AdditionalDelayedWorkerThreads à 2, provisionnant des threads de travail supplémentaires en mode noyau pour éviter les blocages de files d'attente d'E/S."),
        "optimize_default_ttl" => Some("Définit la valeur par défaut du Time-To-Live (TTL) IP à 64 sauts, correspondant à la norme Linux/macOS. Une valeur de 64 est idéale pour l'ensemble des topologies de routage actuelles."),
        "optimize_disk_io_page_lock_limit" => Some("Définit IoPageLockLimit à 983040 octets (960 Ko), augmentant la mémoire verrouillable pour les transferts directs d'E/S, améliorant les transferts DMA volumineux sur SSD NVMe."),
        "optimize_dns_cache_parameters" => Some("Définit MaxCacheEntryTtlLimit=86400 et MaxNegativeCacheTtl=5, permettant au résolveur DNS de conserver les résolutions réussies plus longtemps tout en réessayant rapidement les échecs."),
        "optimize_gpu_preemption_level" => Some("Définit GpuPreemptionLevel=0 dans le planificateur de pilotes graphiques. Empêche l'interruption des shaders lourds en cours d'exécution, réduisant les micro-saccades et la latence en jeu."),
        "optimize_interrupt_steering" => Some("Configure la politique de routage des interruptions en définissant InterruptSteeringDisabled=0 et DistributeTimers=1, répartissant les interruptions matérielles sur tous les cœurs plutôt que sur le CPU 0."),
        "optimize_mmcss_proaudio_scheduling" => Some("Définit la catégorie de planification et la priorité SFIO de la tâche Pro Audio de MMCSS sur 'High', garantissant aux threads audio temps réel la priorité multimédia la plus élevée dans les STAN et logiciels de musique."),
        "optimize_ntfs_memory_usage" => Some("Définit NtfsMemoryUsage à 2, demandant au pilote NTFS d'utiliser une plus grande allocation du pool paginé pour la mise en cache des répertoires et métadonnées, améliorant les performances de compilation."),
        "optimize_object_manager_quota" => Some("Augmente la limite de table de descripteurs via SessionPoolSize=48 dans Memory Management, évitant la saturation lors de l'exécution simultanée de nombreux processus, EDI et compilateurs."),
        "optimize_timer_resolution_requests" => Some("Définit GlobalTimerResolutionRequests=1 dans la clé du noyau, permettant aux applications de demander la résolution de minuterie maximale (0,5 ms) sur l'ensemble du système pour éliminer le jitter audio."),
                "disable_driver_search_order_probing" => Some("Empêche Windows Update d'interroger automatiquement ses serveurs en arrière-plan lors de la recherche de pilotes. Élimine les délais d'installation de périphériques et les requêtes réseau."),
        "disable_fast_user_switching" => Some("Désactive le basculement rapide d'utilisateur pour éviter que les sessions inactives et leurs tâches ne restent en RAM physique, libérant la bande passante mémoire et le processeur."),
        "disable_ip_source_routing" => Some("Définit DisableIPSourceRouting=2 pour rejeter tous les paquets IP routés par la source. Empêche l'usurpation de paquets, le détournement d'itinéraire et la surcharge de la pile réseau."),
        "disable_netsh_teredo_tunneling" => Some("Désactive l'interface de tunneling Teredo en définissant DisabledComponents à 32 dans IPv6. Teredo encapsule IPv6 dans des paquets UDP IPv4, introduisant une latence de traversée NAT."),
        "disable_network_adapter_power_saving" => Some("Configure NDIS PnPCapabilities à 24 pour désactiver la mise en veille des cartes réseau. Empêche la carte réseau de passer en mode veille durant les pauses de jeu, éliminant les latences de paquets."),
        "disable_remote_desktop_udp" => Some("Force le client Bureau à distance (RDP) à utiliser TCP au lieu de UDP. Élimine la saturation de mémoire tampon UDP et empêche le trafic RDP d'interférer avec les paquets de jeu UDP."),
        "disable_smart_card_service" => Some("Désactive le service de carte à puce en définissant Start=4. Élimine les interrogations en arrière-plan des lecteurs de carte à puce et des jetons cryptographiques sur les PC de jeu."),
        "optimize_kerberos_ticket_cache" => Some("Fixe le tampon MaxTokenSize de Kerberos à 65535 octets, évitant les dépassements de mémoire tampon et les renégociations de tickets répétées dans les environnements à groupes complexes."),
        "optimize_tcp_syn_attack_protection" => Some("Active la défense contre les inondations TCP SYN dans la pile TCP/IP (SynAttackProtect=1). Accélère les délais de négociation lors d'afflux de connexions, protégeant les sockets de jeu."),
        "optimize_usb_selective_suspend_hub" => Some("Désactive la suspension sélective USB au niveau du pilote racine (DisableSelectiveSuspend=1). Empêche les souris et claviers de jeu d'entrer en veille, éliminant la latence de réveil."),
        _ => lookup(id).map(|t| t.description),
    }
}
