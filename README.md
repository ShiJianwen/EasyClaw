# EasyClaw

ZeroClaw 桌面客户端 — 零配置、开箱即用。

用户安装后双击打开即可使用 ZeroClaw 全部功能，无需任何命令行操作或手动配置。

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri v2 |
| 前端 | Vue 3 + TypeScript |
| 路由 | Vue Router 4 (Hash Mode) |
| 构建工具 | Vite 5 |
| 样式 | Tailwind CSS 3.4 |
| 图标 | lucide-vue-next |
| 前端测试 | Vitest + @vue/test-utils |
| 后端 | Rust |
| 后端测试 | cargo test + tempfile |
| 包管理器 | Yarn |
| Gateway | ZeroClaw 0.1.0（二进制内嵌） |

## 核心特性

- **对话助手**: 与 AI 助手自然对话，通过 SSE 流式通信实时显示回复
- **零配置启动**: 内嵌 ZeroClaw 二进制，首次启动自动调用 `zeroclaw onboard` 完成初始化
- **自动 Gateway 管理**: 通过 `zeroclaw service install/start/status` 管理后台服务
- **幂等初始化**: 安全的重复初始化，已有 config.toml 时自动跳过 onboard
- **优雅降级**: Gateway 启动失败不阻塞主界面使用
- **用户可定制**: 配置和模板文件由 ZeroClaw 生成在 `~/.zeroclaw/`，可直接修改

## 项目结构

```
EasyClaw/
├── src/                        # Vue 前端源码
│   ├── App.vue                 # 根组件 (Splash ↔ AppLayout 切换)
│   ├── router/
│   │   └── index.ts            # 路由配置 (Hash Mode: / → HomePage, /chat → ChatView)
│   ├── components/
│   │   ├── SplashScreen.vue    # 启动初始化页
│   │   ├── AppLayout.vue       # 全局布局壳 (导航栏 + router-view + 状态栏)
│   │   ├── HomePage.vue        # 首页 (欢迎横幅 + 功能卡片导航)
│   │   └── ChatView.vue        # 对话助手页 (消息列表 + 输入框 + SSE 流式)
│   ├── composables/
│   │   ├── useInitialization.ts # 初始化逻辑
│   │   └── useChat.ts          # 对话逻辑 (SSE 流式通信 + AbortController)
│   └── styles/
│       └── main.css            # Tailwind + 全局样式 + chat 气泡样式
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── commands/
│   │   │   ├── init.rs         # 初始化命令 (安装二进制 + zeroclaw onboard)
│   │   │   └── gateway.rs      # Gateway 管理 (service install/start/status)
│   │   └── utils/
│   │       ├── fs.rs           # 二进制安装工具
│   │       └── paths.rs        # 路径工具
│   └── resources/
│       └── bin/
│           └── zeroclaw        # ZeroClaw 二进制 (Mach-O arm64, ~16.5MB)
├── tests/                      # 前端测试
│   ├── components/
│   │   ├── SplashScreen.spec.ts
│   │   ├── AppLayout.spec.ts
│   │   ├── HomePage.spec.ts
│   │   └── ChatView.spec.ts
│   └── composables/
│       ├── useInitialization.spec.ts
│       └── useChat.spec.ts
└── package.json
```

## 开发环境搭建

### 前置要求

- **Node.js** >= 20
- **Rust** >= 1.77 (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- **Yarn** (`npm install -g yarn`)
- **Tauri CLI** (`cargo install tauri-cli`)

### 安装依赖

```bash
yarn install
```

### 启动开发模式

```bash
source "$HOME/.cargo/env"  # 确保 Rust 在 PATH 中
yarn tauri dev
```

前端热更新运行在 `http://localhost:1420`，Rust 后端变更会自动重新编译。

## 运行测试

### 前端测试

```bash
yarn test              # 运行一次
yarn test:watch        # 监听模式
```

### Rust 后端测试

```bash
cd src-tauri
cargo test
```

## 构建发布

```bash
yarn tauri build
```

产物位于 `src-tauri/target/release/bundle/`，支持：
- macOS: `.dmg` / `.app`
- Windows: `.msi` / `.exe`
- Linux: `.deb` / `.AppImage`

## 初始化流程

```
应用启动
  ↓
检查 ~/.zeroclaw/config.toml 是否存在
  ├── 存在 → 直接进入主界面
  └── 不存在 → 执行初始化:
        1. 安装二进制: resources/bin/zeroclaw → ~/.zeroclaw/bin/zeroclaw
        2. 运行 zeroclaw onboard (生成 config.toml + workspace 全套文件)
        3. 启动 Gateway: service install → service start (失败不阻塞)
        ↓
      进入主界面
```

### 用户目录结构（由 zeroclaw onboard 生成）

```
~/.zeroclaw/
├── bin/
│   └── zeroclaw           # 二进制
├── config.toml            # 主配置 (chmod 600)
└── workspace/
    ├── MEMORY.md           # 长期记忆
    ├── USER.md             # 用户信息
    ├── SOUL.md             # AI 人格
    ├── sessions/           # 会话记录
    ├── memory/             # 记忆存储
    └── skills/             # 技能包
```

## License

MIT
