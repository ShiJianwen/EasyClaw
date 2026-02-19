---
name: zeroclaw-binary-embed
overview: 将 ZeroClaw 二进制文件内嵌到项目中，修复 Gateway 启动和状态检查命令，使其匹配 zeroclaw CLI 的实际子命令结构。
design:
  architecture:
    framework: vue
todos:
  - id: evaluate-integration
    content: 评估 ZeroClaw 源码集成 vs 二进制内嵌方案，决定采用二进制内嵌（松耦合、独立升级、编译速度快）
    status: completed
  - id: embed-binary
    content: 将 zeroclaw 二进制（Mach-O arm64, 16.5MB）放置到 src-tauri/resources/bin/zeroclaw，验证可执行性
    status: completed
    dependencies:
      - evaluate-integration
  - id: fix-gateway-commands
    content: 修复 gateway.rs 中的命令调用，serve --status → service status，serve --background → service install + service start
    status: completed
    dependencies:
      - embed-binary
  - id: verify-tests
    content: 运行 Rust 16 + 前端 17 = 33 个测试全部通过，zeroclaw --version 正常输出 0.1.0
    status: completed
    dependencies:
      - fix-gateway-commands
---

## 背景

项目需要集成 ZeroClaw 作为 Gateway 后台服务。评估了两种方案：

1. **源码集成**（作为 Rust crate 依赖）：ZeroClaw 有 `lib.rs`，技术上可行，但未发布到 crates.io，依赖链庞大，API 不稳定
2. **二进制内嵌**（当前方案）：将编译好的 zeroclaw 二进制打包进 Tauri 应用

最终选择二进制内嵌方案，理由：解耦、升级灵活、编译速度快、ZeroClaw 还不稳定。

## 变更内容

### 1. 二进制文件

- 路径：`src-tauri/resources/bin/zeroclaw`
- 格式：Mach-O arm64，约 16.5MB
- 版本：zeroclaw 0.1.0

### 2. gateway.rs 命令修复

ZeroClaw CLI 实际子命令结构与预期不同：

| 预期命令 | 实际命令 |
|---------|---------|
| `serve --status` | `service status` |
| `serve --background` | `service install` + `service start` |

修改了 `gateway_status()` 和 `start_gateway()` 函数中的命令调用。

### 3. Gateway 启动流程

```
service install → 注册到 launchd/systemd（幂等）
service start   → 启动服务
service status  → 检查运行状态
```
