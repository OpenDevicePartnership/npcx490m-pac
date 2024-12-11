#[doc = "Register `OTP_ADDR_H` reader"]
pub type R = crate::R<OtpAddrHSpec>;
#[doc = "Register `OTP_ADDR_H` writer"]
pub type W = crate::W<OtpAddrHSpec>;
#[doc = "Field `ADDR_H` reader - OTP Byte Address High"]
pub type AddrHR = crate::FieldReader;
#[doc = "Field `ADDR_H` writer - OTP Byte Address High"]
pub type AddrHW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - OTP Byte Address High"]
    #[inline(always)]
    pub fn addr_h(&self) -> AddrHR {
        AddrHR::new(self.bits & 0x3f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_ADDR_H")
            .field("addr_h", &self.addr_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - OTP Byte Address High"]
    #[inline(always)]
    pub fn addr_h(&mut self) -> AddrHW<OtpAddrHSpec> {
        AddrHW::new(self, 0)
    }
}
#[doc = "OTP Address High Register (OTP_ADDR_H)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_addr_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_addr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpAddrHSpec;
impl crate::RegisterSpec for OtpAddrHSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`otp_addr_h::R`](R) reader structure"]
impl crate::Readable for OtpAddrHSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_addr_h::W`](W) writer structure"]
impl crate::Writable for OtpAddrHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OTP_ADDR_H to value 0"]
impl crate::Resettable for OtpAddrHSpec {
    const RESET_VALUE: u8 = 0;
}
