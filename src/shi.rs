#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    shicfg1: Shicfg1,
    shicfg2: Shicfg2,
    _reserved2: [u8; 0x02],
    evenable: Evenable,
    evstat: Evstat,
    capability: Capability,
    status: Status,
    _reserved6: [u8; 0x01],
    ibufstat: Ibufstat,
    obufstat: Obufstat,
    shicfg3: Shicfg3,
    shicfg4: Shicfg4,
    shicfg5: Shicfg5,
    evstat2: Evstat2,
    evenable2: Evenable2,
    shicfg6: Shicfg6,
    sbobuf: Sbobuf,
    _reserved15: [u8; 0x0d],
    obuf0: Obuf0,
    obuf1: Obuf1,
    obuf2: Obuf2,
    obuf3: Obuf3,
    obuf4: Obuf4,
    obuf5: Obuf5,
    obuf6: Obuf6,
    obuf7: Obuf7,
    obuf8: Obuf8,
    obuf9: Obuf9,
    obuf10: Obuf10,
    obuf11: Obuf11,
    obuf12: Obuf12,
    obuf13: Obuf13,
    obuf14: Obuf14,
    obuf15: Obuf15,
    obuf16: Obuf16,
    obuf17: Obuf17,
    obuf18: Obuf18,
    obuf19: Obuf19,
    obuf20: Obuf20,
    obuf21: Obuf21,
    obuf22: Obuf22,
    obuf23: Obuf23,
    obuf24: Obuf24,
    obuf25: Obuf25,
    obuf26: Obuf26,
    obuf27: Obuf27,
    obuf28: Obuf28,
    obuf29: Obuf29,
    obuf30: Obuf30,
    obuf31: Obuf31,
    obuf32: Obuf32,
    obuf33: Obuf33,
    obuf34: Obuf34,
    obuf35: Obuf35,
    obuf36: Obuf36,
    obuf37: Obuf37,
    obuf38: Obuf38,
    obuf39: Obuf39,
    obuf40: Obuf40,
    obuf41: Obuf41,
    obuf42: Obuf42,
    obuf43: Obuf43,
    obuf44: Obuf44,
    obuf45: Obuf45,
    obuf46: Obuf46,
    obuf47: Obuf47,
    obuf48: Obuf48,
    obuf49: Obuf49,
    obuf50: Obuf50,
    obuf51: Obuf51,
    obuf52: Obuf52,
    obuf53: Obuf53,
    obuf54: Obuf54,
    obuf55: Obuf55,
    obuf56: Obuf56,
    obuf57: Obuf57,
    obuf58: Obuf58,
    obuf59: Obuf59,
    obuf60: Obuf60,
    obuf61: Obuf61,
    obuf62: Obuf62,
    obuf63: Obuf63,
    obuf64: Obuf64,
    obuf65: Obuf65,
    obuf66: Obuf66,
    obuf67: Obuf67,
    obuf68: Obuf68,
    obuf69: Obuf69,
    obuf70: Obuf70,
    obuf71: Obuf71,
    obuf72: Obuf72,
    obuf73: Obuf73,
    obuf74: Obuf74,
    obuf75: Obuf75,
    obuf76: Obuf76,
    obuf77: Obuf77,
    obuf78: Obuf78,
    obuf79: Obuf79,
    obuf80: Obuf80,
    obuf81: Obuf81,
    obuf82: Obuf82,
    obuf83: Obuf83,
    obuf84: Obuf84,
    obuf85: Obuf85,
    obuf86: Obuf86,
    obuf87: Obuf87,
    obuf88: Obuf88,
    obuf89: Obuf89,
    obuf90: Obuf90,
    obuf91: Obuf91,
    obuf92: Obuf92,
    obuf93: Obuf93,
    obuf94: Obuf94,
    obuf95: Obuf95,
    obuf96: Obuf96,
    obuf97: Obuf97,
    obuf98: Obuf98,
    obuf99: Obuf99,
    obuf100: Obuf100,
    obuf101: Obuf101,
    obuf102: Obuf102,
    obuf103: Obuf103,
    obuf104: Obuf104,
    obuf105: Obuf105,
    obuf106: Obuf106,
    obuf107: Obuf107,
    obuf108: Obuf108,
    obuf109: Obuf109,
    obuf110: Obuf110,
    obuf111: Obuf111,
    obuf112: Obuf112,
    obuf113: Obuf113,
    obuf114: Obuf114,
    obuf115: Obuf115,
    obuf116: Obuf116,
    obuf117: Obuf117,
    obuf118: Obuf118,
    obuf119: Obuf119,
    obuf120: Obuf120,
    obuf121: Obuf121,
    obuf122: Obuf122,
    obuf123: Obuf123,
    obuf124: Obuf124,
    obuf125: Obuf125,
    obuf126: Obuf126,
    obuf127: Obuf127,
    ibuf0: Ibuf0,
    ibuf1: Ibuf1,
    ibuf2: Ibuf2,
    ibuf3: Ibuf3,
    ibuf4: Ibuf4,
    ibuf5: Ibuf5,
    ibuf6: Ibuf6,
    ibuf7: Ibuf7,
    ibuf8: Ibuf8,
    ibuf9: Ibuf9,
    ibuf10: Ibuf10,
    ibuf11: Ibuf11,
    ibuf12: Ibuf12,
    ibuf13: Ibuf13,
    ibuf14: Ibuf14,
    ibuf15: Ibuf15,
    ibuf16: Ibuf16,
    ibuf17: Ibuf17,
    ibuf18: Ibuf18,
    ibuf19: Ibuf19,
    ibuf20: Ibuf20,
    ibuf21: Ibuf21,
    ibuf22: Ibuf22,
    ibuf23: Ibuf23,
    ibuf24: Ibuf24,
    ibuf25: Ibuf25,
    ibuf26: Ibuf26,
    ibuf27: Ibuf27,
    ibuf28: Ibuf28,
    ibuf29: Ibuf29,
    ibuf30: Ibuf30,
    ibuf31: Ibuf31,
    ibuf32: Ibuf32,
    ibuf33: Ibuf33,
    ibuf34: Ibuf34,
    ibuf35: Ibuf35,
    ibuf36: Ibuf36,
    ibuf37: Ibuf37,
    ibuf38: Ibuf38,
    ibuf39: Ibuf39,
    ibuf40: Ibuf40,
    ibuf41: Ibuf41,
    ibuf42: Ibuf42,
    ibuf43: Ibuf43,
    ibuf44: Ibuf44,
    ibuf45: Ibuf45,
    ibuf46: Ibuf46,
    ibuf47: Ibuf47,
    ibuf48: Ibuf48,
    ibuf49: Ibuf49,
    ibuf50: Ibuf50,
    ibuf51: Ibuf51,
    ibuf52: Ibuf52,
    ibuf53: Ibuf53,
    ibuf54: Ibuf54,
    ibuf55: Ibuf55,
    ibuf56: Ibuf56,
    ibuf57: Ibuf57,
    ibuf58: Ibuf58,
    ibuf59: Ibuf59,
    ibuf60: Ibuf60,
    ibuf61: Ibuf61,
    ibuf62: Ibuf62,
    ibuf63: Ibuf63,
    ibuf64: Ibuf64,
    ibuf65: Ibuf65,
    ibuf66: Ibuf66,
    ibuf67: Ibuf67,
    ibuf68: Ibuf68,
    ibuf69: Ibuf69,
    ibuf70: Ibuf70,
    ibuf71: Ibuf71,
    ibuf72: Ibuf72,
    ibuf73: Ibuf73,
    ibuf74: Ibuf74,
    ibuf75: Ibuf75,
    ibuf76: Ibuf76,
    ibuf77: Ibuf77,
    ibuf78: Ibuf78,
    ibuf79: Ibuf79,
    ibuf80: Ibuf80,
    ibuf81: Ibuf81,
    ibuf82: Ibuf82,
    ibuf83: Ibuf83,
    ibuf84: Ibuf84,
    ibuf85: Ibuf85,
    ibuf86: Ibuf86,
    ibuf87: Ibuf87,
    ibuf88: Ibuf88,
    ibuf89: Ibuf89,
    ibuf90: Ibuf90,
    ibuf91: Ibuf91,
    ibuf92: Ibuf92,
    ibuf93: Ibuf93,
    ibuf94: Ibuf94,
    ibuf95: Ibuf95,
    ibuf96: Ibuf96,
    ibuf97: Ibuf97,
    ibuf98: Ibuf98,
    ibuf99: Ibuf99,
    ibuf100: Ibuf100,
    ibuf101: Ibuf101,
    ibuf102: Ibuf102,
    ibuf103: Ibuf103,
    ibuf104: Ibuf104,
    ibuf105: Ibuf105,
    ibuf106: Ibuf106,
    ibuf107: Ibuf107,
    ibuf108: Ibuf108,
    ibuf109: Ibuf109,
    ibuf110: Ibuf110,
    ibuf111: Ibuf111,
    ibuf112: Ibuf112,
    ibuf113: Ibuf113,
    ibuf114: Ibuf114,
    ibuf115: Ibuf115,
    ibuf116: Ibuf116,
    ibuf117: Ibuf117,
    ibuf118: Ibuf118,
    ibuf119: Ibuf119,
    ibuf120: Ibuf120,
    ibuf121: Ibuf121,
    ibuf122: Ibuf122,
    ibuf123: Ibuf123,
    ibuf124: Ibuf124,
    ibuf125: Ibuf125,
    ibuf126: Ibuf126,
    ibuf127: Ibuf127,
}
impl RegisterBlock {
    #[doc = "0x01 - SHI Configuration 1"]
    #[inline(always)]
    pub const fn shicfg1(&self) -> &Shicfg1 {
        &self.shicfg1
    }
    #[doc = "0x02 - SHI Configuration 2"]
    #[inline(always)]
    pub const fn shicfg2(&self) -> &Shicfg2 {
        &self.shicfg2
    }
    #[doc = "0x05 - Event Enable"]
    #[inline(always)]
    pub const fn evenable(&self) -> &Evenable {
        &self.evenable
    }
    #[doc = "0x06 - Event Status"]
    #[inline(always)]
    pub const fn evstat(&self) -> &Evstat {
        &self.evstat
    }
    #[doc = "0x07 - SHI Capabilities"]
    #[inline(always)]
    pub const fn capability(&self) -> &Capability {
        &self.capability
    }
    #[doc = "0x08 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0a - Input Buffer Status"]
    #[inline(always)]
    pub const fn ibufstat(&self) -> &Ibufstat {
        &self.ibufstat
    }
    #[doc = "0x0b - Output Buffer Status"]
    #[inline(always)]
    pub const fn obufstat(&self) -> &Obufstat {
        &self.obufstat
    }
    #[doc = "0x0c - SHI Configuration 3"]
    #[inline(always)]
    pub const fn shicfg3(&self) -> &Shicfg3 {
        &self.shicfg3
    }
    #[doc = "0x0d - SHI Configuration 4"]
    #[inline(always)]
    pub const fn shicfg4(&self) -> &Shicfg4 {
        &self.shicfg4
    }
    #[doc = "0x0e - SHI Configuration 5"]
    #[inline(always)]
    pub const fn shicfg5(&self) -> &Shicfg5 {
        &self.shicfg5
    }
    #[doc = "0x0f - Event Status 2"]
    #[inline(always)]
    pub const fn evstat2(&self) -> &Evstat2 {
        &self.evstat2
    }
    #[doc = "0x10 - Event Enable 2"]
    #[inline(always)]
    pub const fn evenable2(&self) -> &Evenable2 {
        &self.evenable2
    }
    #[doc = "0x11 - SHI Configuration 6"]
    #[inline(always)]
    pub const fn shicfg6(&self) -> &Shicfg6 {
        &self.shicfg6
    }
    #[doc = "0x12 - Single Byte Output Buffer"]
    #[inline(always)]
    pub const fn sbobuf(&self) -> &Sbobuf {
        &self.sbobuf
    }
    #[doc = "0x20 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf0(&self) -> &Obuf0 {
        &self.obuf0
    }
    #[doc = "0x21 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf1(&self) -> &Obuf1 {
        &self.obuf1
    }
    #[doc = "0x22 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf2(&self) -> &Obuf2 {
        &self.obuf2
    }
    #[doc = "0x23 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf3(&self) -> &Obuf3 {
        &self.obuf3
    }
    #[doc = "0x24 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf4(&self) -> &Obuf4 {
        &self.obuf4
    }
    #[doc = "0x25 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf5(&self) -> &Obuf5 {
        &self.obuf5
    }
    #[doc = "0x26 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf6(&self) -> &Obuf6 {
        &self.obuf6
    }
    #[doc = "0x27 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf7(&self) -> &Obuf7 {
        &self.obuf7
    }
    #[doc = "0x28 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf8(&self) -> &Obuf8 {
        &self.obuf8
    }
    #[doc = "0x29 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf9(&self) -> &Obuf9 {
        &self.obuf9
    }
    #[doc = "0x2a - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf10(&self) -> &Obuf10 {
        &self.obuf10
    }
    #[doc = "0x2b - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf11(&self) -> &Obuf11 {
        &self.obuf11
    }
    #[doc = "0x2c - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf12(&self) -> &Obuf12 {
        &self.obuf12
    }
    #[doc = "0x2d - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf13(&self) -> &Obuf13 {
        &self.obuf13
    }
    #[doc = "0x2e - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf14(&self) -> &Obuf14 {
        &self.obuf14
    }
    #[doc = "0x2f - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf15(&self) -> &Obuf15 {
        &self.obuf15
    }
    #[doc = "0x30 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf16(&self) -> &Obuf16 {
        &self.obuf16
    }
    #[doc = "0x31 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf17(&self) -> &Obuf17 {
        &self.obuf17
    }
    #[doc = "0x32 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf18(&self) -> &Obuf18 {
        &self.obuf18
    }
    #[doc = "0x33 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf19(&self) -> &Obuf19 {
        &self.obuf19
    }
    #[doc = "0x34 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf20(&self) -> &Obuf20 {
        &self.obuf20
    }
    #[doc = "0x35 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf21(&self) -> &Obuf21 {
        &self.obuf21
    }
    #[doc = "0x36 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf22(&self) -> &Obuf22 {
        &self.obuf22
    }
    #[doc = "0x37 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf23(&self) -> &Obuf23 {
        &self.obuf23
    }
    #[doc = "0x38 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf24(&self) -> &Obuf24 {
        &self.obuf24
    }
    #[doc = "0x39 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf25(&self) -> &Obuf25 {
        &self.obuf25
    }
    #[doc = "0x3a - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf26(&self) -> &Obuf26 {
        &self.obuf26
    }
    #[doc = "0x3b - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf27(&self) -> &Obuf27 {
        &self.obuf27
    }
    #[doc = "0x3c - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf28(&self) -> &Obuf28 {
        &self.obuf28
    }
    #[doc = "0x3d - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf29(&self) -> &Obuf29 {
        &self.obuf29
    }
    #[doc = "0x3e - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf30(&self) -> &Obuf30 {
        &self.obuf30
    }
    #[doc = "0x3f - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf31(&self) -> &Obuf31 {
        &self.obuf31
    }
    #[doc = "0x40 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf32(&self) -> &Obuf32 {
        &self.obuf32
    }
    #[doc = "0x41 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf33(&self) -> &Obuf33 {
        &self.obuf33
    }
    #[doc = "0x42 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf34(&self) -> &Obuf34 {
        &self.obuf34
    }
    #[doc = "0x43 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf35(&self) -> &Obuf35 {
        &self.obuf35
    }
    #[doc = "0x44 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf36(&self) -> &Obuf36 {
        &self.obuf36
    }
    #[doc = "0x45 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf37(&self) -> &Obuf37 {
        &self.obuf37
    }
    #[doc = "0x46 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf38(&self) -> &Obuf38 {
        &self.obuf38
    }
    #[doc = "0x47 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf39(&self) -> &Obuf39 {
        &self.obuf39
    }
    #[doc = "0x48 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf40(&self) -> &Obuf40 {
        &self.obuf40
    }
    #[doc = "0x49 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf41(&self) -> &Obuf41 {
        &self.obuf41
    }
    #[doc = "0x4a - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf42(&self) -> &Obuf42 {
        &self.obuf42
    }
    #[doc = "0x4b - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf43(&self) -> &Obuf43 {
        &self.obuf43
    }
    #[doc = "0x4c - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf44(&self) -> &Obuf44 {
        &self.obuf44
    }
    #[doc = "0x4d - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf45(&self) -> &Obuf45 {
        &self.obuf45
    }
    #[doc = "0x4e - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf46(&self) -> &Obuf46 {
        &self.obuf46
    }
    #[doc = "0x4f - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf47(&self) -> &Obuf47 {
        &self.obuf47
    }
    #[doc = "0x50 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf48(&self) -> &Obuf48 {
        &self.obuf48
    }
    #[doc = "0x51 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf49(&self) -> &Obuf49 {
        &self.obuf49
    }
    #[doc = "0x52 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf50(&self) -> &Obuf50 {
        &self.obuf50
    }
    #[doc = "0x53 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf51(&self) -> &Obuf51 {
        &self.obuf51
    }
    #[doc = "0x54 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf52(&self) -> &Obuf52 {
        &self.obuf52
    }
    #[doc = "0x55 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf53(&self) -> &Obuf53 {
        &self.obuf53
    }
    #[doc = "0x56 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf54(&self) -> &Obuf54 {
        &self.obuf54
    }
    #[doc = "0x57 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf55(&self) -> &Obuf55 {
        &self.obuf55
    }
    #[doc = "0x58 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf56(&self) -> &Obuf56 {
        &self.obuf56
    }
    #[doc = "0x59 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf57(&self) -> &Obuf57 {
        &self.obuf57
    }
    #[doc = "0x5a - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf58(&self) -> &Obuf58 {
        &self.obuf58
    }
    #[doc = "0x5b - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf59(&self) -> &Obuf59 {
        &self.obuf59
    }
    #[doc = "0x5c - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf60(&self) -> &Obuf60 {
        &self.obuf60
    }
    #[doc = "0x5d - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf61(&self) -> &Obuf61 {
        &self.obuf61
    }
    #[doc = "0x5e - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf62(&self) -> &Obuf62 {
        &self.obuf62
    }
    #[doc = "0x5f - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf63(&self) -> &Obuf63 {
        &self.obuf63
    }
    #[doc = "0x60 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf64(&self) -> &Obuf64 {
        &self.obuf64
    }
    #[doc = "0x61 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf65(&self) -> &Obuf65 {
        &self.obuf65
    }
    #[doc = "0x62 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf66(&self) -> &Obuf66 {
        &self.obuf66
    }
    #[doc = "0x63 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf67(&self) -> &Obuf67 {
        &self.obuf67
    }
    #[doc = "0x64 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf68(&self) -> &Obuf68 {
        &self.obuf68
    }
    #[doc = "0x65 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf69(&self) -> &Obuf69 {
        &self.obuf69
    }
    #[doc = "0x66 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf70(&self) -> &Obuf70 {
        &self.obuf70
    }
    #[doc = "0x67 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf71(&self) -> &Obuf71 {
        &self.obuf71
    }
    #[doc = "0x68 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf72(&self) -> &Obuf72 {
        &self.obuf72
    }
    #[doc = "0x69 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf73(&self) -> &Obuf73 {
        &self.obuf73
    }
    #[doc = "0x6a - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf74(&self) -> &Obuf74 {
        &self.obuf74
    }
    #[doc = "0x6b - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf75(&self) -> &Obuf75 {
        &self.obuf75
    }
    #[doc = "0x6c - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf76(&self) -> &Obuf76 {
        &self.obuf76
    }
    #[doc = "0x6d - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf77(&self) -> &Obuf77 {
        &self.obuf77
    }
    #[doc = "0x6e - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf78(&self) -> &Obuf78 {
        &self.obuf78
    }
    #[doc = "0x6f - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf79(&self) -> &Obuf79 {
        &self.obuf79
    }
    #[doc = "0x70 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf80(&self) -> &Obuf80 {
        &self.obuf80
    }
    #[doc = "0x71 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf81(&self) -> &Obuf81 {
        &self.obuf81
    }
    #[doc = "0x72 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf82(&self) -> &Obuf82 {
        &self.obuf82
    }
    #[doc = "0x73 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf83(&self) -> &Obuf83 {
        &self.obuf83
    }
    #[doc = "0x74 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf84(&self) -> &Obuf84 {
        &self.obuf84
    }
    #[doc = "0x75 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf85(&self) -> &Obuf85 {
        &self.obuf85
    }
    #[doc = "0x76 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf86(&self) -> &Obuf86 {
        &self.obuf86
    }
    #[doc = "0x77 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf87(&self) -> &Obuf87 {
        &self.obuf87
    }
    #[doc = "0x78 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf88(&self) -> &Obuf88 {
        &self.obuf88
    }
    #[doc = "0x79 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf89(&self) -> &Obuf89 {
        &self.obuf89
    }
    #[doc = "0x7a - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf90(&self) -> &Obuf90 {
        &self.obuf90
    }
    #[doc = "0x7b - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf91(&self) -> &Obuf91 {
        &self.obuf91
    }
    #[doc = "0x7c - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf92(&self) -> &Obuf92 {
        &self.obuf92
    }
    #[doc = "0x7d - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf93(&self) -> &Obuf93 {
        &self.obuf93
    }
    #[doc = "0x7e - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf94(&self) -> &Obuf94 {
        &self.obuf94
    }
    #[doc = "0x7f - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf95(&self) -> &Obuf95 {
        &self.obuf95
    }
    #[doc = "0x80 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf96(&self) -> &Obuf96 {
        &self.obuf96
    }
    #[doc = "0x81 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf97(&self) -> &Obuf97 {
        &self.obuf97
    }
    #[doc = "0x82 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf98(&self) -> &Obuf98 {
        &self.obuf98
    }
    #[doc = "0x83 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf99(&self) -> &Obuf99 {
        &self.obuf99
    }
    #[doc = "0x84 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf100(&self) -> &Obuf100 {
        &self.obuf100
    }
    #[doc = "0x85 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf101(&self) -> &Obuf101 {
        &self.obuf101
    }
    #[doc = "0x86 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf102(&self) -> &Obuf102 {
        &self.obuf102
    }
    #[doc = "0x87 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf103(&self) -> &Obuf103 {
        &self.obuf103
    }
    #[doc = "0x88 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf104(&self) -> &Obuf104 {
        &self.obuf104
    }
    #[doc = "0x89 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf105(&self) -> &Obuf105 {
        &self.obuf105
    }
    #[doc = "0x8a - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf106(&self) -> &Obuf106 {
        &self.obuf106
    }
    #[doc = "0x8b - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf107(&self) -> &Obuf107 {
        &self.obuf107
    }
    #[doc = "0x8c - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf108(&self) -> &Obuf108 {
        &self.obuf108
    }
    #[doc = "0x8d - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf109(&self) -> &Obuf109 {
        &self.obuf109
    }
    #[doc = "0x8e - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf110(&self) -> &Obuf110 {
        &self.obuf110
    }
    #[doc = "0x8f - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf111(&self) -> &Obuf111 {
        &self.obuf111
    }
    #[doc = "0x90 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf112(&self) -> &Obuf112 {
        &self.obuf112
    }
    #[doc = "0x91 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf113(&self) -> &Obuf113 {
        &self.obuf113
    }
    #[doc = "0x92 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf114(&self) -> &Obuf114 {
        &self.obuf114
    }
    #[doc = "0x93 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf115(&self) -> &Obuf115 {
        &self.obuf115
    }
    #[doc = "0x94 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf116(&self) -> &Obuf116 {
        &self.obuf116
    }
    #[doc = "0x95 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf117(&self) -> &Obuf117 {
        &self.obuf117
    }
    #[doc = "0x96 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf118(&self) -> &Obuf118 {
        &self.obuf118
    }
    #[doc = "0x97 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf119(&self) -> &Obuf119 {
        &self.obuf119
    }
    #[doc = "0x98 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf120(&self) -> &Obuf120 {
        &self.obuf120
    }
    #[doc = "0x99 - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf121(&self) -> &Obuf121 {
        &self.obuf121
    }
    #[doc = "0x9a - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf122(&self) -> &Obuf122 {
        &self.obuf122
    }
    #[doc = "0x9b - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf123(&self) -> &Obuf123 {
        &self.obuf123
    }
    #[doc = "0x9c - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf124(&self) -> &Obuf124 {
        &self.obuf124
    }
    #[doc = "0x9d - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf125(&self) -> &Obuf125 {
        &self.obuf125
    }
    #[doc = "0x9e - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf126(&self) -> &Obuf126 {
        &self.obuf126
    }
    #[doc = "0x9f - Output Buffer Register"]
    #[inline(always)]
    pub const fn obuf127(&self) -> &Obuf127 {
        &self.obuf127
    }
    #[doc = "0xa0 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf0(&self) -> &Ibuf0 {
        &self.ibuf0
    }
    #[doc = "0xa1 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf1(&self) -> &Ibuf1 {
        &self.ibuf1
    }
    #[doc = "0xa2 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf2(&self) -> &Ibuf2 {
        &self.ibuf2
    }
    #[doc = "0xa3 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf3(&self) -> &Ibuf3 {
        &self.ibuf3
    }
    #[doc = "0xa4 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf4(&self) -> &Ibuf4 {
        &self.ibuf4
    }
    #[doc = "0xa5 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf5(&self) -> &Ibuf5 {
        &self.ibuf5
    }
    #[doc = "0xa6 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf6(&self) -> &Ibuf6 {
        &self.ibuf6
    }
    #[doc = "0xa7 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf7(&self) -> &Ibuf7 {
        &self.ibuf7
    }
    #[doc = "0xa8 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf8(&self) -> &Ibuf8 {
        &self.ibuf8
    }
    #[doc = "0xa9 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf9(&self) -> &Ibuf9 {
        &self.ibuf9
    }
    #[doc = "0xaa - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf10(&self) -> &Ibuf10 {
        &self.ibuf10
    }
    #[doc = "0xab - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf11(&self) -> &Ibuf11 {
        &self.ibuf11
    }
    #[doc = "0xac - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf12(&self) -> &Ibuf12 {
        &self.ibuf12
    }
    #[doc = "0xad - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf13(&self) -> &Ibuf13 {
        &self.ibuf13
    }
    #[doc = "0xae - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf14(&self) -> &Ibuf14 {
        &self.ibuf14
    }
    #[doc = "0xaf - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf15(&self) -> &Ibuf15 {
        &self.ibuf15
    }
    #[doc = "0xb0 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf16(&self) -> &Ibuf16 {
        &self.ibuf16
    }
    #[doc = "0xb1 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf17(&self) -> &Ibuf17 {
        &self.ibuf17
    }
    #[doc = "0xb2 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf18(&self) -> &Ibuf18 {
        &self.ibuf18
    }
    #[doc = "0xb3 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf19(&self) -> &Ibuf19 {
        &self.ibuf19
    }
    #[doc = "0xb4 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf20(&self) -> &Ibuf20 {
        &self.ibuf20
    }
    #[doc = "0xb5 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf21(&self) -> &Ibuf21 {
        &self.ibuf21
    }
    #[doc = "0xb6 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf22(&self) -> &Ibuf22 {
        &self.ibuf22
    }
    #[doc = "0xb7 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf23(&self) -> &Ibuf23 {
        &self.ibuf23
    }
    #[doc = "0xb8 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf24(&self) -> &Ibuf24 {
        &self.ibuf24
    }
    #[doc = "0xb9 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf25(&self) -> &Ibuf25 {
        &self.ibuf25
    }
    #[doc = "0xba - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf26(&self) -> &Ibuf26 {
        &self.ibuf26
    }
    #[doc = "0xbb - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf27(&self) -> &Ibuf27 {
        &self.ibuf27
    }
    #[doc = "0xbc - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf28(&self) -> &Ibuf28 {
        &self.ibuf28
    }
    #[doc = "0xbd - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf29(&self) -> &Ibuf29 {
        &self.ibuf29
    }
    #[doc = "0xbe - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf30(&self) -> &Ibuf30 {
        &self.ibuf30
    }
    #[doc = "0xbf - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf31(&self) -> &Ibuf31 {
        &self.ibuf31
    }
    #[doc = "0xc0 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf32(&self) -> &Ibuf32 {
        &self.ibuf32
    }
    #[doc = "0xc1 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf33(&self) -> &Ibuf33 {
        &self.ibuf33
    }
    #[doc = "0xc2 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf34(&self) -> &Ibuf34 {
        &self.ibuf34
    }
    #[doc = "0xc3 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf35(&self) -> &Ibuf35 {
        &self.ibuf35
    }
    #[doc = "0xc4 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf36(&self) -> &Ibuf36 {
        &self.ibuf36
    }
    #[doc = "0xc5 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf37(&self) -> &Ibuf37 {
        &self.ibuf37
    }
    #[doc = "0xc6 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf38(&self) -> &Ibuf38 {
        &self.ibuf38
    }
    #[doc = "0xc7 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf39(&self) -> &Ibuf39 {
        &self.ibuf39
    }
    #[doc = "0xc8 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf40(&self) -> &Ibuf40 {
        &self.ibuf40
    }
    #[doc = "0xc9 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf41(&self) -> &Ibuf41 {
        &self.ibuf41
    }
    #[doc = "0xca - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf42(&self) -> &Ibuf42 {
        &self.ibuf42
    }
    #[doc = "0xcb - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf43(&self) -> &Ibuf43 {
        &self.ibuf43
    }
    #[doc = "0xcc - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf44(&self) -> &Ibuf44 {
        &self.ibuf44
    }
    #[doc = "0xcd - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf45(&self) -> &Ibuf45 {
        &self.ibuf45
    }
    #[doc = "0xce - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf46(&self) -> &Ibuf46 {
        &self.ibuf46
    }
    #[doc = "0xcf - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf47(&self) -> &Ibuf47 {
        &self.ibuf47
    }
    #[doc = "0xd0 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf48(&self) -> &Ibuf48 {
        &self.ibuf48
    }
    #[doc = "0xd1 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf49(&self) -> &Ibuf49 {
        &self.ibuf49
    }
    #[doc = "0xd2 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf50(&self) -> &Ibuf50 {
        &self.ibuf50
    }
    #[doc = "0xd3 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf51(&self) -> &Ibuf51 {
        &self.ibuf51
    }
    #[doc = "0xd4 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf52(&self) -> &Ibuf52 {
        &self.ibuf52
    }
    #[doc = "0xd5 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf53(&self) -> &Ibuf53 {
        &self.ibuf53
    }
    #[doc = "0xd6 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf54(&self) -> &Ibuf54 {
        &self.ibuf54
    }
    #[doc = "0xd7 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf55(&self) -> &Ibuf55 {
        &self.ibuf55
    }
    #[doc = "0xd8 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf56(&self) -> &Ibuf56 {
        &self.ibuf56
    }
    #[doc = "0xd9 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf57(&self) -> &Ibuf57 {
        &self.ibuf57
    }
    #[doc = "0xda - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf58(&self) -> &Ibuf58 {
        &self.ibuf58
    }
    #[doc = "0xdb - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf59(&self) -> &Ibuf59 {
        &self.ibuf59
    }
    #[doc = "0xdc - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf60(&self) -> &Ibuf60 {
        &self.ibuf60
    }
    #[doc = "0xdd - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf61(&self) -> &Ibuf61 {
        &self.ibuf61
    }
    #[doc = "0xde - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf62(&self) -> &Ibuf62 {
        &self.ibuf62
    }
    #[doc = "0xdf - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf63(&self) -> &Ibuf63 {
        &self.ibuf63
    }
    #[doc = "0xe0 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf64(&self) -> &Ibuf64 {
        &self.ibuf64
    }
    #[doc = "0xe1 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf65(&self) -> &Ibuf65 {
        &self.ibuf65
    }
    #[doc = "0xe2 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf66(&self) -> &Ibuf66 {
        &self.ibuf66
    }
    #[doc = "0xe3 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf67(&self) -> &Ibuf67 {
        &self.ibuf67
    }
    #[doc = "0xe4 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf68(&self) -> &Ibuf68 {
        &self.ibuf68
    }
    #[doc = "0xe5 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf69(&self) -> &Ibuf69 {
        &self.ibuf69
    }
    #[doc = "0xe6 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf70(&self) -> &Ibuf70 {
        &self.ibuf70
    }
    #[doc = "0xe7 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf71(&self) -> &Ibuf71 {
        &self.ibuf71
    }
    #[doc = "0xe8 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf72(&self) -> &Ibuf72 {
        &self.ibuf72
    }
    #[doc = "0xe9 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf73(&self) -> &Ibuf73 {
        &self.ibuf73
    }
    #[doc = "0xea - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf74(&self) -> &Ibuf74 {
        &self.ibuf74
    }
    #[doc = "0xeb - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf75(&self) -> &Ibuf75 {
        &self.ibuf75
    }
    #[doc = "0xec - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf76(&self) -> &Ibuf76 {
        &self.ibuf76
    }
    #[doc = "0xed - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf77(&self) -> &Ibuf77 {
        &self.ibuf77
    }
    #[doc = "0xee - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf78(&self) -> &Ibuf78 {
        &self.ibuf78
    }
    #[doc = "0xef - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf79(&self) -> &Ibuf79 {
        &self.ibuf79
    }
    #[doc = "0xf0 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf80(&self) -> &Ibuf80 {
        &self.ibuf80
    }
    #[doc = "0xf1 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf81(&self) -> &Ibuf81 {
        &self.ibuf81
    }
    #[doc = "0xf2 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf82(&self) -> &Ibuf82 {
        &self.ibuf82
    }
    #[doc = "0xf3 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf83(&self) -> &Ibuf83 {
        &self.ibuf83
    }
    #[doc = "0xf4 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf84(&self) -> &Ibuf84 {
        &self.ibuf84
    }
    #[doc = "0xf5 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf85(&self) -> &Ibuf85 {
        &self.ibuf85
    }
    #[doc = "0xf6 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf86(&self) -> &Ibuf86 {
        &self.ibuf86
    }
    #[doc = "0xf7 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf87(&self) -> &Ibuf87 {
        &self.ibuf87
    }
    #[doc = "0xf8 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf88(&self) -> &Ibuf88 {
        &self.ibuf88
    }
    #[doc = "0xf9 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf89(&self) -> &Ibuf89 {
        &self.ibuf89
    }
    #[doc = "0xfa - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf90(&self) -> &Ibuf90 {
        &self.ibuf90
    }
    #[doc = "0xfb - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf91(&self) -> &Ibuf91 {
        &self.ibuf91
    }
    #[doc = "0xfc - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf92(&self) -> &Ibuf92 {
        &self.ibuf92
    }
    #[doc = "0xfd - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf93(&self) -> &Ibuf93 {
        &self.ibuf93
    }
    #[doc = "0xfe - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf94(&self) -> &Ibuf94 {
        &self.ibuf94
    }
    #[doc = "0xff - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf95(&self) -> &Ibuf95 {
        &self.ibuf95
    }
    #[doc = "0x100 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf96(&self) -> &Ibuf96 {
        &self.ibuf96
    }
    #[doc = "0x101 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf97(&self) -> &Ibuf97 {
        &self.ibuf97
    }
    #[doc = "0x102 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf98(&self) -> &Ibuf98 {
        &self.ibuf98
    }
    #[doc = "0x103 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf99(&self) -> &Ibuf99 {
        &self.ibuf99
    }
    #[doc = "0x104 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf100(&self) -> &Ibuf100 {
        &self.ibuf100
    }
    #[doc = "0x105 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf101(&self) -> &Ibuf101 {
        &self.ibuf101
    }
    #[doc = "0x106 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf102(&self) -> &Ibuf102 {
        &self.ibuf102
    }
    #[doc = "0x107 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf103(&self) -> &Ibuf103 {
        &self.ibuf103
    }
    #[doc = "0x108 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf104(&self) -> &Ibuf104 {
        &self.ibuf104
    }
    #[doc = "0x109 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf105(&self) -> &Ibuf105 {
        &self.ibuf105
    }
    #[doc = "0x10a - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf106(&self) -> &Ibuf106 {
        &self.ibuf106
    }
    #[doc = "0x10b - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf107(&self) -> &Ibuf107 {
        &self.ibuf107
    }
    #[doc = "0x10c - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf108(&self) -> &Ibuf108 {
        &self.ibuf108
    }
    #[doc = "0x10d - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf109(&self) -> &Ibuf109 {
        &self.ibuf109
    }
    #[doc = "0x10e - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf110(&self) -> &Ibuf110 {
        &self.ibuf110
    }
    #[doc = "0x10f - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf111(&self) -> &Ibuf111 {
        &self.ibuf111
    }
    #[doc = "0x110 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf112(&self) -> &Ibuf112 {
        &self.ibuf112
    }
    #[doc = "0x111 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf113(&self) -> &Ibuf113 {
        &self.ibuf113
    }
    #[doc = "0x112 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf114(&self) -> &Ibuf114 {
        &self.ibuf114
    }
    #[doc = "0x113 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf115(&self) -> &Ibuf115 {
        &self.ibuf115
    }
    #[doc = "0x114 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf116(&self) -> &Ibuf116 {
        &self.ibuf116
    }
    #[doc = "0x115 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf117(&self) -> &Ibuf117 {
        &self.ibuf117
    }
    #[doc = "0x116 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf118(&self) -> &Ibuf118 {
        &self.ibuf118
    }
    #[doc = "0x117 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf119(&self) -> &Ibuf119 {
        &self.ibuf119
    }
    #[doc = "0x118 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf120(&self) -> &Ibuf120 {
        &self.ibuf120
    }
    #[doc = "0x119 - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf121(&self) -> &Ibuf121 {
        &self.ibuf121
    }
    #[doc = "0x11a - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf122(&self) -> &Ibuf122 {
        &self.ibuf122
    }
    #[doc = "0x11b - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf123(&self) -> &Ibuf123 {
        &self.ibuf123
    }
    #[doc = "0x11c - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf124(&self) -> &Ibuf124 {
        &self.ibuf124
    }
    #[doc = "0x11d - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf125(&self) -> &Ibuf125 {
        &self.ibuf125
    }
    #[doc = "0x11e - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf126(&self) -> &Ibuf126 {
        &self.ibuf126
    }
    #[doc = "0x11f - Input Buffer Register"]
    #[inline(always)]
    pub const fn ibuf127(&self) -> &Ibuf127 {
        &self.ibuf127
    }
}
#[doc = "SHICFG1 (rw) register accessor: SHI Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shicfg1`]
module"]
#[doc(alias = "SHICFG1")]
pub type Shicfg1 = crate::Reg<shicfg1::Shicfg1Spec>;
#[doc = "SHI Configuration 1"]
pub mod shicfg1;
#[doc = "SHICFG2 (rw) register accessor: SHI Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shicfg2`]
module"]
#[doc(alias = "SHICFG2")]
pub type Shicfg2 = crate::Reg<shicfg2::Shicfg2Spec>;
#[doc = "SHI Configuration 2"]
pub mod shicfg2;
#[doc = "EVENABLE (rw) register accessor: Event Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`evenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evenable`]
module"]
#[doc(alias = "EVENABLE")]
pub type Evenable = crate::Reg<evenable::EvenableSpec>;
#[doc = "Event Enable"]
pub mod evenable;
#[doc = "EVSTAT (rw) register accessor: Event Status\n\nYou can [`read`](crate::Reg::read) this register and get [`evstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat`]
module"]
#[doc(alias = "EVSTAT")]
pub type Evstat = crate::Reg<evstat::EvstatSpec>;
#[doc = "Event Status"]
pub mod evstat;
#[doc = "CAPABILITY (rw) register accessor: SHI Capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`capability::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capability::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capability`]
module"]
#[doc(alias = "CAPABILITY")]
pub type Capability = crate::Reg<capability::CapabilitySpec>;
#[doc = "SHI Capabilities"]
pub mod capability;
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "IBUFSTAT (rw) register accessor: Input Buffer Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ibufstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibufstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibufstat`]
module"]
#[doc(alias = "IBUFSTAT")]
pub type Ibufstat = crate::Reg<ibufstat::IbufstatSpec>;
#[doc = "Input Buffer Status"]
pub mod ibufstat;
#[doc = "OBUFSTAT (rw) register accessor: Output Buffer Status\n\nYou can [`read`](crate::Reg::read) this register and get [`obufstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obufstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obufstat`]
module"]
#[doc(alias = "OBUFSTAT")]
pub type Obufstat = crate::Reg<obufstat::ObufstatSpec>;
#[doc = "Output Buffer Status"]
pub mod obufstat;
#[doc = "SHICFG3 (rw) register accessor: SHI Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shicfg3`]
module"]
#[doc(alias = "SHICFG3")]
pub type Shicfg3 = crate::Reg<shicfg3::Shicfg3Spec>;
#[doc = "SHI Configuration 3"]
pub mod shicfg3;
#[doc = "SHICFG4 (rw) register accessor: SHI Configuration 4\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shicfg4`]
module"]
#[doc(alias = "SHICFG4")]
pub type Shicfg4 = crate::Reg<shicfg4::Shicfg4Spec>;
#[doc = "SHI Configuration 4"]
pub mod shicfg4;
#[doc = "SHICFG5 (rw) register accessor: SHI Configuration 5\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shicfg5`]
module"]
#[doc(alias = "SHICFG5")]
pub type Shicfg5 = crate::Reg<shicfg5::Shicfg5Spec>;
#[doc = "SHI Configuration 5"]
pub mod shicfg5;
#[doc = "EVSTAT2 (rw) register accessor: Event Status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`evstat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evstat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat2`]
module"]
#[doc(alias = "EVSTAT2")]
pub type Evstat2 = crate::Reg<evstat2::Evstat2Spec>;
#[doc = "Event Status 2"]
pub mod evstat2;
#[doc = "EVENABLE2 (rw) register accessor: Event Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`evenable2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evenable2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evenable2`]
module"]
#[doc(alias = "EVENABLE2")]
pub type Evenable2 = crate::Reg<evenable2::Evenable2Spec>;
#[doc = "Event Enable 2"]
pub mod evenable2;
#[doc = "SHICFG6 (rw) register accessor: SHI Configuration 6\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shicfg6`]
module"]
#[doc(alias = "SHICFG6")]
pub type Shicfg6 = crate::Reg<shicfg6::Shicfg6Spec>;
#[doc = "SHI Configuration 6"]
pub mod shicfg6;
#[doc = "SBOBUF (rw) register accessor: Single Byte Output Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`sbobuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbobuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbobuf`]
module"]
#[doc(alias = "SBOBUF")]
pub type Sbobuf = crate::Reg<sbobuf::SbobufSpec>;
#[doc = "Single Byte Output Buffer"]
pub mod sbobuf;
#[doc = "OBUF0 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf0`]
module"]
#[doc(alias = "OBUF0")]
pub type Obuf0 = crate::Reg<obuf0::Obuf0Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf0;
#[doc = "OBUF1 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf1`]
module"]
#[doc(alias = "OBUF1")]
pub type Obuf1 = crate::Reg<obuf1::Obuf1Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf1;
#[doc = "OBUF2 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf2`]
module"]
#[doc(alias = "OBUF2")]
pub type Obuf2 = crate::Reg<obuf2::Obuf2Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf2;
#[doc = "OBUF3 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf3`]
module"]
#[doc(alias = "OBUF3")]
pub type Obuf3 = crate::Reg<obuf3::Obuf3Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf3;
#[doc = "OBUF4 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf4`]
module"]
#[doc(alias = "OBUF4")]
pub type Obuf4 = crate::Reg<obuf4::Obuf4Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf4;
#[doc = "OBUF5 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf5`]
module"]
#[doc(alias = "OBUF5")]
pub type Obuf5 = crate::Reg<obuf5::Obuf5Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf5;
#[doc = "OBUF6 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf6`]
module"]
#[doc(alias = "OBUF6")]
pub type Obuf6 = crate::Reg<obuf6::Obuf6Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf6;
#[doc = "OBUF7 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf7`]
module"]
#[doc(alias = "OBUF7")]
pub type Obuf7 = crate::Reg<obuf7::Obuf7Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf7;
#[doc = "OBUF8 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf8`]
module"]
#[doc(alias = "OBUF8")]
pub type Obuf8 = crate::Reg<obuf8::Obuf8Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf8;
#[doc = "OBUF9 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf9`]
module"]
#[doc(alias = "OBUF9")]
pub type Obuf9 = crate::Reg<obuf9::Obuf9Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf9;
#[doc = "OBUF10 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf10`]
module"]
#[doc(alias = "OBUF10")]
pub type Obuf10 = crate::Reg<obuf10::Obuf10Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf10;
#[doc = "OBUF11 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf11`]
module"]
#[doc(alias = "OBUF11")]
pub type Obuf11 = crate::Reg<obuf11::Obuf11Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf11;
#[doc = "OBUF12 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf12`]
module"]
#[doc(alias = "OBUF12")]
pub type Obuf12 = crate::Reg<obuf12::Obuf12Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf12;
#[doc = "OBUF13 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf13`]
module"]
#[doc(alias = "OBUF13")]
pub type Obuf13 = crate::Reg<obuf13::Obuf13Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf13;
#[doc = "OBUF14 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf14`]
module"]
#[doc(alias = "OBUF14")]
pub type Obuf14 = crate::Reg<obuf14::Obuf14Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf14;
#[doc = "OBUF15 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf15`]
module"]
#[doc(alias = "OBUF15")]
pub type Obuf15 = crate::Reg<obuf15::Obuf15Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf15;
#[doc = "OBUF16 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf16`]
module"]
#[doc(alias = "OBUF16")]
pub type Obuf16 = crate::Reg<obuf16::Obuf16Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf16;
#[doc = "OBUF17 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf17`]
module"]
#[doc(alias = "OBUF17")]
pub type Obuf17 = crate::Reg<obuf17::Obuf17Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf17;
#[doc = "OBUF18 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf18`]
module"]
#[doc(alias = "OBUF18")]
pub type Obuf18 = crate::Reg<obuf18::Obuf18Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf18;
#[doc = "OBUF19 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf19`]
module"]
#[doc(alias = "OBUF19")]
pub type Obuf19 = crate::Reg<obuf19::Obuf19Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf19;
#[doc = "OBUF20 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf20`]
module"]
#[doc(alias = "OBUF20")]
pub type Obuf20 = crate::Reg<obuf20::Obuf20Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf20;
#[doc = "OBUF21 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf21`]
module"]
#[doc(alias = "OBUF21")]
pub type Obuf21 = crate::Reg<obuf21::Obuf21Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf21;
#[doc = "OBUF22 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf22`]
module"]
#[doc(alias = "OBUF22")]
pub type Obuf22 = crate::Reg<obuf22::Obuf22Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf22;
#[doc = "OBUF23 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf23`]
module"]
#[doc(alias = "OBUF23")]
pub type Obuf23 = crate::Reg<obuf23::Obuf23Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf23;
#[doc = "OBUF24 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf24`]
module"]
#[doc(alias = "OBUF24")]
pub type Obuf24 = crate::Reg<obuf24::Obuf24Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf24;
#[doc = "OBUF25 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf25`]
module"]
#[doc(alias = "OBUF25")]
pub type Obuf25 = crate::Reg<obuf25::Obuf25Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf25;
#[doc = "OBUF26 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf26`]
module"]
#[doc(alias = "OBUF26")]
pub type Obuf26 = crate::Reg<obuf26::Obuf26Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf26;
#[doc = "OBUF27 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf27`]
module"]
#[doc(alias = "OBUF27")]
pub type Obuf27 = crate::Reg<obuf27::Obuf27Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf27;
#[doc = "OBUF28 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf28`]
module"]
#[doc(alias = "OBUF28")]
pub type Obuf28 = crate::Reg<obuf28::Obuf28Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf28;
#[doc = "OBUF29 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf29`]
module"]
#[doc(alias = "OBUF29")]
pub type Obuf29 = crate::Reg<obuf29::Obuf29Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf29;
#[doc = "OBUF30 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf30`]
module"]
#[doc(alias = "OBUF30")]
pub type Obuf30 = crate::Reg<obuf30::Obuf30Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf30;
#[doc = "OBUF31 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf31`]
module"]
#[doc(alias = "OBUF31")]
pub type Obuf31 = crate::Reg<obuf31::Obuf31Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf31;
#[doc = "OBUF32 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf32`]
module"]
#[doc(alias = "OBUF32")]
pub type Obuf32 = crate::Reg<obuf32::Obuf32Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf32;
#[doc = "OBUF33 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf33`]
module"]
#[doc(alias = "OBUF33")]
pub type Obuf33 = crate::Reg<obuf33::Obuf33Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf33;
#[doc = "OBUF34 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf34`]
module"]
#[doc(alias = "OBUF34")]
pub type Obuf34 = crate::Reg<obuf34::Obuf34Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf34;
#[doc = "OBUF35 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf35`]
module"]
#[doc(alias = "OBUF35")]
pub type Obuf35 = crate::Reg<obuf35::Obuf35Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf35;
#[doc = "OBUF36 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf36`]
module"]
#[doc(alias = "OBUF36")]
pub type Obuf36 = crate::Reg<obuf36::Obuf36Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf36;
#[doc = "OBUF37 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf37`]
module"]
#[doc(alias = "OBUF37")]
pub type Obuf37 = crate::Reg<obuf37::Obuf37Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf37;
#[doc = "OBUF38 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf38`]
module"]
#[doc(alias = "OBUF38")]
pub type Obuf38 = crate::Reg<obuf38::Obuf38Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf38;
#[doc = "OBUF39 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf39`]
module"]
#[doc(alias = "OBUF39")]
pub type Obuf39 = crate::Reg<obuf39::Obuf39Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf39;
#[doc = "OBUF40 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf40`]
module"]
#[doc(alias = "OBUF40")]
pub type Obuf40 = crate::Reg<obuf40::Obuf40Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf40;
#[doc = "OBUF41 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf41`]
module"]
#[doc(alias = "OBUF41")]
pub type Obuf41 = crate::Reg<obuf41::Obuf41Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf41;
#[doc = "OBUF42 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf42`]
module"]
#[doc(alias = "OBUF42")]
pub type Obuf42 = crate::Reg<obuf42::Obuf42Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf42;
#[doc = "OBUF43 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf43`]
module"]
#[doc(alias = "OBUF43")]
pub type Obuf43 = crate::Reg<obuf43::Obuf43Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf43;
#[doc = "OBUF44 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf44`]
module"]
#[doc(alias = "OBUF44")]
pub type Obuf44 = crate::Reg<obuf44::Obuf44Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf44;
#[doc = "OBUF45 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf45`]
module"]
#[doc(alias = "OBUF45")]
pub type Obuf45 = crate::Reg<obuf45::Obuf45Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf45;
#[doc = "OBUF46 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf46`]
module"]
#[doc(alias = "OBUF46")]
pub type Obuf46 = crate::Reg<obuf46::Obuf46Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf46;
#[doc = "OBUF47 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf47`]
module"]
#[doc(alias = "OBUF47")]
pub type Obuf47 = crate::Reg<obuf47::Obuf47Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf47;
#[doc = "OBUF48 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf48`]
module"]
#[doc(alias = "OBUF48")]
pub type Obuf48 = crate::Reg<obuf48::Obuf48Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf48;
#[doc = "OBUF49 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf49`]
module"]
#[doc(alias = "OBUF49")]
pub type Obuf49 = crate::Reg<obuf49::Obuf49Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf49;
#[doc = "OBUF50 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf50`]
module"]
#[doc(alias = "OBUF50")]
pub type Obuf50 = crate::Reg<obuf50::Obuf50Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf50;
#[doc = "OBUF51 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf51`]
module"]
#[doc(alias = "OBUF51")]
pub type Obuf51 = crate::Reg<obuf51::Obuf51Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf51;
#[doc = "OBUF52 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf52`]
module"]
#[doc(alias = "OBUF52")]
pub type Obuf52 = crate::Reg<obuf52::Obuf52Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf52;
#[doc = "OBUF53 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf53`]
module"]
#[doc(alias = "OBUF53")]
pub type Obuf53 = crate::Reg<obuf53::Obuf53Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf53;
#[doc = "OBUF54 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf54`]
module"]
#[doc(alias = "OBUF54")]
pub type Obuf54 = crate::Reg<obuf54::Obuf54Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf54;
#[doc = "OBUF55 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf55`]
module"]
#[doc(alias = "OBUF55")]
pub type Obuf55 = crate::Reg<obuf55::Obuf55Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf55;
#[doc = "OBUF56 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf56`]
module"]
#[doc(alias = "OBUF56")]
pub type Obuf56 = crate::Reg<obuf56::Obuf56Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf56;
#[doc = "OBUF57 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf57`]
module"]
#[doc(alias = "OBUF57")]
pub type Obuf57 = crate::Reg<obuf57::Obuf57Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf57;
#[doc = "OBUF58 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf58`]
module"]
#[doc(alias = "OBUF58")]
pub type Obuf58 = crate::Reg<obuf58::Obuf58Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf58;
#[doc = "OBUF59 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf59`]
module"]
#[doc(alias = "OBUF59")]
pub type Obuf59 = crate::Reg<obuf59::Obuf59Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf59;
#[doc = "OBUF60 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf60`]
module"]
#[doc(alias = "OBUF60")]
pub type Obuf60 = crate::Reg<obuf60::Obuf60Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf60;
#[doc = "OBUF61 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf61`]
module"]
#[doc(alias = "OBUF61")]
pub type Obuf61 = crate::Reg<obuf61::Obuf61Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf61;
#[doc = "OBUF62 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf62`]
module"]
#[doc(alias = "OBUF62")]
pub type Obuf62 = crate::Reg<obuf62::Obuf62Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf62;
#[doc = "OBUF63 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf63`]
module"]
#[doc(alias = "OBUF63")]
pub type Obuf63 = crate::Reg<obuf63::Obuf63Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf63;
#[doc = "OBUF64 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf64`]
module"]
#[doc(alias = "OBUF64")]
pub type Obuf64 = crate::Reg<obuf64::Obuf64Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf64;
#[doc = "OBUF65 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf65`]
module"]
#[doc(alias = "OBUF65")]
pub type Obuf65 = crate::Reg<obuf65::Obuf65Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf65;
#[doc = "OBUF66 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf66`]
module"]
#[doc(alias = "OBUF66")]
pub type Obuf66 = crate::Reg<obuf66::Obuf66Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf66;
#[doc = "OBUF67 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf67`]
module"]
#[doc(alias = "OBUF67")]
pub type Obuf67 = crate::Reg<obuf67::Obuf67Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf67;
#[doc = "OBUF68 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf68`]
module"]
#[doc(alias = "OBUF68")]
pub type Obuf68 = crate::Reg<obuf68::Obuf68Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf68;
#[doc = "OBUF69 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf69`]
module"]
#[doc(alias = "OBUF69")]
pub type Obuf69 = crate::Reg<obuf69::Obuf69Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf69;
#[doc = "OBUF70 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf70`]
module"]
#[doc(alias = "OBUF70")]
pub type Obuf70 = crate::Reg<obuf70::Obuf70Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf70;
#[doc = "OBUF71 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf71`]
module"]
#[doc(alias = "OBUF71")]
pub type Obuf71 = crate::Reg<obuf71::Obuf71Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf71;
#[doc = "OBUF72 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf72`]
module"]
#[doc(alias = "OBUF72")]
pub type Obuf72 = crate::Reg<obuf72::Obuf72Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf72;
#[doc = "OBUF73 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf73`]
module"]
#[doc(alias = "OBUF73")]
pub type Obuf73 = crate::Reg<obuf73::Obuf73Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf73;
#[doc = "OBUF74 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf74`]
module"]
#[doc(alias = "OBUF74")]
pub type Obuf74 = crate::Reg<obuf74::Obuf74Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf74;
#[doc = "OBUF75 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf75`]
module"]
#[doc(alias = "OBUF75")]
pub type Obuf75 = crate::Reg<obuf75::Obuf75Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf75;
#[doc = "OBUF76 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf76`]
module"]
#[doc(alias = "OBUF76")]
pub type Obuf76 = crate::Reg<obuf76::Obuf76Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf76;
#[doc = "OBUF77 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf77`]
module"]
#[doc(alias = "OBUF77")]
pub type Obuf77 = crate::Reg<obuf77::Obuf77Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf77;
#[doc = "OBUF78 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf78`]
module"]
#[doc(alias = "OBUF78")]
pub type Obuf78 = crate::Reg<obuf78::Obuf78Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf78;
#[doc = "OBUF79 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf79`]
module"]
#[doc(alias = "OBUF79")]
pub type Obuf79 = crate::Reg<obuf79::Obuf79Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf79;
#[doc = "OBUF80 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf80`]
module"]
#[doc(alias = "OBUF80")]
pub type Obuf80 = crate::Reg<obuf80::Obuf80Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf80;
#[doc = "OBUF81 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf81`]
module"]
#[doc(alias = "OBUF81")]
pub type Obuf81 = crate::Reg<obuf81::Obuf81Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf81;
#[doc = "OBUF82 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf82`]
module"]
#[doc(alias = "OBUF82")]
pub type Obuf82 = crate::Reg<obuf82::Obuf82Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf82;
#[doc = "OBUF83 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf83`]
module"]
#[doc(alias = "OBUF83")]
pub type Obuf83 = crate::Reg<obuf83::Obuf83Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf83;
#[doc = "OBUF84 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf84`]
module"]
#[doc(alias = "OBUF84")]
pub type Obuf84 = crate::Reg<obuf84::Obuf84Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf84;
#[doc = "OBUF85 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf85`]
module"]
#[doc(alias = "OBUF85")]
pub type Obuf85 = crate::Reg<obuf85::Obuf85Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf85;
#[doc = "OBUF86 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf86`]
module"]
#[doc(alias = "OBUF86")]
pub type Obuf86 = crate::Reg<obuf86::Obuf86Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf86;
#[doc = "OBUF87 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf87`]
module"]
#[doc(alias = "OBUF87")]
pub type Obuf87 = crate::Reg<obuf87::Obuf87Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf87;
#[doc = "OBUF88 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf88`]
module"]
#[doc(alias = "OBUF88")]
pub type Obuf88 = crate::Reg<obuf88::Obuf88Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf88;
#[doc = "OBUF89 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf89`]
module"]
#[doc(alias = "OBUF89")]
pub type Obuf89 = crate::Reg<obuf89::Obuf89Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf89;
#[doc = "OBUF90 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf90`]
module"]
#[doc(alias = "OBUF90")]
pub type Obuf90 = crate::Reg<obuf90::Obuf90Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf90;
#[doc = "OBUF91 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf91`]
module"]
#[doc(alias = "OBUF91")]
pub type Obuf91 = crate::Reg<obuf91::Obuf91Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf91;
#[doc = "OBUF92 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf92`]
module"]
#[doc(alias = "OBUF92")]
pub type Obuf92 = crate::Reg<obuf92::Obuf92Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf92;
#[doc = "OBUF93 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf93`]
module"]
#[doc(alias = "OBUF93")]
pub type Obuf93 = crate::Reg<obuf93::Obuf93Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf93;
#[doc = "OBUF94 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf94`]
module"]
#[doc(alias = "OBUF94")]
pub type Obuf94 = crate::Reg<obuf94::Obuf94Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf94;
#[doc = "OBUF95 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf95`]
module"]
#[doc(alias = "OBUF95")]
pub type Obuf95 = crate::Reg<obuf95::Obuf95Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf95;
#[doc = "OBUF96 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf96`]
module"]
#[doc(alias = "OBUF96")]
pub type Obuf96 = crate::Reg<obuf96::Obuf96Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf96;
#[doc = "OBUF97 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf97`]
module"]
#[doc(alias = "OBUF97")]
pub type Obuf97 = crate::Reg<obuf97::Obuf97Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf97;
#[doc = "OBUF98 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf98`]
module"]
#[doc(alias = "OBUF98")]
pub type Obuf98 = crate::Reg<obuf98::Obuf98Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf98;
#[doc = "OBUF99 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf99`]
module"]
#[doc(alias = "OBUF99")]
pub type Obuf99 = crate::Reg<obuf99::Obuf99Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf99;
#[doc = "OBUF100 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf100`]
module"]
#[doc(alias = "OBUF100")]
pub type Obuf100 = crate::Reg<obuf100::Obuf100Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf100;
#[doc = "OBUF101 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf101::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf101`]
module"]
#[doc(alias = "OBUF101")]
pub type Obuf101 = crate::Reg<obuf101::Obuf101Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf101;
#[doc = "OBUF102 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf102::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf102`]
module"]
#[doc(alias = "OBUF102")]
pub type Obuf102 = crate::Reg<obuf102::Obuf102Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf102;
#[doc = "OBUF103 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf103::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf103::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf103`]
module"]
#[doc(alias = "OBUF103")]
pub type Obuf103 = crate::Reg<obuf103::Obuf103Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf103;
#[doc = "OBUF104 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf104`]
module"]
#[doc(alias = "OBUF104")]
pub type Obuf104 = crate::Reg<obuf104::Obuf104Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf104;
#[doc = "OBUF105 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf105::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf105::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf105`]
module"]
#[doc(alias = "OBUF105")]
pub type Obuf105 = crate::Reg<obuf105::Obuf105Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf105;
#[doc = "OBUF106 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf106::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf106::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf106`]
module"]
#[doc(alias = "OBUF106")]
pub type Obuf106 = crate::Reg<obuf106::Obuf106Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf106;
#[doc = "OBUF107 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf107::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf107::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf107`]
module"]
#[doc(alias = "OBUF107")]
pub type Obuf107 = crate::Reg<obuf107::Obuf107Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf107;
#[doc = "OBUF108 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf108`]
module"]
#[doc(alias = "OBUF108")]
pub type Obuf108 = crate::Reg<obuf108::Obuf108Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf108;
#[doc = "OBUF109 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf109::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf109::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf109`]
module"]
#[doc(alias = "OBUF109")]
pub type Obuf109 = crate::Reg<obuf109::Obuf109Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf109;
#[doc = "OBUF110 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf110`]
module"]
#[doc(alias = "OBUF110")]
pub type Obuf110 = crate::Reg<obuf110::Obuf110Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf110;
#[doc = "OBUF111 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf111`]
module"]
#[doc(alias = "OBUF111")]
pub type Obuf111 = crate::Reg<obuf111::Obuf111Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf111;
#[doc = "OBUF112 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf112`]
module"]
#[doc(alias = "OBUF112")]
pub type Obuf112 = crate::Reg<obuf112::Obuf112Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf112;
#[doc = "OBUF113 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf113`]
module"]
#[doc(alias = "OBUF113")]
pub type Obuf113 = crate::Reg<obuf113::Obuf113Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf113;
#[doc = "OBUF114 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf114`]
module"]
#[doc(alias = "OBUF114")]
pub type Obuf114 = crate::Reg<obuf114::Obuf114Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf114;
#[doc = "OBUF115 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf115`]
module"]
#[doc(alias = "OBUF115")]
pub type Obuf115 = crate::Reg<obuf115::Obuf115Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf115;
#[doc = "OBUF116 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf116`]
module"]
#[doc(alias = "OBUF116")]
pub type Obuf116 = crate::Reg<obuf116::Obuf116Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf116;
#[doc = "OBUF117 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf117`]
module"]
#[doc(alias = "OBUF117")]
pub type Obuf117 = crate::Reg<obuf117::Obuf117Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf117;
#[doc = "OBUF118 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf118`]
module"]
#[doc(alias = "OBUF118")]
pub type Obuf118 = crate::Reg<obuf118::Obuf118Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf118;
#[doc = "OBUF119 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf119`]
module"]
#[doc(alias = "OBUF119")]
pub type Obuf119 = crate::Reg<obuf119::Obuf119Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf119;
#[doc = "OBUF120 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf120`]
module"]
#[doc(alias = "OBUF120")]
pub type Obuf120 = crate::Reg<obuf120::Obuf120Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf120;
#[doc = "OBUF121 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf121`]
module"]
#[doc(alias = "OBUF121")]
pub type Obuf121 = crate::Reg<obuf121::Obuf121Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf121;
#[doc = "OBUF122 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf122`]
module"]
#[doc(alias = "OBUF122")]
pub type Obuf122 = crate::Reg<obuf122::Obuf122Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf122;
#[doc = "OBUF123 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf123`]
module"]
#[doc(alias = "OBUF123")]
pub type Obuf123 = crate::Reg<obuf123::Obuf123Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf123;
#[doc = "OBUF124 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf124`]
module"]
#[doc(alias = "OBUF124")]
pub type Obuf124 = crate::Reg<obuf124::Obuf124Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf124;
#[doc = "OBUF125 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf125`]
module"]
#[doc(alias = "OBUF125")]
pub type Obuf125 = crate::Reg<obuf125::Obuf125Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf125;
#[doc = "OBUF126 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf126`]
module"]
#[doc(alias = "OBUF126")]
pub type Obuf126 = crate::Reg<obuf126::Obuf126Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf126;
#[doc = "OBUF127 (rw) register accessor: Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obuf127`]
module"]
#[doc(alias = "OBUF127")]
pub type Obuf127 = crate::Reg<obuf127::Obuf127Spec>;
#[doc = "Output Buffer Register"]
pub mod obuf127;
#[doc = "IBUF0 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf0`]
module"]
#[doc(alias = "IBUF0")]
pub type Ibuf0 = crate::Reg<ibuf0::Ibuf0Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf0;
#[doc = "IBUF1 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf1`]
module"]
#[doc(alias = "IBUF1")]
pub type Ibuf1 = crate::Reg<ibuf1::Ibuf1Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf1;
#[doc = "IBUF2 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf2`]
module"]
#[doc(alias = "IBUF2")]
pub type Ibuf2 = crate::Reg<ibuf2::Ibuf2Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf2;
#[doc = "IBUF3 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf3`]
module"]
#[doc(alias = "IBUF3")]
pub type Ibuf3 = crate::Reg<ibuf3::Ibuf3Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf3;
#[doc = "IBUF4 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf4`]
module"]
#[doc(alias = "IBUF4")]
pub type Ibuf4 = crate::Reg<ibuf4::Ibuf4Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf4;
#[doc = "IBUF5 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf5`]
module"]
#[doc(alias = "IBUF5")]
pub type Ibuf5 = crate::Reg<ibuf5::Ibuf5Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf5;
#[doc = "IBUF6 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf6`]
module"]
#[doc(alias = "IBUF6")]
pub type Ibuf6 = crate::Reg<ibuf6::Ibuf6Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf6;
#[doc = "IBUF7 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf7`]
module"]
#[doc(alias = "IBUF7")]
pub type Ibuf7 = crate::Reg<ibuf7::Ibuf7Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf7;
#[doc = "IBUF8 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf8`]
module"]
#[doc(alias = "IBUF8")]
pub type Ibuf8 = crate::Reg<ibuf8::Ibuf8Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf8;
#[doc = "IBUF9 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf9`]
module"]
#[doc(alias = "IBUF9")]
pub type Ibuf9 = crate::Reg<ibuf9::Ibuf9Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf9;
#[doc = "IBUF10 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf10`]
module"]
#[doc(alias = "IBUF10")]
pub type Ibuf10 = crate::Reg<ibuf10::Ibuf10Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf10;
#[doc = "IBUF11 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf11`]
module"]
#[doc(alias = "IBUF11")]
pub type Ibuf11 = crate::Reg<ibuf11::Ibuf11Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf11;
#[doc = "IBUF12 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf12`]
module"]
#[doc(alias = "IBUF12")]
pub type Ibuf12 = crate::Reg<ibuf12::Ibuf12Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf12;
#[doc = "IBUF13 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf13`]
module"]
#[doc(alias = "IBUF13")]
pub type Ibuf13 = crate::Reg<ibuf13::Ibuf13Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf13;
#[doc = "IBUF14 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf14`]
module"]
#[doc(alias = "IBUF14")]
pub type Ibuf14 = crate::Reg<ibuf14::Ibuf14Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf14;
#[doc = "IBUF15 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf15`]
module"]
#[doc(alias = "IBUF15")]
pub type Ibuf15 = crate::Reg<ibuf15::Ibuf15Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf15;
#[doc = "IBUF16 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf16`]
module"]
#[doc(alias = "IBUF16")]
pub type Ibuf16 = crate::Reg<ibuf16::Ibuf16Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf16;
#[doc = "IBUF17 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf17`]
module"]
#[doc(alias = "IBUF17")]
pub type Ibuf17 = crate::Reg<ibuf17::Ibuf17Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf17;
#[doc = "IBUF18 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf18`]
module"]
#[doc(alias = "IBUF18")]
pub type Ibuf18 = crate::Reg<ibuf18::Ibuf18Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf18;
#[doc = "IBUF19 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf19`]
module"]
#[doc(alias = "IBUF19")]
pub type Ibuf19 = crate::Reg<ibuf19::Ibuf19Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf19;
#[doc = "IBUF20 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf20`]
module"]
#[doc(alias = "IBUF20")]
pub type Ibuf20 = crate::Reg<ibuf20::Ibuf20Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf20;
#[doc = "IBUF21 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf21`]
module"]
#[doc(alias = "IBUF21")]
pub type Ibuf21 = crate::Reg<ibuf21::Ibuf21Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf21;
#[doc = "IBUF22 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf22`]
module"]
#[doc(alias = "IBUF22")]
pub type Ibuf22 = crate::Reg<ibuf22::Ibuf22Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf22;
#[doc = "IBUF23 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf23`]
module"]
#[doc(alias = "IBUF23")]
pub type Ibuf23 = crate::Reg<ibuf23::Ibuf23Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf23;
#[doc = "IBUF24 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf24`]
module"]
#[doc(alias = "IBUF24")]
pub type Ibuf24 = crate::Reg<ibuf24::Ibuf24Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf24;
#[doc = "IBUF25 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf25`]
module"]
#[doc(alias = "IBUF25")]
pub type Ibuf25 = crate::Reg<ibuf25::Ibuf25Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf25;
#[doc = "IBUF26 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf26`]
module"]
#[doc(alias = "IBUF26")]
pub type Ibuf26 = crate::Reg<ibuf26::Ibuf26Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf26;
#[doc = "IBUF27 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf27`]
module"]
#[doc(alias = "IBUF27")]
pub type Ibuf27 = crate::Reg<ibuf27::Ibuf27Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf27;
#[doc = "IBUF28 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf28`]
module"]
#[doc(alias = "IBUF28")]
pub type Ibuf28 = crate::Reg<ibuf28::Ibuf28Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf28;
#[doc = "IBUF29 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf29`]
module"]
#[doc(alias = "IBUF29")]
pub type Ibuf29 = crate::Reg<ibuf29::Ibuf29Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf29;
#[doc = "IBUF30 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf30`]
module"]
#[doc(alias = "IBUF30")]
pub type Ibuf30 = crate::Reg<ibuf30::Ibuf30Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf30;
#[doc = "IBUF31 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf31`]
module"]
#[doc(alias = "IBUF31")]
pub type Ibuf31 = crate::Reg<ibuf31::Ibuf31Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf31;
#[doc = "IBUF32 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf32`]
module"]
#[doc(alias = "IBUF32")]
pub type Ibuf32 = crate::Reg<ibuf32::Ibuf32Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf32;
#[doc = "IBUF33 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf33`]
module"]
#[doc(alias = "IBUF33")]
pub type Ibuf33 = crate::Reg<ibuf33::Ibuf33Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf33;
#[doc = "IBUF34 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf34`]
module"]
#[doc(alias = "IBUF34")]
pub type Ibuf34 = crate::Reg<ibuf34::Ibuf34Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf34;
#[doc = "IBUF35 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf35`]
module"]
#[doc(alias = "IBUF35")]
pub type Ibuf35 = crate::Reg<ibuf35::Ibuf35Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf35;
#[doc = "IBUF36 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf36`]
module"]
#[doc(alias = "IBUF36")]
pub type Ibuf36 = crate::Reg<ibuf36::Ibuf36Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf36;
#[doc = "IBUF37 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf37`]
module"]
#[doc(alias = "IBUF37")]
pub type Ibuf37 = crate::Reg<ibuf37::Ibuf37Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf37;
#[doc = "IBUF38 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf38`]
module"]
#[doc(alias = "IBUF38")]
pub type Ibuf38 = crate::Reg<ibuf38::Ibuf38Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf38;
#[doc = "IBUF39 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf39`]
module"]
#[doc(alias = "IBUF39")]
pub type Ibuf39 = crate::Reg<ibuf39::Ibuf39Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf39;
#[doc = "IBUF40 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf40`]
module"]
#[doc(alias = "IBUF40")]
pub type Ibuf40 = crate::Reg<ibuf40::Ibuf40Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf40;
#[doc = "IBUF41 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf41`]
module"]
#[doc(alias = "IBUF41")]
pub type Ibuf41 = crate::Reg<ibuf41::Ibuf41Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf41;
#[doc = "IBUF42 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf42`]
module"]
#[doc(alias = "IBUF42")]
pub type Ibuf42 = crate::Reg<ibuf42::Ibuf42Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf42;
#[doc = "IBUF43 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf43`]
module"]
#[doc(alias = "IBUF43")]
pub type Ibuf43 = crate::Reg<ibuf43::Ibuf43Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf43;
#[doc = "IBUF44 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf44`]
module"]
#[doc(alias = "IBUF44")]
pub type Ibuf44 = crate::Reg<ibuf44::Ibuf44Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf44;
#[doc = "IBUF45 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf45`]
module"]
#[doc(alias = "IBUF45")]
pub type Ibuf45 = crate::Reg<ibuf45::Ibuf45Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf45;
#[doc = "IBUF46 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf46`]
module"]
#[doc(alias = "IBUF46")]
pub type Ibuf46 = crate::Reg<ibuf46::Ibuf46Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf46;
#[doc = "IBUF47 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf47`]
module"]
#[doc(alias = "IBUF47")]
pub type Ibuf47 = crate::Reg<ibuf47::Ibuf47Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf47;
#[doc = "IBUF48 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf48`]
module"]
#[doc(alias = "IBUF48")]
pub type Ibuf48 = crate::Reg<ibuf48::Ibuf48Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf48;
#[doc = "IBUF49 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf49`]
module"]
#[doc(alias = "IBUF49")]
pub type Ibuf49 = crate::Reg<ibuf49::Ibuf49Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf49;
#[doc = "IBUF50 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf50`]
module"]
#[doc(alias = "IBUF50")]
pub type Ibuf50 = crate::Reg<ibuf50::Ibuf50Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf50;
#[doc = "IBUF51 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf51`]
module"]
#[doc(alias = "IBUF51")]
pub type Ibuf51 = crate::Reg<ibuf51::Ibuf51Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf51;
#[doc = "IBUF52 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf52`]
module"]
#[doc(alias = "IBUF52")]
pub type Ibuf52 = crate::Reg<ibuf52::Ibuf52Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf52;
#[doc = "IBUF53 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf53`]
module"]
#[doc(alias = "IBUF53")]
pub type Ibuf53 = crate::Reg<ibuf53::Ibuf53Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf53;
#[doc = "IBUF54 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf54`]
module"]
#[doc(alias = "IBUF54")]
pub type Ibuf54 = crate::Reg<ibuf54::Ibuf54Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf54;
#[doc = "IBUF55 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf55`]
module"]
#[doc(alias = "IBUF55")]
pub type Ibuf55 = crate::Reg<ibuf55::Ibuf55Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf55;
#[doc = "IBUF56 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf56`]
module"]
#[doc(alias = "IBUF56")]
pub type Ibuf56 = crate::Reg<ibuf56::Ibuf56Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf56;
#[doc = "IBUF57 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf57`]
module"]
#[doc(alias = "IBUF57")]
pub type Ibuf57 = crate::Reg<ibuf57::Ibuf57Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf57;
#[doc = "IBUF58 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf58`]
module"]
#[doc(alias = "IBUF58")]
pub type Ibuf58 = crate::Reg<ibuf58::Ibuf58Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf58;
#[doc = "IBUF59 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf59`]
module"]
#[doc(alias = "IBUF59")]
pub type Ibuf59 = crate::Reg<ibuf59::Ibuf59Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf59;
#[doc = "IBUF60 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf60`]
module"]
#[doc(alias = "IBUF60")]
pub type Ibuf60 = crate::Reg<ibuf60::Ibuf60Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf60;
#[doc = "IBUF61 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf61`]
module"]
#[doc(alias = "IBUF61")]
pub type Ibuf61 = crate::Reg<ibuf61::Ibuf61Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf61;
#[doc = "IBUF62 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf62`]
module"]
#[doc(alias = "IBUF62")]
pub type Ibuf62 = crate::Reg<ibuf62::Ibuf62Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf62;
#[doc = "IBUF63 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf63`]
module"]
#[doc(alias = "IBUF63")]
pub type Ibuf63 = crate::Reg<ibuf63::Ibuf63Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf63;
#[doc = "IBUF64 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf64`]
module"]
#[doc(alias = "IBUF64")]
pub type Ibuf64 = crate::Reg<ibuf64::Ibuf64Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf64;
#[doc = "IBUF65 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf65`]
module"]
#[doc(alias = "IBUF65")]
pub type Ibuf65 = crate::Reg<ibuf65::Ibuf65Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf65;
#[doc = "IBUF66 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf66`]
module"]
#[doc(alias = "IBUF66")]
pub type Ibuf66 = crate::Reg<ibuf66::Ibuf66Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf66;
#[doc = "IBUF67 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf67`]
module"]
#[doc(alias = "IBUF67")]
pub type Ibuf67 = crate::Reg<ibuf67::Ibuf67Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf67;
#[doc = "IBUF68 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf68`]
module"]
#[doc(alias = "IBUF68")]
pub type Ibuf68 = crate::Reg<ibuf68::Ibuf68Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf68;
#[doc = "IBUF69 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf69`]
module"]
#[doc(alias = "IBUF69")]
pub type Ibuf69 = crate::Reg<ibuf69::Ibuf69Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf69;
#[doc = "IBUF70 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf70`]
module"]
#[doc(alias = "IBUF70")]
pub type Ibuf70 = crate::Reg<ibuf70::Ibuf70Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf70;
#[doc = "IBUF71 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf71`]
module"]
#[doc(alias = "IBUF71")]
pub type Ibuf71 = crate::Reg<ibuf71::Ibuf71Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf71;
#[doc = "IBUF72 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf72`]
module"]
#[doc(alias = "IBUF72")]
pub type Ibuf72 = crate::Reg<ibuf72::Ibuf72Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf72;
#[doc = "IBUF73 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf73`]
module"]
#[doc(alias = "IBUF73")]
pub type Ibuf73 = crate::Reg<ibuf73::Ibuf73Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf73;
#[doc = "IBUF74 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf74`]
module"]
#[doc(alias = "IBUF74")]
pub type Ibuf74 = crate::Reg<ibuf74::Ibuf74Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf74;
#[doc = "IBUF75 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf75`]
module"]
#[doc(alias = "IBUF75")]
pub type Ibuf75 = crate::Reg<ibuf75::Ibuf75Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf75;
#[doc = "IBUF76 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf76`]
module"]
#[doc(alias = "IBUF76")]
pub type Ibuf76 = crate::Reg<ibuf76::Ibuf76Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf76;
#[doc = "IBUF77 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf77`]
module"]
#[doc(alias = "IBUF77")]
pub type Ibuf77 = crate::Reg<ibuf77::Ibuf77Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf77;
#[doc = "IBUF78 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf78`]
module"]
#[doc(alias = "IBUF78")]
pub type Ibuf78 = crate::Reg<ibuf78::Ibuf78Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf78;
#[doc = "IBUF79 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf79`]
module"]
#[doc(alias = "IBUF79")]
pub type Ibuf79 = crate::Reg<ibuf79::Ibuf79Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf79;
#[doc = "IBUF80 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf80`]
module"]
#[doc(alias = "IBUF80")]
pub type Ibuf80 = crate::Reg<ibuf80::Ibuf80Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf80;
#[doc = "IBUF81 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf81`]
module"]
#[doc(alias = "IBUF81")]
pub type Ibuf81 = crate::Reg<ibuf81::Ibuf81Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf81;
#[doc = "IBUF82 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf82`]
module"]
#[doc(alias = "IBUF82")]
pub type Ibuf82 = crate::Reg<ibuf82::Ibuf82Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf82;
#[doc = "IBUF83 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf83`]
module"]
#[doc(alias = "IBUF83")]
pub type Ibuf83 = crate::Reg<ibuf83::Ibuf83Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf83;
#[doc = "IBUF84 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf84`]
module"]
#[doc(alias = "IBUF84")]
pub type Ibuf84 = crate::Reg<ibuf84::Ibuf84Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf84;
#[doc = "IBUF85 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf85`]
module"]
#[doc(alias = "IBUF85")]
pub type Ibuf85 = crate::Reg<ibuf85::Ibuf85Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf85;
#[doc = "IBUF86 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf86`]
module"]
#[doc(alias = "IBUF86")]
pub type Ibuf86 = crate::Reg<ibuf86::Ibuf86Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf86;
#[doc = "IBUF87 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf87`]
module"]
#[doc(alias = "IBUF87")]
pub type Ibuf87 = crate::Reg<ibuf87::Ibuf87Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf87;
#[doc = "IBUF88 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf88`]
module"]
#[doc(alias = "IBUF88")]
pub type Ibuf88 = crate::Reg<ibuf88::Ibuf88Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf88;
#[doc = "IBUF89 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf89`]
module"]
#[doc(alias = "IBUF89")]
pub type Ibuf89 = crate::Reg<ibuf89::Ibuf89Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf89;
#[doc = "IBUF90 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf90`]
module"]
#[doc(alias = "IBUF90")]
pub type Ibuf90 = crate::Reg<ibuf90::Ibuf90Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf90;
#[doc = "IBUF91 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf91`]
module"]
#[doc(alias = "IBUF91")]
pub type Ibuf91 = crate::Reg<ibuf91::Ibuf91Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf91;
#[doc = "IBUF92 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf92`]
module"]
#[doc(alias = "IBUF92")]
pub type Ibuf92 = crate::Reg<ibuf92::Ibuf92Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf92;
#[doc = "IBUF93 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf93`]
module"]
#[doc(alias = "IBUF93")]
pub type Ibuf93 = crate::Reg<ibuf93::Ibuf93Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf93;
#[doc = "IBUF94 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf94`]
module"]
#[doc(alias = "IBUF94")]
pub type Ibuf94 = crate::Reg<ibuf94::Ibuf94Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf94;
#[doc = "IBUF95 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf95`]
module"]
#[doc(alias = "IBUF95")]
pub type Ibuf95 = crate::Reg<ibuf95::Ibuf95Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf95;
#[doc = "IBUF96 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf96`]
module"]
#[doc(alias = "IBUF96")]
pub type Ibuf96 = crate::Reg<ibuf96::Ibuf96Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf96;
#[doc = "IBUF97 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf97`]
module"]
#[doc(alias = "IBUF97")]
pub type Ibuf97 = crate::Reg<ibuf97::Ibuf97Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf97;
#[doc = "IBUF98 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf98`]
module"]
#[doc(alias = "IBUF98")]
pub type Ibuf98 = crate::Reg<ibuf98::Ibuf98Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf98;
#[doc = "IBUF99 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf99`]
module"]
#[doc(alias = "IBUF99")]
pub type Ibuf99 = crate::Reg<ibuf99::Ibuf99Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf99;
#[doc = "IBUF100 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf100`]
module"]
#[doc(alias = "IBUF100")]
pub type Ibuf100 = crate::Reg<ibuf100::Ibuf100Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf100;
#[doc = "IBUF101 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf101::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf101`]
module"]
#[doc(alias = "IBUF101")]
pub type Ibuf101 = crate::Reg<ibuf101::Ibuf101Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf101;
#[doc = "IBUF102 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf102::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf102`]
module"]
#[doc(alias = "IBUF102")]
pub type Ibuf102 = crate::Reg<ibuf102::Ibuf102Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf102;
#[doc = "IBUF103 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf103::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf103::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf103`]
module"]
#[doc(alias = "IBUF103")]
pub type Ibuf103 = crate::Reg<ibuf103::Ibuf103Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf103;
#[doc = "IBUF104 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf104`]
module"]
#[doc(alias = "IBUF104")]
pub type Ibuf104 = crate::Reg<ibuf104::Ibuf104Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf104;
#[doc = "IBUF105 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf105::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf105::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf105`]
module"]
#[doc(alias = "IBUF105")]
pub type Ibuf105 = crate::Reg<ibuf105::Ibuf105Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf105;
#[doc = "IBUF106 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf106::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf106::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf106`]
module"]
#[doc(alias = "IBUF106")]
pub type Ibuf106 = crate::Reg<ibuf106::Ibuf106Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf106;
#[doc = "IBUF107 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf107::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf107::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf107`]
module"]
#[doc(alias = "IBUF107")]
pub type Ibuf107 = crate::Reg<ibuf107::Ibuf107Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf107;
#[doc = "IBUF108 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf108`]
module"]
#[doc(alias = "IBUF108")]
pub type Ibuf108 = crate::Reg<ibuf108::Ibuf108Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf108;
#[doc = "IBUF109 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf109::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf109::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf109`]
module"]
#[doc(alias = "IBUF109")]
pub type Ibuf109 = crate::Reg<ibuf109::Ibuf109Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf109;
#[doc = "IBUF110 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf110`]
module"]
#[doc(alias = "IBUF110")]
pub type Ibuf110 = crate::Reg<ibuf110::Ibuf110Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf110;
#[doc = "IBUF111 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf111`]
module"]
#[doc(alias = "IBUF111")]
pub type Ibuf111 = crate::Reg<ibuf111::Ibuf111Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf111;
#[doc = "IBUF112 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf112`]
module"]
#[doc(alias = "IBUF112")]
pub type Ibuf112 = crate::Reg<ibuf112::Ibuf112Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf112;
#[doc = "IBUF113 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf113`]
module"]
#[doc(alias = "IBUF113")]
pub type Ibuf113 = crate::Reg<ibuf113::Ibuf113Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf113;
#[doc = "IBUF114 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf114`]
module"]
#[doc(alias = "IBUF114")]
pub type Ibuf114 = crate::Reg<ibuf114::Ibuf114Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf114;
#[doc = "IBUF115 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf115`]
module"]
#[doc(alias = "IBUF115")]
pub type Ibuf115 = crate::Reg<ibuf115::Ibuf115Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf115;
#[doc = "IBUF116 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf116`]
module"]
#[doc(alias = "IBUF116")]
pub type Ibuf116 = crate::Reg<ibuf116::Ibuf116Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf116;
#[doc = "IBUF117 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf117`]
module"]
#[doc(alias = "IBUF117")]
pub type Ibuf117 = crate::Reg<ibuf117::Ibuf117Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf117;
#[doc = "IBUF118 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf118`]
module"]
#[doc(alias = "IBUF118")]
pub type Ibuf118 = crate::Reg<ibuf118::Ibuf118Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf118;
#[doc = "IBUF119 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf119`]
module"]
#[doc(alias = "IBUF119")]
pub type Ibuf119 = crate::Reg<ibuf119::Ibuf119Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf119;
#[doc = "IBUF120 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf120`]
module"]
#[doc(alias = "IBUF120")]
pub type Ibuf120 = crate::Reg<ibuf120::Ibuf120Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf120;
#[doc = "IBUF121 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf121`]
module"]
#[doc(alias = "IBUF121")]
pub type Ibuf121 = crate::Reg<ibuf121::Ibuf121Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf121;
#[doc = "IBUF122 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf122`]
module"]
#[doc(alias = "IBUF122")]
pub type Ibuf122 = crate::Reg<ibuf122::Ibuf122Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf122;
#[doc = "IBUF123 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf123`]
module"]
#[doc(alias = "IBUF123")]
pub type Ibuf123 = crate::Reg<ibuf123::Ibuf123Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf123;
#[doc = "IBUF124 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf124`]
module"]
#[doc(alias = "IBUF124")]
pub type Ibuf124 = crate::Reg<ibuf124::Ibuf124Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf124;
#[doc = "IBUF125 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf125`]
module"]
#[doc(alias = "IBUF125")]
pub type Ibuf125 = crate::Reg<ibuf125::Ibuf125Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf125;
#[doc = "IBUF126 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf126`]
module"]
#[doc(alias = "IBUF126")]
pub type Ibuf126 = crate::Reg<ibuf126::Ibuf126Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf126;
#[doc = "IBUF127 (rw) register accessor: Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibuf127`]
module"]
#[doc(alias = "IBUF127")]
pub type Ibuf127 = crate::Reg<ibuf127::Ibuf127Spec>;
#[doc = "Input Buffer Register"]
pub mod ibuf127;
