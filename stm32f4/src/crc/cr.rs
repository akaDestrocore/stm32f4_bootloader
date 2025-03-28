# [doc = "Register `CR` writer"] pub type W = crate :: W < CrSpec > ; # [doc = "Control regidter\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Resetw { # [doc = "1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF"] Reset = 1 , } impl From < Resetw > for bool { # [inline (always)] fn from (variant : Resetw) -> Self { variant as u8 != 0 } } # [doc = "Field `RESET` writer - Control regidter"] pub type ResetW < 'a , REG > = crate :: BitWriter < 'a , REG , Resetw > ; impl < 'a , REG > ResetW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF"] # [inline (always)] pub fn reset (self) -> & 'a mut crate :: W < REG > { self . variant (Resetw :: Reset) } } impl W { # [doc = "Bit 0 - Control regidter"] # [inline (always)] pub fn reset (& mut self) -> ResetW < CrSpec > { ResetW :: new (self , 0) } } # [doc = "Control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CrSpec ; impl crate :: RegisterSpec for CrSpec { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"] impl crate :: Writable for CrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets CR to value 0"] impl crate :: Resettable for CrSpec { }