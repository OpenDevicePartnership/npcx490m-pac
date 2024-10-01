#[doc = "Register `HIPMnIE` reader"]
pub type R = crate::R<HipmnIeSpec>;
#[doc = "Register `HIPMnIE` writer"]
pub type W = crate::W<HipmnIeSpec>;
#[doc = "Field `IRQE` reader - IRQ Enable"]
pub type IrqeR = crate::BitReader;
#[doc = "Field `IRQE` writer - IRQ Enable"]
pub type IrqeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCIE` reader - SCI Enable"]
pub type ScieR = crate::BitReader;
#[doc = "Field `SCIE` writer - SCI Enable"]
pub type ScieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMIE` reader - SMI Enable"]
pub type SmieR = crate::BitReader;
#[doc = "Field `SMIE` writer - SMI Enable"]
pub type SmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIRQE` reader - Hardware IRQ Enable"]
pub type HirqeR = crate::BitReader;
#[doc = "Field `HIRQE` writer - Hardware IRQ Enable"]
pub type HirqeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSCIE` reader - Hardware SCI Enable"]
pub type HscieR = crate::BitReader;
#[doc = "Field `HSCIE` writer - Hardware SCI Enable"]
pub type HscieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSMIE` reader - Hardware SMI Enable"]
pub type HsmieR = crate::BitReader;
#[doc = "Field `HSMIE` writer - Hardware SMI Enable"]
pub type HsmieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IRQ Enable"]
    #[inline(always)]
    pub fn irqe(&self) -> IrqeR {
        IrqeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCI Enable"]
    #[inline(always)]
    pub fn scie(&self) -> ScieR {
        ScieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMI Enable"]
    #[inline(always)]
    pub fn smie(&self) -> SmieR {
        SmieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hardware IRQ Enable"]
    #[inline(always)]
    pub fn hirqe(&self) -> HirqeR {
        HirqeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hardware SCI Enable"]
    #[inline(always)]
    pub fn hscie(&self) -> HscieR {
        HscieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hardware SMI Enable"]
    #[inline(always)]
    pub fn hsmie(&self) -> HsmieR {
        HsmieR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIPMnIE")
            .field("irqe", &self.irqe())
            .field("scie", &self.scie())
            .field("smie", &self.smie())
            .field("hirqe", &self.hirqe())
            .field("hscie", &self.hscie())
            .field("hsmie", &self.hsmie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqe(&mut self) -> IrqeW<HipmnIeSpec> {
        IrqeW::new(self, 0)
    }
    #[doc = "Bit 1 - SCI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scie(&mut self) -> ScieW<HipmnIeSpec> {
        ScieW::new(self, 1)
    }
    #[doc = "Bit 2 - SMI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smie(&mut self) -> SmieW<HipmnIeSpec> {
        SmieW::new(self, 2)
    }
    #[doc = "Bit 3 - Hardware IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hirqe(&mut self) -> HirqeW<HipmnIeSpec> {
        HirqeW::new(self, 3)
    }
    #[doc = "Bit 4 - Hardware SCI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hscie(&mut self) -> HscieW<HipmnIeSpec> {
        HscieW::new(self, 4)
    }
    #[doc = "Bit 5 - Hardware SMI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsmie(&mut self) -> HsmieW<HipmnIeSpec> {
        HsmieW::new(self, 5)
    }
}
#[doc = "Host Interface PM n Interrupt Enable Register (HIPMnIE)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HipmnIeSpec;
impl crate::RegisterSpec for HipmnIeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hipmn_ie::R`](R) reader structure"]
impl crate::Readable for HipmnIeSpec {}
#[doc = "`write(|w| ..)` method takes [`hipmn_ie::W`](W) writer structure"]
impl crate::Writable for HipmnIeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIPMnIE to value 0"]
impl crate::Resettable for HipmnIeSpec {
    const RESET_VALUE: u8 = 0;
}
