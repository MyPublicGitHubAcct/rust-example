/*
    Startup file for stm32f746g, which will do:
    - Define the vector table for the MCU
    - Define the reset handler
    - Define the exception handlers
 */

// 1. Define the vector table for the MCU
unsafe extern "C" 
{
    unsafe fn MemManage_Handler();
    unsafe fn BusFault_Handler();
    unsafe fn UsageFault_Handler();
    unsafe fn SVCall_Handler();
    unsafe fn PendSV_Handler();
    unsafe fn SysTick_Handler();
    unsafe fn WWDG_Handler();
    unsafe fn PVD_Handler();
    unsafe fn TAMP_STAMP_Handler();
    unsafe fn RTC_WKUP_Handler();
    unsafe fn FLASH_Handler();
    unsafe fn RCC_Handler();
    unsafe fn EXTI0_Handler();
    unsafe fn EXTI1_Handler();
    unsafe fn EXTI2_Handler();
    unsafe fn EXTI3_Handler();
    unsafe fn EXTI4_Handler();
    unsafe fn DMA1_Stream0_Handler();
    unsafe fn DMA1_Stream1_Handler();
    unsafe fn DMA1_Stream2_Handler();
    unsafe fn DMA1_Stream3_Handler();
    unsafe fn DMA1_Stream4_Handler();
    unsafe fn DMA1_Stream5_Handler();
    unsafe fn DMA1_Stream6_Handler();
    unsafe fn ADC_Handler();
    unsafe fn CAN1_TX_Handler();
    unsafe fn CAN1_RX0_Handler();
    unsafe fn CAN1_RX1_Handler();
    unsafe fn CAN1_SCE_Handler();
    unsafe fn EXTI9_5_Handler();
    unsafe fn TIM1_BRK_TIM9_Handler();
    unsafe fn TIM1_UP_TIM10_Handler();
    unsafe fn TIM1_TRG_COM_TIM11_Handler();
    unsafe fn TIM1_CC_Handler();
    unsafe fn TIM2_Handler();
    unsafe fn TIM3_Handler();
    unsafe fn TIM4_Handler();
    unsafe fn I2C1_EV_Handler();
    unsafe fn I2C1_ER_Handler();
    unsafe fn I2C2_EV_Handler();
    unsafe fn I2C2_ER_Handler();
    unsafe fn SPI1_Handler();
    unsafe fn SPI2_Handler();
    unsafe fn USART1_Handler();
    unsafe fn USART2_Handler();
    unsafe fn USART3_Handler();
    unsafe fn EXTI15_10_Handler();
    unsafe fn RTC_ALARM_Handler();
    unsafe fn OTG_FS_WKUP_Handler();
    unsafe fn TIM8_BRK_TIM12_Handler();
    unsafe fn TIM8_UP_TIM13_Handler();
    unsafe fn TIM8_TRG_COM_TIM14_Handler();
    unsafe fn TIM8_CC_Handler();
    unsafe fn DMA1_Stream7_Handler();
    unsafe fn FMC_Handler();
    unsafe fn SDMMC1_Handler();
    unsafe fn TIM5_Handler();
    unsafe fn SPI3_Handler();
    unsafe fn UART4_Handler();
    unsafe fn UART5_Handler();
    unsafe fn TIM6_DAC_Handler();
    unsafe fn TIM7_Handler();
    unsafe fn DMA2_Stream0_Handler();
    unsafe fn DMA2_Stream1_Handler();
    unsafe fn DMA2_Stream2_Handler();
    unsafe fn DMA2_Stream3_Handler();
    unsafe fn DMA2_Stream4_Handler();
    unsafe fn ETH_Handler();
    unsafe fn ETH_WKUP_Handler();
    unsafe fn CAN2_TX_Handler();
    unsafe fn CAN2_RX0_Handler();
    unsafe fn CAN2_RX1_Handler();
    unsafe fn CAN2_SCE_Handler();
    unsafe fn OTG_FS_Handler();
    unsafe fn DMA2_Stream5_Handler();
    unsafe fn DMA2_Stream6_Handler();
    unsafe fn DMA2_Stream7_Handler();
    unsafe fn USART6_Handler();
    unsafe fn I2C3_EV_Handler();
    unsafe fn I2C3_ER_Handler();
    unsafe fn OTG_HS_EP1_OUT_Handler();
    unsafe fn OTG_HS_EP1_IN_Handler();
    unsafe fn OTG_HS_WKUP_Handler();
    unsafe fn OTG_HS_Handler();
    unsafe fn DCMI_Handler();
    unsafe fn CRYP_Handler();
    unsafe fn HASH_RNG_Handler();
    unsafe fn FPU_Handler();
    unsafe fn UART7_Handler();
    unsafe fn UART8_Handler();
    unsafe fn SPI4_Handler();
    unsafe fn SPI5_Handler();
    unsafe fn SPI6_Handler();
    unsafe fn SAI1_Handler();
    unsafe fn LCD_TFT_Handler();
    unsafe fn LTDC_ER_Handler();
    unsafe fn DMA2D_Handler();
    unsafe fn SAI2_Handler();
    unsafe fn QuadSPI_Handler();
    unsafe fn LPTimer1_Handler();
    unsafe fn HDMI_CEC_Handler();
    unsafe fn I2C4_EV_Handler();
    unsafe fn I2C4_ER_Handler();
    unsafe fn SPDIFRX_Handler();
}

