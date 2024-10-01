#[doc = "Register `VWREGIDX` reader"]
pub type R = crate::R<VwregidxSpec>;
#[doc = "Register `VWREGIDX` writer"]
pub type W = crate::W<VwregidxSpec>;
#[doc = "Field `VWREGIDX` reader - Virtual Wire Register Index"]
pub type VwregidxR = crate::FieldReader<u16>;
#[doc = "Field `VWREGIDX` writer - Virtual Wire Register Index"]
pub type VwregidxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 2:11 - Virtual Wire Register Index"]
    #[inline(always)]
    pub fn vwregidx(&self) -> VwregidxR {
        VwregidxR::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VWREGIDX")
            .field("vwregidx", &self.vwregidx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 2:11 - Virtual Wire Register Index"]
    #[inline(always)]
    #[must_use]
    pub fn vwregidx(&mut self) -> VwregidxW<VwregidxSpec> {
        VwregidxW::new(self, 2)
    }
}
#[doc = "Virtual Wire Register Index Register (VWREGIDX)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwregidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwregidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VwregidxSpec;
impl crate::RegisterSpec for VwregidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vwregidx::R`](R) reader structure"]
impl crate::Readable for VwregidxSpec {}
#[doc = "`write(|w| ..)` method takes [`vwregidx::W`](W) writer structure"]
impl crate::Writable for VwregidxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VWREGIDX to value 0"]
impl crate::Resettable for VwregidxSpec {
    const RESET_VALUE: u32 = 0;
}
