#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `IBFS` reader - Input Buffer Full Status"]
pub type IbfsR = crate::BitReader;
#[doc = "Field `IBFS` writer - Input Buffer Full Status"]
pub type IbfsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Input Buffer Full Status"]
    #[inline(always)]
    pub fn ibfs(&self) -> IbfsR {
        IbfsR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("ibfs", &self.ibfs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - Input Buffer Full Status"]
    #[inline(always)]
    #[must_use]
    pub fn ibfs(&mut self) -> IbfsW<StatusSpec> {
        IbfsW::new(self, 7)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u8 = 0;
}
