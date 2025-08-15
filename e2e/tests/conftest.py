"""Pytest configuration and fixtures for e2e tests using real filesystem."""

import os
import tempfile
from pathlib import Path
from typing import Generator

import pytest
import pexpect

from .types import INPUT_TYPE

@pytest.fixture
def temp_home_dir() -> Generator[Path, None, None]:
    """Create a temporary home directory for testing."""
    with tempfile.TemporaryDirectory() as temp_dir:
        original_home = os.environ.get("HOME")
        original_userprofile = os.environ.get("USERPROFILE")
        
        # Set HOME to temp directory
        os.environ["HOME"] = temp_dir
        os.environ["USERPROFILE"] = temp_dir
        
        try:
            yield Path(temp_dir)
        finally:
            # Restore original environment
            if original_home:
                os.environ["HOME"] = original_home
            else:
                os.environ.pop("HOME", None)
                
            if original_userprofile:
                os.environ["USERPROFILE"] = original_userprofile
            else:
                os.environ.pop("USERPROFILE", None)


@pytest.fixture
def temp_config_file(temp_home_dir: Path) -> Path:
    """Create a temporary config file path for testing."""
    return temp_home_dir / "test_config.toml"


@pytest.fixture
def test_binary() -> str:
    """Get path to the built binary."""
    import sys
    
    # Determine binary extension based on platform
    extension = ".exe" if sys.platform == "win32" else ""
    
    # Try to find the binary in common locations
    possible_paths = [
        f"target/debug/ccv{extension}",
        f"target/release/ccv{extension}",
    ]
    
    repo_root = Path(__file__).parent.parent.parent
    
    for path_str in possible_paths:
        binary_path = repo_root / path_str
        if binary_path.exists():
            return str(binary_path.absolute())
    
    # If binary not found, skip the test
    pytest.skip("Binary not found. Please run 'cargo build' first.")


@pytest.fixture
def temp_work_dir() -> Generator[Path, None, None]:
    """Create a temporary working directory."""
    with tempfile.TemporaryDirectory() as temp_dir:
        yield Path(temp_dir)


@pytest.fixture
def mock_input_data() -> INPUT_TYPE:
    endpoint_name = "https://test-endpoint.com"
    token = "test_token"
    test_model = "test_model"
    test_fastmodel = "test_fastmodel"

    input_data = f"{endpoint_name}\n{token}\n{test_model}\n{test_fastmodel}\nN\n"
    return (endpoint_name, token, test_model, test_fastmodel, input_data)


@pytest.fixture
def create_environment(test_binary: str, temp_config_file: Path) -> Generator:
    """Fixture to create environments using pexpect for interactive prompts."""
    
    def _create_environment(
        env_name: str,
        endpoint: str = "https://test-endpoint.com",
        token: str = "test_token",
        model: str = "test_model",
        fast_model: str = "test_fastmodel",
        set_current: bool = False
    ) -> None:
        """Create an environment with the given parameters."""
        cmd = [
            test_binary,
            "--config-file",
            str(temp_config_file),
            "create",
            env_name
        ]
        
        try:
            child = pexpect.spawn(" ".join(cmd), timeout=10)
            
            # Wait for endpoint prompt
            child.sendline(endpoint)
            
            # Wait for token prompt
            child.sendline(token)
            
            # Wait for model prompt
            child.sendline(model)
            
            # Wait for fast model prompt
            child.sendline(fast_model)
            
            # Wait for set current prompt
            response = "Y" if set_current else "N"
            child.sendline(response)
            
            # Wait for completion
            child.expect(pexpect.EOF)
            
            # Check exit status
            child.close()
            if child.exitstatus != 0:
                raise RuntimeError(f"Failed to create environment: {child.before}")
                
        except pexpect.TIMEOUT:
            raise RuntimeError("Timed out waiting for interactive prompts")
        except pexpect.EOF:
            raise RuntimeError("Unexpected EOF during environment creation")
    
    yield _create_environment


@pytest.fixture
def remove_environment(test_binary: str, temp_config_file: Path) -> Generator:
    
    def delete_env(env_name: str, confirm: bool) -> str:
        cmd = [
            test_binary,
            "--config-file",
            str(temp_config_file),
            "remove",
            env_name
        ]
        
        try:
            child = pexpect.spawn(" ".join(cmd), timeout=10)
            child.sendline("Y" if confirm else "N")
            child.expect(pexpect.EOF)
            output = child.before.decode()
            child.close()
        except pexpect.TIMEOUT:
            raise RuntimeError("Timed out waiting for interactive prompts")
        except pexpect.EOF:
            raise RuntimeError("Unexpected EOF during environment creation")
        return output

    yield delete_env