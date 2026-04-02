use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

// 用 Mutex 包装 Connection 实现线程安全
// 本来想用 RwLock，但 rusqlite Connection 也不支持 Sync
static DB: Mutex<Option<Connection>> = Mutex::new(None);

// 获取数据库目录路径
// 桌面端：使用项目目录下的 ./data/ 文件夹
// 安卓：使用应用私有目录
fn get_data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    // 判断是否为移动端
    #[cfg(mobile)]
    {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("获取应用数据目录失败：{}", e))?;
        Ok(data_dir)
    }
    
    #[cfg(desktop)]
    {
        // 桌面端使用项目目录下的 ./data/ 文件夹
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let data_dir = PathBuf::from(&manifest_dir).join("data");
        
        // 确保目录存在
        fs::create_dir_all(&data_dir).map_err(|e| {
            format!("创建 data 目录失败：{}", e)
        })?;
        
        Ok(data_dir)
    }
}

// 数据库初始化入口
// 需要在 app 启动时调用，传入 app handle
pub fn init_db(app: &tauri::AppHandle) -> Result<()> {
    let mut db_lock = DB.lock().unwrap();
    
    // 检查是否已初始化
    if db_lock.is_some() {
        return Ok(());
    }

    // 获取正确的数据目录
    let data_dir = get_data_dir(app).map_err(|e| {
        rusqlite::Error::InvalidPath(PathBuf::from(e))
    })?;
    
    // 确保目录存在
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| {
            rusqlite::Error::InvalidPath(PathBuf::from(format!("创建目录失败：{}", e)))
        })?;
    }

    // 打开或创建数据库文件
    let db_path = data_dir.join("tokengo.db");
    let conn = Connection::open(&db_path)?;

    // 创建主密码表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS master_password (
            id INTEGER PRIMARY KEY,
            password_hash TEXT NOT NULL,
            salt TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )",
        [],
    )?;

    // 创建 TOTP 密钥表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS totp_secrets (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            encrypted_secret TEXT NOT NULL,
            nonce TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )",
        [],
    )?;

    // 创建设置表（存储主题等用户偏好）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    // 存入全局
    *db_lock = Some(conn);

    Ok(())
}

// 获取数据库连接引用
// 调用前必须先执行 init_db()
pub fn get_db() -> Option<std::sync::MutexGuard<'static, Option<Connection>>> {
    DB.lock().ok()
}

// 测试用的重置函数，清空数据
pub fn reset_db() -> Result<()> {
    let db_lock = DB.lock().unwrap();
    if let Some(ref conn) = *db_lock {
        conn.execute("DELETE FROM master_password", [])?;
        conn.execute("DELETE FROM totp_secrets", [])?;
    }
    Ok(())
}

// 保存设置
pub fn save_setting(key: &str, value: &str) -> Result<(), String> {
    let db_guard = get_db().ok_or("数据库未初始化")?;
    let conn = db_guard.as_ref().ok_or("数据库连接不存在")?;
    
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        [key, value],
    ).map_err(|e| format!("保存设置失败: {}", e))?;
    
    Ok(())
}

// 获取设置
pub fn get_setting(key: &str) -> Result<Option<String>, String> {
    let db_guard = get_db().ok_or("数据库未初始化")?;
    let conn = db_guard.as_ref().ok_or("数据库连接不存在")?;
    
    let result: Result<String, rusqlite::Error> = conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        [key],
        |row| row.get(0),
    );
    
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("查询设置失败: {}", e)),
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_init_db() {
        // 切换到临时目录测试
        let tmp_dir = env::temp_dir().join("tokengo_test");
        env::set_current_dir(&tmp_dir).ok();
        
        // 清理之前的测试数据
        let _ = fs::remove_dir_all(&tmp_dir);
        let _ = fs::create_dir_all(&tmp_dir);
        
        // 测试初始化
        let result = init_db();
        assert!(result.is_ok());
        
        // 第二次初始化应该成功（幂等）
        let result = init_db();
        assert!(result.is_ok());
        
        // 验证能获取连接
        let db = get_db();
        assert!(db.is_some());
        
        // 清理
        let _ = fs::remove_dir_all(&tmp_dir);
    }

    #[test]
    fn test_reset_db() {
        let tmp_dir = env::temp_dir().join("tokengo_test_reset");
        env::set_current_dir(&tmp_dir).ok();
        let _ = fs::remove_dir_all(&tmp_dir);
        let _ = fs::create_dir_all(&tmp_dir);
        
        // 先初始化
        init_db().unwrap();
        
        // 测试重置
        let result = reset_db();
        assert!(result.is_ok());
        
        // 清理
        let _ = fs::remove_dir_all(&tmp_dir);
    }
}
