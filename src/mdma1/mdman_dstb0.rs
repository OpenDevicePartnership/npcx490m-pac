#[doc = "Register `MDMAn_DSTB0` reader"]
pub type R = crate::R<MdmanDstb0Spec>;
#[doc = "Register `MDMAn_DSTB0` writer"]
pub type W = crate::W<MdmanDstb0Spec>;
#[doc = "Field `DST_BASE_ADDR19_0` reader - 20-bit Destination Base Address"]
pub type DstBaseAddr19_0R = crate::FieldReader<u32>;
#[doc = "Field `DST_BASE_ADDR19_0` writer - 20-bit Destination Base Address"]
pub type DstBaseAddr19_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DST_BASE_ADDR31_20` reader - 32-bit Destination Base Address"]
pub type DstBaseAddr31_20R = crate::FieldReader<u16>;
#[doc = "Field `DST_BASE_ADDR31_20` writer - 32-bit Destination Base Address"]
pub type DstBaseAddr31_20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 20-bit Destination Base Address"]
    #[inline(always)]
    pub fn dst_base_addr19_0(&self) -> DstBaseAddr19_0R {
        DstBaseAddr19_0R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - 32-bit Destination Base Address"]
    #[inline(always)]
    pub fn dst_base_addr31_20(&self) -> DstBaseAddr31_20R {
        DstBaseAddr31_20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDMAn_DSTB0")
            .field("dst_base_addr19_0", &self.dst_base_addr19_0())
            .field("dst_base_addr31_20", &self.dst_base_addr31_20())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - 20-bit Destination Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn dst_base_addr19_0(&mut self) -> DstBaseAddr19_0W<MdmanDstb0Spec> {
        DstBaseAddr19_0W::new(self, 0)
    }
    #[doc = "Bits 20:31 - 32-bit Destination Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn dst_base_addr31_20(&mut self) -> DstBaseAddr31_20W<MdmanDstb0Spec> {
        DstBaseAddr31_20W::new(self, 20)
    }
}
#[doc = "Channel 0 Destination Base Address Register (MDMAn_DSTB0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdman_dstb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdman_dstb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmanDstb0Spec;
impl crate::RegisterSpec for MdmanDstb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdman_dstb0::R`](R) reader structure"]
impl crate::Readable for MdmanDstb0Spec {}
#[doc = "`write(|w| ..)` method takes [`mdman_dstb0::W`](W) writer structure"]
impl crate::Writable for MdmanDstb0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMAn_DSTB0 to value 0"]
impl crate::Resettable for MdmanDstb0Spec {
    const RESET_VALUE: u32 = 0;
}
