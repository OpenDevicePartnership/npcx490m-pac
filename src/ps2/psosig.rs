#[doc = "Register `PSOSIG` reader"]
pub type R = crate::R<PsosigSpec>;
#[doc = "Register `PSOSIG` writer"]
pub type W = crate::W<PsosigSpec>;
#[doc = "Field `WDAT0` reader - Write Data Signal Channel 0"]
pub type Wdat0R = crate::BitReader;
#[doc = "Field `WDAT0` writer - Write Data Signal Channel 0"]
pub type Wdat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDAT1` reader - Write Data Signal Channel 1"]
pub type Wdat1R = crate::BitReader;
#[doc = "Field `WDAT1` writer - Write Data Signal Channel 1"]
pub type Wdat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDAT2` reader - Write Data Signal Channel 2"]
pub type Wdat2R = crate::BitReader;
#[doc = "Field `WDAT2` writer - Write Data Signal Channel 2"]
pub type Wdat2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK0` reader - Enable Channel 0"]
pub type Clk0R = crate::BitReader;
#[doc = "Field `CLK0` writer - Enable Channel 0"]
pub type Clk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK1` reader - Enable Channel 1"]
pub type Clk1R = crate::BitReader;
#[doc = "Field `CLK1` writer - Enable Channel 1"]
pub type Clk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK2` reader - Enable Channel 2"]
pub type Clk2R = crate::BitReader;
#[doc = "Field `CLK2` writer - Enable Channel 2"]
pub type Clk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDAT3` reader - Write Data Signal Channel 3"]
pub type Wdat3R = crate::BitReader;
#[doc = "Field `WDAT3` writer - Write Data Signal Channel 3"]
pub type Wdat3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK3` reader - Enable Channel 3"]
pub type Clk3R = crate::BitReader;
#[doc = "Field `CLK3` writer - Enable Channel 3"]
pub type Clk3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Data Signal Channel 0"]
    #[inline(always)]
    pub fn wdat0(&self) -> Wdat0R {
        Wdat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Data Signal Channel 1"]
    #[inline(always)]
    pub fn wdat1(&self) -> Wdat1R {
        Wdat1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Data Signal Channel 2"]
    #[inline(always)]
    pub fn wdat2(&self) -> Wdat2R {
        Wdat2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Channel 0"]
    #[inline(always)]
    pub fn clk0(&self) -> Clk0R {
        Clk0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Channel 1"]
    #[inline(always)]
    pub fn clk1(&self) -> Clk1R {
        Clk1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Channel 2"]
    #[inline(always)]
    pub fn clk2(&self) -> Clk2R {
        Clk2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Data Signal Channel 3"]
    #[inline(always)]
    pub fn wdat3(&self) -> Wdat3R {
        Wdat3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Channel 3"]
    #[inline(always)]
    pub fn clk3(&self) -> Clk3R {
        Clk3R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSOSIG")
            .field("wdat0", &self.wdat0())
            .field("wdat1", &self.wdat1())
            .field("wdat2", &self.wdat2())
            .field("clk0", &self.clk0())
            .field("clk1", &self.clk1())
            .field("clk2", &self.clk2())
            .field("wdat3", &self.wdat3())
            .field("clk3", &self.clk3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write Data Signal Channel 0"]
    #[inline(always)]
    pub fn wdat0(&mut self) -> Wdat0W<PsosigSpec> {
        Wdat0W::new(self, 0)
    }
    #[doc = "Bit 1 - Write Data Signal Channel 1"]
    #[inline(always)]
    pub fn wdat1(&mut self) -> Wdat1W<PsosigSpec> {
        Wdat1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write Data Signal Channel 2"]
    #[inline(always)]
    pub fn wdat2(&mut self) -> Wdat2W<PsosigSpec> {
        Wdat2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Channel 0"]
    #[inline(always)]
    pub fn clk0(&mut self) -> Clk0W<PsosigSpec> {
        Clk0W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Channel 1"]
    #[inline(always)]
    pub fn clk1(&mut self) -> Clk1W<PsosigSpec> {
        Clk1W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Channel 2"]
    #[inline(always)]
    pub fn clk2(&mut self) -> Clk2W<PsosigSpec> {
        Clk2W::new(self, 5)
    }
    #[doc = "Bit 6 - Write Data Signal Channel 3"]
    #[inline(always)]
    pub fn wdat3(&mut self) -> Wdat3W<PsosigSpec> {
        Wdat3W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Channel 3"]
    #[inline(always)]
    pub fn clk3(&mut self) -> Clk3W<PsosigSpec> {
        Clk3W::new(self, 7)
    }
}
#[doc = "PS/2 Output Signal Register (PSOSIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`psosig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psosig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsosigSpec;
impl crate::RegisterSpec for PsosigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psosig::R`](R) reader structure"]
impl crate::Readable for PsosigSpec {}
#[doc = "`write(|w| ..)` method takes [`psosig::W`](W) writer structure"]
impl crate::Writable for PsosigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSOSIG to value 0"]
impl crate::Resettable for PsosigSpec {
    const RESET_VALUE: u8 = 0;
}
