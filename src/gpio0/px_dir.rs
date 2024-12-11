#[doc = "Register `PxDIR` reader"]
pub type R = crate::R<PxDirSpec>;
#[doc = "Register `PxDIR` writer"]
pub type W = crate::W<PxDirSpec>;
#[doc = "Direction for Pin %s\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    #[doc = "0: Direction Input"]
    Input = 0,
    #[doc = "1: Direction Output"]
    Output = 1,
}
impl From<Direction> for bool {
    #[inline(always)]
    fn from(variant: Direction) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN(0-7)` reader - Direction for Pin %s"]
pub type PinR = crate::BitReader<Direction>;
impl PinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Direction {
        match self.bits {
            false => Direction::Input,
            true => Direction::Output,
        }
    }
    #[doc = "Direction Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Direction::Input
    }
    #[doc = "Direction Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Direction::Output
    }
}
#[doc = "Field `PIN(0-7)` writer - Direction for Pin %s"]
pub type PinW<'a, REG> = crate::BitWriter<'a, REG, Direction>;
impl<'a, REG> PinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direction Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Direction::Input)
    }
    #[doc = "Direction Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Direction::Output)
    }
}
impl R {
    #[doc = "Direction for Pin (0-7)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PIN0` field.</div>"]
    #[inline(always)]
    pub fn pin(&self, n: u8) -> PinR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PinR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Direction for Pin (0-7)"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = PinR> + '_ {
        (0..8).map(move |n| PinR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Direction for Pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> PinR {
        PinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction for Pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> PinR {
        PinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction for Pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> PinR {
        PinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Direction for Pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> PinR {
        PinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction for Pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> PinR {
        PinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction for Pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> PinR {
        PinR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction for Pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> PinR {
        PinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Direction for Pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> PinR {
        PinR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PxDIR")
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
    #[doc = "Direction for Pin (0-7)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PIN0` field.</div>"]
    #[inline(always)]
    pub fn pin(&mut self, n: u8) -> PinW<PxDirSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PinW::new(self, n)
    }
    #[doc = "Bit 0 - Direction for Pin 0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PinW<PxDirSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bit 1 - Direction for Pin 1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PinW<PxDirSpec> {
        PinW::new(self, 1)
    }
    #[doc = "Bit 2 - Direction for Pin 2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PinW<PxDirSpec> {
        PinW::new(self, 2)
    }
    #[doc = "Bit 3 - Direction for Pin 3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PinW<PxDirSpec> {
        PinW::new(self, 3)
    }
    #[doc = "Bit 4 - Direction for Pin 4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PinW<PxDirSpec> {
        PinW::new(self, 4)
    }
    #[doc = "Bit 5 - Direction for Pin 5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PinW<PxDirSpec> {
        PinW::new(self, 5)
    }
    #[doc = "Bit 6 - Direction for Pin 6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PinW<PxDirSpec> {
        PinW::new(self, 6)
    }
    #[doc = "Bit 7 - Direction for Pin 7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PinW<PxDirSpec> {
        PinW::new(self, 7)
    }
}
#[doc = "Port GPIOx Direction Register (PxDIR)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PxDirSpec;
impl crate::RegisterSpec for PxDirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`px_dir::R`](R) reader structure"]
impl crate::Readable for PxDirSpec {}
#[doc = "`write(|w| ..)` method takes [`px_dir::W`](W) writer structure"]
impl crate::Writable for PxDirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PxDIR to value 0"]
impl crate::Resettable for PxDirSpec {
    const RESET_VALUE: u8 = 0;
}
