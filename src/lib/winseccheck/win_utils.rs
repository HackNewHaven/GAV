use windows_registry::{Key, LOCAL_MACHINE}; // Ensure correct imports
use std::error::Error;

// Ensure this function only compiles on Windows
#[cfg(target_os = "windows")]
pub fn check_windows_defender() -> Result<bool, Box<dyn Error>> {
    // Open registry for Windows Defender
    let DisableAntiSpy = LOCAL_MACHINE
        .open(r#"SOFTWARE\Microsoft\Windows Defender"#)?
        .get_u32("DisableAntiSpyware")
        .unwrap_or(1); // Default to 1 (disabled) if value is missing

    let DisableAV = LOCAL_MACHINE
        .open(r#"SOFTWARE\Microsoft\Windows Defender"#)?
        .get_u32("DisableAntiVirus")
        .unwrap_or(1);

    let tamperProtection = LOCAL_MACHINE
        .open(r#"SOFTWARE\Microsoft\Windows Defender\Features"#)?
        .get_u32("DisableTamperProtection")
        .unwrap_or(1);

    let cloudrotection_SubmitSamplesConsent = LOCAL_MACHINE
        .open(r#"SOFTWARE\Microsoft\Windows Defender\Spynet"#)?
        .get_u32("SubmitSamplesConsent")
        .unwrap_or(0);

    println!("Windows Defender is disabled: {}", DisableAntiSpy != 0);
    println!("Anti-Virus is disabled: {}", DisableAV != 0);
    println!("Tamper Protection is disabled: {}", tamperProtection != 0);
    println!(
        "Cloud Protection Submit Samples Consent: {}",
        cloudrotection_SubmitSamplesConsent
    );

    // Return false if any critical settings are disabled
    Ok(DisableAntiSpy == 0 && DisableAV == 0 && tamperProtection == 0)
}

#[cfg(target_os = "windows")]
pub fn enable_sec_settings() -> Result<(), Box<dyn Error>> {
    // Open registry for Windows Defender
    let key = LOCAL_MACHINE.open(r#"SOFTWARE\Microsoft\Windows Defender"#)?;

    // Set critical settings to enabled
    key.set_u32("DisableAntiSpyware", 0)?;
    key.set_u32("DisableAntiVirus", 0)?;
    key.set_u32("SubmitSamplesConsent", 1)?;

    let key = LOCAL_MACHINE.open(r#"SOFTWARE\Microsoft\Windows Defender\Features"#)?;
    key.set_u32("DisableTamperProtection", 0)?;

    println!("Windows Defender settings have been enabled.");
    Ok(())
}

// Add a fallback for non-Windows platforms
#[cfg(not(target_os = "windows"))]
pub fn check_windows_defender() -> Result<bool, Box<dyn Error>> {
    println!("This function is only supported on Windows.");
    Ok(false)
}

#[cfg(not(target_os = "windows"))]
pub fn enable_sec_settings() -> Result<(), Box<dyn Error>> {
    println!("This function is only supported on Windows.");
    Ok(())
}
