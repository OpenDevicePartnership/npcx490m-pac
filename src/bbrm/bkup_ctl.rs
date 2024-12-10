#[doc = "Register `BKUP_CTL` reader"]
pub type R = crate::R<BkupCtlSpec>;
#[doc = "Register `BKUP_CTL` writer"]
pub type W = crate::W<BkupCtlSpec>;
#[doc = "Field `INTRUD1_POL` reader - Intruder1 Polarity"]
pub type Intrud1PolR = crate::BitReader;
#[doc = "Field `INTRUD1_POL` writer - Intruder1 Polarity"]
pub type Intrud1PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUD2_POL` reader - Intruder2 Polarity"]
pub type Intrud2PolR = crate::BitReader;
#[doc = "Field `INTRUD2_POL` writer - Intruder2 Polarity"]
pub type Intrud2PolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Intruder1 Polarity"]
    #[inline(always)]
    pub fn intrud1_pol(&self) -> Intrud1PolR {
        Intrud1PolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Intruder2 Polarity"]
    #[inline(always)]
    pub fn intrud2_pol(&self) -> Intrud2PolR {
        Intrud2PolR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKUP_CTL")
            .field("intrud1_pol", &self.intrud1_pol())
            .field("intrud2_pol", &self.intrud2_pol())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - Intruder1 Polarity"]
    #[inline(always)]
    pub fn intrud1_pol(&mut self) -> Intrud1PolW<BkupCtlSpec> {
        Intrud1PolW::new(self, 5)
    }
    #[doc = "Bit 6 - Intruder2 Polarity"]
    #[inline(always)]
    pub fn intrud2_pol(&mut self) -> Intrud2PolW<BkupCtlSpec> {
        Intrud2PolW::new(self, 6)
    }
}
#[doc = "BBRM Control Register (BKUP_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`bkup_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkup_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BkupCtlSpec;
impl crate::RegisterSpec for BkupCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bkup_ctl::R`](R) reader structure"]
impl crate::Readable for BkupCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`bkup_ctl::W`](W) writer structure"]
impl crate::Writable for BkupCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BKUP_CTL to value 0"]
impl crate::Resettable for BkupCtlSpec {
    const RESET_VALUE: u8 = 0;
}
