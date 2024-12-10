#[doc = "Register `OTP_ADDR_L` reader"]
pub type R = crate::R<OtpAddrLSpec>;
#[doc = "Register `OTP_ADDR_L` writer"]
pub type W = crate::W<OtpAddrLSpec>;
#[doc = "Field `BIT_ADDR` reader - OTP Bit Address"]
pub type BitAddrR = crate::FieldReader;
#[doc = "Field `BIT_ADDR` writer - OTP Bit Address"]
pub type BitAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADDR_L` reader - OTP Byte Address Low"]
pub type AddrLR = crate::FieldReader;
#[doc = "Field `ADDR_L` writer - OTP Byte Address Low"]
pub type AddrLW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - OTP Bit Address"]
    #[inline(always)]
    pub fn bit_addr(&self) -> BitAddrR {
        BitAddrR::new(self.bits & 7)
    }
    #[doc = "Bits 3:7 - OTP Byte Address Low"]
    #[inline(always)]
    pub fn addr_l(&self) -> AddrLR {
        AddrLR::new((self.bits >> 3) & 0x1f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_ADDR_L")
            .field("bit_addr", &self.bit_addr())
            .field("addr_l", &self.addr_l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - OTP Bit Address"]
    #[inline(always)]
    pub fn bit_addr(&mut self) -> BitAddrW<OtpAddrLSpec> {
        BitAddrW::new(self, 0)
    }
    #[doc = "Bits 3:7 - OTP Byte Address Low"]
    #[inline(always)]
    pub fn addr_l(&mut self) -> AddrLW<OtpAddrLSpec> {
        AddrLW::new(self, 3)
    }
}
#[doc = "OTP Address Low Register (OTP_ADDR_L)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_addr_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_addr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpAddrLSpec;
impl crate::RegisterSpec for OtpAddrLSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`otp_addr_l::R`](R) reader structure"]
impl crate::Readable for OtpAddrLSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_addr_l::W`](W) writer structure"]
impl crate::Writable for OtpAddrLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OTP_ADDR_L to value 0"]
impl crate::Resettable for OtpAddrLSpec {
    const RESET_VALUE: u8 = 0;
}
