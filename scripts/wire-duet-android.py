"""Idempotent local generated-manifest wiring; no deployment or provider registration."""
import sys
import xml.etree.ElementTree as ET

path = sys.argv[1]
android = "http://schemas.android.com/apk/res/android"
ET.register_namespace("android", android)
a = lambda name: f"{{{android}}}{name}"
tree = ET.parse(path)
app = tree.getroot().find("application")
if app is None:
    raise SystemExit("Generated application is missing")
if not any(p.get(a("name")) == "coda.duet.DoctorPdfProvider" for p in app.findall("provider")):
    ET.SubElement(app, "provider", {
        a("name"): "coda.duet.DoctorPdfProvider",
        a("authorities"): "${applicationId}.duet.pdf",
        a("exported"): "false",
        a("grantUriPermissions"): "true",
    })
tree.write(path, encoding="utf-8", xml_declaration=True)
