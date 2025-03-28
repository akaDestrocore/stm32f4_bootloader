# [doc = "Register `ISR` reader"] pub type R = crate :: R < IsrSpec > ; # [doc = "Register `ISR` writer"] pub type W = crate :: W < IsrSpec > ; # [doc = "Alarm %s write flag\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Alrawfr { # [doc = "0: Alarm update not allowed"] UpdateNotAllowed = 0 , # [doc = "1: Alarm update allowed"] UpdateAllowed = 1 , } impl From < Alrawfr > for bool { # [inline (always)] fn from (variant : Alrawfr) -> Self { variant as u8 != 0 } } # [doc = "Field `ALRWF(A,B)` reader - Alarm %s write flag"] pub type AlrwfR = crate :: BitReader < Alrawfr > ; impl AlrwfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Alrawfr { match self . bits { false => Alrawfr :: UpdateNotAllowed , true => Alrawfr :: UpdateAllowed , } } # [doc = "Alarm update not allowed"] # [inline (always)] pub fn is_update_not_allowed (& self) -> bool { * self == Alrawfr :: UpdateNotAllowed } # [doc = "Alarm update allowed"] # [inline (always)] pub fn is_update_allowed (& self) -> bool { * self == Alrawfr :: UpdateAllowed } } # [doc = "Wakeup timer write flag\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Wutwfr { # [doc = "0: Wakeup timer configuration update not allowed"] UpdateNotAllowed = 0 , # [doc = "1: Wakeup timer configuration update allowed"] UpdateAllowed = 1 , } impl From < Wutwfr > for bool { # [inline (always)] fn from (variant : Wutwfr) -> Self { variant as u8 != 0 } } # [doc = "Field `WUTWF` reader - Wakeup timer write flag"] pub type WutwfR = crate :: BitReader < Wutwfr > ; impl WutwfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Wutwfr { match self . bits { false => Wutwfr :: UpdateNotAllowed , true => Wutwfr :: UpdateAllowed , } } # [doc = "Wakeup timer configuration update not allowed"] # [inline (always)] pub fn is_update_not_allowed (& self) -> bool { * self == Wutwfr :: UpdateNotAllowed } # [doc = "Wakeup timer configuration update allowed"] # [inline (always)] pub fn is_update_allowed (& self) -> bool { * self == Wutwfr :: UpdateAllowed } } # [doc = "Shift operation pending\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Shpfr { # [doc = "0: No shift operation is pending"] NoShiftPending = 0 , # [doc = "1: A shift operation is pending"] ShiftPending = 1 , } impl From < Shpfr > for bool { # [inline (always)] fn from (variant : Shpfr) -> Self { variant as u8 != 0 } } # [doc = "Field `SHPF` reader - Shift operation pending"] pub type ShpfR = crate :: BitReader < Shpfr > ; impl ShpfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Shpfr { match self . bits { false => Shpfr :: NoShiftPending , true => Shpfr :: ShiftPending , } } # [doc = "No shift operation is pending"] # [inline (always)] pub fn is_no_shift_pending (& self) -> bool { * self == Shpfr :: NoShiftPending } # [doc = "A shift operation is pending"] # [inline (always)] pub fn is_shift_pending (& self) -> bool { * self == Shpfr :: ShiftPending } } # [doc = "Field `SHPF` writer - Shift operation pending"] pub type ShpfW < 'a , REG > = crate :: BitWriter < 'a , REG , Shpfr > ; impl < 'a , REG > ShpfW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No shift operation is pending"] # [inline (always)] pub fn no_shift_pending (self) -> & 'a mut crate :: W < REG > { self . variant (Shpfr :: NoShiftPending) } # [doc = "A shift operation is pending"] # [inline (always)] pub fn shift_pending (self) -> & 'a mut crate :: W < REG > { self . variant (Shpfr :: ShiftPending) } } # [doc = "Initialization status flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Initsr { # [doc = "0: Calendar has not been initialized"] NotInitalized = 0 , # [doc = "1: Calendar has been initialized"] Initalized = 1 , } impl From < Initsr > for bool { # [inline (always)] fn from (variant : Initsr) -> Self { variant as u8 != 0 } } # [doc = "Field `INITS` reader - Initialization status flag"] pub type InitsR = crate :: BitReader < Initsr > ; impl InitsR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Initsr { match self . bits { false => Initsr :: NotInitalized , true => Initsr :: Initalized , } } # [doc = "Calendar has not been initialized"] # [inline (always)] pub fn is_not_initalized (& self) -> bool { * self == Initsr :: NotInitalized } # [doc = "Calendar has been initialized"] # [inline (always)] pub fn is_initalized (& self) -> bool { * self == Initsr :: Initalized } } # [doc = "Registers synchronization flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Rsfr { # [doc = "0: Calendar shadow registers not yet synchronized"] NotSynced = 0 , # [doc = "1: Calendar shadow registers synchronized"] Synced = 1 , } impl From < Rsfr > for bool { # [inline (always)] fn from (variant : Rsfr) -> Self { variant as u8 != 0 } } # [doc = "Field `RSF` reader - Registers synchronization flag"] pub type RsfR = crate :: BitReader < Rsfr > ; impl RsfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Rsfr { match self . bits { false => Rsfr :: NotSynced , true => Rsfr :: Synced , } } # [doc = "Calendar shadow registers not yet synchronized"] # [inline (always)] pub fn is_not_synced (& self) -> bool { * self == Rsfr :: NotSynced } # [doc = "Calendar shadow registers synchronized"] # [inline (always)] pub fn is_synced (& self) -> bool { * self == Rsfr :: Synced } } # [doc = "Registers synchronization flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RsfwWO { # [doc = "0: This flag is cleared by software by writing 0"] Clear = 0 , } impl From < RsfwWO > for bool { # [inline (always)] fn from (variant : RsfwWO) -> Self { variant as u8 != 0 } } # [doc = "Field `RSF` writer - Registers synchronization flag"] pub type RsfW < 'a , REG > = crate :: BitWriter0C < 'a , REG , RsfwWO > ; impl < 'a , REG > RsfW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "This flag is cleared by software by writing 0"] # [inline (always)] pub fn clear (self) -> & 'a mut crate :: W < REG > { self . variant (RsfwWO :: Clear) } } # [doc = "Initialization flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Initfr { # [doc = "0: Calendar registers update is not allowed"] NotAllowed = 0 , # [doc = "1: Calendar registers update is allowed"] Allowed = 1 , } impl From < Initfr > for bool { # [inline (always)] fn from (variant : Initfr) -> Self { variant as u8 != 0 } } # [doc = "Field `INITF` reader - Initialization flag"] pub type InitfR = crate :: BitReader < Initfr > ; impl InitfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Initfr { match self . bits { false => Initfr :: NotAllowed , true => Initfr :: Allowed , } } # [doc = "Calendar registers update is not allowed"] # [inline (always)] pub fn is_not_allowed (& self) -> bool { * self == Initfr :: NotAllowed } # [doc = "Calendar registers update is allowed"] # [inline (always)] pub fn is_allowed (& self) -> bool { * self == Initfr :: Allowed } } # [doc = "Initialization mode\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Init { # [doc = "0: Free running mode"] FreeRunningMode = 0 , # [doc = "1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."] InitMode = 1 , } impl From < Init > for bool { # [inline (always)] fn from (variant : Init) -> Self { variant as u8 != 0 } } # [doc = "Field `INIT` reader - Initialization mode"] pub type InitR = crate :: BitReader < Init > ; impl InitR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Init { match self . bits { false => Init :: FreeRunningMode , true => Init :: InitMode , } } # [doc = "Free running mode"] # [inline (always)] pub fn is_free_running_mode (& self) -> bool { * self == Init :: FreeRunningMode } # [doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."] # [inline (always)] pub fn is_init_mode (& self) -> bool { * self == Init :: InitMode } } # [doc = "Field `INIT` writer - Initialization mode"] pub type InitW < 'a , REG > = crate :: BitWriter < 'a , REG , Init > ; impl < 'a , REG > InitW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Free running mode"] # [inline (always)] pub fn free_running_mode (self) -> & 'a mut crate :: W < REG > { self . variant (Init :: FreeRunningMode) } # [doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."] # [inline (always)] pub fn init_mode (self) -> & 'a mut crate :: W < REG > { self . variant (Init :: InitMode) } } # [doc = "Alarm %s flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Alrafr { # [doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm register (RTC_ALRMxR)"] Match = 1 , } impl From < Alrafr > for bool { # [inline (always)] fn from (variant : Alrafr) -> Self { variant as u8 != 0 } } # [doc = "Field `ALRF(A,B)` reader - Alarm %s flag"] pub type AlrfR = crate :: BitReader < Alrafr > ; impl AlrfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Alrafr > { match self . bits { true => Some (Alrafr :: Match) , _ => None , } } # [doc = "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm register (RTC_ALRMxR)"] # [inline (always)] pub fn is_match (& self) -> bool { * self == Alrafr :: Match } } # [doc = "Alarm %s flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum AlrafwWO { # [doc = "0: This flag is cleared by software by writing 0"] Clear = 0 , } impl From < AlrafwWO > for bool { # [inline (always)] fn from (variant : AlrafwWO) -> Self { variant as u8 != 0 } } # [doc = "Field `ALRF(A,B)` writer - Alarm %s flag"] pub type AlrfW < 'a , REG > = crate :: BitWriter0C < 'a , REG , AlrafwWO > ; impl < 'a , REG > AlrfW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "This flag is cleared by software by writing 0"] # [inline (always)] pub fn clear (self) -> & 'a mut crate :: W < REG > { self . variant (AlrafwWO :: Clear) } } # [doc = "Wakeup timer flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Wutfr { # [doc = "1: This flag is set by hardware when the wakeup auto-reload counter reaches 0"] Zero = 1 , } impl From < Wutfr > for bool { # [inline (always)] fn from (variant : Wutfr) -> Self { variant as u8 != 0 } } # [doc = "Field `WUTF` reader - Wakeup timer flag"] pub type WutfR = crate :: BitReader < Wutfr > ; impl WutfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Wutfr > { match self . bits { true => Some (Wutfr :: Zero) , _ => None , } } # [doc = "This flag is set by hardware when the wakeup auto-reload counter reaches 0"] # [inline (always)] pub fn is_zero (& self) -> bool { * self == Wutfr :: Zero } } # [doc = "Wakeup timer flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum WutfwWO { # [doc = "0: This flag is cleared by software by writing 0"] Clear = 0 , } impl From < WutfwWO > for bool { # [inline (always)] fn from (variant : WutfwWO) -> Self { variant as u8 != 0 } } # [doc = "Field `WUTF` writer - Wakeup timer flag"] pub type WutfW < 'a , REG > = crate :: BitWriter0C < 'a , REG , WutfwWO > ; impl < 'a , REG > WutfW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "This flag is cleared by software by writing 0"] # [inline (always)] pub fn clear (self) -> & 'a mut crate :: W < REG > { self . variant (WutfwWO :: Clear) } } # [doc = "Time-stamp flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Tsfr { # [doc = "1: This flag is set by hardware when a time-stamp event occurs"] TimestampEvent = 1 , } impl From < Tsfr > for bool { # [inline (always)] fn from (variant : Tsfr) -> Self { variant as u8 != 0 } } # [doc = "Field `TSF` reader - Time-stamp flag"] pub type TsfR = crate :: BitReader < Tsfr > ; impl TsfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Tsfr > { match self . bits { true => Some (Tsfr :: TimestampEvent) , _ => None , } } # [doc = "This flag is set by hardware when a time-stamp event occurs"] # [inline (always)] pub fn is_timestamp_event (& self) -> bool { * self == Tsfr :: TimestampEvent } } # [doc = "Time-stamp flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum TsfwWO { # [doc = "0: This flag is cleared by software by writing 0"] Clear = 0 , } impl From < TsfwWO > for bool { # [inline (always)] fn from (variant : TsfwWO) -> Self { variant as u8 != 0 } } # [doc = "Field `TSF` writer - Time-stamp flag"] pub type TsfW < 'a , REG > = crate :: BitWriter0C < 'a , REG , TsfwWO > ; impl < 'a , REG > TsfW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "This flag is cleared by software by writing 0"] # [inline (always)] pub fn clear (self) -> & 'a mut crate :: W < REG > { self . variant (TsfwWO :: Clear) } } # [doc = "Time-stamp overflow flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Tsovfr { # [doc = "1: This flag is set by hardware when a time-stamp event occurs while TSF is already set"] Overflow = 1 , } impl From < Tsovfr > for bool { # [inline (always)] fn from (variant : Tsovfr) -> Self { variant as u8 != 0 } } # [doc = "Field `TSOVF` reader - Time-stamp overflow flag"] pub type TsovfR = crate :: BitReader < Tsovfr > ; impl TsovfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Tsovfr > { match self . bits { true => Some (Tsovfr :: Overflow) , _ => None , } } # [doc = "This flag is set by hardware when a time-stamp event occurs while TSF is already set"] # [inline (always)] pub fn is_overflow (& self) -> bool { * self == Tsovfr :: Overflow } } # [doc = "Time-stamp overflow flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum TsovfwWO { # [doc = "0: This flag is cleared by software by writing 0"] Clear = 0 , } impl From < TsovfwWO > for bool { # [inline (always)] fn from (variant : TsovfwWO) -> Self { variant as u8 != 0 } } # [doc = "Field `TSOVF` writer - Time-stamp overflow flag"] pub type TsovfW < 'a , REG > = crate :: BitWriter0C < 'a , REG , TsovfwWO > ; impl < 'a , REG > TsovfW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "This flag is cleared by software by writing 0"] # [inline (always)] pub fn clear (self) -> & 'a mut crate :: W < REG > { self . variant (TsovfwWO :: Clear) } } # [doc = "Tamper detection flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Tamp1fr { # [doc = "1: This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input"] Tampered = 1 , } impl From < Tamp1fr > for bool { # [inline (always)] fn from (variant : Tamp1fr) -> Self { variant as u8 != 0 } } # [doc = "Field `TAMP1F` reader - Tamper detection flag"] pub type Tamp1fR = crate :: BitReader < Tamp1fr > ; impl Tamp1fR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Tamp1fr > { match self . bits { true => Some (Tamp1fr :: Tampered) , _ => None , } } # [doc = "This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input"] # [inline (always)] pub fn is_tampered (& self) -> bool { * self == Tamp1fr :: Tampered } } # [doc = "Tamper detection flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Tamp1fwWO { # [doc = "0: Flag cleared by software writing 0"] Clear = 0 , } impl From < Tamp1fwWO > for bool { # [inline (always)] fn from (variant : Tamp1fwWO) -> Self { variant as u8 != 0 } } # [doc = "Field `TAMP1F` writer - Tamper detection flag"] pub type Tamp1fW < 'a , REG > = crate :: BitWriter0C < 'a , REG , Tamp1fwWO > ; impl < 'a , REG > Tamp1fW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Flag cleared by software writing 0"] # [inline (always)] pub fn clear (self) -> & 'a mut crate :: W < REG > { self . variant (Tamp1fwWO :: Clear) } } # [doc = "Field `TAMP2F` reader - TAMPER2 detection flag"] pub use Tamp1fR as Tamp2fR ; # [doc = "Field `TAMP2F` writer - TAMPER2 detection flag"] pub use Tamp1fW as Tamp2fW ; # [doc = "Recalibration pending Flag\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Recalpfr { # [doc = "1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"] Pending = 1 , } impl From < Recalpfr > for bool { # [inline (always)] fn from (variant : Recalpfr) -> Self { variant as u8 != 0 } } # [doc = "Field `RECALPF` reader - Recalibration pending Flag"] pub type RecalpfR = crate :: BitReader < Recalpfr > ; impl RecalpfR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < Recalpfr > { match self . bits { true => Some (Recalpfr :: Pending) , _ => None , } } # [doc = "The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"] # [inline (always)] pub fn is_pending (& self) -> bool { * self == Recalpfr :: Pending } } impl R { # [doc = "Alarm (A,B) write flag"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ALRAWF` field.</div>"] # [inline (always)] pub fn alrwf (& self , n : u8) -> AlrwfR { # [allow (clippy :: no_effect)] [() ; 2] [n as usize] ; AlrwfR :: new (((self . bits >> n) & 1) != 0) } # [doc = "Iterator for array of:"] # [doc = "Alarm (A,B) write flag"] # [inline (always)] pub fn alrwf_iter (& self) -> impl Iterator < Item = AlrwfR > + '_ { (0 .. 2) . map (move | n | AlrwfR :: new (((self . bits >> n) & 1) != 0)) } # [doc = "Bit 0 - Alarm A write flag"] # [inline (always)] pub fn alrawf (& self) -> AlrwfR { AlrwfR :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Alarm B write flag"] # [inline (always)] pub fn alrbwf (& self) -> AlrwfR { AlrwfR :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Wakeup timer write flag"] # [inline (always)] pub fn wutwf (& self) -> WutwfR { WutwfR :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Shift operation pending"] # [inline (always)] pub fn shpf (& self) -> ShpfR { ShpfR :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - Initialization status flag"] # [inline (always)] pub fn inits (& self) -> InitsR { InitsR :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - Registers synchronization flag"] # [inline (always)] pub fn rsf (& self) -> RsfR { RsfR :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - Initialization flag"] # [inline (always)] pub fn initf (& self) -> InitfR { InitfR :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 7 - Initialization mode"] # [inline (always)] pub fn init (& self) -> InitR { InitR :: new (((self . bits >> 7) & 1) != 0) } # [doc = "Alarm (A,B) flag"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ALRAF` field.</div>"] # [inline (always)] pub fn alrf (& self , n : u8) -> AlrfR { # [allow (clippy :: no_effect)] [() ; 2] [n as usize] ; AlrfR :: new (((self . bits >> (n + 8)) & 1) != 0) } # [doc = "Iterator for array of:"] # [doc = "Alarm (A,B) flag"] # [inline (always)] pub fn alrf_iter (& self) -> impl Iterator < Item = AlrfR > + '_ { (0 .. 2) . map (move | n | AlrfR :: new (((self . bits >> (n + 8)) & 1) != 0)) } # [doc = "Bit 8 - Alarm A flag"] # [inline (always)] pub fn alraf (& self) -> AlrfR { AlrfR :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - Alarm B flag"] # [inline (always)] pub fn alrbf (& self) -> AlrfR { AlrfR :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - Wakeup timer flag"] # [inline (always)] pub fn wutf (& self) -> WutfR { WutfR :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - Time-stamp flag"] # [inline (always)] pub fn tsf (& self) -> TsfR { TsfR :: new (((self . bits >> 11) & 1) != 0) } # [doc = "Bit 12 - Time-stamp overflow flag"] # [inline (always)] pub fn tsovf (& self) -> TsovfR { TsovfR :: new (((self . bits >> 12) & 1) != 0) } # [doc = "Bit 13 - Tamper detection flag"] # [inline (always)] pub fn tamp1f (& self) -> Tamp1fR { Tamp1fR :: new (((self . bits >> 13) & 1) != 0) } # [doc = "Bit 14 - TAMPER2 detection flag"] # [inline (always)] pub fn tamp2f (& self) -> Tamp2fR { Tamp2fR :: new (((self . bits >> 14) & 1) != 0) } # [doc = "Bit 16 - Recalibration pending Flag"] # [inline (always)] pub fn recalpf (& self) -> RecalpfR { RecalpfR :: new (((self . bits >> 16) & 1) != 0) } } impl W { # [doc = "Bit 3 - Shift operation pending"] # [inline (always)] pub fn shpf (& mut self) -> ShpfW < IsrSpec > { ShpfW :: new (self , 3) } # [doc = "Bit 5 - Registers synchronization flag"] # [inline (always)] pub fn rsf (& mut self) -> RsfW < IsrSpec > { RsfW :: new (self , 5) } # [doc = "Bit 7 - Initialization mode"] # [inline (always)] pub fn init (& mut self) -> InitW < IsrSpec > { InitW :: new (self , 7) } # [doc = "Alarm (A,B) flag"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ALRAF` field.</div>"] # [inline (always)] pub fn alrf (& mut self , n : u8) -> AlrfW < IsrSpec > { # [allow (clippy :: no_effect)] [() ; 2] [n as usize] ; AlrfW :: new (self , n + 8) } # [doc = "Bit 8 - Alarm A flag"] # [inline (always)] pub fn alraf (& mut self) -> AlrfW < IsrSpec > { AlrfW :: new (self , 8) } # [doc = "Bit 9 - Alarm B flag"] # [inline (always)] pub fn alrbf (& mut self) -> AlrfW < IsrSpec > { AlrfW :: new (self , 9) } # [doc = "Bit 10 - Wakeup timer flag"] # [inline (always)] pub fn wutf (& mut self) -> WutfW < IsrSpec > { WutfW :: new (self , 10) } # [doc = "Bit 11 - Time-stamp flag"] # [inline (always)] pub fn tsf (& mut self) -> TsfW < IsrSpec > { TsfW :: new (self , 11) } # [doc = "Bit 12 - Time-stamp overflow flag"] # [inline (always)] pub fn tsovf (& mut self) -> TsovfW < IsrSpec > { TsovfW :: new (self , 12) } # [doc = "Bit 13 - Tamper detection flag"] # [inline (always)] pub fn tamp1f (& mut self) -> Tamp1fW < IsrSpec > { Tamp1fW :: new (self , 13) } # [doc = "Bit 14 - TAMPER2 detection flag"] # [inline (always)] pub fn tamp2f (& mut self) -> Tamp2fW < IsrSpec > { Tamp2fW :: new (self , 14) } } # [doc = "initialization and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct IsrSpec ; impl crate :: RegisterSpec for IsrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`isr::R`](R) reader structure"] impl crate :: Readable for IsrSpec { } # [doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"] impl crate :: Writable for IsrSpec { type Safety = crate :: Unsafe ; const ZERO_TO_MODIFY_FIELDS_BITMAP : u32 = 0x7f20 ; } # [doc = "`reset()` method sets ISR to value 0x07"] impl crate :: Resettable for IsrSpec { const RESET_VALUE : u32 = 0x07 ; }