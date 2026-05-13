import pytest

from tucana import load_protocol, AVAILABLE_PROTOCOLS


def test_available_protocols():
    """Test that AVAILABLE_PROTOCOLS is defined."""
    assert AVAILABLE_PROTOCOLS == ("shared", "aquila", "sagittarius", "velorum")


def test_load_shared():
    """Test loading shared protocol."""
    load_protocol("shared")
    from tucana.generated import shared  # noqa: F401


def test_load_aquila():
    """Test loading aquila protocol."""
    load_protocol("aquila")
    from tucana.generated import aquila  # noqa: F401


def test_load_sagittarius():
    """Test loading sagittarius protocol."""
    load_protocol("sagittarius")
    from tucana.generated import sagittarius  # noqa: F401


def test_load_velorum():
    """Test loading velorum protocol."""
    load_protocol("velorum")
    from tucana.generated import velorum  # noqa: F401


def test_load_invalid_protocol():
    """Test loading invalid protocol raises error."""
    with pytest.raises(ValueError, match="Unknown protocol"):
        load_protocol("invalid")
