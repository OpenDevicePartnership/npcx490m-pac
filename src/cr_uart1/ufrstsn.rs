#[doc = "Register `UFRSTSn` reader"]
pub type R = crate::R<UfrstsnSpec>;
#[doc = "Field `RFULL_LEVEL` reader - Receive FIFO Full Level"]
pub type RfullLevelR = crate::FieldReader;
#[doc = "Field `RFULL_LEVEL_STS` reader - Receive FIFO Full Level Status"]
pub type RfullLevelStsR = crate::BitReader;
#[doc = "Field `RFIFO_NEMPTY_STS` reader - Receive FIFO Not Empty Status"]
pub type RfifoNemptyStsR = crate::BitReader;
#[doc = "Field `ERR` reader - Receive Error"]
pub type ErrR = crate::BitReader;
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
#[doc = "FIFO Mode Receive Status Register (UFRSTSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ufrstsn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UfrstsnSpec;
impl crate::RegisterSpec for UfrstsnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ufrstsn::R`](R) reader structure"]
impl crate::Readable for UfrstsnSpec {}
#[doc = "`reset()` method sets UFRSTSn to value 0"]
impl crate::Resettable for UfrstsnSpec {
    const RESET_VALUE: u8 = 0;
}
