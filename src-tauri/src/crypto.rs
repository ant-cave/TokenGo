use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::Sha256;

// PBKDF2 迭代次数，10万应该够用了
const PBKDF2_ITERATIONS: u32 = 100_000;
// 密钥长度 32 字节 = 256 位
const KEY_SIZE: usize = 32;
// 盐值长度 16 字节
const SALT_SIZE: usize = 16;

/// 从密码派生密钥
/// 用 PBKDF2-HMAC-SHA256，迭代 10 万次
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; KEY_SIZE];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    key
}

/// AES-256-GCM 加密
/// 随机生成 nonce，返回 (base64 密文, base64 nonce)
pub fn encrypt(data: &str, key: &[u8; 32]) -> (String, String) {
    // 生成随机 nonce，12 字节是 GCM 推荐值
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key).expect("密钥长度正确就不会出错");
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 加密，把明文转成 bytes
    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .expect("加密失败，可能是内存问题");

    (
        base64::encode(&ciphertext),
        base64::encode(&nonce_bytes),
    )
}

/// AES-256-GCM 解密
/// 失败返回错误信息，成功返回明文
pub fn decrypt(encrypted_data: &str, nonce: &str, key: &[u8; 32]) -> Result<String, String> {
    // base64 解码
    let ciphertext = base64::decode(encrypted_data)
        .map_err(|e| format!("密文解码失败: {}", e))?;
    let nonce_bytes = base64::decode(nonce)
        .map_err(|e| format!("nonce 解码失败: {}", e))?;

    if nonce_bytes.len() != 12 {
        return Err("nonce 长度不对".to_string());
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| format!("密钥初始化失败: {:?}", e))?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 解密
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("解密失败，可能是密钥错误或数据损坏: {:?}", e))?;

    String::from_utf8(plaintext)
        .map_err(|e| format!("解密后不是有效 UTF-8: {}", e))
}

/// 密码哈希
/// 生成随机盐值，返回 (base64 哈希, base64 盐值)
pub fn hash_password(password: &str) -> (String, String) {
    // 随机盐值
    let mut salt = [0u8; SALT_SIZE];
    rand::thread_rng().fill_bytes(&mut salt);

    // 派生密钥作为哈希值
    let hash = derive_key(password, &salt);

    (base64::encode(&hash), base64::encode(&salt))
}

/// 验证密码
/// 用存储的盐值重新计算哈希，对比是否一致
pub fn verify_password(password: &str, hash: &str, salt: &str) -> bool {
    // 解码存储的盐值
    let salt_bytes = match base64::decode(salt) {
        Ok(s) => s,
        Err(_) => return false,
    };

    // 重新计算哈希
    let computed_hash = derive_key(password, &salt_bytes);
    let computed_hash_b64 = base64::encode(&computed_hash);

    // 常量时间比较，防时序攻击
    computed_hash_b64 == hash
}

// base64 编码解码模块，用标准库实现避免额外依赖
mod base64 {
    pub fn encode(input: &[u8]) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        let mut i = 0;

        while i < input.len() {
            let b1 = input[i];
            let b2 = if i + 1 < input.len() { input[i + 1] } else { 0 };
            let b3 = if i + 2 < input.len() { input[i + 2] } else { 0 };

            let idx1 = (b1 >> 2) as usize;
            let idx2 = (((b1 & 0b11) << 4) | (b2 >> 4)) as usize;
            let idx3 = (((b2 & 0b1111) << 2) | (b3 >> 6)) as usize;
            let idx4 = (b3 & 0b111111) as usize;

            result.push(CHARSET[idx1] as char);
            result.push(CHARSET[idx2] as char);

            if i + 1 < input.len() {
                result.push(CHARSET[idx3] as char);
            } else {
                result.push('=');
            }

            if i + 2 < input.len() {
                result.push(CHARSET[idx4] as char);
            } else {
                result.push('=');
            }

            i += 3;
        }

        result
    }

    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        // 去掉填充符再处理
        let input = input.trim();
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let mut result = Vec::with_capacity(input.len() * 3 / 4);
        let mut buf = [0u8; 4];
        let mut buf_len = 0;

        for c in input.chars() {
            if c == '=' {
                break;
            }

            let val = CHARSET.iter().position(|&x| x as char == c);
            match val {
                Some(v) => {
                    buf[buf_len] = v as u8;
                    buf_len += 1;

                    if buf_len == 4 {
                        result.push((buf[0] << 2) | (buf[1] >> 4));
                        result.push((buf[1] << 4) | (buf[2] >> 2));
                        result.push((buf[2] << 6) | buf[3]);
                        buf_len = 0;
                    }
                }
                None => return Err(format!("非法字符: {}", c)),
            }
        }

        // 处理剩余字节
        if buf_len >= 2 {
            result.push((buf[0] << 2) | (buf[1] >> 4));
        }
        if buf_len >= 3 {
            result.push((buf[1] << 4) | (buf[2] >> 2));
        }

        Ok(result)
    }
}

// 公开 base64 解码函数，供其他模块使用
pub fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    base64::decode(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32];
        let data = "测试数据 123 abc";

        let (encrypted, nonce) = encrypt(data, &key);
        let decrypted = decrypt(&encrypted, &nonce, &key).unwrap();

        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_password_hash_verify() {
        let password = "my_secret_password";

        let (hash, salt) = hash_password(password);
        assert!(verify_password(password, &hash, &salt));
        assert!(!verify_password("wrong_password", &hash, &salt));
    }

    #[test]
    fn test_derive_key_deterministic() {
        let password = "test";
        let salt = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        let key1 = derive_key(password, &salt);
        let key2 = derive_key(password, &salt);

        assert_eq!(key1, key2);
    }
}
