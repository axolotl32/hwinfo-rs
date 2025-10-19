use crate::bindings;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::fmt;
use std::os::raw::c_char;
use std::str::Utf8Error;

/// A type alias for `Result<T, HwinfoError>`.
pub type Result<T> = std::result::Result<T, HwinfoError>;

/// Represents all possible errors that can occur in this library.
#[derive(Debug)]
pub enum HwinfoError {
    DataUnavailable(String), // c lib returned a nptr, it could not retrieve the data.
    InvalidString(Utf8Error), // library returned a string that is not valid utf-8 (don't ask)
}

impl fmt::Display for HwinfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HwinfoError::DataUnavailable(msg) => {
                write!(f, "Failed to retrieve hardware information: {}", msg)
            }
            HwinfoError::InvalidString(err) => {
                write!(f, "String from C library is not valid UTF-8: {}", err)
            }
        }
    }
}

impl std::error::Error for HwinfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HwinfoError::InvalidString(err) => Some(err),
            _ => None,
        }
    }
}

/// Safely converts a C string (`*mut c_char`) to a Rust `Result<String>`.
unsafe fn c_char_to_string(s: *mut c_char) -> Result<String> {
    if s.is_null() {
        Ok(String::new())
    } else {
        unsafe {
            CStr::from_ptr(s)
                .to_str()
                .map(String::from)
                .map_err(HwinfoError::InvalidString)
        }
    }
}

