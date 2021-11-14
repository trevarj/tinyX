use ecdsa::elliptic_curve::Field;
use ecdsa::hazmat::SignPrimitive;
use p256::ecdsa::signature::digest::Digest;
use p256::ecdsa::signature::{DigestSigner, DigestVerifier, Signature, Signer, Verifier};
use p256::ecdsa::Signature as OtherSignature;
use p256::Scalar;

fn main() {
    let pem = include_str!("../../../../../.config/tiny/ecdsa.pem");
    // decode challege
    let challenge = base64::decode("yVvoxP9DdyNTDW1+FtKvPgELEQVAVrLcOeBjUPlU13k=").unwrap();

    // construct private key
    let sec1_doc: sec1::EcPrivateKeyDocument = pem.parse().unwrap();
    let ec_key = sec1_doc.private_key().private_key;
    let privkey = p256::SecretKey::from_bytes(ec_key).unwrap();
    let k = p256::Scalar::random(rand::thread_rng());
    let msg = p256::Scalar::from_bytes_reduced(&challenge);
    let sig = privkey.to_secret_scalar().try_sign_prehashed(&k, &msg);
    // complete challenge
    // let sig = privkey.sign(digest).to_der();

    let answer = base64::encode(sig.to_bytes());
    dbg!(&answer, answer.as_bytes().len());

    let decoded_sig = base64::decode(&answer).unwrap();
    let sig = OtherSignature::from_der(&decoded_sig).unwrap();
    let pubkey = p256::ecdsa::VerifyingKey::from_sec1_bytes(
        &base64::decode("Alou0zf/p/unvRJMMSWvm0pRawclBnZ4zmqxj9K8GnkZ").unwrap(),
    )
    .unwrap();

    // pubkey.verify_prehash(&challenge, &sig).unwrap();
}
