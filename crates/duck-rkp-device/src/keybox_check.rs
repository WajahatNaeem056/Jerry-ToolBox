//! Test-only structural check of a generated keybox (replaces the Tricky Store validator so
//! this workspace carries no unrelated crate).

pub(crate) struct Summary {
    pub chain_lengths: Vec<usize>,
    pub has_ecdsa: bool,
    pub counts_declared_correctly: bool,
}

pub(crate) fn validate(xml: &str) -> Result<Summary, String> {
    if !xml.contains("<CertificateChain>") || !xml.contains("<PrivateKey format=\"pem\">") {
        return Err("keybox has no private key or certificate chain".into());
    }
    let certificates = xml.matches("<Certificate format=\"pem\">").count();
    let declared = xml
        .split("<NumberOfCertificates>")
        .nth(1)
        .and_then(|rest| rest.split("</NumberOfCertificates>").next())
        .and_then(|value| value.trim().parse::<usize>().ok())
        .ok_or("NumberOfCertificates is missing")?;
    Ok(Summary {
        chain_lengths: vec![certificates],
        has_ecdsa: xml.contains("algorithm=\"ecdsa\""),
        counts_declared_correctly: declared == certificates,
    })
}
