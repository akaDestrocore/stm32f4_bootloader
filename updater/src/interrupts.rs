//! Обработчики прерываний по умолчанию для STM32F4xx

use cortex_m::interrupt;

// Определяем все прерывания STM32F4
#[interrupt]
fn WWDG() { loop {} }

#[interrupt]
fn PVD() { loop {} }

#[interrupt]
fn TAMP_STAMP() { loop {} }

#[interrupt]
fn RTC_WKUP() { loop {} }

#[interrupt]
fn FLASH() { loop {} }

#[interrupt]
fn RCC() { loop {} }

#[interrupt]
fn EXTI0() { loop {} }

#[interrupt]
fn EXTI1() { loop {} }

#[interrupt]
fn EXTI2() { loop {} }

#[interrupt]
fn EXTI3() { loop {} }

#[interrupt]
fn EXTI4() { loop {} }

#[interrupt]
fn DMA1_Stream0() { loop {} }

#[interrupt]
fn DMA1_Stream1() { loop {} }

#[interrupt]
fn DMA1_Stream2() { loop {} }

#[interrupt]
fn DMA1_Stream3() { loop {} }

#[interrupt]
fn DMA1_Stream4() { loop {} }

#[interrupt]
fn DMA1_Stream5() { loop {} }

#[interrupt]
fn DMA1_Stream6() { loop {} }

#[interrupt]
fn ADC() { loop {} }

#[interrupt]
fn CAN1_TX() { loop {} }

#[interrupt]
fn CAN1_RX0() { loop {} }

#[interrupt]
fn CAN1_RX1() { loop {} }

#[interrupt]
fn CAN1_SCE() { loop {} }

#[interrupt]
fn EXTI9_5() { loop {} }

#[interrupt]
fn TIM1_BRK_TIM9() { loop {} }

#[interrupt]
fn TIM1_UP_TIM10() { loop {} }

#[interrupt]
fn TIM1_TRG_COM_TIM11() { loop {} }

#[interrupt]
fn TIM1_CC() { loop {} }

#[interrupt]
fn TIM2() { loop {} }

#[interrupt]
fn TIM3() { loop {} }

#[interrupt]
fn TIM4() { loop {} }

#[interrupt]
fn I2C1_EV() { loop {} }

#[interrupt]
fn I2C1_ER() { loop {} }

#[interrupt]
fn I2C2_EV() { loop {} }

#[interrupt]
fn I2C2_ER() { loop {} }

#[interrupt]
fn SPI1() { loop {} }

#[interrupt]
fn SPI2() { loop {} }

#[interrupt]
fn USART1() { loop {} }

#[interrupt]
fn USART2() { loop {} }

#[interrupt]
fn USART3() { loop {} }

#[interrupt]
fn EXTI15_10() { loop {} }

#[interrupt]
fn RTC_Alarm() { loop {} }

#[interrupt]
fn OTG_FS_WKUP() { loop {} }

#[interrupt]
fn TIM8_BRK_TIM12() { loop {} }

#[interrupt]
fn TIM8_UP_TIM13() { loop {} }

#[interrupt]
fn TIM8_TRG_COM_TIM14() { loop {} }

#[interrupt]
fn TIM8_CC() { loop {} }

#[interrupt]
fn DMA1_Stream7() { loop {} }

#[interrupt]
fn FSMC() { loop {} }

#[interrupt]
fn SDIO() { loop {} }

#[interrupt]
fn TIM5() { loop {} }

#[interrupt]
fn SPI3() { loop {} }

#[interrupt]
fn UART4() { loop {} }

#[interrupt]
fn UART5() { loop {} }

#[interrupt]
fn TIM6_DAC() { loop {} }

#[interrupt]
fn TIM7() { loop {} }

#[interrupt]
fn DMA2_Stream0() { loop {} }

#[interrupt]
fn DMA2_Stream1() { loop {} }

#[interrupt]
fn DMA2_Stream2() { loop {} }

#[interrupt]
fn DMA2_Stream3() { loop {} }

#[interrupt]
fn DMA2_Stream4() { loop {} }

#[interrupt]
fn ETH() { loop {} }

#[interrupt]
fn ETH_WKUP() { loop {} }

#[interrupt]
fn CAN2_TX() { loop {} }

#[interrupt]
fn CAN2_RX0() { loop {} }

#[interrupt]
fn CAN2_RX1() { loop {} }

#[interrupt]
fn CAN2_SCE() { loop {} }

#[interrupt]
fn OTG_FS() { loop {} }

#[interrupt]
fn DMA2_Stream5() { loop {} }

#[interrupt]
fn DMA2_Stream6() { loop {} }

#[interrupt]
fn DMA2_Stream7() { loop {} }

#[interrupt]
fn USART6() { loop {} }

#[interrupt]
fn I2C3_EV() { loop {} }

#[interrupt]
fn I2C3_ER() { loop {} }

#[interrupt]
fn OTG_HS_EP1_OUT() { loop {} }

#[interrupt]
fn OTG_HS_EP1_IN() { loop {} }

#[interrupt]
fn OTG_HS_WKUP() { loop {} }

#[interrupt]
fn OTG_HS() { loop {} }

#[interrupt]
fn DCMI() { loop {} }

#[interrupt]
fn HASH_RNG() { loop {} }

#[interrupt]
fn FPU() { loop {} }