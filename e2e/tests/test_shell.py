"""Tests for shell activation functionality."""

import subprocess
from pathlib import Path


def test_activate_nonexistent_environment(test_binary: str, temp_config_file: Path) -> None:
    """Test activating nonexistent environment fails."""
    result = subprocess.run(
        [test_binary, "--config-file", str(temp_config_file), "shell", "nonexistent"],
        capture_output=True,
        text=True,
        timeout=10,
    )
    
    assert result.returncode != 0
    assert "does not exist" in result.stderr.lower()


def test_activate_with_no_environment_set(test_binary: str, temp_config_file: Path) -> None:
    """Test activate with no environment specified and no global set."""
    result = subprocess.run(
        [test_binary, "--config-file", str(temp_config_file), "shell"],
        capture_output=True,
        text=True,
        timeout=10,
    )
    
    assert result.returncode != 0
    assert "no environment specified" in result.stderr.lower()


