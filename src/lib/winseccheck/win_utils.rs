use windows_registry::*;

//TODO: Create for windows only execution

//create function to check windows defender settings

//check if windows defender is enabled, and settings are correct
//if settings not correct, create option to set the settings all together
pub fn check_windows_defender() -> bool {
    // Open registry for Windows Defender
    let DisableAntiSpy = LOCAL_MACHINE
        .open(r#"SOFTWARE\Microsoft\Windows Defender"#)
        .expect("Failed to get registry key")
        .get_u32("DisableAntiSpyware")
        .expect("Failed to get registry value");
    //DisableAntiSpy != 0;

    let DisableAV = LOCAL_MACHINE
        .open(r#"SOFTWARE\Microsoft\Windows Defender"#)
        .expect("Failed to get registry key")
        .get_u32("DisableAntiVirus")
        .expect("Failed to get registry value");
    //DisableAV != 0;
    let tamperProtection = LOCAL_MACHINE
        .open(r#"SOFTWARE\Microsoft\Windows Defender\Features"#)
        .expect("Failed to get registry key")
        .get_u32("DisableTamperProtection")
        .expect("Failed to get registry value");

    //check if cloud protection is enabled
    //if not enabled notify user and prompt to enable

    let cloudrotection_SubmitSamplesConsent = LOCAL_MACHINE
        .open(r#"SOFTWARE\Microsoft\Windows Defender\Spynet"#)
        .expect("Failed to get registry key")
        .get_u32("SubmitSamplesConsent")
        .expect("Failed to get registry value");

    //check if exploit protection is enabled
    //if not enabled notify user and prompt to enable

    //check if controlled folder access is enabled
    //if not enabled notify user and prompt to enable

    //check if firewall is enabled
    //if not enabled notify user and prompt to enable

    //check if network protection is enabled
    //if not enabled notify user and prompt to enable

    //check if ransomware protection is enabled
    //if not enabled notify user and prompt to enable

    println!("Windows Defender is disabled: {}", DisableAntiSpy);
    println!("Anti-Virus is disabled {}", DisableAV);
    println!("Tamper Protection is disabled {}", tamperProtection);
    //if any of the above are disabled return false, else return true

    true //ok(())
}

//check if DisableAntiSpyware is set to 0 (enabled)
//let defender_enabled = key.get_u32("DisableAntiSpyware")?;
//check if DisableRealtimeMonitoring is ebabledc
//let realtime_enabled = key.get_value("DisableRealtimeMonitoring").unwrap_or(Value::Dword(0)) != Value::(0);

//if disabled notify user and prompt to enable

// ok(())
