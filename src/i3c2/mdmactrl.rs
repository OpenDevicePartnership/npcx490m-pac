#[doc = "Register `MDMACTRL` reader"]
pub type R = crate::R<MdmactrlSpec>;
#[doc = "Register `MDMACTRL` writer"]
pub type W = crate::W<MdmactrlSpec>;
#[doc = "Field `DMAFB` reader - DMA from Bus"]
pub type DmafbR = crate::FieldReader;
#[doc = "Field `DMAFB` writer - DMA from Bus"]
pub type DmafbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMATB` reader - DMA to Bus"]
pub type DmatbR = crate::FieldReader;
#[doc = "Field `DMATB` writer - DMA to Bus"]
pub type DmatbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - DMA from Bus"]
    #[inline(always)]
    pub fn dmafb(&self) -> DmafbR {
        DmafbR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DMA to Bus"]
    #[inline(always)]
    pub fn dmatb(&self) -> DmatbR {
        DmatbR::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDMACTRL")
            .field("dmafb", &self.dmafb())
            .field("dmatb", &self.dmatb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - DMA from Bus"]
    #[inline(always)]
    #[must_use]
    pub fn dmafb(&mut self) -> DmafbW<MdmactrlSpec> {
        DmafbW::new(self, 0)
    }
    #[doc = "Bits 2:3 - DMA to Bus"]
    #[inline(always)]
    #[must_use]
    pub fn dmatb(&mut self) -> DmatbW<MdmactrlSpec> {
        DmatbW::new(self, 2)
    }
}
#[doc = "Controller DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmactrlSpec;
impl crate::RegisterSpec for MdmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdmactrl::R`](R) reader structure"]
impl crate::Readable for MdmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mdmactrl::W`](W) writer structure"]
impl crate::Writable for MdmactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMACTRL to value 0"]
impl crate::Resettable for MdmactrlSpec {
    const RESET_VALUE: u32 = 0;
}
