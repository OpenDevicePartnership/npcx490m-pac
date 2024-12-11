#[doc = "Register `PERCTL` reader"]
pub type R = crate::R<PerctlSpec>;
#[doc = "Register `PERCTL` writer"]
pub type W = crate::W<PerctlSpec>;
#[doc = "Field `PER_PC_FREE` reader - Peripheral Channel Posted/Completion Receive Queue Free"]
pub type PerPcFreeR = crate::BitReader;
#[doc = "Field `PER_PC_FREE` writer - Peripheral Channel Posted/Completion Receive Queue Free"]
pub type PerPcFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMDMA_TR_SL` reader - Bus Master DMA Transmit/Receive Select"]
pub type BmdmaTrSlR = crate::BitReader;
#[doc = "Field `BMDMA_TR_SL` writer - Bus Master DMA Transmit/Receive Select"]
pub type BmdmaTrSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMSTRPHDR` reader - Read Bus Master Strip Header"]
pub type BmstrphdrR = crate::BitReader;
#[doc = "Field `BMSTRPHDR` writer - Read Bus Master Strip Header"]
pub type BmstrphdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMDMATHRESH` reader - Read Bus Master DMA Request Threshold"]
pub type BmdmathreshR = crate::FieldReader;
#[doc = "Field `BMDMATHRESH` writer - Read Bus Master DMA Request Threshold"]
pub type BmdmathreshW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BMBURSTSIZE` reader - Bus Master Burst Mode Read Transfer Size"]
pub type BmburstsizeR = crate::FieldReader;
#[doc = "Field `BMBURSTSIZE` writer - Bus Master Burst Mode Read Transfer Size"]
pub type BmburstsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSTPBUFHEADS` reader - Reset Peripheral Buffer Heads"]
pub type RstpbufheadsR = crate::BitReader;
#[doc = "Field `RSTPBUFHEADS` writer - Reset Peripheral Buffer Heads"]
pub type RstpbufheadsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM64_ACCESS` reader - Memory 64-Bit Access Select"]
pub type Mem64AccessR = crate::BitReader;
#[doc = "Field `MEM64_ACCESS` writer - Memory 64-Bit Access Select"]
pub type Mem64AccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTPSBUFHEAD` reader - Reset Peripheral Target Buffer Head"]
pub type RstpsbufheadR = crate::BitReader;
#[doc = "Field `RSTPSBUFHEAD` writer - Reset Peripheral Target Buffer Head"]
pub type RstpsbufheadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMBRSTEN` reader - Bus Master Burst Mode Read Enable"]
pub type BmbrstenR = crate::BitReader;
#[doc = "Field `BMBRSTEN` writer - Bus Master Burst Mode Read Enable"]
pub type BmbrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMBURST_BFULL` reader - Bus Master Burst Mode Read Receive Buffer Full"]
pub type BmburstBfullR = crate::BitReader;
#[doc = "Field `BMBURST_BFULL` writer - Bus Master Burst Mode Read Receive Buffer Full"]
pub type BmburstBfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BM_NP_AVAIL` reader - Bus Master Non-Posted Available"]
pub type BmNpAvailR = crate::BitReader;
#[doc = "Field `BM_NP_AVAIL` writer - Bus Master Non-Posted Available"]
pub type BmNpAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BM_PC_AVAIL` reader - Bus Master Posted Available"]
pub type BmPcAvailR = crate::BitReader;
#[doc = "Field `BM_PC_AVAIL` writer - Bus Master Posted Available"]
pub type BmPcAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BM_MSG_AVAIL` reader - Bus Master Message Available"]
pub type BmMsgAvailR = crate::BitReader;
#[doc = "Field `BM_MSG_AVAIL` writer - Bus Master Message Available"]
pub type BmMsgAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMPKT_LEN` reader - Bus Master Packet Length"]
pub type BmpktLenR = crate::FieldReader;
#[doc = "Field `BMPKT_LEN` writer - Bus Master Packet Length"]
pub type BmpktLenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Peripheral Channel Posted/Completion Receive Queue Free"]
    #[inline(always)]
    pub fn per_pc_free(&self) -> PerPcFreeR {
        PerPcFreeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus Master DMA Transmit/Receive Select"]
    #[inline(always)]
    pub fn bmdma_tr_sl(&self) -> BmdmaTrSlR {
        BmdmaTrSlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read Bus Master Strip Header"]
    #[inline(always)]
    pub fn bmstrphdr(&self) -> BmstrphdrR {
        BmstrphdrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Read Bus Master DMA Request Threshold"]
    #[inline(always)]
    pub fn bmdmathresh(&self) -> BmdmathreshR {
        BmdmathreshR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:12 - Bus Master Burst Mode Read Transfer Size"]
    #[inline(always)]
    pub fn bmburstsize(&self) -> BmburstsizeR {
        BmburstsizeR::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - Reset Peripheral Buffer Heads"]
    #[inline(always)]
    pub fn rstpbufheads(&self) -> RstpbufheadsR {
        RstpbufheadsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Memory 64-Bit Access Select"]
    #[inline(always)]
    pub fn mem64_access(&self) -> Mem64AccessR {
        Mem64AccessR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset Peripheral Target Buffer Head"]
    #[inline(always)]
    pub fn rstpsbufhead(&self) -> RstpsbufheadR {
        RstpsbufheadR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bus Master Burst Mode Read Enable"]
    #[inline(always)]
    pub fn bmbrsten(&self) -> BmbrstenR {
        BmbrstenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bus Master Burst Mode Read Receive Buffer Full"]
    #[inline(always)]
    pub fn bmburst_bfull(&self) -> BmburstBfullR {
        BmburstBfullR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus Master Non-Posted Available"]
    #[inline(always)]
    pub fn bm_np_avail(&self) -> BmNpAvailR {
        BmNpAvailR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bus Master Posted Available"]
    #[inline(always)]
    pub fn bm_pc_avail(&self) -> BmPcAvailR {
        BmPcAvailR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bus Master Message Available"]
    #[inline(always)]
    pub fn bm_msg_avail(&self) -> BmMsgAvailR {
        BmMsgAvailR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Bus Master Packet Length"]
    #[inline(always)]
    pub fn bmpkt_len(&self) -> BmpktLenR {
        BmpktLenR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERCTL")
            .field("per_pc_free", &self.per_pc_free())
            .field("bmdma_tr_sl", &self.bmdma_tr_sl())
            .field("bmstrphdr", &self.bmstrphdr())
            .field("bmdmathresh", &self.bmdmathresh())
            .field("bmburstsize", &self.bmburstsize())
            .field("rstpbufheads", &self.rstpbufheads())
            .field("mem64_access", &self.mem64_access())
            .field("rstpsbufhead", &self.rstpsbufhead())
            .field("bmbrsten", &self.bmbrsten())
            .field("bmburst_bfull", &self.bmburst_bfull())
            .field("bm_np_avail", &self.bm_np_avail())
            .field("bm_pc_avail", &self.bm_pc_avail())
            .field("bm_msg_avail", &self.bm_msg_avail())
            .field("bmpkt_len", &self.bmpkt_len())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Channel Posted/Completion Receive Queue Free"]
    #[inline(always)]
    pub fn per_pc_free(&mut self) -> PerPcFreeW<PerctlSpec> {
        PerPcFreeW::new(self, 0)
    }
    #[doc = "Bit 1 - Bus Master DMA Transmit/Receive Select"]
    #[inline(always)]
    pub fn bmdma_tr_sl(&mut self) -> BmdmaTrSlW<PerctlSpec> {
        BmdmaTrSlW::new(self, 1)
    }
    #[doc = "Bit 2 - Read Bus Master Strip Header"]
    #[inline(always)]
    pub fn bmstrphdr(&mut self) -> BmstrphdrW<PerctlSpec> {
        BmstrphdrW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Read Bus Master DMA Request Threshold"]
    #[inline(always)]
    pub fn bmdmathresh(&mut self) -> BmdmathreshW<PerctlSpec> {
        BmdmathreshW::new(self, 3)
    }
    #[doc = "Bits 5:12 - Bus Master Burst Mode Read Transfer Size"]
    #[inline(always)]
    pub fn bmburstsize(&mut self) -> BmburstsizeW<PerctlSpec> {
        BmburstsizeW::new(self, 5)
    }
    #[doc = "Bit 13 - Reset Peripheral Buffer Heads"]
    #[inline(always)]
    pub fn rstpbufheads(&mut self) -> RstpbufheadsW<PerctlSpec> {
        RstpbufheadsW::new(self, 13)
    }
    #[doc = "Bit 14 - Memory 64-Bit Access Select"]
    #[inline(always)]
    pub fn mem64_access(&mut self) -> Mem64AccessW<PerctlSpec> {
        Mem64AccessW::new(self, 14)
    }
    #[doc = "Bit 15 - Reset Peripheral Target Buffer Head"]
    #[inline(always)]
    pub fn rstpsbufhead(&mut self) -> RstpsbufheadW<PerctlSpec> {
        RstpsbufheadW::new(self, 15)
    }
    #[doc = "Bit 16 - Bus Master Burst Mode Read Enable"]
    #[inline(always)]
    pub fn bmbrsten(&mut self) -> BmbrstenW<PerctlSpec> {
        BmbrstenW::new(self, 16)
    }
    #[doc = "Bit 17 - Bus Master Burst Mode Read Receive Buffer Full"]
    #[inline(always)]
    pub fn bmburst_bfull(&mut self) -> BmburstBfullW<PerctlSpec> {
        BmburstBfullW::new(self, 17)
    }
    #[doc = "Bit 19 - Bus Master Non-Posted Available"]
    #[inline(always)]
    pub fn bm_np_avail(&mut self) -> BmNpAvailW<PerctlSpec> {
        BmNpAvailW::new(self, 19)
    }
    #[doc = "Bit 20 - Bus Master Posted Available"]
    #[inline(always)]
    pub fn bm_pc_avail(&mut self) -> BmPcAvailW<PerctlSpec> {
        BmPcAvailW::new(self, 20)
    }
    #[doc = "Bit 21 - Bus Master Message Available"]
    #[inline(always)]
    pub fn bm_msg_avail(&mut self) -> BmMsgAvailW<PerctlSpec> {
        BmMsgAvailW::new(self, 21)
    }
    #[doc = "Bits 24:31 - Bus Master Packet Length"]
    #[inline(always)]
    pub fn bmpkt_len(&mut self) -> BmpktLenW<PerctlSpec> {
        BmpktLenW::new(self, 24)
    }
}
#[doc = "Peripheral Channel Control Register (PERCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`perctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerctlSpec;
impl crate::RegisterSpec for PerctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perctl::R`](R) reader structure"]
impl crate::Readable for PerctlSpec {}
#[doc = "`write(|w| ..)` method takes [`perctl::W`](W) writer structure"]
impl crate::Writable for PerctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERCTL to value 0"]
impl crate::Resettable for PerctlSpec {
    const RESET_VALUE: u32 = 0;
}
