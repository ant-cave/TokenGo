# TokenGo

TOTP 密码管理器 - 一个轻量级的双因素认证工具

## 项目简介

这是一个用 Tauri + Vue 3 开发的本地 TOTP 密码管理器。主要功能：

- 本地存储加密的 TOTP 密钥
- 实时生成 6 位动态验证码
- 长按查看明文密钥（需验证主密码）
- 支持手动输入和 otpauth:// URI 导入

## 技术选型

### 为什么选 Tauri？

最开始想用 Electron，但觉得太重。试了下 Tauri，打包后才 10MB 左右，启动快很多。

### 加密方案

- 主密码用 PBKDF2-HMAC-SHA256 派生密钥
- 密钥数据用 AES-256-GCM 加密存储
- 每个密钥随机生成 nonce

### 数据库

用 SQLite，rusqlite 库。本来想用更轻量的，但觉得 SQLite 足够了。桌面端存项目目录的 `data/` 文件夹，移动端用应用私有目录。

## 已知问题

### 1. 密码修改功能未实现

Settings.vue 里有修改密码的界面，但 Rust 后端还没实现 `change_master_password` 命令。

**原因：** 修改密码需要解密所有密钥再用新密钥加密，量有点大，还没写。

**临时方案：** 目前只能导出密钥，删掉应用重装，再重新导入。

### 2. 移动端适配不完善

Android 构建配置有了，但界面在手机上可能显示有问题。主要是 Sidebar 和主内容区的布局没针对竖屏优化。

### 3. 缺少备份/恢复功能

数据存在本地，丢了就没了。考虑加个导出加密备份的功能，但还没写。

## 运行说明

### 开发环境

```bash
# 安装依赖
pnpm install

# 启动开发服务器
pnpm tauri dev
```

### 构建

```bash
# 桌面端（Windows）
pnpm tauri build

# Android（需要 Android SDK）
pnpm tauri build --target android
```

### 常见错误

1. **Rust 环境没装**
   - 安装 [rustup](https://rustup.rs/)
   - 运行 `rustup toolchain install stable`
2. **Android 构建失败**
   - 确保已安装 Android SDK
   - 设置 `ANDROID_HOME` 环境变量
   - 参考 [Tauri Android 文档](https://tauri.app/guides/building/android/)
3. **依赖下载慢**
   - 配置 Rust 镜像（如阿里云）
   - 或者耐心等待（第一次构建要下载很多依赖）

## 使用方法

1. **首次使用**：设置主密码（至少 6 位）
2. **添加密钥**：
   - 手动输入 Base32 编码的密钥
   - 或粘贴 `otpauth://` URI
3. **查看密钥**：长按密钥卡片，输入主密码后显示明文
4. **复制验证码**：点击验证码卡片自动复制到剪贴板

## 文件结构

```
TokenGo/
├── src/                 # 前端代码
│   ├── views/          # 页面组件
│   │   ├── Auth.vue    # 登录/设置页面
│   │   ├── Home.vue    # 密钥列表
│   │   ├── AddSecret.vue  # 手动添加
│   │   ├── ImportSecret.vue # URI 导入
│   │   └── Settings.vue     # 设置页面
│   ├── components/     # 可复用组件
│   │   └── Sidebar.vue  # 侧边栏
│   └── router/         # 路由配置
├── src-tauri/          # Rust 后端
│   ├── src/
│   │   ├── main.rs     # 入口
│   │   ├── lib.rs      # 核心逻辑
│   │   ├── crypto.rs   # 加密解密
│   │   └── db.rs       # 数据库操作
│   └── tauri.conf.json # Tauri 配置
└── package.json
```

<br />

## 协议

本软件遵循 [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.html) 开源协议。

## 致谢

- [Tauri](https://tauri.app/) - 轻量级桌面应用框架
- [Vue 3](https://vuejs.org/) - 前端框架
- [google-authenticator-libpam](https://github.com/google/google-authenticator-libpam) - TOTP 算法参考

## 作者

ANT - [github.com/ant-cave](https://github.com/ant-cave)
