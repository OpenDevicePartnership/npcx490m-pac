#[doc = "Register `PSCON` reader"]
pub type R = crate::R<PsconSpec>;
#[doc = "Register `PSCON` writer"]
pub type W = crate::W<PsconSpec>;
#[doc = "Field `EN` reader - Shift Mechanism Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Shift Mechanism Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMT` reader - Transmit Enable"]
pub type XmtR = crate::BitReader;
#[doc = "Field `XMT` writer - Transmit Enable"]
pub type XmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV` reader - High Drive"]
pub type HdrvR = crate::FieldReader;
#[doc = "Field `HDRV` writer - High Drive"]
pub type HdrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDB` reader - Input Debounce"]
pub type IdbR = crate::FieldReader;
#[doc = "Field `IDB` writer - Input Debounce"]
pub type IdbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WPUEN` reader - Weak Pull-Up Enable"]
pub type WpuenR = crate::BitReader;
#[doc = "Field `WPUEN` writer - Weak Pull-Up Enable"]
pub type WpuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shift Mechanism Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Enable"]
    #[inline(always)]
    pub fn xmt(&self) -> XmtR {
        XmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - High Drive"]
    #[inline(always)]
    pub fn hdrv(&self) -> HdrvR {
        HdrvR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:6 - Input Debounce"]
    #[inline(always)]
    pub fn idb(&self) -> IdbR {
        IdbR::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Weak Pull-Up Enable"]
    #[inline(always)]
    pub fn wpuen(&self) -> WpuenR {
        WpuenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSCON")
            .field("en", &self.en())
            .field("xmt", &self.xmt())
            .field("hdrv", &self.hdrv())
            .field("idb", &self.idb())
            .field("wpuen", &self.wpuen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Shift Mechanism Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<PsconSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Enable"]
    #[inline(always)]
    pub fn xmt(&mut self) -> XmtW<PsconSpec> {
        XmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - High Drive"]
    #[inline(always)]
    pub fn hdrv(&mut self) -> HdrvW<PsconSpec> {
        HdrvW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Input Debounce"]
    #[inline(always)]
    pub fn idb(&mut self) -> IdbW<PsconSpec> {
        IdbW::new(self, 4)
    }
    #[doc = "Bit 7 - Weak Pull-Up Enable"]
    #[inline(always)]
    pub fn wpuen(&mut self) -> WpuenW<PsconSpec> {
        WpuenW::new(self, 7)
    }
}
#[doc = "PS/2 Control Register (PSCON)\n\nYou can [`read`](crate::Reg::read) this register and get [`pscon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsconSpec;
impl crate::RegisterSpec for PsconSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pscon::R`](R) reader structure"]
impl crate::Readable for PsconSpec {}
#[doc = "`write(|w| ..)` method takes [`pscon::W`](W) writer structure"]
impl crate::Writable for PsconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSCON to value 0"]
impl crate::Resettable for PsconSpec {
    const RESET_VALUE: u8 = 0;
}
