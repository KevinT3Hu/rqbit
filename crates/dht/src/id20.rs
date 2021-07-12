use std::cmp::Ordering;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Id20(pub [u8; 20]);

impl std::fmt::Debug for Id20 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;
        for byte in self.0 {
            write!(f, "{:02x?}", byte)?;
        }
        write!(f, ">")?;
        Ok(())
    }
}

impl Serialize for Id20 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for Id20 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Id20;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a 20 byte slice")
            }
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() != 20 {
                    return Err(E::invalid_length(20, &self));
                }
                let mut buf = [0u8; 20];
                buf.copy_from_slice(&v);
                Ok(Id20(buf))
            }
        }
        deserializer.deserialize_bytes(Visitor {})
    }
}

impl Id20 {
    pub fn distance(&self, other: &Id20) -> Id20 {
        let mut xor = [0u8; 20];
        for (idx, (s, o)) in self
            .0
            .iter()
            .copied()
            .zip(other.0.iter().copied())
            .enumerate()
        {
            xor[idx] = s ^ o;
        }
        Id20(xor)
    }
}

impl Ord for Id20 {
    fn cmp(&self, other: &Id20) -> Ordering {
        for (s, o) in self.0.iter().copied().zip(other.0.iter().copied()) {
            match s.cmp(&o) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => continue,
                Ordering::Greater => return Ordering::Greater,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd<Id20> for Id20 {
    fn partial_cmp(&self, other: &Id20) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
