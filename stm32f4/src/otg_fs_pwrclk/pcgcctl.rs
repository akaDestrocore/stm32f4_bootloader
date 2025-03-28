# [doc = "Register `PCGCCTL` reader"] pub type R = crate :: R < PcgcctlSpec > ; # [doc = "Register `PCGCCTL` writer"] pub type W = crate :: W < PcgcctlSpec > ; # [doc = "Field `STPPCLK` reader - Stop PHY clock"] pub type StppclkR = crate :: BitReader ; # [doc = "Field `STPPCLK` writer - Stop PHY clock"] pub type StppclkW < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `GATEHCLK` reader - Gate HCLK"] pub type GatehclkR = crate :: BitReader ; # [doc = "Field `GATEHCLK` writer - Gate HCLK"] pub type GatehclkW < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `PHYSUSP` reader - PHY Suspended"] pub type PhysuspR = crate :: BitReader ; # [doc = "Field `PHYSUSP` writer - PHY Suspended"] pub type PhysuspW < 'a , REG > = crate :: BitWriter < 'a , REG > ; impl R { # [doc = "Bit 0 - Stop PHY clock"] # [inline (always)] pub fn stppclk (& self) -> StppclkR { StppclkR :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Gate HCLK"] # [inline (always)] pub fn gatehclk (& self) -> GatehclkR { GatehclkR :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 4 - PHY Suspended"] # [inline (always)] pub fn physusp (& self) -> PhysuspR { PhysuspR :: new (((self . bits >> 4) & 1) != 0) } } impl W { # [doc = "Bit 0 - Stop PHY clock"] # [inline (always)] pub fn stppclk (& mut self) -> StppclkW < PcgcctlSpec > { StppclkW :: new (self , 0) } # [doc = "Bit 1 - Gate HCLK"] # [inline (always)] pub fn gatehclk (& mut self) -> GatehclkW < PcgcctlSpec > { GatehclkW :: new (self , 1) } # [doc = "Bit 4 - PHY Suspended"] # [inline (always)] pub fn physusp (& mut self) -> PhysuspW < PcgcctlSpec > { PhysuspW :: new (self , 4) } } # [doc = "OTG_FS power and clock gating control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct PcgcctlSpec ; impl crate :: RegisterSpec for PcgcctlSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"] impl crate :: Readable for PcgcctlSpec { } # [doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"] impl crate :: Writable for PcgcctlSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets PCGCCTL to value 0"] impl crate :: Resettable for PcgcctlSpec { }