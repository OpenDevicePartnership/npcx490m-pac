#[doc = "Register `IHIOA` reader"]
pub type R = crate::R<IhioaSpec>;
#[doc = "Register `IHIOA` writer"]
pub type W = crate::W<IhioaSpec>;
#[doc = "Field `IH_OFS` reader - Indirect Host I/O Offset"]
pub type IhOfsR = crate::FieldReader;
#[doc = "Field `IH_OFS` writer - Indirect Host I/O Offset"]
pub type IhOfsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indirect Host I/O Offset"]
    #[inline(always)]
    pub fn ih_ofs(&self) -> IhOfsR {
        IhOfsR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IHIOA")
            .field("ih_ofs", &self.ih_ofs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirect Host I/O Offset"]
    #[inline(always)]
    pub fn ih_ofs(&mut self) -> IhOfsW<IhioaSpec> {
        IhOfsW::new(self, 0)
    }
}
#[doc = "Indirect Host I/O Address Register (IHIOA)\n\nYou can [`read`](crate::Reg::read) this register and get [`ihioa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ihioa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhioaSpec;
impl crate::RegisterSpec for IhioaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ihioa::R`](R) reader structure"]
impl crate::Readable for IhioaSpec {}
#[doc = "`write(|w| ..)` method takes [`ihioa::W`](W) writer structure"]
impl crate::Writable for IhioaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets IHIOA to value 0"]
impl crate::Resettable for IhioaSpec {
    const RESET_VALUE: u16 = 0;
}
