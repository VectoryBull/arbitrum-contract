#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::*, prelude::*, storage::*};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
#[storage]
#[entrypoint]
pub struct IoTDataProcessor {
    temperature: StorageVec<StorageI256>,
    humidity: StorageVec<StorageI256>,
    vibration1: StorageVec<StorageI256>,
    vibration2: StorageVec<StorageI256>,
    gyro: StorageVec<StorageI256>,
}


/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl IoTDataProcessor {
    pub fn constructor(&self,
        min_temp: I256,
        max_temp: I256,
        min_hum: I256,
        max_hum: I256,
        max_vib1: I256,
        max_vib2: I256 
        // TODO: public key
        ) {
        //
    }

    pub fn store_sensor_data(
        &mut self,
        temperature: I256,
        humidity: I256,
        vibration1: I256,
        vibration2: I256,
        gyro: I256,
    ) {
        self.temperature.push(temperature);
        self.humidity.push(humidity);
        self.vibration1.push(vibration1);
        self.vibration2.push(vibration2);
        self.gyro.push(gyro);
    }

    /*

    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Sets a number in storage to a user-specified value.
    pub fn mul_number(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Sets a number in storage to a user-specified value.
    pub fn add_number(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }

    /// Adds the wei value from msg_value to the number in storage.
    #[payable]
    pub fn add_from_msg_value(&mut self) {
        let number = self.number.get();
        self.set_number(number + self.vm().msg_value());
    }
        */
}
