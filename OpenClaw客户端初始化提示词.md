# OpenClaw 客户端初始化提示词

> **角色**：你是一个经验丰富的全栈开发者，正在为 OpenClaw 开发一个降低使用门槛的桌面客户端。你的核心任务是让用户安装后无需任何配置即可使用 OpenClaw 的全部功能。

## 核心指令
1. **预置默认配置**：在客户端安装包中内置 OpenClaw 的默认配置文件
2. **自动化初始化**：客户端首次启动时自动完成 OpenClaw 初始化流程
3. **零用户干预**：用户无需运行任何命令行工具或回答交互式问题

## 具体实现方案

### 1. 预置默认配置文件
在客户端资源目录中包含以下文件：
```
resources/
├── openclaw.json          # 主配置文件
├── MEMORY.md             # 默认长期记忆
├── USER.md               # 用户信息模板
├── SOUL.md               # AI 助手人格模板
└── skills/               # 预装技能包
    ├── dida/
    ├── obsidian/
    └── summarize/
```

#### openclaw.json 内容示例：
```json
{
  "model": "bailian/qwen3-max-2026-01-23",
  "defaultModel": "bailian/qwen3-max-2026-01-23",
  "workspace": "~/.openclaw/workspace",
  "gatewayPort": 18789,
  "channels": {
    "telegram": {
      "enabled": false
    }
  },
  "memory": {
    "enabled": true,
    "engine": "qmd"
  }
}
```

### 2. 自动化初始化流程
客户端首次启动时执行以下步骤：

#### Rust 后端逻辑（src-tauri/src/main.rs）：
```rust
#[tauri::command]
async fn initialize_openclaw() -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("无法获取用户目录")?;
    let openclaw_dir = home_dir.join(".openclaw");
    let workspace_dir = openclaw_dir.join("workspace");
    
    // 1. 创建目录结构
    std::fs::create_dir_all(&workspace_dir).map_err(|e| e.to_string())?;
    
    // 2. 复制预置配置文件
    let config_src = std::path::Path::new("resources/openclaw.json");
    let config_dst = openclaw_dir.join("openclaw.json");
    std::fs::copy(config_src, config_dst).map_err(|e| e.to_string())?;
    
    // 3. 复制默认记忆文件
    let memory_files = ["MEMORY.md", "USER.md", "SOUL.md", "AGENTS.md"];
    for file in memory_files.iter() {
        let src = std::path::Path::new(&format!("resources/{}", file));
        let dst = workspace_dir.join(file);
        if src.exists() {
            std::fs::copy(src, dst).map_err(|e| e.to_string())?;
        }
    }
    
    // 4. 复制预装技能包
    let skills_src = std::path::Path::new("resources/skills");
    let skills_dst = workspace_dir.join("skills");
    if skills_src.exists() {
        copy_dir_recursive(skills_src, &skills_dst).map_err(|e| e.to_string())?;
    }
    
    // 5. 启动 Gateway 服务
    let status = Command::new("bin/openclaw")
        .args(["gateway", "status"])
        .output()
        .map_err(|e| e.to_string())?;
    
    if !status.status.success() {
        Command::new("bin/openclaw")
            .args(["gateway", "--background"])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok("OpenClaw 初始化完成".to_string())
}
```

### 3. 前端调用逻辑
```svelte
<script>
import { invoke } from '@tauri-apps/api/tauri';

async function onAppStart() {
  // 检查是否已初始化
  const initialized = await checkInitialization();
  if (!initialized) {
    try {
      await invoke('initialize_openclaw');
      showSuccessMessage();
    } catch (error) {
      showErrorMessage(error);
    }
  }
}

// 应用启动时调用
onAppStart();
</script>
```

## 质量要求
- **初始化时间**：< 5 秒（包括 Gateway 启动）
- **磁盘占用**：< 50MB（含预装技能包）
- **兼容性**：支持 macOS 12+、Windows 10+、主流 Linux
- **错误处理**：网络离线时仍能完成本地初始化

## 禁止行为
- ❌ 要求用户手动运行 `openclaw configure`
- ❌ 弹出终端窗口执行命令
- ❌ 依赖用户已安装 Node.js/npm

## 成功标准
普通用户安装客户端后：
1. 双击打开应用
2. 看到加载进度条（<5秒）
3. 直接进入主界面，可立即使用所有功能
4. 完全不知道 OpenClaw/CLI/Gateway 的存在