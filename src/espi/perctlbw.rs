#[doc = "Register `PERCTLBW` reader"]
pub type R = crate::R<PerctlbwSpec>;
#[doc = "Register `PERCTLBW` writer"]
pub type W = crate::W<PerctlbwSpec>;
#[doc = "Field `BMWDMATHRESH` reader - Bus Master Write DMA Request Threshold"]
pub type BmwdmathreshR = crate::FieldReader;
#[doc = "Field `BMWDMATHRESH` writer - Bus Master Write DMA Request Threshold"]
pub type BmwdmathreshW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BMWBURSTSIZE` reader - Bus Master Burst Mode Write Transfer Size"]
pub type BmwburstsizeR = crate::FieldReader;
#[doc = "Field `BMWBURSTSIZE` writer - Bus Master Burst Mode Write Transfer Size"]
pub type BmwburstsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BMWBRSTEN` reader - Bus Master Burst Mode Write Enable"]
pub type BmwbrstenR = crate::BitReader;
#[doc = "Field `BMWBRSTEN` writer - Bus Master Burst Mode Write Enable"]
pub type BmwbrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMWBURST_BEMPTY` reader - Bus Master Burst Mode Write Transmit Buffer Empty"]
pub type BmwburstBemptyR = crate::BitReader;
#[doc = "Field `BMWBURST_BEMPTY` writer - Bus Master Burst Mode Write Transmit Buffer Empty"]
pub type BmwburstBemptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 3:4 - Bus Master Write DMA Request Threshold"]
    #[inline(always)]
    pub fn bmwdmathresh(&self) -> BmwdmathreshR {
        BmwdmathreshR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:12 - Bus Master Burst Mode Write Transfer Size"]
    #[inline(always)]
    pub fn bmwburstsize(&self) -> BmwburstsizeR {
        BmwburstsizeR::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Bus Master Burst Mode Write Enable"]
    #[inline(always)]
    pub fn bmwbrsten(&self) -> BmwbrstenR {
        BmwbrstenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bus Master Burst Mode Write Transmit Buffer Empty"]
    #[inline(always)]
    pub fn bmwburst_bempty(&self) -> BmwburstBemptyR {
        BmwburstBemptyR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERCTLBW")
            .field("bmwdmathresh", &self.bmwdmathresh())
            .field("bmwburstsize", &self.bmwburstsize())
            .field("bmwbrsten", &self.bmwbrsten())
            .field("bmwburst_bempty", &self.bmwburst_bempty())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:4 - Bus Master Write DMA Request Threshold"]
    #[inline(always)]
    pub fn bmwdmathresh(&mut self) -> BmwdmathreshW<PerctlbwSpec> {
        BmwdmathreshW::new(self, 3)
    }
    #[doc = "Bits 5:12 - Bus Master Burst Mode Write Transfer Size"]
    #[inline(always)]
    pub fn bmwburstsize(&mut self) -> BmwburstsizeW<PerctlbwSpec> {
        BmwburstsizeW::new(self, 5)
    }
    #[doc = "Bit 16 - Bus Master Burst Mode Write Enable"]
    #[inline(always)]
    pub fn bmwbrsten(&mut self) -> BmwbrstenW<PerctlbwSpec> {
        BmwbrstenW::new(self, 16)
    }
    #[doc = "Bit 17 - Bus Master Burst Mode Write Transmit Buffer Empty"]
    #[inline(always)]
    pub fn bmwburst_bempty(&mut self) -> BmwburstBemptyW<PerctlbwSpec> {
        BmwburstBemptyW::new(self, 17)
    }
}
#[doc = "Peripheral Channel Control for Bus Master Write Register (PERCTLBW)\n\nYou can [`read`](crate::Reg::read) this register and get [`perctlbw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perctlbw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerctlbwSpec;
impl crate::RegisterSpec for PerctlbwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perctlbw::R`](R) reader structure"]
impl crate::Readable for PerctlbwSpec {}
#[doc = "`write(|w| ..)` method takes [`perctlbw::W`](W) writer structure"]
impl crate::Writable for PerctlbwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERCTLBW to value 0"]
impl crate::Resettable for PerctlbwSpec {
    const RESET_VALUE: u32 = 0;
}
