#[doc = "Register `Dn_FIU_DMM_CYC` reader"]
pub type R = crate::R<DnFiuDmmCycSpec>;
#[doc = "Register `Dn_FIU_DMM_CYC` writer"]
pub type W = crate::W<DnFiuDmmCycSpec>;
#[doc = "Field `DN_DMM_CYC_NUM` reader - Device 'n' Dummy Cycles Number"]
pub type DnDmmCycNumR = crate::FieldReader;
#[doc = "Field `DN_DMM_CYC_NUM` writer - Device 'n' Dummy Cycles Number"]
pub type DnDmmCycNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Device 'n' Dummy Cycles Number"]
    #[inline(always)]
    pub fn dn_dmm_cyc_num(&self) -> DnDmmCycNumR {
        DnDmmCycNumR::new(self.bits & 0x1f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dn_FIU_DMM_CYC")
            .field("dn_dmm_cyc_num", &self.dn_dmm_cyc_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Device 'n' Dummy Cycles Number"]
    #[inline(always)]
    pub fn dn_dmm_cyc_num(&mut self) -> DnDmmCycNumW<DnFiuDmmCycSpec> {
        DnDmmCycNumW::new(self, 0)
    }
}
#[doc = "Device 'n' FIU Dummy Cycle Register (Dn_FIU_DMM_CYC)\n\nYou can [`read`](crate::Reg::read) this register and get [`dn_fiu_dmm_cyc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dn_fiu_dmm_cyc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DnFiuDmmCycSpec;
impl crate::RegisterSpec for DnFiuDmmCycSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dn_fiu_dmm_cyc::R`](R) reader structure"]
impl crate::Readable for DnFiuDmmCycSpec {}
#[doc = "`write(|w| ..)` method takes [`dn_fiu_dmm_cyc::W`](W) writer structure"]
impl crate::Writable for DnFiuDmmCycSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets Dn_FIU_DMM_CYC to value 0"]
impl crate::Resettable for DnFiuDmmCycSpec {
    const RESET_VALUE: u8 = 0;
}
