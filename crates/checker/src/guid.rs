// Helper for Guid (simplified for example)
#[derive(Default, Debug)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl From<*mut windows::core::GUID> for Guid {
    fn from(value: *mut windows::core::GUID) -> Self {
        if value.is_null() {
            return Guid::default();
        }
        let guid = unsafe { *value };

        // Convert the ptr's from the windows GUID to our custom Guid, dereferencing the pointer where needed

        Guid {
            data1: guid.data1,
            data2: guid.data2,
            data3: guid.data3,
            data4: guid.data4,
        }
    }
}

fn push_bytes(mut bytes: Vec<u8>, value: &[u8]) -> Vec<u8> {
    for byte in value.iter() {
        bytes.push(*byte);
    }
    bytes
}

impl From<Guid> for String {
    fn from(value: Guid) -> Self {
        let bytes1 = value.data1.to_le_bytes();
        let bytes2 = value.data2.to_le_bytes();
        let bytes3 = value.data3.to_le_bytes();
        let mut bytes4 = [0u8; 8];
        bytes4.copy_from_slice(&value.data4);

        let mut str = push_bytes(vec![], &bytes1);
        str.append(&mut push_bytes(vec![], &bytes2));
        str.append(&mut push_bytes(vec![], &bytes3));
        str.append(&mut push_bytes(vec![], &bytes4));

        str.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()
    }
}
