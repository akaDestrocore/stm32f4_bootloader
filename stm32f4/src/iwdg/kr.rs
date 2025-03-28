# [doc = "Register `KR` writer"] pub type W = crate :: W < KrSpec > ; # [doc = "Key value (write only, read 0000h)\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u16)] pub enum Key { # [doc = "21845: Enable access to PR, RLR and WINR registers"] Unlock = 21845 , # [doc = "43690: Feed watchdog with RLR register value"] Feed = 43690 , # [doc = "52428: Start the watchdog"] Start = 52428 , } impl From < Key > for u16 { # [inline (always)] fn from (variant : Key) -> Self { variant as _ } } impl crate :: FieldSpec for Key { type Ux = u16 ; } impl crate :: IsEnum for Key { } # [doc = "Field `KEY` writer - Key value (write only, read 0000h)"] pub type KeyW < 'a , REG > = crate :: FieldWriter < 'a , REG , 16 , Key > ; impl < 'a , REG > KeyW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u16 > { # [doc = "Enable access to PR, RLR and WINR registers"] # [inline (always)] pub fn unlock (self) -> & 'a mut crate :: W < REG > { self . variant (Key :: Unlock) } # [doc = "Feed watchdog with RLR register value"] # [inline (always)] pub fn feed (self) -> & 'a mut crate :: W < REG > { self . variant (Key :: Feed) } # [doc = "Start the watchdog"] # [inline (always)] pub fn start (self) -> & 'a mut crate :: W < REG > { self . variant (Key :: Start) } } impl W { # [doc = "Bits 0:15 - Key value (write only, read 0000h)"] # [inline (always)] pub fn key (& mut self) -> KeyW < KrSpec > { KeyW :: new (self , 0) } } # [doc = "Key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct KrSpec ; impl crate :: RegisterSpec for KrSpec { type Ux = u16 ; } # [doc = "`write(|w| ..)` method takes [`kr::W`](W) writer structure"] impl crate :: Writable for KrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets KR to value 0"] impl crate :: Resettable for KrSpec { }