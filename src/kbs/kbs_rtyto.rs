#[doc = "Register `KBS_RTYTO` reader"]
pub type R = crate::R<KbsRtytoSpec>;
#[doc = "Register `KBS_RTYTO` writer"]
pub type W = crate::W<KbsRtytoSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Keyboard Scan Retry Timeout Register (KBS_RTYTO)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_rtyto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_rtyto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsRtytoSpec;
impl crate::RegisterSpec for KbsRtytoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbs_rtyto::R`](R) reader structure"]
impl crate::Readable for KbsRtytoSpec {}
#[doc = "`write(|w| ..)` method takes [`kbs_rtyto::W`](W) writer structure"]
impl crate::Writable for KbsRtytoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBS_RTYTO to value 0"]
impl crate::Resettable for KbsRtytoSpec {
    const RESET_VALUE: u8 = 0;
}
