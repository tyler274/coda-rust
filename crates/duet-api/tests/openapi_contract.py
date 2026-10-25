"""Focused generation-boundary checks; run with python3 -m unittest discover.

Located with the API owner rather than the read-only reference specification.
"""
import importlib.util
from pathlib import Path
import unittest
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location("compose", ROOT / "scripts/duet-wire/compose.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class ContractComposition(unittest.TestCase):
    def setUp(self):
        self.base = {
            "openapi": "3.1.0",
            "paths": {"/page": {"get": {"operationId": "read_page"}}},
            "components": {"schemas": {"Page": {"type": "object"}}},
        }

    def test_empty_overlay_preserves_existing_contract(self):
        self.assertEqual(module.compose(self.base, {"paths": {}, "components": {"schemas": {}}}), self.base)

    def test_new_path_operation_and_dto(self):
        additions = {"paths": {"/page": {"post": {"operationId": "publish"}}, "/new": {"get": {}}},
                     "components": {"schemas": {"Publication": {"type": "object"}}}}
        result = module.compose(self.base, additions)
        self.assertEqual(result["components"]["schemas"]["Page"], {"type": "object"})
        self.assertIn("Publication", result["components"]["schemas"])
        self.assertIn("get", result["paths"]["/page"])
        self.assertIn("post", result["paths"]["/page"])
        self.assertNotIn("post", self.base["paths"]["/page"])

    def test_reject_changed_dto_operation_or_metadata(self):
        for additions in [
            {"components": {"schemas": {"Page": {"type": "string"}}}},
            {"paths": {"/page": {"get": {"operationId": "different"}}}},
            {"info": {"title": "replacement"}},
            {"paths": []},
        ]:
            with self.assertRaises(ValueError):
                module.compose(self.base, additions)
