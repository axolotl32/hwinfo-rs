#include "hwinfo/hwinfo_c.h"

#include <cstring>
#include <string>
#include <vector>

#include "hwinfo/hwinfo.h"

extern "C" {

// --- Helper Functions for Memory Management ---

// Allocates a new C-style string and copies the content of a std::string into it.
// The caller is responsible for freeing this memory with delete[].
char* copy_string(const std::string& s) {
  if (s.empty()) {
    char* c_str = new char[1];
    c_str[0] = '\0';
    return c_str;
  }
  char* c_str = new char[s.length() + 1];
  std::strcpy(c_str, s.c_str());
  return c_str;
}

// Frees a C_StringArray, including all individual strings.
void free_string_array(C_StringArray* arr) {
    if (!arr) return;
    for (int i = 0; i < arr->count; ++i) {
        delete[] arr->strings[i];
    }
    delete[] arr->strings;
    delete arr;
}

// --- Component Implementations ---

// CPU
static std::vector<hwinfo::CPU> cpus;

int get_cpu_count() {
  if (cpus.empty()) {
    cpus = hwinfo::getAllCPUs();
  }
  return static_cast<int>(cpus.size());
}

C_CPU* get_all_cpus() {
  if (cpus.empty()) {
    cpus = hwinfo::getAllCPUs();
  }
  if (cpus.empty()) {
    return nullptr;
  }
  C_CPU* c_cpus = new C_CPU[cpus.size()];
  for (size_t i = 0; i < cpus.size(); ++i) {
    c_cpus[i].id = cpus[i].id();
    c_cpus[i].vendor = copy_string(cpus[i].vendor());
    c_cpus[i].modelName = copy_string(cpus[i].modelName());
    c_cpus[i].numPhysicalCores = cpus[i].numPhysicalCores();
    c_cpus[i].numLogicalCores = cpus[i].numLogicalCores();
    c_cpus[i].maxClockSpeed_MHz = cpus[i].maxClockSpeed_MHz();
    c_cpus[i].regularClockSpeed_MHz = cpus[i].regularClockSpeed_MHz();
    c_cpus[i].L1CacheSize_Bytes = cpus[i].L1CacheSize_Bytes();
    c_cpus[i].L2CacheSize_Bytes = cpus[i].L2CacheSize_Bytes();
    c_cpus[i].L3CacheSize_Bytes = cpus[i].L3CacheSize_Bytes();
    const auto& flags = cpus[i].flags();
    c_cpus[i].flags.count = static_cast<int>(flags.size());
    c_cpus[i].flags.strings = new char*[flags.size()];
    for(size_t j = 0; j < flags.size(); ++j) {
        c_cpus[i].flags.strings[j] = copy_string(flags[j]);
    }
  }
  return c_cpus;
}

double get_cpu_utilization(int cpu_id) {
    if (cpus.empty()) { cpus = hwinfo::getAllCPUs(); }
    if (cpu_id < 0 || cpu_id >= cpus.size()) return -1.0;
    return cpus[cpu_id].currentUtilisation();
}

C_DoubleArray* get_cpu_thread_utilizations(int cpu_id) {
    if (cpus.empty()) { cpus = hwinfo::getAllCPUs(); }
    if (cpu_id < 0 || cpu_id >= cpus.size()) return nullptr;
    std::vector<double> utils = cpus[cpu_id].threadsUtilisation();
    C_DoubleArray* result = new C_DoubleArray();
    result->count = static_cast<int>(utils.size());
    result->values = new double[utils.size()];
    std::memcpy(result->values, utils.data(), utils.size() * sizeof(double));
    return result;
}

C_Int64Array* get_cpu_thread_speeds_mhz(int cpu_id) {
    if (cpus.empty()) { cpus = hwinfo::getAllCPUs(); }
    if (cpu_id < 0 || cpu_id >= cpus.size()) return nullptr;
    std::vector<int64_t> speeds = cpus[cpu_id].currentClockSpeed_MHz();
    C_Int64Array* result = new C_Int64Array();
    result->count = static_cast<int>(speeds.size());
    result->values = new int64_t[speeds.size()];
    std::memcpy(result->values, speeds.data(), speeds.size() * sizeof(int64_t));
    return result;
}

void free_cpu_info(C_CPU* c_cpus, int count) {
  if (c_cpus) {
    for (int i = 0; i < count; ++i) {
      delete[] c_cpus[i].vendor;
      delete[] c_cpus[i].modelName;
      for (int j = 0; j < c_cpus[i].flags.count; ++j) {
        delete[] c_cpus[i].flags.strings[j];
      }
      delete[] c_cpus[i].flags.strings;
    }
    delete[] c_cpus;
  }
}

void free_double_array(C_DoubleArray* arr) {
    if (arr) {
        delete[] arr->values;
        delete arr;
    }
}

void free_int64_array(C_Int64Array* arr) {
    if (arr) {
        delete[] arr->values;
        delete arr;
    }
}


// OS
C_OS* get_os_info() {
  hwinfo::OS os;
  C_OS* c_os = new C_OS();
  c_os->name = copy_string(os.name());
  c_os->version = copy_string(os.version());
  c_os->kernel = copy_string(os.kernel());
  c_os->is32bit = os.is32bit();
  c_os->is64bit = os.is64bit();
  c_os->isLittleEndian = os.isLittleEndian();
  return c_os;
}

void free_os_info(C_OS* os) {
  if (os) {
    delete[] os->name;
    delete[] os->version;
    delete[] os->kernel;
    delete os;
  }
}

// GPU
static std::vector<hwinfo::GPU> gpus;

int get_gpu_count() {
  if (gpus.empty()) {
    gpus = hwinfo::getAllGPUs();
  }
  return static_cast<int>(gpus.size());
}

C_GPU* get_all_gpus() {
  if (gpus.empty()) {
    gpus = hwinfo::getAllGPUs();
  }
  if (gpus.empty()) {
    return nullptr;
  }
  C_GPU* c_gpus = new C_GPU[gpus.size()];
  for (size_t i = 0; i < gpus.size(); ++i) {
    c_gpus[i].id = gpus[i].id();
    c_gpus[i].vendor = copy_string(gpus[i].vendor());
    c_gpus[i].name = copy_string(gpus[i].name());
    c_gpus[i].driverVersion = copy_string(gpus[i].driverVersion());
    c_gpus[i].memory_Bytes = gpus[i].memory_Bytes();
    c_gpus[i].frequency_MHz = gpus[i].frequency_MHz();
    c_gpus[i].num_cores = gpus[i].num_cores();
    c_gpus[i].vendor_id = copy_string(gpus[i].vendor_id());
    c_gpus[i].device_id = copy_string(gpus[i].device_id());
  }
  return c_gpus;
}

void free_gpu_info(C_GPU* c_gpus, int count) {
  if (c_gpus) {
    for (int i = 0; i < count; ++i) {
      delete[] c_gpus[i].vendor;
      delete[] c_gpus[i].name;
      delete[] c_gpus[i].driverVersion;
      delete[] c_gpus[i].vendor_id;
      delete[] c_gpus[i].device_id;
    }
    delete[] c_gpus;
  }
}

// Memory
C_MemoryInfo* get_memory_info() {
  hwinfo::Memory mem;
  C_MemoryInfo* c_mem = new C_MemoryInfo();
  c_mem->total_Bytes = mem.total_Bytes();
  c_mem->free_Bytes = mem.free_Bytes();
  c_mem->available_Bytes = mem.available_Bytes();
  const auto& modules = mem.modules();
  c_mem->module_count = static_cast<int>(modules.size());
  c_mem->modules = new C_RAM_Module[modules.size()];
  for (size_t i = 0; i < modules.size(); ++i) {
      c_mem->modules[i].id = modules[i].id;
      c_mem->modules[i].vendor = copy_string(modules[i].vendor);
      c_mem->modules[i].name = copy_string(modules[i].name);
      c_mem->modules[i].model = copy_string(modules[i].model);
      c_mem->modules[i].serial_number = copy_string(modules[i].serial_number);
      c_mem->modules[i].total_Bytes = modules[i].total_Bytes;
      c_mem->modules[i].frequency_Hz = modules[i].frequency_Hz;
  }
  return c_mem;
}

void free_memory_info(C_MemoryInfo* memory_info) {
    if (memory_info) {
        for (int i = 0; i < memory_info->module_count; ++i) {
            delete[] memory_info->modules[i].vendor;
            delete[] memory_info->modules[i].name;
            delete[] memory_info->modules[i].model;
            delete[] memory_info->modules[i].serial_number;
        }
        delete[] memory_info->modules;
        delete memory_info;
    }
}

// Mainboard
C_MainBoard* get_mainboard_info() {
  hwinfo::MainBoard mb;
  C_MainBoard* c_mb = new C_MainBoard();
  c_mb->vendor = copy_string(mb.vendor());
  c_mb->name = copy_string(mb.name());
  c_mb->version = copy_string(mb.version());
  c_mb->serialNumber = copy_string(mb.serialNumber());
  return c_mb;
}

void free_mainboard_info(C_MainBoard* mainboard) {
  if (mainboard) {
    delete[] mainboard->vendor;
    delete[] mainboard->name;
    delete[] mainboard->version;
    delete[] mainboard->serialNumber;
    delete mainboard;
  }
}

// Disk
static std::vector<hwinfo::Disk> disks;

int get_disk_count() {
    if (disks.empty()) {
        disks = hwinfo::getAllDisks();
    }
    return static_cast<int>(disks.size());
}

C_Disk* get_all_disks() {
    if (disks.empty()) {
        disks = hwinfo::getAllDisks();
    }
    if (disks.empty()) {
        return nullptr;
    }
    C_Disk* c_disks = new C_Disk[disks.size()];
    for (size_t i = 0; i < disks.size(); ++i) {
        c_disks[i].id = disks[i].id();
        c_disks[i].vendor = copy_string(disks[i].vendor());
        c_disks[i].model = copy_string(disks[i].model());
        c_disks[i].serialNumber = copy_string(disks[i].serialNumber());
        c_disks[i].size_Bytes = disks[i].size_Bytes();
        c_disks[i].free_size_Bytes = disks[i].free_size_Bytes();
        const auto& volumes = disks[i].volumes();
        c_disks[i].volumes.count = static_cast<int>(volumes.size());
        c_disks[i].volumes.strings = new char*[volumes.size()];
        for(size_t j = 0; j < volumes.size(); ++j) {
            c_disks[i].volumes.strings[j] = copy_string(volumes[j]);
        }
    }
    return c_disks;
}

void free_disk_info(C_Disk* c_disks, int count) {
    if (c_disks) {
        for(int i = 0; i < count; ++i) {
            delete[] c_disks[i].vendor;
            delete[] c_disks[i].model;
            delete[] c_disks[i].serialNumber;
            for (int j = 0; j < c_disks[i].volumes.count; ++j) {
                delete[] c_disks[i].volumes.strings[j];
            }
            delete[] c_disks[i].volumes.strings;
        }
        delete[] c_disks;
    }
}

// Battery
static std::vector<hwinfo::Battery> batteries;

int get_battery_count() {
    if (batteries.empty()) {
        batteries = hwinfo::getAllBatteries();
    }
    return static_cast<int>(batteries.size());
}

C_Battery* get_all_batteries() {
    if (batteries.empty()) {
        batteries = hwinfo::getAllBatteries();
    }
    if (batteries.empty()) {
        return nullptr;
    }
    C_Battery* c_batteries = new C_Battery[batteries.size()];
    for(size_t i = 0; i < batteries.size(); ++i) {
        c_batteries[i].vendor = copy_string(batteries[i].getVendor());
        c_batteries[i].model = copy_string(batteries[i].getModel());
        c_batteries[i].serialNumber = copy_string(batteries[i].getSerialNumber());
        c_batteries[i].technology = copy_string(batteries[i].getTechnology());
        c_batteries[i].energyFull = batteries[i].getEnergyFull();
        c_batteries[i].energyNow = batteries[i].energyNow();
        c_batteries[i].charging = batteries[i].charging();
    }
    return c_batteries;
}

void free_battery_info(C_Battery* c_batteries, int count) {
    if (c_batteries) {
        for (int i = 0; i < count; ++i) {
            delete[] c_batteries[i].vendor;
            delete[] c_batteries[i].model;
            delete[] c_batteries[i].serialNumber;
            delete[] c_batteries[i].technology;
        }
        delete[] c_batteries;
    }
}


// Network
static std::vector<hwinfo::Network> networks;
int get_network_count() {
    if (networks.empty()) {
        networks = hwinfo::getAllNetworks();
    }
    return static_cast<int>(networks.size());
}

C_Network* get_all_networks() {
    if (networks.empty()) {
        networks = hwinfo::getAllNetworks();
    }
    if (networks.empty()) {
        return nullptr;
    }
    C_Network* c_networks = new C_Network[networks.size()];
    for (size_t i = 0; i < networks.size(); ++i) {
        c_networks[i].interfaceIndex = copy_string(networks[i].interfaceIndex());
        c_networks[i].description = copy_string(networks[i].description());
        c_networks[i].mac = copy_string(networks[i].mac());
        c_networks[i].ip4 = copy_string(networks[i].ip4());
        c_networks[i].ip6 = copy_string(networks[i].ip6());
    }
    return c_networks;
}

void free_network_info(C_Network* c_networks, int count) {
    if (c_networks) {
        for (int i = 0; i < count; ++i) {
            delete[] c_networks[i].interfaceIndex;
            delete[] c_networks[i].description;
            delete[] c_networks[i].mac;
            delete[] c_networks[i].ip4;
            delete[] c_networks[i].ip6;
        }
        delete[] c_networks;
    }
}


}  // extern "C"