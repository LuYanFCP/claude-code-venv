"""Tests for environment management functionality."""

import subprocess
from pathlib import Path

import toml

from .types import INPUT_TYPE


def test_set_current_environment(
    test_binary: str,
    temp_config_file: Path,
    create_environment,
    mock_input_data: INPUT_TYPE,
) -> None:
    """Test setting current environment."""
    env_name = "current-env"
    endpoint_name, token, test_model, test_fastmodel, _ = mock_input_data

    # Create environment using the fixture
    create_environment(
        env_name=env_name,
        endpoint=endpoint_name,
        token=token,
        model=test_model,
        fast_model=test_fastmodel,
    )

    # Set as current
    result = subprocess.run(
        [test_binary, "--config-file", str(temp_config_file), "global", env_name],
        capture_output=True,
        text=True,
        timeout=10,
    )

    assert result.returncode == 0

    # Verify current environment was set
    with open(temp_config_file) as f:
        config = toml.load(f)

    assert "global_env" in config
    assert config["global_env"] == env_name


def test_set_nonexistent_environment(
    test_binary: str, temp_config_file: Path, mock_input_data: INPUT_TYPE
) -> None:
    """Test setting nonexistent environment fails."""
    result = subprocess.run(
        [test_binary, "--config-file", str(temp_config_file), "global", "nonexistent"],
        capture_output=True,
        text=True,
        timeout=10,
    )

    assert result.returncode != 0
    assert "does not exist" in result.stderr.lower()


def test_remove_environment(
    test_binary: str,
    temp_config_file: Path,
    create_environment,
    remove_environment,
    mock_input_data: INPUT_TYPE,
) -> None:
    """Test removing environment."""
    env_name = "to-remove"
    endpoint_name, token, test_model, test_fastmodel, _ = mock_input_data

    # Create environment using the fixture
    create_environment(
        env_name=env_name,
        endpoint=endpoint_name,
        token=token,
        model=test_model,
        fast_model=test_fastmodel,
    )

    # Remove environment
    result = remove_environment(env_name, True)
    assert "removed" in result

    # Verify environment was removed
    with open(temp_config_file) as f:
        config = toml.load(f)

    assert env_name not in config.get("environments", {})


def test_remove_current_environment(
    test_binary: str,
    temp_config_file: Path,
    create_environment,
    mock_input_data: INPUT_TYPE,
    remove_environment,
) -> None:
    """Test removing current environment unsets current."""
    env_name = "current-to-remove"
    endpoint_name, token, test_model, test_fastmodel, _ = mock_input_data

    # Create and set as current
    create_environment(
        env_name=env_name,
        endpoint=endpoint_name,
        token=token,
        model=test_model,
        fast_model=test_fastmodel,
        set_current=True,
    )
    subprocess.run(
        [test_binary, "--config-file", str(temp_config_file), "global", env_name],
        capture_output=True,
        timeout=10,
    )

    # Verify it's current
    with open(temp_config_file) as f:
        config = toml.load(f)
    assert config.get("global_env") == env_name

    # Remove environment
    remove_environment(env_name, True)

    # Verify current was unset
    with open(temp_config_file) as f:
        config = toml.load(f)

    assert config.get("global_env") != env_name


def test_list_environments_with_data(
    test_binary: str,
    temp_config_file: Path,
    create_environment,
    mock_input_data: INPUT_TYPE,
) -> None:
    """Test listing environments with data."""
    env_names = ["env1", "env2", "env3"]
    endpoint_name, token, test_model, test_fastmodel, _ = mock_input_data

    # Create multiple environments using the fixture
    for name in env_names:
        create_environment(
            env_name=name,
            endpoint=endpoint_name,
            token=token,
            model=test_model,
            fast_model=test_fastmodel,
        )

    # List environments
    result = subprocess.run(
        [test_binary, "--config-file", str(temp_config_file), "envs"],
        capture_output=True,
        text=True,
        timeout=10,
    )

    assert result.returncode == 0

    # Verify all environments are listed
    for name in env_names:
        assert name in result.stdout
