"""Apply authorized profile entitlements locally, never register or deploy an app."""
import os
from pathlib import Path
import plistlib
import subprocess
import sys
import tempfile

app = Path(sys.argv[1]).resolve()
bundle = os.environ["DUET_IOS_BUNDLE_ID"]
identity = os.environ["DUET_IOS_SIGNING_IDENTITY"]
with (app / "Info.plist").open("rb") as source:
    if plistlib.load(source)["CFBundleIdentifier"] != bundle:
        raise SystemExit("Built bundle does not match Identity Platform bundle restriction")
profile = plistlib.loads(subprocess.check_output(["security", "cms", "-D", "-i", str(app / "embedded.mobileprovision")]))
authorized = profile["Entitlements"]
if authorized.get("com.apple.developer.applesignin") != ["Default"]:
    raise SystemExit("Provisioning profile does not authorize Sign in with Apple")
if not authorized.get("com.apple.developer.healthkit"):
    raise SystemExit("Provisioning profile does not authorize Apple Health")
app_id = authorized.get("application-identifier", "")
if not app_id.endswith("." + bundle):
    raise SystemExit("Provisioning profile does not explicitly authorize this bundle")
# Preserve only the approved profile's application/keychain/team/debug entitlements.
# Never add an entitlement not authorized by the provisioning profile.
keys = ["application-identifier", "com.apple.developer.team-identifier", "keychain-access-groups", "get-task-allow", "com.apple.developer.applesignin", "com.apple.developer.healthkit"]
entitlements = {key: authorized[key] for key in keys if key in authorized}
with tempfile.TemporaryDirectory(prefix="duet-local-sign-") as directory:
    path = Path(directory) / "entitlements.plist"
    path.write_bytes(plistlib.dumps(entitlements))
    subprocess.run(["codesign", "--force", "--sign", identity, "--entitlements", str(path), str(app)], check=True)
subprocess.run(["codesign", "--verify", "--strict", str(app)], check=True)
