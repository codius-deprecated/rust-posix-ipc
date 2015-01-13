#![feature(trace_macros)]
pub mod signals {
    extern crate libc;
    use std::os;
    use std::mem;

    #[derive(Hash, Eq, PartialEq, Copy, Show, FromPrimitive)]
    pub enum Signal {
        Hup = 1,
        Int,
        Quit,
        Ill,
        Trap,
        Abrt,
        Bus,
        Fpe,
        Kill,
        Usr1,
        Segv,
        Usr2,
        Pipe,
        Alrm,
        Term,
        StkFlt,
        Chld,
        Cont,
        Stop,
        Tstp,
        Ttin,
    }

    impl Signal {
        pub fn raise(self) -> Result<(), usize> {
            match unsafe { raise(self as libc::c_int) } {
                0 => Result::Ok(()),
                _ => Result::Err(os::errno())
            }
        }

        pub fn kill(self, pid: libc::pid_t) -> Result<(), usize> {
            match unsafe { kill(pid, self as libc::c_int) } {
                0 => Result::Ok(()),
                _ => Result::Err(os::errno())
            }
        }

        pub fn handle(self, handler: fn(Signal)) -> Result<(), usize> {
            match unsafe { signal (self as libc::c_int, mem::transmute(glue::rust_signal_handler)) } {
                -1 => Result::Err(os::errno()),
                _ => { glue::set_handler(self, handler); Result::Ok(()) }
            }
        }
    }

    mod glue {
        extern crate libc;
        extern crate alloc;
        use super::Signal;
        use std::num::FromPrimitive;
        use self::alloc::arc::Arc;

        static mut handlers: [fn(Signal); 15] = [
            null_handler, null_handler, null_handler, null_handler, null_handler,
            null_handler, null_handler, null_handler, null_handler, null_handler,
            null_handler, null_handler, null_handler, null_handler, null_handler
        ];

        pub fn set_handler (sig: Signal, f: fn(Signal)) {
            unsafe { handlers[sig as usize] = f }
        }

        fn null_handler(s: Signal) {}

        pub unsafe extern "C" fn rust_signal_handler(sig: libc::c_int) {
            let f = handlers[sig as usize];
            f(FromPrimitive::from_i32(sig).expect("Unknown signal"));
        }
    }

    extern "C" {
        fn raise(sig: libc::c_int) -> libc::c_int;
        fn signal(sig: libc::c_int, handler: *const libc::c_void) -> libc::c_int;
        fn kill(pid: libc::pid_t, sig: libc::c_int) -> libc::c_int;
    }
}
