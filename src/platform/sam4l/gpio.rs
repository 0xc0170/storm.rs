use core::intrinsics;
use hil;

pub mod register {
    #[repr(C, packed)]
    #[derive(Copy)]
    pub struct Full {
        val: u32,
        set: u32,
        clear: u32,
        toggle: u32
    }

    impl Full {
        pub fn read(&self) -> u32 {
            volatile!(self.val)
        }

        pub fn set(&mut self, bits: u32) {
            volatile!(self.set = bits)
        }

        pub fn set_bit(&mut self, bit_num: u32) {
            self.set(1 << bit_num)
        }

        pub fn clear(&mut self, bits: u32) {
            volatile!(self.clear = bits)
        }

        pub fn toggle(&mut self, bits: u32) {
            volatile!(self.toggle = bits)
        }
    }

    #[repr(C, packed)]
    #[derive(Copy)]
    pub struct ReadOnly {
        val: u32,
        reserved: [u32; 3],
    }

    impl ReadOnly {
        pub fn read(&self) -> u32 {
            volatile!(self.val)
        }
    }

    #[repr(C, packed)]
    #[derive(Copy)]
    pub struct ReadClear {
        val: u32,
        reserved0: u32,
        clear: u32,
        reserved1: u32
    }

    impl ReadClear {
        pub fn read(&self) -> u32 {
            volatile!(self.val)
        }

        pub fn clear(&mut self, bits: u32) {
            volatile!(self.clear = bits)
        }
    }
}

#[repr(C, packed)]
struct GPIOPortRegisters {
    gper: register::Full,
    pmr0: register::Full,
    pmr1: register::Full,
    pmr2: register::Full,
    oder: register::Full,
    ovr: register::Full,
    pvr: register::ReadOnly,
    puer: register::Full,
    pder: register::Full,
    ier: register::Full,
    imr0: register::Full,
    imr1: register::Full,
    gfer: register::Full,
    ifr: register::ReadClear,
    reserved1: [u32; 8],
    ocdr0: register::Full,
    ocdr1: register::Full,
    reserved2: [u32; 4],
    osrr0: register::Full,
    reserved3: [u32; 8],
    ster: register::Full,
    reserved4: [u32; 4],
    ever: register::Full,
    reserved5: [u32; 26],
    parameter: u32,
    version: u32,
}

#[derive(Copy)]
pub enum PeripheralFunction {
    A, B, C, D, E, F, G, H
}

impl PeripheralFunction {
    pub fn bit0(self) -> u32 {
        (self as u32) & 0b1
    }

    pub fn bit1(self) -> u32 {
        ((self as u32) & 0b10) >> 1
    }

    pub fn bit2(self) -> u32 {
        ((self as u32) & 0b100) >> 2
    }
}

const BASE_ADDRESS: isize = 0x400E1000;
const SIZE: isize = 0x200;

#[derive(Copy)]
pub enum Port {
    PORT0 = BASE_ADDRESS,
    PORT1 = BASE_ADDRESS + SIZE,
    PORT2 = BASE_ADDRESS + SIZE * 2,
}

#[derive(Copy)]
pub enum Pin {
    P0,  P1,  P2,  P3,  P4,  P5,  P6,  P7,
    P8,  P9,  P10, P11, P12, P13, P14, P15,
    P16, P17, P18, P19, P20, P21, P22, P23,
    P24, P25, P26, P27, P28, P29, P30, P31
}

impl Pin {
    pub fn mask(self) -> u32 {
        1 << self as u32
    }
}

#[derive(Copy)]
pub struct Params {
    pub pin: Pin,
    pub port: Port,
}

pub struct GPIOPin {
    port: &'static mut GPIOPortRegisters,
    pin: Pin
}

impl GPIOPin {
    pub fn new(params: Params) -> GPIOPin {
        GPIOPin {
            port: unsafe { intrinsics::transmute(params.port) },
            pin: params.pin
        }
    }

    pub fn select_peripheral(&mut self, function: PeripheralFunction) {
        // Set pin to be controlled by peripheral function
        self.port.gper.clear(self.pin.mask());

        // Set PMR0-2 according to passed in peripheral
        let n = self.pin as u32;
        self.port.pmr0.set(function.bit0() << n);
        self.port.pmr1.set(function.bit1() << n);
        self.port.pmr2.set(function.bit2() << n);
    }
}

impl hil::GPIOPin for GPIOPin {
    fn enable_output(&mut self) {
        self.port.gper.set(self.pin.mask());
        self.port.oder.set(self.pin.mask());
        self.port.ster.clear(self.pin.mask());
    }

    fn read(&self) -> bool {
        (self.port.pvr.read() & self.pin.mask()) > 0
    }

    fn toggle(&mut self) {
        self.port.ovr.toggle(self.pin.mask());
    }

    fn set(&mut self) {
        self.port.ovr.set(self.pin.mask());
    }

    fn clear(&mut self) {
        self.port.ovr.clear(self.pin.mask());
    }
}

