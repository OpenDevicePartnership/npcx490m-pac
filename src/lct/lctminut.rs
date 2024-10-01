#[doc = "Register `LCTMINUT` reader"]
pub type R = crate::R<LctminutSpec>;
#[doc = "Register `LCTMINUT` writer"]
pub type W = crate::W<LctminutSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCT Minutes Register (LCTMINUT)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctminut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctminut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LctminutSpec;
impl crate::RegisterSpec for LctminutSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lctminut::R`](R) reader structure"]
impl crate::Readable for LctminutSpec {}
#[doc = "`write(|w| ..)` method takes [`lctminut::W`](W) writer structure"]
impl crate::Writable for LctminutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LCTMINUT to value 0"]
impl crate::Resettable for LctminutSpec {
    const RESET_VALUE: u8 = 0;
}
