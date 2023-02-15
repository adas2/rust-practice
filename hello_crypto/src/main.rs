extern crate openssl;

use openssl::asn1::Asn1Time;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::extension::{ExtendedKeyUsage, KeyUsage};
use openssl::x509::{X509Name, X509Req, X509};

fn main() {
    // Generate a new RSA key
    let rsa = Rsa::generate(2048).unwrap();
    let private_key = PKey::from_rsa(rsa).unwrap();

    // Create a certificate request
    let mut req = X509Req::builder().unwrap();
    req.set_pubkey(&private_key).unwrap();

    // Set the subject of the request
    let mut name = X509Name::builder().unwrap();
    name.append_entry_by_text("CN", "abhidas.com").unwrap();

    // build a X509Name
    let name = name.build();

    // Set Issuer Name
    req.set_subject_name(&name).unwrap();

    // Sign the request with the private key
    req.sign(&private_key, openssl::hash::MessageDigest::sha256())
        .unwrap();

    // Create the X509Req
    let req = req.build();

    // Create a template for the certificate
    let mut x509_builder = X509::builder().unwrap();

    // Set version, Issuer Name, SAN from Request
    x509_builder.set_version(2).unwrap();
    x509_builder.set_subject_name(req.subject_name()).unwrap();
    x509_builder.set_issuer_name(&name).unwrap();

    // Add the public key to the certificate
    x509_builder.set_pubkey(&private_key).unwrap();

    // Set the validity period of the certificate
    let not_before = Asn1Time::days_from_now(0).unwrap();
    let not_after = Asn1Time::days_from_now(365).unwrap();
    x509_builder.set_not_before(&not_before).unwrap();
    x509_builder.set_not_after(&not_after).unwrap();

    // Add key extensions
    let ext1 = KeyUsage::new()
        .digital_signature()
        .key_encipherment()
        .build()
        .unwrap();

    let ext2 = ExtendedKeyUsage::new().server_auth().build().unwrap();

    x509_builder.append_extension(ext1).unwrap();
    x509_builder.append_extension(ext2).unwrap();

    // Sign the certificate with issuer private key
    x509_builder
        .sign(&private_key, openssl::hash::MessageDigest::sha256())
        .unwrap();

    // Get the resulting X509 certificate object
    let cert = x509_builder.build();

    // Encode the certificate and private key in PEM format
    let cert_pem = cert.to_pem().unwrap();
    let key_pem = private_key.private_key_to_pem_pkcs8().unwrap();

    // Print the PEM-encoded certificate and private key
    println!("{}", String::from_utf8(cert_pem).unwrap());
    println!("{}", String::from_utf8(key_pem).unwrap());
}
