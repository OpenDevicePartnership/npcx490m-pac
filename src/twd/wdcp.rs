#[doc = "Register `WDCP` reader"]
pub type R = crate::R<WdcpSpec>;
#[doc = "Register `WDCP` writer"]
pub type W = crate::W<WdcpSpec>;
#[doc = "Field `WDIV` reader - default"]
pub type WdivR = crate::FieldReader;
#[doc = "Field `WDIV` writer - default"]
pub type WdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - default"]
    #[inline(always)]
    pub fn wdiv(&self) -> WdivR {
        WdivR::new(self.bits & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDCP").field("wdiv", &self.wdiv()).finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - default"]
    #[inline(always)]
    pub fn wdiv(&mut self) -> WdivW<WdcpSpec> {
        WdivW::new(self, 0)
    }
}
#[doc = "Watchdog Clock Prescaler Register (WDCP)\n\nYou can [`read`](crate::Reg::read) this register and get [`wdcp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdcp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdcpSpec;
impl crate::RegisterSpec for WdcpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdcp::R`](R) reader structure"]
impl crate::Readable for WdcpSpec {}
#[doc = "`write(|w| ..)` method takes [`wdcp::W`](W) writer structure"]
impl crate::Writable for WdcpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WDCP to value 0"]
impl crate::Resettable for WdcpSpec {
    const RESET_VALUE: u8 = 0;
}
