#[doc = "Register `WIN3_RD_PROT` reader"]
pub type R = crate::R<Win3RdProtSpec>;
#[doc = "Register `WIN3_RD_PROT` writer"]
pub type W = crate::W<Win3RdProtSpec>;
#[doc = "Field `RW3_RP0` reader - RAM Access Window 3 Read Protect 0"]
pub type Rw3Rp0R = crate::BitReader;
#[doc = "Field `RW3_RP0` writer - RAM Access Window 3 Read Protect 0"]
pub type Rw3Rp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW3_RP1` reader - RAM Access Window 3 Read Protect 1"]
pub type Rw3Rp1R = crate::BitReader;
#[doc = "Field `RW3_RP1` writer - RAM Access Window 3 Read Protect 1"]
pub type Rw3Rp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW3_RP2` reader - RAM Access Window 3 Read Protect 2"]
pub type Rw3Rp2R = crate::BitReader;
#[doc = "Field `RW3_RP2` writer - RAM Access Window 3 Read Protect 2"]
pub type Rw3Rp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW3_RP3` reader - RAM Access Window 3 Read Protect 3"]
pub type Rw3Rp3R = crate::BitReader;
#[doc = "Field `RW3_RP3` writer - RAM Access Window 3 Read Protect 3"]
pub type Rw3Rp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW3_RP4` reader - RAM Access Window 3 Read Protect 4"]
pub type Rw3Rp4R = crate::BitReader;
#[doc = "Field `RW3_RP4` writer - RAM Access Window 3 Read Protect 4"]
pub type Rw3Rp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW3_RP5` reader - RAM Access Window 3 Read Protect 5"]
pub type Rw3Rp5R = crate::BitReader;
#[doc = "Field `RW3_RP5` writer - RAM Access Window 3 Read Protect 5"]
pub type Rw3Rp5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW3_RP6` reader - RAM Access Window 3 Read Protect 6"]
pub type Rw3Rp6R = crate::BitReader;
#[doc = "Field `RW3_RP6` writer - RAM Access Window 3 Read Protect 6"]
pub type Rw3Rp6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW3_RP7` reader - RAM Access Window 3 Read Protect 7"]
pub type Rw3Rp7R = crate::BitReader;
#[doc = "Field `RW3_RP7` writer - RAM Access Window 3 Read Protect 7"]
pub type Rw3Rp7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM Access Window 3 Read Protect 0"]
    #[inline(always)]
    pub fn rw3_rp0(&self) -> Rw3Rp0R {
        Rw3Rp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Access Window 3 Read Protect 1"]
    #[inline(always)]
    pub fn rw3_rp1(&self) -> Rw3Rp1R {
        Rw3Rp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Access Window 3 Read Protect 2"]
    #[inline(always)]
    pub fn rw3_rp2(&self) -> Rw3Rp2R {
        Rw3Rp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM Access Window 3 Read Protect 3"]
    #[inline(always)]
    pub fn rw3_rp3(&self) -> Rw3Rp3R {
        Rw3Rp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RAM Access Window 3 Read Protect 4"]
    #[inline(always)]
    pub fn rw3_rp4(&self) -> Rw3Rp4R {
        Rw3Rp4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAM Access Window 3 Read Protect 5"]
    #[inline(always)]
    pub fn rw3_rp5(&self) -> Rw3Rp5R {
        Rw3Rp5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RAM Access Window 3 Read Protect 6"]
    #[inline(always)]
    pub fn rw3_rp6(&self) -> Rw3Rp6R {
        Rw3Rp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RAM Access Window 3 Read Protect 7"]
    #[inline(always)]
    pub fn rw3_rp7(&self) -> Rw3Rp7R {
        Rw3Rp7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIN3_RD_PROT")
            .field("rw3_rp0", &self.rw3_rp0())
            .field("rw3_rp1", &self.rw3_rp1())
            .field("rw3_rp2", &self.rw3_rp2())
            .field("rw3_rp3", &self.rw3_rp3())
            .field("rw3_rp4", &self.rw3_rp4())
            .field("rw3_rp5", &self.rw3_rp5())
            .field("rw3_rp6", &self.rw3_rp6())
            .field("rw3_rp7", &self.rw3_rp7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM Access Window 3 Read Protect 0"]
    #[inline(always)]
    pub fn rw3_rp0(&mut self) -> Rw3Rp0W<Win3RdProtSpec> {
        Rw3Rp0W::new(self, 0)
    }
    #[doc = "Bit 1 - RAM Access Window 3 Read Protect 1"]
    #[inline(always)]
    pub fn rw3_rp1(&mut self) -> Rw3Rp1W<Win3RdProtSpec> {
        Rw3Rp1W::new(self, 1)
    }
    #[doc = "Bit 2 - RAM Access Window 3 Read Protect 2"]
    #[inline(always)]
    pub fn rw3_rp2(&mut self) -> Rw3Rp2W<Win3RdProtSpec> {
        Rw3Rp2W::new(self, 2)
    }
    #[doc = "Bit 3 - RAM Access Window 3 Read Protect 3"]
    #[inline(always)]
    pub fn rw3_rp3(&mut self) -> Rw3Rp3W<Win3RdProtSpec> {
        Rw3Rp3W::new(self, 3)
    }
    #[doc = "Bit 4 - RAM Access Window 3 Read Protect 4"]
    #[inline(always)]
    pub fn rw3_rp4(&mut self) -> Rw3Rp4W<Win3RdProtSpec> {
        Rw3Rp4W::new(self, 4)
    }
    #[doc = "Bit 5 - RAM Access Window 3 Read Protect 5"]
    #[inline(always)]
    pub fn rw3_rp5(&mut self) -> Rw3Rp5W<Win3RdProtSpec> {
        Rw3Rp5W::new(self, 5)
    }
    #[doc = "Bit 6 - RAM Access Window 3 Read Protect 6"]
    #[inline(always)]
    pub fn rw3_rp6(&mut self) -> Rw3Rp6W<Win3RdProtSpec> {
        Rw3Rp6W::new(self, 6)
    }
    #[doc = "Bit 7 - RAM Access Window 3 Read Protect 7"]
    #[inline(always)]
    pub fn rw3_rp7(&mut self) -> Rw3Rp7W<Win3RdProtSpec> {
        Rw3Rp7W::new(self, 7)
    }
}
#[doc = "Shared Access Window 3 Read Protect Register (WIN3_RD_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win3_rd_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win3_rd_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3RdProtSpec;
impl crate::RegisterSpec for Win3RdProtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`win3_rd_prot::R`](R) reader structure"]
impl crate::Readable for Win3RdProtSpec {}
#[doc = "`write(|w| ..)` method takes [`win3_rd_prot::W`](W) writer structure"]
impl crate::Writable for Win3RdProtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WIN3_RD_PROT to value 0"]
impl crate::Resettable for Win3RdProtSpec {
    const RESET_VALUE: u8 = 0;
}
