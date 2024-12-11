#[doc = "Register `HIKMST` reader"]
pub type R = crate::R<HikmstSpec>;
#[doc = "Register `HIKMST` writer"]
pub type W = crate::W<HikmstSpec>;
#[doc = "Field `OBF` reader - Output Buffer Full"]
pub type ObfR = crate::BitReader;
#[doc = "Field `OBF` writer - Output Buffer Full"]
pub type ObfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBF` reader - Input Buffer Full"]
pub type IbfR = crate::BitReader;
#[doc = "Field `IBF` writer - Input Buffer Full"]
pub type IbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0` reader - Flag 0"]
pub type F0R = crate::BitReader;
#[doc = "Field `F0` writer - Flag 0"]
pub type F0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A2` reader - A2 Address"]
pub type A2R = crate::BitReader;
#[doc = "Field `A2` writer - A2 Address"]
pub type A2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST3_ST0` reader - Status Bits"]
pub type St3St0R = crate::FieldReader;
#[doc = "Field `ST3_ST0` writer - Status Bits"]
pub type St3St0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Output Buffer Full"]
    #[inline(always)]
    pub fn obf(&self) -> ObfR {
        ObfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Buffer Full"]
    #[inline(always)]
    pub fn ibf(&self) -> IbfR {
        IbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flag 0"]
    #[inline(always)]
    pub fn f0(&self) -> F0R {
        F0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A2 Address"]
    #[inline(always)]
    pub fn a2(&self) -> A2R {
        A2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Status Bits"]
    #[inline(always)]
    pub fn st3_st0(&self) -> St3St0R {
        St3St0R::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIKMST")
            .field("obf", &self.obf())
            .field("ibf", &self.ibf())
            .field("f0", &self.f0())
            .field("a2", &self.a2())
            .field("st3_st0", &self.st3_st0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Output Buffer Full"]
    #[inline(always)]
    pub fn obf(&mut self) -> ObfW<HikmstSpec> {
        ObfW::new(self, 0)
    }
    #[doc = "Bit 1 - Input Buffer Full"]
    #[inline(always)]
    pub fn ibf(&mut self) -> IbfW<HikmstSpec> {
        IbfW::new(self, 1)
    }
    #[doc = "Bit 2 - Flag 0"]
    #[inline(always)]
    pub fn f0(&mut self) -> F0W<HikmstSpec> {
        F0W::new(self, 2)
    }
    #[doc = "Bit 3 - A2 Address"]
    #[inline(always)]
    pub fn a2(&mut self) -> A2W<HikmstSpec> {
        A2W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Status Bits"]
    #[inline(always)]
    pub fn st3_st0(&mut self) -> St3St0W<HikmstSpec> {
        St3St0W::new(self, 4)
    }
}
#[doc = "Host Interface Keyboard/Mouse Status Register (HIKMST)\n\nYou can [`read`](crate::Reg::read) this register and get [`hikmst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hikmst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HikmstSpec;
impl crate::RegisterSpec for HikmstSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hikmst::R`](R) reader structure"]
impl crate::Readable for HikmstSpec {}
#[doc = "`write(|w| ..)` method takes [`hikmst::W`](W) writer structure"]
impl crate::Writable for HikmstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIKMST to value 0"]
impl crate::Resettable for HikmstSpec {
    const RESET_VALUE: u8 = 0;
}
