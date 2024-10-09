#[doc = "Register `FLM_CQ%s` reader"]
pub type R = crate::R<FlmCqSpec>;
#[doc = "Register `FLM_CQ%s` writer"]
pub type W = crate::W<FlmCqSpec>;
#[doc = "Field `QMASK(0-7)` reader - Qualifier Compare Mask"]
pub type QmaskR = crate::BitReader;
#[doc = "Field `QMASK(0-7)` writer - Qualifier Compare Mask"]
pub type QmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QVAL` reader - Qualifier Compare Value"]
pub type QvalR = crate::FieldReader;
#[doc = "Field `QVAL` writer - Qualifier Compare Value"]
pub type QvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Qualifier Compare Byte Index\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qbyte {
    #[doc = "1: `1`"]
    FirstByte = 1,
    #[doc = "2: `10`"]
    SecondByte = 2,
    #[doc = "3: `11`"]
    ThirdByte = 3,
    #[doc = "4: `100`"]
    FourthByte = 4,
    #[doc = "5: `101`"]
    FifthByte = 5,
    #[doc = "6: `110`"]
    SixthByte = 6,
    #[doc = "7: `111`"]
    SeventhByte = 7,
}
impl From<Qbyte> for u8 {
    #[inline(always)]
    fn from(variant: Qbyte) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qbyte {
    type Ux = u8;
}
impl crate::IsEnum for Qbyte {}
#[doc = "Field `QBYTE` reader - Qualifier Compare Byte Index"]
pub type QbyteR = crate::FieldReader<Qbyte>;
impl QbyteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Qbyte> {
        match self.bits {
            1 => Some(Qbyte::FirstByte),
            2 => Some(Qbyte::SecondByte),
            3 => Some(Qbyte::ThirdByte),
            4 => Some(Qbyte::FourthByte),
            5 => Some(Qbyte::FifthByte),
            6 => Some(Qbyte::SixthByte),
            7 => Some(Qbyte::SeventhByte),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_first_byte(&self) -> bool {
        *self == Qbyte::FirstByte
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_second_byte(&self) -> bool {
        *self == Qbyte::SecondByte
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_third_byte(&self) -> bool {
        *self == Qbyte::ThirdByte
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_fourth_byte(&self) -> bool {
        *self == Qbyte::FourthByte
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_fifth_byte(&self) -> bool {
        *self == Qbyte::FifthByte
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_sixth_byte(&self) -> bool {
        *self == Qbyte::SixthByte
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_seventh_byte(&self) -> bool {
        *self == Qbyte::SeventhByte
    }
}
#[doc = "Field `QBYTE` writer - Qualifier Compare Byte Index"]
pub type QbyteW<'a, REG> = crate::FieldWriter<'a, REG, 4, Qbyte>;
impl<'a, REG> QbyteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn first_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Qbyte::FirstByte)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn second_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Qbyte::SecondByte)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn third_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Qbyte::ThirdByte)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn fourth_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Qbyte::FourthByte)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn fifth_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Qbyte::FifthByte)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sixth_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Qbyte::SixthByte)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn seventh_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Qbyte::SeventhByte)
    }
}
#[doc = "Field `QPOL` reader - Qualifier Compare Polarity"]
pub type QpolR = crate::BitReader;
#[doc = "Field `QPOL` writer - Qualifier Compare Polarity"]
pub type QpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QEN(0-7)` reader - Qualifier Enable"]
pub type QenR = crate::BitReader;
#[doc = "Field `QEN(0-7)` writer - Qualifier Enable"]
pub type QenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Qualifier Compare Mask"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `QMASK0` field.</div>"]
    #[inline(always)]
    pub fn qmask(&self, n: u8) -> QmaskR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        QmaskR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Qualifier Compare Mask"]
    #[inline(always)]
    pub fn qmask_iter(&self) -> impl Iterator<Item = QmaskR> + '_ {
        (0..8).map(move |n| QmaskR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Qualifier Compare Mask"]
    #[inline(always)]
    pub fn qmask0(&self) -> QmaskR {
        QmaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Qualifier Compare Mask"]
    #[inline(always)]
    pub fn qmask1(&self) -> QmaskR {
        QmaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Qualifier Compare Mask"]
    #[inline(always)]
    pub fn qmask2(&self) -> QmaskR {
        QmaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Qualifier Compare Mask"]
    #[inline(always)]
    pub fn qmask3(&self) -> QmaskR {
        QmaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Qualifier Compare Mask"]
    #[inline(always)]
    pub fn qmask4(&self) -> QmaskR {
        QmaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Qualifier Compare Mask"]
    #[inline(always)]
    pub fn qmask5(&self) -> QmaskR {
        QmaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Qualifier Compare Mask"]
    #[inline(always)]
    pub fn qmask6(&self) -> QmaskR {
        QmaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Qualifier Compare Mask"]
    #[inline(always)]
    pub fn qmask7(&self) -> QmaskR {
        QmaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Qualifier Compare Value"]
    #[inline(always)]
    pub fn qval(&self) -> QvalR {
        QvalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Qualifier Compare Byte Index"]
    #[inline(always)]
    pub fn qbyte(&self) -> QbyteR {
        QbyteR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Qualifier Compare Polarity"]
    #[inline(always)]
    pub fn qpol(&self) -> QpolR {
        QpolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Qualifier Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `QEN0` field.</div>"]
    #[inline(always)]
    pub fn qen(&self, n: u8) -> QenR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        QenR::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Qualifier Enable"]
    #[inline(always)]
    pub fn qen_iter(&self) -> impl Iterator<Item = QenR> + '_ {
        (0..8).map(move |n| QenR::new(((self.bits >> (n + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - Qualifier Enable"]
    #[inline(always)]
    pub fn qen0(&self) -> QenR {
        QenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Qualifier Enable"]
    #[inline(always)]
    pub fn qen1(&self) -> QenR {
        QenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Qualifier Enable"]
    #[inline(always)]
    pub fn qen2(&self) -> QenR {
        QenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Qualifier Enable"]
    #[inline(always)]
    pub fn qen3(&self) -> QenR {
        QenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Qualifier Enable"]
    #[inline(always)]
    pub fn qen4(&self) -> QenR {
        QenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Qualifier Enable"]
    #[inline(always)]
    pub fn qen5(&self) -> QenR {
        QenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Qualifier Enable"]
    #[inline(always)]
    pub fn qen6(&self) -> QenR {
        QenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Qualifier Enable"]
    #[inline(always)]
    pub fn qen7(&self) -> QenR {
        QenR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_CQ")
            .field("qmask0", &self.qmask0())
            .field("qmask1", &self.qmask1())
            .field("qmask2", &self.qmask2())
            .field("qmask3", &self.qmask3())
            .field("qmask4", &self.qmask4())
            .field("qmask5", &self.qmask5())
            .field("qmask6", &self.qmask6())
            .field("qmask7", &self.qmask7())
            .field("qval", &self.qval())
            .field("qbyte", &self.qbyte())
            .field("qpol", &self.qpol())
            .field("qen0", &self.qen0())
            .field("qen1", &self.qen1())
            .field("qen2", &self.qen2())
            .field("qen3", &self.qen3())
            .field("qen4", &self.qen4())
            .field("qen5", &self.qen5())
            .field("qen6", &self.qen6())
            .field("qen7", &self.qen7())
            .finish()
    }
}
impl W {
    #[doc = "Qualifier Compare Mask"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `QMASK0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn qmask(&mut self, n: u8) -> QmaskW<FlmCqSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        QmaskW::new(self, n)
    }
    #[doc = "Bit 0 - Qualifier Compare Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qmask0(&mut self) -> QmaskW<FlmCqSpec> {
        QmaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Qualifier Compare Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qmask1(&mut self) -> QmaskW<FlmCqSpec> {
        QmaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Qualifier Compare Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qmask2(&mut self) -> QmaskW<FlmCqSpec> {
        QmaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Qualifier Compare Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qmask3(&mut self) -> QmaskW<FlmCqSpec> {
        QmaskW::new(self, 3)
    }
    #[doc = "Bit 4 - Qualifier Compare Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qmask4(&mut self) -> QmaskW<FlmCqSpec> {
        QmaskW::new(self, 4)
    }
    #[doc = "Bit 5 - Qualifier Compare Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qmask5(&mut self) -> QmaskW<FlmCqSpec> {
        QmaskW::new(self, 5)
    }
    #[doc = "Bit 6 - Qualifier Compare Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qmask6(&mut self) -> QmaskW<FlmCqSpec> {
        QmaskW::new(self, 6)
    }
    #[doc = "Bit 7 - Qualifier Compare Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qmask7(&mut self) -> QmaskW<FlmCqSpec> {
        QmaskW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Qualifier Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn qval(&mut self) -> QvalW<FlmCqSpec> {
        QvalW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Qualifier Compare Byte Index"]
    #[inline(always)]
    #[must_use]
    pub fn qbyte(&mut self) -> QbyteW<FlmCqSpec> {
        QbyteW::new(self, 16)
    }
    #[doc = "Bit 20 - Qualifier Compare Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn qpol(&mut self) -> QpolW<FlmCqSpec> {
        QpolW::new(self, 20)
    }
    #[doc = "Qualifier Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `QEN0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn qen(&mut self, n: u8) -> QenW<FlmCqSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        QenW::new(self, n + 24)
    }
    #[doc = "Bit 24 - Qualifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qen0(&mut self) -> QenW<FlmCqSpec> {
        QenW::new(self, 24)
    }
    #[doc = "Bit 25 - Qualifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qen1(&mut self) -> QenW<FlmCqSpec> {
        QenW::new(self, 25)
    }
    #[doc = "Bit 26 - Qualifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qen2(&mut self) -> QenW<FlmCqSpec> {
        QenW::new(self, 26)
    }
    #[doc = "Bit 27 - Qualifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qen3(&mut self) -> QenW<FlmCqSpec> {
        QenW::new(self, 27)
    }
    #[doc = "Bit 28 - Qualifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qen4(&mut self) -> QenW<FlmCqSpec> {
        QenW::new(self, 28)
    }
    #[doc = "Bit 29 - Qualifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qen5(&mut self) -> QenW<FlmCqSpec> {
        QenW::new(self, 29)
    }
    #[doc = "Bit 30 - Qualifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qen6(&mut self) -> QenW<FlmCqSpec> {
        QenW::new(self, 30)
    }
    #[doc = "Bit 31 - Qualifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qen7(&mut self) -> QenW<FlmCqSpec> {
        QenW::new(self, 31)
    }
}
#[doc = "FLM Command Qualifier Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCqSpec;
impl crate::RegisterSpec for FlmCqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cq::R`](R) reader structure"]
impl crate::Readable for FlmCqSpec {}
#[doc = "`write(|w| ..)` method takes [`flm_cq::W`](W) writer structure"]
impl crate::Writable for FlmCqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CQ%s to value 0"]
impl crate::Resettable for FlmCqSpec {
    const RESET_VALUE: u32 = 0;
}
