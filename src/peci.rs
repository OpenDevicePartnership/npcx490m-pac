#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    peci_ctl_sts: PeciCtlSts,
    peci_rd_length: PeciRdLength,
    peci_addr: PeciAddr,
    peci_cmd: PeciCmd,
    peci_ctl2: PeciCtl2,
    peci_index: PeciIndex,
    peci_idata: PeciIdata,
    peci_wr_length: PeciWrLength,
    _reserved8: [u8; 0x03],
    peci_wr_fcs: PeciWrFcs,
    peci_rd_fcs: PeciRdFcs,
    peci_aw_fcs: PeciAwFcs,
    _reserved11: [u8; 0x01],
    peci_rate: PeciRate,
    _reserved_12_peci_data: [u8; 0x01],
    _reserved_13_peci_data: [u8; 0x01],
    _reserved_14_peci_data: [u8; 0x01],
    _reserved_15_peci_data: [u8; 0x01],
    _reserved_16_peci_data: [u8; 0x01],
    _reserved_17_peci_data: [u8; 0x01],
    _reserved_18_peci_data: [u8; 0x01],
    _reserved_19_peci_data: [u8; 0x01],
    _reserved_20_peci_data: [u8; 0x01],
    _reserved_21_peci_data: [u8; 0x01],
    _reserved_22_peci_data: [u8; 0x01],
    _reserved_23_peci_data: [u8; 0x01],
    _reserved_24_peci_data: [u8; 0x01],
    _reserved_25_peci_data: [u8; 0x01],
    _reserved_26_peci_data: [u8; 0x01],
    _reserved_27_peci_data: [u8; 0x01],
    _reserved_28_peci_data: [u8; 0x01],
    _reserved_29_peci_data: [u8; 0x01],
    _reserved_30_peci_data: [u8; 0x01],
    _reserved_31_peci_data: [u8; 0x01],
    _reserved_32_peci_data: [u8; 0x01],
    _reserved_33_peci_data: [u8; 0x01],
    _reserved_34_peci_data: [u8; 0x01],
    _reserved_35_peci_data: [u8; 0x01],
    _reserved_36_peci_data: [u8; 0x01],
    _reserved_37_peci_data: [u8; 0x01],
    _reserved_38_peci_data: [u8; 0x01],
    _reserved_39_peci_data: [u8; 0x01],
    _reserved_40_peci_data: [u8; 0x01],
    _reserved_41_peci_data: [u8; 0x01],
    _reserved_42_peci_data: [u8; 0x01],
    _reserved_43_peci_data: [u8; 0x01],
    _reserved_44_peci_data: [u8; 0x01],
    _reserved_45_peci_data: [u8; 0x01],
    _reserved_46_peci_data: [u8; 0x01],
    _reserved_47_peci_data: [u8; 0x01],
    _reserved_48_peci_data: [u8; 0x01],
    _reserved_49_peci_data: [u8; 0x01],
    _reserved_50_peci_data: [u8; 0x01],
    _reserved_51_peci_data: [u8; 0x01],
    _reserved_52_peci_data: [u8; 0x01],
    _reserved_53_peci_data: [u8; 0x01],
    _reserved_54_peci_data: [u8; 0x01],
    _reserved_55_peci_data: [u8; 0x01],
    _reserved_56_peci_data: [u8; 0x01],
    _reserved_57_peci_data: [u8; 0x01],
    _reserved_58_peci_data: [u8; 0x01],
    _reserved_59_peci_data: [u8; 0x01],
    _reserved_60_peci_data: [u8; 0x01],
    _reserved_61_peci_data: [u8; 0x01],
    _reserved_62_peci_data: [u8; 0x01],
    _reserved_63_peci_data: [u8; 0x01],
    _reserved_64_peci_data: [u8; 0x01],
    _reserved_65_peci_data: [u8; 0x01],
    _reserved_66_peci_data: [u8; 0x01],
    _reserved_67_peci_data: [u8; 0x01],
    _reserved_68_peci_data: [u8; 0x01],
    _reserved_69_peci_data: [u8; 0x01],
    _reserved_70_peci_data: [u8; 0x01],
    _reserved_71_peci_data: [u8; 0x01],
    _reserved_72_peci_data: [u8; 0x01],
    _reserved_73_peci_data: [u8; 0x01],
    _reserved_74_peci_data: [u8; 0x01],
    _reserved_75_peci_data: [u8; 0x01],
}
impl RegisterBlock {
    #[doc = "0x00 - PECI Control Status Register (PECI_CTL_STS)"]
    #[inline(always)]
    pub const fn peci_ctl_sts(&self) -> &PeciCtlSts {
        &self.peci_ctl_sts
    }
    #[doc = "0x01 - PECI Read Length Register (PECI_RD_LENGTH)"]
    #[inline(always)]
    pub const fn peci_rd_length(&self) -> &PeciRdLength {
        &self.peci_rd_length
    }
    #[doc = "0x02 - PECI Address Register (PECI_ADDR)"]
    #[inline(always)]
    pub const fn peci_addr(&self) -> &PeciAddr {
        &self.peci_addr
    }
    #[doc = "0x03 - PECI Command Register (PECI_CMD)"]
    #[inline(always)]
    pub const fn peci_cmd(&self) -> &PeciCmd {
        &self.peci_cmd
    }
    #[doc = "0x04 - PECI Control 2 Register (PECI_CTL2)"]
    #[inline(always)]
    pub const fn peci_ctl2(&self) -> &PeciCtl2 {
        &self.peci_ctl2
    }
    #[doc = "0x05 - PECI Index Register (PECI_INDEX)"]
    #[inline(always)]
    pub const fn peci_index(&self) -> &PeciIndex {
        &self.peci_index
    }
    #[doc = "0x06 - PECI Index Data Registers (PECI_IDATA)"]
    #[inline(always)]
    pub const fn peci_idata(&self) -> &PeciIdata {
        &self.peci_idata
    }
    #[doc = "0x07 - PECI Write Length Register (PECI_WR_LENGTH)"]
    #[inline(always)]
    pub const fn peci_wr_length(&self) -> &PeciWrLength {
        &self.peci_wr_length
    }
    #[doc = "0x0b - PECI Write FCS Register (PECI_WR_FCS)"]
    #[inline(always)]
    pub const fn peci_wr_fcs(&self) -> &PeciWrFcs {
        &self.peci_wr_fcs
    }
    #[doc = "0x0c - PECI Read FCS Register (PECI_RD_FCS)"]
    #[inline(always)]
    pub const fn peci_rd_fcs(&self) -> &PeciRdFcs {
        &self.peci_rd_fcs
    }
    #[doc = "0x0d - PECI Assured Write FCS Register (PECI_AW_FCS)"]
    #[inline(always)]
    pub const fn peci_aw_fcs(&self) -> &PeciAwFcs {
        &self.peci_aw_fcs
    }
    #[doc = "0x0f - PECI Transfer Rate Register (PECI_RATE)"]
    #[inline(always)]
    pub const fn peci_rate(&self) -> &PeciRate {
        &self.peci_rate
    }
    #[doc = "0x10 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out0(&self) -> &PeciDataOut0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in0(&self) -> &PeciDataIn0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x11 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out1(&self) -> &PeciDataOut1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(17).cast() }
    }
    #[doc = "0x11 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in1(&self) -> &PeciDataIn1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(17).cast() }
    }
    #[doc = "0x12 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out2(&self) -> &PeciDataOut2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x12 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in2(&self) -> &PeciDataIn2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x13 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out3(&self) -> &PeciDataOut3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(19).cast() }
    }
    #[doc = "0x13 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in3(&self) -> &PeciDataIn3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(19).cast() }
    }
    #[doc = "0x14 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out4(&self) -> &PeciDataOut4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in4(&self) -> &PeciDataIn4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x15 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out5(&self) -> &PeciDataOut5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(21).cast() }
    }
    #[doc = "0x15 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in5(&self) -> &PeciDataIn5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(21).cast() }
    }
    #[doc = "0x16 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out6(&self) -> &PeciDataOut6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(22).cast() }
    }
    #[doc = "0x16 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in6(&self) -> &PeciDataIn6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(22).cast() }
    }
    #[doc = "0x17 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out7(&self) -> &PeciDataOut7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(23).cast() }
    }
    #[doc = "0x17 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in7(&self) -> &PeciDataIn7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(23).cast() }
    }
    #[doc = "0x18 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out8(&self) -> &PeciDataOut8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in8(&self) -> &PeciDataIn8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x19 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out9(&self) -> &PeciDataOut9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(25).cast() }
    }
    #[doc = "0x19 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in9(&self) -> &PeciDataIn9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(25).cast() }
    }
    #[doc = "0x1a - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out10(&self) -> &PeciDataOut10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1a - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in10(&self) -> &PeciDataIn10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1b - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out11(&self) -> &PeciDataOut11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(27).cast() }
    }
    #[doc = "0x1b - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in11(&self) -> &PeciDataIn11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(27).cast() }
    }
    #[doc = "0x1c - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out12(&self) -> &PeciDataOut12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in12(&self) -> &PeciDataIn12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1d - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out13(&self) -> &PeciDataOut13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(29).cast() }
    }
    #[doc = "0x1d - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in13(&self) -> &PeciDataIn13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(29).cast() }
    }
    #[doc = "0x1e - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out14(&self) -> &PeciDataOut14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(30).cast() }
    }
    #[doc = "0x1e - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in14(&self) -> &PeciDataIn14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(30).cast() }
    }
    #[doc = "0x1f - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out15(&self) -> &PeciDataOut15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(31).cast() }
    }
    #[doc = "0x1f - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in15(&self) -> &PeciDataIn15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(31).cast() }
    }
    #[doc = "0x20 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out16(&self) -> &PeciDataOut16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in16(&self) -> &PeciDataIn16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x21 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out17(&self) -> &PeciDataOut17 {
        unsafe { &*(self as *const Self).cast::<u8>().add(33).cast() }
    }
    #[doc = "0x21 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in17(&self) -> &PeciDataIn17 {
        unsafe { &*(self as *const Self).cast::<u8>().add(33).cast() }
    }
    #[doc = "0x22 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out18(&self) -> &PeciDataOut18 {
        unsafe { &*(self as *const Self).cast::<u8>().add(34).cast() }
    }
    #[doc = "0x22 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in18(&self) -> &PeciDataIn18 {
        unsafe { &*(self as *const Self).cast::<u8>().add(34).cast() }
    }
    #[doc = "0x23 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out19(&self) -> &PeciDataOut19 {
        unsafe { &*(self as *const Self).cast::<u8>().add(35).cast() }
    }
    #[doc = "0x23 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in19(&self) -> &PeciDataIn19 {
        unsafe { &*(self as *const Self).cast::<u8>().add(35).cast() }
    }
    #[doc = "0x24 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out20(&self) -> &PeciDataOut20 {
        unsafe { &*(self as *const Self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x24 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in20(&self) -> &PeciDataIn20 {
        unsafe { &*(self as *const Self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x25 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out21(&self) -> &PeciDataOut21 {
        unsafe { &*(self as *const Self).cast::<u8>().add(37).cast() }
    }
    #[doc = "0x25 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in21(&self) -> &PeciDataIn21 {
        unsafe { &*(self as *const Self).cast::<u8>().add(37).cast() }
    }
    #[doc = "0x26 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out22(&self) -> &PeciDataOut22 {
        unsafe { &*(self as *const Self).cast::<u8>().add(38).cast() }
    }
    #[doc = "0x26 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in22(&self) -> &PeciDataIn22 {
        unsafe { &*(self as *const Self).cast::<u8>().add(38).cast() }
    }
    #[doc = "0x27 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out23(&self) -> &PeciDataOut23 {
        unsafe { &*(self as *const Self).cast::<u8>().add(39).cast() }
    }
    #[doc = "0x27 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in23(&self) -> &PeciDataIn23 {
        unsafe { &*(self as *const Self).cast::<u8>().add(39).cast() }
    }
    #[doc = "0x28 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out24(&self) -> &PeciDataOut24 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in24(&self) -> &PeciDataIn24 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x29 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out25(&self) -> &PeciDataOut25 {
        unsafe { &*(self as *const Self).cast::<u8>().add(41).cast() }
    }
    #[doc = "0x29 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in25(&self) -> &PeciDataIn25 {
        unsafe { &*(self as *const Self).cast::<u8>().add(41).cast() }
    }
    #[doc = "0x2a - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out26(&self) -> &PeciDataOut26 {
        unsafe { &*(self as *const Self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2a - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in26(&self) -> &PeciDataIn26 {
        unsafe { &*(self as *const Self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2b - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out27(&self) -> &PeciDataOut27 {
        unsafe { &*(self as *const Self).cast::<u8>().add(43).cast() }
    }
    #[doc = "0x2b - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in27(&self) -> &PeciDataIn27 {
        unsafe { &*(self as *const Self).cast::<u8>().add(43).cast() }
    }
    #[doc = "0x2c - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out28(&self) -> &PeciDataOut28 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in28(&self) -> &PeciDataIn28 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2d - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out29(&self) -> &PeciDataOut29 {
        unsafe { &*(self as *const Self).cast::<u8>().add(45).cast() }
    }
    #[doc = "0x2d - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in29(&self) -> &PeciDataIn29 {
        unsafe { &*(self as *const Self).cast::<u8>().add(45).cast() }
    }
    #[doc = "0x2e - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out30(&self) -> &PeciDataOut30 {
        unsafe { &*(self as *const Self).cast::<u8>().add(46).cast() }
    }
    #[doc = "0x2e - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in30(&self) -> &PeciDataIn30 {
        unsafe { &*(self as *const Self).cast::<u8>().add(46).cast() }
    }
    #[doc = "0x2f - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out31(&self) -> &PeciDataOut31 {
        unsafe { &*(self as *const Self).cast::<u8>().add(47).cast() }
    }
    #[doc = "0x2f - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in31(&self) -> &PeciDataIn31 {
        unsafe { &*(self as *const Self).cast::<u8>().add(47).cast() }
    }
    #[doc = "0x30 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out32(&self) -> &PeciDataOut32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in32(&self) -> &PeciDataIn32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x31 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out33(&self) -> &PeciDataOut33 {
        unsafe { &*(self as *const Self).cast::<u8>().add(49).cast() }
    }
    #[doc = "0x31 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in33(&self) -> &PeciDataIn33 {
        unsafe { &*(self as *const Self).cast::<u8>().add(49).cast() }
    }
    #[doc = "0x32 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out34(&self) -> &PeciDataOut34 {
        unsafe { &*(self as *const Self).cast::<u8>().add(50).cast() }
    }
    #[doc = "0x32 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in34(&self) -> &PeciDataIn34 {
        unsafe { &*(self as *const Self).cast::<u8>().add(50).cast() }
    }
    #[doc = "0x33 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out35(&self) -> &PeciDataOut35 {
        unsafe { &*(self as *const Self).cast::<u8>().add(51).cast() }
    }
    #[doc = "0x33 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in35(&self) -> &PeciDataIn35 {
        unsafe { &*(self as *const Self).cast::<u8>().add(51).cast() }
    }
    #[doc = "0x34 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out36(&self) -> &PeciDataOut36 {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in36(&self) -> &PeciDataIn36 {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x35 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out37(&self) -> &PeciDataOut37 {
        unsafe { &*(self as *const Self).cast::<u8>().add(53).cast() }
    }
    #[doc = "0x35 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in37(&self) -> &PeciDataIn37 {
        unsafe { &*(self as *const Self).cast::<u8>().add(53).cast() }
    }
    #[doc = "0x36 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out38(&self) -> &PeciDataOut38 {
        unsafe { &*(self as *const Self).cast::<u8>().add(54).cast() }
    }
    #[doc = "0x36 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in38(&self) -> &PeciDataIn38 {
        unsafe { &*(self as *const Self).cast::<u8>().add(54).cast() }
    }
    #[doc = "0x37 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out39(&self) -> &PeciDataOut39 {
        unsafe { &*(self as *const Self).cast::<u8>().add(55).cast() }
    }
    #[doc = "0x37 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in39(&self) -> &PeciDataIn39 {
        unsafe { &*(self as *const Self).cast::<u8>().add(55).cast() }
    }
    #[doc = "0x38 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out40(&self) -> &PeciDataOut40 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in40(&self) -> &PeciDataIn40 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x39 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out41(&self) -> &PeciDataOut41 {
        unsafe { &*(self as *const Self).cast::<u8>().add(57).cast() }
    }
    #[doc = "0x39 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in41(&self) -> &PeciDataIn41 {
        unsafe { &*(self as *const Self).cast::<u8>().add(57).cast() }
    }
    #[doc = "0x3a - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out42(&self) -> &PeciDataOut42 {
        unsafe { &*(self as *const Self).cast::<u8>().add(58).cast() }
    }
    #[doc = "0x3a - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in42(&self) -> &PeciDataIn42 {
        unsafe { &*(self as *const Self).cast::<u8>().add(58).cast() }
    }
    #[doc = "0x3b - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out43(&self) -> &PeciDataOut43 {
        unsafe { &*(self as *const Self).cast::<u8>().add(59).cast() }
    }
    #[doc = "0x3b - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in43(&self) -> &PeciDataIn43 {
        unsafe { &*(self as *const Self).cast::<u8>().add(59).cast() }
    }
    #[doc = "0x3c - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out44(&self) -> &PeciDataOut44 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in44(&self) -> &PeciDataIn44 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3d - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out45(&self) -> &PeciDataOut45 {
        unsafe { &*(self as *const Self).cast::<u8>().add(61).cast() }
    }
    #[doc = "0x3d - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in45(&self) -> &PeciDataIn45 {
        unsafe { &*(self as *const Self).cast::<u8>().add(61).cast() }
    }
    #[doc = "0x3e - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out46(&self) -> &PeciDataOut46 {
        unsafe { &*(self as *const Self).cast::<u8>().add(62).cast() }
    }
    #[doc = "0x3e - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in46(&self) -> &PeciDataIn46 {
        unsafe { &*(self as *const Self).cast::<u8>().add(62).cast() }
    }
    #[doc = "0x3f - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out47(&self) -> &PeciDataOut47 {
        unsafe { &*(self as *const Self).cast::<u8>().add(63).cast() }
    }
    #[doc = "0x3f - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in47(&self) -> &PeciDataIn47 {
        unsafe { &*(self as *const Self).cast::<u8>().add(63).cast() }
    }
    #[doc = "0x40 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out48(&self) -> &PeciDataOut48 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in48(&self) -> &PeciDataIn48 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x41 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out49(&self) -> &PeciDataOut49 {
        unsafe { &*(self as *const Self).cast::<u8>().add(65).cast() }
    }
    #[doc = "0x41 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in49(&self) -> &PeciDataIn49 {
        unsafe { &*(self as *const Self).cast::<u8>().add(65).cast() }
    }
    #[doc = "0x42 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out50(&self) -> &PeciDataOut50 {
        unsafe { &*(self as *const Self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x42 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in50(&self) -> &PeciDataIn50 {
        unsafe { &*(self as *const Self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out51(&self) -> &PeciDataOut51 {
        unsafe { &*(self as *const Self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x43 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in51(&self) -> &PeciDataIn51 {
        unsafe { &*(self as *const Self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x44 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out52(&self) -> &PeciDataOut52 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in52(&self) -> &PeciDataIn52 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x45 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out53(&self) -> &PeciDataOut53 {
        unsafe { &*(self as *const Self).cast::<u8>().add(69).cast() }
    }
    #[doc = "0x45 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in53(&self) -> &PeciDataIn53 {
        unsafe { &*(self as *const Self).cast::<u8>().add(69).cast() }
    }
    #[doc = "0x46 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out54(&self) -> &PeciDataOut54 {
        unsafe { &*(self as *const Self).cast::<u8>().add(70).cast() }
    }
    #[doc = "0x46 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in54(&self) -> &PeciDataIn54 {
        unsafe { &*(self as *const Self).cast::<u8>().add(70).cast() }
    }
    #[doc = "0x47 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out55(&self) -> &PeciDataOut55 {
        unsafe { &*(self as *const Self).cast::<u8>().add(71).cast() }
    }
    #[doc = "0x47 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in55(&self) -> &PeciDataIn55 {
        unsafe { &*(self as *const Self).cast::<u8>().add(71).cast() }
    }
    #[doc = "0x48 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out56(&self) -> &PeciDataOut56 {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in56(&self) -> &PeciDataIn56 {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x49 - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out57(&self) -> &PeciDataOut57 {
        unsafe { &*(self as *const Self).cast::<u8>().add(73).cast() }
    }
    #[doc = "0x49 - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in57(&self) -> &PeciDataIn57 {
        unsafe { &*(self as *const Self).cast::<u8>().add(73).cast() }
    }
    #[doc = "0x4a - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out58(&self) -> &PeciDataOut58 {
        unsafe { &*(self as *const Self).cast::<u8>().add(74).cast() }
    }
    #[doc = "0x4a - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in58(&self) -> &PeciDataIn58 {
        unsafe { &*(self as *const Self).cast::<u8>().add(74).cast() }
    }
    #[doc = "0x4b - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out59(&self) -> &PeciDataOut59 {
        unsafe { &*(self as *const Self).cast::<u8>().add(75).cast() }
    }
    #[doc = "0x4b - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in59(&self) -> &PeciDataIn59 {
        unsafe { &*(self as *const Self).cast::<u8>().add(75).cast() }
    }
    #[doc = "0x4c - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out60(&self) -> &PeciDataOut60 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in60(&self) -> &PeciDataIn60 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4d - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out61(&self) -> &PeciDataOut61 {
        unsafe { &*(self as *const Self).cast::<u8>().add(77).cast() }
    }
    #[doc = "0x4d - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in61(&self) -> &PeciDataIn61 {
        unsafe { &*(self as *const Self).cast::<u8>().add(77).cast() }
    }
    #[doc = "0x4e - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out62(&self) -> &PeciDataOut62 {
        unsafe { &*(self as *const Self).cast::<u8>().add(78).cast() }
    }
    #[doc = "0x4e - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in62(&self) -> &PeciDataIn62 {
        unsafe { &*(self as *const Self).cast::<u8>().add(78).cast() }
    }
    #[doc = "0x4f - PECI Data Out Register (PECI_DATA_OUT00-3F)"]
    #[inline(always)]
    pub const fn peci_data_out63(&self) -> &PeciDataOut63 {
        unsafe { &*(self as *const Self).cast::<u8>().add(79).cast() }
    }
    #[doc = "0x4f - PECI Data In Register (PECI_DATA_IN00-3F)"]
    #[inline(always)]
    pub const fn peci_data_in63(&self) -> &PeciDataIn63 {
        unsafe { &*(self as *const Self).cast::<u8>().add(79).cast() }
    }
}
#[doc = "PECI_CTL_STS (rw) register accessor: PECI Control Status Register (PECI_CTL_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_ctl_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_ctl_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_ctl_sts`]
module"]
#[doc(alias = "PECI_CTL_STS")]
pub type PeciCtlSts = crate::Reg<peci_ctl_sts::PeciCtlStsSpec>;
#[doc = "PECI Control Status Register (PECI_CTL_STS)"]
pub mod peci_ctl_sts;
#[doc = "PECI_RD_LENGTH (rw) register accessor: PECI Read Length Register (PECI_RD_LENGTH)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_rd_length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_rd_length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_rd_length`]
module"]
#[doc(alias = "PECI_RD_LENGTH")]
pub type PeciRdLength = crate::Reg<peci_rd_length::PeciRdLengthSpec>;
#[doc = "PECI Read Length Register (PECI_RD_LENGTH)"]
pub mod peci_rd_length;
#[doc = "PECI_ADDR (rw) register accessor: PECI Address Register (PECI_ADDR)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_addr`]
module"]
#[doc(alias = "PECI_ADDR")]
pub type PeciAddr = crate::Reg<peci_addr::PeciAddrSpec>;
#[doc = "PECI Address Register (PECI_ADDR)"]
pub mod peci_addr;
#[doc = "PECI_CMD (rw) register accessor: PECI Command Register (PECI_CMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_cmd`]
module"]
#[doc(alias = "PECI_CMD")]
pub type PeciCmd = crate::Reg<peci_cmd::PeciCmdSpec>;
#[doc = "PECI Command Register (PECI_CMD)"]
pub mod peci_cmd;
#[doc = "PECI_CTL2 (rw) register accessor: PECI Control 2 Register (PECI_CTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_ctl2`]
module"]
#[doc(alias = "PECI_CTL2")]
pub type PeciCtl2 = crate::Reg<peci_ctl2::PeciCtl2Spec>;
#[doc = "PECI Control 2 Register (PECI_CTL2)"]
pub mod peci_ctl2;
#[doc = "PECI_INDEX (rw) register accessor: PECI Index Register (PECI_INDEX)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_index`]
module"]
#[doc(alias = "PECI_INDEX")]
pub type PeciIndex = crate::Reg<peci_index::PeciIndexSpec>;
#[doc = "PECI Index Register (PECI_INDEX)"]
pub mod peci_index;
#[doc = "PECI_IDATA (rw) register accessor: PECI Index Data Registers (PECI_IDATA)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_idata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_idata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_idata`]
module"]
#[doc(alias = "PECI_IDATA")]
pub type PeciIdata = crate::Reg<peci_idata::PeciIdataSpec>;
#[doc = "PECI Index Data Registers (PECI_IDATA)"]
pub mod peci_idata;
#[doc = "PECI_WR_LENGTH (rw) register accessor: PECI Write Length Register (PECI_WR_LENGTH)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_wr_length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_wr_length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_wr_length`]
module"]
#[doc(alias = "PECI_WR_LENGTH")]
pub type PeciWrLength = crate::Reg<peci_wr_length::PeciWrLengthSpec>;
#[doc = "PECI Write Length Register (PECI_WR_LENGTH)"]
pub mod peci_wr_length;
#[doc = "PECI_WR_FCS (rw) register accessor: PECI Write FCS Register (PECI_WR_FCS)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_wr_fcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_wr_fcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_wr_fcs`]
module"]
#[doc(alias = "PECI_WR_FCS")]
pub type PeciWrFcs = crate::Reg<peci_wr_fcs::PeciWrFcsSpec>;
#[doc = "PECI Write FCS Register (PECI_WR_FCS)"]
pub mod peci_wr_fcs;
#[doc = "PECI_RD_FCS (rw) register accessor: PECI Read FCS Register (PECI_RD_FCS)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_rd_fcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_rd_fcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_rd_fcs`]
module"]
#[doc(alias = "PECI_RD_FCS")]
pub type PeciRdFcs = crate::Reg<peci_rd_fcs::PeciRdFcsSpec>;
#[doc = "PECI Read FCS Register (PECI_RD_FCS)"]
pub mod peci_rd_fcs;
#[doc = "PECI_AW_FCS (rw) register accessor: PECI Assured Write FCS Register (PECI_AW_FCS)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_aw_fcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_aw_fcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_aw_fcs`]
module"]
#[doc(alias = "PECI_AW_FCS")]
pub type PeciAwFcs = crate::Reg<peci_aw_fcs::PeciAwFcsSpec>;
#[doc = "PECI Assured Write FCS Register (PECI_AW_FCS)"]
pub mod peci_aw_fcs;
#[doc = "PECI_RATE (rw) register accessor: PECI Transfer Rate Register (PECI_RATE)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_rate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_rate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_rate`]
module"]
#[doc(alias = "PECI_RATE")]
pub type PeciRate = crate::Reg<peci_rate::PeciRateSpec>;
#[doc = "PECI Transfer Rate Register (PECI_RATE)"]
pub mod peci_rate;
#[doc = "PECI_DATA_IN0 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in0`]
module"]
#[doc(alias = "PECI_DATA_IN0")]
pub type PeciDataIn0 = crate::Reg<peci_data_in0::PeciDataIn0Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in0;
#[doc = "PECI_DATA_IN1 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in1`]
module"]
#[doc(alias = "PECI_DATA_IN1")]
pub type PeciDataIn1 = crate::Reg<peci_data_in1::PeciDataIn1Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in1;
#[doc = "PECI_DATA_IN2 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in2`]
module"]
#[doc(alias = "PECI_DATA_IN2")]
pub type PeciDataIn2 = crate::Reg<peci_data_in2::PeciDataIn2Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in2;
#[doc = "PECI_DATA_IN3 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in3`]
module"]
#[doc(alias = "PECI_DATA_IN3")]
pub type PeciDataIn3 = crate::Reg<peci_data_in3::PeciDataIn3Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in3;
#[doc = "PECI_DATA_IN4 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in4`]
module"]
#[doc(alias = "PECI_DATA_IN4")]
pub type PeciDataIn4 = crate::Reg<peci_data_in4::PeciDataIn4Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in4;
#[doc = "PECI_DATA_IN5 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in5`]
module"]
#[doc(alias = "PECI_DATA_IN5")]
pub type PeciDataIn5 = crate::Reg<peci_data_in5::PeciDataIn5Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in5;
#[doc = "PECI_DATA_IN6 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in6`]
module"]
#[doc(alias = "PECI_DATA_IN6")]
pub type PeciDataIn6 = crate::Reg<peci_data_in6::PeciDataIn6Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in6;
#[doc = "PECI_DATA_IN7 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in7`]
module"]
#[doc(alias = "PECI_DATA_IN7")]
pub type PeciDataIn7 = crate::Reg<peci_data_in7::PeciDataIn7Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in7;
#[doc = "PECI_DATA_IN8 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in8`]
module"]
#[doc(alias = "PECI_DATA_IN8")]
pub type PeciDataIn8 = crate::Reg<peci_data_in8::PeciDataIn8Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in8;
#[doc = "PECI_DATA_IN9 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in9`]
module"]
#[doc(alias = "PECI_DATA_IN9")]
pub type PeciDataIn9 = crate::Reg<peci_data_in9::PeciDataIn9Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in9;
#[doc = "PECI_DATA_IN10 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in10`]
module"]
#[doc(alias = "PECI_DATA_IN10")]
pub type PeciDataIn10 = crate::Reg<peci_data_in10::PeciDataIn10Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in10;
#[doc = "PECI_DATA_IN11 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in11`]
module"]
#[doc(alias = "PECI_DATA_IN11")]
pub type PeciDataIn11 = crate::Reg<peci_data_in11::PeciDataIn11Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in11;
#[doc = "PECI_DATA_IN12 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in12`]
module"]
#[doc(alias = "PECI_DATA_IN12")]
pub type PeciDataIn12 = crate::Reg<peci_data_in12::PeciDataIn12Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in12;
#[doc = "PECI_DATA_IN13 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in13`]
module"]
#[doc(alias = "PECI_DATA_IN13")]
pub type PeciDataIn13 = crate::Reg<peci_data_in13::PeciDataIn13Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in13;
#[doc = "PECI_DATA_IN14 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in14`]
module"]
#[doc(alias = "PECI_DATA_IN14")]
pub type PeciDataIn14 = crate::Reg<peci_data_in14::PeciDataIn14Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in14;
#[doc = "PECI_DATA_IN15 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in15`]
module"]
#[doc(alias = "PECI_DATA_IN15")]
pub type PeciDataIn15 = crate::Reg<peci_data_in15::PeciDataIn15Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in15;
#[doc = "PECI_DATA_IN16 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in16`]
module"]
#[doc(alias = "PECI_DATA_IN16")]
pub type PeciDataIn16 = crate::Reg<peci_data_in16::PeciDataIn16Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in16;
#[doc = "PECI_DATA_IN17 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in17`]
module"]
#[doc(alias = "PECI_DATA_IN17")]
pub type PeciDataIn17 = crate::Reg<peci_data_in17::PeciDataIn17Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in17;
#[doc = "PECI_DATA_IN18 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in18`]
module"]
#[doc(alias = "PECI_DATA_IN18")]
pub type PeciDataIn18 = crate::Reg<peci_data_in18::PeciDataIn18Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in18;
#[doc = "PECI_DATA_IN19 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in19`]
module"]
#[doc(alias = "PECI_DATA_IN19")]
pub type PeciDataIn19 = crate::Reg<peci_data_in19::PeciDataIn19Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in19;
#[doc = "PECI_DATA_IN20 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in20`]
module"]
#[doc(alias = "PECI_DATA_IN20")]
pub type PeciDataIn20 = crate::Reg<peci_data_in20::PeciDataIn20Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in20;
#[doc = "PECI_DATA_IN21 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in21`]
module"]
#[doc(alias = "PECI_DATA_IN21")]
pub type PeciDataIn21 = crate::Reg<peci_data_in21::PeciDataIn21Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in21;
#[doc = "PECI_DATA_IN22 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in22`]
module"]
#[doc(alias = "PECI_DATA_IN22")]
pub type PeciDataIn22 = crate::Reg<peci_data_in22::PeciDataIn22Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in22;
#[doc = "PECI_DATA_IN23 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in23`]
module"]
#[doc(alias = "PECI_DATA_IN23")]
pub type PeciDataIn23 = crate::Reg<peci_data_in23::PeciDataIn23Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in23;
#[doc = "PECI_DATA_IN24 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in24`]
module"]
#[doc(alias = "PECI_DATA_IN24")]
pub type PeciDataIn24 = crate::Reg<peci_data_in24::PeciDataIn24Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in24;
#[doc = "PECI_DATA_IN25 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in25`]
module"]
#[doc(alias = "PECI_DATA_IN25")]
pub type PeciDataIn25 = crate::Reg<peci_data_in25::PeciDataIn25Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in25;
#[doc = "PECI_DATA_IN26 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in26`]
module"]
#[doc(alias = "PECI_DATA_IN26")]
pub type PeciDataIn26 = crate::Reg<peci_data_in26::PeciDataIn26Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in26;
#[doc = "PECI_DATA_IN27 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in27`]
module"]
#[doc(alias = "PECI_DATA_IN27")]
pub type PeciDataIn27 = crate::Reg<peci_data_in27::PeciDataIn27Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in27;
#[doc = "PECI_DATA_IN28 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in28`]
module"]
#[doc(alias = "PECI_DATA_IN28")]
pub type PeciDataIn28 = crate::Reg<peci_data_in28::PeciDataIn28Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in28;
#[doc = "PECI_DATA_IN29 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in29`]
module"]
#[doc(alias = "PECI_DATA_IN29")]
pub type PeciDataIn29 = crate::Reg<peci_data_in29::PeciDataIn29Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in29;
#[doc = "PECI_DATA_IN30 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in30`]
module"]
#[doc(alias = "PECI_DATA_IN30")]
pub type PeciDataIn30 = crate::Reg<peci_data_in30::PeciDataIn30Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in30;
#[doc = "PECI_DATA_IN31 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in31`]
module"]
#[doc(alias = "PECI_DATA_IN31")]
pub type PeciDataIn31 = crate::Reg<peci_data_in31::PeciDataIn31Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in31;
#[doc = "PECI_DATA_IN32 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in32`]
module"]
#[doc(alias = "PECI_DATA_IN32")]
pub type PeciDataIn32 = crate::Reg<peci_data_in32::PeciDataIn32Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in32;
#[doc = "PECI_DATA_IN33 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in33`]
module"]
#[doc(alias = "PECI_DATA_IN33")]
pub type PeciDataIn33 = crate::Reg<peci_data_in33::PeciDataIn33Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in33;
#[doc = "PECI_DATA_IN34 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in34`]
module"]
#[doc(alias = "PECI_DATA_IN34")]
pub type PeciDataIn34 = crate::Reg<peci_data_in34::PeciDataIn34Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in34;
#[doc = "PECI_DATA_IN35 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in35`]
module"]
#[doc(alias = "PECI_DATA_IN35")]
pub type PeciDataIn35 = crate::Reg<peci_data_in35::PeciDataIn35Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in35;
#[doc = "PECI_DATA_IN36 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in36`]
module"]
#[doc(alias = "PECI_DATA_IN36")]
pub type PeciDataIn36 = crate::Reg<peci_data_in36::PeciDataIn36Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in36;
#[doc = "PECI_DATA_IN37 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in37`]
module"]
#[doc(alias = "PECI_DATA_IN37")]
pub type PeciDataIn37 = crate::Reg<peci_data_in37::PeciDataIn37Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in37;
#[doc = "PECI_DATA_IN38 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in38`]
module"]
#[doc(alias = "PECI_DATA_IN38")]
pub type PeciDataIn38 = crate::Reg<peci_data_in38::PeciDataIn38Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in38;
#[doc = "PECI_DATA_IN39 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in39`]
module"]
#[doc(alias = "PECI_DATA_IN39")]
pub type PeciDataIn39 = crate::Reg<peci_data_in39::PeciDataIn39Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in39;
#[doc = "PECI_DATA_IN40 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in40`]
module"]
#[doc(alias = "PECI_DATA_IN40")]
pub type PeciDataIn40 = crate::Reg<peci_data_in40::PeciDataIn40Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in40;
#[doc = "PECI_DATA_IN41 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in41`]
module"]
#[doc(alias = "PECI_DATA_IN41")]
pub type PeciDataIn41 = crate::Reg<peci_data_in41::PeciDataIn41Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in41;
#[doc = "PECI_DATA_IN42 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in42`]
module"]
#[doc(alias = "PECI_DATA_IN42")]
pub type PeciDataIn42 = crate::Reg<peci_data_in42::PeciDataIn42Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in42;
#[doc = "PECI_DATA_IN43 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in43`]
module"]
#[doc(alias = "PECI_DATA_IN43")]
pub type PeciDataIn43 = crate::Reg<peci_data_in43::PeciDataIn43Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in43;
#[doc = "PECI_DATA_IN44 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in44`]
module"]
#[doc(alias = "PECI_DATA_IN44")]
pub type PeciDataIn44 = crate::Reg<peci_data_in44::PeciDataIn44Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in44;
#[doc = "PECI_DATA_IN45 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in45`]
module"]
#[doc(alias = "PECI_DATA_IN45")]
pub type PeciDataIn45 = crate::Reg<peci_data_in45::PeciDataIn45Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in45;
#[doc = "PECI_DATA_IN46 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in46`]
module"]
#[doc(alias = "PECI_DATA_IN46")]
pub type PeciDataIn46 = crate::Reg<peci_data_in46::PeciDataIn46Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in46;
#[doc = "PECI_DATA_IN47 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in47`]
module"]
#[doc(alias = "PECI_DATA_IN47")]
pub type PeciDataIn47 = crate::Reg<peci_data_in47::PeciDataIn47Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in47;
#[doc = "PECI_DATA_IN48 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in48`]
module"]
#[doc(alias = "PECI_DATA_IN48")]
pub type PeciDataIn48 = crate::Reg<peci_data_in48::PeciDataIn48Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in48;
#[doc = "PECI_DATA_IN49 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in49`]
module"]
#[doc(alias = "PECI_DATA_IN49")]
pub type PeciDataIn49 = crate::Reg<peci_data_in49::PeciDataIn49Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in49;
#[doc = "PECI_DATA_IN50 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in50`]
module"]
#[doc(alias = "PECI_DATA_IN50")]
pub type PeciDataIn50 = crate::Reg<peci_data_in50::PeciDataIn50Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in50;
#[doc = "PECI_DATA_IN51 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in51`]
module"]
#[doc(alias = "PECI_DATA_IN51")]
pub type PeciDataIn51 = crate::Reg<peci_data_in51::PeciDataIn51Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in51;
#[doc = "PECI_DATA_IN52 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in52`]
module"]
#[doc(alias = "PECI_DATA_IN52")]
pub type PeciDataIn52 = crate::Reg<peci_data_in52::PeciDataIn52Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in52;
#[doc = "PECI_DATA_IN53 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in53`]
module"]
#[doc(alias = "PECI_DATA_IN53")]
pub type PeciDataIn53 = crate::Reg<peci_data_in53::PeciDataIn53Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in53;
#[doc = "PECI_DATA_IN54 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in54`]
module"]
#[doc(alias = "PECI_DATA_IN54")]
pub type PeciDataIn54 = crate::Reg<peci_data_in54::PeciDataIn54Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in54;
#[doc = "PECI_DATA_IN55 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in55`]
module"]
#[doc(alias = "PECI_DATA_IN55")]
pub type PeciDataIn55 = crate::Reg<peci_data_in55::PeciDataIn55Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in55;
#[doc = "PECI_DATA_IN56 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in56`]
module"]
#[doc(alias = "PECI_DATA_IN56")]
pub type PeciDataIn56 = crate::Reg<peci_data_in56::PeciDataIn56Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in56;
#[doc = "PECI_DATA_IN57 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in57`]
module"]
#[doc(alias = "PECI_DATA_IN57")]
pub type PeciDataIn57 = crate::Reg<peci_data_in57::PeciDataIn57Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in57;
#[doc = "PECI_DATA_IN58 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in58`]
module"]
#[doc(alias = "PECI_DATA_IN58")]
pub type PeciDataIn58 = crate::Reg<peci_data_in58::PeciDataIn58Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in58;
#[doc = "PECI_DATA_IN59 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in59`]
module"]
#[doc(alias = "PECI_DATA_IN59")]
pub type PeciDataIn59 = crate::Reg<peci_data_in59::PeciDataIn59Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in59;
#[doc = "PECI_DATA_IN60 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in60`]
module"]
#[doc(alias = "PECI_DATA_IN60")]
pub type PeciDataIn60 = crate::Reg<peci_data_in60::PeciDataIn60Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in60;
#[doc = "PECI_DATA_IN61 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in61`]
module"]
#[doc(alias = "PECI_DATA_IN61")]
pub type PeciDataIn61 = crate::Reg<peci_data_in61::PeciDataIn61Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in61;
#[doc = "PECI_DATA_IN62 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in62`]
module"]
#[doc(alias = "PECI_DATA_IN62")]
pub type PeciDataIn62 = crate::Reg<peci_data_in62::PeciDataIn62Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in62;
#[doc = "PECI_DATA_IN63 (rw) register accessor: PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_in63`]
module"]
#[doc(alias = "PECI_DATA_IN63")]
pub type PeciDataIn63 = crate::Reg<peci_data_in63::PeciDataIn63Spec>;
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)"]
pub mod peci_data_in63;
#[doc = "PECI_DATA_OUT0 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out0`]
module"]
#[doc(alias = "PECI_DATA_OUT0")]
pub type PeciDataOut0 = crate::Reg<peci_data_out0::PeciDataOut0Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out0;
#[doc = "PECI_DATA_OUT1 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out1`]
module"]
#[doc(alias = "PECI_DATA_OUT1")]
pub type PeciDataOut1 = crate::Reg<peci_data_out1::PeciDataOut1Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out1;
#[doc = "PECI_DATA_OUT2 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out2`]
module"]
#[doc(alias = "PECI_DATA_OUT2")]
pub type PeciDataOut2 = crate::Reg<peci_data_out2::PeciDataOut2Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out2;
#[doc = "PECI_DATA_OUT3 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out3`]
module"]
#[doc(alias = "PECI_DATA_OUT3")]
pub type PeciDataOut3 = crate::Reg<peci_data_out3::PeciDataOut3Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out3;
#[doc = "PECI_DATA_OUT4 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out4`]
module"]
#[doc(alias = "PECI_DATA_OUT4")]
pub type PeciDataOut4 = crate::Reg<peci_data_out4::PeciDataOut4Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out4;
#[doc = "PECI_DATA_OUT5 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out5`]
module"]
#[doc(alias = "PECI_DATA_OUT5")]
pub type PeciDataOut5 = crate::Reg<peci_data_out5::PeciDataOut5Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out5;
#[doc = "PECI_DATA_OUT6 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out6`]
module"]
#[doc(alias = "PECI_DATA_OUT6")]
pub type PeciDataOut6 = crate::Reg<peci_data_out6::PeciDataOut6Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out6;
#[doc = "PECI_DATA_OUT7 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out7`]
module"]
#[doc(alias = "PECI_DATA_OUT7")]
pub type PeciDataOut7 = crate::Reg<peci_data_out7::PeciDataOut7Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out7;
#[doc = "PECI_DATA_OUT8 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out8`]
module"]
#[doc(alias = "PECI_DATA_OUT8")]
pub type PeciDataOut8 = crate::Reg<peci_data_out8::PeciDataOut8Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out8;
#[doc = "PECI_DATA_OUT9 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out9`]
module"]
#[doc(alias = "PECI_DATA_OUT9")]
pub type PeciDataOut9 = crate::Reg<peci_data_out9::PeciDataOut9Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out9;
#[doc = "PECI_DATA_OUT10 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out10`]
module"]
#[doc(alias = "PECI_DATA_OUT10")]
pub type PeciDataOut10 = crate::Reg<peci_data_out10::PeciDataOut10Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out10;
#[doc = "PECI_DATA_OUT11 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out11`]
module"]
#[doc(alias = "PECI_DATA_OUT11")]
pub type PeciDataOut11 = crate::Reg<peci_data_out11::PeciDataOut11Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out11;
#[doc = "PECI_DATA_OUT12 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out12`]
module"]
#[doc(alias = "PECI_DATA_OUT12")]
pub type PeciDataOut12 = crate::Reg<peci_data_out12::PeciDataOut12Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out12;
#[doc = "PECI_DATA_OUT13 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out13`]
module"]
#[doc(alias = "PECI_DATA_OUT13")]
pub type PeciDataOut13 = crate::Reg<peci_data_out13::PeciDataOut13Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out13;
#[doc = "PECI_DATA_OUT14 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out14`]
module"]
#[doc(alias = "PECI_DATA_OUT14")]
pub type PeciDataOut14 = crate::Reg<peci_data_out14::PeciDataOut14Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out14;
#[doc = "PECI_DATA_OUT15 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out15`]
module"]
#[doc(alias = "PECI_DATA_OUT15")]
pub type PeciDataOut15 = crate::Reg<peci_data_out15::PeciDataOut15Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out15;
#[doc = "PECI_DATA_OUT16 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out16`]
module"]
#[doc(alias = "PECI_DATA_OUT16")]
pub type PeciDataOut16 = crate::Reg<peci_data_out16::PeciDataOut16Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out16;
#[doc = "PECI_DATA_OUT17 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out17`]
module"]
#[doc(alias = "PECI_DATA_OUT17")]
pub type PeciDataOut17 = crate::Reg<peci_data_out17::PeciDataOut17Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out17;
#[doc = "PECI_DATA_OUT18 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out18`]
module"]
#[doc(alias = "PECI_DATA_OUT18")]
pub type PeciDataOut18 = crate::Reg<peci_data_out18::PeciDataOut18Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out18;
#[doc = "PECI_DATA_OUT19 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out19`]
module"]
#[doc(alias = "PECI_DATA_OUT19")]
pub type PeciDataOut19 = crate::Reg<peci_data_out19::PeciDataOut19Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out19;
#[doc = "PECI_DATA_OUT20 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out20`]
module"]
#[doc(alias = "PECI_DATA_OUT20")]
pub type PeciDataOut20 = crate::Reg<peci_data_out20::PeciDataOut20Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out20;
#[doc = "PECI_DATA_OUT21 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out21`]
module"]
#[doc(alias = "PECI_DATA_OUT21")]
pub type PeciDataOut21 = crate::Reg<peci_data_out21::PeciDataOut21Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out21;
#[doc = "PECI_DATA_OUT22 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out22`]
module"]
#[doc(alias = "PECI_DATA_OUT22")]
pub type PeciDataOut22 = crate::Reg<peci_data_out22::PeciDataOut22Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out22;
#[doc = "PECI_DATA_OUT23 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out23`]
module"]
#[doc(alias = "PECI_DATA_OUT23")]
pub type PeciDataOut23 = crate::Reg<peci_data_out23::PeciDataOut23Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out23;
#[doc = "PECI_DATA_OUT24 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out24`]
module"]
#[doc(alias = "PECI_DATA_OUT24")]
pub type PeciDataOut24 = crate::Reg<peci_data_out24::PeciDataOut24Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out24;
#[doc = "PECI_DATA_OUT25 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out25`]
module"]
#[doc(alias = "PECI_DATA_OUT25")]
pub type PeciDataOut25 = crate::Reg<peci_data_out25::PeciDataOut25Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out25;
#[doc = "PECI_DATA_OUT26 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out26`]
module"]
#[doc(alias = "PECI_DATA_OUT26")]
pub type PeciDataOut26 = crate::Reg<peci_data_out26::PeciDataOut26Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out26;
#[doc = "PECI_DATA_OUT27 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out27`]
module"]
#[doc(alias = "PECI_DATA_OUT27")]
pub type PeciDataOut27 = crate::Reg<peci_data_out27::PeciDataOut27Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out27;
#[doc = "PECI_DATA_OUT28 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out28`]
module"]
#[doc(alias = "PECI_DATA_OUT28")]
pub type PeciDataOut28 = crate::Reg<peci_data_out28::PeciDataOut28Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out28;
#[doc = "PECI_DATA_OUT29 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out29`]
module"]
#[doc(alias = "PECI_DATA_OUT29")]
pub type PeciDataOut29 = crate::Reg<peci_data_out29::PeciDataOut29Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out29;
#[doc = "PECI_DATA_OUT30 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out30`]
module"]
#[doc(alias = "PECI_DATA_OUT30")]
pub type PeciDataOut30 = crate::Reg<peci_data_out30::PeciDataOut30Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out30;
#[doc = "PECI_DATA_OUT31 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out31`]
module"]
#[doc(alias = "PECI_DATA_OUT31")]
pub type PeciDataOut31 = crate::Reg<peci_data_out31::PeciDataOut31Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out31;
#[doc = "PECI_DATA_OUT32 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out32`]
module"]
#[doc(alias = "PECI_DATA_OUT32")]
pub type PeciDataOut32 = crate::Reg<peci_data_out32::PeciDataOut32Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out32;
#[doc = "PECI_DATA_OUT33 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out33`]
module"]
#[doc(alias = "PECI_DATA_OUT33")]
pub type PeciDataOut33 = crate::Reg<peci_data_out33::PeciDataOut33Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out33;
#[doc = "PECI_DATA_OUT34 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out34`]
module"]
#[doc(alias = "PECI_DATA_OUT34")]
pub type PeciDataOut34 = crate::Reg<peci_data_out34::PeciDataOut34Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out34;
#[doc = "PECI_DATA_OUT35 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out35`]
module"]
#[doc(alias = "PECI_DATA_OUT35")]
pub type PeciDataOut35 = crate::Reg<peci_data_out35::PeciDataOut35Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out35;
#[doc = "PECI_DATA_OUT36 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out36`]
module"]
#[doc(alias = "PECI_DATA_OUT36")]
pub type PeciDataOut36 = crate::Reg<peci_data_out36::PeciDataOut36Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out36;
#[doc = "PECI_DATA_OUT37 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out37`]
module"]
#[doc(alias = "PECI_DATA_OUT37")]
pub type PeciDataOut37 = crate::Reg<peci_data_out37::PeciDataOut37Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out37;
#[doc = "PECI_DATA_OUT38 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out38`]
module"]
#[doc(alias = "PECI_DATA_OUT38")]
pub type PeciDataOut38 = crate::Reg<peci_data_out38::PeciDataOut38Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out38;
#[doc = "PECI_DATA_OUT39 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out39`]
module"]
#[doc(alias = "PECI_DATA_OUT39")]
pub type PeciDataOut39 = crate::Reg<peci_data_out39::PeciDataOut39Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out39;
#[doc = "PECI_DATA_OUT40 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out40`]
module"]
#[doc(alias = "PECI_DATA_OUT40")]
pub type PeciDataOut40 = crate::Reg<peci_data_out40::PeciDataOut40Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out40;
#[doc = "PECI_DATA_OUT41 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out41`]
module"]
#[doc(alias = "PECI_DATA_OUT41")]
pub type PeciDataOut41 = crate::Reg<peci_data_out41::PeciDataOut41Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out41;
#[doc = "PECI_DATA_OUT42 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out42`]
module"]
#[doc(alias = "PECI_DATA_OUT42")]
pub type PeciDataOut42 = crate::Reg<peci_data_out42::PeciDataOut42Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out42;
#[doc = "PECI_DATA_OUT43 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out43`]
module"]
#[doc(alias = "PECI_DATA_OUT43")]
pub type PeciDataOut43 = crate::Reg<peci_data_out43::PeciDataOut43Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out43;
#[doc = "PECI_DATA_OUT44 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out44`]
module"]
#[doc(alias = "PECI_DATA_OUT44")]
pub type PeciDataOut44 = crate::Reg<peci_data_out44::PeciDataOut44Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out44;
#[doc = "PECI_DATA_OUT45 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out45`]
module"]
#[doc(alias = "PECI_DATA_OUT45")]
pub type PeciDataOut45 = crate::Reg<peci_data_out45::PeciDataOut45Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out45;
#[doc = "PECI_DATA_OUT46 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out46`]
module"]
#[doc(alias = "PECI_DATA_OUT46")]
pub type PeciDataOut46 = crate::Reg<peci_data_out46::PeciDataOut46Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out46;
#[doc = "PECI_DATA_OUT47 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out47`]
module"]
#[doc(alias = "PECI_DATA_OUT47")]
pub type PeciDataOut47 = crate::Reg<peci_data_out47::PeciDataOut47Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out47;
#[doc = "PECI_DATA_OUT48 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out48`]
module"]
#[doc(alias = "PECI_DATA_OUT48")]
pub type PeciDataOut48 = crate::Reg<peci_data_out48::PeciDataOut48Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out48;
#[doc = "PECI_DATA_OUT49 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out49`]
module"]
#[doc(alias = "PECI_DATA_OUT49")]
pub type PeciDataOut49 = crate::Reg<peci_data_out49::PeciDataOut49Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out49;
#[doc = "PECI_DATA_OUT50 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out50`]
module"]
#[doc(alias = "PECI_DATA_OUT50")]
pub type PeciDataOut50 = crate::Reg<peci_data_out50::PeciDataOut50Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out50;
#[doc = "PECI_DATA_OUT51 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out51`]
module"]
#[doc(alias = "PECI_DATA_OUT51")]
pub type PeciDataOut51 = crate::Reg<peci_data_out51::PeciDataOut51Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out51;
#[doc = "PECI_DATA_OUT52 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out52`]
module"]
#[doc(alias = "PECI_DATA_OUT52")]
pub type PeciDataOut52 = crate::Reg<peci_data_out52::PeciDataOut52Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out52;
#[doc = "PECI_DATA_OUT53 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out53`]
module"]
#[doc(alias = "PECI_DATA_OUT53")]
pub type PeciDataOut53 = crate::Reg<peci_data_out53::PeciDataOut53Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out53;
#[doc = "PECI_DATA_OUT54 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out54`]
module"]
#[doc(alias = "PECI_DATA_OUT54")]
pub type PeciDataOut54 = crate::Reg<peci_data_out54::PeciDataOut54Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out54;
#[doc = "PECI_DATA_OUT55 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out55`]
module"]
#[doc(alias = "PECI_DATA_OUT55")]
pub type PeciDataOut55 = crate::Reg<peci_data_out55::PeciDataOut55Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out55;
#[doc = "PECI_DATA_OUT56 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out56`]
module"]
#[doc(alias = "PECI_DATA_OUT56")]
pub type PeciDataOut56 = crate::Reg<peci_data_out56::PeciDataOut56Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out56;
#[doc = "PECI_DATA_OUT57 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out57`]
module"]
#[doc(alias = "PECI_DATA_OUT57")]
pub type PeciDataOut57 = crate::Reg<peci_data_out57::PeciDataOut57Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out57;
#[doc = "PECI_DATA_OUT58 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out58`]
module"]
#[doc(alias = "PECI_DATA_OUT58")]
pub type PeciDataOut58 = crate::Reg<peci_data_out58::PeciDataOut58Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out58;
#[doc = "PECI_DATA_OUT59 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out59`]
module"]
#[doc(alias = "PECI_DATA_OUT59")]
pub type PeciDataOut59 = crate::Reg<peci_data_out59::PeciDataOut59Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out59;
#[doc = "PECI_DATA_OUT60 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out60`]
module"]
#[doc(alias = "PECI_DATA_OUT60")]
pub type PeciDataOut60 = crate::Reg<peci_data_out60::PeciDataOut60Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out60;
#[doc = "PECI_DATA_OUT61 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out61`]
module"]
#[doc(alias = "PECI_DATA_OUT61")]
pub type PeciDataOut61 = crate::Reg<peci_data_out61::PeciDataOut61Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out61;
#[doc = "PECI_DATA_OUT62 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out62`]
module"]
#[doc(alias = "PECI_DATA_OUT62")]
pub type PeciDataOut62 = crate::Reg<peci_data_out62::PeciDataOut62Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out62;
#[doc = "PECI_DATA_OUT63 (rw) register accessor: PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peci_data_out63`]
module"]
#[doc(alias = "PECI_DATA_OUT63")]
pub type PeciDataOut63 = crate::Reg<peci_data_out63::PeciDataOut63Spec>;
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)"]
pub mod peci_data_out63;
