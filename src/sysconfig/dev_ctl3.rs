#[doc = "Register `DEV_CTL3` reader"]
pub type R = crate::R<DevCtl3Spec>;
#[doc = "Register `DEV_CTL3` writer"]
pub type W = crate::W<DevCtl3Spec>;
#[doc = "Field `RNGINT_MD` reader - RNG Modules Interrupt Mode"]
pub type RngintMdR = crate::BitReader;
#[doc = "Field `RNGINT_MD` writer - RNG Modules Interrupt Mode"]
pub type RngintMdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FVCC1_PURST_EN` reader - Force VCC1 Power-Up Reset Enable"]
pub type Fvcc1PurstEnR = crate::BitReader;
#[doc = "Field `FVCC1_PURST_EN` writer - Force VCC1 Power-Up Reset Enable"]
pub type Fvcc1PurstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C1_MS` reader - I3CI1 Module Controller/Target Select"]
pub type I3c1MsR = crate::BitReader;
#[doc = "Field `I3C1_MS` writer - I3CI1 Module Controller/Target Select"]
pub type I3c1MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C2_MS` reader - I3CI2 Module Controller/Target Select"]
pub type I3c2MsR = crate::BitReader;
#[doc = "Field `I3C2_MS` writer - I3CI2 Module Controller/Target Select"]
pub type I3c2MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C3_MS` reader - I3CI3 Module Controller/Target Select"]
pub type I3c3MsR = crate::BitReader;
#[doc = "Field `I3C3_MS` writer - I3CI3 Module Controller/Target Select"]
pub type I3c3MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIO_CLK_SEL` reader - SIO Modules Clock Select"]
pub type SioClkSelR = crate::FieldReader;
#[doc = "Field `SIO_CLK_SEL` writer - SIO Modules Clock Select"]
pub type SioClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - RNG Modules Interrupt Mode"]
    #[inline(always)]
    pub fn rngint_md(&self) -> RngintMdR {
        RngintMdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force VCC1 Power-Up Reset Enable"]
    #[inline(always)]
    pub fn fvcc1_purst_en(&self) -> Fvcc1PurstEnR {
        Fvcc1PurstEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I3CI1 Module Controller/Target Select"]
    #[inline(always)]
    pub fn i3c1_ms(&self) -> I3c1MsR {
        I3c1MsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I3CI2 Module Controller/Target Select"]
    #[inline(always)]
    pub fn i3c2_ms(&self) -> I3c2MsR {
        I3c2MsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I3CI3 Module Controller/Target Select"]
    #[inline(always)]
    pub fn i3c3_ms(&self) -> I3c3MsR {
        I3c3MsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SIO Modules Clock Select"]
    #[inline(always)]
    pub fn sio_clk_sel(&self) -> SioClkSelR {
        SioClkSelR::new((self.bits >> 6) & 3)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEV_CTL3")
            .field("rngint_md", &self.rngint_md())
            .field("fvcc1_purst_en", &self.fvcc1_purst_en())
            .field("i3c1_ms", &self.i3c1_ms())
            .field("i3c2_ms", &self.i3c2_ms())
            .field("i3c3_ms", &self.i3c3_ms())
            .field("sio_clk_sel", &self.sio_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - RNG Modules Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rngint_md(&mut self) -> RngintMdW<DevCtl3Spec> {
        RngintMdW::new(self, 1)
    }
    #[doc = "Bit 2 - Force VCC1 Power-Up Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fvcc1_purst_en(&mut self) -> Fvcc1PurstEnW<DevCtl3Spec> {
        Fvcc1PurstEnW::new(self, 2)
    }
    #[doc = "Bit 3 - I3CI1 Module Controller/Target Select"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1_ms(&mut self) -> I3c1MsW<DevCtl3Spec> {
        I3c1MsW::new(self, 3)
    }
    #[doc = "Bit 4 - I3CI2 Module Controller/Target Select"]
    #[inline(always)]
    #[must_use]
    pub fn i3c2_ms(&mut self) -> I3c2MsW<DevCtl3Spec> {
        I3c2MsW::new(self, 4)
    }
    #[doc = "Bit 5 - I3CI3 Module Controller/Target Select"]
    #[inline(always)]
    #[must_use]
    pub fn i3c3_ms(&mut self) -> I3c3MsW<DevCtl3Spec> {
        I3c3MsW::new(self, 5)
    }
    #[doc = "Bits 6:7 - SIO Modules Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn sio_clk_sel(&mut self) -> SioClkSelW<DevCtl3Spec> {
        SioClkSelW::new(self, 6)
    }
}
#[doc = "Device Control 3 Register (DEV_CTL3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCtl3Spec;
impl crate::RegisterSpec for DevCtl3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dev_ctl3::R`](R) reader structure"]
impl crate::Readable for DevCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_ctl3::W`](W) writer structure"]
impl crate::Writable for DevCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEV_CTL3 to value 0"]
impl crate::Resettable for DevCtl3Spec {
    const RESET_VALUE: u8 = 0;
}
