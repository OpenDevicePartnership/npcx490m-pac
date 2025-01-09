#[doc = "Register `WKPNDn%s` reader"]
pub type R = crate::R<WkpndnSpec>;
#[doc = "Register `WKPNDn%s` writer"]
pub type W = crate::W<WkpndnSpec>;
#[doc = "Indicates that a valid trigger event occurred\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventPending {
    #[doc = "0: No wake-up event occurred"]
    NotPending = 0,
    #[doc = "1: Wake-up event is pending"]
    Pending = 1,
}
impl From<EventPending> for bool {
    #[inline(always)]
    fn from(variant: EventPending) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT(0-7)` reader - Indicates that a valid trigger event occurred"]
pub type InputR = crate::BitReader<EventPending>;
impl InputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventPending {
        match self.bits {
            false => EventPending::NotPending,
            true => EventPending::Pending,
        }
    }
    #[doc = "No wake-up event occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == EventPending::NotPending
    }
    #[doc = "Wake-up event is pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EventPending::Pending
    }
}
#[doc = "Field `INPUT(0-7)` writer - Indicates that a valid trigger event occurred"]
pub type InputW<'a, REG> = crate::BitWriter<'a, REG, EventPending>;
impl<'a, REG> InputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No wake-up event occurred"]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut crate::W<REG> {
        self.variant(EventPending::NotPending)
    }
    #[doc = "Wake-up event is pending"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(EventPending::Pending)
    }
}
impl R {
    #[doc = "Indicates that a valid trigger event occurred"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&self, n: u8) -> InputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input_iter(&self) -> impl Iterator<Item = InputR> + '_ {
        (0..8).map(move |n| InputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input0(&self) -> InputR {
        InputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input1(&self) -> InputR {
        InputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input2(&self) -> InputR {
        InputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input3(&self) -> InputR {
        InputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input4(&self) -> InputR {
        InputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input5(&self) -> InputR {
        InputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input6(&self) -> InputR {
        InputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input7(&self) -> InputR {
        InputR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKPNDn")
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
    #[doc = "Indicates that a valid trigger event occurred"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&mut self, n: u8) -> InputW<WkpndnSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputW::new(self, n)
    }
    #[doc = "Bit 0 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input0(&mut self) -> InputW<WkpndnSpec> {
        InputW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input1(&mut self) -> InputW<WkpndnSpec> {
        InputW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input2(&mut self) -> InputW<WkpndnSpec> {
        InputW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input3(&mut self) -> InputW<WkpndnSpec> {
        InputW::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input4(&mut self) -> InputW<WkpndnSpec> {
        InputW::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input5(&mut self) -> InputW<WkpndnSpec> {
        InputW::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input6(&mut self) -> InputW<WkpndnSpec> {
        InputW::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates that a valid trigger event occurred"]
    #[inline(always)]
    pub fn input7(&mut self) -> InputW<WkpndnSpec> {
        InputW::new(self, 7)
    }
}
#[doc = "Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkpndnSpec;
impl crate::RegisterSpec for WkpndnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkpndn::R`](R) reader structure"]
impl crate::Readable for WkpndnSpec {}
#[doc = "`write(|w| ..)` method takes [`wkpndn::W`](W) writer structure"]
impl crate::Writable for WkpndnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKPNDn%s to value 0"]
impl crate::Resettable for WkpndnSpec {
    const RESET_VALUE: u8 = 0;
}
