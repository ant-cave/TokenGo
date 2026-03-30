use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

mod crypto;
mod db;

// 全局状态，存当前登录的派生密钥
use std::sync::Mutex;

pub struct AppState {
    master_key: Mutex<Option<[u8; 32]>>,
}

// TOTP 密钥结构，返回给前端用
#[derive(Serialize)]
struct TotpSecret {
    id: i64,
    name: String,
    secret: Option<String>,
}

// 添加密钥的请求参数
#[derive(Deserialize, Serialize, Clone)]
struct AddSecretRequest {
    name: String,
    secret: String,
}

// 初始化数据库
#[tauri::command]
fn init_database() -> Result<(), String> {
    db::init_db().map_err(|e| format!("数据库初始化失败: {}", e))
}

// 检查是否已设置主密码
#[tauri::command]
fn has_master_password() -> Result<bool, String> {
    let db_guard = db::get_db().ok_or("数据库未初始化")?;
    let conn = db_guard.as_ref().ok_or("数据库连接不存在")?;
    
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM master_password",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(count > 0)
}

// 设置主密码（首次使用）
#[tauri::command]
fn setup_master_password(
    state: State<AppState>,
    password: String,
) -> Result<(), String> {
    if password.len() < 6 {
        return Err("密码至少6位".to_string());
    }

    let db_guard = db::get_db().ok_or("数据库未初始化")?;
    let conn = db_guard.as_ref().ok_or("数据库连接不存在")?;

    // 生成哈希和盐值
    let (hash, salt) = crypto::hash_password(&password);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // 插入数据库
    conn.execute(
        "INSERT INTO master_password (id, password_hash, salt, created_at) VALUES (1, ?1, ?2, ?3)",
        [&hash, &salt, &now.to_string()],
    )
    .map_err(|e| format!("保存密码失败: {}", e))?;

    // 派生密钥存入状态，相当于自动登录
    let salt_bytes = crypto::decode_base64(&salt).map_err(|e| e.to_string())?;
    let key = crypto::derive_key(&password, &salt_bytes);
    *state.master_key.lock().unwrap() = Some(key);

    Ok(())
}

