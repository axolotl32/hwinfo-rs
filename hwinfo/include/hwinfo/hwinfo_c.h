#ifndef HWINFO_C_H
#define HWINFO_C_H

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// --- Generic Array Structs ---
// Used for returning arrays of strings (e.g., CPU flags, disk volumes).
typedef struct {
  int count;
  char** strings;
} C_StringArray;

// Used for returning arrays of doubles (e.g., CPU thread utilizations).
typedef struct {
  int count;
  double* values;
} C_DoubleArray;

// Used for returning arrays of int64_t (e.g., CPU thread clock speeds).
typedef struct {
  int count;
  int64_t* values;
} C_Int64Array;

// --- Component-Specific Structs ---

typedef struct {
  int id;
  char* vendor;
  char* modelName;
  int numPhysicalCores;
  int numLogicalCores;
  int64_t maxClockSpeed_MHz;
  int64_t regularClockSpeed_MHz;
  int64_t L1CacheSize_Bytes;
  int64_t L2CacheSize_Bytes;
  int64_t L3CacheSize_Bytes;
  C_StringArray flags;
} C_CPU;

typedef struct {
  char* name;
  char* version;
  char* kernel;
  bool is32bit;
  bool is64bit;
  bool isLittleEndian;
} C_OS;

typedef struct {
  int id;
  char* vendor;
  char* name;
  char* driverVersion;
  int64_t memory_Bytes;
  int64_t frequency_MHz;
  int num_cores;
  char* vendor_id;
  char* device_id;
} C_GPU;

typedef struct {
  int id;
  char* vendor;
  char* name;
  char* model;
  char* serial_number;
  int64_t total_Bytes;
  int64_t frequency_Hz;
} C_RAM_Module;

typedef struct {
  int64_t total_Bytes;
  int64_t free_Bytes;
  int64_t available_Bytes;
  int module_count;
  C_RAM_Module* modules;
} C_MemoryInfo;

typedef struct {
  char* vendor;
  char* name;
  char* version;
  char* serialNumber;
} C_MainBoard;

typedef struct {
  int id;
  char* vendor;
  char* model;
  char* serialNumber;
  int64_t size_Bytes;
  int64_t free_size_Bytes;
  C_StringArray volumes;
} C_Disk;

typedef struct {
  int id;
  char* vendor;
  char* model;
  char* serialNumber;
  char* technology;
  uint32_t energyFull;
  uint32_t energyNow;
  bool charging;
} C_Battery;

typedef struct {
  char* interfaceIndex;
  char* description;
  char* mac;
  char* ip4;
  char* ip6;
} C_Network;


// --- C API Functions ---
// Note: For every 'get' function that returns a pointer, you MUST call the
// corresponding 'free' function to avoid memory leaks.

// CPU
int get_cpu_count();
C_CPU* get_all_cpus();
double get_cpu_utilization(int cpu_id); // Overall utilization for a given CPU socket
C_DoubleArray* get_cpu_thread_utilizations(int cpu_id);
C_Int64Array* get_cpu_thread_speeds_mhz(int cpu_id);
void free_cpu_info(C_CPU* cpus, int count);
void free_double_array(C_DoubleArray* arr);
void free_int64_array(C_Int64Array* arr);

// OS
C_OS* get_os_info();
void free_os_info(C_OS* os);

// GPU
int get_gpu_count();
C_GPU* get_all_gpus();
void free_gpu_info(C_GPU* gpus, int count);

// Memory
C_MemoryInfo* get_memory_info();
void free_memory_info(C_MemoryInfo* memory_info);

// Mainboard
C_MainBoard* get_mainboard_info();
void free_mainboard_info(C_MainBoard* mainboard);

// Disk
int get_disk_count();
C_Disk* get_all_disks();
void free_disk_info(C_Disk* disks, int count);

// Battery
int get_battery_count();
C_Battery* get_all_batteries();
void free_battery_info(C_Battery* batteries, int count);

// Network
int get_network_count();
C_Network* get_all_networks();
void free_network_info(C_Network* networks, int count);

#ifdef __cplusplus
}
#endif

#endif  // HWINFO_C_H