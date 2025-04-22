mod cli;

pub use shared::{crate_authors, crate_description, crate_name, crate_version};
use windows::Win32::System::Power::SetSuspendState;

fn main() -> windows::core::Result<()> {
    let cli = cli::Cli::new();
    let timeout = std::time::Duration::from_secs(cli.timeout.unwrap_or(5)); // Sleep timeout from CLI argument

    std::thread::sleep(timeout); // Sleep for 5 seconds

    let result = unsafe { SetSuspendState(cli.hibernate, cli.force, cli.wake_events) };
    if result {
        println!("System is entering sleep.");
    } else {
        let error = windows::core::Error::from_win32();
        eprintln!("Failed to initiate sleep: {}", error);
        std::process::exit(1);
    }

    // if result.0 == 0 {
    //     let error = windows::core::Error::from_win32();
    //     eprintln!("Failed to initiate sleep: {}", error);
    //     std::process::exit(1);
    // }
    println!("System is entering sleep.");
    Ok(())
}
