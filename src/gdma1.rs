#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    channel: [Channel; 2],
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - Channel%s"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - Channel%s"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        self.channel.iter()
    }
}
#[doc = "Channel%s"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "Channel%s"]
pub mod channel;
