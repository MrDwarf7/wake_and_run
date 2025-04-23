# wake_and_run

A lightweight Rust CLI tool to trigger sleep or hibernation on Windows systems using the Windows API (SetSuspendState). Designed for automation and power management, it supports configurable timeouts and options to control sleep behavior.

## Features

Initiate system sleep or hibernation with customizable parameters.
Optional timeout to delay sleep execution.
Control whether to force sleep, allow wake events, or enable hibernation.
Safe and efficient, built with Rust and the windows crate.

## Installation

### Prerequisites

Rust and Cargo (install from rustup.rs).
Windows 10 or 11 (required for SetSuspendState API).
Administrative privileges may be required for some operations.

### Build from Source

Clone the repository:

```ps1
git clone https://github.com/your-username/wake_and_run.git
cd wake_and_run
```

### Build and install:

```ps1
cargo build --release
```

#### (Optional) Copy the binary to a directory in your PATH:

```ps1
$myFavTool = "sleep"
# or
$myFavTool = "checker"
cp target/release/$myFavTool.exe C:\Windows\System32\
```

## Usage

Run the tool from the command line with optional arguments to control sleep behavior.

### Basic Command

Trigger immediate sleep:

```ps1
sleep
```

## CLI Options

| Flag         | Long Form           | Description                                    | Default                      |
| ------------ | ------------------- | ---------------------------------------------- | ---------------------------- |
| -t <seconds> | --timeout <seconds> | Delay sleep by the specified number of seconds | None (immediate)             |
| -n           | --hibernate         | Enable hibernation instead of sleep            | false (sleep)                |
| -f           | --force             | Force the operation, ignoring system state     | false (respect system state) |
| -w           | --wake-events       | Allow wake events (e.g., mouse, keyboard)      | false (disable wake events)  |

## Examples

- Sleep after a 10-second delay:

```ps1
sleep --timeout 10
```

- Force hibernation immediately:

```ps1
sleep --hibernate --force
```

- Sleep with wake events enabled:

```ps1
sleep -w
```

- Combine options:

```ps1
sleep -t 30 -f -w
```

## Notes

- Corporate Environments: Ensure you have sufficient permissions. Some systems may restrict power state changes due to Group Policy.
- Modern Standby: On systems with Modern Standby (S0ix), the tool triggers the low-power idle state instead of traditional S3 sleep.
- Error Handling: The tool reports Windows API errors if sleep fails (e.g., due to active applications or insufficient permissions).

## Contributing

Contributions are welcome! Please submit issues or pull requests to the GitHub repository.

## License

MIT License. See LICENSE for details.
