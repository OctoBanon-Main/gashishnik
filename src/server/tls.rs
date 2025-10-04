use anyhow::Result;
use std::{fs, sync::Arc};
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    ServerConfig,
};

use tokio_rustls::rustls::pki_types::pem::PemObject;

pub fn load_tls_acceptor(cert_path: &str, key_path: &str) -> Result<Arc<TlsAcceptor>> {
    let cert_pem = fs::read(cert_path)?;
    let key_pem = fs::read(key_path)?;

    let certs = CertificateDer::pem_slice_iter(&cert_pem)
        .collect::<Result<Vec<_>, _>>()?;
    let key = PrivateKeyDer::from_pem_slice(&key_pem)?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    Ok(Arc::new(TlsAcceptor::from(Arc::new(config))))
}