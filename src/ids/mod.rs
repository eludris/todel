//! A simple collection of ID related utilities.
use std::time::{Duration, SystemTime};

lazy_static! {
    pub static ref ELUDRIS_EPOCH: SystemTime =
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_650_000_000);
}

/// Generates an instance id from the instance's name.
pub fn generate_instance_id(instance_name: &str) -> u32 {
    (SystemTime::now()
        .duration_since(*ELUDRIS_EPOCH)
        .expect("Couldn't get current timestamp")
        .as_millis() as u32
        & 0xFFFF)
        << 8
        | *instance_name
            .as_bytes()
            .first()
            .expect("Couldn't find the first character of the instance name") as u32
}

/// A type that's used for ceating and managing IDs by keeping track of a sequence number.
///
/// ## Example
///
/// ```rust
/// use todel::ids::{IDGenerator, generate_instance_id};
///
/// let instance_id = generate_instance_id(&"EpicChat"); // This is ideally fetched from a database.
/// let mut generator = IDGenerator::new(instance_id); // Create a new ID generator with your instance ID.
///
/// generator.generate_id(); // Generate an ID which also increments the sequence.
/// ```
pub struct IDGenerator {
    instance_id: u32,
    sequence: u8,
}

impl IDGenerator {
    /// Creates a new IDGenerator from an instance ID.
    pub fn new(instance_id: u32) -> IDGenerator {
        IDGenerator {
            instance_id,
            sequence: 0,
        }
    }

    pub fn generate_id(&mut self) -> u64 {
        if self.sequence == u8::MAX {
            self.sequence = 0
        } else {
            self.sequence += 1;
        }
        ((SystemTime::now()
            .duration_since(*ELUDRIS_EPOCH)
            .expect("Couldn't get current timestamp")
            .as_millis() as u64)
            << 24
            | (self.instance_id as u64))
            << 8
            | self.sequence as u64
    }
}

#[cfg(test)]
mod tests {
    use super::{generate_instance_id, IDGenerator};

    #[test]
    fn instance_id() {
        let instance_name = "WooChat";
        let id = generate_instance_id(&instance_name);

        assert_eq!(id & 0xFF, *instance_name.as_bytes().first().unwrap() as u32)
    }

    #[test]
    fn id_generator() {
        let instance_id = generate_instance_id("WooChat");
        let mut generator = IDGenerator::new(instance_id);

        let id = generator.generate_id();
        assert_eq!(id & 0xFF, 1);
        assert_eq!((id & 0xFFFFFF00) >> 8, instance_id as u64);

        let id = generator.generate_id();
        assert_eq!(id & 0xFF, 2);
        assert_eq!((id & 0xFFFFFF00) >> 8, instance_id as u64);
    }

    #[test]
    fn id_generator_overflow() {
        let instance_id = generate_instance_id("WooChat");
        let mut generator = IDGenerator {
            instance_id,
            sequence: u8::MAX - 1,
        };

        let id = generator.generate_id();
        assert_eq!(id & 0xFF, u8::MAX as u64);
        assert_eq!((id & 0xFFFFFF00) >> 8, instance_id as u64);

        let id = generator.generate_id();
        assert_eq!(id & 0xFF, 0);
        assert_eq!((id & 0xFFFFFF00) >> 8, instance_id as u64);

        let id = generator.generate_id();
        assert_eq!(id & 0xFF, 1);
        assert_eq!((id & 0xFFFFFF00) >> 8, instance_id as u64);
    }
}
