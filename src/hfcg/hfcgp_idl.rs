#[doc = "Register `HFCGP_IDL` reader"]
pub type R = crate::R<HfcgpIdlSpec>;
#[doc = "Register `HFCGP_IDL` writer"]
pub type W = crate::W<HfcgpIdlSpec>;
#[doc = "Field `FPRED_IDL_EN` reader - Core Clock Prescaler Divider in Idle Value Enable"]
pub type FpredIdlEnR = crate::BitReader;
#[doc = "Field `FPRED_IDL_EN` writer - Core Clock Prescaler Divider in Idle Value Enable"]
pub type FpredIdlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPRED_IDL` reader - Core Clock Prescaler Divider in Idle Value"]
pub type FpredIdlR = crate::FieldReader;
#[doc = "Field `FPRED_IDL` writer - Core Clock Prescaler Divider in Idle Value"]
pub type FpredIdlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 3 - Core Clock Prescaler Divider in Idle Value Enable"]
    #[inline(always)]
    pub fn fpred_idl_en(&self) -> FpredIdlEnR {
        FpredIdlEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Core Clock Prescaler Divider in Idle Value"]
    #[inline(always)]
    pub fn fpred_idl(&self) -> FpredIdlR {
        FpredIdlR::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCGP_IDL")
            .field("fpred_idl_en", &self.fpred_idl_en())
            .field("fpred_idl", &self.fpred_idl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Core Clock Prescaler Divider in Idle Value Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpred_idl_en(&mut self) -> FpredIdlEnW<HfcgpIdlSpec> {
        FpredIdlEnW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Core Clock Prescaler Divider in Idle Value"]
    #[inline(always)]
    #[must_use]
    pub fn fpred_idl(&mut self) -> FpredIdlW<HfcgpIdlSpec> {
        FpredIdlW::new(self, 4)
    }
}
#[doc = "HFCG Prescaler in Idle Register (HFCGP_IDL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgp_idl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgp_idl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfcgpIdlSpec;
impl crate::RegisterSpec for HfcgpIdlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcgp_idl::R`](R) reader structure"]
impl crate::Readable for HfcgpIdlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfcgp_idl::W`](W) writer structure"]
impl crate::Writable for HfcgpIdlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCGP_IDL to value 0"]
impl crate::Resettable for HfcgpIdlSpec {
    const RESET_VALUE: u8 = 0;
}
