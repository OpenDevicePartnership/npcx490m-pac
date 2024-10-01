#[doc = "Register `PSISIG` reader"]
pub type R = crate::R<PsisigSpec>;
#[doc = "Register `PSISIG` writer"]
pub type W = crate::W<PsisigSpec>;
#[doc = "Field `RDAT0` reader - Read Data Signal Channel 0"]
pub type Rdat0R = crate::BitReader;
#[doc = "Field `RDAT0` writer - Read Data Signal Channel 0"]
pub type Rdat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDAT1` reader - Read Data Signal Channel 1"]
pub type Rdat1R = crate::BitReader;
#[doc = "Field `RDAT1` writer - Read Data Signal Channel 1"]
pub type Rdat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDAT2` reader - Read Data Signal Channel 2"]
pub type Rdat2R = crate::BitReader;
#[doc = "Field `RDAT2` writer - Read Data Signal Channel 2"]
pub type Rdat2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCLK0` reader - Read Clock Signal Channel 0"]
pub type Rclk0R = crate::BitReader;
#[doc = "Field `RCLK0` writer - Read Clock Signal Channel 0"]
pub type Rclk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCLK1` reader - Read Clock Signal Channel 1"]
pub type Rclk1R = crate::BitReader;
#[doc = "Field `RCLK1` writer - Read Clock Signal Channel 1"]
pub type Rclk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCLK2` reader - Read Clock Signal Channel 2"]
pub type Rclk2R = crate::BitReader;
#[doc = "Field `RCLK2` writer - Read Clock Signal Channel 2"]
pub type Rclk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDAT3` reader - Read Data Signal Channel 3"]
pub type Rdat3R = crate::BitReader;
#[doc = "Field `RDAT3` writer - Read Data Signal Channel 3"]
pub type Rdat3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCLK3` reader - Read Clock Signal Channel 3"]
pub type Rclk3R = crate::BitReader;
#[doc = "Field `RCLK3` writer - Read Clock Signal Channel 3"]
pub type Rclk3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read Data Signal Channel 0"]
    #[inline(always)]
    pub fn rdat0(&self) -> Rdat0R {
        Rdat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Data Signal Channel 1"]
    #[inline(always)]
    pub fn rdat1(&self) -> Rdat1R {
        Rdat1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read Data Signal Channel 2"]
    #[inline(always)]
    pub fn rdat2(&self) -> Rdat2R {
        Rdat2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read Clock Signal Channel 0"]
    #[inline(always)]
    pub fn rclk0(&self) -> Rclk0R {
        Rclk0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Clock Signal Channel 1"]
    #[inline(always)]
    pub fn rclk1(&self) -> Rclk1R {
        Rclk1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read Clock Signal Channel 2"]
    #[inline(always)]
    pub fn rclk2(&self) -> Rclk2R {
        Rclk2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read Data Signal Channel 3"]
    #[inline(always)]
    pub fn rdat3(&self) -> Rdat3R {
        Rdat3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read Clock Signal Channel 3"]
    #[inline(always)]
    pub fn rclk3(&self) -> Rclk3R {
        Rclk3R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSISIG")
            .field("rdat0", &self.rdat0())
            .field("rdat1", &self.rdat1())
            .field("rdat2", &self.rdat2())
            .field("rclk0", &self.rclk0())
            .field("rclk1", &self.rclk1())
            .field("rclk2", &self.rclk2())
            .field("rdat3", &self.rdat3())
            .field("rclk3", &self.rclk3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Read Data Signal Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn rdat0(&mut self) -> Rdat0W<PsisigSpec> {
        Rdat0W::new(self, 0)
    }
    #[doc = "Bit 1 - Read Data Signal Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn rdat1(&mut self) -> Rdat1W<PsisigSpec> {
        Rdat1W::new(self, 1)
    }
    #[doc = "Bit 2 - Read Data Signal Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn rdat2(&mut self) -> Rdat2W<PsisigSpec> {
        Rdat2W::new(self, 2)
    }
    #[doc = "Bit 3 - Read Clock Signal Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn rclk0(&mut self) -> Rclk0W<PsisigSpec> {
        Rclk0W::new(self, 3)
    }
    #[doc = "Bit 4 - Read Clock Signal Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn rclk1(&mut self) -> Rclk1W<PsisigSpec> {
        Rclk1W::new(self, 4)
    }
    #[doc = "Bit 5 - Read Clock Signal Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn rclk2(&mut self) -> Rclk2W<PsisigSpec> {
        Rclk2W::new(self, 5)
    }
    #[doc = "Bit 6 - Read Data Signal Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn rdat3(&mut self) -> Rdat3W<PsisigSpec> {
        Rdat3W::new(self, 6)
    }
    #[doc = "Bit 7 - Read Clock Signal Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn rclk3(&mut self) -> Rclk3W<PsisigSpec> {
        Rclk3W::new(self, 7)
    }
}
#[doc = "PS/2 Input Signal Register (PSISIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`psisig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psisig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsisigSpec;
impl crate::RegisterSpec for PsisigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psisig::R`](R) reader structure"]
impl crate::Readable for PsisigSpec {}
#[doc = "`write(|w| ..)` method takes [`psisig::W`](W) writer structure"]
impl crate::Writable for PsisigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSISIG to value 0"]
impl crate::Resettable for PsisigSpec {
    const RESET_VALUE: u8 = 0;
}
