#[doc = "Register `FLASHCTL` reader"]
pub type R = crate::R<FlashctlSpec>;
#[doc = "Register `FLASHCTL` writer"]
pub type W = crate::W<FlashctlSpec>;
#[doc = "Field `FLASH_ACC_NP_FREE` reader - Flash Access Non-Posted Queue Free"]
pub type FlashAccNpFreeR = crate::BitReader;
#[doc = "Field `FLASH_ACC_NP_FREE` writer - Flash Access Non-Posted Queue Free"]
pub type FlashAccNpFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_ACC_TX_AVAIL` reader - Flash Access Transmit Queue Available"]
pub type FlashAccTxAvailR = crate::BitReader;
#[doc = "Field `FLASH_ACC_TX_AVAIL` writer - Flash Access Transmit Queue Available"]
pub type FlashAccTxAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRPHDR` reader - Strip Header"]
pub type StrphdrR = crate::BitReader;
#[doc = "Field `STRPHDR` writer - Strip Header"]
pub type StrphdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATHRESH` reader - DMA Request Threshold"]
pub type DmathreshR = crate::FieldReader;
#[doc = "Field `DMATHRESH` writer - DMA Request Threshold"]
pub type DmathreshW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMTSIZE` reader - Automatic Mode Transfer Size"]
pub type AmtsizeR = crate::FieldReader;
#[doc = "Field `AMTSIZE` writer - Automatic Mode Transfer Size"]
pub type AmtsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSTBUFHEADS` reader - Reset Buffer Heads"]
pub type RstbufheadsR = crate::BitReader;
#[doc = "Field `RSTBUFHEADS` writer - Reset Buffer Heads"]
pub type RstbufheadsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMTEN` reader - Automatic Mode Enable"]
pub type AmtenR = crate::BitReader;
#[doc = "Field `AMTEN` writer - Automatic Mode Enable"]
pub type AmtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMT_BFULL` reader - Automatic Mode Receive Buffer Full"]
pub type AmtBfullR = crate::BitReader;
#[doc = "Field `AMT_BFULL` writer - Automatic Mode Receive Buffer Full"]
pub type AmtBfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF_AUTO_READ` reader - Target-Attached Flash Automatic Read"]
pub type SafAutoReadR = crate::BitReader;
#[doc = "Field `SAF_AUTO_READ` writer - Target-Attached Flash Automatic Read"]
pub type SafAutoReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_RD_DIS_CTL` reader - Disable Automatic Read Control"]
pub type AutoRdDisCtlR = crate::BitReader;
#[doc = "Field `AUTO_RD_DIS_CTL` writer - Disable Automatic Read Control"]
pub type AutoRdDisCtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_FLASH_NP_FREE` reader - Block FLASH_NP_FREE"]
pub type BlkFlashNpFreeR = crate::BitReader;
#[doc = "Field `BLK_FLASH_NP_FREE` writer - Block FLASH_NP_FREE"]
pub type BlkFlashNpFreeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash Access Non-Posted Queue Free"]
    #[inline(always)]
    pub fn flash_acc_np_free(&self) -> FlashAccNpFreeR {
        FlashAccNpFreeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Access Transmit Queue Available"]
    #[inline(always)]
    pub fn flash_acc_tx_avail(&self) -> FlashAccTxAvailR {
        FlashAccTxAvailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Strip Header"]
    #[inline(always)]
    pub fn strphdr(&self) -> StrphdrR {
        StrphdrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DMA Request Threshold"]
    #[inline(always)]
    pub fn dmathresh(&self) -> DmathreshR {
        DmathreshR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:12 - Automatic Mode Transfer Size"]
    #[inline(always)]
    pub fn amtsize(&self) -> AmtsizeR {
        AmtsizeR::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - Reset Buffer Heads"]
    #[inline(always)]
    pub fn rstbufheads(&self) -> RstbufheadsR {
        RstbufheadsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Automatic Mode Enable"]
    #[inline(always)]
    pub fn amten(&self) -> AmtenR {
        AmtenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Automatic Mode Receive Buffer Full"]
    #[inline(always)]
    pub fn amt_bfull(&self) -> AmtBfullR {
        AmtBfullR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Target-Attached Flash Automatic Read"]
    #[inline(always)]
    pub fn saf_auto_read(&self) -> SafAutoReadR {
        SafAutoReadR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Disable Automatic Read Control"]
    #[inline(always)]
    pub fn auto_rd_dis_ctl(&self) -> AutoRdDisCtlR {
        AutoRdDisCtlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Block FLASH_NP_FREE"]
    #[inline(always)]
    pub fn blk_flash_np_free(&self) -> BlkFlashNpFreeR {
        BlkFlashNpFreeR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHCTL")
            .field("flash_acc_np_free", &self.flash_acc_np_free())
            .field("flash_acc_tx_avail", &self.flash_acc_tx_avail())
            .field("strphdr", &self.strphdr())
            .field("dmathresh", &self.dmathresh())
            .field("amtsize", &self.amtsize())
            .field("rstbufheads", &self.rstbufheads())
            .field("amten", &self.amten())
            .field("amt_bfull", &self.amt_bfull())
            .field("saf_auto_read", &self.saf_auto_read())
            .field("auto_rd_dis_ctl", &self.auto_rd_dis_ctl())
            .field("blk_flash_np_free", &self.blk_flash_np_free())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flash Access Non-Posted Queue Free"]
    #[inline(always)]
    pub fn flash_acc_np_free(&mut self) -> FlashAccNpFreeW<FlashctlSpec> {
        FlashAccNpFreeW::new(self, 0)
    }
    #[doc = "Bit 1 - Flash Access Transmit Queue Available"]
    #[inline(always)]
    pub fn flash_acc_tx_avail(&mut self) -> FlashAccTxAvailW<FlashctlSpec> {
        FlashAccTxAvailW::new(self, 1)
    }
    #[doc = "Bit 2 - Strip Header"]
    #[inline(always)]
    pub fn strphdr(&mut self) -> StrphdrW<FlashctlSpec> {
        StrphdrW::new(self, 2)
    }
    #[doc = "Bits 3:4 - DMA Request Threshold"]
    #[inline(always)]
    pub fn dmathresh(&mut self) -> DmathreshW<FlashctlSpec> {
        DmathreshW::new(self, 3)
    }
    #[doc = "Bits 5:12 - Automatic Mode Transfer Size"]
    #[inline(always)]
    pub fn amtsize(&mut self) -> AmtsizeW<FlashctlSpec> {
        AmtsizeW::new(self, 5)
    }
    #[doc = "Bit 13 - Reset Buffer Heads"]
    #[inline(always)]
    pub fn rstbufheads(&mut self) -> RstbufheadsW<FlashctlSpec> {
        RstbufheadsW::new(self, 13)
    }
    #[doc = "Bit 16 - Automatic Mode Enable"]
    #[inline(always)]
    pub fn amten(&mut self) -> AmtenW<FlashctlSpec> {
        AmtenW::new(self, 16)
    }
    #[doc = "Bit 17 - Automatic Mode Receive Buffer Full"]
    #[inline(always)]
    pub fn amt_bfull(&mut self) -> AmtBfullW<FlashctlSpec> {
        AmtBfullW::new(self, 17)
    }
    #[doc = "Bit 18 - Target-Attached Flash Automatic Read"]
    #[inline(always)]
    pub fn saf_auto_read(&mut self) -> SafAutoReadW<FlashctlSpec> {
        SafAutoReadW::new(self, 18)
    }
    #[doc = "Bit 19 - Disable Automatic Read Control"]
    #[inline(always)]
    pub fn auto_rd_dis_ctl(&mut self) -> AutoRdDisCtlW<FlashctlSpec> {
        AutoRdDisCtlW::new(self, 19)
    }
    #[doc = "Bit 20 - Block FLASH_NP_FREE"]
    #[inline(always)]
    pub fn blk_flash_np_free(&mut self) -> BlkFlashNpFreeW<FlashctlSpec> {
        BlkFlashNpFreeW::new(self, 20)
    }
}
#[doc = "Flash Channel Control Register (FLASHCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlSpec;
impl crate::RegisterSpec for FlashctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl::R`](R) reader structure"]
impl crate::Readable for FlashctlSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl::W`](W) writer structure"]
impl crate::Writable for FlashctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHCTL to value 0"]
impl crate::Resettable for FlashctlSpec {
    const RESET_VALUE: u32 = 0;
}
