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

    def test_retirement_preserves_practice_doctor_and_reused_schema_contracts(self):
        def response(name):
            return {"get": {"responses": {"200": {"$ref": "#/components/schemas/" + name}}}}

        self.base["paths"].update({
            "/api/sharing/overview": response("Overview"),
            "/api/sharing/device": response("Device"),
            "/api/physician-sharing/status": response("Practice"),
            "/api/doctor-page": response("Doctor"),
        })
        self.base["components"]["schemas"].update({
            "Overview": {"$ref": "#/components/schemas/Peer"},
            "Peer": {"$ref": "#/components/schemas/Device"},
            "Device": {"$ref": "#/components/schemas/Evidence"},
            "Practice": {"$ref": "#/components/schemas/Evidence"},
            "Doctor": {"$ref": "#/components/schemas/Evidence"},
            "Evidence": {"type": "object"},
        })
        result = module.compose(self.base, {})
        self.assertEqual(set(result["paths"]), {"/page", "/api/physician-sharing/status", "/api/doctor-page"})
        self.assertEqual(set(result["components"]["schemas"]), {"Page", "Practice", "Doctor", "Evidence"})
        self.assertIn("/api/sharing/overview", self.base["paths"])
        self.assertIn("Device", self.base["components"]["schemas"])
