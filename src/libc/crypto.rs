/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! CommonCrypto and friends

use crate::dyld::FunctionExports;
use crate::mem::{ConstVoidPtr, MutPtr};
use crate::{export_c_func, Environment};
use std::ops::Deref;

type CCHmacAlgorithm = u32;

const kCCHmacAlgSHA1:   CCHmacAlgorithm = 0;
const kCCHmacAlgMD5:    CCHmacAlgorithm = 1;
const kCCHmacAlgSHA256: CCHmacAlgorithm = 2;
const kCCHmacAlgSHA384: CCHmacAlgorithm = 3;
const kCCHmacAlgSHA512: CCHmacAlgorithm = 4;

type CCPBKDFAlgorithm = u32;
type CCPseudoRandomAlgorithm = u32;

const kCCPBKDF2: CCPBKDFAlgorithm = 2;
const kCCPRFHmacSHA1: CCPseudoRandomAlgorithm = 1;
const kCCPRFHmacSHA256: CCPseudoRandomAlgorithm = 2;

const kCCSuccess: i32 = 0;
const kCCParamError: i32 = -4300;

type CCAlgorithm = u32;
type CCOptions = u32;
type CCOperation = u32;

const kCCEncrypt: CCOperation = 0;
const kCCDecrypt: CCOperation = 1;

const kCCAlgorithmAES128: CCAlgorithm = 0;

const kCCOptionPKCS7Padding: CCOptions = 1;

fn CC_MD5(env: &mut Environment, data: ConstVoidPtr, len: u32, md: MutPtr<u8>) -> MutPtr<u8> {
    let digest = md5::compute(env.mem.bytes_at(data.cast(), len));
    env.mem.bytes_at_mut(md, 16).copy_from_slice(digest.deref());
    md
}

fn CC_SHA1(env: &mut Environment, data: ConstVoidPtr, len: u32, md: MutPtr<u8>) -> MutPtr<u8> {
    let digest = sha1::Sha1::from(env.mem.bytes_at(data.cast(), len)).digest().bytes();
    env.mem.bytes_at_mut(md, 20).copy_from_slice(&digest);
    md
}

fn CC_SHA224(env: &mut Environment, data: ConstVoidPtr, len: u32, md: MutPtr<u8>) -> MutPtr<u8> {
    use sha2::{Digest, Sha224};
    let digest = Sha224::digest(env.mem.bytes_at(data.cast(), len));
    env.mem.bytes_at_mut(md, 28).copy_from_slice(&digest);
    md
}

fn CC_SHA256(env: &mut Environment, data: ConstVoidPtr, len: u32, md: MutPtr<u8>) -> MutPtr<u8> {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(env.mem.bytes_at(data.cast(), len));
    env.mem.bytes_at_mut(md, 32).copy_from_slice(&digest);
    md
}

fn CC_SHA384(env: &mut Environment, data: ConstVoidPtr, len: u32, md: MutPtr<u8>) -> MutPtr<u8> {
    use sha2::{Digest, Sha384};
    let digest = Sha384::digest(env.mem.bytes_at(data.cast(), len));
    env.mem.bytes_at_mut(md, 48).copy_from_slice(&digest);
    md
}

fn CC_SHA512(env: &mut Environment, data: ConstVoidPtr, len: u32, md: MutPtr<u8>) -> MutPtr<u8> {
    use sha2::{Digest, Sha512};
    let digest = Sha512::digest(env.mem.bytes_at(data.cast(), len));
    env.mem.bytes_at_mut(md, 64).copy_from_slice(&digest);
    md
}

