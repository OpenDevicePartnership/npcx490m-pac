#[doc = "Register `SHCFG` reader"]
pub type R = crate::R<ShcfgSpec>;
#[doc = "Register `SHCFG` writer"]
pub type W = crate::W<ShcfgSpec>;
#[doc = "Field `IMAEN` reader - Indirect Memory Access Enable"]
pub type ImaenR = crate::BitReader;
#[doc = "Field `IMAEN` writer - Indirect Memory Access Enable"]
pub type ImaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indirect Memory Access Enable"]
    #[inline(always)]
    pub fn imaen(&self) -> ImaenR {
        ImaenR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHCFG")
            .field("imaen", &self.imaen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Indirect Memory Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn imaen(&mut self) -> ImaenW<ShcfgSpec> {
        ImaenW::new(self, 0)
    }
}
#[doc = "Shared Memory Configuration Register (SHCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`shcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShcfgSpec;
impl crate::RegisterSpec for ShcfgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`shcfg::R`](R) reader structure"]
impl crate::Readable for ShcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`shcfg::W`](W) writer structure"]
impl crate::Writable for ShcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SHCFG to value 0"]
impl crate::Resettable for ShcfgSpec {
    const RESET_VALUE: u16 = 0;
}
