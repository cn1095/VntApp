# VNT GUI

VNT GUI

## Build

### Install

After [Flutter](https://docs.flutter.dev/get-started/install) and [Rust](https://www.rust-lang.org/tools/install) are installed, install `flutter_rust_bridge`

### Run
```
flutter run
```


## Special

Thanks to ChatGPT for helping with a lot of the work on this project.



# VntApp 开发指南

## 项目概述

VntApp 是一个基于 Flutter 和 Rust 的跨平台 VPN 客户端应用，支持 Windows、macOS、Linux 和 Android 平台。该项目使用 Flutter 作为 UI 框架，Rust 作为核心网络处理引擎，通过 flutter_rust_bridge 实现两者的互操作。

## 技术栈

-   **前端框架**: Flutter 3.x
-   **后端语言**: Rust
-   **跨语言桥接**: flutter_rust_bridge 2.0.0-dev.37
-   **核心依赖**: VNT (Virtual Network Tunneling) 库
-   **支持平台**: Windows, macOS, Linux, Android

## 环境要求

### 基础环境

-   Flutter SDK (>= 3.3.0)
-   Dart SDK (>= 3.4.0 < 4.0.0)
-   Rust 工具链
-   flutter_rust_bridge CLI

### 平台特定要求

#### Windows

-   Visual Studio Build Tools
-   Windows SDK

#### macOS

-   Xcode
-   iOS Simulator (可选)

#### Linux

-   GCC/Clang
-   CMake
-   GTK3 开发库

#### Android

-   Android Studio
-   Android SDK
-   NDK

## 项目结构

```
VntApp/
├── android/                    # Android 平台配置
├── ios/                        # iOS 平台配置
├── linux/                      # Linux 平台配置
├── macos/                      # macOS 平台配置
├── windows/                    # Windows 平台配置
├── web/                        # Web 平台配置 (未使用)
├── lib/                        # Flutter 源代码
│   ├── main.dart              # 应用入口
│   ├── vnt/                   # VNT 相关管理类
│   │   └── vnt_manager.dart   # VNT 连接管理器
│   ├── src/                   # 生成的 Rust 绑定代码
│   │   └── rust/              # Flutter-Rust 桥接代码
│   ├── widgets/               # 自定义 UI 组件
│   ├── utils/                 # 工具类
│   ├── *_page.dart           # 各个页面组件
│   ├── network_config.dart    # 网络配置数据模型
│   └── data_persistence.dart  # 数据持久化
├── rust/                       # Rust 核心代码
│   ├── src/
│   │   ├── lib.rs            # Rust 库入口
│   │   └── api/              # FFI API 接口
│   │       ├── mod.rs
│   │       └── vnt_api.rs    # 主要 API 实现
│   └── Cargo.toml            # Rust 依赖配置
├── rust_builder/              # Rust 构建工具
├── assets/                    # 静态资源
│   ├── app_icon.ico
│   ├── app_icon.png
│   └── log4rs.yaml           # 日志配置
├── test_driver/               # 集成测试
├── integration_test/          # 集成测试
├── pubspec.yaml              # Flutter 依赖配置
├── flutter_rust_bridge.yaml  # 桥接配置
└── analysis_options.yaml     # 代码分析配置
```

## 核心功能模块

### 1. VNT 连接管理 (`lib/vnt/vnt_manager.dart`)

-   VPN 连接的创建、启动、停止
-   连接状态监控
-   配置管理

### 2. 网络配置 (`lib/network_config.dart`)

-   服务器地址配置
-   设备信息设置
-   加密算法选择
-   端口映射配置

### 3. UI 组件

-   **主页面** (`lib/main.dart`): 应用入口和主界面
-   **连接页面** (`lib/connected_page.dart`): 显示连接状态和统计信息
-   **配置页面** (`lib/network_config_input_page.dart`): 网络配置输入
-   **设置页面** (`lib/settings_page.dart`): 应用设置
-   **关于页面** (`lib/about_page.dart`): 应用信息

### 4. Rust 核心 (`rust/src/api/vnt_api.rs`)

-   VNT 核心功能封装
-   网络连接处理
-   数据加密/解密
-   跨平台网络接口

## 依赖说明

### Flutter 依赖

```yaml
dependencies:
  flutter: sdk: flutter
  cupertino_icons: ^1.0.6          # iOS 风格图标
  device_info_plus: ^10.0.0        # 设备信息获取
  shared_preferences: ^2.0.0       # 本地数据存储
  flutter_rust_bridge: 2.0.0-dev.37  # Rust 桥接
  path_provider: ^2.1.3            # 路径访问
  yaml: ^3.1.0                     # YAML 解析
  json2yaml: ^3.0.1               # JSON 转 YAML
  url_launcher: ^6.2.6             # URL 启动器
  system_tray: ^2.0.3              # 系统托盘
  bitsdojo_window: ^0.1.6          # 窗口管理
  window_manager: ^0.3.9           # 窗口管理器
  synchronized: ^3.0.0             # 同步工具
  fl_chart: ^0.68.0                # 图表组件
```

### Rust 依赖

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }  # 序列化
flutter_rust_bridge = "=2.0.0-dev.37"              # Flutter 桥接
vnt = { git = "https://github.com/lbl8603/vnt.git", branch = "1.2.x", features = ["ring-cipher", "wss"] }  # VNT 核心库
anyhow = "1.0.82"                                   # 错误处理
tokio = { version = "1.37.0", features = ["full"] } # 异步运行时
log = "0.4.17"                                      # 日志
log4rs = "1.2.0"                                    # 日志配置
```

## 开发环境搭建

### 1. 安装基础环境

```bash
# 安装 Flutter
# 参考: https://docs.flutter.dev/get-started/install

