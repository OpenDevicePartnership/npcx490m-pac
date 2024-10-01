#[doc = "Register `HIPMnCTL` reader"]
pub type R = crate::R<HipmnCtlSpec>;
#[doc = "Register `HIPMnCTL` writer"]
pub type W = crate::W<HipmnCtlSpec>;
#[doc = "Field `IBFIE` reader - Input Buffer Full Interrupt Enable"]
pub type IbfieR = crate::BitReader;
#[doc = "Field `IBFIE` writer - Input Buffer Full Interrupt Enable"]
pub type IbfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBEIE` reader - Output Buffer Empty Interrupt Enable"]
pub type ObeieR = crate::BitReader;
#[doc = "Field `OBEIE` writer - Output Buffer Empty Interrupt Enable"]
pub type ObeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCIPOL` reader - SCI Negative Polarity"]
pub type ScipolR = crate::BitReader;
#[doc = "Field `SCIPOL` writer - SCI Negative Polarity"]
pub type ScipolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Input Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn ibfie(&self) -> IbfieR {
        IbfieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn obeie(&self) -> ObeieR {
        ObeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - SCI Negative Polarity"]
    #[inline(always)]
    pub fn scipol(&self) -> ScipolR {
        ScipolR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIPMnCTL")
            .field("ibfie", &self.ibfie())
            .field("obeie", &self.obeie())
            .field("scipol", &self.scipol())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Input Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ibfie(&mut self) -> IbfieW<HipmnCtlSpec> {
        IbfieW::new(self, 0)
    }
    #[doc = "Bit 1 - Output Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn obeie(&mut self) -> ObeieW<HipmnCtlSpec> {
        ObeieW::new(self, 1)
    }
    #[doc = "Bit 6 - SCI Negative Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn scipol(&mut self) -> ScipolW<HipmnCtlSpec> {
        ScipolW::new(self, 6)
    }
}
#[doc = "Host Interface PM n Control Register (HIPMnCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HipmnCtlSpec;
impl crate::RegisterSpec for HipmnCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hipmn_ctl::R`](R) reader structure"]
impl crate::Readable for HipmnCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`hipmn_ctl::W`](W) writer structure"]
impl crate::Writable for HipmnCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIPMnCTL to value 0"]
impl crate::Resettable for HipmnCtlSpec {
    const RESET_VALUE: u8 = 0;
}
