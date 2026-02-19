---
name: simplify-init-with-onboard
overview: 精简初始化流程，改用 zeroclaw onboard 命令自动生成配置和 workspace 结构，删除项目中手动维护的资源模板文件，大幅简化代码。
design:
  architecture:
    framework: vue
todos:
  - id: fix-config-schema
    content: 修复 config.toml schema 不匹配问题（缺少 auto_save 等字段），改为调用 zeroclaw onboard 生成正确配置
    status: completed
  - id: refactor-init
    content: 重构 init.rs，初始化流程简化为：安装二进制 → 调用 zeroclaw onboard（幂等，已有 config 时跳过）
    status: completed
    dependencies:
      - fix-config-schema
  - id: remove-templates
    content: 删除 resources/ 下的 MEMORY.md、USER.md、SOUL.md、config.toml、skills/ 目录，全部由 zeroclaw onboard 生成
    status: completed
    dependencies:
      - refactor-init
  - id: cleanup-code
    content: 清理 fs.rs 中不再需要的 copy_dir_recursive 和 copy_file_if_not_exists 函数，简化 tauri.conf.json resources 配置
    status: completed
    dependencies:
      - remove-templates
  - id: update-tests
    content: 更新 init.rs 测试，使用 mock shell 脚本模拟 zeroclaw onboard 行为，Rust 12 + 前端 17 = 29 个测试全部通过
    status: completed
    dependencies:
      - cleanup-code
---

## 背景

手动维护的 `config.toml` 与 zeroclaw 二进制期望的 schema 不一致（缺少 `[memory].auto_save` 等字段），导致 `service status` 命令解析失败。

核心思路：既然 zeroclaw 提供了 `onboard` 命令来生成完整的配置和 workspace 结构，就不需要我们自己维护这些模板文件了。

## 变更内容

### 1. 删除的资源文件

| 文件 | 原因 |
|------|------|
| `resources/MEMORY.md` | zeroclaw onboard 自动生成 |
| `resources/USER.md` | zeroclaw onboard 自动生成 |
| `resources/SOUL.md` | zeroclaw onboard 自动生成 |
| `resources/config.toml` | zeroclaw onboard 自动生成（且 schema 匹配） |
| `resources/skills/` | zeroclaw onboard 自动生成 |

### 2. 现在的 resources 目录

```
resources/
└── bin/
    └── zeroclaw   (16.5MB, 唯一需要维护的资源)
```

### 3. 简化后的初始化流程（2 步）

1. **安装二进制**：`resources/bin/zeroclaw` → `~/.zeroclaw/bin/zeroclaw`
2. **运行 onboard**：`zeroclaw onboard --provider bailian --model qwen3-max-2026-01-23 --memory sqlite`
   - 幂等：已存在 config.toml 时自动跳过
   - 自动生成：config.toml、workspace 目录、MEMORY.md、USER.md、SOUL.md、skills/
   - 自动 chmod 600 config.toml（修复权限警告）

### 4. 代码清理

- `init.rs`：从 ~100 行精简到 ~80 行，删除模板复制和 skills 复制逻辑
- `fs.rs`：删除 `copy_dir_recursive` 和 `copy_file_if_not_exists` 函数（不再需要）
- `tauri.conf.json`：resources 配置从 3 项精简为 `["resources/bin/*"]`
- 测试：使用 mock shell 脚本模拟 onboard 命令，保持测试覆盖

### 5. 用户定制方式

需要定制化时直接去 `~/.zeroclaw/` 下修改对应文件即可，初始化不会覆盖已有文件。
