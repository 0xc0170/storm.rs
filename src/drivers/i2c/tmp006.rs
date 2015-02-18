use hil;

///
/// Device driver for the TI TMP006 contactless temperature sensor
///


pub struct TMP006Params {
	pub addr: u16
}

// Define the temperature sensor device. This is valid as long as we have
// an I2C device that implements the I2C interface.
pub struct TMP006 <I2C: hil::i2c::I2C> {
	i2c:  I2C,
	addr: u16 // I2C address
};


impl <I2C: hil::i2c::I2C> TMP006 <I2C> {

	pub fn new (mut i2c_device: I2C, params: TMP006Params) -> TMP006<I2C> {
		// return
		TMP006 {
			i2c: i2c_device,
			addr: params.addr
		}
	}

}













