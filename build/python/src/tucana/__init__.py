"""Tucana - Code0 GRPC Protocol Python Library"""

__version__ = "0.0.0"

# Lazy protocol loading
def load_protocol(protocol: str) -> None:
    """Load a protocol module.

    Args:
        protocol: Protocol name (shared, aquila, sagittarius, velorum)

    Raises:
        ValueError: If protocol name is unknown
    """
    if protocol == "shared":
        from . import generated  # noqa: F401
        from .generated import shared  # noqa: F401
    elif protocol == "aquila":
        from . import generated  # noqa: F401
        from .generated import aquila  # noqa: F401
    elif protocol == "sagittarius":
        from . import generated  # noqa: F401
        from .generated import sagittarius  # noqa: F401
    elif protocol == "velorum":
        from . import generated  # noqa: F401
        from .generated import velorum  # noqa: F401
    else:
        raise ValueError(f"Unknown protocol: {protocol}")

AVAILABLE_PROTOCOLS = ("shared", "aquila", "sagittarius", "velorum")

__all__ = [
    "load_protocol",
    "AVAILABLE_PROTOCOLS",
]

