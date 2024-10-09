#[doc = "Register `UFRSn` reader"]
pub type R = crate::R<UfrsnSpec>;
#[doc = "Register `UFRSn` writer"]
pub type W = crate::W<UfrsnSpec>;
#[doc = "Stop Bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopBits {
    #[doc = "0: `0`"]
    OneBit = 0,
    #[doc = "1: `1`"]
    TwoBits = 1,
}
impl From<StopBits> for bool {
    #[inline(always)]
    fn from(variant: StopBits) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STP` reader - Stop Bits"]
pub type StpR = crate::BitReader<StopBits>;
impl StpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopBits {
        match self.bits {
            false => StopBits::OneBit,
            true => StopBits::TwoBits,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_one_bit(&self) -> bool {
        *self == StopBits::OneBit
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_two_bits(&self) -> bool {
        *self == StopBits::TwoBits
    }
}
#[doc = "Field `STP` writer - Stop Bits"]
pub type StpW<'a, REG> = crate::BitWriter<'a, REG, StopBits>;
impl<'a, REG> StpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn one_bit(self) -> &'a mut crate::W<REG> {
        self.variant(StopBits::OneBit)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn two_bits(self) -> &'a mut crate::W<REG> {
        self.variant(StopBits::TwoBits)
    }
}
#[doc = "Parity Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ParitySelect {
    #[doc = "0: `0`"]
    Odd = 0,
    #[doc = "1: `1`"]
    Even = 1,
    #[doc = "2: `10`"]
    Mark = 2,
    #[doc = "3: `11`"]
    Space = 3,
}
impl From<ParitySelect> for u8 {
    #[inline(always)]
    fn from(variant: ParitySelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ParitySelect {
    type Ux = u8;
}
impl crate::IsEnum for ParitySelect {}
#[doc = "Field `PSEL` reader - Parity Select"]
pub type PselR = crate::FieldReader<ParitySelect>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ParitySelect {
        match self.bits {
            0 => ParitySelect::Odd,
            1 => ParitySelect::Even,
            2 => ParitySelect::Mark,
            3 => ParitySelect::Space,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == ParitySelect::Odd
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == ParitySelect::Even
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == ParitySelect::Mark
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == ParitySelect::Space
    }
}
#[doc = "Field `PSEL` writer - Parity Select"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 2, ParitySelect, crate::Safe>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(ParitySelect::Odd)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(ParitySelect::Even)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut crate::W<REG> {
        self.variant(ParitySelect::Mark)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn space(self) -> &'a mut crate::W<REG> {
        self.variant(ParitySelect::Space)
    }
}
#[doc = "Field `PEN` reader - Parity Enable"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Parity Enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Stop Bits"]
    #[inline(always)]
    pub fn stp(&self) -> StpR {
        StpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UFRSn")
            .field("stp", &self.stp())
            .field("psel", &self.psel())
            .field("pen", &self.pen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Stop Bits"]
    #[inline(always)]
    #[must_use]
    pub fn stp(&mut self) -> StpW<UfrsnSpec> {
        StpW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<UfrsnSpec> {
        PselW::new(self, 4)
    }
    #[doc = "Bit 6 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<UfrsnSpec> {
        PenW::new(self, 6)
    }
}
#[doc = "Frame Select Register (UFRSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ufrsn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ufrsn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UfrsnSpec;
impl crate::RegisterSpec for UfrsnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ufrsn::R`](R) reader structure"]
impl crate::Readable for UfrsnSpec {}
#[doc = "`write(|w| ..)` method takes [`ufrsn::W`](W) writer structure"]
impl crate::Writable for UfrsnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UFRSn to value 0"]
impl crate::Resettable for UfrsnSpec {
    const RESET_VALUE: u8 = 0;
}
