#[doc = "Register `FLASH_PRTR_BADDR0-157` reader"]
pub type R = crate::R<FlashPrtrBaddr0157Spec>;
#[doc = "Register `FLASH_PRTR_BADDR0-157` writer"]
pub type W = crate::W<FlashPrtrBaddr0157Spec>;
#[doc = "Field `FLASH_PRTR_BADDR` reader - Flash Protection Base Address"]
pub type FlashPrtrBaddrR = crate::FieldReader<u16>;
#[doc = "Field `FLASH_PRTR_BADDR` writer - Flash Protection Base Address"]
pub type FlashPrtrBaddrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `FRNG_WPR` reader - Flash Range Write Protect"]
pub type FrngWprR = crate::BitReader;
#[doc = "Field `FRNG_WPR` writer - Flash Range Write Protect"]
pub type FrngWprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRNG_RPR` reader - Flash Range Read Protect"]
pub type FrngRprR = crate::BitReader;
#[doc = "Field `FRNG_RPR` writer - Flash Range Read Protect"]
pub type FrngRprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF_PROT_LCK` reader - Target-Attached Flash Protection Lock"]
pub type SafProtLckR = crate::BitReader;
#[doc = "Field `SAF_PROT_LCK` writer - Target-Attached Flash Protection Lock"]
pub type SafProtLckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 12:26 - Flash Protection Base Address"]
    #[inline(always)]
    pub fn flash_prtr_baddr(&self) -> FlashPrtrBaddrR {
        FlashPrtrBaddrR::new(((self.bits >> 12) & 0x7fff) as u16)
    }
    #[doc = "Bit 29 - Flash Range Write Protect"]
    #[inline(always)]
    pub fn frng_wpr(&self) -> FrngWprR {
        FrngWprR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Flash Range Read Protect"]
    #[inline(always)]
    pub fn frng_rpr(&self) -> FrngRprR {
        FrngRprR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Target-Attached Flash Protection Lock"]
    #[inline(always)]
    pub fn saf_prot_lck(&self) -> SafProtLckR {
        SafProtLckR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_PRTR_BADDR0-157")
            .field("flash_prtr_baddr", &self.flash_prtr_baddr())
            .field("frng_wpr", &self.frng_wpr())
            .field("frng_rpr", &self.frng_rpr())
            .field("saf_prot_lck", &self.saf_prot_lck())
            .finish()
    }
}
impl W {
    #[doc = "Bits 12:26 - Flash Protection Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn flash_prtr_baddr(&mut self) -> FlashPrtrBaddrW<FlashPrtrBaddr0157Spec> {
        FlashPrtrBaddrW::new(self, 12)
    }
    #[doc = "Bit 29 - Flash Range Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn frng_wpr(&mut self) -> FrngWprW<FlashPrtrBaddr0157Spec> {
        FrngWprW::new(self, 29)
    }
    #[doc = "Bit 30 - Flash Range Read Protect"]
    #[inline(always)]
    #[must_use]
    pub fn frng_rpr(&mut self) -> FrngRprW<FlashPrtrBaddr0157Spec> {
        FrngRprW::new(self, 30)
    }
    #[doc = "Bit 31 - Target-Attached Flash Protection Lock"]
    #[inline(always)]
    #[must_use]
    pub fn saf_prot_lck(&mut self) -> SafProtLckW<FlashPrtrBaddr0157Spec> {
        SafProtLckW::new(self, 31)
    }
}
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0157::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0157::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashPrtrBaddr0157Spec;
impl crate::RegisterSpec for FlashPrtrBaddr0157Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_prtr_baddr0157::R`](R) reader structure"]
impl crate::Readable for FlashPrtrBaddr0157Spec {}
#[doc = "`write(|w| ..)` method takes [`flash_prtr_baddr0157::W`](W) writer structure"]
impl crate::Writable for FlashPrtrBaddr0157Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_PRTR_BADDR0-157 to value 0"]
impl crate::Resettable for FlashPrtrBaddr0157Spec {
    const RESET_VALUE: u32 = 0;
}
