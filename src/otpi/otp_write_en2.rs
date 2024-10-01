#[doc = "Register `OTP_WRITE_EN2` reader"]
pub type R = crate::R<OtpWriteEn2Spec>;
#[doc = "Register `OTP_WRITE_EN2` writer"]
pub type W = crate::W<OtpWriteEn2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP Write Enable 0-3 Register (OTP_WRITE_EN0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_write_en2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_write_en2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpWriteEn2Spec;
impl crate::RegisterSpec for OtpWriteEn2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`otp_write_en2::R`](R) reader structure"]
impl crate::Readable for OtpWriteEn2Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_write_en2::W`](W) writer structure"]
impl crate::Writable for OtpWriteEn2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OTP_WRITE_EN2 to value 0"]
impl crate::Resettable for OtpWriteEn2Spec {
    const RESET_VALUE: u8 = 0;
}
