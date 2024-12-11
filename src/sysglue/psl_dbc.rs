#[doc = "Register `PSL_DBC` reader"]
pub type R = crate::R<PslDbcSpec>;
#[doc = "Register `PSL_DBC` writer"]
pub type W = crate::W<PslDbcSpec>;
#[doc = "Field `BC_SEL` reader - PSL Debounce Select"]
pub type BcSelR = crate::FieldReader;
#[doc = "Field `BC_SEL` writer - PSL Debounce Select"]
pub type BcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PSL Debounce Select"]
    #[inline(always)]
    pub fn bc_sel(&self) -> BcSelR {
        BcSelR::new(self.bits & 7)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSL_DBC")
            .field("bc_sel", &self.bc_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - PSL Debounce Select"]
    #[inline(always)]
    pub fn bc_sel(&mut self) -> BcSelW<PslDbcSpec> {
        BcSelW::new(self, 0)
    }
}
#[doc = "PSL Debounce Register (PSL_DBC)\n\nYou can [`read`](crate::Reg::read) this register and get [`psl_dbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl_dbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PslDbcSpec;
impl crate::RegisterSpec for PslDbcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psl_dbc::R`](R) reader structure"]
impl crate::Readable for PslDbcSpec {}
#[doc = "`write(|w| ..)` method takes [`psl_dbc::W`](W) writer structure"]
impl crate::Writable for PslDbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSL_DBC to value 0"]
impl crate::Resettable for PslDbcSpec {
    const RESET_VALUE: u8 = 0;
}
