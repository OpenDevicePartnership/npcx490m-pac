#[doc = "Register `FLM_TCCA2` reader"]
pub type R = crate::R<FlmTcca2Spec>;
#[doc = "Register `FLM_TCCA2` writer"]
pub type W = crate::W<FlmTcca2Spec>;
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
        f.debug_struct("FLM_TCCA2")
            .field("cmdce", &self.cmdce())
            .field("cmbce", &self.cmbce())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CMD Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdce(&mut self) -> CmdceW<FlmTcca2Spec> {
        CmdceW::new(self, 0)
    }
    #[doc = "Bits 8:15 - CMB Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmbce(&mut self) -> CmbceW<FlmTcca2Spec> {
        CmbceW::new(self, 8)
    }
}
#[doc = "FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcca2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcca2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmTcca2Spec;
impl crate::RegisterSpec for FlmTcca2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_tcca2::R`](R) reader structure"]
impl crate::Readable for FlmTcca2Spec {}
#[doc = "`write(|w| ..)` method takes [`flm_tcca2::W`](W) writer structure"]
impl crate::Writable for FlmTcca2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_TCCA2 to value 0"]
impl crate::Resettable for FlmTcca2Spec {
    const RESET_VALUE: u32 = 0;
}
