
#[derive(Debug, Clone)]
pub struct Header {
    pub opcode: u16,
    pub object_id: u32,
    pub size: u16
}

impl Header {
    pub fn new(object_id: u32, opcode: u16, size: u16) -> Header {
        Header {
            opcode,
            object_id,
            size
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
       let object_id_bytes = self.object_id.to_le_bytes();
        let opcode_bytes = self.opcode.to_le_bytes();
        let size_bytes = self.size.to_le_bytes();
        let mut request = Vec::new(); 
        request.extend_from_slice(&object_id_bytes);
        request.extend_from_slice(&opcode_bytes);
        request.extend_from_slice(&size_bytes);
        request

    }
}
