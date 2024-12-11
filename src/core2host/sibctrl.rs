#[doc = "Register `SIBCTRL` reader"]
pub type R = crate::R<SibctrlSpec>;
#[doc = "Register `SIBCTRL` writer"]
pub type W = crate::W<SibctrlSpec>;
#[doc = "Field `CSAE` reader - Core-to-Host Modules Access Enable"]
pub type CsaeR = crate::BitReader;
#[doc = "Field `CSAE` writer - Core-to-Host Modules Access Enable"]
pub type CsaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRD` reader - Core Read from Host Modules"]
pub type CsrdR = crate::BitReader;
#[doc = "Field `CSRD` writer - Core Read from Host Modules"]
pub type CsrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSWR` reader - Core Write to Host Modules"]
pub type CswrR = crate::BitReader;
#[doc = "Field `CSWR` writer - Core Write to Host Modules"]
pub type CswrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core-to-Host Modules Access Enable"]
    #[inline(always)]
    pub fn csae(&self) -> CsaeR {
        CsaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Read from Host Modules"]
    #[inline(always)]
    pub fn csrd(&self) -> CsrdR {
        CsrdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core Write to Host Modules"]
    #[inline(always)]
    pub fn cswr(&self) -> CswrR {
        CswrR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIBCTRL")
            .field("csae", &self.csae())
            .field("csrd", &self.csrd())
            .field("cswr", &self.cswr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Core-to-Host Modules Access Enable"]
    #[inline(always)]
    pub fn csae(&mut self) -> CsaeW<SibctrlSpec> {
        CsaeW::new(self, 0)
    }
    #[doc = "Bit 1 - Core Read from Host Modules"]
    #[inline(always)]
    pub fn csrd(&mut self) -> CsrdW<SibctrlSpec> {
        CsrdW::new(self, 1)
    }
    #[doc = "Bit 2 - Core Write to Host Modules"]
    #[inline(always)]
    pub fn cswr(&mut self) -> CswrW<SibctrlSpec> {
        CswrW::new(self, 2)
    }
}
#[doc = "HMIB Control Register (SIBCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`sibctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sibctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SibctrlSpec;
impl crate::RegisterSpec for SibctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sibctrl::R`](R) reader structure"]
impl crate::Readable for SibctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sibctrl::W`](W) writer structure"]
impl crate::Writable for SibctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SIBCTRL to value 0"]
impl crate::Resettable for SibctrlSpec {
    const RESET_VALUE: u8 = 0;
}
