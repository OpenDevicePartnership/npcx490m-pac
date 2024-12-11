#[doc = "Register `HIPMnIC` reader"]
pub type R = crate::R<HipmnIcSpec>;
#[doc = "Register `HIPMnIC` writer"]
pub type W = crate::W<HipmnIcSpec>;
#[doc = "Field `IRQB` reader - Host Interrupt Request Control Bit"]
pub type IrqbR = crate::BitReader;
#[doc = "Field `IRQB` writer - Host Interrupt Request Control Bit"]
pub type IrqbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMIB` reader - Host SMI Request Control Bit"]
pub type SmibR = crate::BitReader;
#[doc = "Field `SMIB` writer - Host SMI Request Control Bit"]
pub type SmibW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCIB` reader - Host SCI Request Control Bit"]
pub type ScibR = crate::BitReader;
#[doc = "Field `SCIB` writer - Host SCI Request Control Bit"]
pub type ScibW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMIPOL` reader - SMI Negative Polarity"]
pub type SmipolR = crate::BitReader;
#[doc = "Field `SMIPOL` writer - SMI Negative Polarity"]
pub type SmipolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCIIS` reader - SCI on IBF Start"]
pub type SciisR = crate::BitReader;
#[doc = "Field `SCIIS` writer - SCI on IBF Start"]
pub type SciisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host Interrupt Request Control Bit"]
    #[inline(always)]
    pub fn irqb(&self) -> IrqbR {
        IrqbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host SMI Request Control Bit"]
    #[inline(always)]
    pub fn smib(&self) -> SmibR {
        SmibR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host SCI Request Control Bit"]
    #[inline(always)]
    pub fn scib(&self) -> ScibR {
        ScibR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - SMI Negative Polarity"]
    #[inline(always)]
    pub fn smipol(&self) -> SmipolR {
        SmipolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCI on IBF Start"]
    #[inline(always)]
    pub fn sciis(&self) -> SciisR {
        SciisR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIPMnIC")
            .field("irqb", &self.irqb())
            .field("smib", &self.smib())
            .field("scib", &self.scib())
            .field("smipol", &self.smipol())
            .field("sciis", &self.sciis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Host Interrupt Request Control Bit"]
    #[inline(always)]
    pub fn irqb(&mut self) -> IrqbW<HipmnIcSpec> {
        IrqbW::new(self, 0)
    }
    #[doc = "Bit 1 - Host SMI Request Control Bit"]
    #[inline(always)]
    pub fn smib(&mut self) -> SmibW<HipmnIcSpec> {
        SmibW::new(self, 1)
    }
    #[doc = "Bit 2 - Host SCI Request Control Bit"]
    #[inline(always)]
    pub fn scib(&mut self) -> ScibW<HipmnIcSpec> {
        ScibW::new(self, 2)
    }
    #[doc = "Bit 6 - SMI Negative Polarity"]
    #[inline(always)]
    pub fn smipol(&mut self) -> SmipolW<HipmnIcSpec> {
        SmipolW::new(self, 6)
    }
    #[doc = "Bit 7 - SCI on IBF Start"]
    #[inline(always)]
    pub fn sciis(&mut self) -> SciisW<HipmnIcSpec> {
        SciisW::new(self, 7)
    }
}
#[doc = "Host Interface PM n Interrupt Control Register (HIPMnIC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_ic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_ic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HipmnIcSpec;
impl crate::RegisterSpec for HipmnIcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hipmn_ic::R`](R) reader structure"]
impl crate::Readable for HipmnIcSpec {}
#[doc = "`write(|w| ..)` method takes [`hipmn_ic::W`](W) writer structure"]
impl crate::Writable for HipmnIcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIPMnIC to value 0"]
impl crate::Resettable for HipmnIcSpec {
    const RESET_VALUE: u8 = 0;
}
