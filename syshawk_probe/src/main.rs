#[macro_use]
extern crate rocket;

use std::ops::{Deref, DerefMut};
use rocket::serde::json::Json;
use rocket::State;
use syshawklib;
use syshawklib::sysinfo::SystemExt;

#[get("/cpu")]
pub fn cpu() -> Json<syshawklib::cpu::Cpu> {
    let mut sys = syshawklib::sysinfo::System::new_all();
    let cpu = syshawklib::cpu::Cpu::collect(&mut sys);
    return Json(cpu);
}

#[get("/mem")]
pub fn memory() -> Json<syshawklib::memory::Memory> {
    let mut sys = syshawklib::sysinfo::System::new_all();
    let mem = syshawklib::memory::Memory::collect(&mut sys);
    return Json(mem);
}

#[get("/")]
pub fn system() -> Json<syshawklib::system::System> {
    let system = syshawklib::system::System::collect();
    return Json(system);
}

#[launch]
fn rocket() -> _ {

    rocket::build().mount("/", routes![cpu, memory, system])
}