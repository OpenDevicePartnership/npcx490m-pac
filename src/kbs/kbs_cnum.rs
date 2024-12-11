#[doc = "Register `KBS_CNUM` reader"]
pub type R = crate::R<KbsCnumSpec>;
#[doc = "Register `KBS_CNUM` writer"]
pub type W = crate::W<KbsCnumSpec>;
#[doc = "Field `KBS_COL_NUM` reader - Keyboard Scan Columns Number"]
pub type KbsColNumR = crate::FieldReader;
#[doc = "Field `KBS_COL_NUM` writer - Keyboard Scan Columns Number"]
pub type KbsColNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Keyboard Scan Columns Number"]
    #[inline(always)]
    pub fn kbs_col_num(&self) -> KbsColNumR {
        KbsColNumR::new(self.bits & 0x1f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KBS_CNUM")
            .field("kbs_col_num", &self.kbs_col_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Keyboard Scan Columns Number"]
    #[inline(always)]
    pub fn kbs_col_num(&mut self) -> KbsColNumW<KbsCnumSpec> {
        KbsColNumW::new(self, 0)
    }
}
#[doc = "Keyboard Scan Columns Number Register (KBS_CNUM)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_cnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_cnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsCnumSpec;
impl crate::RegisterSpec for KbsCnumSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbs_cnum::R`](R) reader structure"]
impl crate::Readable for KbsCnumSpec {}
#[doc = "`write(|w| ..)` method takes [`kbs_cnum::W`](W) writer structure"]
impl crate::Writable for KbsCnumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBS_CNUM to value 0"]
impl crate::Resettable for KbsCnumSpec {
    const RESET_VALUE: u8 = 0;
}
