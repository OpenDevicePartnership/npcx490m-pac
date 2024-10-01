#[doc = "Register `TnECLR` reader"]
pub type R = crate::R<TnEclrSpec>;
#[doc = "Register `TnECLR` writer"]
pub type W = crate::W<TnEclrSpec>;
#[doc = "Field `TACLR` reader - Timer Pending A Clear"]
pub type TaclrR = crate::BitReader;
#[doc = "Field `TACLR` writer - Timer Pending A Clear"]
pub type TaclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCLR` reader - Timer Pending B Clear"]
pub type TbclrR = crate::BitReader;
#[doc = "Field `TBCLR` writer - Timer Pending B Clear"]
pub type TbclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCLR` reader - Timer Pending C Clear"]
pub type TcclrR = crate::BitReader;
#[doc = "Field `TCCLR` writer - Timer Pending C Clear"]
pub type TcclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCLR` reader - Timer Pending D Clear"]
pub type TdclrR = crate::BitReader;
#[doc = "Field `TDCLR` writer - Timer Pending D Clear"]
pub type TdclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TECLR` reader - Timer Pending E Clear"]
pub type TeclrR = crate::BitReader;
#[doc = "Field `TECLR` writer - Timer Pending E Clear"]
pub type TeclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFCLR` reader - Timer Pending F Clear"]
pub type TfclrR = crate::BitReader;
#[doc = "Field `TFCLR` writer - Timer Pending F Clear"]
pub type TfclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer Pending A Clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TaclrR {
        TaclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Pending B Clear"]
    #[inline(always)]
    pub fn tbclr(&self) -> TbclrR {
        TbclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer Pending C Clear"]
    #[inline(always)]
    pub fn tcclr(&self) -> TcclrR {
        TcclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer Pending D Clear"]
    #[inline(always)]
    pub fn tdclr(&self) -> TdclrR {
        TdclrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer Pending E Clear"]
    #[inline(always)]
    pub fn teclr(&self) -> TeclrR {
        TeclrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer Pending F Clear"]
    #[inline(always)]
    pub fn tfclr(&self) -> TfclrR {
        TfclrR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TnECLR")
            .field("taclr", &self.taclr())
            .field("tbclr", &self.tbclr())
            .field("tcclr", &self.tcclr())
            .field("tdclr", &self.tdclr())
            .field("teclr", &self.teclr())
            .field("tfclr", &self.tfclr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer Pending A Clear"]
    #[inline(always)]
    #[must_use]
    pub fn taclr(&mut self) -> TaclrW<TnEclrSpec> {
        TaclrW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Pending B Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tbclr(&mut self) -> TbclrW<TnEclrSpec> {
        TbclrW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer Pending C Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcclr(&mut self) -> TcclrW<TnEclrSpec> {
        TcclrW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer Pending D Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tdclr(&mut self) -> TdclrW<TnEclrSpec> {
        TdclrW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer Pending E Clear"]
    #[inline(always)]
    #[must_use]
    pub fn teclr(&mut self) -> TeclrW<TnEclrSpec> {
        TeclrW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer Pending F Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tfclr(&mut self) -> TfclrW<TnEclrSpec> {
        TfclrW::new(self, 5)
    }
}
#[doc = "Timer Event Clear Register (TnECLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_eclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_eclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnEclrSpec;
impl crate::RegisterSpec for TnEclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tn_eclr::R`](R) reader structure"]
impl crate::Readable for TnEclrSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_eclr::W`](W) writer structure"]
impl crate::Writable for TnEclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TnECLR to value 0"]
impl crate::Resettable for TnEclrSpec {
    const RESET_VALUE: u8 = 0;
}
