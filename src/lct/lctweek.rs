#[doc = "Register `LCTWEEK` reader"]
pub type R = crate::R<LctweekSpec>;
#[doc = "Register `LCTWEEK` writer"]
pub type W = crate::W<LctweekSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCT Weeks LSB Register (LCTWEEK)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctweek::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctweek::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LctweekSpec;
impl crate::RegisterSpec for LctweekSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lctweek::R`](R) reader structure"]
impl crate::Readable for LctweekSpec {}
#[doc = "`write(|w| ..)` method takes [`lctweek::W`](W) writer structure"]
impl crate::Writable for LctweekSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LCTWEEK to value 0"]
impl crate::Resettable for LctweekSpec {
    const RESET_VALUE: u8 = 0;
}
