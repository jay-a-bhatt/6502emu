pub mod lda {
    use crate::Byte;
    pub const IMMEDIATE: Byte = 0xA9; // Same as INS_LDA_IM in video, for example
    pub const ZERO_PAGE: Byte = 0xA5;
    pub const ZERO_PAGE_X: Byte = 0xB5;
    pub const ABSOLUTE: Byte = 0xAD;
    pub const ABSOLUTE_X: Byte = 0xBD;
    pub const ABSOLUTE_Y: Byte = 0xB9;
    pub const INDIRECT_X: Byte = 0xA1;
    pub const INDIRECT_Y: Byte = 0xB1;
}

pub mod ldx {
    use crate::Byte;
    pub const IMMEDIATE: Byte = 0xA2;
    pub const ZERO_PAGE: Byte = 0xA6;
    pub const ZERO_PAGE_Y: Byte = 0xB6;
    pub const ABSOLUTE: Byte = 0xAE;
    pub const ABSOLUTE_Y: Byte = 0xBE;
}

pub mod ldy {
    use crate::Byte;
    pub const IMMEDIATE: Byte = 0xA0;
}