#[doc = "Register `UMA_DMM` reader"]
pub type R = crate::R<UmaDmmSpec>;
#[doc = "Register `UMA_DMM` writer"]
pub type W = crate::W<UmaDmmSpec>;
#[doc = "Field `DMCSIZ` reader - Dummy Cycles Size"]
pub type DmcsizR = crate::FieldReader;
#[doc = "Field `DMCSIZ` writer - Dummy Cycles Size"]
pub type DmcsizW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Dummy Cycles Size"]
    #[inline(always)]
    pub fn dmcsiz(&self) -> DmcsizR {
        DmcsizR::new(self.bits & 0x3f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UMA_DMM")
            .field("dmcsiz", &self.dmcsiz())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Dummy Cycles Size"]
    #[inline(always)]
    pub fn dmcsiz(&mut self) -> DmcsizW<UmaDmmSpec> {
        DmcsizW::new(self, 0)
    }
}
#[doc = "UMA Dummy Register (UMA_DMM)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_dmm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_dmm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UmaDmmSpec;
impl crate::RegisterSpec for UmaDmmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uma_dmm::R`](R) reader structure"]
impl crate::Readable for UmaDmmSpec {}
#[doc = "`write(|w| ..)` method takes [`uma_dmm::W`](W) writer structure"]
impl crate::Writable for UmaDmmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UMA_DMM to value 0"]
impl crate::Resettable for UmaDmmSpec {
    const RESET_VALUE: u8 = 0;
}
