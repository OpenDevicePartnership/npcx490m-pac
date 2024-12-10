#[doc = "Register `STATUS_IMG` reader"]
pub type R = crate::R<StatusImgSpec>;
#[doc = "Register `STATUS_IMG` writer"]
pub type W = crate::W<StatusImgSpec>;
#[doc = "Field `PC_FREE` reader - Peripheral Channel Posted Receive Free"]
pub type PcFreeR = crate::BitReader;
#[doc = "Field `PC_FREE` writer - Peripheral Channel Posted Receive Free"]
pub type PcFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NP_FREE` reader - Peripheral Channel Non-Posted Receive. Free"]
pub type NpFreeR = crate::BitReader;
#[doc = "Field `NP_FREE` writer - Peripheral Channel Non-Posted Receive. Free"]
pub type NpFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VWIRE_FREE` reader - Virtual Wire Channel Receive Free"]
pub type VwireFreeR = crate::BitReader;
#[doc = "Field `VWIRE_FREE` writer - Virtual Wire Channel Receive Free"]
pub type VwireFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOB_FREE` reader - OOB Channel Posted Receive Free"]
pub type OobFreeR = crate::BitReader;
#[doc = "Field `OOB_FREE` writer - OOB Channel Posted Receive Free"]
pub type OobFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC_AVAIL` reader - Peripheral Channel Posted/Completion Transmit. Available"]
pub type PcAvailR = crate::BitReader;
#[doc = "Field `PC_AVAIL` writer - Peripheral Channel Posted/Completion Transmit. Available"]
pub type PcAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NP_AVAIL` reader - Peripheral Channel Non-Posted Transmit Available"]
pub type NpAvailR = crate::BitReader;
#[doc = "Field `NP_AVAIL` writer - Peripheral Channel Non-Posted Transmit Available"]
pub type NpAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VWIRE_AVAIL` reader - Virtual Wire Channel Transmit Available"]
pub type VwireAvailR = crate::BitReader;
#[doc = "Field `VWIRE_AVAIL` writer - Virtual Wire Channel Transmit Available"]
pub type VwireAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOB_AVAIL` reader - OOB Channel Posted Transmit. Available"]
pub type OobAvailR = crate::BitReader;
#[doc = "Field `OOB_AVAIL` writer - OOB Channel Posted Transmit. Available"]
pub type OobAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_C_FREE` reader - Flash Channel Completion Receive Available"]
pub type FlashCFreeR = crate::BitReader;
#[doc = "Field `FLASH_C_FREE` writer - Flash Channel Completion Receive Available"]
pub type FlashCFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_NP_FREE` reader - Flash Channel Non-Posted Receive Free"]
pub type FlashNpFreeR = crate::BitReader;
#[doc = "Field `FLASH_NP_FREE` writer - Flash Channel Non-Posted Receive Free"]
pub type FlashNpFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_C_AVAIL` reader - Flash Channel Completion Transmit Available"]
pub type FlashCAvailR = crate::BitReader;
#[doc = "Field `FLASH_C_AVAIL` writer - Flash Channel Completion Transmit Available"]
pub type FlashCAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_NP_AVAIL` reader - Flash Channel Non-Posted Transmit Available"]
pub type FlashNpAvailR = crate::BitReader;
#[doc = "Field `FLASH_NP_AVAIL` writer - Flash Channel Non-Posted Transmit Available"]
pub type FlashNpAvailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral Channel Posted Receive Free"]
    #[inline(always)]
    pub fn pc_free(&self) -> PcFreeR {
        PcFreeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Channel Non-Posted Receive. Free"]
    #[inline(always)]
    pub fn np_free(&self) -> NpFreeR {
        NpFreeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Virtual Wire Channel Receive Free"]
    #[inline(always)]
    pub fn vwire_free(&self) -> VwireFreeR {
        VwireFreeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OOB Channel Posted Receive Free"]
    #[inline(always)]
    pub fn oob_free(&self) -> OobFreeR {
        OobFreeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral Channel Posted/Completion Transmit. Available"]
    #[inline(always)]
    pub fn pc_avail(&self) -> PcAvailR {
        PcAvailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral Channel Non-Posted Transmit Available"]
    #[inline(always)]
    pub fn np_avail(&self) -> NpAvailR {
        NpAvailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Virtual Wire Channel Transmit Available"]
    #[inline(always)]
    pub fn vwire_avail(&self) -> VwireAvailR {
        VwireAvailR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OOB Channel Posted Transmit. Available"]
    #[inline(always)]
    pub fn oob_avail(&self) -> OobAvailR {
        OobAvailR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Channel Completion Receive Available"]
    #[inline(always)]
    pub fn flash_c_free(&self) -> FlashCFreeR {
        FlashCFreeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash Channel Non-Posted Receive Free"]
    #[inline(always)]
    pub fn flash_np_free(&self) -> FlashNpFreeR {
        FlashNpFreeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Flash Channel Completion Transmit Available"]
    #[inline(always)]
    pub fn flash_c_avail(&self) -> FlashCAvailR {
        FlashCAvailR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flash Channel Non-Posted Transmit Available"]
    #[inline(always)]
    pub fn flash_np_avail(&self) -> FlashNpAvailR {
        FlashNpAvailR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_IMG")
            .field("pc_free", &self.pc_free())
            .field("np_free", &self.np_free())
            .field("vwire_free", &self.vwire_free())
            .field("oob_free", &self.oob_free())
            .field("pc_avail", &self.pc_avail())
            .field("np_avail", &self.np_avail())
            .field("vwire_avail", &self.vwire_avail())
            .field("oob_avail", &self.oob_avail())
            .field("flash_c_free", &self.flash_c_free())
            .field("flash_np_free", &self.flash_np_free())
            .field("flash_c_avail", &self.flash_c_avail())
            .field("flash_np_avail", &self.flash_np_avail())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Channel Posted Receive Free"]
    #[inline(always)]
    pub fn pc_free(&mut self) -> PcFreeW<StatusImgSpec> {
        PcFreeW::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral Channel Non-Posted Receive. Free"]
    #[inline(always)]
    pub fn np_free(&mut self) -> NpFreeW<StatusImgSpec> {
        NpFreeW::new(self, 1)
    }
    #[doc = "Bit 2 - Virtual Wire Channel Receive Free"]
    #[inline(always)]
    pub fn vwire_free(&mut self) -> VwireFreeW<StatusImgSpec> {
        VwireFreeW::new(self, 2)
    }
    #[doc = "Bit 3 - OOB Channel Posted Receive Free"]
    #[inline(always)]
    pub fn oob_free(&mut self) -> OobFreeW<StatusImgSpec> {
        OobFreeW::new(self, 3)
    }
    #[doc = "Bit 4 - Peripheral Channel Posted/Completion Transmit. Available"]
    #[inline(always)]
    pub fn pc_avail(&mut self) -> PcAvailW<StatusImgSpec> {
        PcAvailW::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral Channel Non-Posted Transmit Available"]
    #[inline(always)]
    pub fn np_avail(&mut self) -> NpAvailW<StatusImgSpec> {
        NpAvailW::new(self, 5)
    }
    #[doc = "Bit 6 - Virtual Wire Channel Transmit Available"]
    #[inline(always)]
    pub fn vwire_avail(&mut self) -> VwireAvailW<StatusImgSpec> {
        VwireAvailW::new(self, 6)
    }
    #[doc = "Bit 7 - OOB Channel Posted Transmit. Available"]
    #[inline(always)]
    pub fn oob_avail(&mut self) -> OobAvailW<StatusImgSpec> {
        OobAvailW::new(self, 7)
    }
    #[doc = "Bit 8 - Flash Channel Completion Receive Available"]
    #[inline(always)]
    pub fn flash_c_free(&mut self) -> FlashCFreeW<StatusImgSpec> {
        FlashCFreeW::new(self, 8)
    }
    #[doc = "Bit 9 - Flash Channel Non-Posted Receive Free"]
    #[inline(always)]
    pub fn flash_np_free(&mut self) -> FlashNpFreeW<StatusImgSpec> {
        FlashNpFreeW::new(self, 9)
    }
    #[doc = "Bit 12 - Flash Channel Completion Transmit Available"]
    #[inline(always)]
    pub fn flash_c_avail(&mut self) -> FlashCAvailW<StatusImgSpec> {
        FlashCAvailW::new(self, 12)
    }
    #[doc = "Bit 13 - Flash Channel Non-Posted Transmit Available"]
    #[inline(always)]
    pub fn flash_np_avail(&mut self) -> FlashNpAvailW<StatusImgSpec> {
        FlashNpAvailW::new(self, 13)
    }
}
#[doc = "Status Image Register (STATUS_IMG)\n\nYou can [`read`](crate::Reg::read) this register and get [`status_img::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_img::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusImgSpec;
impl crate::RegisterSpec for StatusImgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status_img::R`](R) reader structure"]
impl crate::Readable for StatusImgSpec {}
#[doc = "`write(|w| ..)` method takes [`status_img::W`](W) writer structure"]
impl crate::Writable for StatusImgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets STATUS_IMG to value 0"]
impl crate::Resettable for StatusImgSpec {
    const RESET_VALUE: u16 = 0;
}
