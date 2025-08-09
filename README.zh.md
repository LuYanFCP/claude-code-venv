# claude-code-venv

一个类似 pyenv 的 Claude Code 环境管理器，专为管理不同 Claude Code 配置而设计。

## 功能特性

- **环境管理**: 创建、列出和管理多个 Claude Code 环境
- **Anthropic 专用配置**: 交互式设置 Anthropic API 变量
- **全局默认**: 设置全局默认环境
- **Shell 集成**: 直接在 shell 中激活环境
- **跨平台支持**: 支持类 Unix 系统和 Windows

## 安装

```bash
# 直接从 GitHub 安装
cargo install --git https://github.com/LuYanFCP/claude-code-venv

# 安装后，二进制文件将以 `ccv` 命令提供
ccv --help
```

## 快速开始

### 创建环境

```bash
# 创建包含 Anthropic 专用变量的新环境
ccv create my-env

# 交互式提示：
# - 环境名称: my-env
# - ANTHROPIC_BASE_URL: https://api.anthropic.com
# - ANTHROPIC_AUTH_TOKEN: 你的令牌
# - ANTHROPIC_MODEL: claude-3-5-sonnet-20241022
# - ANTHROPIC_SMALL_FAST_MODEL: claude-3-haiku-20240307
```

### 列出环境

```bash
# 列出所有可用环境
ccv envs
```

### 设置全局环境

```bash
# 设置全局默认环境
ccv global my-env
```

### 激活环境

```bash
# 激活特定环境
ccv shell my-env

# 激活全局/默认环境
ccv shell
```

### 查看当前环境

```bash
# 显示当前激活的环境
ccv current
```

## 使用示例

### 完整工作流

```bash
# 1. 创建 Anthropic 环境
ccv create anthropic-prod

# 2. 设置为全局默认
ccv global anthropic-prod

# 3. 在新 shell 中激活
ccv shell

# 4. 验证环境变量
env | grep ANTHROPIC
```

### 多环境管理

```bash
# 创建开发环境
ccv create anthropic-dev

# 创建预发布环境
ccv create anthropic-staging

# 在环境间切换
ccv shell anthropic-dev
ccv shell anthropic-staging
```

## 命令参考

| 命令 | 描述 |
|---------|-------------|
| `ccv envs` | 列出所有环境 |
| `ccv create [name]` | 创建新环境 |
| `ccv global <name>` | 设置全局环境 |
| `ccv shell [name]` | 激活环境 |
| `ccv current` | 显示当前环境 |
| `ccv remove <name>` | 移除环境 |

## 环境变量

创建环境时会配置以下变量：

- `ANTHROPIC_BASE_URL` - API 调用的基础 URL
- `ANTHROPIC_AUTH_TOKEN` - 认证令牌
- `ANTHROPIC_MODEL` - 主要使用的模型
- `ANTHROPIC_SMALL_FAST_MODEL` - 快速响应的模型

## 配置文件

默认配置文件：`~/.claude-code-env.toml`

## 贡献

欢迎贡献！请随时提交问题和拉取请求。

## 许可证

MIT 许可证 - 详见 LICENSE 文件