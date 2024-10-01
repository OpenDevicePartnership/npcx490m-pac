#[doc = "Register `LCTWEEKM` reader"]
pub type R = crate::R<LctweekmSpec>;
#[doc = "Register `LCTWEEKM` writer"]
pub type W = crate::W<LctweekmSpec>;
#[doc = "Field `WEEKS` reader - Weeks Value"]
pub type WeeksR = crate::FieldReader;
#[doc = "Field `WEEKS` writer - Weeks Value"]
pub type WeeksW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Weeks Value"]
    #[inline(always)]
    pub fn weeks(&self) -> WeeksR {
        WeeksR::new(self.bits & 7)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCTWEEKM")
            .field("weeks", &self.weeks())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Weeks Value"]
    #[inline(always)]
    #[must_use]
    pub fn weeks(&mut self) -> WeeksW<LctweekmSpec> {
        WeeksW::new(self, 0)
    }
}
#[doc = "LCT Weeks MSB Register (LCTWEEKM)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctweekm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctweekm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LctweekmSpec;
impl crate::RegisterSpec for LctweekmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lctweekm::R`](R) reader structure"]
impl crate::Readable for LctweekmSpec {}
#[doc = "`write(|w| ..)` method takes [`lctweekm::W`](W) writer structure"]
impl crate::Writable for LctweekmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LCTWEEKM to value 0"]
impl crate::Resettable for LctweekmSpec {
    const RESET_VALUE: u8 = 0;
}
