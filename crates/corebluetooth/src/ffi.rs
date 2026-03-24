//! ObjC selector constants for CoreBluetooth.
#![allow(dead_code)]

pub mod central_manager {
    pub const CLASS: &[u8] = b"CBCentralManager\0";
    pub const SEL_STATE: &[u8] = b"state\0";
    pub const SEL_IS_SCANNING: &[u8] = b"isScanning\0";
    pub const SEL_SCAN_FOR_PERIPHERALS: &[u8] = b"scanForPeripheralsWithServices:options:\0";
    pub const SEL_STOP_SCAN: &[u8] = b"stopScan\0";
    pub const SEL_CONNECT_PERIPHERAL: &[u8] = b"connectPeripheral:options:\0";
    pub const SEL_CANCEL_CONNECTION: &[u8] = b"cancelPeripheralConnection:\0";
    pub const SEL_RETRIEVE_PERIPHERALS: &[u8] = b"retrievePeripheralsWithIdentifiers:\0";
    pub const SEL_RETRIEVE_CONNECTED: &[u8] = b"retrieveConnectedPeripheralsWithServices:\0";
}

pub mod peripheral_manager {
    pub const CLASS: &[u8] = b"CBPeripheralManager\0";
    pub const SEL_STATE: &[u8] = b"state\0";
    pub const SEL_IS_ADVERTISING: &[u8] = b"isAdvertising\0";
    pub const SEL_START_ADVERTISING: &[u8] = b"startAdvertising:\0";
    pub const SEL_STOP_ADVERTISING: &[u8] = b"stopAdvertising\0";
    pub const SEL_ADD_SERVICE: &[u8] = b"addService:\0";
    pub const SEL_REMOVE_SERVICE: &[u8] = b"removeService:\0";
    pub const SEL_REMOVE_ALL_SERVICES: &[u8] = b"removeAllServices\0";
    pub const SEL_RESPOND_TO_REQUEST: &[u8] = b"respondToRequest:withResult:\0";
    pub const SEL_UPDATE_VALUE: &[u8] = b"updateValue:forCharacteristic:onSubscribedCentrals:\0";
}

pub mod peripheral {
    pub const CLASS: &[u8] = b"CBPeripheral\0";
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_IDENTIFIER: &[u8] = b"identifier\0";
    pub const SEL_STATE: &[u8] = b"state\0";
    pub const SEL_SERVICES: &[u8] = b"services\0";
    pub const SEL_RSSI: &[u8] = b"RSSI\0";
    pub const SEL_DISCOVER_SERVICES: &[u8] = b"discoverServices:\0";
    pub const SEL_DISCOVER_CHARACTERISTICS: &[u8] = b"discoverCharacteristics:forService:\0";
    pub const SEL_READ_VALUE: &[u8] = b"readValueForCharacteristic:\0";
    pub const SEL_WRITE_VALUE: &[u8] = b"writeValue:forCharacteristic:type:\0";
    pub const SEL_SET_NOTIFY_VALUE: &[u8] = b"setNotifyValue:forCharacteristic:\0";
    pub const SEL_DISCOVER_DESCRIPTORS: &[u8] = b"discoverDescriptorsForCharacteristic:\0";
    pub const SEL_READ_RSSI: &[u8] = b"readRSSI\0";
    pub const SEL_MAXIMUM_WRITE_VALUE_LENGTH: &[u8] = b"maximumWriteValueLengthForType:\0";
    pub const SEL_CAN_SEND_WRITE_WITHOUT_RESPONSE: &[u8] = b"canSendWriteWithoutResponse\0";
}

pub mod service {
    pub const CLASS: &[u8] = b"CBService\0";
    pub const SEL_UUID: &[u8] = b"UUID\0";
    pub const SEL_PERIPHERAL: &[u8] = b"peripheral\0";
    pub const SEL_IS_PRIMARY: &[u8] = b"isPrimary\0";
    pub const SEL_CHARACTERISTICS: &[u8] = b"characteristics\0";
    pub const SEL_INCLUDED_SERVICES: &[u8] = b"includedServices\0";
}

pub mod characteristic {
    pub const CLASS: &[u8] = b"CBCharacteristic\0";
    pub const SEL_UUID: &[u8] = b"UUID\0";
    pub const SEL_SERVICE: &[u8] = b"service\0";
    pub const SEL_VALUE: &[u8] = b"value\0";
    pub const SEL_PROPERTIES: &[u8] = b"properties\0";
    pub const SEL_IS_NOTIFYING: &[u8] = b"isNotifying\0";
    pub const SEL_DESCRIPTORS: &[u8] = b"descriptors\0";
}

pub mod uuid {
    pub const CLASS: &[u8] = b"CBUUID\0";
    pub const SEL_UUID_WITH_STRING: &[u8] = b"UUIDWithString:\0";
    pub const SEL_UUID_WITH_DATA: &[u8] = b"UUIDWithData:\0";
    pub const SEL_UUID_STRING: &[u8] = b"UUIDString\0";
    pub const SEL_DATA: &[u8] = b"data\0";
}

/// CBCharacteristicProperties flags.
pub mod characteristic_properties {
    pub const BROADCAST: usize = 0x01;
    pub const READ: usize = 0x02;
    pub const WRITE_WITHOUT_RESPONSE: usize = 0x04;
    pub const WRITE: usize = 0x08;
    pub const NOTIFY: usize = 0x10;
    pub const INDICATE: usize = 0x20;
    pub const AUTHENTICATED_SIGNED_WRITES: usize = 0x40;
    pub const EXTENDED_PROPERTIES: usize = 0x80;
}

/// CBCharacteristicWriteType.
pub mod write_type {
    pub const WITH_RESPONSE: isize = 0;
    pub const WITHOUT_RESPONSE: isize = 1;
}
