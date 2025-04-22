# CreateWakeTask.ps1
# Creates a Task Scheduler task to wake the system at a specified time and run a Python script

# Function to validate time format (HH:mm:ss or HH:mm)
function ValidateTimeFormat {
    param (
        [string]$time
    )
    if ($time -match "^([0-1]?[0-9]|2[0-3]):[0-5][0-9](:[0-5][0-9])?$") {
        # If seconds are not provided, append :00
        if ($time -notmatch ":[0-5][0-9]$") {
            return "$time:00"
        }
        return $time
    }
    return $null
}

# Function to prompt and validate time input
function Get-ValidTime {
    while ($true) {
        $inputTime = Read-Host "Enter the time to wake and run the script (HH:mm or HH:mm:ss, e.g., 06:00 or 06:00:00)"
        $validatedTime = ValidateTimeFormat -time $inputTime
        if ($validatedTime) {
            return $validatedTime
        }
        Write-Host "Invalid time format. Please use HH:mm or HH:mm:ss (e.g., 06:00 or 06:00:00)."
    }
}

# Function to find python.exe in PATH
function Find-PythonInPath {
    $pythonPaths = @()
    $envPaths = $env:PATH -split ';'
    foreach ($path in $envPaths) {
        $pythonExe = Join-Path -Path $path -ChildPath "python.exe"
        if (Test-Path -Path $pythonExe -PathType Leaf) {
            $pythonPaths += $pythonExe
        }
    }
    return $pythonPaths | Select-Object -Unique
}

# Function to prompt and validate file path
function Get-ValidFilePath {
    param (
        [string]$prompt,
        [string]$fileType
    )
    while ($true) {
        $path = Read-Host $prompt
        $path = $path.Trim('"').Trim()
        if (-not $path -and $fileType -eq "python") {
            # Try finding python.exe in PATH
            $pythonPaths = Find-PythonInPath
            if ($pythonPaths.Count -gt 0) {
                Write-Host "Found the following Python executables in PATH:"
                for ($i = 0; $i -lt $pythonPaths.Count; $i++) {
                    Write-Host "$($i + 1): $($pythonPaths[$i])"
                }
                $selection = Read-Host "Select a Python executable by number (or press Enter to skip and provide a path)"
                if ($selection -match '^\d+$' -and $selection -ge 1 -and $selection -le $pythonPaths.Count) {
                    return $pythonPaths[$selection - 1]
                }
                Write-Host "No selection made. Please provide a path to python.exe."
                continue
            } else {
                Write-Host "No python.exe found in PATH. Please provide a valid path."
                continue
            }
        }
        if (-not $path) {
            Write-Host "Path cannot be empty."
            continue
        }
        if ($fileType -eq "python" -and -not $path.EndsWith("python.exe")) {
            Write-Host "Path must point to python.exe."
            continue
        }
        if ($fileType -eq "script" -and -not $path.EndsWith(".py")) {
            Write-Host "Path must point to a .py file."
            continue
        }
        if (Test-Path -Path $path -PathType Leaf) {
            return $path
        }
        Write-Host "File does not exist. Please provide a valid path."
    }
}

# Function to prompt and validate non-empty string
function Get-NonEmptyString {
    param (
        [string]$prompt
    )
    while ($true) {
        $inputValue = Read-Host $prompt
        $inputValue = $inputValue.Trim()
        if ($inputValue) {
            return $inputValue
        }
        Write-Host "Input cannot be empty."
    }
}

# Prompt user for inputs
Write-Host "This script will ask for administrative privileges to create a scheduled task." -ForegroundColor Yellow
Write-Host "This is specifically for running the created task as SYSTEM." -ForegroundColor Yellow

$taskName = Get-NonEmptyString -prompt "Enter the task name (e.g., WakeAndRunPython)"
$taskDescription = Get-NonEmptyString -prompt "Enter the task description"
$triggerTime = Get-ValidTime
$pythonPath = Get-ValidFilePath -prompt "Enter the full path to python.exe (or press Enter to search PATH)" -fileType "python"
$scriptPath = Get-ValidFilePath -prompt "Enter the full path to the Python script (.py)" -fileType "script"

# Define the action (run Python script)
$action = New-ScheduledTaskAction -Execute $pythonPath -Argument "`"$scriptPath`""

# Define the trigger (daily at specified time with wake)
$trigger = New-ScheduledTaskTrigger -Daily -At $triggerTime
$trigger.Settings = New-ScheduledTaskSettingsSet
$trigger.Settings.WakeToRun = $true

# Define task settings
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -WakeToRun -ExecutionTimeLimit (New-TimeSpan -Hours 1)
$settings.RestartCount = 3
$settings.RestartInterval = (New-TimeSpan -Minutes 1)

# Register the task
try {
    Register-ScheduledTask -TaskName $taskName `
                          -Description $taskDescription `
                          -Action $action `
                          -Trigger $trigger `
                          -Settings $settings `
                          -User "SYSTEM" `
                          -Force `
                          -ErrorAction Stop
    Write-Output "Task '$taskName' created successfully."
}
catch {
    Write-Error "Failed to create task: $_"
    exit 1
}
