# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

**KeePass One** 是一个基于Flutter的密码管理器应用，支持KeePass数据库文件(.kdbx)的集成。这是一个复杂的混合移动应用，结合了Flutter的UI能力和Rust的性能及安全特性用于加密操作。

## 常用命令

### 开发命令
```bash
# 安装依赖
flutter pub get

# 代码生成（必须）
make build_runner
# 或
dart run build_runner build --delete-conflicting-outputs

# 运行应用
flutter run

# 运行测试
flutter test
```

### 代码生成相关
- 使用`build_runner`生成Drift数据库类、Freezed不可变类和JSON序列化代码
- Rust代码通过flutter_rust_bridge自动构建
- 生成的文件在git控制外，但必须在使用前生成

## 架构设计

### 1. 混合架构模式
- **Flutter层**: 处理UI、状态管理和业务逻辑
- **Rust层**: 提供高性能加密操作和数据处理
- **Flutter Rust Bridge**: 实现Dart和Rust间的无缝通信

### 2. 状态管理
- **Riverpod**: 主要状态管理解决方案
- **Provider**: 用于本地组件状态(如FilePickerProvider)
- **GetIt**: 服务的依赖注入

### 3. 数据库层
- **Drift ORM**: 本地SQLite数据库的类型安全ORM
- **架构**: 管理KDBX数据库元数据（配置、同步状态、时间戳）

### 4. 同步架构
- **工厂模式**: 用于同步驱动程序
- **多源支持**: 本地文件、WebDAV等
- **可扩展设计**: 支持SFTP、OneDrive、S3等未来扩展

## 核心依赖

### Flutter主要依赖
- `flutter_riverpod`: 状态管理
- `drift`: 数据库ORM
- `flutter_rust_bridge`: Rust集成
- `webdav_client`: WebDAV文件同步
- `file_picker`: 文件选择界面
- `dio`: HTTP客户端
- `cupertino_icons`: iOS风格UI组件

### Rust加密依赖
- `aes`, `twofish`, `chacha20`: 加密算法
- `sha1`, `sha2`: 哈希函数
- `rust-argon2`: 密钥派生
- `zeroize`: 安全内存处理

## 目录结构

```
lib/
├── main.dart                 # 应用入口
├── pages/                    # UI页面
│   ├── home.dart            # 主页面
│   ├── db_index/            # 数据库索引
│   └── db_add/              # 数据库添加流程
├── services/                 # 业务逻辑
│   ├── database/            # 本地数据库操作
│   ├── sync/                # 同步驱动程序
│   └── file_system/         # 文件系统抽象
├── widgets/                  # 可重用UI组件
│   ├── file_picker/         # 自定义文件选择器
│   └── sheet.dart           # 模态框组件
├── app/                      # 应用级服务
├── utils/                    # 工具函数
└── src/rust/                 # 生成的Rust绑定
```

## 数据流架构

1. **初始化**: Rust库 → GetIt服务 → Riverpod提供者
2. **数据库操作**: UI → Riverpod → Drift ORM → SQLite
3. **文件操作**: UI → FileSystemProvider → SyncDriver → 外部服务
4. **加密操作**: UI → Dart → Flutter Rust Bridge → Rust函数

## 开发注意事项

### 代码生成
- 所有Drift数据库类都需要使用`build_runner`生成（`*.g.dart`文件）
- Freezed不可变类需要生成（`*.freezed.dart`文件）
- JSON序列化代码需要生成（`*.g.dart`文件）

### 文件选择器
- 使用自定义FilePicker组件支持.kdbx文件验证
- 支持本地文件和WebDAV远程文件选择
- 包含进度跟踪和错误处理

### 同步功能
- WebDAV同步已实现
- 使用工厂模式支持多种同步驱动
- 完整的错误处理和异常类型

### UI模式
- 使用Cupertino设计系统
- 基于导航器的路由
- 模态框用于文件选择

## 实现状态

基于代码分析，当前实现状态：
- ✅ 基础应用结构和导航
- ✅ 带Drift ORM的数据库层
- ✅ WebDAV同步实现
- ✅ 文件选择器组件
- ✅ Rust集成基础
- 🔄 UI屏幕开发中（许多显示占位符内容）
- 📋 KeePass格式解析（Rust层已准备就绪）

## 安全考虑

- Rust加密层使用`zeroize`进行内存安全操作
- Flutter层使用Drift进行安全数据库存储
- 安全的文件处理实践
- 加密配置存储

## 测试策略
- 工具函数的单元测试
- 配置的集成测试
- Widget测试结构已就位