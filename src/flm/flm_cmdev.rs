#[doc = "Register `FLM_CMDEV` reader"]
pub type R = crate::R<FlmCmdevSpec>;
#[doc = "Field `CMDEV(0-31)` reader - CMD Event %s"]
pub type CmdevR = crate::BitReader;
impl R {
    #[doc = "CMD Event (0-31)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CMDEV0` field.</div>"]
    #[inline(always)]
    pub fn cmdev(&self, n: u8) -> CmdevR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        CmdevR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "CMD Event (0-31)"]
    #[inline(always)]
    pub fn cmdev_iter(&self) -> impl Iterator<Item = CmdevR> + '_ {
        (0..32).map(move |n| CmdevR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - CMD Event 0"]
    #[inline(always)]
    pub fn cmdev0(&self) -> CmdevR {
        CmdevR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD Event 1"]
    #[inline(always)]
    pub fn cmdev1(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMD Event 2"]
    #[inline(always)]
    pub fn cmdev2(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMD Event 3"]
    #[inline(always)]
    pub fn cmdev3(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMD Event 4"]
    #[inline(always)]
    pub fn cmdev4(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMD Event 5"]
    #[inline(always)]
    pub fn cmdev5(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMD Event 6"]
    #[inline(always)]
    pub fn cmdev6(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMD Event 7"]
    #[inline(always)]
    pub fn cmdev7(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CMD Event 8"]
    #[inline(always)]
    pub fn cmdev8(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMD Event 9"]
    #[inline(always)]
    pub fn cmdev9(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CMD Event 10"]
    #[inline(always)]
    pub fn cmdev10(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CMD Event 11"]
    #[inline(always)]
    pub fn cmdev11(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CMD Event 12"]
    #[inline(always)]
    pub fn cmdev12(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CMD Event 13"]
    #[inline(always)]
    pub fn cmdev13(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CMD Event 14"]
    #[inline(always)]
    pub fn cmdev14(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CMD Event 15"]
    #[inline(always)]
    pub fn cmdev15(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CMD Event 16"]
    #[inline(always)]
    pub fn cmdev16(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CMD Event 17"]
    #[inline(always)]
    pub fn cmdev17(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CMD Event 18"]
    #[inline(always)]
    pub fn cmdev18(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CMD Event 19"]
    #[inline(always)]
    pub fn cmdev19(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CMD Event 20"]
    #[inline(always)]
    pub fn cmdev20(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CMD Event 21"]
    #[inline(always)]
    pub fn cmdev21(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CMD Event 22"]
    #[inline(always)]
    pub fn cmdev22(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CMD Event 23"]
    #[inline(always)]
    pub fn cmdev23(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CMD Event 24"]
    #[inline(always)]
    pub fn cmdev24(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CMD Event 25"]
    #[inline(always)]
    pub fn cmdev25(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CMD Event 26"]
    #[inline(always)]
    pub fn cmdev26(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CMD Event 27"]
    #[inline(always)]
    pub fn cmdev27(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CMD Event 28"]
    #[inline(always)]
    pub fn cmdev28(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CMD Event 29"]
    #[inline(always)]
    pub fn cmdev29(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CMD Event 30"]
    #[inline(always)]
    pub fn cmdev30(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CMD Event 31"]
    #[inline(always)]
    pub fn cmdev31(&self) -> CmdevR {
        CmdevR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_CMDEV")
            .field("cmdev0", &self.cmdev0())
            .field("cmdev1", &self.cmdev1())
            .field("cmdev2", &self.cmdev2())
            .field("cmdev3", &self.cmdev3())
            .field("cmdev4", &self.cmdev4())
            .field("cmdev5", &self.cmdev5())
            .field("cmdev6", &self.cmdev6())
            .field("cmdev7", &self.cmdev7())
            .field("cmdev8", &self.cmdev8())
            .field("cmdev9", &self.cmdev9())
            .field("cmdev10", &self.cmdev10())
            .field("cmdev11", &self.cmdev11())
            .field("cmdev12", &self.cmdev12())
            .field("cmdev13", &self.cmdev13())
            .field("cmdev14", &self.cmdev14())
            .field("cmdev15", &self.cmdev15())
            .field("cmdev16", &self.cmdev16())
            .field("cmdev17", &self.cmdev17())
            .field("cmdev18", &self.cmdev18())
            .field("cmdev19", &self.cmdev19())
            .field("cmdev20", &self.cmdev20())
            .field("cmdev21", &self.cmdev21())
            .field("cmdev22", &self.cmdev22())
            .field("cmdev23", &self.cmdev23())
            .field("cmdev24", &self.cmdev24())
            .field("cmdev25", &self.cmdev25())
            .field("cmdev26", &self.cmdev26())
            .field("cmdev27", &self.cmdev27())
            .field("cmdev28", &self.cmdev28())
            .field("cmdev29", &self.cmdev29())
            .field("cmdev30", &self.cmdev30())
            .field("cmdev31", &self.cmdev31())
            .finish()
    }
}
#[doc = "FLM CMD Event Register (FLM_CMDEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCmdevSpec;
impl crate::RegisterSpec for FlmCmdevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cmdev::R`](R) reader structure"]
impl crate::Readable for FlmCmdevSpec {}
#[doc = "`reset()` method sets FLM_CMDEV to value 0"]
impl crate::Resettable for FlmCmdevSpec {
    const RESET_VALUE: u32 = 0;
}
