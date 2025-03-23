use windows_registry::*;

//TODO: Create for windows only execution

//create function to check windows defender settings

//check if windows defender is enabled, and settings are correct
//if settings not correct, create option to set the settings all together
pub fn check_windows_defender() -> bool {
    // Open registry for Windows Defender
    let val = LOCAL_MACHINE
    .open(r#"SOFTWARE\Microsoft\Windows Defender"#)
    .expect("Failed to get registry key")
    .get_u32("DisableAntiSpyware")
    .expect("Failed to get registry value");
val != 0
}

    //check if DisableAntiSpyware is set to 0 (enabled)
    //let defender_enabled = key.get_u32("DisableAntiSpyware")?;
    //check if DisableRealtimeMonitoring is ebabledc
    //let realtime_enabled = key.get_value("DisableRealtimeMonitoring").unwrap_or(Value::Dword(0)) != Value::(0);

    //if disabled notify user and prompt to enable

   // ok(())
