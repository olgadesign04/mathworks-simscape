//! Windows registry read helpers for Mathworks Simscape.

pub const APP_KEY: &str = "Software\\Mathworks\\Mathworks Simscape";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Mathworks Simscape";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Mathworks Simscape") },
        RegValue { name: "Version",     value: String::from("v2.2.14") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Mathworks Simscape") },
        RegValue { name: "Publisher",       value: String::from("Mathworks") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Mathworks Simscape\\uninstall.exe\"") },
    ]
}
