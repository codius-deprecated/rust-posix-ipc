extern crate "posix-ipc" as ipc;

static mut caught: bool = false;

fn catch_usr1(s: ipc::signals::Signal) {
    unsafe { caught = true }
}

#[test]
fn raise_and_catch() {
    ipc::signals::Signal::Usr1.handle(catch_usr1);
    ipc::signals::Signal::Usr1.raise();
    assert!(unsafe { caught });
}
