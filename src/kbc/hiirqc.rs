#[doc = "Register `HIIRQC` reader"]
pub type R = crate::R<HiirqcSpec>;
#[doc = "Register `HIIRQC` writer"]
pub type W = crate::W<HiirqcSpec>;
#[doc = "Field `IRQ1B` reader - Host Interrupt Request 1 Control Bit"]
pub type Irq1bR = crate::BitReader;
#[doc = "Field `IRQ1B` writer - Host Interrupt Request 1 Control Bit"]
pub type Irq1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ12B` reader - Host Interrupt Request 12 Control Bit"]
pub type Irq12bR = crate::BitReader;
#[doc = "Field `IRQ12B` writer - Host Interrupt Request 12 Control Bit"]
pub type Irq12bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQNPOL` reader - Negative Polarity"]
pub type IrqnpolR = crate::BitReader;
#[doc = "Field `IRQNPOL` writer - Negative Polarity"]
pub type IrqnpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host Interrupt Request 1 Control Bit"]
    #[inline(always)]
    pub fn irq1b(&self) -> Irq1bR {
        Irq1bR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Interrupt Request 12 Control Bit"]
    #[inline(always)]
    pub fn irq12b(&self) -> Irq12bR {
        Irq12bR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Negative Polarity"]
    #[inline(always)]
    pub fn irqnpol(&self) -> IrqnpolR {
        IrqnpolR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIIRQC")
            .field("irq1b", &self.irq1b())
            .field("irq12b", &self.irq12b())
            .field("irqnpol", &self.irqnpol())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Host Interrupt Request 1 Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn irq1b(&mut self) -> Irq1bW<HiirqcSpec> {
        Irq1bW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Interrupt Request 12 Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn irq12b(&mut self) -> Irq12bW<HiirqcSpec> {
        Irq12bW::new(self, 1)
    }
    #[doc = "Bit 6 - Negative Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn irqnpol(&mut self) -> IrqnpolW<HiirqcSpec> {
        IrqnpolW::new(self, 6)
    }
}
#[doc = "Host Interface IRQ Control Register (HIIRQC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hiirqc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hiirqc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HiirqcSpec;
impl crate::RegisterSpec for HiirqcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hiirqc::R`](R) reader structure"]
impl crate::Readable for HiirqcSpec {}
#[doc = "`write(|w| ..)` method takes [`hiirqc::W`](W) writer structure"]
impl crate::Writable for HiirqcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIIRQC to value 0"]
impl crate::Resettable for HiirqcSpec {
    const RESET_VALUE: u8 = 0;
}
