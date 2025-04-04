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
    max_temp_change: StorageU256,

    min_hum: StorageU256,
    max_hum: StorageU256,
    max_vib1: StorageU256,
    max_vib2: StorageU256,

    final_state: StorageU256, // 0: Needs to be checked, 1: Okay, 2: Not okay

    public_key_e: StorageU256,
    public_key_n: StorageU256, 
}


/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl IoTDataProcessor {
    pub fn init(&mut self,
        min_temp: U256,
        max_temp: U256,
        max_temp_change: U256,
        min_hum: U256,
        max_hum: U256,
        max_vib1: U256,
        max_vib2: U256,
        public_key_e: U256,
        public_key_n: U256,
    ) {
        let tmp = U256::from(0);
        self.final_state.set(tmp);
        self.min_temp.set(min_temp);
        self.max_temp.set(max_temp);
        self.max_temp_change.set(max_temp_change);
        self.min_hum.set(min_hum);
        self.max_hum.set(max_hum);
        self.max_vib1.set(max_vib1);
        self.max_vib2.set(max_vib2);
        self.public_key_e.set(public_key_e);
        self.public_key_n.set(public_key_n);
    }

    pub fn store_sensor_data(
        &mut self,
        temperature: U256,
        humidity: U256,
        vibration1: U256,
        vibration2: U256,
        gyro: U256,
        signature: U256,
    ) {
        // Verify the signature using the public key
        let hash = temperature + humidity + vibration1 + vibration2 + gyro;
        if hash != self.decrypt(signature, (self.public_key_e.get(), self.public_key_n.get())) {
            // Signature verification failed
            panic!("Signature verification failed");
        }

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
            for i in 1..self.temperature.len() {
                let current_temp = self.temperature.get(i).unwrap();
                let previous_temp = self.temperature.get(i - 1).unwrap();
                
                // Calculate the absolute difference (discrete derivative)
                let difference = if current_temp > previous_temp {
                    current_temp - previous_temp
                } else {
                    previous_temp - current_temp
                };
                
                // Check if the difference exceeds the maximum allowed rate of change
                if U256::from(difference) > self.max_temp_change.get() {  // Assumes max_temp_change is a U256 field in your struct
                    all_within_bounds = false;
                    break;
                }
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

    pub fn encrypt(&mut self,message: U256, public_key: (U256, U256)) -> U256 {
        let (e, n) = public_key;
        self.mod_exp(message, e, n) // Modular exponentiation
    }

    pub fn decrypt(&mut self,cipher: U256, private_key: (U256, U256)) -> U256 {
        let (d, n) = private_key;
        self.mod_exp(cipher, d, n) // Modular exponentiation
    }

    /// Efficient modular exponentiation (base^exp % modulus)
    pub fn mod_exp(&mut self,mut base: U256, mut exp: U256, modulus: U256) -> U256 {
        let mut result: U256 = U256::from(1);
        base %= modulus;
        while exp > U256::from(0) {
            if exp % U256::from(2) == U256::from(1) {
                result = (result * base) % modulus;
            }
            exp /= U256::from(2);
            base = (base * base) % modulus;
        }
        result
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rsa(){
        use stylus_sdk::{console, testing::*};
        let vm = TestVM::default();
        let mut contract = IoTDataProcessor::from(&vm);
        
        let res = contract.encrypt(U256::from(56), (U256::from(65537), U256::from(3233)));
        console!(res);
    }
}