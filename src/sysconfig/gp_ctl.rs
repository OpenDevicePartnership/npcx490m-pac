#[doc = "Register `GP_CTL` reader"]
pub type R = crate::R<GpCtlSpec>;
#[doc = "Register `GP_CTL` writer"]
pub type W = crate::W<GpCtlSpec>;
#[doc = "Field `GP_EN` reader - Gang Programmer Enable"]
pub type GpEnR = crate::FieldReader;
#[doc = "Field `GP_EN` writer - Gang Programmer Enable"]
pub type GpEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Gang Programmer Enable"]
    #[inline(always)]
    pub fn gp_en(&self) -> GpEnR {
        GpEnR::new(self.bits & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_CTL")
            .field("gp_en", &self.gp_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Gang Programmer Enable"]
    #[inline(always)]
    pub fn gp_en(&mut self) -> GpEnW<GpCtlSpec> {
        GpEnW::new(self, 0)
    }
}
#[doc = "Gang Programmer Control Register (GP_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`gp_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpCtlSpec;
impl crate::RegisterSpec for GpCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gp_ctl::R`](R) reader structure"]
impl crate::Readable for GpCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`gp_ctl::W`](W) writer structure"]
impl crate::Writable for GpCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GP_CTL to value 0x06"]
impl crate::Resettable for GpCtlSpec {
    const RESET_VALUE: u8 = 0x06;
}
