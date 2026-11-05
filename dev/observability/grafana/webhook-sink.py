"""Local stand-in for Slack's incoming webhook (compose profile `grafana`).

Grafana's Slack contact point posts here instead of real Slack. Each POST is
printed as one JSON line (`docker compose logs webhook-sink`) and kept in
memory; GET /received returns them all as a JSON array. Replies "ok", the
body Slack's incoming webhooks return on success.
"""

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
        RECEIVED.append({"path": self.path, "payload": payload})
        print(json.dumps({"path": self.path, "payload": payload}), flush=True)
        self.reply(200, "ok")


http.server.ThreadingHTTPServer(("0.0.0.0", 8080), Sink).serve_forever()