#[unsafe(link_section = ".isr_vector")]
#[used]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 113] = [
    Some(reset_handler),
    Some(nmi_handler),
    Some(hardfault_hanlder),
    Some(MemManage_Handler),
    Some(BusFault_Handler),
    Some(UsageFault_Handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    None,
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    Some(WWDG_Handler),
    Some(PVD_Handler),
    Some(TAMP_STAMP_Handler),
    Some(RTC_WKUP_Handler),
    Some(FLASH_Handler),
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_Stream0_Handler),
    Some(DMA1_Stream1_Handler),
    Some(DMA1_Stream2_Handler),
    Some(DMA1_Stream3_Handler),
    Some(DMA1_Stream4_Handler),
    Some(DMA1_Stream5_Handler),
    Some(DMA1_Stream6_Handler),
    Some(ADC_Handler),
    Some(CAN1_TX_Handler),
    Some(CAN1_RX0_Handler),
    Some(CAN1_RX1_Handler),
    Some(CAN1_SCE_Handler),
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_TIM9_Handler),
    Some(TIM1_UP_TIM10_Handler),
    Some(TIM1_TRG_COM_TIM11_Handler),
    Some(TIM1_CC_Handler),
    Some(TIM2_Handler),
    Some(TIM3_Handler),
    Some(TIM4_Handler),
    Some(I2C1_EV_Handler),
    Some(I2C1_ER_Handler),
    Some(I2C2_EV_Handler),
    Some(I2C2_ER_Handler),
    Some(SPI1_Handler),
    Some(SPI2_Handler),
    Some(USART1_Handler),
    Some(USART2_Handler),
    Some(USART3_Handler),
    Some(EXTI15_10_Handler),
    Some(RTC_ALARM_Handler),
    Some(OTG_FS_WKUP_Handler),
    Some(TIM8_BRK_TIM12_Handler),
    Some(TIM8_UP_TIM13_Handler),
    Some(TIM8_TRG_COM_TIM14_Handler),
    Some(TIM8_CC_Handler),
    Some(DMA1_Stream7_Handler),
    Some(FMC_Handler),
    Some(SDMMC1_Handler),
    Some(TIM5_Handler),
    Some(SPI3_Handler),
    Some(UART4_Handler),
    Some(UART5_Handler),
    Some(TIM6_DAC_Handler),
    Some(TIM7_Handler),
    Some(DMA2_Stream0_Handler),
    Some(DMA2_Stream1_Handler),
    Some(DMA2_Stream2_Handler),
    Some(DMA2_Stream3_Handler),
    Some(DMA2_Stream4_Handler),
    Some(ETH_Handler),
    Some(ETH_WKUP_Handler),
    Some(CAN2_TX_Handler),
    Some(CAN2_RX0_Handler),
    Some(CAN2_RX1_Handler),
    Some(CAN2_SCE_Handler),
    Some(OTG_FS_Handler),
    Some(DMA2_Stream5_Handler),
    Some(DMA2_Stream6_Handler),
    Some(DMA2_Stream7_Handler),
    Some(USART6_Handler),
    Some(I2C3_EV_Handler),
    Some(I2C3_ER_Handler),
    Some(OTG_HS_EP1_OUT_Handler),
    Some(OTG_HS_EP1_IN_Handler),
    Some(OTG_HS_WKUP_Handler),
    Some(OTG_HS_Handler),
    Some(DCMI_Handler),
    Some(CRYP_Handler),
    Some(HASH_RNG_Handler),
    Some(FPU_Handler),
    Some(UART7_Handler),
    Some(UART8_Handler),
    Some(SPI4_Handler),
    Some(SPI5_Handler),
    Some(SPI6_Handler),
    Some(SAI1_Handler),
    Some(LCD_TFT_Handler),
    Some(LTDC_ER_Handler),
    Some(DMA2D_Handler),
    Some(SAI2_Handler),
    Some(QuadSPI_Handler),
    Some(LPTimer1_Handler),
    Some(HDMI_CEC_Handler),
    Some(I2C4_EV_Handler),
    Some(I2C4_ER_Handler),
    Some(SPDIFRX_Handler),
];

#[unsafe(no_mangle)]
extern "C" fn nmi_handler() {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn hardfault_hanlder() {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn default_handler() {
    loop {}
}

// 2. Define the reset handler
#[unsafe(no_mangle)]
extern "C" fn reset_handler() {

    // 1. Copy the .data from FLASH to RAM
    // 2. Zero out the .bss section in the RAM
    // 3. Call main()
    crate::main();
}

// 3. Define the exception handlers
