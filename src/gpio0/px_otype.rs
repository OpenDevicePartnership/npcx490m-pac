#[doc = "Register `PxOTYPE` reader"]
pub type R = crate::R<PxOtypeSpec>;
#[doc = "Register `PxOTYPE` writer"]
pub type W = crate::W<PxOtypeSpec>;
#[doc = "Output Type selection for Pin %s\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    #[doc = "0: Push-pull"]
    Pushpull = 0,
    #[doc = "1: Open-drain"]
    Opendrain = 1,
}
impl From<Type> for bool {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN(0-7)` reader - Output Type selection for Pin %s"]
pub type PinR = crate::BitReader<Type>;
impl PinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Type {
        match self.bits {
            false => Type::Pushpull,
            true => Type::Opendrain,
        }
    }
    #[doc = "Push-pull"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Type::Pushpull
    }
    #[doc = "Open-drain"]
    #[inline(always)]
    pub fn is_opendrain(&self) -> bool {
        *self == Type::Opendrain
    }
}
#[doc = "Field `PIN(0-7)` writer - Output Type selection for Pin %s"]
pub type PinW<'a, REG> = crate::BitWriter<'a, REG, Type>;
impl<'a, REG> PinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Pushpull)
    }
    #[doc = "Open-drain"]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Opendrain)
    }
}
impl R {
    #[doc = "Output Type selection for Pin (0-7)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PIN0` field.</div>"]
    #[inline(always)]
    pub fn pin(&self, n: u8) -> PinR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PinR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output Type selection for Pin (0-7)"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = PinR> + '_ {
        (0..8).map(move |n| PinR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Output Type selection for Pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> PinR {
        PinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Type selection for Pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> PinR {
        PinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Type selection for Pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> PinR {
        PinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Type selection for Pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> PinR {
        PinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output Type selection for Pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> PinR {
        PinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output Type selection for Pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> PinR {
        PinR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Type selection for Pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> PinR {
        PinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output Type selection for Pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> PinR {
        PinR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PxOTYPE")
            .field("pin0", &self.pin0())
            .field("pin1", &self.pin1())
            .field("pin2", &self.pin2())
            .field("pin3", &self.pin3())
            .field("pin4", &self.pin4())
            .field("pin5", &self.pin5())
            .field("pin6", &self.pin6())
            .field("pin7", &self.pin7())
            .finish()
    }
}
impl W {
    #[doc = "Output Type selection for Pin (0-7)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PIN0` field.</div>"]
    #[inline(always)]
    pub fn pin(&mut self, n: u8) -> PinW<PxOtypeSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PinW::new(self, n)
    }
    #[doc = "Bit 0 - Output Type selection for Pin 0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PinW<PxOtypeSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bit 1 - Output Type selection for Pin 1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PinW<PxOtypeSpec> {
        PinW::new(self, 1)
    }
    #[doc = "Bit 2 - Output Type selection for Pin 2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PinW<PxOtypeSpec> {
        PinW::new(self, 2)
    }
    #[doc = "Bit 3 - Output Type selection for Pin 3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PinW<PxOtypeSpec> {
        PinW::new(self, 3)
    }
    #[doc = "Bit 4 - Output Type selection for Pin 4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PinW<PxOtypeSpec> {
        PinW::new(self, 4)
    }
    #[doc = "Bit 5 - Output Type selection for Pin 5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PinW<PxOtypeSpec> {
        PinW::new(self, 5)
    }
    #[doc = "Bit 6 - Output Type selection for Pin 6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PinW<PxOtypeSpec> {
        PinW::new(self, 6)
    }
    #[doc = "Bit 7 - Output Type selection for Pin 7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PinW<PxOtypeSpec> {
        PinW::new(self, 7)
    }
}
#[doc = "Port GPIOx Output Type Register (PxOTYPE)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_otype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_otype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PxOtypeSpec;
impl crate::RegisterSpec for PxOtypeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`px_otype::R`](R) reader structure"]
impl crate::Readable for PxOtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`px_otype::W`](W) writer structure"]
impl crate::Writable for PxOtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PxOTYPE to value 0"]
impl crate::Resettable for PxOtypeSpec {
    const RESET_VALUE: u8 = 0;
}
