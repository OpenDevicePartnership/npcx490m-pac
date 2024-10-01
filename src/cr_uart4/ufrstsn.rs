#[doc = "Register `UFRSTSn` reader"]
pub type R = crate::R<UfrstsnSpec>;
#[doc = "Register `UFRSTSn` writer"]
pub type W = crate::W<UfrstsnSpec>;
#[doc = "Field `RFULL_LEVEL` reader - Receive FIFO Full Level"]
pub type RfullLevelR = crate::FieldReader;
#[doc = "Field `RFULL_LEVEL` writer - Receive FIFO Full Level"]
pub type RfullLevelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RFULL_LEVEL_STS` reader - Receive FIFO Full Level Status"]
pub type RfullLevelStsR = crate::BitReader;
#[doc = "Field `RFULL_LEVEL_STS` writer - Receive FIFO Full Level Status"]
pub type RfullLevelStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFO_NEMPTY_STS` reader - Receive FIFO Not Empty Status"]
pub type RfifoNemptyStsR = crate::BitReader;
#[doc = "Field `RFIFO_NEMPTY_STS` writer - Receive FIFO Not Empty Status"]
pub type RfifoNemptyStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - Receive Error"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - Receive Error"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Full Level"]
    #[inline(always)]
    pub fn rfull_level(&self) -> RfullLevelR {
        RfullLevelR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Receive FIFO Full Level Status"]
    #[inline(always)]
    pub fn rfull_level_sts(&self) -> RfullLevelStsR {
        RfullLevelStsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Not Empty Status"]
    #[inline(always)]
    pub fn rfifo_nempty_sts(&self) -> RfifoNemptyStsR {
        RfifoNemptyStsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Error"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UFRSTSn")
            .field("rfull_level", &self.rfull_level())
            .field("rfull_level_sts", &self.rfull_level_sts())
            .field("rfifo_nempty_sts", &self.rfifo_nempty_sts())
            .field("err", &self.err())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Receive FIFO Full Level"]
    #[inline(always)]
    #[must_use]
    pub fn rfull_level(&mut self) -> RfullLevelW<UfrstsnSpec> {
        RfullLevelW::new(self, 0)
    }
    #[doc = "Bit 5 - Receive FIFO Full Level Status"]
    #[inline(always)]
    #[must_use]
    pub fn rfull_level_sts(&mut self) -> RfullLevelStsW<UfrstsnSpec> {
        RfullLevelStsW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive FIFO Not Empty Status"]
    #[inline(always)]
    #[must_use]
    pub fn rfifo_nempty_sts(&mut self) -> RfifoNemptyStsW<UfrstsnSpec> {
        RfifoNemptyStsW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Error"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<UfrstsnSpec> {
        ErrW::new(self, 7)
    }
}
#[doc = "FIFO Mode Receive Status Register (UFRSTSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ufrstsn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ufrstsn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UfrstsnSpec;
impl crate::RegisterSpec for UfrstsnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ufrstsn::R`](R) reader structure"]
impl crate::Readable for UfrstsnSpec {}
#[doc = "`write(|w| ..)` method takes [`ufrstsn::W`](W) writer structure"]
impl crate::Writable for UfrstsnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UFRSTSn to value 0"]
impl crate::Resettable for UfrstsnSpec {
    const RESET_VALUE: u8 = 0;
}
