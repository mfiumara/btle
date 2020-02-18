pub mod event;
/// HCI Layer is Little Endian.
pub mod le;
pub mod link_control;
#[cfg(all(feature = "remote"))]
pub mod remote;
#[cfg(all(unix, feature = "std"))]
pub mod socket;
pub mod stream;
use crate::bytes::ToFromBytesEndian;
use core::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct HCIVersionError(());

pub const MAX_ACL_SIZE: usize = (1492 + 4);
pub const MAX_SCO_SIZE: usize = 255;
pub const MAX_EVENT_SIZE: usize = 260;
pub const MAX_FRAME_SIZE: usize = MAX_ACL_SIZE + 4;

pub enum DevEvent {
    Reg = 1,
    Unreg = 2,
    Up = 3,
    Down = 4,
    Suspend = 5,
    Resume = 6,
}
pub enum BusType {
    Virtual = 0,
    USB = 1,
    PCCard = 2,
    UART = 3,
    RS232 = 4,
    PCI = 5,
    SDIO = 6,
}
pub enum ControllerType {
    BREDR = 0x00,
    AMP = 0x01,
}
/// Bluetooth Version reported by HCI Controller according to HCISpec. More versions may be added in
/// the future once they are released.
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
#[repr(u8)]
pub enum Version {
    Bluetooth1v0b = 0,
    Bluetooth1v1 = 1,
    Bluetooth1v2 = 2,
    Bluetooth2v0 = 3,
    Bluetooth2v1 = 4,
    Bluetooth3v0 = 5,
    Bluetooth4v0 = 6,
    Bluetooth4v1 = 7,
    Bluetooth4v2 = 8,
    Bluetooth5v0 = 9,
    Bluetooth5v1 = 10,
    Bluetooth5v2 = 11,
}
impl From<Version> for u8 {
    fn from(v: Version) -> Self {
        v as u8
    }
}
impl TryFrom<u8> for Version {
    type Error = HCIVersionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Version::Bluetooth1v0b),
            1 => Ok(Version::Bluetooth1v1),
            2 => Ok(Version::Bluetooth1v2),
            3 => Ok(Version::Bluetooth2v0),
            4 => Ok(Version::Bluetooth2v1),
            5 => Ok(Version::Bluetooth3v0),
            6 => Ok(Version::Bluetooth4v0),
            7 => Ok(Version::Bluetooth4v1),
            8 => Ok(Version::Bluetooth4v2),
            9 => Ok(Version::Bluetooth5v0),
            10 => Ok(Version::Bluetooth5v1),
            11 => Ok(Version::Bluetooth5v2),
            _ => Err(HCIVersionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct HCIConversionError(());
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum ErrorCode {
    Ok = 0x00,
    UnknownHCICommand = 0x01,
    NoConnection = 0x02,
    HardwareFailure = 0x03,
    PageTimeout = 0x04,
    AuthenticationFailure = 0x05,
    KeyMissing = 0x06,
    MemoryFull = 0x07,
    ConnectionTimeout = 0x08,
    MaxNumberOfConnections = 0x09,
    MaxNumberOfSCOConnectionsToADevice = 0x0A,
    ACLConnectionAlreadyExists = 0x0B,
    CommandDisallowed = 0x0C,
    HostRejectedDueToLimitedResources = 0x0D,
    HostRejectedDueToSecurityReasons = 0x0E,
    HostRejectedDueToARemoteDeviceOnlyAPersonalDevice = 0x0F,
    HostTimeout = 0x10,
    UnsupportedFeatureOrParameterValue = 0x11,
    InvalidHCICommandParameters = 0x12,
    OtherEndTerminatedConnectionUserEndedConnection = 0x13,
    OtherEndTerminatedConnectionLowResources = 0x14,
    OtherEndTerminatedConnectionAboutToPowerOff = 0x15,
    ConnectionTerminatedByLocalHost = 0x16,
    RepeatedAttempts = 0x17,
    PairingNotAllowed = 0x18,
    UnknownLMPPDU = 0x19,
    UnsupportedRemoteFeature = 0x1A,
    SCOOffsetRejected = 0x1B,
    SCOIntervalRejected = 0x1C,
    SCOAirModeRejected = 0x1D,
    InvalidLMPParameters = 0x1E,
    UnspecifiedError = 0x1F,
    UnsupportedLMPParameter = 0x20,
    RoleChangeNotAllowed = 0x21,
    LMPResponseTimeout = 0x22,
    LMPErrorTransactionCollision = 0x23,
    LMPPDUNotAllowed = 0x24,
    EncryptionModeNotAcceptable = 0x25,
    UnitKeyUsed = 0x26,
    QoSNotSupported = 0x27,
    InstantPassed = 0x28,
    PairingWithUnitKeyNotSupported = 0x29,
    TransactionCollision = 0x2A,
    QOSUnacceptableParameter = 0x2C,
    QOSRejected = 0x2D,
    ClassificationNotSupported = 0x2E,
    InsufficientSecurity = 0x2F,
    ParameterOutOfRange = 0x30,
    RoleSwitchPending = 0x32,
    SlotViolation = 0x34,
    RoleSwitchFailed = 0x35,
    EIRTooLarge = 0x36,
    SimplePairingNotSupported = 0x37,
    HostBusyPairing = 0x38,
}
impl From<ErrorCode> for u8 {
    fn from(code: ErrorCode) -> Self {
        code as u8
    }
}
impl TryFrom<u8> for ErrorCode {
    type Error = HCIConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ErrorCode::Ok),
            0x01 => Ok(ErrorCode::UnknownHCICommand),
            0x02 => Ok(ErrorCode::NoConnection),
            0x03 => Ok(ErrorCode::HardwareFailure),
            0x04 => Ok(ErrorCode::PageTimeout),
            0x05 => Ok(ErrorCode::AuthenticationFailure),
            0x06 => Ok(ErrorCode::KeyMissing),
            0x07 => Ok(ErrorCode::MemoryFull),
            0x08 => Ok(ErrorCode::ConnectionTimeout),
            0x09 => Ok(ErrorCode::MaxNumberOfConnections),
            0x0A => Ok(ErrorCode::MaxNumberOfSCOConnectionsToADevice),
            0x0B => Ok(ErrorCode::ACLConnectionAlreadyExists),
            0x0C => Ok(ErrorCode::CommandDisallowed),
            0x0D => Ok(ErrorCode::HostRejectedDueToLimitedResources),
            0x0E => Ok(ErrorCode::HostRejectedDueToSecurityReasons),
            0x0F => Ok(ErrorCode::HostRejectedDueToARemoteDeviceOnlyAPersonalDevice),
            0x10 => Ok(ErrorCode::HostTimeout),
            0x11 => Ok(ErrorCode::UnsupportedFeatureOrParameterValue),
            0x12 => Ok(ErrorCode::InvalidHCICommandParameters),
            0x13 => Ok(ErrorCode::OtherEndTerminatedConnectionUserEndedConnection),
            0x14 => Ok(ErrorCode::OtherEndTerminatedConnectionLowResources),
            0x15 => Ok(ErrorCode::OtherEndTerminatedConnectionAboutToPowerOff),
            0x16 => Ok(ErrorCode::ConnectionTerminatedByLocalHost),
            0x17 => Ok(ErrorCode::RepeatedAttempts),
            0x18 => Ok(ErrorCode::PairingNotAllowed),
            0x19 => Ok(ErrorCode::UnknownLMPPDU),
            0x1A => Ok(ErrorCode::UnsupportedRemoteFeature),
            0x1B => Ok(ErrorCode::SCOOffsetRejected),
            0x1C => Ok(ErrorCode::SCOIntervalRejected),
            0x1D => Ok(ErrorCode::SCOAirModeRejected),
            0x1E => Ok(ErrorCode::InvalidLMPParameters),
            0x1F => Ok(ErrorCode::UnspecifiedError),
            0x20 => Ok(ErrorCode::UnsupportedLMPParameter),
            0x21 => Ok(ErrorCode::RoleChangeNotAllowed),
            0x22 => Ok(ErrorCode::LMPResponseTimeout),
            0x23 => Ok(ErrorCode::LMPErrorTransactionCollision),
            0x24 => Ok(ErrorCode::LMPPDUNotAllowed),
            0x25 => Ok(ErrorCode::EncryptionModeNotAcceptable),
            0x26 => Ok(ErrorCode::UnitKeyUsed),
            0x27 => Ok(ErrorCode::QoSNotSupported),
            0x28 => Ok(ErrorCode::InstantPassed),
            0x29 => Ok(ErrorCode::PairingWithUnitKeyNotSupported),
            _ => Err(HCIConversionError(())),
        }
    }
}
/// HCI Event Code. 8-bit code corresponding to an HCI Event. Check the Bluetooth Core Spec for more.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum EventCode {
    InquiryComplete = 0x01,
    InquiryResult = 0x02,
    ConnectionComplete = 0x03,
    ConnectionRequest = 0x04,
    DisconnectionComplete = 0x05,
    AuthenticationComplete = 0x06,
    RemoteNameRequestComplete = 0x07,
    EncryptionChange = 0x08,
    ChangeConnectionLinkKeyComplete = 0x09,
    MasterLinkKeyComplete = 0x0A,
    ReadRemoteSupportedFeaturesComplete = 0x0B,
    ReadRemoteVersionInformationComplete = 0x0C,
    QoSSetupComplete = 0x0D,
    CommandComplete = 0x0E,
    CommandStatus = 0x0F,
    FlushOccurred = 0x11,
    RoleChange = 0x12,
    NumberOfCompletedPackets = 0x13,
    ModeChange = 0x14,
    ReturnLinkKeys = 0x15,
    PINCodeRequest = 0x16,
    LinkKeyRequest = 0x17,
    LinkKeyNotification = 0x18,
    LoopbackCommand = 0x19,
    DataBufferOverflow = 0x1A,
    MaxSlotsChange = 0x1B,
    ReadClockOffsetComplete = 0x1C,
    ConnectionPacketTypeChanged = 0x1D,
    QoSViolation = 0x1E,
    PageScanRepetitionModeChange = 0x20,
    FlowSpecificationComplete = 0x21,
    InquiryResultWithRSSI = 0x22,
    ReadRemoteExtendedFeaturesComplete = 0x23,
    SynchronousConnectionComplete = 0x2C,
    SynchronousConnectionChanged = 0x2D,
    SniffSubrating = 0x2E,
    ExtendedInquiryResult = 0x2F,
    EncryptionKeyRefreshComplete = 0x30,
    IOCapabilityRequest = 0x31,
    IOCapabilityResponse = 0x32,
    UserConfirmationRequest = 0x33,
    UserPasskeyRequest = 0x34,
    RemoteOOBDataRequest = 0x35,
    SimplePairingComplete = 0x36,
    LinkSupervisionTimeoutChanged = 0x38,
    EnhancedFlushComplete = 0x39,
    UserPasskeyNotification = 0x3B,
    KeypressNotification = 0x3C,
    RemoteHostSupportedFeaturesNotification = 0x3D,
    PhysicalLinkComplete = 0x40,
    ChannelSelected = 0x41,
    DisconnectionPhysicalLinkComplete = 0x42,
    PhysicalLinkLostEarlyWarning = 0x43,
    PhysicalLinkRecovery = 0x44,
    LogicalLinkComplete = 0x45,
    DisconnectionLogicalLinkComplete = 0x46,
    FlowSpecModifyComplete = 0x47,
    NumberOfCompletedDataBlocks = 0x48,
    ShortRangeModeChangeComplete = 0x4C,
    AMPStatusChange = 0x4D,
    AMPStartTest = 0x49,
    AMPTestEnd = 0x4A,
    AMPReceiverReport = 0x4B,
    LEMeta = 0x3E,
}
impl From<EventCode> for u8 {
    fn from(code: EventCode) -> Self {
        code as u8
    }
}
impl From<EventCode> for u32 {
    fn from(code: EventCode) -> Self {
        code as u32
    }
}
impl TryFrom<u8> for EventCode {
    type Error = HCIConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(EventCode::InquiryComplete),
            0x02 => Ok(EventCode::InquiryResult),
            0x03 => Ok(EventCode::ConnectionComplete),
            0x04 => Ok(EventCode::ConnectionRequest),
            0x05 => Ok(EventCode::DisconnectionComplete),
            0x06 => Ok(EventCode::AuthenticationComplete),
            0x07 => Ok(EventCode::RemoteNameRequestComplete),
            0x08 => Ok(EventCode::EncryptionChange),
            0x09 => Ok(EventCode::ChangeConnectionLinkKeyComplete),
            0x0A => Ok(EventCode::MasterLinkKeyComplete),
            0x0B => Ok(EventCode::ReadRemoteSupportedFeaturesComplete),
            0x0C => Ok(EventCode::ReadRemoteVersionInformationComplete),
            0x0D => Ok(EventCode::QoSSetupComplete),
            0x0E => Ok(EventCode::CommandComplete),
            0x0F => Ok(EventCode::CommandStatus),
            0x11 => Ok(EventCode::FlushOccurred),
            0x12 => Ok(EventCode::RoleChange),
            0x13 => Ok(EventCode::NumberOfCompletedPackets),
            0x14 => Ok(EventCode::ModeChange),
            0x15 => Ok(EventCode::ReturnLinkKeys),
            0x16 => Ok(EventCode::PINCodeRequest),
            0x17 => Ok(EventCode::LinkKeyRequest),
            0x18 => Ok(EventCode::LinkKeyNotification),
            0x19 => Ok(EventCode::LoopbackCommand),
            0x1A => Ok(EventCode::DataBufferOverflow),
            0x1B => Ok(EventCode::MaxSlotsChange),
            0x1C => Ok(EventCode::ReadClockOffsetComplete),
            0x1D => Ok(EventCode::ConnectionPacketTypeChanged),
            0x1E => Ok(EventCode::QoSViolation),
            0x20 => Ok(EventCode::PageScanRepetitionModeChange),
            0x21 => Ok(EventCode::FlowSpecificationComplete),
            0x22 => Ok(EventCode::InquiryResultWithRSSI),
            0x23 => Ok(EventCode::ReadRemoteExtendedFeaturesComplete),
            0x2C => Ok(EventCode::SynchronousConnectionComplete),
            0x2D => Ok(EventCode::SynchronousConnectionChanged),
            0x2E => Ok(EventCode::SniffSubrating),
            0x2F => Ok(EventCode::ExtendedInquiryResult),
            0x30 => Ok(EventCode::EncryptionKeyRefreshComplete),
            0x33 => Ok(EventCode::IOCapabilityRequest),
            0x32 => Ok(EventCode::IOCapabilityResponse),
            0x31 => Ok(EventCode::UserConfirmationRequest),
            0x34 => Ok(EventCode::UserPasskeyRequest),
            0x35 => Ok(EventCode::RemoteOOBDataRequest),
            0x36 => Ok(EventCode::SimplePairingComplete),
            0x38 => Ok(EventCode::LinkSupervisionTimeoutChanged),
            0x39 => Ok(EventCode::EnhancedFlushComplete),
            0x3B => Ok(EventCode::UserPasskeyNotification),
            0x3C => Ok(EventCode::KeypressNotification),
            0x3D => Ok(EventCode::RemoteHostSupportedFeaturesNotification),
            0x40 => Ok(EventCode::PhysicalLinkComplete),
            0x41 => Ok(EventCode::ChannelSelected),
            0x42 => Ok(EventCode::DisconnectionPhysicalLinkComplete),
            0x43 => Ok(EventCode::PhysicalLinkLostEarlyWarning),
            0x44 => Ok(EventCode::PhysicalLinkRecovery),
            0x45 => Ok(EventCode::LogicalLinkComplete),
            0x46 => Ok(EventCode::DisconnectionLogicalLinkComplete),
            0x47 => Ok(EventCode::FlowSpecModifyComplete),
            0x48 => Ok(EventCode::NumberOfCompletedDataBlocks),
            0x4C => Ok(EventCode::ShortRangeModeChangeComplete),
            0x4D => Ok(EventCode::AMPStatusChange),
            0x49 => Ok(EventCode::AMPStartTest),
            0x4A => Ok(EventCode::AMPTestEnd),
            0x4B => Ok(EventCode::AMPReceiverReport),
            0x3E => Ok(EventCode::LEMeta),
            _ => Err(HCIConversionError(())),
        }
    }
}
pub const EVENT_MAX_LEN: usize = 255;
pub const COMMAND_MAX_LEN: usize = 255;
pub const FULL_COMMAND_MAX_LEN: usize = COMMAND_MAX_LEN + OPCODE_LEN + 1;
pub const EVENT_CODE_LEN: usize = 1;
/// 6 bit OGF. (OpCode Ground Field)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(u8)]
pub enum OGF {
    NOP = 0x00,
    LinkControl = 0x01,
    LinkPolicy = 0x02,
    HCIControlBaseband = 0x03,
    InformationalParameters = 0x04,
    StatusParameters = 0x05,
    Testing = 0x06,
    LEController = 0x08,
    VendorSpecific = 0x3F,
}
impl Default for OGF {
    fn default() -> Self {
        OGF::NOP
    }
}
impl From<OGF> for u8 {
    fn from(ogf: OGF) -> Self {
        ogf as u8
    }
}
impl TryFrom<u8> for OGF {
    type Error = HCIConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OGF::NOP),
            0x01 => Ok(OGF::LinkControl),
            0x02 => Ok(OGF::LinkPolicy),
            0x03 => Ok(OGF::HCIControlBaseband),
            0x04 => Ok(OGF::InformationalParameters),
            0x05 => Ok(OGF::StatusParameters),
            0x06 => Ok(OGF::Testing),
            0x08 => Ok(OGF::LEController),
            0x3F => Ok(OGF::VendorSpecific),
            _ => Err(HCIConversionError(())),
        }
    }
}
pub const OCF_MAX: u16 = (1 << 10) - 1;
/// 10 bit OCF (OpCode Command Field)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct OCF(u16);
impl OCF {
    /// Creates a new 10-bit OCF
    /// # Panics
    /// Panics if `ocf > OCF_MAX` (if `ocf` isn't 10-bit)
    pub fn new(ocf: u16) -> Self {
        assert!(ocf <= OCF_MAX, "ocf bigger than 10 bits");
        Self(ocf)
    }
    /// Creates a new 10-bit OCF by masking a u16
    pub fn new_masked(ocf: u16) -> Self {
        Self(ocf & OCF_MAX)
    }
}
impl From<OCF> for u16 {
    fn from(ocf: OCF) -> Self {
        ocf.0
    }
}
const OPCODE_LEN: usize = 2;
/// 16-bit HCI Opcode. Contains a OGF (OpCode Ground Field) and OCF (OpCode Command Field).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct Opcode(pub OGF, pub OCF);
impl Opcode {
    pub const fn byte_len() -> usize {
        OPCODE_LEN
    }
    pub fn pack(&self, buf: &mut [u8]) -> Result<(), HCIPackError> {
        if buf.len() != OPCODE_LEN {
            Err(HCIPackError::BadLength)
        } else {
            buf[..2].copy_from_slice(&u16::from(*self).to_bytes_le());
            Ok(())
        }
    }
    pub fn unpack(buf: &[u8]) -> Result<Opcode, HCIPackError> {
        if buf.len() != OPCODE_LEN {
            Err(HCIPackError::BadLength)
        } else {
            Ok(u16::from_bytes_le(&buf)
                .expect("length checked above")
                .try_into()
                .ok()
                .ok_or(HCIPackError::BadBytes)?)
        }
    }
    pub const fn nop() -> Opcode {
        Opcode(OGF::NOP, OCF(0))
    }
    pub fn is_nop(&self) -> bool {
        self.0 == OGF::NOP
    }
}
impl From<Opcode> for u16 {
    fn from(opcode: Opcode) -> Self {
        (opcode.1).0 & (u16::from(u8::from(opcode.0)) << 10)
    }
}
impl TryFrom<u16> for Opcode {
    type Error = HCIConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let ogf = OGF::try_from(u8::try_from(value >> 10).expect("OGF is 6-bits"))?;
        let ocf = OCF::new_masked(value);
        Ok(Opcode(ogf, ocf))
    }
}
pub struct CommandPacket<Storage: AsRef<[u8]>> {
    opcode: Opcode,
    parameters: Storage,
}
/// Unprocessed HCI Event Packet
pub struct EventPacket<Storage: AsRef<[u8]>> {
    event_opcode: EventCode,
    parameters: Storage,
}
impl<Storage: AsRef<[u8]>> EventPacket<Storage> {
    pub fn new(opcode: EventCode, parameters: Storage) -> Self {
        Self {
            event_opcode: opcode,
            parameters,
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum HCIPackError {
    BadOpcode,
    BadLength,
    SmallBuffer,
    BadBytes,
}
pub trait ReturnParameters {
    const EVENT_CODE: EventCode;
    fn byte_len(&self) -> usize;
    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized;
    fn unpack_from_packet<Storage: AsRef<[u8]>>(
        packet: &EventPacket<Storage>,
    ) -> Result<Self, HCIPackError>
    where
        Self: Sized,
    {
        if packet.event_opcode != Self::EVENT_CODE {
            Err(HCIPackError::BadOpcode)
        } else {
            Self::unpack_from(packet.parameters.as_ref())
        }
    }
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError>;
}
pub struct StatusReturn {
    pub status: ErrorCode,
}
impl StatusReturn {
    pub const fn byte_len() -> usize {
        1
    }
}
impl ReturnParameters for StatusReturn {
    const EVENT_CODE: EventCode = EventCode::CommandComplete;

    fn byte_len(&self) -> usize {
        Self::byte_len()
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError> {
        if buf.len() != Self::byte_len() {
            Err(HCIPackError::BadLength)
        } else {
            Ok(Self {
                status: ErrorCode::try_from(buf[0]).map_err(|_| HCIPackError::BadBytes)?,
            })
        }
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError> {
        if buf.len() != Self::byte_len() {
            Err(HCIPackError::BadLength)
        } else {
            buf[0] = self.status.into();
            Ok(())
        }
    }
}
pub trait Command {
    type Return: ReturnParameters;
    fn opcode() -> Opcode;
    fn full_len(&self) -> usize {
        self.byte_len() + OPCODE_LEN + 1
    }
    fn byte_len(&self) -> usize;
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError>;
    fn pack_full(&self, buf: &mut [u8]) -> Result<usize, HCIPackError> {
        if buf.len() != self.full_len() {
            Err(HCIPackError::BadLength)
        } else {
            self.pack_into(&mut buf[3..])?;
            Self::opcode().pack(&mut buf[..OPCODE_LEN])?;
            buf[2] =
                u8::try_from(self.byte_len()).expect("commands can only have 0xFF parameter bytes");
            Ok(self.byte_len() + OPCODE_LEN)
        }
    }
    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized;
}
pub trait Event {
    const CODE: EventCode;
    fn byte_len(&self) -> usize;
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError>;
    fn pack_full(&self, buf: &mut [u8]) -> Result<usize, HCIPackError> {
        if buf.len() != self.byte_len() + EVENT_CODE_LEN + 1 {
            Err(HCIPackError::BadLength)
        } else {
            self.pack_into(&mut buf[2..])?;
            buf[0] = Self::CODE.into();
            buf[1] =
                u8::try_from(self.byte_len()).expect("events can only have 0xFF parameter bytes");
            Ok(self.byte_len() + EVENT_CODE_LEN + 1)
        }
    }
    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized;
}
