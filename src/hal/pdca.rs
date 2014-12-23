use core::intrinsics::*;
use core::mem::*;

// We don't want DMAChannel to be Copy'able so move errors are caught at
// compile time (copying isn't technically unsafe, but it's probably never what
// we want and should be caught at compile time).
#[allow(dead_code, missing_copy_implementations)]
pub struct DMAChannel {
    mar : uint,
    psr : uint,
    tcr : uint,
    marr : uint,
    cr : uint,
    mr : uint,
    sr : uint,
    ier : uint,
    idr : uint,
    imr : uint,
    isr : uint,
}

pub const DMA_BASE : uint = 0x400A2000;

#[deriving(Copy)]
pub enum CH {
    CH0 = 0x0,
    CH1 = 0x40,
    CH2 = 2 * 0x40,
    CH3 = 3 * 0x40,
    CH4 = 4 * 0x40,
    CH5 = 5 * 0x40,
    CH6 = 6 * 0x40,
    CH7 = 7 * 0x40,
    CH8 = 8 * 0x40,
    CH9 = 9 * 0x40,
    CH10 = 10 * 0x40,
    CH11 = 11 * 0x40,
    CH12 = 12 * 0x40,
    CH13 = 13 * 0x40,
    CH14 = 14 * 0x40,
    CH15 = 15 * 0x40,
}

#[deriving(Copy)]
pub enum Peripheral {
    USART0RX = 0,
    USART1RX = 1,
    USART2RX = 2,
    USART3RX = 3,
    SPIRX = 4,
    TWIM0RX = 5,
    TWIM1RX = 6,
    TWIM2RX = 7,
    TWIM3RX = 8,
    TWIS0RX = 9,
    TWIS1RX = 10,
    ADCIFERX = 11,
    CATBRX = 12,
    // RESERVED 13
    IISCRX0 = 14,
    IISCRX1 = 15,
    PARCRX = 16,
    AESARX = 17,
    USART0TX = 18,
    USART1TX = 19,
    USART2TX = 20,
    USART3TX = 21,
    SPITX = 22,
    TWIM0TX = 23,
    TWIM1TX = 24,
    TWIM2TX = 25,
    TWIM3TX = 26,
    TWIS0TX = 27,
    TWIS1TX = 28,
    ADCIFETX = 29,
    CATBTX = 30,
    ADBACBTX0 = 31,
    ADBACBTX1 = 32,
    IISCTX0 = 33,
    IISCTX1 = 34,
    DACCTX = 35,
    AESATX = 36
}

pub struct KO<T> {
    data: *const T
}

impl <T> KO<T> {
    pub fn addr(self) -> uint {
        self.data as uint
    }

    pub fn size(self) -> uint {
        size_of::<T>()
    }
}

impl DMAChannel {
    pub unsafe fn new(ch : CH, pid : Peripheral) -> &'static mut DMAChannel {
        let channel = (DMA_BASE + (ch as uint)) as *mut DMAChannel;
        volatile_store(&mut(*channel).psr, pid as uint & 0xff);
        transmute(channel)
    }

    pub fn set_buffer<T>(&mut self, buf : KO<T>) {
        unsafe {
            volatile_store(&mut(*self).mar, buf.addr());
        }
    }
}

