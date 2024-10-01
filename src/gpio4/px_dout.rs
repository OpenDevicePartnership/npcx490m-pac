#[doc = "Register `PxDOUT` reader"]
pub type R = crate::R<PxDoutSpec>;
#[doc = "Register `PxDOUT` writer"]
pub type W = crate::W<PxDoutSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port GPIOx Data Out Register (PxDOUT)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_dout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_dout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PxDoutSpec;
impl crate::RegisterSpec for PxDoutSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`px_dout::R`](R) reader structure"]
impl crate::Readable for PxDoutSpec {}
#[doc = "`write(|w| ..)` method takes [`px_dout::W`](W) writer structure"]
impl crate::Writable for PxDoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PxDOUT to value 0"]
impl crate::Resettable for PxDoutSpec {
    const RESET_VALUE: u8 = 0;
}
