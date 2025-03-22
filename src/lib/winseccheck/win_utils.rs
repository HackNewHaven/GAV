use windows_registry::*;

//TODO: Create for windows only execution

//create function to check windows defender settings

//check if windows defender is enabled, and settings are correct
//if settings not correct, create option to set the settings all together
pub fn check_windows_defender() {
    // Open registry for Windows Defender
    let key = LOCAL_MACHINE.create(r"SOFTWARE\Policies\Microsoft\Windows Defender")?;

    //check if DisableAntiSpyware is set to 0 (enabled)
    let defender_enabled = key.get_value("DisableAntiSpyware").unwrap_or(0) == 0;
    //check if DisableRealtimeMonitoring is ebabled
    let realtime_enabled = key.get_value("DisableRealtimeMonitoring").unwrap_or(0) == 0;

    //if disabled notify user and prompt to enable
}

pub fn check_windows_defender_policies() {
    // Open registry for Windows Defender policies
    let key = LOCAL_MACHINE.create(r"SOFTWARE\Policies\Microsoft\Windows Defender")?;
    //check if DisableAntiSpyware is set to 0 (enabled)
    let defender_enabled = LOCAL_MACHINE.create("DisableAntiSpyware").unwrap_or(0) == 0;
    //check if DisableRealtimeMonitoring is ebabled
    let realtime_enabled = key.get_value("DisableRealtimeMonitoring").unwrap_or(0) == 0;

    //if disabled notify user and prompt to enable

}

pub fn check_currentctrlset(){
    let key = LOCAL_MACHINE.create(r"HKLM\SYSTEM\CurrentControlSet\Services\WinDefend")?;
    let win_defendenabled = key.get_value("Start").unwrap_or(4) == 4;
    
}
