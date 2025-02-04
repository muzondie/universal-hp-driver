# Universal HP Driver  

A tool for Windows 10/11 that simplifies driver management for HP devices. It scans your system, identifies missing or outdated drivers, and installs them automatically. Built with Rust and featuring a simple GUI, it replaces the need to manually search for individual drivers.  

## Download  
1. Go to the [Releases](https://github.com/muzondie/universal-hp-driver/releases) tab on GitHub.  
2. Download the latest `.zip` file.  
3. Unzip the file and run `UniversalHpDriver.exe`.  

## Usage  
1. **Run the app** after unzipping.  
2. **Wait for detection**: The tool scans connected HP devices.  
3. **Review and install**: Confirm the drivers you want to install.  
4. **Restart if prompted** to complete setup.  

## Features  
- **Automatic detection** of HP printers, scanners, and others.  
- **Offline installation** for common drivers (no internet required).  
- **Silent install mode** (use `--silent` in command line for background setup).  
- **Driver updates**: Checks for newer versions and upgrades existing drivers.  
- **Logging**: Generates a report after installation for troubleshooting.  
- **Lightweight**: Minimal system resource usage.  
- **Supports multiple HP device categories**:  
  - Printers (inkjet, laser, office)  
  - Scanners  
  - All-in-one devices  
  - Older hardware
  - And more
- **Uninstall tool**: Removes outdated or conflicting drivers.  

## Build from Source  
For developers:  
1. Install [Rust](https://www.rust-lang.org/tools/install).  
2. Clone the repository:  
   ```bash  
   git clone https://github.com/muzondie/universal-hp-driver.git  
   ```  
3. Build the project:  
   ```bash  
   cd universal-hp-driver  
   cargo build --release  
   ```  
4. Find the executable in `target/release/`.  

## Contributing  
Contributions are currently closed due to maintenance constraints.

## License  
MIT License. See [LICENSE](LICENSE) file.