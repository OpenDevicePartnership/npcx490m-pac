#[doc = "Register `MWMSG_DDR` reader"]
pub type R = crate::R<MwmsgDdrSpec>;
#[doc = "Register `MWMSG_DDR` writer"]
pub type W = crate::W<MwmsgDdrSpec>;
#[doc = "Field `DATA` reader - Data for HDR-DDR Writes"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Data for HDR-DDR Writes"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data for HDR-DDR Writes"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MWMSG_DDR")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Data for HDR-DDR Writes"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<MwmsgDdrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Start or Continue DDR Message Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mwmsg_ddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwmsg_ddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwmsgDdrSpec;
impl crate::RegisterSpec for MwmsgDdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mwmsg_ddr::R`](R) reader structure"]
impl crate::Readable for MwmsgDdrSpec {}
#[doc = "`write(|w| ..)` method takes [`mwmsg_ddr::W`](W) writer structure"]
impl crate::Writable for MwmsgDdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWMSG_DDR to value 0"]
impl crate::Resettable for MwmsgDdrSpec {
    const RESET_VALUE: u32 = 0;
}
