use hwinfo_rs::hwinfo;
use std::process;

fn bytes_to_gb(bytes: i64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

fn run() -> hwinfo::Result<()> {
    println!("--- Hardware Information Report ---");

    // 1. Operating System
    println!("\n[ Operating System ]");
    let os = hwinfo::os_info()?;
    println!("  Name: {}", os.name);
    println!("  Version: {}", os.version);
    println!("  Kernel: {}", os.kernel);
    println!("  Is 64-bit: {}", os.is_64_bit);

    // 2. Mainboard
    println!("\n[ Mainboard ]");
    let mainboard = hwinfo::mainboard_info()?;
    println!("  Vendor: {}", mainboard.vendor);
    println!("  Name: {}", mainboard.name);
    println!("  Version: {}", mainboard.version);
    println!("  Serial Number: {}", mainboard.serial_number);

    // 3. Memory
    println!("\n[ Memory (RAM) ]");
    let mem = hwinfo::memory_info()?;
    println!("  Total: {:.2} GB", bytes_to_gb(mem.total_bytes));
    println!("  Available: {:.2} GB", bytes_to_gb(mem.available_bytes));
    println!("  Free: {:.2} GB", bytes_to_gb(mem.free_bytes));
    if mem.modules.is_empty() {
        println!("  No individual RAM module data available.");
    } else {
        println!("  RAM Modules ({} found):", mem.modules.len());
        for module in mem.modules {
            println!("    - ID {}: {} {} ({} MHz, {:.2} GB)",
                module.id,
                module.vendor,
                module.model,
                module.frequency_hz / 1_000_000,
                bytes_to_gb(module.total_bytes)
            );
        }
    }

    // 4. CPU
    println!("\n[ CPUs ]");
    let cpus = hwinfo::cpus()?;
    println!("  Sockets found: {}", cpus.len());
    for cpu in cpus {
        println!("  - CPU ID {}: {}", cpu.id, cpu.model_name);
        println!("    Vendor: {}", cpu.vendor);
        println!("    Cores: {} physical, {} logical", cpu.num_physical_cores, cpu.num_logical_cores);
        println!("    Clock Speed: {} MHz (Max: {} MHz)", cpu.regular_clock_speed_mhz, cpu.max_clock_speed_mhz);
        println!("    L3 Cache: {} KB", cpu.l3_cache_size_bytes / 1024);
        
        // Test the dynamic, per-CPU functions
        let utilization = hwinfo::cpu_utilization(cpu.id);
        println!("    Current Utilization: {:.2}%", utilization * 100.0);
        
        let thread_speeds = hwinfo::cpu_thread_speeds_mhz(cpu.id)?;
        println!("    Thread Speeds (MHz): {:?}", thread_speeds);

        let thread_utils = hwinfo::cpu_thread_utilizations(cpu.id)?;
        println!("    Thread Utilizations: {:?}", thread_utils);

        if !cpu.flags.is_empty() {
            println!("    Flags: {}", cpu.flags.join(", "));
        }
    }

    // 5. GPUs
    println!("\n[ GPUs ]");
    let gpus = hwinfo::gpus()?;
    if gpus.is_empty() {
        println!("  No GPUs found.");
    } else {
        println!("  GPUs found: {}", gpus.len());
        for gpu in gpus {
            println!("  - GPU ID {}: {}", gpu.id, gpu.name);
            println!("    Vendor: {}", gpu.vendor);
            println!("    VRAM: {:.2} GB", bytes_to_gb(gpu.memory_bytes));
            println!("    Frequency: {} MHz", gpu.frequency_mhz);
            println!("    Cores: {}", gpu.num_cores);
            println!("    Driver Version: {}", gpu.driver_version);
        }
    }

    // 6. Disks
    println!("\n[ Disks ]");
    let disks = hwinfo::disks()?;
    if disks.is_empty() {
        println!("  No disks found.");
    } else {
        println!("  Disks found: {}", disks.len());
        for disk in disks {
            println!("  - Disk ID {}: {} ({})", disk.id, disk.model, disk.vendor);
            println!("    Serial: {}", disk.serial_number);
            println!("    Size: {:.2} GB", bytes_to_gb(disk.size_bytes));
            println!("    Volumes: {}", disk.volumes.join(", "));
        }
    }

    // 7. Batteries
    println!("\n[ Batteries ]");
    let batteries = hwinfo::batteries()?;
    if batteries.is_empty() {
        println!("  No batteries found.");
    } else {
        println!("  Batteries found: {}", batteries.len());
        for battery in batteries {
            println!("  - Battery ID {}: {} ({})", battery.id, battery.model, battery.vendor);
            let charge_percent = (battery.energy_now_mwh as f32 / battery.energy_full_mwh as f32) * 100.0;
            println!("    Charge: {:.2}% ({}/{} mWh)", charge_percent, battery.energy_now_mwh, battery.energy_full_mwh);
            println!("    Is Charging: {}", battery.is_charging);
            println!("    Technology: {}", battery.technology);
        }
    }

    // 8. Network Interfaces
    println!("\n[ Network Interfaces ]");
    let networks = hwinfo::networks()?;
    if networks.is_empty() {
        println!("  No network interfaces found.");
    } else {
        println!("  Interfaces found: {}", networks.len());
        for net in networks {
            println!("  - {}", net.description);
            println!("    MAC Address: {}", net.mac_address);
            println!("    IPv4: {}", net.ipv4_address);
            println!("    IPv6: {}", net.ipv6_address);
        }
    }

    println!("\n--- End of Report ---");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("[ERROR] Failed to generate hardware report: {}", e);
        process::exit(1);
    }
}