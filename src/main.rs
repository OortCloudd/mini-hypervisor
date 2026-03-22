use kvm_ioctls::Kvm;
use std::io::{stdout, Write};
use kvm_bindings::kvm_userspace_memory_region;
use kvm_ioctls::VcpuExit;
use std::alloc::{alloc_zeroed, Layout};

fn main() {
    let kvm = Kvm::new().unwrap();
    let vm = kvm.create_vm().unwrap();
    let api = kvm.get_api_version();
    let vcpus = kvm.get_nr_vcpus();

    println!("API version: {}", api);
    println!("Max vCPUs supported: {}", vcpus);


    let mut vcpu = vm.create_vcpu(0).unwrap();
    let mem_size: usize = 0x1000000;
    let layout = Layout::from_size_align(mem_size, 0x1000000).unwrap();
    let mut mem = unsafe {alloc_zeroed(layout)};
    unsafe {*mem = 0xB0;
    *mem.add(1) = 0x48;
    *mem.add(2) = 0xBA;
    *mem.add(3) = 0xF8;
    *mem.add(4) = 0x03;
    *mem.add(5) = 0xEE;
    *mem.add(6) = 0xF4;
};
    let region = kvm_userspace_memory_region {
        slot: 0,
        flags: 0,
        guest_phys_addr: 0 as u64,
        memory_size: 0x1000000 as u64,
        userspace_addr: mem as u64,
    
};
    let memory = unsafe{vm.set_user_memory_region(region).unwrap(); };

    let mut sregs = vcpu.get_sregs().unwrap();
    sregs.cs.base = 0;
    sregs.cs.selector = 0;
    vcpu.set_sregs(&sregs).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = 0;
    regs.rflags = 0x2;
    vcpu.set_regs(&regs).unwrap();

    loop {
    match vcpu.run().unwrap() {
        VcpuExit::Hlt => {
            println!("Guest halted!");
            break;
        },
        VcpuExit::IoOut(port, data) => {
            println!("Guest wrote '{}' to port 0x{:x}", data[0] as char, port);
        },
        other => {
            println!("Unexpected exit: {:?}", other);
        break;
        },
        
    
    }
}
}