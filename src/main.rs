use std::process::{Command, exit}; 
use regex::Regex;
use clap::{App, Arg};

/// Function to check memory usage of the ELF file using `avr-size`.
fn check_elf_memory(file: &str) {
    // Call `avr-size` with the ELF file
    let output = Command::new("avr-size")
        .arg("-A")
        .arg(file)
        .output()
        .expect("Failed to execute avr-size");

    if !output.status.success() {
        eprintln!("Error executing avr-size on file: {}", file);
        exit(1);
    }

    // Convert the output to a string
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("avr-size output:\n{}", output_str);

    // Use regex to extract memory usage details
    let data_regex = Regex::new(r"\.data\s+(\d+)").unwrap();
    let text_regex = Regex::new(r"\.text\s+(\d+)").unwrap();
    let bss_regex = Regex::new(r"\.bss\s+(\d+)").unwrap();

    let data_size = data_regex.captures(&output_str).and_then(|caps| caps[1].parse::<u64>().ok()).unwrap_or(0);
    let text_size = text_regex.captures(&output_str).and_then(|caps| caps[1].parse::<u64>().ok()).unwrap_or(0);
    let bss_size = bss_regex.captures(&output_str).and_then(|caps| caps[1].parse::<u64>().ok()).unwrap_or(0);

    // Calculate memory usage
    let rom_usage = text_size + data_size;
    let ram_usage = data_size + bss_size;

    // Display results
    println!("\nMemory Usage:");
    println!("--------------");
    println!("ROM (Flash): {} bytes", rom_usage);
    println!("RAM: {} bytes", ram_usage);
}

/// Function to display information about the project and authors
fn show_about_us() {
    println!("MemoryMap Tool\n");
    println!("This tool checks the memory usage of .elf files for AVR controllers.");
    println!("\nAuthor: Sreeraj V. Rajesh");
    println!("License: MIT License");
    println!("GitHub: https://github.com/your-repository-link");
}

fn main() {
    // Set up CLI arguments using clap
    let matches = App::new("MemoryMap")
        .version("1.0")
        .author("Sreeraj V. Rajesh")
        .about("Check memory usage of .elf files for AVR controllers")
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .value_name("FILE")
                .help("Check the memory usage of the specified ELF file")
                .takes_value(true),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Display the version of the tool"),
        )
        .arg(
            Arg::new("aboutus")
                .long("aboutus")
                .help("Display information about the project and its authors"),
        )
        .get_matches();

    // Handle `--check` option
    if let Some(elf_file) = matches.value_of("check") {
        check_elf_memory(elf_file);
    }
    // Handle `--version` option
    else if matches.is_present("version") {
        println!("MemoryMap version 1.0");
    }
    // Handle `--aboutus` option
    else if matches.is_present("aboutus") {
        show_about_us();
    } else {
        // Default to showing help if no arguments are passed
        eprintln!("Please provide an ELF file with -c or --check.");
        exit(1);
    }
}
