#[doc = "Register `HCBAH` reader"]
pub type R = crate::R<HcbahSpec>;
#[doc = "Register `HCBAH` writer"]
pub type W = crate::W<HcbahSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Configuration Base Address High Register (HCBAH)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbah::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbah::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcbahSpec;
impl crate::RegisterSpec for HcbahSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hcbah::R`](R) reader structure"]
impl crate::Readable for HcbahSpec {}
#[doc = "`write(|w| ..)` method takes [`hcbah::W`](W) writer structure"]
impl crate::Writable for HcbahSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HCBAH to value 0"]
impl crate::Resettable for HcbahSpec {
    const RESET_VALUE: u8 = 0;
}
