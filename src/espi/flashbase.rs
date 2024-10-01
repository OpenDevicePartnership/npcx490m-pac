#[doc = "Register `FLASHBASE` reader"]
pub type R = crate::R<FlashbaseSpec>;
#[doc = "Register `FLASHBASE` writer"]
pub type W = crate::W<FlashbaseSpec>;
#[doc = "Field `FLBASE_LCK` reader - FLASHBASE Lock"]
pub type FlbaseLckR = crate::BitReader;
#[doc = "Field `FLBASE_LCK` writer - FLASHBASE Lock"]
pub type FlbaseLckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLBASE_ADDR` reader - Flash Base Address"]
pub type FlbaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `FLBASE_ADDR` writer - Flash Base Address"]
pub type FlbaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - FLASHBASE Lock"]
    #[inline(always)]
    pub fn flbase_lck(&self) -> FlbaseLckR {
        FlbaseLckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 12:26 - Flash Base Address"]
    #[inline(always)]
    pub fn flbase_addr(&self) -> FlbaseAddrR {
        FlbaseAddrR::new(((self.bits >> 12) & 0x7fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHBASE")
            .field("flbase_lck", &self.flbase_lck())
            .field("flbase_addr", &self.flbase_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FLASHBASE Lock"]
    #[inline(always)]
    #[must_use]
    pub fn flbase_lck(&mut self) -> FlbaseLckW<FlashbaseSpec> {
        FlbaseLckW::new(self, 0)
    }
    #[doc = "Bits 12:26 - Flash Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn flbase_addr(&mut self) -> FlbaseAddrW<FlashbaseSpec> {
        FlbaseAddrW::new(self, 12)
    }
}
#[doc = "Flash Base Register (FLASHBASE)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashbaseSpec;
impl crate::RegisterSpec for FlashbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashbase::R`](R) reader structure"]
impl crate::Readable for FlashbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`flashbase::W`](W) writer structure"]
impl crate::Writable for FlashbaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHBASE to value 0"]
impl crate::Resettable for FlashbaseSpec {
    const RESET_VALUE: u32 = 0;
}
