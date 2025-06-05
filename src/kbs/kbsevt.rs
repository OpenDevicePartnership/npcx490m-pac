#[doc = "Register `KBSEVT` reader"]
pub type R = crate::R<KbsevtSpec>;
#[doc = "Register `KBSEVT` writer"]
pub type W = crate::W<KbsevtSpec>;
#[doc = "Field `KBSDONE` reader - Keyboard Scan Done Indication"]
pub type KbsdoneR = crate::BitReader;
#[doc = "Field `KBSDONE` writer - Keyboard Scan Done Indication"]
pub type KbsdoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `KBSERR` reader - Keyboard Scan Error"]
pub type KbserrR = crate::BitReader;
#[doc = "Field `KBSERR` writer - Keyboard Scan Error"]
pub type KbserrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Keyboard Scan Done Indication"]
    #[inline(always)]
    pub fn kbsdone(&self) -> KbsdoneR {
        KbsdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keyboard Scan Error"]
    #[inline(always)]
    pub fn kbserr(&self) -> KbserrR {
        KbserrR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KBSEVT")
            .field("kbsdone", &self.kbsdone())
            .field("kbserr", &self.kbserr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Keyboard Scan Done Indication"]
    #[inline(always)]
    pub fn kbsdone(&mut self) -> KbsdoneW<KbsevtSpec> {
        KbsdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Keyboard Scan Error"]
    #[inline(always)]
    pub fn kbserr(&mut self) -> KbserrW<KbsevtSpec> {
        KbserrW::new(self, 1)
    }
}
#[doc = "Keyboard Scan Event Register (KBSEVT)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsevt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsevt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsevtSpec;
impl crate::RegisterSpec for KbsevtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbsevt::R`](R) reader structure"]
impl crate::Readable for KbsevtSpec {}
#[doc = "`write(|w| ..)` method takes [`kbsevt::W`](W) writer structure"]
impl crate::Writable for KbsevtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x03;
}
#[doc = "`reset()` method sets KBSEVT to value 0"]
impl crate::Resettable for KbsevtSpec {
    const RESET_VALUE: u8 = 0;
}
