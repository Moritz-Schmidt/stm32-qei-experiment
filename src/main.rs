#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    Peripherals,
    peripherals::TIM3,
    timer::{
        GeneralInstance4Channel,
        qei::{self, Qei},
    },
};
use embassy_time::Timer;
use log::info;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_log!();
    let p: Peripherals = embassy_stm32::init(Default::default());

    let qei_conf = qei::Config::default();

    let qei = Qei::new(p.TIM3, p.PC6, p.PC7, qei_conf);
    loop {
        let count = qei.count();
        info!("QEI count: {}", count);

        match qei.read_direction() {
            qei::Direction::Upcounting => info!("QEI direction: Upcounting"),
            qei::Direction::Downcounting => info!("QEI direction: Downcounting"),
        }

        Timer::after_millis(100).await;
    }
}
