use crate::bytes::Storage;
use crate::error::IOError;
use crate::hci;
use crate::hci::command::CommandPacket;
use crate::hci::event::{EventCode, EventPacket, StaticEventBuffer};
use crate::hci::packet::{PacketType, RawPacket};
use crate::hci::usb::device::{Device, DeviceIdentifier};
use crate::hci::usb::Error;
use core::convert::TryFrom;
use core::pin::Pin;
use core::time::Duration;
use futures_util::future::LocalBoxFuture;

pub const HCI_COMMAND_ENDPOINT: u8 = 0x01;
pub const ACL_DATA_OUT_ENDPOINT: u8 = 0x02;
pub const HCI_EVENT_ENDPOINT: u8 = 0x81;
pub const ACL_DATA_IN_ENDPOINT: u8 = 0x82;

pub const INTERFACE_NUM: u8 = 0x00;

/// USB Bluetooth Adapter.
pub struct Adapter {
    handle: rusb::DeviceHandle<rusb::Context>,
    device_descriptor: rusb::DeviceDescriptor,
    _private: (),
}
impl core::fmt::Debug for Adapter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Adapter({:?})", self.device_descriptor)
    }
}
impl Adapter {
    /// Timeout for USB transfers (1s). If expired, it'll return `IOError::TimedOut`. Hoping to just
    /// be a temporary solution until I get Async USB working.
    pub const TIMEOUT: Duration = Duration::from_secs(1);
    pub fn from_handle(mut handle: rusb::DeviceHandle<rusb::Context>) -> Result<Adapter, Error> {
        handle.claim_interface(INTERFACE_NUM)?;
        Ok(Adapter::from_parts(
            handle.device().device_descriptor()?,
            handle,
        ))
    }
    pub(crate) fn from_parts(
        device_descriptor: rusb::DeviceDescriptor,
        handle: rusb::DeviceHandle<rusb::Context>,
    ) -> Adapter {
        Adapter {
            handle,
            _private: (),
            device_descriptor,
        }
    }
    /// Internal USB Device handle from `rusb`. Maybe change in the future if we use a different
    /// crate than `rusb`.
    pub fn rusb_handle_mut(&mut self) -> &mut rusb::DeviceHandle<rusb::Context> {
        &mut self.handle
    }
    pub fn rusb_handle(&self) -> &rusb::DeviceHandle<rusb::Context> {
        &self.handle
    }
    pub fn device_identifier(&self) -> DeviceIdentifier {
        DeviceIdentifier {
            vendor_id: self.device_descriptor.vendor_id(),
            product_id: self.device_descriptor.product_id(),
        }
    }
    pub fn get_manufacturer_string(&self) -> Result<Option<String>, Error> {
        // Note, uses device's primary language and replaces any UTF-8 with '?'.
        // (According to libusb)
        match self.device_descriptor.manufacturer_string_index() {
            Some(index) => Ok(Some(self.handle.read_string_descriptor_ascii(index)?)),
            None => Ok(None),
        }
    }
    pub fn get_product_string(&self) -> Result<Option<String>, Error> {
        // Note, uses device's primary language and replaces any UTF-8 with '?'.
        // (According to libusb)
        match self.device_descriptor.product_string_index() {
            Some(index) => Ok(Some(self.handle.read_string_descriptor_ascii(index)?)),
            None => Ok(None),
        }
    }
    pub fn get_serial_number_string(&self) -> Result<Option<String>, Error> {
        // Note, uses device's primary language and replaces any UTF-8 with '?'.
        // (According to libusb)
        match self.device_descriptor.manufacturer_string_index() {
            Some(index) => Ok(Some(self.handle.read_string_descriptor_ascii(index)?)),
            None => Ok(None),
        }
    }
    pub fn write_hci_command_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        //TODO: Change from synchronous IO to Async IO.
        let mut index = 0;
        let size = bytes.len();
        // TODO: Fix probably infinite loop
        while index < size {
            // bmRequestType = 0x20, bRequest = 0x00, wValue = 0x00, wIndex = 0x00 according to
            // Bluetooth Core Spec v5.2 Vol 4 Part B 2.2
            let amount =
                self.handle
                    .write_control(0x20, 0, 0, 0, &bytes[index..], Self::TIMEOUT)?;
            if amount == 0 {
                return Err(Error(IOError::TimedOut));
            }
            index += amount;
        }
        Ok(())
    }
    pub fn read_some_event_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        Ok(self
            .handle
            .read_interrupt(HCI_EVENT_ENDPOINT, buf, Self::TIMEOUT)?)
    }
    pub fn read_some_acl_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        Ok(self
            .handle
            .read_bulk(ACL_DATA_IN_ENDPOINT, buf, Self::TIMEOUT)?)
    }
    pub fn read_event_bytes(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        // TODO: Change from synchronous IO to Async IO.
        let mut index = 0;
        let size = buf.len();
        // TODO: Fix probably infinite loop
        while index < size {
            let amount = match self.read_some_event_bytes(&mut buf[index..]) {
                Ok(a) => a,
                Err(Error(IOError::TimedOut)) => 0,
                Err(e) => return Err(e),
            };
            index += amount;
        }
        Ok(())
    }
    pub fn read_event_packet<Buf: Storage<u8>>(
        &mut self,
    ) -> Result<EventPacket<Buf>, hci::adapter::Error> {
        let mut header = [0u8; 2];
        self.read_event_bytes(&mut header[..])?;
        let event_code =
            EventCode::try_from(header[0]).map_err(|_| hci::StreamError::BadEventCode)?;
        let len = header[1];
        let mut buf = Buf::with_size(len.into());
        self.read_event_bytes(buf.as_mut())?;
        Ok(EventPacket {
            event_code,
            parameters: buf,
        })
    }
    pub fn write_packet(&mut self, packet: RawPacket<&[u8]>) -> Result<(), Error> {
        // TODO: change this API to safer error handling
        match packet.packet_type {
            PacketType::Command => self.write_hci_command_bytes(packet.buf),
            PacketType::ACLData => unimplemented!(),
            PacketType::SCOData => unimplemented!(),
            PacketType::Event => panic!("can't write an event packet"),
            PacketType::Vendor => unimplemented!(),
        }
    }
    pub fn device(&self) -> Device {
        Device::new(self.handle.device())
    }
    pub fn reset(&mut self) -> Result<(), Error> {
        self.handle.reset()?;
        Ok(())
    }
}
impl Drop for Adapter {
    fn drop(&mut self) {
        // We claim the interface when we make the adapter so we must release when we drop.
        let _ = self.handle.release_interface(INTERFACE_NUM);
    }
}

impl hci::adapter::Adapter for Adapter {
    fn write_command<'s, 'p: 's>(
        mut self: Pin<&'s mut Self>,
        packet: CommandPacket<&'p [u8]>,
    ) -> LocalBoxFuture<'s, Result<(), hci::adapter::Error>> {
        let packed = packet.to_raw_packet::<StaticEventBuffer>();
        Box::pin(async move {
            self.write_hci_command_bytes(packed.buf.as_ref())
                .map_err(hci::adapter::Error::from)
        })
    }

    fn read_event<'s, 'p: 's, S: Storage<u8> + 'p>(
        mut self: Pin<&'s mut Self>,
    ) -> LocalBoxFuture<'s, Result<EventPacket<S>, hci::adapter::Error>> {
        Box::pin(async move { self.read_event_packet().map_err(hci::adapter::Error::from) })
    }
}
