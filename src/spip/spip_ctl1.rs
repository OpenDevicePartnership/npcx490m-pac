#[doc = "Register `SPIP_CTL1` reader"]
pub type R = crate::R<SpipCtl1Spec>;
#[doc = "Register `SPIP_CTL1` writer"]
pub type W = crate::W<SpipCtl1Spec>;
#[doc = "Field `SPIEN` reader - SPI Enable"]
pub type SpienR = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPI Enable"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD` reader - Data Interface Mode"]
pub type ModR = crate::BitReader;
#[doc = "Field `MOD` writer - Data Interface Mode"]
pub type ModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIR` reader - Enable Interrupt for Read"]
pub type EirR = crate::BitReader;
#[doc = "Field `EIR` writer - Enable Interrupt for Read"]
pub type EirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIW` reader - Enable Interrupt for Write"]
pub type EiwR = crate::BitReader;
#[doc = "Field `EIW` writer - Enable Interrupt for Write"]
pub type EiwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCM` reader - Clocking Mode"]
pub type ScmR = crate::BitReader;
#[doc = "Field `SCM` writer - Clocking Mode"]
pub type ScmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCIDL` reader - Value of SPI_SCLK when Bus is Idle"]
pub type ScidlR = crate::BitReader;
#[doc = "Field `SCIDL` writer - Value of SPI_SCLK when Bus is Idle"]
pub type ScidlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCDV6_0` reader - Shift Clock Divider Value"]
pub type Scdv6_0R = crate::FieldReader;
#[doc = "Field `SCDV6_0` writer - Shift Clock Divider Value"]
pub type Scdv6_0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - SPI Enable"]
    #[inline(always)]
    pub fn spien(&self) -> SpienR {
        SpienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Data Interface Mode"]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Interrupt for Read"]
    #[inline(always)]
    pub fn eir(&self) -> EirR {
        EirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Interrupt for Write"]
    #[inline(always)]
    pub fn eiw(&self) -> EiwR {
        EiwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clocking Mode"]
    #[inline(always)]
    pub fn scm(&self) -> ScmR {
        ScmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Value of SPI_SCLK when Bus is Idle"]
    #[inline(always)]
    pub fn scidl(&self) -> ScidlR {
        ScidlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Shift Clock Divider Value"]
    #[inline(always)]
    pub fn scdv6_0(&self) -> Scdv6_0R {
        Scdv6_0R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIP_CTL1")
            .field("spien", &self.spien())
            .field("mod_", &self.mod_())
            .field("eir", &self.eir())
            .field("eiw", &self.eiw())
            .field("scm", &self.scm())
            .field("scidl", &self.scidl())
            .field("scdv6_0", &self.scdv6_0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SpienW<SpipCtl1Spec> {
        SpienW::new(self, 0)
    }
    #[doc = "Bit 2 - Data Interface Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> ModW<SpipCtl1Spec> {
        ModW::new(self, 2)
    }
    #[doc = "Bit 5 - Enable Interrupt for Read"]
    #[inline(always)]
    #[must_use]
    pub fn eir(&mut self) -> EirW<SpipCtl1Spec> {
        EirW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Interrupt for Write"]
    #[inline(always)]
    #[must_use]
    pub fn eiw(&mut self) -> EiwW<SpipCtl1Spec> {
        EiwW::new(self, 6)
    }
    #[doc = "Bit 7 - Clocking Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scm(&mut self) -> ScmW<SpipCtl1Spec> {
        ScmW::new(self, 7)
    }
    #[doc = "Bit 8 - Value of SPI_SCLK when Bus is Idle"]
    #[inline(always)]
    #[must_use]
    pub fn scidl(&mut self) -> ScidlW<SpipCtl1Spec> {
        ScidlW::new(self, 8)
    }
    #[doc = "Bits 9:15 - Shift Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn scdv6_0(&mut self) -> Scdv6_0W<SpipCtl1Spec> {
        Scdv6_0W::new(self, 9)
    }
}
#[doc = "SPIP Control 1 Register (SPIP_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`spip_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spip_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpipCtl1Spec;
impl crate::RegisterSpec for SpipCtl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spip_ctl1::R`](R) reader structure"]
impl crate::Readable for SpipCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`spip_ctl1::W`](W) writer structure"]
impl crate::Writable for SpipCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPIP_CTL1 to value 0"]
impl crate::Resettable for SpipCtl1Spec {
    const RESET_VALUE: u16 = 0;
}