# 安装 Rust
# 参考: https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 flutter_rust_bridge
cargo install flutter_rust_bridge_codegen
```

### 2. 克隆并设置项目

```bash
# 克隆项目 (假设您已有项目代码)
cd VntApp

# 获取 Flutter 依赖
flutter pub get

# 构建 Rust 代码
cd rust
cargo build
cd ..
```

### 3. 生成桥接代码

```bash
# 生成 Flutter-Rust 桥接代码
flutter_rust_bridge_codegen generate
```

## 构建和运行

### 开发模式运行

```bash
# 运行 Flutter 应用 (调试模式)
flutter run

# 指定平台运行
flutter run -d windows
flutter run -d android
flutter run -d macos
flutter run -d linux
```

### 生产构建

```bash
# Windows
flutter build windows

# Android APK
flutter build apk

# Android App Bundle
flutter build appbundle

# macOS
flutter build macos

# Linux
flutter build linux
```

## 开发流程

### 1. 修改 Rust 代码

1. 编辑 `rust/src/api/` 下的文件
2. 重新生成桥接代码: `flutter_rust_bridge_codegen generate`
3. 重新构建: `flutter run`

### 2. 修改 Flutter 代码

1. 编辑 `lib/` 下的 Dart 文件
2. 热重载: `r` (在 flutter run 终端中)
3. 热重启: `R` (在 flutter run 终端中)

### 3. 添加新功能

1. 在 `rust/src/api/vnt_api.rs` 中添加 Rust 函数
2. 使用 `#[flutter_rust_bridge::frb]` 注解标记
3. 重新生成桥接代码
4. 在 Flutter 端调用新函数

## 调试技巧

### 1. 日志调试

-   Rust 端: 使用 `log::debug!()`, `log::info!()` 等
-   Flutter 端: 使用 `debugPrint()`, `print()`
-   日志配置文件: `assets/log4rs.yaml`

### 2. 平台特定调试

```bash
# Android 日志
flutter logs

# 查看设备
flutter devices

# 清理构建缓存
flutter clean
```

### 3. Rust 调试

```bash
cd rust
cargo test        # 运行测试
cargo check       # 检查语法
cargo clippy      # 代码检查
```

## 常见问题

### 1. 桥接代码生成失败

-   确保 Rust 代码编译通过
-   检查 `flutter_rust_bridge.yaml` 配置
-   重新安装 flutter_rust_bridge_codegen

### 2. 平台特定构建问题

-   Windows: 确保安装了 Visual Studio Build Tools
-   Android: 检查 NDK 版本兼容性
-   macOS: 确保 Xcode 版本足够新

### 3. VNT 连接问题

-   检查网络配置是否正确
-   查看日志文件了解详细错误信息
-   确认服务器地址可达

## 发布流程

### 1. 版本管理

-   更新 `pubspec.yaml` 中的版本号
-   更新 `rust/Cargo.toml` 中的版本号
-   创建 Git 标签

### 2. 构建发布版本

```bash
# 清理缓存
flutter clean

# 生产构建
flutter build windows --release
flutter build apk --release
flutter build appbundle --release
```

### 3. 测试

-   在目标平台上测试所有核心功能
-   验证 VPN 连接正常工作
-   检查 UI 响应性和稳定性

## 贡献指南

1. Fork 项目
2. 创建功能分支: `git checkout -b feature/new-feature`
3. 提交更改: `git commit -am 'Add new feature'`
4. 推送分支: `git push origin feature/new-feature`
5. 创建 Pull Request

## 许可证

本项目使用相应的开源许可证，详见 `LICENSE` 文件。

## 致谢

特别感谢 ChatGPT 在本项目开发过程中提供的帮助。

---

_指南最后更新于: 2025 年 7 月 3 日_


