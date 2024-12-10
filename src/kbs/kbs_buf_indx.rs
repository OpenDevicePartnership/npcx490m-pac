#[doc = "Register `KBS_BUF_INDX` reader"]
pub type R = crate::R<KbsBufIndxSpec>;
#[doc = "Register `KBS_BUF_INDX` writer"]
pub type W = crate::W<KbsBufIndxSpec>;
#[doc = "Field `KBS_BUF_INDX` reader - Keyboard Scan Buffer Index"]
pub type KbsBufIndxR = crate::FieldReader;
#[doc = "Field `KBS_BUF_INDX` writer - Keyboard Scan Buffer Index"]
pub type KbsBufIndxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Keyboard Scan Buffer Index"]
    #[inline(always)]
    pub fn kbs_buf_indx(&self) -> KbsBufIndxR {
        KbsBufIndxR::new(self.bits & 0x1f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KBS_BUF_INDX")
            .field("kbs_buf_indx", &self.kbs_buf_indx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Keyboard Scan Buffer Index"]
    #[inline(always)]
    pub fn kbs_buf_indx(&mut self) -> KbsBufIndxW<KbsBufIndxSpec> {
        KbsBufIndxW::new(self, 0)
    }
}
#[doc = "Keyboard Scan Buffer Index Register (KBS_BUF_INDX)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_buf_indx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_buf_indx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsBufIndxSpec;
impl crate::RegisterSpec for KbsBufIndxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbs_buf_indx::R`](R) reader structure"]
impl crate::Readable for KbsBufIndxSpec {}
#[doc = "`write(|w| ..)` method takes [`kbs_buf_indx::W`](W) writer structure"]
impl crate::Writable for KbsBufIndxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBS_BUF_INDX to value 0"]
impl crate::Resettable for KbsBufIndxSpec {
    const RESET_VALUE: u8 = 0;
}
