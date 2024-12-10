#[doc = "Register `VWCTL` reader"]
pub type R = crate::R<VwctlSpec>;
#[doc = "Register `VWCTL` writer"]
pub type W = crate::W<VwctlSpec>;
#[doc = "Field `INTWIN` reader - Interrupt Window Select"]
pub type IntwinR = crate::FieldReader;
#[doc = "Field `INTWIN` writer - Interrupt Window Select"]
pub type IntwinW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPVWMAP` reader - GPIO Virtual Wire Indices Mapping"]
pub type GpvwmapR = crate::FieldReader;
#[doc = "Field `GPVWMAP` writer - GPIO Virtual Wire Indices Mapping"]
pub type GpvwmapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ_DIS` reader - IRQ Deassert"]
pub type IrqDisR = crate::BitReader;
#[doc = "Field `IRQ_DIS` writer - IRQ Deassert"]
pub type IrqDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_VALID` reader - No Valid Bits"]
pub type NoValidR = crate::BitReader;
#[doc = "Field `NO_VALID` writer - No Valid Bits"]
pub type NoValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Interrupt Window Select"]
    #[inline(always)]
    pub fn intwin(&self) -> IntwinR {
        IntwinR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO Virtual Wire Indices Mapping"]
    #[inline(always)]
    pub fn gpvwmap(&self) -> GpvwmapR {
        GpvwmapR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - IRQ Deassert"]
    #[inline(always)]
    pub fn irq_dis(&self) -> IrqDisR {
        IrqDisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Valid Bits"]
    #[inline(always)]
    pub fn no_valid(&self) -> NoValidR {
        NoValidR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VWCTL")
            .field("intwin", &self.intwin())
            .field("gpvwmap", &self.gpvwmap())
            .field("irq_dis", &self.irq_dis())
            .field("no_valid", &self.no_valid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt Window Select"]
    #[inline(always)]
    pub fn intwin(&mut self) -> IntwinW<VwctlSpec> {
        IntwinW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO Virtual Wire Indices Mapping"]
    #[inline(always)]
    pub fn gpvwmap(&mut self) -> GpvwmapW<VwctlSpec> {
        GpvwmapW::new(self, 2)
    }
    #[doc = "Bit 4 - IRQ Deassert"]
    #[inline(always)]
    pub fn irq_dis(&mut self) -> IrqDisW<VwctlSpec> {
        IrqDisW::new(self, 4)
    }
    #[doc = "Bit 5 - No Valid Bits"]
    #[inline(always)]
    pub fn no_valid(&mut self) -> NoValidW<VwctlSpec> {
        NoValidW::new(self, 5)
    }
}
#[doc = "Virtual Wire Channel Control Register (VWCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VwctlSpec;
impl crate::RegisterSpec for VwctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vwctl::R`](R) reader structure"]
impl crate::Readable for VwctlSpec {}
#[doc = "`write(|w| ..)` method takes [`vwctl::W`](W) writer structure"]
impl crate::Writable for VwctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VWCTL to value 0"]
impl crate::Resettable for VwctlSpec {
    const RESET_VALUE: u32 = 0;
}
