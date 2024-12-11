#[doc = "Register `TnECTRL` reader"]
pub type R = crate::R<TnEctrlSpec>;
#[doc = "Register `TnECTRL` writer"]
pub type W = crate::W<TnEctrlSpec>;
#[doc = "Field `TAPND` reader - Timer Event Source A Pending"]
pub type TapndR = crate::BitReader;
#[doc = "Field `TAPND` writer - Timer Event Source A Pending"]
pub type TapndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBPND` reader - Timer Event Source B Pending"]
pub type TbpndR = crate::BitReader;
#[doc = "Field `TBPND` writer - Timer Event Source B Pending"]
pub type TbpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCPND` reader - Timer Event Source C Pending"]
pub type TcpndR = crate::BitReader;
#[doc = "Field `TCPND` writer - Timer Event Source C Pending"]
pub type TcpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDPND` reader - Timer Event Source D Pending"]
pub type TdpndR = crate::BitReader;
#[doc = "Field `TDPND` writer - Timer Event Source D Pending"]
pub type TdpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEPND` reader - Timer Event Source E Pending"]
pub type TepndR = crate::BitReader;
#[doc = "Field `TEPND` writer - Timer Event Source E Pending"]
pub type TepndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFPND` reader - Timer Event Source F Pending"]
pub type TfpndR = crate::BitReader;
#[doc = "Field `TFPND` writer - Timer Event Source F Pending"]
pub type TfpndW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer Event Source A Pending"]
    #[inline(always)]
    pub fn tapnd(&self) -> TapndR {
        TapndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Event Source B Pending"]
    #[inline(always)]
    pub fn tbpnd(&self) -> TbpndR {
        TbpndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer Event Source C Pending"]
    #[inline(always)]
    pub fn tcpnd(&self) -> TcpndR {
        TcpndR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer Event Source D Pending"]
    #[inline(always)]
    pub fn tdpnd(&self) -> TdpndR {
        TdpndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer Event Source E Pending"]
    #[inline(always)]
    pub fn tepnd(&self) -> TepndR {
        TepndR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer Event Source F Pending"]
    #[inline(always)]
    pub fn tfpnd(&self) -> TfpndR {
        TfpndR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TnECTRL")
            .field("tapnd", &self.tapnd())
            .field("tbpnd", &self.tbpnd())
            .field("tcpnd", &self.tcpnd())
            .field("tdpnd", &self.tdpnd())
            .field("tepnd", &self.tepnd())
            .field("tfpnd", &self.tfpnd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer Event Source A Pending"]
    #[inline(always)]
    pub fn tapnd(&mut self) -> TapndW<TnEctrlSpec> {
        TapndW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Event Source B Pending"]
    #[inline(always)]
    pub fn tbpnd(&mut self) -> TbpndW<TnEctrlSpec> {
        TbpndW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer Event Source C Pending"]
    #[inline(always)]
    pub fn tcpnd(&mut self) -> TcpndW<TnEctrlSpec> {
        TcpndW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer Event Source D Pending"]
    #[inline(always)]
    pub fn tdpnd(&mut self) -> TdpndW<TnEctrlSpec> {
        TdpndW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer Event Source E Pending"]
    #[inline(always)]
    pub fn tepnd(&mut self) -> TepndW<TnEctrlSpec> {
        TepndW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer Event Source F Pending"]
    #[inline(always)]
    pub fn tfpnd(&mut self) -> TfpndW<TnEctrlSpec> {
        TfpndW::new(self, 5)
    }
}
#[doc = "Timer Event Control Register (TnECTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_ectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_ectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnEctrlSpec;
impl crate::RegisterSpec for TnEctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tn_ectrl::R`](R) reader structure"]
impl crate::Readable for TnEctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_ectrl::W`](W) writer structure"]
impl crate::Writable for TnEctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TnECTRL to value 0"]
impl crate::Resettable for TnEctrlSpec {
    const RESET_VALUE: u8 = 0;
}
