//! Compile-time API stubs for platforms without Linux `AF_PACKET`.

use async_trait::async_trait;
use bytes::Bytes;
use rustnmap_common::MacAddr;
use std::time::Duration;

use crate::{BpfProgram, EngineStats, PacketEngine, PacketError, Result, RingConfig};

fn unsupported() -> PacketError {
    PacketError::NotSupported("raw packet scanning requires Linux".into())
}

/// Placeholder for the Linux ring ownership type.
#[derive(Debug)]
pub struct RingRef;

/// Owned packet data used by the portable API surface.
#[derive(Debug, Clone)]
pub struct ZeroCopyPacket(Bytes);

impl ZeroCopyPacket {
    /// Returns packet bytes.
    #[must_use]
    pub const fn data(&self) -> &Bytes {
        &self.0
    }
}

/// Linux packet engine placeholder.
#[derive(Debug)]
pub struct MmapPacketEngine;

impl MmapPacketEngine {
    /// Reports that raw packet scanning is unavailable.
    pub fn new(_if_name: &str, _config: RingConfig) -> Result<Self> {
        Err(unsupported())
    }

    /// No packets are available on unsupported platforms.
    pub fn try_recv(&mut self) -> Result<Option<crate::PacketBuffer>> {
        Err(unsupported())
    }

    /// No zero-copy packets are available on unsupported platforms.
    pub fn try_recv_zero_copy(&mut self) -> Result<Option<ZeroCopyPacket>> {
        Err(unsupported())
    }
}

#[async_trait]
impl PacketEngine for MmapPacketEngine {
    async fn start(&mut self) -> Result<()> {
        Err(unsupported())
    }
    async fn recv(&mut self) -> Result<Option<ZeroCopyPacket>> {
        Err(unsupported())
    }
    async fn send(&self, _packet: &[u8]) -> Result<usize> {
        Err(unsupported())
    }
    async fn stop(&mut self) -> Result<()> {
        Err(unsupported())
    }
    fn stats(&self) -> EngineStats {
        EngineStats::default()
    }
    fn flush(&self) -> Result<()> {
        Err(unsupported())
    }
    fn set_filter(&self, _filter: &BpfProgram) -> Result<()> {
        Err(unsupported())
    }
}

/// Async Linux packet engine placeholder.
#[derive(Debug)]
pub struct AsyncPacketEngine;

impl AsyncPacketEngine {
    /// Reports that raw packet scanning is unavailable.
    pub fn new(_if_name: &str, _config: RingConfig) -> Result<Self> {
        Err(unsupported())
    }
    /// Reports that raw packet scanning is unavailable.
    pub async fn start_no_background(&mut self) -> Result<()> {
        Err(unsupported())
    }
    /// Reports that raw packet scanning is unavailable.
    pub async fn recv_timeout(&mut self, _timeout: Duration) -> Result<Option<ZeroCopyPacket>> {
        Err(unsupported())
    }
    /// Reports that raw packet scanning is unavailable.
    pub async fn try_recv_direct(&self) -> Result<Option<ZeroCopyPacket>> {
        Err(unsupported())
    }
    /// Returns no interface index.
    #[must_use]
    pub const fn interface_index(&self) -> u32 {
        0
    }
    /// Returns an empty MAC address.
    #[must_use]
    pub const fn mac_address(&self) -> MacAddr {
        MacAddr::new([0; 6])
    }
}

#[async_trait]
impl PacketEngine for AsyncPacketEngine {
    async fn start(&mut self) -> Result<()> {
        Err(unsupported())
    }
    async fn recv(&mut self) -> Result<Option<ZeroCopyPacket>> {
        Err(unsupported())
    }
    async fn send(&self, _packet: &[u8]) -> Result<usize> {
        Err(unsupported())
    }
    async fn stop(&mut self) -> Result<()> {
        Err(unsupported())
    }
    fn stats(&self) -> EngineStats {
        EngineStats::default()
    }
    fn flush(&self) -> Result<()> {
        Err(unsupported())
    }
    fn set_filter(&self, _filter: &BpfProgram) -> Result<()> {
        Err(unsupported())
    }
}

/// Synchronous Linux packet engine placeholder.
pub type RecvfromPacketEngine = MmapPacketEngine;

/// Linux packet stream placeholder.
#[derive(Debug)]
pub struct PacketStream;
