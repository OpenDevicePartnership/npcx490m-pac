#[doc = "Register `OTP_DOUT` reader"]
pub type R = crate::R<OtpDoutSpec>;
#[doc = "Register `OTP_DOUT` writer"]
pub type W = crate::W<OtpDoutSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP Read Data Register (OTP_DOUT)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_dout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_dout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpDoutSpec;
impl crate::RegisterSpec for OtpDoutSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`otp_dout::R`](R) reader structure"]
impl crate::Readable for OtpDoutSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_dout::W`](W) writer structure"]
impl crate::Writable for OtpDoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OTP_DOUT to value 0"]
impl crate::Resettable for OtpDoutSpec {
    const RESET_VALUE: u8 = 0;
}
