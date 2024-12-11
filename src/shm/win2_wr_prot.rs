#[doc = "Register `WIN2_WR_PROT` reader"]
pub type R = crate::R<Win2WrProtSpec>;
#[doc = "Register `WIN2_WR_PROT` writer"]
pub type W = crate::W<Win2WrProtSpec>;
#[doc = "Field `RW2_WP0` reader - RAM Access Window 2 Write Protect 0"]
pub type Rw2Wp0R = crate::BitReader;
#[doc = "Field `RW2_WP0` writer - RAM Access Window 2 Write Protect 0"]
pub type Rw2Wp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW2_WP1` reader - RAM Access Window 2 Write Protect 1"]
pub type Rw2Wp1R = crate::BitReader;
#[doc = "Field `RW2_WP1` writer - RAM Access Window 2 Write Protect 1"]
pub type Rw2Wp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW2_WP2` reader - RAM Access Window 2 Write Protect 2"]
pub type Rw2Wp2R = crate::BitReader;
#[doc = "Field `RW2_WP2` writer - RAM Access Window 2 Write Protect 2"]
pub type Rw2Wp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW2_WP3` reader - RAM Access Window 2 Write Protect 3"]
pub type Rw2Wp3R = crate::BitReader;
#[doc = "Field `RW2_WP3` writer - RAM Access Window 2 Write Protect 3"]
pub type Rw2Wp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW2_WP4` reader - RAM Access Window 2 Write Protect 4"]
pub type Rw2Wp4R = crate::BitReader;
#[doc = "Field `RW2_WP4` writer - RAM Access Window 2 Write Protect 4"]
pub type Rw2Wp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW2_WP5` reader - RAM Access Window 2 Write Protect 5"]
pub type Rw2Wp5R = crate::BitReader;
#[doc = "Field `RW2_WP5` writer - RAM Access Window 2 Write Protect 5"]
pub type Rw2Wp5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW2_WP6` reader - RAM Access Window 2 Write Protect 6"]
pub type Rw2Wp6R = crate::BitReader;
#[doc = "Field `RW2_WP6` writer - RAM Access Window 2 Write Protect 6"]
pub type Rw2Wp6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW2_WP7` reader - RAM Access Window 2 Write Protect 7"]
pub type Rw2Wp7R = crate::BitReader;
#[doc = "Field `RW2_WP7` writer - RAM Access Window 2 Write Protect 7"]
pub type Rw2Wp7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM Access Window 2 Write Protect 0"]
    #[inline(always)]
    pub fn rw2_wp0(&self) -> Rw2Wp0R {
        Rw2Wp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Access Window 2 Write Protect 1"]
    #[inline(always)]
    pub fn rw2_wp1(&self) -> Rw2Wp1R {
        Rw2Wp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Access Window 2 Write Protect 2"]
    #[inline(always)]
    pub fn rw2_wp2(&self) -> Rw2Wp2R {
        Rw2Wp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM Access Window 2 Write Protect 3"]
    #[inline(always)]
    pub fn rw2_wp3(&self) -> Rw2Wp3R {
        Rw2Wp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RAM Access Window 2 Write Protect 4"]
    #[inline(always)]
    pub fn rw2_wp4(&self) -> Rw2Wp4R {
        Rw2Wp4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAM Access Window 2 Write Protect 5"]
    #[inline(always)]
    pub fn rw2_wp5(&self) -> Rw2Wp5R {
        Rw2Wp5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RAM Access Window 2 Write Protect 6"]
    #[inline(always)]
    pub fn rw2_wp6(&self) -> Rw2Wp6R {
        Rw2Wp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RAM Access Window 2 Write Protect 7"]
    #[inline(always)]
    pub fn rw2_wp7(&self) -> Rw2Wp7R {
        Rw2Wp7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIN2_WR_PROT")
            .field("rw2_wp0", &self.rw2_wp0())
            .field("rw2_wp1", &self.rw2_wp1())
            .field("rw2_wp2", &self.rw2_wp2())
            .field("rw2_wp3", &self.rw2_wp3())
            .field("rw2_wp4", &self.rw2_wp4())
            .field("rw2_wp5", &self.rw2_wp5())
            .field("rw2_wp6", &self.rw2_wp6())
            .field("rw2_wp7", &self.rw2_wp7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM Access Window 2 Write Protect 0"]
    #[inline(always)]
    pub fn rw2_wp0(&mut self) -> Rw2Wp0W<Win2WrProtSpec> {
        Rw2Wp0W::new(self, 0)
    }
    #[doc = "Bit 1 - RAM Access Window 2 Write Protect 1"]
    #[inline(always)]
    pub fn rw2_wp1(&mut self) -> Rw2Wp1W<Win2WrProtSpec> {
        Rw2Wp1W::new(self, 1)
    }
    #[doc = "Bit 2 - RAM Access Window 2 Write Protect 2"]
    #[inline(always)]
    pub fn rw2_wp2(&mut self) -> Rw2Wp2W<Win2WrProtSpec> {
        Rw2Wp2W::new(self, 2)
    }
    #[doc = "Bit 3 - RAM Access Window 2 Write Protect 3"]
    #[inline(always)]
    pub fn rw2_wp3(&mut self) -> Rw2Wp3W<Win2WrProtSpec> {
        Rw2Wp3W::new(self, 3)
    }
    #[doc = "Bit 4 - RAM Access Window 2 Write Protect 4"]
    #[inline(always)]
    pub fn rw2_wp4(&mut self) -> Rw2Wp4W<Win2WrProtSpec> {
        Rw2Wp4W::new(self, 4)
    }
    #[doc = "Bit 5 - RAM Access Window 2 Write Protect 5"]
    #[inline(always)]
    pub fn rw2_wp5(&mut self) -> Rw2Wp5W<Win2WrProtSpec> {
        Rw2Wp5W::new(self, 5)
    }
    #[doc = "Bit 6 - RAM Access Window 2 Write Protect 6"]
    #[inline(always)]
    pub fn rw2_wp6(&mut self) -> Rw2Wp6W<Win2WrProtSpec> {
        Rw2Wp6W::new(self, 6)
    }
    #[doc = "Bit 7 - RAM Access Window 2 Write Protect 7"]
    #[inline(always)]
    pub fn rw2_wp7(&mut self) -> Rw2Wp7W<Win2WrProtSpec> {
        Rw2Wp7W::new(self, 7)
    }
}
#[doc = "Shared Access Window 2 Write Protect Register (WIN2_WR_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win2_wr_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win2_wr_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2WrProtSpec;
impl crate::RegisterSpec for Win2WrProtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`win2_wr_prot::R`](R) reader structure"]
impl crate::Readable for Win2WrProtSpec {}
#[doc = "`write(|w| ..)` method takes [`win2_wr_prot::W`](W) writer structure"]
impl crate::Writable for Win2WrProtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WIN2_WR_PROT to value 0"]
impl crate::Resettable for Win2WrProtSpec {
    const RESET_VALUE: u8 = 0;
}
