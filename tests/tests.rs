#![feature(std_misc)]
extern crate posix_ipc as ipc;

use ipc::signals;
use std::thread;
use std::sync::{Arc, Semaphore, Mutex};

#[test]
fn raise_and_catch_with_closure() {
    let mut caught: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let mut sem: Arc<Semaphore> = Arc::new(Semaphore::new(1));
    sem.acquire();
    {
        let sem = sem.clone();
        let caught = caught.clone();
        let f = move |s: signals::Signal| {
            *caught.lock().unwrap() = true; sem.release()
        };
        unsafe {
            signals::Signal::Usr1.handle(Box::new(f)).unwrap();
        }
    }
    signals::Signal::Usr1.raise().unwrap();
    sem.acquire();
    assert!(*caught.lock().unwrap());
}

#[test]
fn raise_and_catch_with_func() {
    static mut caught: bool = false;
    {
        fn f(s: signals::Signal) {unsafe { caught = true }}
        unsafe {
            signals::Signal::Usr1.handle(Box::new(f)).unwrap();
        }
    }
    signals::Signal::Usr1.raise().unwrap();
    thread::sleep_ms(500); // this is really racy :(
    assert!(unsafe { caught });
}
