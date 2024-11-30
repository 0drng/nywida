use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOCALE: String = std::env::var("LANG")
    .unwrap_or("en".to_owned())
    .split("_")
    .nth(0)
    .unwrap()
    .to_lowercase();

    static ref TRANSLATIONS:HashMap<String, HashMap<Labels, String>> = HashMap::from([
        ("ce".to_owned(), HashMap::from([
            (Labels::Error_PackageManager_NotInstalled, "错误：未安装软件包管理器 {0}".to_owned()),
            (Labels::Error_Which_NotInstalled, "错误：未安装程序 which，因此无法解析依赖关系".to_owned()),
            (Labels::Error_UserAbort, "错误：用户已中止操作".to_owned()),
            (Labels::Error_NoRoot, "程序应以 root 用户身份执行。尽量使用 sudo/doas。".to_owned()),
            (Labels::Error_NoAURHelper, "错误：未找到 AUR 助手".to_owned()),
            (Labels::Error_InstallationFailed, "错误：安装软件包失败".to_owned()),
            (Labels::Error_CommandFailed, "错误：无法生成命令".to_owned()),
            (Labels::Error_CommandStdoutFailed, "错误：无法从命令获取标准输出".to_owned()),
            (Labels::Info_ConfirmContinue, "信息：是否要继续？y/N: ".to_owned()),
            (Labels::Info_StartingPackageInstallation, "信息：开始安装软件包：{0}".to_owned()),
            (Labels::Info_ExecutingPreScript, "信息：执行前置脚本：{0}".to_owned()),
            (Labels::Info_ExecutingPostScript, "信息：执行后续脚本：{0}".to_owned()),
            (Labels::Info_NewlyInstalledPackages, "信息：{0} 个新/已安装的软件包".to_owned()),
            (Labels::Info_CopyingFile, "信息：将文件 {0} 复制到 {1}".to_owned()),
        ])),
        ("de".to_owned(), HashMap::from([
            (Labels::Error_PackageManager_NotInstalled, "FEHLER: Der Paket-Manager {0} ist nicht installiert".to_owned()),
            (Labels::Error_Which_NotInstalled, "FEHLER: Das Programm which wurde nicht installiert. Dadurch können Abhängigkeiten nicht ermittelt werden".to_owned()),
            (Labels::Error_UserAbort, "FEHLER: Der Nutzer hat den Vorgang abgebrochen".to_owned()),
            (Labels::Error_NoRoot, "Das Programm muss mit dem root Benutzer ausgeführt werden. Versuche es mit sudo/doas.".to_owned()),
            (Labels::Error_NoAURHelper, "FEHLER: Kein AUR-Helfer gefunden".to_owned()),
            (Labels::Error_InstallationFailed, "FEHLER: Installation der Pakete fehlgeschlagen".to_owned()),
            (Labels::Error_CommandFailed, "FEHLER: Der Befehl konnte nicht ausgeführt werden".to_owned()),
            (Labels::Error_CommandStdoutFailed, "FEHLER: Die Standardausgabe des Befehls konnte nicht gelesen werden".to_owned()),
            (Labels::Info_ConfirmContinue, "INFO: Möchten Sie fortfahren? y/N: ".to_owned()),
            (Labels::Info_StartingPackageInstallation, "INFO: Start der Paketinstallation von: {0}".to_owned()),
            (Labels::Info_ExecutingPreScript, "INFO: Ausführen des vorherigen Skripts: {0}".to_owned()),
            (Labels::Info_ExecutingPostScript, "INFO: Ausführen des Folge-Scriptes: {0}".to_owned()),
            (Labels::Info_NewlyInstalledPackages, "INFO: {0} neu/installierte Pakete".to_owned()),
            (Labels::Info_CopyingFile, "INFO: Kopiere Datei {0} nach {1}".to_owned()),
        ])),
        ("en".to_owned(), HashMap::from([
            (Labels::Error_PackageManager_NotInstalled, "ERROR: The package manager {0} is not installed".to_owned()),
            (Labels::Error_Which_NotInstalled, "ERROR: The program which is not installed. This prevents dependency resolution".to_owned()),
            (Labels::Error_UserAbort, "ERROR: The user has aborted the operation".to_owned()),
            (Labels::Error_NoRoot, "ERROR: The program should be run as root. Try using sudo/doas.".to_owned()),
            (Labels::Error_NoAURHelper, "ERROR: No AUR helper found".to_owned()),
            (Labels::Error_InstallationFailed, "ERROR: Failed to install packages".to_owned()),
            (Labels::Error_CommandFailed, "ERROR: Failed to spawn command".to_owned()),
            (Labels::Error_CommandStdoutFailed, "ERROR: Failed to get stdout from command".to_owned()),
            (Labels::Info_ConfirmContinue, "INFO: Do you want to continue? y/N: ".to_owned()),
            (Labels::Info_StartingPackageInstallation, "INFO: Starting package installation of: {0}".to_owned()),
            (Labels::Info_ExecutingPreScript, "INFO: Executing pre-script: {0}".to_owned()),
            (Labels::Info_ExecutingPostScript, "INFO: Executing post-script: {0}".to_owned()),
            (Labels::Info_NewlyInstalledPackages, "INFO: {0} newly/installed packages".to_owned()),
            (Labels::Info_CopyingFile, "INFO: Copying file {0} to {1}".to_owned()),
        ])),
        ("ru".to_owned(), HashMap::from([
            (Labels::Error_PackageManager_NotInstalled, "ОШИБКА: Менеджер пакетов {0} не установлен".to_owned()),
            (Labels::Error_Which_NotInstalled, "ОШИБКА: Программа which не установлена. Это мешает определить зависимости".to_owned()),
            (Labels::Error_UserAbort, "ОШИБКА: Пользователь прервал операцию".to_owned()),
            (Labels::Error_NoRoot, "ОШИБКА: Программа должна быть запущена от имени root. Попробуйте использовать sudo/doas.".to_owned()),
            (Labels::Error_NoAURHelper, "ОШИБКА: AUR-хелпер не найден".to_owned()),
            (Labels::Error_InstallationFailed, "ОШИБКА: Не удалось установить пакеты".to_owned()),
            (Labels::Error_CommandFailed, "ОШИБКА: Не удалось выполнить команду".to_owned()),
            (Labels::Error_CommandStdoutFailed, "ОШИБКА: Не удалось получить стандартный вывод команды".to_owned()),
            (Labels::Info_ConfirmContinue, "ИНФО: Хотите продолжить? y/N: ".to_owned()),        
            (Labels::Info_StartingPackageInstallation, "ИНФО: Начало установки пакетов: {0}".to_owned()),
            (Labels::Info_ExecutingPreScript, "ИНФО: Выполнение предскрипта: {0}".to_owned()),
            (Labels::Info_ExecutingPostScript, "ИНФО: Выполнение последующего скрипта: {0}".to_owned()),
            (Labels::Info_NewlyInstalledPackages, "ИНФО: {0} новых/установленных пакетов".to_owned()),
            (Labels::Info_CopyingFile, "ИНФО: Копирование файла {0} в {1}".to_owned()),
        ])),
    ]);
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Hash)]
pub enum Labels {
    // ERROR
    Error_PackageManager_NotInstalled,
    Error_Which_NotInstalled,
    Error_UserAbort,
    Error_NoRoot,
    Error_NoAURHelper,
    Error_InstallationFailed,
    Error_CommandStdoutFailed,
    Error_CommandFailed,
    // INFO
    Info_ConfirmContinue,
    Info_StartingPackageInstallation,
    Info_ExecutingPreScript,
    Info_ExecutingPostScript,
    Info_NewlyInstalledPackages,
    Info_CopyingFile,
}

pub fn t(label: Labels, params: Option<Vec<String>>) -> String {
    let text: String = TRANSLATIONS.get(&LOCALE.to_string()).unwrap().get(&label).unwrap().to_owned();

    if let Some(params) = params {
        return replace(text, params);
    }

    return text;
}

fn replace(mut text: String, params: Vec<String>) -> String {
    for (i, param) in params.iter().enumerate() {
        let placeholder: String = format!("{{{}}}", i);
        text = text.replace(&placeholder, &param);
    }
    return text;
}
