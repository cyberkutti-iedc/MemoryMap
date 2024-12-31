Here’s the updated **README** with the new name, **MemoryMap**, reflecting the changes you requested:

---

# MemoryMap

**MemoryMap** is a command-line utility to analyze the memory usage of `.elf` files for AVR microcontrollers. It uses `avr-size` to extract details about memory sections such as ROM and RAM usage, making it easier to optimize embedded applications.

## Features

- Supports multiple AVR microcontrollers like ATmega328P, ATmega2560, and others.
- Displays detailed memory usage for sections such as `.text`, `.data`, and `.bss`.
- Provides version information and an about section.
- Easy-to-use CLI with options for checking `.elf` files and viewing help.

## Installation

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).
- **avr-size**: Install AVR-GCC toolchain, which includes `avr-size`. For Windows, you can use packages like [WinAVR](http://winavr.sourceforge.net/) or [Mingw-w64](http://mingw-w64.org/).

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/your-repository-link/memorymap

# Navigate to the project directory
cd memorymap

# Build the project
cargo build --release
```

## Usage

### Basic Commands

1. **Check ELF File Memory Usage:**

   ```bash
   memorymap -c <path-to-your-elf-file>
   ```

   Example:

   ```bash
   memorymap -c main.elf
   ```

2. **Display Version:**

   ```bash
   memorymap --version
   ```

3. **About the Tool:**

   ```bash
   memorymap --aboutus
   ```

4. **Help:**

   ```bash
   memorymap --help
   ```

## Example Output

```bash
$ memorymap -c main.elf
Memory Usage:
   text    data     bss     dec     hex filename
   1536       0       9    1545     609 main.elf
```

## Supported Targets

This tool supports building for the following platforms:

- Windows
- Linux
- macOS

### Cross-Compilation

If you are building for platforms other than your host, use Rust's cross-compilation capabilities. For example:

```bash
cargo build --release --target=x86_64-unknown-linux-gnu
```

Ensure the necessary toolchains and linkers are installed on your system.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author

- **Sreeraj V. Rajesh**
- GitHub: [cyberkutti-iedc](https://github.com/cyberkutti-iedc)
