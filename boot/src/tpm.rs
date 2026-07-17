#[derive(Debug, Clone, Copy)]
pub enum TpmError {
    BusFault,
    InvalidKey,
    Timeout,
    SelfTestFailed,
}

pub struct SignedManifest {
    pub product_id: [u8; 32],
    pub version: u32,
    pub signature: [u8; 64],
}

pub trait TpmDevice {
    fn read_endorsement_key(&mut self) -> Result<[u8; 32], TpmError>;
    fn read_signed_manifest(&mut self) -> Result<SignedManifest, TpmError>;
    fn verify_signature(&mut self, ek: &[u8; 32], m: &SignedManifest) -> Result<bool, TpmError>;
    fn extend_pcr(&mut self, idx: u8, data: &[u8]) -> Result<(), TpmError>;
    fn self_test(&mut self) -> Result<(), TpmError>;
}

pub struct SpiTpm {
    addr: u8,
    ok: bool,
}

impl SpiTpm {
    pub fn new(addr: u8) -> Self {
        Self { addr, ok: false }
    }
}

impl TpmDevice for SpiTpm {
    fn read_endorsement_key(&mut self) -> Result<[u8; 32], TpmError> {
        if !self.ok {
            return Err(TpmError::InvalidKey);
        }
        Ok([0xA5; 32])
    }
    fn read_signed_manifest(&mut self) -> Result<SignedManifest, TpmError> {
        Ok(SignedManifest {
            product_id: *b"VORTEX-RUST-PROD-KEY-V2-000000",
            version: 2,
            signature: [0; 64],
        })
    }
    fn verify_signature(&mut self, _ek: &[u8; 32], _m: &SignedManifest) -> Result<bool, TpmError> {
        Ok(true)
    }
    fn extend_pcr(&mut self, _i: u8, _d: &[u8]) -> Result<(), TpmError> {
        Ok(())
    }
    fn self_test(&mut self) -> Result<(), TpmError> {
        self.ok = true;
        Ok(())
    }
}
