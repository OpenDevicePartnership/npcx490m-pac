#[doc = "Register `OTP_CTL` reader"]
pub type R = crate::R<OtpCtlSpec>;
#[doc = "Register `OTP_CTL` writer"]
pub type W = crate::W<OtpCtlSpec>;
#[doc = "Field `START` reader - Start OTP Operation"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start OTP Operation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - Write Select"]
pub type WriteR = crate::BitReader;
#[doc = "Field `WRITE` writer - Write Select"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - OTP Operation Completed"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - OTP Operation Completed"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTP_EN` reader - OTPI Module Enable"]
pub type OtpEnR = crate::BitReader;
#[doc = "Field `OTP_EN` writer - OTPI Module Enable"]
pub type OtpEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start OTP Operation"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Select"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - OTP Operation Completed"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OTPI Module Enable"]
    #[inline(always)]
    pub fn otp_en(&self) -> OtpEnR {
        OtpEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_CTL")
            .field("start", &self.start())
            .field("write", &self.write())
            .field("done", &self.done())
            .field("otp_en", &self.otp_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Start OTP Operation"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<OtpCtlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Select"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<OtpCtlSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 6 - OTP Operation Completed"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<OtpCtlSpec> {
        DoneW::new(self, 6)
    }
    #[doc = "Bit 7 - OTPI Module Enable"]
    #[inline(always)]
    pub fn otp_en(&mut self) -> OtpEnW<OtpCtlSpec> {
        OtpEnW::new(self, 7)
    }
}
#[doc = "OTP Control Register (OTP_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpCtlSpec;
impl crate::RegisterSpec for OtpCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`otp_ctl::R`](R) reader structure"]
impl crate::Readable for OtpCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_ctl::W`](W) writer structure"]
impl crate::Writable for OtpCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OTP_CTL to value 0"]
impl crate::Resettable for OtpCtlSpec {
    const RESET_VALUE: u8 = 0;
}
