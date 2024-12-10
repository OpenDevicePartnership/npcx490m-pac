#[doc = "Register `FLM_CTL` reader"]
pub type R = crate::R<FlmCtlSpec>;
#[doc = "Register `FLM_CTL` writer"]
pub type W = crate::W<FlmCtlSpec>;
#[doc = "Field `RDY` reader - Ready"]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - Ready"]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANGE` reader - Change and Update Configuration Parameters"]
pub type ChangeR = crate::BitReader;
#[doc = "Field `CHANGE` writer - Change and Update Configuration Parameters"]
pub type ChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLCK` reader - Reversible Lock"]
pub type RlckR = crate::FieldReader;
#[doc = "Field `RLCK` writer - Reversible Lock"]
pub type RlckW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEN` reader - Module Enable"]
pub type MenR = crate::BitReader;
#[doc = "Field `MEN` writer - Module Enable"]
pub type MenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK` reader - Lock"]
pub type LckR = crate::BitReader;
#[doc = "Field `LCK` writer - Lock"]
pub type LckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ready"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Change and Update Configuration Parameters"]
    #[inline(always)]
    pub fn change(&self) -> ChangeR {
        ChangeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reversible Lock"]
    #[inline(always)]
    pub fn rlck(&self) -> RlckR {
        RlckR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Module Enable"]
    #[inline(always)]
    pub fn men(&self) -> MenR {
        MenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    pub fn lck(&self) -> LckR {
        LckR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_CTL")
            .field("rdy", &self.rdy())
            .field("change", &self.change())
            .field("rlck", &self.rlck())
            .field("men", &self.men())
            .field("lck", &self.lck())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Ready"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<FlmCtlSpec> {
        RdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Change and Update Configuration Parameters"]
    #[inline(always)]
    pub fn change(&mut self) -> ChangeW<FlmCtlSpec> {
        ChangeW::new(self, 1)
    }
    #[doc = "Bits 8:15 - Reversible Lock"]
    #[inline(always)]
    pub fn rlck(&mut self) -> RlckW<FlmCtlSpec> {
        RlckW::new(self, 8)
    }
    #[doc = "Bit 30 - Module Enable"]
    #[inline(always)]
    pub fn men(&mut self) -> MenW<FlmCtlSpec> {
        MenW::new(self, 30)
    }
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    pub fn lck(&mut self) -> LckW<FlmCtlSpec> {
        LckW::new(self, 31)
    }
}
#[doc = "FLM Control Register (FLM_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCtlSpec;
impl crate::RegisterSpec for FlmCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_ctl::R`](R) reader structure"]
impl crate::Readable for FlmCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`flm_ctl::W`](W) writer structure"]
impl crate::Writable for FlmCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CTL to value 0"]
impl crate::Resettable for FlmCtlSpec {
    const RESET_VALUE: u32 = 0;
}
