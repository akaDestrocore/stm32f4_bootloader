# [doc = "Register `DIEPTXF0` reader"] pub type R = crate :: R < Dieptxf0Spec > ; # [doc = "Register `DIEPTXF0` writer"] pub type W = crate :: W < Dieptxf0Spec > ; # [doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"] pub type Tx0fsaR = crate :: FieldReader < u16 > ; # [doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"] pub type Tx0fsaW < 'a , REG > = crate :: FieldWriter < 'a , REG , 16 , u16 > ; # [doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"] pub type Tx0fdR = crate :: FieldReader < u16 > ; # [doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"] pub type Tx0fdW < 'a , REG > = crate :: FieldWriter < 'a , REG , 16 , u16 > ; impl R { # [doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"] # [inline (always)] pub fn tx0fsa (& self) -> Tx0fsaR { Tx0fsaR :: new ((self . bits & 0xffff) as u16) } # [doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"] # [inline (always)] pub fn tx0fd (& self) -> Tx0fdR { Tx0fdR :: new (((self . bits >> 16) & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"] # [inline (always)] pub fn tx0fsa (& mut self) -> Tx0fsaW < Dieptxf0Spec > { Tx0fsaW :: new (self , 0) } # [doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"] # [inline (always)] pub fn tx0fd (& mut self) -> Tx0fdW < Dieptxf0Spec > { Tx0fdW :: new (self , 16) } } # [doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct Dieptxf0Spec ; impl crate :: RegisterSpec for Dieptxf0Spec { type Ux = u32 ; } # [doc = "`read()` method returns [`dieptxf0::R`](R) reader structure"] impl crate :: Readable for Dieptxf0Spec { } # [doc = "`write(|w| ..)` method takes [`dieptxf0::W`](W) writer structure"] impl crate :: Writable for Dieptxf0Spec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets DIEPTXF0 to value 0x0200"] impl crate :: Resettable for Dieptxf0Spec { const RESET_VALUE : u32 = 0x0200 ; }