/// Safely converts a `C_StringArray` to a `Result<Vec<String>>`.
/// Does not free the array, as it's part of a larger struct
/// that will be freed all at once.
unsafe fn c_string_array_to_vec(arr: &bindings::C_StringArray) -> Result<Vec<String>> {
    if arr.strings.is_null() || arr.count <= 0 {
        return Ok(Vec::new());
    }
    unsafe {
        let slice = std::slice::from_raw_parts(arr.strings, arr.count as usize);
        slice.iter().map(|&s| c_char_to_string(s)).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Cpu {
    pub id: i32,
    pub vendor: String,
    pub model_name: String,
    pub num_physical_cores: i32,
    pub num_logical_cores: i32,
    pub max_clock_speed_mhz: i64,
    pub regular_clock_speed_mhz: i64,
    pub l1_cache_size_bytes: i64,
    pub l2_cache_size_bytes: i64,
    pub l3_cache_size_bytes: i64,
    pub flags: Vec<String>,
}

impl TryFrom<&bindings::C_CPU> for Cpu {
    type Error = HwinfoError;
    fn try_from(c_cpu: &bindings::C_CPU) -> Result<Self> {
        unsafe {
            Ok(Cpu {
                id: c_cpu.id,
                vendor: c_char_to_string(c_cpu.vendor)?,
                model_name: c_char_to_string(c_cpu.modelName)?,
                num_physical_cores: c_cpu.numPhysicalCores,
                num_logical_cores: c_cpu.numLogicalCores,
                max_clock_speed_mhz: c_cpu.maxClockSpeed_MHz,
                regular_clock_speed_mhz: c_cpu.regularClockSpeed_MHz,
                l1_cache_size_bytes: c_cpu.L1CacheSize_Bytes,
                l2_cache_size_bytes: c_cpu.L2CacheSize_Bytes,
                l3_cache_size_bytes: c_cpu.L3CacheSize_Bytes,
                flags: c_string_array_to_vec(&c_cpu.flags)?,
            })
        }
    }
}

pub fn cpus() -> Result<Vec<Cpu>> {
    unsafe {
        let count = bindings::get_cpu_count();
        if count <= 0 {
            return Ok(Vec::new());
        }
        let cpus_ptr = bindings::get_all_cpus();
        if cpus_ptr.is_null() {
            return Err(HwinfoError::DataUnavailable("get_all_cpus".into()));
        }

        let result = std::slice::from_raw_parts(cpus_ptr, count as usize)
            .iter()
            .map(Cpu::try_from)
            .collect();

        bindings::free_cpu_info(cpus_ptr, count);
        result
    }
}

pub fn cpu_utilization(cpu_id: i32) -> f64 {
    unsafe { bindings::get_cpu_utilization(cpu_id) }
}

pub fn cpu_thread_utilizations(cpu_id: i32) -> Result<Vec<f64>> {
    unsafe {
        let arr_ptr = bindings::get_cpu_thread_utilizations(cpu_id);
        if arr_ptr.is_null() {
            return Err(HwinfoError::DataUnavailable(format!(
                "get_cpu_thread_utilizations for cpu_id {}",
                cpu_id
            )));
        }
        let arr = &*arr_ptr;
        let result = if arr.values.is_null() || arr.count <= 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(arr.values, arr.count as usize).to_vec()
        };
        bindings::free_double_array(arr_ptr);
        Ok(result)
    }
}

pub fn cpu_thread_speeds_mhz(cpu_id: i32) -> Result<Vec<i64>> {
    unsafe {
        let arr_ptr = bindings::get_cpu_thread_speeds_mhz(cpu_id);
        if arr_ptr.is_null() {
            return Err(HwinfoError::DataUnavailable(format!(
                "get_cpu_thread_speeds_mhz for cpu_id {}",
                cpu_id
            )));
        }
        let arr = &*arr_ptr;
        let result = if arr.values.is_null() || arr.count <= 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(arr.values, arr.count as usize).to_vec()
        };
        bindings::free_int64_array(arr_ptr);
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct Os {
    pub name: String,
    pub version: String,
    pub kernel: String,
    pub is_32_bit: bool,
    pub is_64_bit: bool,
    pub is_little_endian: bool,
}

impl TryFrom<&bindings::C_OS> for Os {
    type Error = HwinfoError;
    fn try_from(c_os: &bindings::C_OS) -> Result<Self> {
        unsafe {
            Ok(Os {
                name: c_char_to_string(c_os.name)?,
                version: c_char_to_string(c_os.version)?,
                kernel: c_char_to_string(c_os.kernel)?,
                is_32_bit: c_os.is32bit,
                is_64_bit: c_os.is64bit,
                is_little_endian: c_os.isLittleEndian,
            })
        }
    }
}

pub fn os_info() -> Result<Os> {
    unsafe {
        let os_ptr = bindings::get_os_info();
        if os_ptr.is_null() {
            return Err(HwinfoError::DataUnavailable("get_os_info".into()));
        }
        let result = Os::try_from(&*os_ptr);
        bindings::free_os_info(os_ptr);
        result
    }
}

#[derive(Debug, Clone)]
pub struct Gpu {
    pub id: i32,
    pub vendor: String,
    pub name: String,
    pub driver_version: String,
    pub memory_bytes: i64,
    pub frequency_mhz: i64,
    pub num_cores: i32,
    pub vendor_id: String,
    pub device_id: String,
}

impl TryFrom<&bindings::C_GPU> for Gpu {
    type Error = HwinfoError;
    fn try_from(c_gpu: &bindings::C_GPU) -> Result<Self> {
        unsafe {
            Ok(Gpu {
                id: c_gpu.id,
                vendor: c_char_to_string(c_gpu.vendor)?,
                name: c_char_to_string(c_gpu.name)?,
                driver_version: c_char_to_string(c_gpu.driverVersion)?,
                memory_bytes: c_gpu.memory_Bytes,
                frequency_mhz: c_gpu.frequency_MHz,
                num_cores: c_gpu.num_cores,
                vendor_id: c_char_to_string(c_gpu.vendor_id)?,
                device_id: c_char_to_string(c_gpu.device_id)?,
            })
        }
    }
}

pub fn gpus() -> Result<Vec<Gpu>> {
    unsafe {
        let count = bindings::get_gpu_count();
        if count <= 0 {
            return Ok(Vec::new());
        }
        let gpus_ptr = bindings::get_all_gpus();
        if gpus_ptr.is_null() {
            return Err(HwinfoError::DataUnavailable("get_all_gpus".into()));
        }
        let result = std::slice::from_raw_parts(gpus_ptr, count as usize)
            .iter()
            .map(Gpu::try_from)
            .collect();
        bindings::free_gpu_info(gpus_ptr, count);
        result
    }
}

#[derive(Debug, Clone)]
pub struct RamModule {
    pub id: i32,
    pub vendor: String,
    pub name: String,
    pub model: String,
    pub serial_number: String,
    pub total_bytes: i64,
    pub frequency_hz: i64,
}

impl TryFrom<&bindings::C_RAM_Module> for RamModule {
    type Error = HwinfoError;
    fn try_from(c_mod: &bindings::C_RAM_Module) -> Result<Self> {
        unsafe {
            Ok(RamModule {
                id: c_mod.id,
                vendor: c_char_to_string(c_mod.vendor)?,
                name: c_char_to_string(c_mod.name)?,
                model: c_char_to_string(c_mod.model)?,
                serial_number: c_char_to_string(c_mod.serial_number)?,
                total_bytes: c_mod.total_Bytes,
                frequency_hz: c_mod.frequency_Hz,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_bytes: i64,
    pub free_bytes: i64,
    pub available_bytes: i64,
    pub modules: Vec<RamModule>,
}

impl TryFrom<&bindings::C_MemoryInfo> for MemoryInfo {
    type Error = HwinfoError;
    fn try_from(c_mem: &bindings::C_MemoryInfo) -> Result<Self> {
        let modules = if c_mem.modules.is_null() || c_mem.module_count <= 0 {
            Vec::new()
        } else {
            unsafe {
                std::slice::from_raw_parts(c_mem.modules, c_mem.module_count as usize)
                    .iter()
                    .map(RamModule::try_from)
                    .collect::<Result<Vec<RamModule>>>()?
            }
        };
        Ok(MemoryInfo {
            total_bytes: c_mem.total_Bytes,
            free_bytes: c_mem.free_Bytes,
            available_bytes: c_mem.available_Bytes,
            modules,
        })
    }
}

pub fn memory_info() -> Result<MemoryInfo> {
    unsafe {
        let mem_ptr = bindings::get_memory_info();
        if mem_ptr.is_null() {
            return Err(HwinfoError::DataUnavailable("get_memory_info".into()));
        }
        let result = MemoryInfo::try_from(&*mem_ptr);
        bindings::free_memory_info(mem_ptr);
        result
    }
}

#[derive(Debug, Clone)]
pub struct MainBoard {
    pub vendor: String,
    pub name: String,
    pub version: String,
    pub serial_number: String,
}

impl TryFrom<&bindings::C_MainBoard> for MainBoard {
    type Error = HwinfoError;
    fn try_from(c_mb: &bindings::C_MainBoard) -> Result<Self> {
        unsafe {
            Ok(MainBoard {
                vendor: c_char_to_string(c_mb.vendor)?,
                name: c_char_to_string(c_mb.name)?,
                version: c_char_to_string(c_mb.version)?,
                serial_number: c_char_to_string(c_mb.serialNumber)?,
            })
        }
    }
}

pub fn mainboard_info() -> Result<MainBoard> {
    unsafe {
        let mb_ptr = bindings::get_mainboard_info();
        if mb_ptr.is_null() {
            return Err(HwinfoError::DataUnavailable("get_mainboard_info".into()));
        }
        let result = MainBoard::try_from(&*mb_ptr);
        bindings::free_mainboard_info(mb_ptr);
        result
    }
}

#[derive(Debug, Clone)]
pub struct Disk {
    pub id: i32,
    pub vendor: String,
    pub model: String,
    pub serial_number: String,
    pub size_bytes: i64,
    pub free_size_bytes: i64,
    pub volumes: Vec<String>,
}

impl TryFrom<&bindings::C_Disk> for Disk {
    type Error = HwinfoError;
    fn try_from(c_disk: &bindings::C_Disk) -> Result<Self> {
        unsafe {
            Ok(Disk {
                id: c_disk.id,
                vendor: c_char_to_string(c_disk.vendor)?,
                model: c_char_to_string(c_disk.model)?,
                serial_number: c_char_to_string(c_disk.serialNumber)?,
                size_bytes: c_disk.size_Bytes,
                free_size_bytes: c_disk.free_size_Bytes,
                volumes: c_string_array_to_vec(&c_disk.volumes)?,
            })
        }
    }
}

pub fn disks() -> Result<Vec<Disk>> {
    unsafe {
        let count = bindings::get_disk_count();
        if count <= 0 {
            return Ok(Vec::new());
        }
        let disks_ptr = bindings::get_all_disks();
        if disks_ptr.is_null() {
            return Err(HwinfoError::DataUnavailable("get_all_disks".into()));
        }
        let result = std::slice::from_raw_parts(disks_ptr, count as usize)
            .iter()
            .map(Disk::try_from)
            .collect();
        bindings::free_disk_info(disks_ptr, count);
        result
    }
}

#[derive(Debug, Clone)]
pub struct Battery {
    pub id: i32,
    pub vendor: String,
    pub model: String,
    pub serial_number: String,
    pub technology: String,
    pub energy_full_mwh: u32,
    pub energy_now_mwh: u32,
    pub is_charging: bool,
}

impl TryFrom<&bindings::C_Battery> for Battery {
    type Error = HwinfoError;
    fn try_from(c_bat: &bindings::C_Battery) -> Result<Self> {
        unsafe {
            Ok(Battery {
                id: c_bat.id,
                vendor: c_char_to_string(c_bat.vendor)?,
                model: c_char_to_string(c_bat.model)?,
                serial_number: c_char_to_string(c_bat.serialNumber)?,
                technology: c_char_to_string(c_bat.technology)?,
                energy_full_mwh: c_bat.energyFull,
                energy_now_mwh: c_bat.energyNow,
                is_charging: c_bat.charging,
            })
        }
    }
}

pub fn batteries() -> Result<Vec<Battery>> {
    unsafe {
        let count = bindings::get_battery_count();
        if count <= 0 {
            return Ok(Vec::new());
        }
        let batteries_ptr = bindings::get_all_batteries();
        if batteries_ptr.is_null() {
            return Err(HwinfoError::DataUnavailable("get_all_batteries".into()));
        }
        let result = std::slice::from_raw_parts(batteries_ptr, count as usize)
            .iter()
            .map(Battery::try_from)
            .collect();
        bindings::free_battery_info(batteries_ptr, count);
        result
    }
}

#[derive(Debug, Clone)]
pub struct Network {
    pub interface_index: String,
    pub description: String,
    pub mac_address: String,
    pub ipv4_address: String,
    pub ipv6_address: String,
}

impl TryFrom<&bindings::C_Network> for Network {
    type Error = HwinfoError;
    fn try_from(c_net: &bindings::C_Network) -> Result<Self> {
        unsafe {
            Ok(Network {
                interface_index: c_char_to_string(c_net.interfaceIndex)?,
                description: c_char_to_string(c_net.description)?,
                mac_address: c_char_to_string(c_net.mac)?,
                ipv4_address: c_char_to_string(c_net.ip4)?,
                ipv6_address: c_char_to_string(c_net.ip6)?,
            })
        }
    }
}

pub fn networks() -> Result<Vec<Network>> {
    unsafe {
        let count = bindings::get_network_count();
        if count <= 0 {
            return Ok(Vec::new());
        }
        let networks_ptr = bindings::get_all_networks();
        if networks_ptr.is_null() {
            return Err(HwinfoError::DataUnavailable("get_all_networks".into()));
        }
        let result = std::slice::from_raw_parts(networks_ptr, count as usize)
            .iter()
            .map(Network::try_from)
            .collect();
        bindings::free_network_info(networks_ptr, count);
        result
    }
}