fn CCHmac(
    env: &mut Environment,
    alg: CCHmacAlgorithm,
    key: ConstVoidPtr,
    key_len: u32,
    data: ConstVoidPtr,
    data_len: u32,
    mac_out: MutPtr<u8>,
) {
    let key = env.mem.bytes_at(key.cast(), key_len);
    let data = env.mem.bytes_at(data.cast(), data_len);

    match alg {
        kCCHmacAlgSHA1 => {
            use hmac::{Hmac, Mac};
            use sha1::Sha1;
            let mut mac = Hmac::<Sha1>::new_from_slice(key).unwrap();
            mac.update(data);
            env.mem.bytes_at_mut(mac_out, 20)
                .copy_from_slice(&mac.finalize().into_bytes());
        }
        kCCHmacAlgSHA256 => {
            use hmac::{Hmac, Mac};
            use sha2::Sha256;
            let mut mac = Hmac::<Sha256>::new_from_slice(key).unwrap();
            mac.update(data);
            env.mem.bytes_at_mut(mac_out, 32)
                .copy_from_slice(&mac.finalize().into_bytes());
        }
        _ => {
            log!("CCHmac: unsupported algorithm {}", alg);
        }
    }
}

fn CCKeyDerivationPBKDF(
    env: &mut Environment,
    alg: CCPBKDFAlgorithm,
    password: ConstVoidPtr,
    password_len: u32,
    salt: ConstVoidPtr,
    salt_len: u32,
    prf: CCPseudoRandomAlgorithm,
    rounds: u32,
    derived_key: MutPtr<u8>,
    derived_key_len: u32,
) -> i32 {
    if alg != kCCPBKDF2 {
        return kCCParamError;
    }

    let password = env.mem.bytes_at(password.cast(), password_len);
    let salt = env.mem.bytes_at(salt.cast(), salt_len);
    let out = env.mem.bytes_at_mut(derived_key, derived_key_len);

    match prf {
        kCCPRFHmacSHA1 => {
            use pbkdf2::pbkdf2_hmac;
            use sha1::Sha1;
            pbkdf2_hmac::<Sha1>(password, salt, rounds, out);
        }
        kCCPRFHmacSHA256 => {
            use pbkdf2::pbkdf2_hmac;
            use sha2::Sha256;
            pbkdf2_hmac::<Sha256>(password, salt, rounds, out);
        }
        _ => return kCCParamError,
    }

    kCCSuccess
}

fn CCCrypt(
    env: &mut Environment,
    op: CCOperation,
    alg: CCAlgorithm,
    options: CCOptions,
    key: ConstVoidPtr,
    key_len: u32,
    iv: ConstVoidPtr,
    data_in: ConstVoidPtr,
    data_in_len: u32,
    data_out: MutPtr<u8>,
    data_out_available: u32,
    data_out_moved: MutPtr<u32>,
) -> i32 {
    if alg != kCCAlgorithmAES128 {
        return kCCParamError;
    }

    let key = env.mem.bytes_at(key.cast(), key_len);
    let iv = if iv.is_null() {
        [0u8; 16]
    } else {
        let mut iv_buf = [0u8; 16];
        iv_buf.copy_from_slice(env.mem.bytes_at(iv.cast(), 16));
        iv_buf
    };

    let input = env.mem.bytes_at(data_in.cast(), data_in_len);
    let out = env.mem.bytes_at_mut(data_out, data_out_available);

    use aes::Aes128;
    use block_modes::{BlockMode, Cbc};
    use block_modes::block_padding::Pkcs7;

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    let cipher = Aes128Cbc::new_from_slices(key, &iv).unwrap();

    let result = match op {
        kCCEncrypt => cipher.encrypt(out, input.len()).unwrap(),
        kCCDecrypt => cipher.decrypt(out).unwrap(),
        _ => return kCCParamError,
    };

    env.mem.write(data_out_moved, result.len() as u32);
    kCCSuccess
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CC_MD5(_, _, _)),
    export_c_func!(CC_SHA1(_, _, _)),
    export_c_func!(CC_SHA224(_, _, _)),
    export_c_func!(CC_SHA256(_, _, _)),
    export_c_func!(CC_SHA384(_, _, _)),
    export_c_func!(CC_SHA512(_, _, _)),
    export_c_func!(CCHmac(_, _, _, _, _, _)),
    export_c_func!(CCKeyDerivationPBKDF(_, _, _, _, _, _, _, _, _)),
    export_c_func!(CCCrypt(_, _, _, _, _, _, _, _, _, _)),
];

