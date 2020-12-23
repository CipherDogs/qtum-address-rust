use basex_rs::{BaseX, Decode, Encode, BITCOIN};
use bitcoin_hashes::sha256;
use bitcoin_hashes::Hash;
use hex;

/// 3a - mainnet
/// 78 - testnet
pub struct QtumAddress {
    prefix: u8,
}

impl QtumAddress {
    /// Converts a base58 pubkeyhash address to a hex address for use in smart contracts.
    pub fn gethexaddress(&self, address: &str) -> Result<String, &str> {
        if address.is_empty() {
            return Err("Invalid address");
        }

        let decode_bytes = match BaseX::new(BITCOIN).decode(address.to_string()) {
            Some(bytes) => bytes,
            None => return Err("Invalid address"),
        };

        let new_bytes = match decode_bytes.get(1..21) {
            Some(hash) => hash,
            None => return Err("Invalid address"),
        };

        let hex = hex::encode(new_bytes);

        Ok(hex)
    }

    /// Converts a raw hex address to a base58 pubkeyhash address
    pub fn fromhexaddress(&self, address: &str) -> Result<String, &str> {
        if address.is_empty() || address.len() != 40 {
            return Err("Invalid address");
        }

        let mut address_bytes = hex::decode(address).unwrap();
        address_bytes.insert(0, self.prefix);

        let checksum = self.hash_sha256(&self.hash_sha256(&address_bytes));
        match checksum.get(0..4) {
            Some(hash) => {
                for byte in hash.iter() {
                    address_bytes.push(*byte);
                }
            }
            None => return Err("Invalid address"),
        };

        let encode = BaseX::new(BITCOIN).encode(&address_bytes);

        Ok(encode)
    }

    /// Adding an ethereum address prefix
    pub fn addprefix(address: &str) -> String {
        format!("0x{}", address)
    }

    /// SHA256 hash function
    fn hash_sha256(&self, byte: &Vec<u8>) -> Vec<u8> {
        hex::decode(sha256::Hash::hash(byte.as_slice()).to_string()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_testnet() {
        let eth_addresses = [
            "6c89a1a6ca2ae7c00b248bb2832d6f480f27da68",
            "49a80104c0d27a9ba29678d07e87a57151107613",
        ];

        let qtum_addresses = [
            "qTTH1Yr2eKCuDLqfxUyBLCAjmomQ8pyrBt",
            "qQGqkA16ZY6bCYy7Qjr77eU4BPsdadibCG",
        ];

        let qtum = QtumAddress { prefix: 0x78 };
        for addr in qtum_addresses.iter() {
            let eth_addr = qtum.gethexaddress(addr).unwrap();
            let qtum_addr = qtum.fromhexaddress(&eth_addr).unwrap();
            assert_eq!(qtum_addr.to_string(), addr.to_string());
        }

        for addr in eth_addresses.iter() {
            let qtum_addr = qtum.fromhexaddress(addr).unwrap();
            let eth_addr = qtum.gethexaddress(&qtum_addr).unwrap();
            assert_eq!(eth_addr.to_string(), addr.to_string());
        }
    }
}
