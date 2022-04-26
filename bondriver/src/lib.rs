pub trait BonDriver {
    #[no_mangle]
    extern "system" fn open_tuner(&self) -> bool { false }
    extern "system" fn close_tuner(&self) -> bool { false }

    extern "system" fn set_channel(&self, channel: u8) -> bool { false }
    extern "system" fn get_signal_level(&self) -> f32 { 0f32 }

    extern "system" fn wait_ts_stream(&self, timeout: u32) -> u32 { 0 }
    extern "system" fn get_ready_count(&self) -> u32 { 0 }

    extern "system" fn get_ts_stream(&self) -> Vec<u8> { vec![] }
}

pub struct TestBonDriver;

impl BonDriver for TestBonDriver {

}

#[no_mangle]
pub unsafe extern "system" fn CreateBonDriver() -> *const TestBonDriver {
    let instance = TestBonDriver{};

    Box::into_raw(Box::new(instance))
}
