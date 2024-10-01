#[doc = "Register `IMA_RD_PROT` reader"]
pub type R = crate::R<ImaRdProtSpec>;
#[doc = "Register `IMA_RD_PROT` writer"]
pub type W = crate::W<ImaRdProtSpec>;
#[doc = "Field `IMA_RP0` reader - RAM Access Window 2 Read Protect 0"]
pub type ImaRp0R = crate::BitReader;
#[doc = "Field `IMA_RP0` writer - RAM Access Window 2 Read Protect 0"]
pub type ImaRp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_RP1` reader - RAM Access Window 2 Read Protect 1"]
pub type ImaRp1R = crate::BitReader;
#[doc = "Field `IMA_RP1` writer - RAM Access Window 2 Read Protect 1"]
pub type ImaRp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_RP2` reader - RAM Access Window 2 Read Protect 2"]
pub type ImaRp2R = crate::BitReader;
#[doc = "Field `IMA_RP2` writer - RAM Access Window 2 Read Protect 2"]
pub type ImaRp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_RP3` reader - RAM Access Window 2 Read Protect 3"]
pub type ImaRp3R = crate::BitReader;
#[doc = "Field `IMA_RP3` writer - RAM Access Window 2 Read Protect 3"]
pub type ImaRp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_RP4` reader - RAM Access Window 2 Read Protect 4"]
pub type ImaRp4R = crate::BitReader;
#[doc = "Field `IMA_RP4` writer - RAM Access Window 2 Read Protect 4"]
pub type ImaRp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_RP5` reader - RAM Access Window 2 Read Protect 5"]
pub type ImaRp5R = crate::BitReader;
#[doc = "Field `IMA_RP5` writer - RAM Access Window 2 Read Protect 5"]
pub type ImaRp5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_RP6` reader - RAM Access Window 2 Read Protect 6"]
pub type ImaRp6R = crate::BitReader;
#[doc = "Field `IMA_RP6` writer - RAM Access Window 2 Read Protect 6"]
pub type ImaRp6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMA_RP7` reader - RAM Access Window 2 Read Protect 7"]
pub type ImaRp7R = crate::BitReader;
#[doc = "Field `IMA_RP7` writer - RAM Access Window 2 Read Protect 7"]
pub type ImaRp7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM Access Window 2 Read Protect 0"]
    #[inline(always)]
    pub fn ima_rp0(&self) -> ImaRp0R {
        ImaRp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Access Window 2 Read Protect 1"]
    #[inline(always)]
    pub fn ima_rp1(&self) -> ImaRp1R {
        ImaRp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Access Window 2 Read Protect 2"]
    #[inline(always)]
    pub fn ima_rp2(&self) -> ImaRp2R {
        ImaRp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM Access Window 2 Read Protect 3"]
    #[inline(always)]
    pub fn ima_rp3(&self) -> ImaRp3R {
        ImaRp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RAM Access Window 2 Read Protect 4"]
    #[inline(always)]
    pub fn ima_rp4(&self) -> ImaRp4R {
        ImaRp4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAM Access Window 2 Read Protect 5"]
    #[inline(always)]
    pub fn ima_rp5(&self) -> ImaRp5R {
        ImaRp5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RAM Access Window 2 Read Protect 6"]
    #[inline(always)]
    pub fn ima_rp6(&self) -> ImaRp6R {
        ImaRp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RAM Access Window 2 Read Protect 7"]
    #[inline(always)]
    pub fn ima_rp7(&self) -> ImaRp7R {
        ImaRp7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMA_RD_PROT")
            .field("ima_rp0", &self.ima_rp0())
            .field("ima_rp1", &self.ima_rp1())
            .field("ima_rp2", &self.ima_rp2())
            .field("ima_rp3", &self.ima_rp3())
            .field("ima_rp4", &self.ima_rp4())
            .field("ima_rp5", &self.ima_rp5())
            .field("ima_rp6", &self.ima_rp6())
            .field("ima_rp7", &self.ima_rp7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM Access Window 2 Read Protect 0"]
    #[inline(always)]
    #[must_use]
    pub fn ima_rp0(&mut self) -> ImaRp0W<ImaRdProtSpec> {
        ImaRp0W::new(self, 0)
    }
    #[doc = "Bit 1 - RAM Access Window 2 Read Protect 1"]
    #[inline(always)]
    #[must_use]
    pub fn ima_rp1(&mut self) -> ImaRp1W<ImaRdProtSpec> {
        ImaRp1W::new(self, 1)
    }
    #[doc = "Bit 2 - RAM Access Window 2 Read Protect 2"]
    #[inline(always)]
    #[must_use]
    pub fn ima_rp2(&mut self) -> ImaRp2W<ImaRdProtSpec> {
        ImaRp2W::new(self, 2)
    }
    #[doc = "Bit 3 - RAM Access Window 2 Read Protect 3"]
    #[inline(always)]
    #[must_use]
    pub fn ima_rp3(&mut self) -> ImaRp3W<ImaRdProtSpec> {
        ImaRp3W::new(self, 3)
    }
    #[doc = "Bit 4 - RAM Access Window 2 Read Protect 4"]
    #[inline(always)]
    #[must_use]
    pub fn ima_rp4(&mut self) -> ImaRp4W<ImaRdProtSpec> {
        ImaRp4W::new(self, 4)
    }
    #[doc = "Bit 5 - RAM Access Window 2 Read Protect 5"]
    #[inline(always)]
    #[must_use]
    pub fn ima_rp5(&mut self) -> ImaRp5W<ImaRdProtSpec> {
        ImaRp5W::new(self, 5)
    }
    #[doc = "Bit 6 - RAM Access Window 2 Read Protect 6"]
    #[inline(always)]
    #[must_use]
    pub fn ima_rp6(&mut self) -> ImaRp6W<ImaRdProtSpec> {
        ImaRp6W::new(self, 6)
    }
    #[doc = "Bit 7 - RAM Access Window 2 Read Protect 7"]
    #[inline(always)]
    #[must_use]
    pub fn ima_rp7(&mut self) -> ImaRp7W<ImaRdProtSpec> {
        ImaRp7W::new(self, 7)
    }
}
#[doc = "Indirect Memory Access Read Protect Register (IMA_RD_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`ima_rd_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ima_rd_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaRdProtSpec;
impl crate::RegisterSpec for ImaRdProtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ima_rd_prot::R`](R) reader structure"]
impl crate::Readable for ImaRdProtSpec {}
#[doc = "`write(|w| ..)` method takes [`ima_rd_prot::W`](W) writer structure"]
impl crate::Writable for ImaRdProtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IMA_RD_PROT to value 0"]
impl crate::Resettable for ImaRdProtSpec {
    const RESET_VALUE: u8 = 0;
}
