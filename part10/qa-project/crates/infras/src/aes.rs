use base64::engine::{general_purpose::STANDARD, Engine};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher::SymmetricCipherError};

pub struct AesCBCCrypto {
    key: Vec<u8>, // key表示32位的字符串对应的bytes
    iv: Vec<u8>,  // iv表示16位的字符串对应的bytes
    key_size: aes::KeySize,
}

// aes key_size类型
pub enum AesKeySize {
    Size128,
    Size192,
    Size256,
}

impl AesCBCCrypto {
    // key表示32位的字符串密钥,iv表示16位的字符串向量
    pub fn new(key: &str, iv: &str, key_size: AesKeySize) -> Self {
        let key_size = match key_size {
            AesKeySize::Size128 => aes::KeySize::KeySize128,
            AesKeySize::Size192 => aes::KeySize::KeySize192,
            AesKeySize::Size256 => aes::KeySize::KeySize256,
        };

        Self {
            key: key.as_bytes().into(),
            iv: iv.as_bytes().into(),
            key_size,
        }
    }

    /// Decrypt a buffer with the given key and iv using AES (128/192256)/CBC/Pkcs encryption.
    /// data 表示需要加密的字符串
    /// 返回的密文是将加密后的Vec<u8>转换为base64格式的字符串和对应的错误
    pub fn encrypt(&self, data: &str) -> Result<String, SymmetricCipherError> {
        let mut encryptor =
            aes::cbc_encryptor(self.key_size, &self.key, &self.iv, blockmodes::PkcsPadding);

        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
        let mut read_buffer = buffer::RefReadBuffer::new(data.as_bytes());
        let mut final_result = Vec::new();

        loop {
            let res = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(
                write_buffer
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|&i| i),
            );
            match res {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => continue,
            }
        }

        // 将加密后的Vec<u8>经过 base64 encode 转换为String
        let decrypted = STANDARD.encode(final_result);
        Ok(decrypted)
    }

    /// Decrypt a buffer with the given key and iv using AES (128/192256)/CBC/Pkcs encryption.
    /// data 表示密文base64格式的字符串
    /// 返回明文字符串和解密的错误信息
    pub fn decrypt(&self, data: &str) -> Result<String, SymmetricCipherError> {
        // 先base64 decode得到Vec<u8>
        let res = STANDARD.decode(data);
        if let Err(err) = res {
            println!("base64 decode data:{} error:{}", data, err);
            return Err(SymmetricCipherError::InvalidPadding);
        }

        let encrypted_data = res.unwrap();
        let mut decrypted =
            aes::cbc_decryptor(self.key_size, &self.key, &self.iv, blockmodes::PkcsPadding);

        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
        let mut read_buffer = buffer::RefReadBuffer::new(&encrypted_data);
        let mut final_result = Vec::new();
        loop {
            let result = decrypted.decrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(
                write_buffer
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|&i| i),
            );
            match result {
                BufferResult::BufferUnderflow => break,
                _ => continue,
            }
        }

        // 将解密后的Vec<u8>经过 from_utf8 转换为String
        let s = std::str::from_utf8(&final_result).unwrap();
        Ok(s.to_string())
    }
}
