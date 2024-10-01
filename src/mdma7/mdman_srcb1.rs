#[doc = "Register `MDMAn_SRCB1` reader"]
pub type R = crate::R<MdmanSrcb1Spec>;
#[doc = "Register `MDMAn_SRCB1` writer"]
pub type W = crate::W<MdmanSrcb1Spec>;
#[doc = "Field `SRC_BASE_ADDR19_0` reader - 20-bit Source Base Address"]
pub type SrcBaseAddr19_0R = crate::FieldReader<u32>;
#[doc = "Field `SRC_BASE_ADDR19_0` writer - 20-bit Source Base Address"]
pub type SrcBaseAddr19_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SRC_BASE_ADDR31_20` reader - 12-bit Source Base Address"]
pub type SrcBaseAddr31_20R = crate::FieldReader<u16>;
#[doc = "Field `SRC_BASE_ADDR31_20` writer - 12-bit Source Base Address"]
pub type SrcBaseAddr31_20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 20-bit Source Base Address"]
    #[inline(always)]
    pub fn src_base_addr19_0(&self) -> SrcBaseAddr19_0R {
        SrcBaseAddr19_0R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - 12-bit Source Base Address"]
    #[inline(always)]
    pub fn src_base_addr31_20(&self) -> SrcBaseAddr31_20R {
        SrcBaseAddr31_20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDMAn_SRCB1")
            .field("src_base_addr19_0", &self.src_base_addr19_0())
            .field("src_base_addr31_20", &self.src_base_addr31_20())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - 20-bit Source Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn src_base_addr19_0(&mut self) -> SrcBaseAddr19_0W<MdmanSrcb1Spec> {
        SrcBaseAddr19_0W::new(self, 0)
    }
    #[doc = "Bits 20:31 - 12-bit Source Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn src_base_addr31_20(&mut self) -> SrcBaseAddr31_20W<MdmanSrcb1Spec> {
        SrcBaseAddr31_20W::new(self, 20)
    }
}
#[doc = "Channel 1 Source Base Address Register (MDMAn_SRCB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdman_srcb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdman_srcb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmanSrcb1Spec;
impl crate::RegisterSpec for MdmanSrcb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdman_srcb1::R`](R) reader structure"]
impl crate::Readable for MdmanSrcb1Spec {}
#[doc = "`write(|w| ..)` method takes [`mdman_srcb1::W`](W) writer structure"]
impl crate::Writable for MdmanSrcb1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMAn_SRCB1 to value 0"]
impl crate::Resettable for MdmanSrcb1Spec {
    const RESET_VALUE: u32 = 0;
}
