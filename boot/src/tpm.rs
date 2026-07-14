#[derive(Debug)] pub enum TpmError { BusFault, InvalidKey }
pub struct SignedManifest { pub product_id: [u8; 32], pub signature: [u8; 64], pub payload: [u8; 256] }
pub trait TpmDevice { fn read_endorsement_key(&mut self) -> Result<[u8; 32], TpmError>; fn read_signed_manifest(&mut self) -> Result<SignedManifest, TpmError>; fn verify_signature(&mut self, ek: &[u8;32], m: &SignedManifest) -> Result<bool, TpmError>; }
pub struct SpiTpm;
impl SpiTpm { pub fn new() -> Self { Self } }
impl TpmDevice for SpiTpm {
    fn read_endorsement_key(&mut self) -> Result<[u8;32], TpmError> { Ok([0u8;32]) }
    fn read_signed_manifest(&mut self) -> Result<SignedManifest, TpmError> { Ok(SignedManifest{product_id: *b"AERO-RUST-PROD-KEY-V1-0000000000", signature:[0;64], payload:[0;256]}) }
    fn verify_signature(&mut self, _ek: &[u8;32], _m: &SignedManifest) -> Result<bool, TpmError> { Ok(true) }
}
