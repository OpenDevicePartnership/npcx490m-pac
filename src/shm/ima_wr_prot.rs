#[doc = "Register `IMA_WR_PROT` reader"]
pub type R = crate::R<ImaWrProtSpec>;
#[doc = "Register `IMA_WR_PROT` writer"]
pub type W = crate::W<ImaWrProtSpec>;
#[doc = "Field `IMA_WP0` reader - RAM Access Window 2 Write Protect 0"]
pub type ImaWp0R = crate::BitReader;
#[doc = "Field `IMA_WP0` writer - RAM Access Window 2 Write Protect 0"]
pub type ImaWp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_WP1` reader - RAM Access Window 2 Write Protect 1"]
pub type ImaWp1R = crate::BitReader;
#[doc = "Field `IMA_WP1` writer - RAM Access Window 2 Write Protect 1"]
pub type ImaWp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_WP2` reader - RAM Access Window 2 Write Protect 2"]
pub type ImaWp2R = crate::BitReader;
#[doc = "Field `IMA_WP2` writer - RAM Access Window 2 Write Protect 2"]
pub type ImaWp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_WP3` reader - RAM Access Window 2 Write Protect 3"]
pub type ImaWp3R = crate::BitReader;
#[doc = "Field `IMA_WP3` writer - RAM Access Window 2 Write Protect 3"]
pub type ImaWp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_WP4` reader - RAM Access Window 2 Write Protect 4"]
pub type ImaWp4R = crate::BitReader;
#[doc = "Field `IMA_WP4` writer - RAM Access Window 2 Write Protect 4"]
pub type ImaWp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_WP5` reader - RAM Access Window 2 Write Protect 5"]
pub type ImaWp5R = crate::BitReader;
#[doc = "Field `IMA_WP5` writer - RAM Access Window 2 Write Protect 5"]
pub type ImaWp5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_WP6` reader - RAM Access Window 2 Write Protect 6"]
pub type ImaWp6R = crate::BitReader;
#[doc = "Field `IMA_WP6` writer - RAM Access Window 2 Write Protect 6"]
pub type ImaWp6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_WP7` reader - RAM Access Window 2 Write Protect 7"]
pub type ImaWp7R = crate::BitReader;
#[doc = "Field `IMA_WP7` writer - RAM Access Window 2 Write Protect 7"]
pub type ImaWp7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM Access Window 2 Write Protect 0"]
    #[inline(always)]
    pub fn ima_wp0(&self) -> ImaWp0R {
        ImaWp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Access Window 2 Write Protect 1"]
    #[inline(always)]
    pub fn ima_wp1(&self) -> ImaWp1R {
        ImaWp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Access Window 2 Write Protect 2"]
    #[inline(always)]
    pub fn ima_wp2(&self) -> ImaWp2R {
        ImaWp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM Access Window 2 Write Protect 3"]
    #[inline(always)]
    pub fn ima_wp3(&self) -> ImaWp3R {
        ImaWp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RAM Access Window 2 Write Protect 4"]
    #[inline(always)]
    pub fn ima_wp4(&self) -> ImaWp4R {
        ImaWp4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAM Access Window 2 Write Protect 5"]
    #[inline(always)]
    pub fn ima_wp5(&self) -> ImaWp5R {
        ImaWp5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RAM Access Window 2 Write Protect 6"]
    #[inline(always)]
    pub fn ima_wp6(&self) -> ImaWp6R {
        ImaWp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RAM Access Window 2 Write Protect 7"]
    #[inline(always)]
    pub fn ima_wp7(&self) -> ImaWp7R {
        ImaWp7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMA_WR_PROT")
            .field("ima_wp0", &self.ima_wp0())
            .field("ima_wp1", &self.ima_wp1())
            .field("ima_wp2", &self.ima_wp2())
            .field("ima_wp3", &self.ima_wp3())
            .field("ima_wp4", &self.ima_wp4())
            .field("ima_wp5", &self.ima_wp5())
            .field("ima_wp6", &self.ima_wp6())
            .field("ima_wp7", &self.ima_wp7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM Access Window 2 Write Protect 0"]
    #[inline(always)]
    pub fn ima_wp0(&mut self) -> ImaWp0W<ImaWrProtSpec> {
        ImaWp0W::new(self, 0)
    }
    #[doc = "Bit 1 - RAM Access Window 2 Write Protect 1"]
    #[inline(always)]
    pub fn ima_wp1(&mut self) -> ImaWp1W<ImaWrProtSpec> {
        ImaWp1W::new(self, 1)
    }
    #[doc = "Bit 2 - RAM Access Window 2 Write Protect 2"]
    #[inline(always)]
    pub fn ima_wp2(&mut self) -> ImaWp2W<ImaWrProtSpec> {
        ImaWp2W::new(self, 2)
    }
    #[doc = "Bit 3 - RAM Access Window 2 Write Protect 3"]
    #[inline(always)]
    pub fn ima_wp3(&mut self) -> ImaWp3W<ImaWrProtSpec> {
        ImaWp3W::new(self, 3)
    }
    #[doc = "Bit 4 - RAM Access Window 2 Write Protect 4"]
    #[inline(always)]
    pub fn ima_wp4(&mut self) -> ImaWp4W<ImaWrProtSpec> {
        ImaWp4W::new(self, 4)
    }
    #[doc = "Bit 5 - RAM Access Window 2 Write Protect 5"]
    #[inline(always)]
    pub fn ima_wp5(&mut self) -> ImaWp5W<ImaWrProtSpec> {
        ImaWp5W::new(self, 5)
    }
    #[doc = "Bit 6 - RAM Access Window 2 Write Protect 6"]
    #[inline(always)]
    pub fn ima_wp6(&mut self) -> ImaWp6W<ImaWrProtSpec> {
        ImaWp6W::new(self, 6)
    }
    #[doc = "Bit 7 - RAM Access Window 2 Write Protect 7"]
    #[inline(always)]
    pub fn ima_wp7(&mut self) -> ImaWp7W<ImaWrProtSpec> {
        ImaWp7W::new(self, 7)
    }
}
#[doc = "Indirect Memory Access Write Protect Register (IMA_WR_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`ima_wr_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ima_wr_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaWrProtSpec;
impl crate::RegisterSpec for ImaWrProtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ima_wr_prot::R`](R) reader structure"]
impl crate::Readable for ImaWrProtSpec {}
#[doc = "`write(|w| ..)` method takes [`ima_wr_prot::W`](W) writer structure"]
impl crate::Writable for ImaWrProtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IMA_WR_PROT to value 0"]
impl crate::Resettable for ImaWrProtSpec {
    const RESET_VALUE: u8 = 0;
}
