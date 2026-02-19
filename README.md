# EasyClaw

OpenClaw 桌面客户端 — 零配置、开箱即用。

用户安装后双击打开即可使用 OpenClaw 全部功能，无需任何命令行操作或手动配置。

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri v2 |
| 前端 | Vue 3 + TypeScript |
| 构建工具 | Vite 5 |
| 样式 | Tailwind CSS 3.4 |
| 图标 | lucide-vue-next |
| 前端测试 | Vitest + @vue/test-utils |
| 后端 | Rust |
| 后端测试 | cargo test + tempfile |
| 包管理器 | Yarn |

## 核心特性

- **零配置启动**: 内置预设配置，首次启动自动完成初始化
- **自动 Gateway 管理**: 自动启动和管理 OpenClaw Gateway 服务
- **幂等初始化**: 安全的重复初始化，不覆盖用户已修改的文件
- **优雅降级**: Gateway 启动失败不阻塞主界面使用
- **预装技能包**: 内置滴答清单、Obsidian、摘要等技能包

## 项目结构

```
EasyClaw/
├── src/                        # Vue 前端源码
│   ├── App.vue                 # 根组件 (Splash ↔ Main 切换)
│   ├── components/
│   │   ├── SplashScreen.vue    # 启动初始化页
│   │   └── MainView.vue        # 主界面
│   ├── composables/
│   │   └── useInitialization.ts # 初始化逻辑
│   └── styles/
│       └── main.css            # Tailwind + 全局样式
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── commands/
│   │   │   ├── init.rs         # 初始化命令 (check/initialize)
│   │   │   └── gateway.rs      # Gateway 管理命令
│   │   └── utils/
│   │       ├── fs.rs           # 递归目录复制
│   │       └── paths.rs        # 路径工具
│   └── resources/              # 预置资源文件
│       ├── openclaw.json       # 主配置
│       ├── MEMORY.md / USER.md / SOUL.md
│       └── skills/             # 预装技能包
├── tests/                      # 前端测试
│   ├── components/
│   │   ├── SplashScreen.spec.ts
│   │   └── MainView.spec.ts
│   └── composables/
│       └── useInitialization.spec.ts
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
检查 ~/.openclaw/openclaw.json 是否存在
  ├── 存在 → 直接进入主界面
  └── 不存在 → 执行初始化:
        1. 创建 ~/.openclaw/workspace/
        2. 复制 openclaw.json → ~/.openclaw/
        3. 复制模板文件 → workspace/
        4. 复制技能包 → workspace/skills/
        5. 启动 Gateway (失败不阻塞)
        ↓
      进入主界面
```

## License

MIT
