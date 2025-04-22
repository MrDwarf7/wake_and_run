mod guid;

use guid::Guid;
use windows::Win32::System::Power::{
    GetSystemPowerStatus,
    // POWER_PLATFORM_ROLE,
    // PowerEnumerate is not available in Win32::System::Power
    PowerGetActiveScheme,
    PowerReadFriendlyName,
    SYSTEM_POWER_STATUS,
};

fn main() -> windows::core::Result<()> {
    // Check system power status (AC/DC, battery level)
    let mut power_status = SYSTEM_POWER_STATUS::default();
    unsafe {
        GetSystemPowerStatus(&mut power_status)?;
    }
    println!(
        "AC Line Status: {}",
        if power_status.ACLineStatus == 1 {
            "Online"
        } else {
            "Offline"
        }
    );
    println!("Battery Life: {}%", power_status.BatteryLifePercent);

    // Check active power scheme
    let power_scheme_guid = Guid::default();
    let power_scheme_guid_size = std::mem::size_of::<Guid>() as u32;
    println!("Power Scheme Guid: {:?}", power_scheme_guid);
    println!("Power Scheme Guid Size: {power_scheme_guid_size}");

    let mut guid_ptr = std::ptr::null_mut();

    unsafe {
        let result = PowerGetActiveScheme(None, &mut guid_ptr).0;
        if result != 0 {
            return Err(windows::core::Error::from_win32());
        }

        if guid_ptr.is_null() {
            return Err(windows::core::Error::from_win32());
        }

        let guid = Guid::from(guid_ptr);

        // Convert the Windows Guid to our custom Guid (simplified example)
        // In a real implementation, you'd need to properly copy the Guid data
        println!("Active Power Scheme Guid pointer: {:?}", guid_ptr);
        println!("Active Power Scheme Guid: {:?}", guid);
    }

    // Note: PowerEnumerate is not available in Win32::System::Power
    // This section has been removed since the function is not available

    // Optionally, read friendly name of power scheme
    let mut name_buffer: Vec<u16> = vec![0; 128];
    let mut name_size = name_buffer.len() as u32;
    unsafe {
        let result = PowerReadFriendlyName(
            None,
            Some(guid_ptr),
            None,
            None,
            Some(name_buffer.as_mut_ptr() as *mut _),
            &mut name_size,
        )
        .0;
        if result != 0 {
            return Err(windows::core::Error::from_win32());
        }
    }

    // Convert to string ensuring we only use the valid portion
    let name = if name_size > 0 {
        let valid_chars = &name_buffer[..(name_size as usize / 2)];
        String::from_utf16_lossy(valid_chars)
    } else {
        String::new()
    };
    println!("Power Scheme Name: {}", name);

    Ok(())
}
