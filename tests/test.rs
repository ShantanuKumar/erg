use std::path::PathBuf;

use erg_common::config::ErgConfig;
use erg_common::error::MultiErrorDisplay;
use erg_common::traits::Runnable;

use erg::dummy::DummyVM;

#[test]
fn exec_class() -> Result<(), ()> {
    expect_success("examples/class.er")
}

#[test]
fn exec_fib() -> Result<(), ()> {
    expect_success("examples/fib.er")
}

#[test]
fn exec_hello_world() -> Result<(), ()> {
    expect_success("examples/helloworld.er")
}

#[test]
fn exec_import() -> Result<(), ()> {
    expect_success("examples/import.er")
}

#[test]
fn exec_move_check() -> Result<(), ()> {
    expect_failure("examples/move_check.er")
}

#[test]
fn exec_record() -> Result<(), ()> {
    expect_success("examples/record.er")
}

#[test]
fn exec_side_effect() -> Result<(), ()> {
    expect_failure("examples/side_effect.er")
}

#[test]
fn exec_trait() -> Result<(), ()> {
    expect_success("examples/trait.er")
}

#[test]
fn exec_tuple() -> Result<(), ()> {
    expect_success("examples/tuple.er")
}

#[test]
fn exec_unpack() -> Result<(), ()> {
    expect_success("examples/unpack.er")
}

#[test]
fn exec_use_py() -> Result<(), ()> {
    expect_success("examples/use_py.er")
}

#[test]
fn exec_with() -> Result<(), ()> {
    expect_success("examples/with.er")
}

fn expect_success(file_path: &'static str) -> Result<(), ()> {
    let cfg = ErgConfig::with_main_path(PathBuf::from(file_path));
    let mut vm = DummyVM::new(cfg);
    match vm.exec() {
        Ok(0) => Ok(()),
        Ok(_) => Err(()),
        Err(errs) => {
            errs.fmt_all_stderr();
            Err(())
        }
    }
}

fn expect_failure(file_path: &'static str) -> Result<(), ()> {
    let cfg = ErgConfig::with_main_path(PathBuf::from(file_path));
    let mut vm = DummyVM::new(cfg);
    match vm.exec() {
        Ok(0) => Err(()),
        Ok(_) => Ok(()),
        Err(errs) => {
            errs.fmt_all_stderr();
            Ok(())
        }
    }
}
