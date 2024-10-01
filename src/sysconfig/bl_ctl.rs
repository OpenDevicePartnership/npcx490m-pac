#[doc = "Register `BL_CTL` reader"]
pub type R = crate::R<BlCtlSpec>;
#[doc = "Register `BL_CTL` writer"]
pub type W = crate::W<BlCtlSpec>;
#[doc = "Field `WDG_RST_IND` reader - Watchdog Reset Indication"]
pub type WdgRstIndR = crate::BitReader;
#[doc = "Field `WDG_RST_IND` writer - Watchdog Reset Indication"]
pub type WdgRstIndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RST_IND` reader - Debugger Reset Indication"]
pub type DbgRstIndR = crate::BitReader;
#[doc = "Field `DBG_RST_IND` writer - Debugger Reset Indication"]
pub type DbgRstIndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL_SRORD` reader - BootLoader Search Order"]
pub type BlSrordR = crate::FieldReader;
#[doc = "Field `BL_SRORD` writer - BootLoader Search Order"]
pub type BlSrordW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Watchdog Reset Indication"]
    #[inline(always)]
    pub fn wdg_rst_ind(&self) -> WdgRstIndR {
        WdgRstIndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debugger Reset Indication"]
    #[inline(always)]
    pub fn dbg_rst_ind(&self) -> DbgRstIndR {
        DbgRstIndR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - BootLoader Search Order"]
    #[inline(always)]
    pub fn bl_srord(&self) -> BlSrordR {
        BlSrordR::new((self.bits >> 3) & 3)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BL_CTL")
            .field("wdg_rst_ind", &self.wdg_rst_ind())
            .field("dbg_rst_ind", &self.dbg_rst_ind())
            .field("bl_srord", &self.bl_srord())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Watchdog Reset Indication"]
    #[inline(always)]
    #[must_use]
    pub fn wdg_rst_ind(&mut self) -> WdgRstIndW<BlCtlSpec> {
        WdgRstIndW::new(self, 1)
    }
    #[doc = "Bit 2 - Debugger Reset Indication"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rst_ind(&mut self) -> DbgRstIndW<BlCtlSpec> {
        DbgRstIndW::new(self, 2)
    }
    #[doc = "Bits 3:4 - BootLoader Search Order"]
    #[inline(always)]
    #[must_use]
    pub fn bl_srord(&mut self) -> BlSrordW<BlCtlSpec> {
        BlSrordW::new(self, 3)
    }
}
#[doc = "BootLoader Control Register (BL_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`bl_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bl_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlCtlSpec;
impl crate::RegisterSpec for BlCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bl_ctl::R`](R) reader structure"]
impl crate::Readable for BlCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`bl_ctl::W`](W) writer structure"]
impl crate::Writable for BlCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BL_CTL to value 0"]
impl crate::Resettable for BlCtlSpec {
    const RESET_VALUE: u8 = 0;
}
