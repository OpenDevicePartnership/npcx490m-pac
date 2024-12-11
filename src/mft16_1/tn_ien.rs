#[doc = "Register `TnIEN` reader"]
pub type R = crate::R<TnIenSpec>;
#[doc = "Register `TnIEN` writer"]
pub type W = crate::W<TnIenSpec>;
#[doc = "Field `TAIEN` reader - Timer Interrupt A Enable"]
pub type TaienR = crate::BitReader;
#[doc = "Field `TAIEN` writer - Timer Interrupt A Enable"]
pub type TaienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBIEN` reader - Timer Interrupt B Enable"]
pub type TbienR = crate::BitReader;
#[doc = "Field `TBIEN` writer - Timer Interrupt B Enable"]
pub type TbienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIEN` reader - Timer Interrupt C Enable"]
pub type TcienR = crate::BitReader;
#[doc = "Field `TCIEN` writer - Timer Interrupt C Enable"]
pub type TcienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIEN` reader - Timer Interrupt D Enable"]
pub type TdienR = crate::BitReader;
#[doc = "Field `TDIEN` writer - Timer Interrupt D Enable"]
pub type TdienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIEN` reader - Timer Interrupt E Enable"]
pub type TeienR = crate::BitReader;
#[doc = "Field `TEIEN` writer - Timer Interrupt E Enable"]
pub type TeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIEN` reader - Timer Interrupt F Enable"]
pub type TfienR = crate::BitReader;
#[doc = "Field `TFIEN` writer - Timer Interrupt F Enable"]
pub type TfienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer Interrupt A Enable"]
    #[inline(always)]
    pub fn taien(&self) -> TaienR {
        TaienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Interrupt B Enable"]
    #[inline(always)]
    pub fn tbien(&self) -> TbienR {
        TbienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer Interrupt C Enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TcienR {
        TcienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer Interrupt D Enable"]
    #[inline(always)]
    pub fn tdien(&self) -> TdienR {
        TdienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer Interrupt E Enable"]
    #[inline(always)]
    pub fn teien(&self) -> TeienR {
        TeienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer Interrupt F Enable"]
    #[inline(always)]
    pub fn tfien(&self) -> TfienR {
        TfienR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TnIEN")
            .field("taien", &self.taien())
            .field("tbien", &self.tbien())
            .field("tcien", &self.tcien())
            .field("tdien", &self.tdien())
            .field("teien", &self.teien())
            .field("tfien", &self.tfien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer Interrupt A Enable"]
    #[inline(always)]
    pub fn taien(&mut self) -> TaienW<TnIenSpec> {
        TaienW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Interrupt B Enable"]
    #[inline(always)]
    pub fn tbien(&mut self) -> TbienW<TnIenSpec> {
        TbienW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer Interrupt C Enable"]
    #[inline(always)]
    pub fn tcien(&mut self) -> TcienW<TnIenSpec> {
        TcienW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer Interrupt D Enable"]
    #[inline(always)]
    pub fn tdien(&mut self) -> TdienW<TnIenSpec> {
        TdienW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer Interrupt E Enable"]
    #[inline(always)]
    pub fn teien(&mut self) -> TeienW<TnIenSpec> {
        TeienW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer Interrupt F Enable"]
    #[inline(always)]
    pub fn tfien(&mut self) -> TfienW<TnIenSpec> {
        TfienW::new(self, 5)
    }
}
#[doc = "Timer Interrupt Enable Register (TnIEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnIenSpec;
impl crate::RegisterSpec for TnIenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tn_ien::R`](R) reader structure"]
impl crate::Readable for TnIenSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_ien::W`](W) writer structure"]
impl crate::Writable for TnIenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TnIEN to value 0"]
impl crate::Resettable for TnIenSpec {
    const RESET_VALUE: u8 = 0;
}
