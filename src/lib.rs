// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::*, prelude::*, storage::*};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
#[storage]
#[entrypoint]
pub struct IoTDataProcessor {
    temperature: StorageVec<StorageU256>,
    humidity: StorageVec<StorageU256>,
    vibration1: StorageVec<StorageU256>,
    vibration2: StorageVec<StorageU256>,
    gyro: StorageVec<StorageU256>,

    min_temp: StorageU256,
    max_temp: StorageU256,
    min_hum: StorageU256,
    max_hum: StorageU256,
    max_vib1: StorageU256,
    max_vib2: StorageU256,

    final_state: StorageU256, // 0: Needs to be checked, 1: Okay, 2: Not okay
}


/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl IoTDataProcessor {
    pub fn init(&mut self,
        min_temp: U256,
        max_temp: U256,
        min_hum: U256,
        max_hum: U256,
        max_vib1: U256,
        max_vib2: U256 
    ) {
        let tmp = U256::from(0);
        self.final_state.set(tmp);
    }

    pub fn store_sensor_data(
        &mut self,
        temperature: U256,
        humidity: U256,
        vibration1: U256,
        vibration2: U256,
        gyro: U256,
    ) {
        self.temperature.push(temperature);
        self.humidity.push(humidity);
        self.vibration1.push(vibration1);
        self.vibration2.push(vibration2);
        self.gyro.push(gyro);
    }

    fn is_temperature_within_bounds(&mut self, temperature: U256) -> bool {
        temperature >= self.min_temp.get() && 
        temperature <= self.max_temp.get()
    }

    fn is_humidity_within_bounds(&mut self, humidity: U256) -> bool {
        humidity >= self.min_hum.get() && 
        humidity <= self.max_hum.get()
    }

    fn is_vibration1_within_bounds(&mut self, vibration1: U256) -> bool {
        vibration1 <= self.max_vib1.get()
    }

    fn is_vibration2_within_bounds(&mut self, vibration2: U256) -> bool {
        vibration2 <= self.max_vib2.get()
    }

    pub fn terminate_data_collection(
        &mut self
    ) {
        let mut all_within_bounds = true;

        for i in 0..self.temperature.len() {
            let temp = self.temperature.get(i).unwrap();
            if !self.is_temperature_within_bounds(temp) {
            all_within_bounds = false;
            break;
            }
        }

        if all_within_bounds {
            for i in 0..self.humidity.len() {
            let hum = self.humidity.get(i).unwrap();
            if !self.is_humidity_within_bounds(hum) {
                all_within_bounds = false;
                break;
            }
            }
        }

        if all_within_bounds {
            for i in 0..self.vibration1.len() {
            let vib1 = self.vibration1.get(i).unwrap();
            if !self.is_vibration1_within_bounds(vib1) {
                all_within_bounds = false;
                break;
            }
            }
        }

        if all_within_bounds {
            for i in 0..self.vibration2.len() {
            let vib2 = self.vibration2.get(i).unwrap();
            if !self.is_vibration2_within_bounds(vib2) {
                all_within_bounds = false;
                break;
            }
            }
        }

        if all_within_bounds {
            // All data is within bounds, proceed with termination logic
            let tmp = U256::from(1);
            self.final_state.set(tmp); // Okay
        } else {
            // Handle the case where some data is out of bounds
            let tmp = U256::from(2);
            self.final_state.set(tmp); // Not okay
        }
    }

    pub fn print_temp(&mut self,
    i: U256) -> U256 {
        self.temperature.get(i).unwrap()
    }

    pub fn print_final_state(&mut self) -> U256 {
        self.final_state.get()
    }
}
