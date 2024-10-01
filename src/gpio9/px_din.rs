#[doc = "Register `PxDIN` reader"]
pub type R = crate::R<PxDinSpec>;
#[doc = "Register `PxDIN` writer"]
pub type W = crate::W<PxDinSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port GPIOx Data In Register (PxDIN)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_din::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_din::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PxDinSpec;
impl crate::RegisterSpec for PxDinSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`px_din::R`](R) reader structure"]
impl crate::Readable for PxDinSpec {}
#[doc = "`write(|w| ..)` method takes [`px_din::W`](W) writer structure"]
impl crate::Writable for PxDinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PxDIN to value 0"]
impl crate::Resettable for PxDinSpec {
    const RESET_VALUE: u8 = 0;
}
