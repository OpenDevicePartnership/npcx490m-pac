#[doc = "Register `WKSTn%s` reader"]
pub type R = crate::R<WkstnSpec>;
#[doc = "Register `WKSTn%s` writer"]
pub type W = crate::W<WkstnSpec>;
#[doc = "Represents the current value of input\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: Input is 0 (low)"]
    Low = 0,
    #[doc = "1: Input is 1 (high)"]
    High = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT(0-7)` reader - Represents the current value of input"]
pub type InputR = crate::BitReader<Status>;
impl InputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Low,
            true => Status::High,
        }
    }
    #[doc = "Input is 0 (low)"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Status::Low
    }
    #[doc = "Input is 1 (high)"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Status::High
    }
}
#[doc = "Field `INPUT(0-7)` writer - Represents the current value of input"]
pub type InputW<'a, REG> = crate::BitWriter<'a, REG, Status>;
impl<'a, REG> InputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input is 0 (low)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Low)
    }
    #[doc = "Input is 1 (high)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Status::High)
    }
}
impl R {
    #[doc = "Represents the current value of input"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&self, n: u8) -> InputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Represents the current value of input"]
    #[inline(always)]
    pub fn input_iter(&self) -> impl Iterator<Item = InputR> + '_ {
        (0..8).map(move |n| InputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Represents the current value of input"]
    #[inline(always)]
    pub fn input0(&self) -> InputR {
        InputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents the current value of input"]
    #[inline(always)]
    pub fn input1(&self) -> InputR {
        InputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents the current value of input"]
    #[inline(always)]
    pub fn input2(&self) -> InputR {
        InputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents the current value of input"]
    #[inline(always)]
    pub fn input3(&self) -> InputR {
        InputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the current value of input"]
    #[inline(always)]
    pub fn input4(&self) -> InputR {
        InputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents the current value of input"]
    #[inline(always)]
    pub fn input5(&self) -> InputR {
        InputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents the current value of input"]
    #[inline(always)]
    pub fn input6(&self) -> InputR {
        InputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents the current value of input"]
    #[inline(always)]
    pub fn input7(&self) -> InputR {
        InputR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKSTn")
            .field("input0", &self.input0())
            .field("input1", &self.input1())
            .field("input2", &self.input2())
            .field("input3", &self.input3())
            .field("input4", &self.input4())
            .field("input5", &self.input5())
            .field("input6", &self.input6())
            .field("input7", &self.input7())
            .finish()
    }
}
impl W {
    #[doc = "Represents the current value of input"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&mut self, n: u8) -> InputW<WkstnSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputW::new(self, n)
    }
    #[doc = "Bit 0 - Represents the current value of input"]
    #[inline(always)]
    pub fn input0(&mut self) -> InputW<WkstnSpec> {
        InputW::new(self, 0)
    }
    #[doc = "Bit 1 - Represents the current value of input"]
    #[inline(always)]
    pub fn input1(&mut self) -> InputW<WkstnSpec> {
        InputW::new(self, 1)
    }
    #[doc = "Bit 2 - Represents the current value of input"]
    #[inline(always)]
    pub fn input2(&mut self) -> InputW<WkstnSpec> {
        InputW::new(self, 2)
    }
    #[doc = "Bit 3 - Represents the current value of input"]
    #[inline(always)]
    pub fn input3(&mut self) -> InputW<WkstnSpec> {
        InputW::new(self, 3)
    }
    #[doc = "Bit 4 - Represents the current value of input"]
    #[inline(always)]
    pub fn input4(&mut self) -> InputW<WkstnSpec> {
        InputW::new(self, 4)
    }
    #[doc = "Bit 5 - Represents the current value of input"]
    #[inline(always)]
    pub fn input5(&mut self) -> InputW<WkstnSpec> {
        InputW::new(self, 5)
    }
    #[doc = "Bit 6 - Represents the current value of input"]
    #[inline(always)]
    pub fn input6(&mut self) -> InputW<WkstnSpec> {
        InputW::new(self, 6)
    }
    #[doc = "Bit 7 - Represents the current value of input"]
    #[inline(always)]
    pub fn input7(&mut self) -> InputW<WkstnSpec> {
        InputW::new(self, 7)
    }
}
#[doc = "Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkstnSpec;
impl crate::RegisterSpec for WkstnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkstn::R`](R) reader structure"]
impl crate::Readable for WkstnSpec {}
#[doc = "`write(|w| ..)` method takes [`wkstn::W`](W) writer structure"]
impl crate::Writable for WkstnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKSTn%s to value 0"]
impl crate::Resettable for WkstnSpec {
    const RESET_VALUE: u8 = 0;
}
