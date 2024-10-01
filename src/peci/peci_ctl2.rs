#[doc = "Register `PECI_CTL2` reader"]
pub type R = crate::R<PeciCtl2Spec>;
#[doc = "Register `PECI_CTL2` writer"]
pub type W = crate::W<PeciCtl2Spec>;
#[doc = "Field `INDEX` reader - Index"]
pub type IndexR = crate::BitReader;
#[doc = "Field `INDEX` writer - Index"]
pub type IndexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_INC_RD` reader - Automatic Increment on Read"]
pub type AutoIncRdR = crate::BitReader;
#[doc = "Field `AUTO_INC_RD` writer - Automatic Increment on Read"]
pub type AutoIncRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_INC_WR` reader - Automatic Increment on Write"]
pub type AutoIncWrR = crate::BitReader;
#[doc = "Field `AUTO_INC_WR` writer - Automatic Increment on Write"]
pub type AutoIncWrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Index"]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Increment on Read"]
    #[inline(always)]
    pub fn auto_inc_rd(&self) -> AutoIncRdR {
        AutoIncRdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Automatic Increment on Write"]
    #[inline(always)]
    pub fn auto_inc_wr(&self) -> AutoIncWrR {
        AutoIncWrR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PECI_CTL2")
            .field("index", &self.index())
            .field("auto_inc_rd", &self.auto_inc_rd())
            .field("auto_inc_wr", &self.auto_inc_wr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Index"]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> IndexW<PeciCtl2Spec> {
        IndexW::new(self, 1)
    }
    #[doc = "Bit 2 - Automatic Increment on Read"]
    #[inline(always)]
    #[must_use]
    pub fn auto_inc_rd(&mut self) -> AutoIncRdW<PeciCtl2Spec> {
        AutoIncRdW::new(self, 2)
    }
    #[doc = "Bit 3 - Automatic Increment on Write"]
    #[inline(always)]
    #[must_use]
    pub fn auto_inc_wr(&mut self) -> AutoIncWrW<PeciCtl2Spec> {
        AutoIncWrW::new(self, 3)
    }
}
#[doc = "PECI Control 2 Register (PECI_CTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciCtl2Spec;
impl crate::RegisterSpec for PeciCtl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_ctl2::R`](R) reader structure"]
impl crate::Readable for PeciCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`peci_ctl2::W`](W) writer structure"]
impl crate::Writable for PeciCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_CTL2 to value 0"]
impl crate::Resettable for PeciCtl2Spec {
    const RESET_VALUE: u8 = 0;
}
