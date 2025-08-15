"""Basic CLI tests for claude-code."""

import subprocess
import toml
from pathlib import Path

from .types import INPUT_TYPE


def test_help_command(test_binary: str) -> None:
    """Test that help command works."""
    result = subprocess.run(
        [test_binary, "--help"], capture_output=True, text=True, timeout=10
    )

    assert result.returncode == 0
    assert "Usage:" in result.stdout
    assert "claude-code" in result.stdout


def test_version_command(test_binary: str) -> None:
    """Test that version command works."""
    result = subprocess.run(
        [test_binary, "--version"], capture_output=True, text=True, timeout=10
    )

    assert result.returncode == 0
    assert result.stdout.strip()


def test_list_environments_empty(test_binary: str, temp_config_file: Path) -> None:
    """Test listing environments when none exist."""
    result = subprocess.run(
        [test_binary, "--config-file", str(temp_config_file), "envs"],
        capture_output=True,
        text=True,
        timeout=10,
    )

    assert result.returncode == 0
    # Should show empty or no environments message
    assert "No environments" in result.stdout or not result.stdout.strip()
    assert "ccv create" in result.stdout or not result.stdout.strip()


def test_create_environment(
    temp_config_file: Path, create_environment, mock_input_data: INPUT_TYPE
) -> None:
    """Test creating a new environment."""
    env_name = "test-env"
    endpoint_name, token, test_model, test_fastmodel, _ = mock_input_data

    # Create environment using the new fixture
    create_environment(
        env_name=env_name,
        endpoint=endpoint_name,
        token=token,
        model=test_model,
        fast_model=test_fastmodel
    )

    # Verify environment was created
    assert temp_config_file.exists()

    with open(temp_config_file) as f:
        config = toml.load(f)

    assert "environments" in config
    assert env_name in config["environments"]
    assert config["environments"][env_name]["variables"] == {
        "ANTHROPIC_MODEL": test_model,
        "ANTHROPIC_BASE_URL": endpoint_name,
        "ANTHROPIC_AUTH_TOKEN": token,
        "ANTHROPIC_SMALL_FAST_MODEL": test_fastmodel,
    }


def test_create_duplicate_environment(
    test_binary: str, temp_config_file: Path, create_environment, mock_input_data: INPUT_TYPE
) -> None:
    """Test creating duplicate environment fails gracefully."""
    env_name = "test-env"
    endpoint_name, token, test_model, test_fastmodel, _ = mock_input_data

    # Create environment first using the fixture
    create_environment(
        env_name=env_name,
        endpoint=endpoint_name,
        token=token,
        model=test_model,
        fast_model=test_fastmodel
    )

    # Try to create duplicate using subprocess (should fail)
    result = subprocess.run(
        [test_binary, "--config-file", str(temp_config_file), "create", env_name],
        capture_output=True,
        text=True,
        timeout=10,
    )

    assert result.returncode != 0
    assert (
        "already exists" in result.stderr.lower()
        or "duplicate" in result.stderr.lower()
    )


def test_invalid_command(test_binary: str) -> None:
    """Test handling of invalid commands."""
    result = subprocess.run(
        [test_binary, "invalid-command"], capture_output=True, text=True, timeout=10
    )

    assert result.returncode != 0
    assert "error" in result.stderr.lower() or "unknown" in result.stderr.lower()
