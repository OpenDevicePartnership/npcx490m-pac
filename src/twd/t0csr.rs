#[doc = "Register `T0CSR` reader"]
pub type R = crate::R<T0csrSpec>;
#[doc = "Register `T0CSR` writer"]
pub type W = crate::W<T0csrSpec>;
#[doc = "Field `RST` reader - Reset"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Reset"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Terminal Count"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - Terminal Count"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDLTD` reader - Watchdog Last Touch Delay"]
pub type WdltdR = crate::BitReader;
#[doc = "Field `WDLTD` writer - Watchdog Last Touch Delay"]
pub type WdltdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD_RUN` reader - Watchdog Run Status"]
pub type WdRunR = crate::BitReader;
#[doc = "Field `WD_RUN` writer - Watchdog Run Status"]
pub type WdRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESDIS` reader - Too Early Service Disable"]
pub type TesdisR = crate::BitReader;
#[doc = "Field `TESDIS` writer - Too Early Service Disable"]
pub type TesdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Terminal Count"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Last Touch Delay"]
    #[inline(always)]
    pub fn wdltd(&self) -> WdltdR {
        WdltdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Watchdog Run Status"]
    #[inline(always)]
    pub fn wd_run(&self) -> WdRunR {
        WdRunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Too Early Service Disable"]
    #[inline(always)]
    pub fn tesdis(&self) -> TesdisR {
        TesdisR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0CSR")
            .field("rst", &self.rst())
            .field("tc", &self.tc())
            .field("wdltd", &self.wdltd())
            .field("wd_run", &self.wd_run())
            .field("tesdis", &self.tesdis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<T0csrSpec> {
        RstW::new(self, 0)
    }
    #[doc = "Bit 1 - Terminal Count"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<T0csrSpec> {
        TcW::new(self, 1)
    }
    #[doc = "Bit 3 - Watchdog Last Touch Delay"]
    #[inline(always)]
    pub fn wdltd(&mut self) -> WdltdW<T0csrSpec> {
        WdltdW::new(self, 3)
    }
    #[doc = "Bit 5 - Watchdog Run Status"]
    #[inline(always)]
    pub fn wd_run(&mut self) -> WdRunW<T0csrSpec> {
        WdRunW::new(self, 5)
    }
    #[doc = "Bit 7 - Too Early Service Disable"]
    #[inline(always)]
    pub fn tesdis(&mut self) -> TesdisW<T0csrSpec> {
        TesdisW::new(self, 7)
    }
}
#[doc = "TWDT0 Control and Status Register (T0CSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`t0csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0csrSpec;
impl crate::RegisterSpec for T0csrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`t0csr::R`](R) reader structure"]
impl crate::Readable for T0csrSpec {}
#[doc = "`write(|w| ..)` method takes [`t0csr::W`](W) writer structure"]
impl crate::Writable for T0csrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets T0CSR to value 0"]
impl crate::Resettable for T0csrSpec {
    const RESET_VALUE: u8 = 0;
}
