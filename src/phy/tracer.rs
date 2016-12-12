use core::marker::PhantomData;

use Error;
use wire::pretty_print::{PrettyPrint, PrettyPrinter};
use super::Device;

/// A tracer device.
///
/// A tracer is a device that prints all packets traversing it
/// to the standard output, and delegates to another device otherwise.
pub struct Tracer<T: Device, U: PrettyPrint> {
    lower:   T,
    phantom: PhantomData<U>
}

impl<T: Device, U: PrettyPrint> Tracer<T, U> {
    /// Create a tracer device.
    pub fn new(lower: T) -> Tracer<T, U> {
        Tracer {
            lower:   lower,
            phantom: PhantomData
        }
    }

    /// Return the underlying device, consuming the tracer.
    pub fn into_lower(self) -> T {
        self.lower
    }
}

impl<T: Device, U: PrettyPrint> Device for Tracer<T, U> {
    type RxBuffer = T::RxBuffer;
    type TxBuffer = TxBuffer<T::TxBuffer, U>;

    fn mtu(&self) -> usize { self.lower.mtu() }

    fn receive(&mut self) -> Result<Self::RxBuffer, Error> {
        let buffer = try!(self.lower.receive());
        print!("{}", PrettyPrinter::<U>::new("<- ", &buffer));
        Ok(buffer)
    }

    fn transmit(&mut self, len: usize) -> Result<Self::TxBuffer, Error> {
        let buffer = try!(self.lower.transmit(len));
        Ok(TxBuffer {
            buffer:  buffer,
            phantom: PhantomData
        })
    }
}

#[doc(hidden)]
pub struct TxBuffer<T: AsRef<[u8]>, U: PrettyPrint> {
    buffer:  T,
    phantom: PhantomData<U>
}

impl<T: AsRef<[u8]>, U: PrettyPrint> AsRef<[u8]>
        for TxBuffer<T, U> {
    fn as_ref(&self) -> &[u8] { self.buffer.as_ref() }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>, U: PrettyPrint> AsMut<[u8]>
        for TxBuffer<T, U> {
    fn as_mut(&mut self) -> &mut [u8] { self.buffer.as_mut() }
}

impl<T: AsRef<[u8]>, U: PrettyPrint> Drop for TxBuffer<T, U> {
    fn drop(&mut self) {
        print!("{}", PrettyPrinter::<U>::new("-> ", &self.buffer));
    }
}