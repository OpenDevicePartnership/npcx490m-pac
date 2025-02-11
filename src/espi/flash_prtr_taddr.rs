#[doc = "Register `FLASH_PRTR_TADDR%s` reader"]
pub type R = crate::R<FlashPrtrTaddrSpec>;
#[doc = "Register `FLASH_PRTR_TADDR%s` writer"]
pub type W = crate::W<FlashPrtrTaddrSpec>;
#[doc = "Field `FLASH_PRT_TADDR` reader - Flash Protection Top Address"]
pub type FlashPrtTaddrR = crate::FieldReader<u16>;
#[doc = "Field `FLASH_PRT_TADDR` writer - Flash Protection Top Address"]
pub type FlashPrtTaddrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 12:26 - Flash Protection Top Address"]
    #[inline(always)]
    pub fn flash_prt_taddr(&self) -> FlashPrtTaddrR {
        FlashPrtTaddrR::new(((self.bits >> 12) & 0x7fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_PRTR_TADDR")
            .field("flash_prt_taddr", &self.flash_prt_taddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 12:26 - Flash Protection Top Address"]
    #[inline(always)]
    pub fn flash_prt_taddr(&mut self) -> FlashPrtTaddrW<FlashPrtrTaddrSpec> {
        FlashPrtTaddrW::new(self, 12)
    }
}
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashPrtrTaddrSpec;
impl crate::RegisterSpec for FlashPrtrTaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_prtr_taddr::R`](R) reader structure"]
impl crate::Readable for FlashPrtrTaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_prtr_taddr::W`](W) writer structure"]
impl crate::Writable for FlashPrtrTaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_PRTR_TADDR%s to value 0"]
impl crate::Resettable for FlashPrtrTaddrSpec {
    const RESET_VALUE: u32 = 0;
}
