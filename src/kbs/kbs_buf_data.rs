#[doc = "Register `KBS_BUF_DATA` reader"]
pub type R = crate::R<KbsBufDataSpec>;
#[doc = "Register `KBS_BUF_DATA` writer"]
pub type W = crate::W<KbsBufDataSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Keyboard Scan Buffer Data Register (KBS_BUF_DATA)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_buf_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_buf_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsBufDataSpec;
impl crate::RegisterSpec for KbsBufDataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbs_buf_data::R`](R) reader structure"]
impl crate::Readable for KbsBufDataSpec {}
#[doc = "`write(|w| ..)` method takes [`kbs_buf_data::W`](W) writer structure"]
impl crate::Writable for KbsBufDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBS_BUF_DATA to value 0"]
impl crate::Resettable for KbsBufDataSpec {
    const RESET_VALUE: u8 = 0;
}
