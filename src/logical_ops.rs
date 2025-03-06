pub mod and {
    use crate::Byte;
    pub const IMMEDIATE: Byte = 0x29;
    pub const ZERO_PAGE: Byte = 0x25;
    pub const ZERO_PAGE_X: Byte = 0x35;
    pub const ABSOLUTE: Byte = 0x2D;
    pub const ABSOLUTE_X: Byte = 0x3D;
    pub const ABSOLUTE_Y: Byte = 0x39;
    pub const INDIRECT_X: Byte = 0x21;
    pub const INDIRECT_Y: Byte = 0x31;
}

pub mod or {
    use crate::Byte;
}

pub mod eor {
    use crate::Byte;
}
