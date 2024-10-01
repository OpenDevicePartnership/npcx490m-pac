#[doc = "Register `TWCP` reader"]
pub type R = crate::R<TwcpSpec>;
#[doc = "Register `TWCP` writer"]
pub type W = crate::W<TwcpSpec>;
#[doc = "Field `MDIV` reader - default"]
pub type MdivR = crate::FieldReader;
#[doc = "Field `MDIV` writer - default"]
pub type MdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - default"]
    #[inline(always)]
    pub fn mdiv(&self) -> MdivR {
        MdivR::new(self.bits & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWCP").field("mdiv", &self.mdiv()).finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - default"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv(&mut self) -> MdivW<TwcpSpec> {
        MdivW::new(self, 0)
    }
}
#[doc = "Timer and Watchdog Clock Prescaler Register (TWCP)\n\nYou can [`read`](crate::Reg::read) this register and get [`twcp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwcpSpec;
impl crate::RegisterSpec for TwcpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twcp::R`](R) reader structure"]
impl crate::Readable for TwcpSpec {}
#[doc = "`write(|w| ..)` method takes [`twcp::W`](W) writer structure"]
impl crate::Writable for TwcpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TWCP to value 0"]
impl crate::Resettable for TwcpSpec {
    const RESET_VALUE: u8 = 0;
}
