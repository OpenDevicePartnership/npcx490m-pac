#[doc = "Register `WIN4_WR_PROT` reader"]
pub type R = crate::R<Win4WrProtSpec>;
#[doc = "Register `WIN4_WR_PROT` writer"]
pub type W = crate::W<Win4WrProtSpec>;
#[doc = "Field `RW4_WP0` reader - RAM Access Window 4 Write Protect 0"]
pub type Rw4Wp0R = crate::BitReader;
#[doc = "Field `RW4_WP0` writer - RAM Access Window 4 Write Protect 0"]
pub type Rw4Wp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW4_WP1` reader - RAM Access Window 4 Write Protect 1"]
pub type Rw4Wp1R = crate::BitReader;
#[doc = "Field `RW4_WP1` writer - RAM Access Window 4 Write Protect 1"]
pub type Rw4Wp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW4_WP2` reader - RAM Access Window 4 Write Protect 2"]
pub type Rw4Wp2R = crate::BitReader;
#[doc = "Field `RW4_WP2` writer - RAM Access Window 4 Write Protect 2"]
pub type Rw4Wp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW4_WP3` reader - RAM Access Window 4 Write Protect 3"]
pub type Rw4Wp3R = crate::BitReader;
#[doc = "Field `RW4_WP3` writer - RAM Access Window 4 Write Protect 3"]
pub type Rw4Wp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW4_WP4` reader - RAM Access Window 4 Write Protect 4"]
pub type Rw4Wp4R = crate::BitReader;
#[doc = "Field `RW4_WP4` writer - RAM Access Window 4 Write Protect 4"]
pub type Rw4Wp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW4_WP5` reader - RAM Access Window 4 Write Protect 5"]
pub type Rw4Wp5R = crate::BitReader;
#[doc = "Field `RW4_WP5` writer - RAM Access Window 4 Write Protect 5"]
pub type Rw4Wp5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW4_WP6` reader - RAM Access Window 4 Write Protect 6"]
pub type Rw4Wp6R = crate::BitReader;
#[doc = "Field `RW4_WP6` writer - RAM Access Window 4 Write Protect 6"]
pub type Rw4Wp6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW4_WP7` reader - RAM Access Window 4 Write Protect 7"]
pub type Rw4Wp7R = crate::BitReader;
#[doc = "Field `RW4_WP7` writer - RAM Access Window 4 Write Protect 7"]
pub type Rw4Wp7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM Access Window 4 Write Protect 0"]
    #[inline(always)]
    pub fn rw4_wp0(&self) -> Rw4Wp0R {
        Rw4Wp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Access Window 4 Write Protect 1"]
    #[inline(always)]
    pub fn rw4_wp1(&self) -> Rw4Wp1R {
        Rw4Wp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Access Window 4 Write Protect 2"]
    #[inline(always)]
    pub fn rw4_wp2(&self) -> Rw4Wp2R {
        Rw4Wp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM Access Window 4 Write Protect 3"]
    #[inline(always)]
    pub fn rw4_wp3(&self) -> Rw4Wp3R {
        Rw4Wp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RAM Access Window 4 Write Protect 4"]
    #[inline(always)]
    pub fn rw4_wp4(&self) -> Rw4Wp4R {
        Rw4Wp4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAM Access Window 4 Write Protect 5"]
    #[inline(always)]
    pub fn rw4_wp5(&self) -> Rw4Wp5R {
        Rw4Wp5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RAM Access Window 4 Write Protect 6"]
    #[inline(always)]
    pub fn rw4_wp6(&self) -> Rw4Wp6R {
        Rw4Wp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RAM Access Window 4 Write Protect 7"]
    #[inline(always)]
    pub fn rw4_wp7(&self) -> Rw4Wp7R {
        Rw4Wp7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIN4_WR_PROT")
            .field("rw4_wp0", &self.rw4_wp0())
            .field("rw4_wp1", &self.rw4_wp1())
            .field("rw4_wp2", &self.rw4_wp2())
            .field("rw4_wp3", &self.rw4_wp3())
            .field("rw4_wp4", &self.rw4_wp4())
            .field("rw4_wp5", &self.rw4_wp5())
            .field("rw4_wp6", &self.rw4_wp6())
            .field("rw4_wp7", &self.rw4_wp7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM Access Window 4 Write Protect 0"]
    #[inline(always)]
    pub fn rw4_wp0(&mut self) -> Rw4Wp0W<Win4WrProtSpec> {
        Rw4Wp0W::new(self, 0)
    }
    #[doc = "Bit 1 - RAM Access Window 4 Write Protect 1"]
    #[inline(always)]
    pub fn rw4_wp1(&mut self) -> Rw4Wp1W<Win4WrProtSpec> {
        Rw4Wp1W::new(self, 1)
    }
    #[doc = "Bit 2 - RAM Access Window 4 Write Protect 2"]
    #[inline(always)]
    pub fn rw4_wp2(&mut self) -> Rw4Wp2W<Win4WrProtSpec> {
        Rw4Wp2W::new(self, 2)
    }
    #[doc = "Bit 3 - RAM Access Window 4 Write Protect 3"]
    #[inline(always)]
    pub fn rw4_wp3(&mut self) -> Rw4Wp3W<Win4WrProtSpec> {
        Rw4Wp3W::new(self, 3)
    }
    #[doc = "Bit 4 - RAM Access Window 4 Write Protect 4"]
    #[inline(always)]
    pub fn rw4_wp4(&mut self) -> Rw4Wp4W<Win4WrProtSpec> {
        Rw4Wp4W::new(self, 4)
    }
    #[doc = "Bit 5 - RAM Access Window 4 Write Protect 5"]
    #[inline(always)]
    pub fn rw4_wp5(&mut self) -> Rw4Wp5W<Win4WrProtSpec> {
        Rw4Wp5W::new(self, 5)
    }
    #[doc = "Bit 6 - RAM Access Window 4 Write Protect 6"]
    #[inline(always)]
    pub fn rw4_wp6(&mut self) -> Rw4Wp6W<Win4WrProtSpec> {
        Rw4Wp6W::new(self, 6)
    }
    #[doc = "Bit 7 - RAM Access Window 4 Write Protect 7"]
    #[inline(always)]
    pub fn rw4_wp7(&mut self) -> Rw4Wp7W<Win4WrProtSpec> {
        Rw4Wp7W::new(self, 7)
    }
}
#[doc = "Shared Access Window 4 Write Protect Register (WIN4_WR_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win4_wr_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win4_wr_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win4WrProtSpec;
impl crate::RegisterSpec for Win4WrProtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`win4_wr_prot::R`](R) reader structure"]
impl crate::Readable for Win4WrProtSpec {}
#[doc = "`write(|w| ..)` method takes [`win4_wr_prot::W`](W) writer structure"]
impl crate::Writable for Win4WrProtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WIN4_WR_PROT to value 0"]
impl crate::Resettable for Win4WrProtSpec {
    const RESET_VALUE: u8 = 0;
}
