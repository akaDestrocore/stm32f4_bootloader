# [doc = "Register `CTL` reader"] pub type R = crate :: R < CtlSpec > ; # [doc = "Register `CTL` writer"] pub type W = crate :: W < CtlSpec > ; # [doc = "Field `MPSIZ` reader - Maximum packet size"] pub type MpsizR = crate :: FieldReader ; # [doc = "Field `MPSIZ` writer - Maximum packet size"] pub type MpsizW < 'a , REG > = crate :: FieldWriter < 'a , REG , 2 > ; # [doc = "Field `USBAEP` reader - USB active endpoint"] pub type UsbaepR = crate :: BitReader ; # [doc = "Field `NAKSTS` reader - NAK status"] pub type NakstsR = crate :: BitReader ; # [doc = "Field `EPTYP` reader - Endpoint type"] pub type EptypR = crate :: FieldReader ; # [doc = "Field `STALL` reader - STALL handshake"] pub type StallR = crate :: BitReader ; # [doc = "Field `STALL` writer - STALL handshake"] pub type StallW < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `TXFNUM` reader - TxFIFO number"] pub type TxfnumR = crate :: FieldReader ; # [doc = "Field `TXFNUM` writer - TxFIFO number"] pub type TxfnumW < 'a , REG > = crate :: FieldWriter < 'a , REG , 4 > ; # [doc = "Field `CNAK` writer - Clear NAK"] pub type CnakW < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `SNAK` writer - Set NAK"] pub type SnakW < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `EPDIS` reader - Endpoint disable"] pub type EpdisR = crate :: BitReader ; # [doc = "Field `EPENA` reader - Endpoint enable"] pub type EpenaR = crate :: BitReader ; # [doc = "Field `EPENA` writer - Endpoint enable"] pub type EpenaW < 'a , REG > = crate :: BitWriter < 'a , REG > ; impl R { # [doc = "Bits 0:1 - Maximum packet size"] # [inline (always)] pub fn mpsiz (& self) -> MpsizR { MpsizR :: new ((self . bits & 3) as u8) } # [doc = "Bit 15 - USB active endpoint"] # [inline (always)] pub fn usbaep (& self) -> UsbaepR { UsbaepR :: new (((self . bits >> 15) & 1) != 0) } # [doc = "Bit 17 - NAK status"] # [inline (always)] pub fn naksts (& self) -> NakstsR { NakstsR :: new (((self . bits >> 17) & 1) != 0) } # [doc = "Bits 18:19 - Endpoint type"] # [inline (always)] pub fn eptyp (& self) -> EptypR { EptypR :: new (((self . bits >> 18) & 3) as u8) } # [doc = "Bit 21 - STALL handshake"] # [inline (always)] pub fn stall (& self) -> StallR { StallR :: new (((self . bits >> 21) & 1) != 0) } # [doc = "Bits 22:25 - TxFIFO number"] # [inline (always)] pub fn txfnum (& self) -> TxfnumR { TxfnumR :: new (((self . bits >> 22) & 0x0f) as u8) } # [doc = "Bit 30 - Endpoint disable"] # [inline (always)] pub fn epdis (& self) -> EpdisR { EpdisR :: new (((self . bits >> 30) & 1) != 0) } # [doc = "Bit 31 - Endpoint enable"] # [inline (always)] pub fn epena (& self) -> EpenaR { EpenaR :: new (((self . bits >> 31) & 1) != 0) } } impl W { # [doc = "Bits 0:1 - Maximum packet size"] # [inline (always)] pub fn mpsiz (& mut self) -> MpsizW < CtlSpec > { MpsizW :: new (self , 0) } # [doc = "Bit 21 - STALL handshake"] # [inline (always)] pub fn stall (& mut self) -> StallW < CtlSpec > { StallW :: new (self , 21) } # [doc = "Bits 22:25 - TxFIFO number"] # [inline (always)] pub fn txfnum (& mut self) -> TxfnumW < CtlSpec > { TxfnumW :: new (self , 22) } # [doc = "Bit 26 - Clear NAK"] # [inline (always)] pub fn cnak (& mut self) -> CnakW < CtlSpec > { CnakW :: new (self , 26) } # [doc = "Bit 27 - Set NAK"] # [inline (always)] pub fn snak (& mut self) -> SnakW < CtlSpec > { SnakW :: new (self , 27) } # [doc = "Bit 31 - Endpoint enable"] # [inline (always)] pub fn epena (& mut self) -> EpenaW < CtlSpec > { EpenaW :: new (self , 31) } } # [doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CtlSpec ; impl crate :: RegisterSpec for CtlSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`ctl::R`](R) reader structure"] impl crate :: Readable for CtlSpec { } # [doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"] impl crate :: Writable for CtlSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets CTL to value 0"] impl crate :: Resettable for CtlSpec { }