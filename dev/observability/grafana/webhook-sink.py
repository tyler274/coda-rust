"""Local stand-in for Slack's incoming webhook (compose profile `grafana`).

Grafana's Slack contact point posts here instead of real Slack. Each POST is
printed as one JSON line (`docker compose logs webhook-sink`) and kept in
memory; GET /received returns them all as a JSON array. Replies "ok", the
body Slack's incoming webhooks return on success.
"""

from datetime import datetime, timezone
import http.server
import json

RECEIVED = []


class Sink(http.server.BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def reply(self, status, body, content_type="text/plain"):
        data = body.encode()
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def do_GET(self):
        if self.path == "/healthz":
            self.reply(200, "ok")
        elif self.path == "/received":
            self.reply(200, json.dumps(RECEIVED), "application/json")
        else:
            self.reply(404, "not found")

    def do_POST(self):
        raw = self.rfile.read(int(self.headers.get("Content-Length", 0)))
        try:
            payload = json.loads(raw)
        except ValueError:
            payload = raw.decode(errors="replace")
        receipt = {
            "path": self.path, "payload": payload,
            "received_at": datetime.now(timezone.utc).isoformat(timespec="microseconds").replace("+00:00", "Z"),
            "source": "local-webhook-sink", "slack_receipt_confirmed": False,
        }
        RECEIVED.append(receipt)
        print(json.dumps(receipt), flush=True)
        self.reply(200, "ok")


if __name__ == "__main__":
    http.server.ThreadingHTTPServer(("0.0.0.0", 8080), Sink).serve_forever()
