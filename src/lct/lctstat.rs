#[doc = "Register `LCTSTAT` reader"]
pub type R = crate::R<LctstatSpec>;
#[doc = "Register `LCTSTAT` writer"]
pub type W = crate::W<LctstatSpec>;
#[doc = "Field `LCTEVST` reader - LCT Event Status"]
pub type LctevstR = crate::BitReader;
#[doc = "Field `LCTEVST` writer - LCT Event Status"]
pub type LctevstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCT Event Status"]
    #[inline(always)]
    pub fn lctevst(&self) -> LctevstR {
        LctevstR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCTSTAT")
            .field("lctevst", &self.lctevst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - LCT Event Status"]
    #[inline(always)]
    pub fn lctevst(&mut self) -> LctevstW<LctstatSpec> {
        LctevstW::new(self, 0)
    }
}
#[doc = "LCT Status Register (LCTSTAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LctstatSpec;
impl crate::RegisterSpec for LctstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lctstat::R`](R) reader structure"]
impl crate::Readable for LctstatSpec {}
#[doc = "`write(|w| ..)` method takes [`lctstat::W`](W) writer structure"]
impl crate::Writable for LctstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LCTSTAT to value 0"]
impl crate::Resettable for LctstatSpec {
    const RESET_VALUE: u8 = 0;
}
