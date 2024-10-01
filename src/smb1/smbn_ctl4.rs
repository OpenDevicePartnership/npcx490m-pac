#[doc = "Register `SMBnCTL4` reader"]
pub type R = crate::R<SmbnCtl4Spec>;
#[doc = "Register `SMBnCTL4` writer"]
pub type W = crate::W<SmbnCtl4Spec>;
#[doc = "Field `HLDT` reader - SDA Hold Time"]
pub type HldtR = crate::FieldReader;
#[doc = "Field `HLDT` writer - SDA Hold Time"]
pub type HldtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LVL_WE` reader - Level Control Write Enable"]
pub type LvlWeR = crate::BitReader;
#[doc = "Field `LVL_WE` writer - Level Control Write Enable"]
pub type LvlWeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - SDA Hold Time"]
    #[inline(always)]
    pub fn hldt(&self) -> HldtR {
        HldtR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 7 - Level Control Write Enable"]
    #[inline(always)]
    pub fn lvl_we(&self) -> LvlWeR {
        LvlWeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnCTL4")
            .field("hldt", &self.hldt())
            .field("lvl_we", &self.lvl_we())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - SDA Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn hldt(&mut self) -> HldtW<SmbnCtl4Spec> {
        HldtW::new(self, 0)
    }
    #[doc = "Bit 7 - Level Control Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvl_we(&mut self) -> LvlWeW<SmbnCtl4Spec> {
        LvlWeW::new(self, 7)
    }
}
#[doc = "SMB Control 4 Register (SMBnCTL4)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_ctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_ctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnCtl4Spec;
impl crate::RegisterSpec for SmbnCtl4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_ctl4::R`](R) reader structure"]
impl crate::Readable for SmbnCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`smbn_ctl4::W`](W) writer structure"]
impl crate::Writable for SmbnCtl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnCTL4 to value 0"]
impl crate::Resettable for SmbnCtl4Spec {
    const RESET_VALUE: u8 = 0;
}
