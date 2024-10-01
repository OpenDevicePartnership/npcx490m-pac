#[doc = "Register `FLM_TCCB3` reader"]
pub type R = crate::R<FlmTccb3Spec>;
#[doc = "Register `FLM_TCCB3` writer"]
pub type W = crate::W<FlmTccb3Spec>;
#[doc = "Field `CMDCE` reader - CMD Count Enable"]
pub type CmdceR = crate::FieldReader;
#[doc = "Field `CMDCE` writer - CMD Count Enable"]
pub type CmdceW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMBCE` reader - CMB Count Enable"]
pub type CmbceR = crate::FieldReader;
#[doc = "Field `CMBCE` writer - CMB Count Enable"]
pub type CmbceW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CMD Count Enable"]
    #[inline(always)]
    pub fn cmdce(&self) -> CmdceR {
        CmdceR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CMB Count Enable"]
    #[inline(always)]
    pub fn cmbce(&self) -> CmbceR {
        CmbceR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_TCCB3")
            .field("cmdce", &self.cmdce())
            .field("cmbce", &self.cmbce())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CMD Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdce(&mut self) -> CmdceW<FlmTccb3Spec> {
        CmdceW::new(self, 0)
    }
    #[doc = "Bits 8:15 - CMB Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmbce(&mut self) -> CmbceW<FlmTccb3Spec> {
        CmbceW::new(self, 8)
    }
}
#[doc = "FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tccb3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tccb3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmTccb3Spec;
impl crate::RegisterSpec for FlmTccb3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_tccb3::R`](R) reader structure"]
impl crate::Readable for FlmTccb3Spec {}
#[doc = "`write(|w| ..)` method takes [`flm_tccb3::W`](W) writer structure"]
impl crate::Writable for FlmTccb3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_TCCB3 to value 0"]
impl crate::Resettable for FlmTccb3Spec {
    const RESET_VALUE: u32 = 0;
}
