#[doc = "Register `KBS_DLY1` reader"]
pub type R = crate::R<KbsDly1Spec>;
#[doc = "Register `KBS_DLY1` writer"]
pub type W = crate::W<KbsDly1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Keyboard Scan Delay 1 Register (KBS_DLY1)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_dly1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_dly1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsDly1Spec;
impl crate::RegisterSpec for KbsDly1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbs_dly1::R`](R) reader structure"]
impl crate::Readable for KbsDly1Spec {}
#[doc = "`write(|w| ..)` method takes [`kbs_dly1::W`](W) writer structure"]
impl crate::Writable for KbsDly1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBS_DLY1 to value 0"]
impl crate::Resettable for KbsDly1Spec {
    const RESET_VALUE: u8 = 0;
}
