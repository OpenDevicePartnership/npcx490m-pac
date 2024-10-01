#[doc = "Register `SMBnCTL2` reader"]
pub type R = crate::R<SmbnCtl2Spec>;
#[doc = "Register `SMBnCTL2` writer"]
pub type W = crate::W<SmbnCtl2Spec>;
#[doc = "Field `ENABLE` reader - default"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - default"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLFRQ6_0` reader - SCL Frequency bits 6 through 0"]
pub type Sclfrq6_0R = crate::FieldReader;
#[doc = "Field `SCLFRQ6_0` writer - SCL Frequency bits 6 through 0"]
pub type Sclfrq6_0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - default"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - SCL Frequency bits 6 through 0"]
    #[inline(always)]
    pub fn sclfrq6_0(&self) -> Sclfrq6_0R {
        Sclfrq6_0R::new((self.bits >> 1) & 0x7f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnCTL2")
            .field("enable", &self.enable())
            .field("sclfrq6_0", &self.sclfrq6_0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - default"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<SmbnCtl2Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 1:7 - SCL Frequency bits 6 through 0"]
    #[inline(always)]
    #[must_use]
    pub fn sclfrq6_0(&mut self) -> Sclfrq6_0W<SmbnCtl2Spec> {
        Sclfrq6_0W::new(self, 1)
    }
}
#[doc = "SMB Control 2 Register (SMBnCTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnCtl2Spec;
impl crate::RegisterSpec for SmbnCtl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_ctl2::R`](R) reader structure"]
impl crate::Readable for SmbnCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`smbn_ctl2::W`](W) writer structure"]
impl crate::Writable for SmbnCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnCTL2 to value 0"]
impl crate::Resettable for SmbnCtl2Spec {
    const RESET_VALUE: u8 = 0;
}
