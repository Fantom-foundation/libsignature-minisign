use libhash::Hash as LibHash;
use libsignature::Signature as LibSignature;
use minisign::SignatureBox as MiniSignature;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Signature<H>(MiniSignature, PhantomData<H>);

impl<H> LibSignature for Signature<H>
where
    H: LibHash,
{
    type Hash = H;
    type PublicKey = minisign::PublicKey;
    type SecretKey = minisign::SecretKey;
    type Error = minisign::PError;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
