#[doc = "Register `UFTSTSn` reader"]
pub type R = crate::R<UftstsnSpec>;
#[doc = "Field `TEMPTY_LEVEL` reader - Transmit FIFO Empty Level"]
pub type TemptyLevelR = crate::FieldReader;
#[doc = "Field `TEMPTY_LEVEL_STS` reader - Transmit FIFO Empty Level Status"]
pub type TemptyLevelStsR = crate::BitReader;
#[doc = "Field `TFIFO_EMPTY_STS` reader - Transmit FIFO Empty Status"]
pub type TfifoEmptyStsR = crate::BitReader;
#[doc = "Field `NXMIP` reader - No Transmit in Progress"]
pub type NxmipR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO Empty Level"]
    #[inline(always)]
    pub fn tempty_level(&self) -> TemptyLevelR {
        TemptyLevelR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty Level Status"]
    #[inline(always)]
    pub fn tempty_level_sts(&self) -> TemptyLevelStsR {
        TemptyLevelStsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO Empty Status"]
    #[inline(always)]
    pub fn tfifo_empty_sts(&self) -> TfifoEmptyStsR {
        TfifoEmptyStsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Transmit in Progress"]
    #[inline(always)]
    pub fn nxmip(&self) -> NxmipR {
        NxmipR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UFTSTSn")
            .field("tempty_level", &self.tempty_level())
            .field("tempty_level_sts", &self.tempty_level_sts())
            .field("tfifo_empty_sts", &self.tfifo_empty_sts())
            .field("nxmip", &self.nxmip())
            .finish()
    }
}
#[doc = "FIFO Mode Transmit Status Register (UFTSTSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`uftstsn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UftstsnSpec;
impl crate::RegisterSpec for UftstsnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uftstsn::R`](R) reader structure"]
impl crate::Readable for UftstsnSpec {}
#[doc = "`reset()` method sets UFTSTSn to value 0"]
impl crate::Resettable for UftstsnSpec {
    const RESET_VALUE: u8 = 0;
}
