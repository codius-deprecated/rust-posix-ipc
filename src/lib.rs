pub mod signals {
    extern crate libc;
    use std::os;

    #[derive(Show, FromPrimitive)]
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
    }

    extern "C" {
        fn raise(sig: libc::c_int) -> libc::c_int;
        fn kill(pid: libc::pid_t, sig: libc::c_int) -> libc::c_int;
    }
}
