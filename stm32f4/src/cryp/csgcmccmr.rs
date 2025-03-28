# [doc = "Register `CSGCMCCM%sR` reader"] pub type R = crate :: R < CsgcmccmrSpec > ; # [doc = "Register `CSGCMCCM%sR` writer"] pub type W = crate :: W < CsgcmccmrSpec > ; # [doc = "Field `CSGCMCCMR` reader - CSGCMCCM0R"] pub type CsgcmccmrR = crate :: FieldReader < u32 > ; # [doc = "Field `CSGCMCCMR` writer - CSGCMCCM0R"] pub type CsgcmccmrW < 'a , REG > = crate :: FieldWriter < 'a , REG , 32 , u32 > ; impl R { # [doc = "Bits 0:31 - CSGCMCCM0R"] # [inline (always)] pub fn csgcmccmr (& self) -> CsgcmccmrR { CsgcmccmrR :: new (self . bits) } } impl W { # [doc = "Bits 0:31 - CSGCMCCM0R"] # [inline (always)] pub fn csgcmccmr (& mut self) -> CsgcmccmrW < CsgcmccmrSpec > { CsgcmccmrW :: new (self , 0) } } # [doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcmccmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CsgcmccmrSpec ; impl crate :: RegisterSpec for CsgcmccmrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`csgcmccmr::R`](R) reader structure"] impl crate :: Readable for CsgcmccmrSpec { } # [doc = "`write(|w| ..)` method takes [`csgcmccmr::W`](W) writer structure"] impl crate :: Writable for CsgcmccmrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets CSGCMCCM%sR to value 0"] impl crate :: Resettable for CsgcmccmrSpec { }