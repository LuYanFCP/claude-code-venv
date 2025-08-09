# claude-code-venv

A Claude Code Environment Manager like pyenv, designed to manage different Claude Code configurations with environment variables.

📖 **中文文档** | [中文README](README.zh.md)

## Features

- **Environment Management**: Create, list, and manage multiple Claude Code environments
- **Anthropic-Specific Configuration**: Interactive setup for Anthropic API variables
- **Global Default**: Set a global default environment
- **Shell Integration**: Activate environments directly in your shell
- **Cross-Platform**: Works on Unix-like systems and Windows

## Installation

```bash
# Install directly from GitHub
cargo install --git https://github.com/LuYanFCP/claude-code-venv

# After installation, the binary will be available as `ccv`
ccv --help
```

## Quick Start

### Create Environment

```bash
# Create new environment with Anthropic-specific variables
ccv create my-env

# Interactive prompts for:
# - Environment name: my-env
# - ANTHROPIC_BASE_URL: https://api.anthropic.com
# - ANTHROPIC_AUTH_TOKEN: your-token-here
# - ANTHROPIC_MODEL: claude-3-5-sonnet-20241022
# - ANTHROPIC_SMALL_FAST_MODEL: claude-3-haiku-20240307
```

### List Environments

```bash
# List all available environments
ccv envs
```

### Set Global Environment

```bash
# Set global default environment
ccv global my-env
```

### Activate Environment

```bash
# Activate specific environment
ccv shell my-env

# Activate global/default environment
ccv shell
```

### Check Current Environment

```bash
# Show current active environment
ccv current
```

## Usage Examples

### Complete Workflow

```bash
# 1. Create Anthropic environment
ccv create anthropic-prod

# 2. Set as global default
ccv global anthropic-prod

# 3. Activate in new shell
ccv shell

# 4. Verify environment variables
env | grep ANTHROPIC
```

### Multi-Environment Management

```bash
# Create development environment
ccv create anthropic-dev

# Create staging environment  
ccv create anthropic-staging

# Switch between environments
ccv shell anthropic-dev
ccv shell anthropic-staging
```

## Command Reference

| Command | Description |
|---------|-------------|
| `ccv envs` | List all environments |
| `ccv create [name]` | Create new environment |
| `ccv global <name>` | Set global environment |
| `ccv shell [name]` | Activate environment |
| `ccv current` | Show current environment |
| `ccv remove <name>` | Remove environment |

## Environment Variables

When you create an environment, the following variables are configured:

- `ANTHROPIC_BASE_URL` - Base URL for API calls
- `ANTHROPIC_AUTH_TOKEN` - Authentication token
- `ANTHROPIC_MODEL` - Primary model to use
- `ANTHROPIC_SMALL_FAST_MODEL` - Fast model for quick responses

## Configuration File

Default configuration file: `~/.claude-code-env.toml`

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

MIT License - see LICENSE file for details