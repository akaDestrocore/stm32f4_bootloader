# [doc = "Register `IER` reader"] pub type R = crate :: R < IerSpec > ; # [doc = "Register `IER` writer"] pub type W = crate :: W < IerSpec > ; # [doc = "TMEIE\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Tmeie { # [doc = "0: No interrupt when RQCPx bit is set"] Disabled = 0 , # [doc = "1: Interrupt generated when RQCPx bit is set"] Enabled = 1 , } impl From < Tmeie > for bool { # [inline (always)] fn from (variant : Tmeie) -> Self { variant as u8 != 0 } } # [doc = "Field `TMEIE` reader - TMEIE"] pub type TmeieR = crate :: BitReader < Tmeie > ; impl TmeieR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Tmeie { match self . bits { false => Tmeie :: Disabled , true => Tmeie :: Enabled , } } # [doc = "No interrupt when RQCPx bit is set"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Tmeie :: Disabled } # [doc = "Interrupt generated when RQCPx bit is set"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Tmeie :: Enabled } } # [doc = "Field `TMEIE` writer - TMEIE"] pub type TmeieW < 'a , REG > = crate :: BitWriter < 'a , REG , Tmeie > ; impl < 'a , REG > TmeieW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt when RQCPx bit is set"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Tmeie :: Disabled) } # [doc = "Interrupt generated when RQCPx bit is set"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Tmeie :: Enabled) } } # [doc = "FMPIE0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Fmpie0 { # [doc = "0: No interrupt generated when state of FMP\\[1:0\\] bits are not 00"] Disabled = 0 , # [doc = "1: Interrupt generated when state of FMP\\[1:0\\] bits are not 00b"] Enabled = 1 , } impl From < Fmpie0 > for bool { # [inline (always)] fn from (variant : Fmpie0) -> Self { variant as u8 != 0 } } # [doc = "Field `FMPIE0` reader - FMPIE0"] pub type Fmpie0R = crate :: BitReader < Fmpie0 > ; impl Fmpie0R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Fmpie0 { match self . bits { false => Fmpie0 :: Disabled , true => Fmpie0 :: Enabled , } } # [doc = "No interrupt generated when state of FMP\\[1:0\\] bits are not 00"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Fmpie0 :: Disabled } # [doc = "Interrupt generated when state of FMP\\[1:0\\] bits are not 00b"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Fmpie0 :: Enabled } } # [doc = "Field `FMPIE0` writer - FMPIE0"] pub type Fmpie0W < 'a , REG > = crate :: BitWriter < 'a , REG , Fmpie0 > ; impl < 'a , REG > Fmpie0W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt generated when state of FMP\\[1:0\\] bits are not 00"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Fmpie0 :: Disabled) } # [doc = "Interrupt generated when state of FMP\\[1:0\\] bits are not 00b"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Fmpie0 :: Enabled) } } # [doc = "FFIE0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Ffie0 { # [doc = "0: No interrupt when FULL bit is set"] Disabled = 0 , # [doc = "1: Interrupt generated when FULL bit is set"] Enabled = 1 , } impl From < Ffie0 > for bool { # [inline (always)] fn from (variant : Ffie0) -> Self { variant as u8 != 0 } } # [doc = "Field `FFIE0` reader - FFIE0"] pub type Ffie0R = crate :: BitReader < Ffie0 > ; impl Ffie0R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Ffie0 { match self . bits { false => Ffie0 :: Disabled , true => Ffie0 :: Enabled , } } # [doc = "No interrupt when FULL bit is set"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Ffie0 :: Disabled } # [doc = "Interrupt generated when FULL bit is set"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Ffie0 :: Enabled } } # [doc = "Field `FFIE0` writer - FFIE0"] pub type Ffie0W < 'a , REG > = crate :: BitWriter < 'a , REG , Ffie0 > ; impl < 'a , REG > Ffie0W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt when FULL bit is set"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Ffie0 :: Disabled) } # [doc = "Interrupt generated when FULL bit is set"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Ffie0 :: Enabled) } } # [doc = "FOVIE0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Fovie0 { # [doc = "0: No interrupt when FOVR bit is set"] Disabled = 0 , # [doc = "1: Interrupt generated when FOVR bit is set"] Enabled = 1 , } impl From < Fovie0 > for bool { # [inline (always)] fn from (variant : Fovie0) -> Self { variant as u8 != 0 } } # [doc = "Field `FOVIE0` reader - FOVIE0"] pub type Fovie0R = crate :: BitReader < Fovie0 > ; impl Fovie0R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Fovie0 { match self . bits { false => Fovie0 :: Disabled , true => Fovie0 :: Enabled , } } # [doc = "No interrupt when FOVR bit is set"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Fovie0 :: Disabled } # [doc = "Interrupt generated when FOVR bit is set"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Fovie0 :: Enabled } } # [doc = "Field `FOVIE0` writer - FOVIE0"] pub type Fovie0W < 'a , REG > = crate :: BitWriter < 'a , REG , Fovie0 > ; impl < 'a , REG > Fovie0W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt when FOVR bit is set"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Fovie0 :: Disabled) } # [doc = "Interrupt generated when FOVR bit is set"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Fovie0 :: Enabled) } } # [doc = "FMPIE1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Fmpie1 { # [doc = "0: No interrupt generated when state of FMP\\[1:0\\] bits are not 00b"] Disabled = 0 , # [doc = "1: Interrupt generated when state of FMP\\[1:0\\] bits are not 00b"] Enabled = 1 , } impl From < Fmpie1 > for bool { # [inline (always)] fn from (variant : Fmpie1) -> Self { variant as u8 != 0 } } # [doc = "Field `FMPIE1` reader - FMPIE1"] pub type Fmpie1R = crate :: BitReader < Fmpie1 > ; impl Fmpie1R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Fmpie1 { match self . bits { false => Fmpie1 :: Disabled , true => Fmpie1 :: Enabled , } } # [doc = "No interrupt generated when state of FMP\\[1:0\\] bits are not 00b"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Fmpie1 :: Disabled } # [doc = "Interrupt generated when state of FMP\\[1:0\\] bits are not 00b"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Fmpie1 :: Enabled } } # [doc = "Field `FMPIE1` writer - FMPIE1"] pub type Fmpie1W < 'a , REG > = crate :: BitWriter < 'a , REG , Fmpie1 > ; impl < 'a , REG > Fmpie1W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt generated when state of FMP\\[1:0\\] bits are not 00b"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Fmpie1 :: Disabled) } # [doc = "Interrupt generated when state of FMP\\[1:0\\] bits are not 00b"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Fmpie1 :: Enabled) } } # [doc = "FFIE1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Ffie1 { # [doc = "0: No interrupt when FULL bit is set"] Disabled = 0 , # [doc = "1: Interrupt generated when FULL bit is set"] Enabled = 1 , } impl From < Ffie1 > for bool { # [inline (always)] fn from (variant : Ffie1) -> Self { variant as u8 != 0 } } # [doc = "Field `FFIE1` reader - FFIE1"] pub type Ffie1R = crate :: BitReader < Ffie1 > ; impl Ffie1R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Ffie1 { match self . bits { false => Ffie1 :: Disabled , true => Ffie1 :: Enabled , } } # [doc = "No interrupt when FULL bit is set"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Ffie1 :: Disabled } # [doc = "Interrupt generated when FULL bit is set"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Ffie1 :: Enabled } } # [doc = "Field `FFIE1` writer - FFIE1"] pub type Ffie1W < 'a , REG > = crate :: BitWriter < 'a , REG , Ffie1 > ; impl < 'a , REG > Ffie1W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt when FULL bit is set"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Ffie1 :: Disabled) } # [doc = "Interrupt generated when FULL bit is set"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Ffie1 :: Enabled) } } # [doc = "FOVIE1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Fovie1 { # [doc = "0: No interrupt when FOVR is set"] Disabled = 0 , # [doc = "1: Interrupt generation when FOVR is set"] Enabled = 1 , } impl From < Fovie1 > for bool { # [inline (always)] fn from (variant : Fovie1) -> Self { variant as u8 != 0 } } # [doc = "Field `FOVIE1` reader - FOVIE1"] pub type Fovie1R = crate :: BitReader < Fovie1 > ; impl Fovie1R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Fovie1 { match self . bits { false => Fovie1 :: Disabled , true => Fovie1 :: Enabled , } } # [doc = "No interrupt when FOVR is set"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Fovie1 :: Disabled } # [doc = "Interrupt generation when FOVR is set"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Fovie1 :: Enabled } } # [doc = "Field `FOVIE1` writer - FOVIE1"] pub type Fovie1W < 'a , REG > = crate :: BitWriter < 'a , REG , Fovie1 > ; impl < 'a , REG > Fovie1W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt when FOVR is set"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Fovie1 :: Disabled) } # [doc = "Interrupt generation when FOVR is set"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Fovie1 :: Enabled) } } # [doc = "EWGIE\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Ewgie { # [doc = "0: ERRI bit will not be set when EWGF is set"] Disabled = 0 , # [doc = "1: ERRI bit will be set when EWGF is set"] Enabled = 1 , } impl From < Ewgie > for bool { # [inline (always)] fn from (variant : Ewgie) -> Self { variant as u8 != 0 } } # [doc = "Field `EWGIE` reader - EWGIE"] pub type EwgieR = crate :: BitReader < Ewgie > ; impl EwgieR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Ewgie { match self . bits { false => Ewgie :: Disabled , true => Ewgie :: Enabled , } } # [doc = "ERRI bit will not be set when EWGF is set"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Ewgie :: Disabled } # [doc = "ERRI bit will be set when EWGF is set"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Ewgie :: Enabled } } # [doc = "Field `EWGIE` writer - EWGIE"] pub type EwgieW < 'a , REG > = crate :: BitWriter < 'a , REG , Ewgie > ; impl < 'a , REG > EwgieW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ERRI bit will not be set when EWGF is set"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Ewgie :: Disabled) } # [doc = "ERRI bit will be set when EWGF is set"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Ewgie :: Enabled) } } # [doc = "EPVIE\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Epvie { # [doc = "0: ERRI bit will not be set when EPVF is set"] Disabled = 0 , # [doc = "1: ERRI bit will be set when EPVF is set"] Enabled = 1 , } impl From < Epvie > for bool { # [inline (always)] fn from (variant : Epvie) -> Self { variant as u8 != 0 } } # [doc = "Field `EPVIE` reader - EPVIE"] pub type EpvieR = crate :: BitReader < Epvie > ; impl EpvieR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Epvie { match self . bits { false => Epvie :: Disabled , true => Epvie :: Enabled , } } # [doc = "ERRI bit will not be set when EPVF is set"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Epvie :: Disabled } # [doc = "ERRI bit will be set when EPVF is set"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Epvie :: Enabled } } # [doc = "Field `EPVIE` writer - EPVIE"] pub type EpvieW < 'a , REG > = crate :: BitWriter < 'a , REG , Epvie > ; impl < 'a , REG > EpvieW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ERRI bit will not be set when EPVF is set"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Epvie :: Disabled) } # [doc = "ERRI bit will be set when EPVF is set"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Epvie :: Enabled) } } # [doc = "BOFIE\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bofie { # [doc = "0: ERRI bit will not be set when BOFF is set"] Disabled = 0 , # [doc = "1: ERRI bit will be set when BOFF is set"] Enabled = 1 , } impl From < Bofie > for bool { # [inline (always)] fn from (variant : Bofie) -> Self { variant as u8 != 0 } } # [doc = "Field `BOFIE` reader - BOFIE"] pub type BofieR = crate :: BitReader < Bofie > ; impl BofieR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bofie { match self . bits { false => Bofie :: Disabled , true => Bofie :: Enabled , } } # [doc = "ERRI bit will not be set when BOFF is set"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Bofie :: Disabled } # [doc = "ERRI bit will be set when BOFF is set"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Bofie :: Enabled } } # [doc = "Field `BOFIE` writer - BOFIE"] pub type BofieW < 'a , REG > = crate :: BitWriter < 'a , REG , Bofie > ; impl < 'a , REG > BofieW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ERRI bit will not be set when BOFF is set"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Bofie :: Disabled) } # [doc = "ERRI bit will be set when BOFF is set"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Bofie :: Enabled) } } # [doc = "LECIE\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Lecie { # [doc = "0: ERRI bit will not be set when the error code in LEC\\[2:0\\] is set by hardware on error detection"] Disabled = 0 , # [doc = "1: ERRI bit will be set when the error code in LEC\\[2:0\\] is set by hardware on error detection"] Enabled = 1 , } impl From < Lecie > for bool { # [inline (always)] fn from (variant : Lecie) -> Self { variant as u8 != 0 } } # [doc = "Field `LECIE` reader - LECIE"] pub type LecieR = crate :: BitReader < Lecie > ; impl LecieR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Lecie { match self . bits { false => Lecie :: Disabled , true => Lecie :: Enabled , } } # [doc = "ERRI bit will not be set when the error code in LEC\\[2:0\\] is set by hardware on error detection"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Lecie :: Disabled } # [doc = "ERRI bit will be set when the error code in LEC\\[2:0\\] is set by hardware on error detection"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Lecie :: Enabled } } # [doc = "Field `LECIE` writer - LECIE"] pub type LecieW < 'a , REG > = crate :: BitWriter < 'a , REG , Lecie > ; impl < 'a , REG > LecieW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ERRI bit will not be set when the error code in LEC\\[2:0\\] is set by hardware on error detection"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Lecie :: Disabled) } # [doc = "ERRI bit will be set when the error code in LEC\\[2:0\\] is set by hardware on error detection"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Lecie :: Enabled) } } # [doc = "ERRIE\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Errie { # [doc = "0: No interrupt will be generated when an error condition is pending in the CAN_ESR"] Disabled = 0 , # [doc = "1: An interrupt will be generation when an error condition is pending in the CAN_ESR"] Enabled = 1 , } impl From < Errie > for bool { # [inline (always)] fn from (variant : Errie) -> Self { variant as u8 != 0 } } # [doc = "Field `ERRIE` reader - ERRIE"] pub type ErrieR = crate :: BitReader < Errie > ; impl ErrieR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Errie { match self . bits { false => Errie :: Disabled , true => Errie :: Enabled , } } # [doc = "No interrupt will be generated when an error condition is pending in the CAN_ESR"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Errie :: Disabled } # [doc = "An interrupt will be generation when an error condition is pending in the CAN_ESR"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Errie :: Enabled } } # [doc = "Field `ERRIE` writer - ERRIE"] pub type ErrieW < 'a , REG > = crate :: BitWriter < 'a , REG , Errie > ; impl < 'a , REG > ErrieW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt will be generated when an error condition is pending in the CAN_ESR"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Errie :: Disabled) } # [doc = "An interrupt will be generation when an error condition is pending in the CAN_ESR"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Errie :: Enabled) } } # [doc = "WKUIE\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Wkuie { # [doc = "0: No interrupt when WKUI is set"] Disabled = 0 , # [doc = "1: Interrupt generated when WKUI bit is set"] Enabled = 1 , } impl From < Wkuie > for bool { # [inline (always)] fn from (variant : Wkuie) -> Self { variant as u8 != 0 } } # [doc = "Field `WKUIE` reader - WKUIE"] pub type WkuieR = crate :: BitReader < Wkuie > ; impl WkuieR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Wkuie { match self . bits { false => Wkuie :: Disabled , true => Wkuie :: Enabled , } } # [doc = "No interrupt when WKUI is set"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Wkuie :: Disabled } # [doc = "Interrupt generated when WKUI bit is set"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Wkuie :: Enabled } } # [doc = "Field `WKUIE` writer - WKUIE"] pub type WkuieW < 'a , REG > = crate :: BitWriter < 'a , REG , Wkuie > ; impl < 'a , REG > WkuieW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt when WKUI is set"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Wkuie :: Disabled) } # [doc = "Interrupt generated when WKUI bit is set"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Wkuie :: Enabled) } } # [doc = "SLKIE\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Slkie { # [doc = "0: No interrupt when SLAKI bit is set"] Disabled = 0 , # [doc = "1: Interrupt generated when SLAKI bit is set"] Enabled = 1 , } impl From < Slkie > for bool { # [inline (always)] fn from (variant : Slkie) -> Self { variant as u8 != 0 } } # [doc = "Field `SLKIE` reader - SLKIE"] pub type SlkieR = crate :: BitReader < Slkie > ; impl SlkieR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Slkie { match self . bits { false => Slkie :: Disabled , true => Slkie :: Enabled , } } # [doc = "No interrupt when SLAKI bit is set"] # [inline (always)] pub fn is_disabled (& self) -> bool { * self == Slkie :: Disabled } # [doc = "Interrupt generated when SLAKI bit is set"] # [inline (always)] pub fn is_enabled (& self) -> bool { * self == Slkie :: Enabled } } # [doc = "Field `SLKIE` writer - SLKIE"] pub type SlkieW < 'a , REG > = crate :: BitWriter < 'a , REG , Slkie > ; impl < 'a , REG > SlkieW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt when SLAKI bit is set"] # [inline (always)] pub fn disabled (self) -> & 'a mut crate :: W < REG > { self . variant (Slkie :: Disabled) } # [doc = "Interrupt generated when SLAKI bit is set"] # [inline (always)] pub fn enabled (self) -> & 'a mut crate :: W < REG > { self . variant (Slkie :: Enabled) } } impl R { # [doc = "Bit 0 - TMEIE"] # [inline (always)] pub fn tmeie (& self) -> TmeieR { TmeieR :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - FMPIE0"] # [inline (always)] pub fn fmpie0 (& self) -> Fmpie0R { Fmpie0R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - FFIE0"] # [inline (always)] pub fn ffie0 (& self) -> Ffie0R { Ffie0R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - FOVIE0"] # [inline (always)] pub fn fovie0 (& self) -> Fovie0R { Fovie0R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - FMPIE1"] # [inline (always)] pub fn fmpie1 (& self) -> Fmpie1R { Fmpie1R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - FFIE1"] # [inline (always)] pub fn ffie1 (& self) -> Ffie1R { Ffie1R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - FOVIE1"] # [inline (always)] pub fn fovie1 (& self) -> Fovie1R { Fovie1R :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 8 - EWGIE"] # [inline (always)] pub fn ewgie (& self) -> EwgieR { EwgieR :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - EPVIE"] # [inline (always)] pub fn epvie (& self) -> EpvieR { EpvieR :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - BOFIE"] # [inline (always)] pub fn bofie (& self) -> BofieR { BofieR :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - LECIE"] # [inline (always)] pub fn lecie (& self) -> LecieR { LecieR :: new (((self . bits >> 11) & 1) != 0) } # [doc = "Bit 15 - ERRIE"] # [inline (always)] pub fn errie (& self) -> ErrieR { ErrieR :: new (((self . bits >> 15) & 1) != 0) } # [doc = "Bit 16 - WKUIE"] # [inline (always)] pub fn wkuie (& self) -> WkuieR { WkuieR :: new (((self . bits >> 16) & 1) != 0) } # [doc = "Bit 17 - SLKIE"] # [inline (always)] pub fn slkie (& self) -> SlkieR { SlkieR :: new (((self . bits >> 17) & 1) != 0) } } impl W { # [doc = "Bit 0 - TMEIE"] # [inline (always)] pub fn tmeie (& mut self) -> TmeieW < IerSpec > { TmeieW :: new (self , 0) } # [doc = "Bit 1 - FMPIE0"] # [inline (always)] pub fn fmpie0 (& mut self) -> Fmpie0W < IerSpec > { Fmpie0W :: new (self , 1) } # [doc = "Bit 2 - FFIE0"] # [inline (always)] pub fn ffie0 (& mut self) -> Ffie0W < IerSpec > { Ffie0W :: new (self , 2) } # [doc = "Bit 3 - FOVIE0"] # [inline (always)] pub fn fovie0 (& mut self) -> Fovie0W < IerSpec > { Fovie0W :: new (self , 3) } # [doc = "Bit 4 - FMPIE1"] # [inline (always)] pub fn fmpie1 (& mut self) -> Fmpie1W < IerSpec > { Fmpie1W :: new (self , 4) } # [doc = "Bit 5 - FFIE1"] # [inline (always)] pub fn ffie1 (& mut self) -> Ffie1W < IerSpec > { Ffie1W :: new (self , 5) } # [doc = "Bit 6 - FOVIE1"] # [inline (always)] pub fn fovie1 (& mut self) -> Fovie1W < IerSpec > { Fovie1W :: new (self , 6) } # [doc = "Bit 8 - EWGIE"] # [inline (always)] pub fn ewgie (& mut self) -> EwgieW < IerSpec > { EwgieW :: new (self , 8) } # [doc = "Bit 9 - EPVIE"] # [inline (always)] pub fn epvie (& mut self) -> EpvieW < IerSpec > { EpvieW :: new (self , 9) } # [doc = "Bit 10 - BOFIE"] # [inline (always)] pub fn bofie (& mut self) -> BofieW < IerSpec > { BofieW :: new (self , 10) } # [doc = "Bit 11 - LECIE"] # [inline (always)] pub fn lecie (& mut self) -> LecieW < IerSpec > { LecieW :: new (self , 11) } # [doc = "Bit 15 - ERRIE"] # [inline (always)] pub fn errie (& mut self) -> ErrieW < IerSpec > { ErrieW :: new (self , 15) } # [doc = "Bit 16 - WKUIE"] # [inline (always)] pub fn wkuie (& mut self) -> WkuieW < IerSpec > { WkuieW :: new (self , 16) } # [doc = "Bit 17 - SLKIE"] # [inline (always)] pub fn slkie (& mut self) -> SlkieW < IerSpec > { SlkieW :: new (self , 17) } } # [doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct IerSpec ; impl crate :: RegisterSpec for IerSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`ier::R`](R) reader structure"] impl crate :: Readable for IerSpec { } # [doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"] impl crate :: Writable for IerSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets IER to value 0"] impl crate :: Resettable for IerSpec { }