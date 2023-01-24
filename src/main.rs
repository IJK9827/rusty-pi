#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

mod boot{
    use core::arch::global_asm;
    global_asm!(
        ".section .text._start"
    );
}

pub fn SPI_W_BYTE_MODE3(byte: u8, clk_t: u32){
    unsafe {
        for n in 7..0 {
            core::ptr::write_volatile(0x3F20_0028 as *mut u32,1<<11); //CLK pin low
            let bit = byte>>n & (1); //MOSI pin //MSB
            if bit == 1 {
                core::ptr::write_volatile(0x3F20_001C as *mut u32,1<<10);
            }else{
                core::ptr::write_volatile(0x3F20_0028 as *mut u32,1<<10);
            }
            for _ in 1..clk_t {
                asm!("nop")
            }
            core::ptr::write_volatile(0x3F20_001C as *mut u32,1<<11); //CLK pin high
            //read pin maybe
            for _ in 1..clk_t {
                asm!("nop")
            }
        }
        for _ in 1..clk_t {
            asm!("nop")
        }
    }
}


pub fn SPI_W_TMC5160(address: u8, config: u32){
    unsafe {
        //address |= 0x80;
        core::ptr::write_volatile(0x3F20_001C as *mut u32,1<<11); //CLK pin high
        core::ptr::write_volatile(0x3F20_0028 as *mut u32,1<<8); //CS pin low
        SPI_W_BYTE_MODE3(address, 256);
        for n in 3..(0) {
            SPI_W_BYTE_MODE3((config>>(n*8)) as u8, 256);
        }
        core::ptr::write_volatile(0x3F20_001C as *mut u32,1<<8); //CS pin high
    }
}

pub fn SPI_W_DAC8581(val: u16){
    unsafe {
        //address |= 0x80;
        core::ptr::write_volatile(0x3F20_001C as *mut u32,1<<11); //CLK pin high
        core::ptr::write_volatile(0x3F20_0028 as *mut u32,1<<7); //CS pin low
        SPI_W_BYTE_MODE3((val>>8) as u8, 256); //Also MSB
        SPI_W_BYTE_MODE3(val as u8, 256);
        core::ptr::write_volatile(0x3F20_001C as *mut u32,1<<7); //CS pin high
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::ptr::write_volatile(0x3F20_0004 as *mut u32,0x0000_0009); //11, 10
        core::ptr::write_volatile(0x3F20_0000 as *mut u32,0x0120_0000); //8, 7

        core::ptr::write_volatile(0x3F20_001C as *mut u32,1<<8); //CS pin high
        core::ptr::write_volatile(0x3F20_001C as *mut u32,1<<7); //CS pin high
        
        SPI_W_TMC5160(0x6C, 0xCA04_0212); //CHOPCONF setting test
        SPI_W_DAC8581(0x8000); //2.5 volt test
    }
    loop{}
}

#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop {}
}