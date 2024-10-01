#[doc = "Register `IHD` reader"]
pub type R = crate::R<IhdSpec>;
#[doc = "Register `IHD` writer"]
pub type W = crate::W<IhdSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Indirect Host Data Register (IHD)\n\nYou can [`read`](crate::Reg::read) this register and get [`ihd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ihd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhdSpec;
impl crate::RegisterSpec for IhdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ihd::R`](R) reader structure"]
impl crate::Readable for IhdSpec {}
#[doc = "`write(|w| ..)` method takes [`ihd::W`](W) writer structure"]
impl crate::Writable for IhdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IHD to value 0"]
impl crate::Resettable for IhdSpec {
    const RESET_VALUE: u8 = 0;
}
