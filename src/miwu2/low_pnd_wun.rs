#[doc = "Register `LOW_PND_WUn` reader"]
pub type R = crate::R<LowPndWunSpec>;
#[doc = "Register `LOW_PND_WUn` writer"]
pub type W = crate::W<LowPndWunSpec>;
#[doc = "Field `PND_IN` reader - Pending Input"]
pub type PndInR = crate::FieldReader;
#[doc = "Field `PND_IN` writer - Pending Input"]
pub type PndInW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PND_GRP` reader - Pending Group"]
pub type PndGrpR = crate::FieldReader;
#[doc = "Field `PND_GRP` writer - Pending Group"]
pub type PndGrpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Pending Input"]
    #[inline(always)]
    pub fn pnd_in(&self) -> PndInR {
        PndInR::new(self.bits & 7)
    }
    #[doc = "Bits 4:7 - Pending Group"]
    #[inline(always)]
    pub fn pnd_grp(&self) -> PndGrpR {
        PndGrpR::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOW_PND_WUn")
            .field("pnd_in", &self.pnd_in())
            .field("pnd_grp", &self.pnd_grp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Pending Input"]
    #[inline(always)]
    #[must_use]
    pub fn pnd_in(&mut self) -> PndInW<LowPndWunSpec> {
        PndInW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Pending Group"]
    #[inline(always)]
    #[must_use]
    pub fn pnd_grp(&mut self) -> PndGrpW<LowPndWunSpec> {
        PndGrpW::new(self, 4)
    }
}
#[doc = "Lowest Pending Wake-Up Register (LOW_PND_WUn)\n\nYou can [`read`](crate::Reg::read) this register and get [`low_pnd_wun::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_pnd_wun::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowPndWunSpec;
impl crate::RegisterSpec for LowPndWunSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`low_pnd_wun::R`](R) reader structure"]
impl crate::Readable for LowPndWunSpec {}
#[doc = "`write(|w| ..)` method takes [`low_pnd_wun::W`](W) writer structure"]
impl crate::Writable for LowPndWunSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LOW_PND_WUn to value 0"]
impl crate::Resettable for LowPndWunSpec {
    const RESET_VALUE: u8 = 0;
}
