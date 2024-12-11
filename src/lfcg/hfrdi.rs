#[doc = "Register `HFRDI` reader"]
pub type R = crate::R<HfrdiSpec>;
#[doc = "Register `HFRDI` writer"]
pub type W = crate::W<HfrdiSpec>;
#[doc = "Field `HFRDI` reader - High-Frequency Reference Divisor I"]
pub type HfrdiR = crate::FieldReader<u16>;
#[doc = "Field `HFRDI` writer - High-Frequency Reference Divisor I"]
pub type HfrdiW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - High-Frequency Reference Divisor I"]
    #[inline(always)]
    pub fn hfrdi(&self) -> HfrdiR {
        HfrdiR::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFRDI")
            .field("hfrdi", &self.hfrdi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - High-Frequency Reference Divisor I"]
    #[inline(always)]
    pub fn hfrdi(&mut self) -> HfrdiW<HfrdiSpec> {
        HfrdiW::new(self, 0)
    }
}
#[doc = "High-Frequency Reference Divisor I Register (HFRDI)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrdi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrdi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfrdiSpec;
impl crate::RegisterSpec for HfrdiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hfrdi::R`](R) reader structure"]
impl crate::Readable for HfrdiSpec {}
#[doc = "`write(|w| ..)` method takes [`hfrdi::W`](W) writer structure"]
impl crate::Writable for HfrdiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HFRDI to value 0"]
impl crate::Resettable for HfrdiSpec {
    const RESET_VALUE: u16 = 0;
}
