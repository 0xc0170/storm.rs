use core::intrinsics;
use sam4l::pm;
use hil::uart;

#[repr(C, packed)]
struct UsartRegisters {
    cr: u32,
    mr: u32,
    ier: u32,
    idr: u32,
    imr: u32,
    csr: u32,
    rhr: u32,
    thr: u32,
    brgr: u32, // 0x20
    rtor: u32,
    ttgr: u32,
    reserved0: [u32; 5],
    fidi: u32, // 0x40
    ner: u32,
    reserved1: u32,
    ifr: u32,
    man: u32,
    linmr: u32,
    linir: u32,
    linbrr: u32,
    wpmr: u32,
    wpsr: u32,
    version: u32
}

const SIZE: isize = 0x4000;
const BASE_ADDRESS: isize = 0x40024000;

static mut NUM_ENABLED : isize = 0;

#[derive(Copy)]
pub enum BaseAddr {
    USART0 = BASE_ADDRESS,
    USART1 = BASE_ADDRESS + SIZE,
    USART2 = BASE_ADDRESS + SIZE * 2,
    USART3 = BASE_ADDRESS + SIZE * 3,
}

#[derive(Copy)]
pub struct Params {
    pub address: BaseAddr,
}

pub struct USART {
    registers: &'static mut UsartRegisters,
    clock_enabled: bool,
    tx_enabled: bool,
    rx_enabled: bool
}

impl USART {
    pub fn new(params: Params) -> USART {
        USART {
            registers: unsafe { intrinsics::transmute(params.address) },
            clock_enabled: false,
            tx_enabled: false,
            rx_enabled: false
        }
    }

    pub fn enable_clock(&mut self) {
        if self.clock_enabled {
            return
        }

        let res = unsafe {
            let num_enabled: *mut isize = &mut NUM_ENABLED as *mut isize;
            intrinsics::atomic_xadd(num_enabled, 1)
        };
        if res == 0 {
            pm::enable_pba_clock(11);
        }
    }

    pub fn disable_clock(&mut self) {
        if !self.clock_enabled {
            return
        }

        let res = unsafe {
            let num_enabled: *mut isize = &mut NUM_ENABLED as *mut isize;
            intrinsics::atomic_xsub(num_enabled, 1)
        };
        if res == 1 {
            pm::disable_pba_clock(11);
        }
    }

    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        let cd = 48000000 / (16 * baud_rate);
        volatile!(self.registers.brgr = cd);
    }

    pub fn set_mode(&mut self, mode: u32) {
        volatile!(self.registers.mr = mode);
    }

    pub fn rx_ready(&self) -> bool {
        volatile!(self.registers.csr) & 0b1 != 0
    }

    pub fn tx_ready(&self) -> bool {
        volatile!(self.registers.csr) & 0b10 != 0
    }

    pub fn enable_rx(&mut self) {
        volatile!(self.registers.cr = 1 << 4);
        self.rx_enabled = true;
        self.enable_clock();
    }

    pub fn disable_rx(&mut self) {
        volatile!(self.registers.cr = 1 << 5);
        if !self.tx_enabled {
            self.disable_clock();
        }
    }

    pub fn enable_tx(&mut self) {
        self.enable_clock();
        volatile!(self.registers.cr = 1 << 6);
        self.tx_enabled = true;
        self.enable_clock();
    }

    pub fn disable_tx(&mut self) {
        volatile!(self.registers.cr = 1 << 7);
        if !self.rx_enabled {
            self.disable_clock();
        }
    }
}

impl uart::UART for USART {
    fn init(&mut self, params: uart::UARTParams) {
        self.enable_clock();

        let chrl = ((params.data_bits - 1) & 0x3) as u32;
        let mode = 0 /* UART mode */
            | 0 << 4 /*USCLKS*/
            | chrl << 6 /* Character Length */
            | (params.parity as u32) << 9 /* Parity */
            | 0 << 12; /* Number of stop bits = 1 */;

        self.set_mode(mode);
        self.set_baud_rate(params.baud_rate);
        // Copied from TinyOS, not exactly sure how to generalize
        volatile!(self.registers.ttgr = 4);
    }

    fn send_byte(&mut self, byte: u8) {
        while !self.tx_ready() {}
        volatile!(self.registers.thr = byte as u32);
    }

    fn enable_rx(&mut self) {
        USART::enable_rx(self);
    }

    fn disable_rx(&mut self) {
        USART::disable_rx(self);
    }

    fn enable_tx(&mut self) {
        USART::enable_tx(self);
    }

    fn disable_tx(&mut self) {
        USART::disable_tx(self);
    }
}

