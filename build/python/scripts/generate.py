#!/usr/bin/env python3

import shutil
import subprocess
import sys
from pathlib import Path
from typing import List, Tuple


def get_proto_files(proto_dir: Path) -> Tuple[List[Path], List[Path]]:
    proto_files = []
    proto_dirs = set()

    for proto_file in proto_dir.rglob("*.proto"):
        proto_files.append(proto_file)
        proto_dirs.add(proto_file.parent)

    return proto_files, sorted(list(proto_dirs))


def generate_protos(proto_root: Path, output_dir: Path, proto_dirs: List[Path]) -> None:
    proto_import_paths = [str(d) for d in proto_dirs]

    proto_file_paths = []
    for proto_dir_path in proto_dirs:
        proto_file_paths.extend(str(f) for f in proto_dir_path.glob("*.proto"))

    if not proto_file_paths:
        print("No proto files found!", file=sys.stderr)
        sys.exit(1)

    cmd = [
        "python", "-m", "grpc_tools.protoc",
    ]

    for import_path in proto_import_paths:
        cmd.extend(["-I", import_path])

    cmd.extend([
        f"--python_out={output_dir}",
        f"--grpc_python_out={output_dir}",
    ])

    cmd.extend(proto_file_paths)

    print(f"\n📍 Running protoc...")
    print(f"   Command: {' '.join(cmd[:6])} ...")
    print(f"   Output: {output_dir}")
    print(f"   Proto files: {len(proto_file_paths)}")

    result = subprocess.run(cmd, cwd=str(proto_root.parent), capture_output=True, text=True)

    if result.returncode != 0:
        print(f"❌ protoc failed!", file=sys.stderr)
        print(f"STDOUT:\n{result.stdout}", file=sys.stderr)
        print(f"STDERR:\n{result.stderr}", file=sys.stderr)
        sys.exit(1)

    if result.stderr:
        print(f"⚠️  protoc warnings:\n{result.stderr}")

    print(f"✓ Proto generation successful")


def organize_generated_files(output_dir: Path) -> None:
    protocol_map = {
        "shared": output_dir / "shared",
        "aquila": output_dir / "aquila",
        "sagittarius_gateway": output_dir / "sagittarius_gateway",
        "sagittarius_rails": output_dir / "sagittarius_rails",
        "velorum": output_dir / "velorum",
    }

    for ns_dir in protocol_map.values():
        ns_dir.mkdir(parents=True, exist_ok=True)
        init_file = ns_dir / "__init__.py"
        if not init_file.exists():
            init_file.write_text("")

    for py_file in output_dir.glob("*.py"):
        if py_file.name.startswith("__"):
            continue

        parts = py_file.stem.split("_")
        protocol = None

        if parts[0] in protocol_map:
            protocol = parts[0]

        if protocol and protocol in protocol_map:
            ns_dir = protocol_map[protocol]
            ns_dir.mkdir(parents=True, exist_ok=True)
            dest = ns_dir / py_file.name
            shutil.move(str(py_file), str(dest))
            print(f"Moved {py_file.name} -> {protocol}/{py_file.name}")


def create_init_files(output_dir: Path) -> None:
    for namespace in ["shared", "aquila", "sagittarius_gateway", "sagittarius_rails", "velorum"]:
        ns_dir = output_dir / namespace
        if ns_dir.exists():
            init_file = ns_dir / "__init__.py"
            init_file.write_text(f'''"""Tucana {namespace.capitalize()} Protocol Buffer modules

Auto-generated from proto files. Import specific modules as needed:

    from tucana.generated.{namespace} import data_type_pb2
    from tucana.generated.{namespace} import data_type_pb2_grpc
"""
''')


def main():
    script_dir = Path(__file__).parent
    build_dir = script_dir.parent
    repo_root = build_dir.parent.parent
    proto_root = repo_root / "proto"

    src_dir = build_dir / "src"
    tucana_dir = src_dir / "tucana"
    generated_dir = tucana_dir / "generated"

    print(f"\n🚀 Tucana Python Proto Generator")
    print(f"   Script: {script_dir}")
    print(f"   Proto root: {proto_root}")
    print(f"   Output: {generated_dir}")

    if not proto_root.exists():
        print(f"❌ Proto directory not found: {proto_root}", file=sys.stderr)
        sys.exit(1)

    generated_dir.mkdir(parents=True, exist_ok=True)

    proto_files, proto_dirs = get_proto_files(proto_root)

    if not proto_files:
        print(f"❌ No proto files found in {proto_root}", file=sys.stderr)
        sys.exit(1)

    print(f"✓ Found {len(proto_files)} proto files in {len(proto_dirs)} directories")

    print(f"\n📍 Cleaning old generated files...")
    for py_file in generated_dir.rglob("*.py"):
        if not py_file.name.startswith("__"):
            py_file.unlink()
            print(f"   Removed {py_file.name}")

    generate_protos(proto_root, str(generated_dir), proto_dirs)

    print(f"\n📍 Organizing generated files...")
    organize_generated_files(generated_dir)

    print(f"\n📍 Creating __init__.py files...")
    create_init_files(generated_dir)

    print(f"\n✅ Generation complete!")
    print(f"   Output directory: {generated_dir}")
    print(f"   Ready to import: from tucana.generated import shared, aquila, sagittarius_gateway, sagittarius_rails, velorum")


if __name__ == "__main__":
    main()
