extern crate posix_ipc as ipc;

use ipc::signals;

#[test]
fn raise_and_catch_with_closure() {
    let mut caught = false;
    {
        let f = |Signal| {caught = true;println!("Caught!")};
        unsafe {
            signals::Signal::Usr1.handle(Box::new(f));
        }
    }
    signals::Signal::Usr1.raise();
    assert!(unsafe { caught });
}

#[test]
fn raise_and_catch_with_func() {
    static mut caught: bool = false;
    {
        fn f(s: signals::Signal) {unsafe { caught = true }}
        unsafe {
            signals::Signal::Usr1.handle(Box::new(f));
        }
    }
    signals::Signal::Usr1.raise();
    assert!(unsafe { caught });
}