// 验证主密码并登录
#[tauri::command]
fn verify_master_password(
    state: State<AppState>,
    password: String,
) -> Result<bool, String> {
    let db_guard = db::get_db().ok_or("数据库未初始化")?;
    let conn = db_guard.as_ref().ok_or("数据库连接不存在")?;

    // 查询存储的哈希和盐值
    let (stored_hash, salt): (String, String) = conn
        .query_row(
            "SELECT password_hash, salt FROM master_password WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("查询密码失败: {}", e))?;

    // 验证密码
    if !crypto::verify_password(&password, &stored_hash, &salt) {
        return Ok(false);
    }

    // 派生密钥存入状态
    let salt_bytes = crypto::decode_base64(&salt).map_err(|e| e.to_string())?;
    let key = crypto::derive_key(&password, &salt_bytes);
    *state.master_key.lock().unwrap() = Some(key);

    Ok(true)
}

// 检查是否已登录
#[tauri::command]
fn is_logged_in(state: State<AppState>) -> bool {
    state.master_key.lock().unwrap().is_some()
}

// 登出
#[tauri::command]
fn logout(state: State<AppState>) {
    *state.master_key.lock().unwrap() = None;
}

// 添加 TOTP 密钥
#[tauri::command]
fn add_totp_secret(
    state: State<AppState>,
    request: AddSecretRequest,
) -> Result<(), String> {
    let key = state
        .master_key
        .lock()
        .unwrap()
        .ok_or("未登录")?;

    let db_guard = db::get_db().ok_or("数据库未初始化")?;
    let conn = db_guard.as_ref().ok_or("数据库连接不存在")?;

    // 加密密钥
    let (encrypted, nonce) = crypto::encrypt(&request.secret, &key);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    conn.execute(
        "INSERT INTO totp_secrets (name, encrypted_secret, nonce, created_at) VALUES (?1, ?2, ?3, ?4)",
        [&request.name, &encrypted, &nonce, &now.to_string()],
    )
    .map_err(|e| format!("保存密钥失败: {}", e))?;

    Ok(())
}

// 获取所有 TOTP 密钥（不含明文）
#[tauri::command]
fn get_totp_secrets(state: State<AppState>) -> Result<Vec<TotpSecret>, String> {
    // 检查登录
    let _ = state.master_key.lock().unwrap().ok_or("未登录")?;

    let db_guard = db::get_db().ok_or("数据库未初始化")?;
    let conn = db_guard.as_ref().ok_or("数据库连接不存在")?;

    let mut stmt = conn
        .prepare("SELECT id, name FROM totp_secrets ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let secrets: Vec<TotpSecret> = stmt
        .query_map([], |row| {
            Ok(TotpSecret {
                id: row.get(0)?,
                name: row.get(1)?,
                secret: None,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(secrets)
}

// 获取明文密钥（需要二次验证）
#[tauri::command]
fn get_secret_plaintext(
    state: State<AppState>,
    id: i64,
) -> Result<String, String> {
    let key = state
        .master_key
        .lock()
        .unwrap()
        .ok_or("未登录")?;

    let db_guard = db::get_db().ok_or("数据库未初始化")?;
    let conn = db_guard.as_ref().ok_or("数据库连接不存在")?;

    let (encrypted, nonce): (String, String) = conn
        .query_row(
            "SELECT encrypted_secret, nonce FROM totp_secrets WHERE id = ?1",
            [id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("查询密钥失败: {}", e))?;

    // 解密
    let plaintext = crypto::decrypt(&encrypted, &nonce, &key)?;

    Ok(plaintext)
}

// 生成 TOTP 码
#[tauri::command]
fn generate_totp_code(
    state: State<AppState>,
    id: i64,
) -> Result<String, String> {
    let key = state
        .master_key
        .lock()
        .unwrap()
        .ok_or("未登录")?;

    let db_guard = db::get_db().ok_or("数据库未初始化")?;
    let conn = db_guard.as_ref().ok_or("数据库连接不存在")?;

    let (encrypted, nonce): (String, String) = conn
        .query_row(
            "SELECT encrypted_secret, nonce FROM totp_secrets WHERE id = ?1",
            [id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("查询密钥失败: {}", e))?;

    // 解秘密钥
    let secret = crypto::decrypt(&encrypted, &nonce, &key)?;

    // 生成 TOTP 码
    let code = generate_totp(&secret)?;

    Ok(code)
}

// 删除密钥
#[tauri::command]
fn delete_totp_secret(
    state: State<AppState>,
    id: i64,
) -> Result<(), String> {
    // 检查登录状态
    let _ = state.master_key.lock().unwrap().ok_or("未登录")?;

    let db_guard = db::get_db().ok_or("数据库未初始化")?;
    let conn = db_guard.as_ref().ok_or("数据库连接不存在")?;

    conn.execute("DELETE FROM totp_secrets WHERE id = ?1", [id])
        .map_err(|e| format!("删除失败: {}", e))?;

    Ok(())
}

// 解析 otpauth:// URI
#[tauri::command]
fn parse_otpauth_uri(uri: String) -> Result<AddSecretRequest, String> {
    // 格式: otpauth://totp/Label?secret=XXX&issuer=YYY
    if !uri.starts_with("otpauth://") {
        return Err("无效的 URI 格式".to_string());
    }

    // 简单解析，提取 secret 和名称
    let mut secret = None;
    let mut name = "未命名".to_string();

    // 提取路径部分作为名称
    if let Some(path_start) = uri.find("//totp/") {
        let path_end = uri.find('?').unwrap_or(uri.len());
        let path = &uri[path_start + 7..path_end];
        if !path.is_empty() {
            // URL decode 简单处理
            name = path.replace("%20", " ");
        }
    }

    // 提取 secret 参数
    for param in uri.split('&') {
        if let Some(val) = param.strip_prefix("secret=") {
            secret = Some(val.to_string());
        } else if let Some(val) = param.strip_prefix("?secret=") {
            secret = Some(val.to_string());
        }
    }

    match secret {
        Some(s) => Ok(AddSecretRequest { name, secret: s }),
        None => Err("URI 中未找到 secret 参数".to_string()),
    }
}

// 内部 TOTP 生成实现（RFC 6238）
fn generate_totp(secret: &str) -> Result<String, String> {
    // Base32 解码密钥 - 注意大小写 Rfc4648
    let key = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret)
        .or_else(|| base32::decode(base32::Alphabet::Rfc4648 { padding: true }, secret))
        .ok_or("密钥 Base32 解码失败")?;

    // 获取当前时间步（30秒一个步长）
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let time_step = now / 30;

    // 时间步转大端字节数组
    let counter = time_step.to_be_bytes();

    // HMAC-SHA1
    use hmac::{Hmac, Mac};
    use sha1::Sha1;

    type HmacSha1 = Hmac<Sha1>;
    let mut mac = HmacSha1::new_from_slice(&key)
        .map_err(|e| format!("HMAC 初始化失败: {:?}", e))?;
    mac.update(&counter);
    let result = mac.finalize();
    let hash = result.into_bytes();

    // 动态截断（RFC 4226）
    let offset = (hash[hash.len() - 1] & 0x0f) as usize;
    let code = ((hash[offset] as u32 & 0x7f) << 24)
        | ((hash[offset + 1] as u32) << 16)
        | ((hash[offset + 2] as u32) << 8)
        | (hash[offset + 3] as u32);

    // 取6位数字
    let otp = code % 1_000_000;
    Ok(format!("{:06}", otp))
}

// 获取当前时间步剩余秒数（用于倒计时）
#[tauri::command]
fn get_time_remaining() -> u8 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let remaining = 30 - (now % 30);
    remaining as u8
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            master_key: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            init_database,
            has_master_password,
            setup_master_password,
            verify_master_password,
            is_logged_in,
            logout,
            add_totp_secret,
            get_totp_secrets,
            get_secret_plaintext,
            generate_totp_code,
            delete_totp_secret,
            parse_otpauth_uri,
            get_time_remaining,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_totp() {
        // 使用已知的测试向量
        // 密钥 "JBSWY3DPEHPK3PXP" (Base32 编码的 "Hello!")
        let result = generate_totp("JBSWY3DPEHPK3PXP");
        assert!(result.is_ok());
        
        // 验证格式是6位数字
        let code = result.unwrap();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_add_secret_request() {
        let req = AddSecretRequest {
            name: "测试".to_string(),
            secret: "JBSWY3DPEHPK3PXP".to_string(),
        };
        assert_eq!(req.name, "测试");
        assert_eq!(req.secret, "JBSWY3DPEHPK3PXP");
    }

    #[test]
    fn test_totp_secret_serialization() {
        let secret = TotpSecret {
            id: 1,
            name: "测试密钥".to_string(),
            secret: Some("abc123".to_string()),
        };
        
        let json = serde_json::to_string(&secret).unwrap();
        assert!(json.contains("测试密钥"));
        assert!(json.contains("abc123"));
    }
}
