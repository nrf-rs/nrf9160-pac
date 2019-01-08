#[doc = "Data sent from the debugger to the CPU"]
pub struct RXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data sent from the debugger to the CPU"]
pub mod rxdata;
#[doc = "Status to indicate if data sent from the debugger to the CPU has been read"]
pub struct RXSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status to indicate if data sent from the debugger to the CPU has been read"]
pub mod rxstatus;
#[doc = "Data sent from the CPU to the debugger"]
pub struct TXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data sent from the CPU to the debugger"]
pub mod txdata;
#[doc = "Status to indicate if data sent from the CPU to the debugger status has been read"]
pub struct TXSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status to indicate if data sent from the CPU to the debugger status has been read"]
pub mod txstatus;
