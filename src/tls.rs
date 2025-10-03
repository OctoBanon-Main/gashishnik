use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};

pub fn load_tls_acceptor(cert_path: &str, key_path: &str) -> Result<Arc<TlsAcceptor>> {
    let certfile = File::open(cert_path)?;
    let mut reader = BufReader::new(certfile);
    let cert_chain = certs(&mut reader)?
        .into_iter()
        .map(Certificate)
        .collect::<Vec<_>>();

    let keyfile = File::open(key_path)?;
    let mut reader = BufReader::new(keyfile);

    let mut keys = pkcs8_private_keys(&mut reader)?;

    if keys.is_empty() {
        let keyfile = File::open(key_path)?;
        let mut reader = BufReader::new(keyfile);
        keys = rsa_private_keys(&mut reader)?;
    }

    if keys.is_empty() {
        anyhow::bail!("no private keys found in {}", key_path);
    }

    let privkey = PrivateKey(keys.remove(0));

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, privkey)?;

    Ok(Arc::new(TlsAcceptor::from(Arc::new(config))))
}