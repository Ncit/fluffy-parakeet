#!/usr/bin/env python3
import json
import sys
from pathlib import Path

try:
    import jsonschema
except ImportError:
    print("jsonschema is required. Install with: python -m pip install jsonschema", file=sys.stderr)
    sys.exit(2)

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "ai" / "dsl_v3.schema.json"
DEFAULT_SCENE_PATH = ROOT / "ai" / "example_scene.json"


def validate(scene_path: Path) -> None:
    schema = json.loads(SCHEMA_PATH.read_text())
    scene = json.loads(scene_path.read_text())
    jsonschema.validate(instance=scene, schema=schema)


def main() -> int:
    scene_paths = [Path(arg) for arg in sys.argv[1:]] or [DEFAULT_SCENE_PATH]
    for scene_path in scene_paths:
        validate(scene_path)
        print(f"valid DSL: {scene_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
