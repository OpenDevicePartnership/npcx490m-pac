#[doc = "Register `OOBCTL` reader"]
pub type R = crate::R<OobctlSpec>;
#[doc = "Register `OOBCTL` writer"]
pub type W = crate::W<OobctlSpec>;
#[doc = "Field `OOB_FREE` reader - OOB Receive Queue Free"]
pub type OobFreeR = crate::BitReader;
#[doc = "Field `OOB_FREE` writer - OOB Receive Queue Free"]
pub type OobFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOB_AVAIL` reader - OOB Transmit Queue Available"]
pub type OobAvailR = crate::BitReader;
#[doc = "Field `OOB_AVAIL` writer - OOB Transmit Queue Available"]
pub type OobAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTBUFHEADS` reader - Reset Buffer Heads"]
pub type RstbufheadsR = crate::BitReader;
#[doc = "Field `RSTBUFHEADS` writer - Reset Buffer Heads"]
pub type RstbufheadsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOBPLSIZE` reader - OOB Channel Maximum Payload Size"]
pub type OobplsizeR = crate::FieldReader;
#[doc = "Field `OOBPLSIZE` writer - OOB Channel Maximum Payload Size"]
pub type OobplsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - OOB Receive Queue Free"]
    #[inline(always)]
    pub fn oob_free(&self) -> OobFreeR {
        OobFreeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OOB Transmit Queue Available"]
    #[inline(always)]
    pub fn oob_avail(&self) -> OobAvailR {
        OobAvailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset Buffer Heads"]
    #[inline(always)]
    pub fn rstbufheads(&self) -> RstbufheadsR {
        RstbufheadsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 10:12 - OOB Channel Maximum Payload Size"]
    #[inline(always)]
    pub fn oobplsize(&self) -> OobplsizeR {
        OobplsizeR::new(((self.bits >> 10) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OOBCTL")
            .field("oob_free", &self.oob_free())
            .field("oob_avail", &self.oob_avail())
            .field("rstbufheads", &self.rstbufheads())
            .field("oobplsize", &self.oobplsize())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - OOB Receive Queue Free"]
    #[inline(always)]
    pub fn oob_free(&mut self) -> OobFreeW<OobctlSpec> {
        OobFreeW::new(self, 0)
    }
    #[doc = "Bit 1 - OOB Transmit Queue Available"]
    #[inline(always)]
    pub fn oob_avail(&mut self) -> OobAvailW<OobctlSpec> {
        OobAvailW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset Buffer Heads"]
    #[inline(always)]
    pub fn rstbufheads(&mut self) -> RstbufheadsW<OobctlSpec> {
        RstbufheadsW::new(self, 2)
    }
    #[doc = "Bits 10:12 - OOB Channel Maximum Payload Size"]
    #[inline(always)]
    pub fn oobplsize(&mut self) -> OobplsizeW<OobctlSpec> {
        OobplsizeW::new(self, 10)
    }
}
#[doc = "OOB Channel Control Register (OOBCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OobctlSpec;
impl crate::RegisterSpec for OobctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oobctl::R`](R) reader structure"]
impl crate::Readable for OobctlSpec {}
#[doc = "`write(|w| ..)` method takes [`oobctl::W`](W) writer structure"]
impl crate::Writable for OobctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OOBCTL to value 0"]
impl crate::Resettable for OobctlSpec {
    const RESET_VALUE: u32 = 0;
}
