# [doc = "Register `DIEPTXF%s` reader"] pub type R = crate :: R < DieptxfSpec > ; # [doc = "Register `DIEPTXF%s` writer"] pub type W = crate :: W < DieptxfSpec > ; # [doc = "Field `INEPTXSA` reader - IN endpoint FIFO2 transmit RAM start address"] pub type IneptxsaR = crate :: FieldReader < u16 > ; # [doc = "Field `INEPTXSA` writer - IN endpoint FIFO2 transmit RAM start address"] pub type IneptxsaW < 'a , REG > = crate :: FieldWriter < 'a , REG , 16 , u16 > ; # [doc = "Field `INEPTXFD` reader - IN endpoint TxFIFO depth"] pub type IneptxfdR = crate :: FieldReader < u16 > ; # [doc = "Field `INEPTXFD` writer - IN endpoint TxFIFO depth"] pub type IneptxfdW < 'a , REG > = crate :: FieldWriter < 'a , REG , 16 , u16 > ; impl R { # [doc = "Bits 0:15 - IN endpoint FIFO2 transmit RAM start address"] # [inline (always)] pub fn ineptxsa (& self) -> IneptxsaR { IneptxsaR :: new ((self . bits & 0xffff) as u16) } # [doc = "Bits 16:31 - IN endpoint TxFIFO depth"] # [inline (always)] pub fn ineptxfd (& self) -> IneptxfdR { IneptxfdR :: new (((self . bits >> 16) & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - IN endpoint FIFO2 transmit RAM start address"] # [inline (always)] pub fn ineptxsa (& mut self) -> IneptxsaW < DieptxfSpec > { IneptxsaW :: new (self , 0) } # [doc = "Bits 16:31 - IN endpoint TxFIFO depth"] # [inline (always)] pub fn ineptxfd (& mut self) -> IneptxfdW < DieptxfSpec > { IneptxfdW :: new (self , 16) } } # [doc = "OTF_FS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct DieptxfSpec ; impl crate :: RegisterSpec for DieptxfSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`dieptxf::R`](R) reader structure"] impl crate :: Readable for DieptxfSpec { } # [doc = "`write(|w| ..)` method takes [`dieptxf::W`](W) writer structure"] impl crate :: Writable for DieptxfSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets DIEPTXF%s to value 0x0200_0400"] impl crate :: Resettable for DieptxfSpec { const RESET_VALUE : u32 = 0x0200_0400 ; }