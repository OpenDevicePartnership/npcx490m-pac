#[doc = "Register `DBGCTRL2` reader"]
pub type R = crate::R<Dbgctrl2Spec>;
#[doc = "Register `DBGCTRL2` writer"]
pub type W = crate::W<Dbgctrl2Spec>;
#[doc = "Field `CCDEV_SEL` reader - Closed Case Development Select"]
pub type CcdevSelR = crate::FieldReader;
#[doc = "Field `CCDEV_SEL` writer - Closed Case Development Select"]
pub type CcdevSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `P80_SPEW_EN` reader - Port80 Spew Enable"]
pub type P80SpewEnR = crate::BitReader;
#[doc = "Field `P80_SPEW_EN` writer - Port80 Spew Enable"]
pub type P80SpewEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDEV_DIR` reader - Closed Case Development Connector Direction"]
pub type CcdevDirR = crate::BitReader;
#[doc = "Field `CCDEV_DIR` writer - Closed Case Development Connector Direction"]
pub type CcdevDirW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Closed Case Development Select"]
    #[inline(always)]
    pub fn ccdev_sel(&self) -> CcdevSelR {
        CcdevSelR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 6 - Port80 Spew Enable"]
    #[inline(always)]
    pub fn p80_spew_en(&self) -> P80SpewEnR {
        P80SpewEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Closed Case Development Connector Direction"]
    #[inline(always)]
    pub fn ccdev_dir(&self) -> CcdevDirR {
        CcdevDirR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGCTRL2")
            .field("ccdev_sel", &self.ccdev_sel())
            .field("p80_spew_en", &self.p80_spew_en())
            .field("ccdev_dir", &self.ccdev_dir())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Closed Case Development Select"]
    #[inline(always)]
    pub fn ccdev_sel(&mut self) -> CcdevSelW<Dbgctrl2Spec> {
        CcdevSelW::new(self, 0)
    }
    #[doc = "Bit 6 - Port80 Spew Enable"]
    #[inline(always)]
    pub fn p80_spew_en(&mut self) -> P80SpewEnW<Dbgctrl2Spec> {
        P80SpewEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Closed Case Development Connector Direction"]
    #[inline(always)]
    pub fn ccdev_dir(&mut self) -> CcdevDirW<Dbgctrl2Spec> {
        CcdevDirW::new(self, 7)
    }
}
#[doc = "Debug Control 2 Register (DBGCTRL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbgctrl2Spec;
impl crate::RegisterSpec for Dbgctrl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgctrl2::R`](R) reader structure"]
impl crate::Readable for Dbgctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`dbgctrl2::W`](W) writer structure"]
impl crate::Writable for Dbgctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DBGCTRL2 to value 0"]
impl crate::Resettable for Dbgctrl2Spec {
    const RESET_VALUE: u8 = 0;
}
