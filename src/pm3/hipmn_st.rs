#[doc = "Register `HIPMnST` reader"]
pub type R = crate::R<HipmnStSpec>;
#[doc = "Register `HIPMnST` writer"]
pub type W = crate::W<HipmnStSpec>;
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
#[doc = "Field `CMD` reader - Command Register"]
pub type CmdR = crate::BitReader;
#[doc = "Field `CMD` writer - Command Register"]
pub type CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST3_ST0` reader - Status"]
pub type St3St0R = crate::FieldReader;
#[doc = "Field `ST3_ST0` writer - Status"]
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
    #[doc = "Bit 3 - Command Register"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Status"]
    #[inline(always)]
    pub fn st3_st0(&self) -> St3St0R {
        St3St0R::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIPMnST")
            .field("obf", &self.obf())
            .field("ibf", &self.ibf())
            .field("f0", &self.f0())
            .field("cmd", &self.cmd())
            .field("st3_st0", &self.st3_st0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Output Buffer Full"]
    #[inline(always)]
    #[must_use]
    pub fn obf(&mut self) -> ObfW<HipmnStSpec> {
        ObfW::new(self, 0)
    }
    #[doc = "Bit 1 - Input Buffer Full"]
    #[inline(always)]
    #[must_use]
    pub fn ibf(&mut self) -> IbfW<HipmnStSpec> {
        IbfW::new(self, 1)
    }
    #[doc = "Bit 2 - Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn f0(&mut self) -> F0W<HipmnStSpec> {
        F0W::new(self, 2)
    }
    #[doc = "Bit 3 - Command Register"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<HipmnStSpec> {
        CmdW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Status"]
    #[inline(always)]
    #[must_use]
    pub fn st3_st0(&mut self) -> St3St0W<HipmnStSpec> {
        St3St0W::new(self, 4)
    }
}
#[doc = "Host Interface PM n Status Register (HIPMnST)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HipmnStSpec;
impl crate::RegisterSpec for HipmnStSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hipmn_st::R`](R) reader structure"]
impl crate::Readable for HipmnStSpec {}
#[doc = "`write(|w| ..)` method takes [`hipmn_st::W`](W) writer structure"]
impl crate::Writable for HipmnStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIPMnST to value 0"]
impl crate::Resettable for HipmnStSpec {
    const RESET_VALUE: u8 = 0;
}
