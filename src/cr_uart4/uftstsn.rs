#[doc = "Register `UFTSTSn` reader"]
pub type R = crate::R<UftstsnSpec>;
#[doc = "Register `UFTSTSn` writer"]
pub type W = crate::W<UftstsnSpec>;
#[doc = "Field `TEMPTY_LEVEL` reader - Transmit FIFO Empty Level"]
pub type TemptyLevelR = crate::FieldReader;
#[doc = "Field `TEMPTY_LEVEL` writer - Transmit FIFO Empty Level"]
pub type TemptyLevelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TEMPTY_LEVEL_STS` reader - Transmit FIFO Empty Level Status"]
pub type TemptyLevelStsR = crate::BitReader;
#[doc = "Field `TEMPTY_LEVEL_STS` writer - Transmit FIFO Empty Level Status"]
pub type TemptyLevelStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIFO_EMPTY_STS` reader - Transmit FIFO Empty Status"]
pub type TfifoEmptyStsR = crate::BitReader;
#[doc = "Field `TFIFO_EMPTY_STS` writer - Transmit FIFO Empty Status"]
pub type TfifoEmptyStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NXMIP` reader - No Transmit in Progress"]
pub type NxmipR = crate::BitReader;
#[doc = "Field `NXMIP` writer - No Transmit in Progress"]
pub type NxmipW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO Empty Level"]
    #[inline(always)]
    #[must_use]
    pub fn tempty_level(&mut self) -> TemptyLevelW<UftstsnSpec> {
        TemptyLevelW::new(self, 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty Level Status"]
    #[inline(always)]
    #[must_use]
    pub fn tempty_level_sts(&mut self) -> TemptyLevelStsW<UftstsnSpec> {
        TemptyLevelStsW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit FIFO Empty Status"]
    #[inline(always)]
    #[must_use]
    pub fn tfifo_empty_sts(&mut self) -> TfifoEmptyStsW<UftstsnSpec> {
        TfifoEmptyStsW::new(self, 6)
    }
    #[doc = "Bit 7 - No Transmit in Progress"]
    #[inline(always)]
    #[must_use]
    pub fn nxmip(&mut self) -> NxmipW<UftstsnSpec> {
        NxmipW::new(self, 7)
    }
}
#[doc = "FIFO Mode Transmit Status Register (UFTSTSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`uftstsn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uftstsn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UftstsnSpec;
impl crate::RegisterSpec for UftstsnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uftstsn::R`](R) reader structure"]
impl crate::Readable for UftstsnSpec {}
#[doc = "`write(|w| ..)` method takes [`uftstsn::W`](W) writer structure"]
impl crate::Writable for UftstsnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UFTSTSn to value 0"]
impl crate::Resettable for UftstsnSpec {
    const RESET_VALUE: u8 = 0;
}
