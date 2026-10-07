use crate::object::panic_obj::RuntimeSignal;

pub fn handle_signal(signal_res: Result<(), RuntimeSignal>) {
    let _signal = match signal_res {
        Ok(_) => return,
        Err(sig) => sig,
    };
}
