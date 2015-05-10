#![feature(trace_macros,alloc)]
#[macro_use] extern crate enum_primitive;
pub mod signals {
    extern crate libc;
    use std::io;
    use std::mem;

    enum_from_primitive! {
    #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
    pub enum Signal {
        None = 0,
        Hup,
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
        Ttou,
        Urg,
        XCpu,
        Xfsz,
        Vtalrm,
        Prof,
        Winch,
        Io,
        Pwr,
        Sys
    }
    }

    impl Signal {
        pub fn raise(self) -> Result<(), libc::c_int> {
            match unsafe { raise(self as libc::c_int) } {
                0 => Result::Ok(()),
                _ => Result::Err(io::Error::last_os_error().raw_os_error().unwrap())
            }
        }

        pub fn kill(self, pid: libc::pid_t) -> Result<(), libc::c_int> {
            match unsafe { kill(pid, self as libc::c_int) } {
                0 => Result::Ok(()),
                _ => Result::Err(io::Error::last_os_error().raw_os_error().unwrap())
            }
        }

        pub unsafe fn handle(self, handler: Box<FnMut(Signal)>) -> Result<(), libc::c_int> {
            match signal (self as libc::c_int, mem::transmute(glue::rust_signal_handler)) {
                -1 => Result::Err(io::Error::last_os_error().raw_os_error().unwrap()),
                _ => { glue::set_handler(self, handler); Result::Ok(()) }
            }
        }
    }

    mod glue {
        extern crate libc;
        extern crate alloc;
        extern crate num;
        use self::num::FromPrimitive;
        use super::Signal;
        use std::mem;

        #[derive(Copy, Clone, Debug)]
        struct FnPtr {
            foo: usize,
            bar: usize
        }

        static mut handlers: [FnPtr; 18] = [
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
        ];

        pub unsafe fn set_handler (sig: Signal, f: Box<FnMut(Signal)>) {
            handlers[sig as usize] = mem::transmute(f);
        }

        pub unsafe extern "C" fn rust_signal_handler(sig: libc::c_int) {
            let f: *mut FnMut(Signal) = mem::transmute(handlers[sig as usize]);
            let p: FnPtr = mem::transmute(f);
            if p.foo != 0 && p.bar != 0 {
                match Signal::from_i32(sig) {
                    Some(s) => (*f)(s),
                    None => panic!("Unknown signal {}", sig)
                }
            }
        }
    }

    extern "C" {
        fn raise(sig: libc::c_int) -> libc::c_int;
        fn signal(sig: libc::c_int, handler: *const libc::c_void) -> libc::c_int;
        fn kill(pid: libc::pid_t, sig: libc::c_int) -> libc::c_int;
    }
}
