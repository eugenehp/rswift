//! ObjC selector constants for Virtualization.
#![allow(dead_code)]

// ── VZVirtualMachine (8 methods, 9 properties) ──
pub mod v_z_virtual_machine {
    pub const CLASS: &[u8] = b"VZVirtualMachine\0";
    pub const SEL_SUPPORTED: &[u8] = b"supported\0";
    pub const SEL_SET_SUPPORTED: &[u8] = b"setSupported:\0";
    pub const SEL_STATE: &[u8] = b"state\0";
    pub const SEL_SET_STATE: &[u8] = b"setState:\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_CAN_START: &[u8] = b"canStart\0";
    pub const SEL_SET_CAN_START: &[u8] = b"setCanStart:\0";
    pub const SEL_CAN_PAUSE: &[u8] = b"canPause\0";
    pub const SEL_SET_CAN_PAUSE: &[u8] = b"setCanPause:\0";
    pub const SEL_CAN_RESUME: &[u8] = b"canResume\0";
    pub const SEL_SET_CAN_RESUME: &[u8] = b"setCanResume:\0";
    pub const SEL_CAN_REQUEST_STOP: &[u8] = b"canRequestStop\0";
    pub const SEL_SET_CAN_REQUEST_STOP: &[u8] = b"setCanRequestStop:\0";
    pub const SEL_MEMORY_BALLOON_DEVICES: &[u8] = b"memoryBalloonDevices\0";
    pub const SEL_SET_MEMORY_BALLOON_DEVICES: &[u8] = b"setMemoryBalloonDevices:\0";
    pub const SEL_SOCKET_DEVICES: &[u8] = b"socketDevices\0";
    pub const SEL_SET_SOCKET_DEVICES: &[u8] = b"setSocketDevices:\0";
    pub const SEL_REQUEST_STOP_WITH_ERROR: &[u8] = b"requestStopWithError:\0";
}

// ── VZVirtualMachineConfiguration (0 methods, 9 properties) ──
pub mod v_z_virtual_machine_configuration {
    pub const SEL_BOOT_LOADER: &[u8] = b"bootLoader\0";
    pub const SEL_SET_BOOT_LOADER: &[u8] = b"setBootLoader:\0";
    pub const SEL_MEMORY_SIZE: &[u8] = b"memorySize\0";
    pub const SEL_SET_MEMORY_SIZE: &[u8] = b"setMemorySize:\0";
    pub const SEL_C_P_U_COUNT: &[u8] = b"CPUCount\0";
    pub const SEL_SET_C_P_U_COUNT: &[u8] = b"setCPUCount:\0";
    pub const SEL_ENTROPY_DEVICES: &[u8] = b"entropyDevices\0";
    pub const SEL_SET_ENTROPY_DEVICES: &[u8] = b"setEntropyDevices:\0";
    pub const SEL_NETWORK_DEVICES: &[u8] = b"networkDevices\0";
    pub const SEL_SET_NETWORK_DEVICES: &[u8] = b"setNetworkDevices:\0";
    pub const SEL_SERIAL_PORTS: &[u8] = b"serialPorts\0";
    pub const SEL_SET_SERIAL_PORTS: &[u8] = b"setSerialPorts:\0";
    pub const SEL_STORAGE_DEVICES: &[u8] = b"storageDevices\0";
    pub const SEL_SET_STORAGE_DEVICES: &[u8] = b"setStorageDevices:\0";
}

// ── VZLinuxBootLoader (0 methods, 3 properties) ──
pub mod v_z_linux_boot_loader {
    pub const SEL_KERNEL_U_R_L: &[u8] = b"kernelURL\0";
    pub const SEL_SET_KERNEL_U_R_L: &[u8] = b"setKernelURL:\0";
    pub const SEL_COMMAND_LINE: &[u8] = b"commandLine\0";
    pub const SEL_SET_COMMAND_LINE: &[u8] = b"setCommandLine:\0";
    pub const SEL_INITIAL_RAMDISK_U_R_L: &[u8] = b"initialRamdiskURL\0";
    pub const SEL_SET_INITIAL_RAMDISK_U_R_L: &[u8] = b"setInitialRamdiskURL:\0";
}

// ── VZDiskImageStorageDeviceAttachment (0 methods, 2 properties) ──
pub mod v_z_disk_image_storage_device_attachment {
    pub const SEL_U_R_L: &[u8] = b"URL\0";
    pub const SEL_SET_U_R_L: &[u8] = b"setURL:\0";
    pub const SEL_READ_ONLY: &[u8] = b"readOnly\0";
    pub const SEL_SET_READ_ONLY: &[u8] = b"setReadOnly:\0";
}

// ── VZVirtioNetworkDeviceConfiguration (0 methods, 0 properties) ──
pub mod v_z_virtio_network_device_configuration {
}

// ── VZNATNetworkDeviceAttachment (0 methods, 0 properties) ──
pub mod v_z_n_a_t_network_device_attachment {
}

// ── VZSharedDirectory (0 methods, 2 properties) ──
pub mod v_z_shared_directory {
}

// Total: 58 selector constants
