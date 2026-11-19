#!/usr/bin/env python3
"""Loopback Responses fixture for the local physician stack and its process tests.

Two counters inject provider problems. Each request takes one from the first
counter that is not zero:
- `--rejections`: the provider rejects the request with HTTP 400, which holds
  the run until a person tries again.
- `--failures`: the provider sends a response that breaks the stream
  contract: it ends with `response.completed`, but the response's status is
  "failed". The harness reads it as a protocol error and does not retry it,
  so the attempt fails at once and recovery tries the work again.
"""
import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from threading import Lock
import time
import uuid


def call(call_id, name, arguments):
    return {"id": uuid.uuid4().hex, "type": "function_call", "status": "completed",
            "call_id": call_id, "name": name,
            "arguments": arguments if isinstance(arguments, str) else json.dumps(arguments)}


def message(text):
    return {"id": uuid.uuid4().hex, "type": "message", "role": "assistant", "status": "completed",
            "content": [{"type": "output_text", "text": text}]}


def output_for(body, rounds, reply):
    results = {item["call_id"]: item["output"] for item in body["input"]
               if item.get("type") == "function_call_output"}
    # Scripted handoffs deliberately drop earlier receipts from the request.
    # The exact note chosen by this fixture identifies the completed prefix.
    texts = {item["content"] for item in body["input"] if isinstance(item.get("content"), str)}
    completed = max((ordinal for ordinal, calls in enumerate(rounds, 1)
                     if any(item["name"] == "save_working_note"
                            and item["arguments"].get("fresh_window")
                            and item["arguments"].get("text") in texts for item in calls)), default=0)
    for ordinal, calls in enumerate(rounds, 1):
        ids = [f"fixed-{ordinal}-{index}" for index in range(len(calls))]
        if ordinal <= completed or all(identity in results for identity in ids):
            continue
        assert all(identity not in results for identity in ids), "missing paired tool result"
        narration = [] if any(tool.get("name") == "finish_reply_revision" for tool in body["tools"]) else [
            message(f"Fixed narration before tool round {ordinal}. ")]
        return narration + [call(identity, item["name"], item["arguments"]) for identity, item in zip(ids, calls)]
    if any(tool.get("name") == "finish_reply_revision" for tool in body["tools"]):
        last = body["input"][-1]
        feedback = json.loads(last["output"]) if last.get("type") == "function_call_output" else {}
        references = []
        if isinstance(feedback, dict):
            if feedback.get("outcome") == "sources_changed":
                references = feedback["added"] + feedback["corrected"]
            elif feedback.get("outcome") == "sources_unread":
                references = feedback["references"]
        identity = f"fixed-{len(body['input'])}"
        if references:
            return [call(identity, "read_patient_sources", {"references": references[:8], "max_result_bytes": 65536})]
        return [call(identity, "finish_reply_revision", {"text": "Noted.", "review_basis": "Synthetic provider response."})]
    # A chat ends through finish_answer, with the answer also written as text.
    # On a run's final request the harness keeps only the text.
    if any(tool.get("name") == "finish_answer" for tool in body["tools"]):
        arguments = finish_arguments(reply)
        return [message(arguments["answer"]),
                call(f"fixed-finish-{len(body['input'])}", "finish_answer", arguments)]
    return [message(reply)]


def finish_arguments(reply):
    """A chat ends through finish_answer. A reply that is a JSON object is the
    finish's arguments as written, so a script can cite what its rounds read.
    Any other reply is an answer that rests on no source."""
    try:
        scripted = json.loads(reply)
    except ValueError:
        scripted = None
    if isinstance(scripted, dict):
        return scripted
    return {"answer": reply, "quotes": [], "descriptions": [],
            "basis": "Synthetic provider response.", "cites_none": True}


def encode_stream(response):
    events = [{"type": "response.created", "response": {"id": response["id"]}}]
    for index, item in enumerate(response["output"]):
        position = {"output_index": index}
        identity = position | {"item_id": item["id"]}
        events.append({"type": "response.output_item.added", **position, "item": item})
        if item["type"] == "function_call":
            events.extend([
                {"type": "response.function_call_arguments.delta", **identity, "delta": item["arguments"]},
                {"type": "response.function_call_arguments.done", **identity, "arguments": item["arguments"]}])
        elif item["type"] == "message":
            for part, content in enumerate(item["content"]):
                where = identity | {"content_index": part}
                events.extend([
                    {"type": "response.content_part.added", **where, "part": {"type": "output_text", "text": ""}},
                    {"type": "response.output_text.delta", **where, "delta": content["text"]},
                    {"type": "response.output_text.done", **where, "text": content["text"]},
                    {"type": "response.content_part.done", **where, "part": content}])
        events.append({"type": "response.output_item.done", **position, "item": item})
    events.append({"type": "response.completed", "response": response})
    return "".join(f"data: {json.dumps(event | {'sequence_number': i})}\n\n"
                   for i, event in enumerate(events)).encode()


def take(counter):
    """Take one injection from a counter file, if it holds any."""
    if counter is None:
        return False
    remaining = int(counter.read_text())
    if remaining:
        counter.write_text(str(remaining - 1))
    return remaining > 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--port", type=int, required=True)
    parser.add_argument("--failures", type=Path, required=True)
    parser.add_argument("--rejections", type=Path)
    parser.add_argument("--delay-ms", type=int, default=0)
    parser.add_argument("--rounds", type=json.loads, default=[])
    parser.add_argument("--rounds-file", type=Path)
    parser.add_argument("--reply", default="Noted.")
    parser.add_argument("--reply-file", type=Path)
    parser.add_argument("--requests-file", type=Path)
    args = parser.parse_args()
    lock = Lock()

    class Handler(BaseHTTPRequestHandler):
        def do_GET(self):
            self.send_response(200 if self.path == "/readyz" else 404)
            self.end_headers()

        def do_POST(self):
            if self.path != "/responses":
                self.send_error(404)
                return
            body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
            with lock:
                if args.requests_file:
                    with args.requests_file.open("a") as recorded:
                        recorded.write(json.dumps(body) + "\n")
            time.sleep(args.delay_ms / 1000)
            with lock:
                if take(args.rejections):
                    rejection = json.dumps({"error": {
                        "message": "Synthetic request rejection", "type": "invalid_request_error",
                        "param": None, "code": None}}).encode()
                    self.send_response(400)
                    self.send_header("Content-Type", "application/json")
                    self.send_header("Content-Length", str(len(rejection)))
                    self.end_headers()
                    self.wfile.write(rejection)
                    return
                failed = take(args.failures)
            if failed:
                response = {"id": uuid.uuid4().hex, "status": "failed", "model": body["model"], "output": []}
            else:
                rounds = json.loads(args.rounds_file.read_text()) if args.rounds_file else args.rounds
                reply = args.reply_file.read_text() if args.reply_file else args.reply
                response = {
                    "id": uuid.uuid4().hex,
                    "status": "completed",
                    "model": body["model"],
                    "output": output_for(body, rounds, reply),
                    "usage": {
                        "input_tokens": 500,
                        "input_tokens_details": {"cached_tokens": 450},
                        "output_tokens": 50,
                        "total_tokens": 550,
                    },
                }
            stream = body.get("stream")
            wire = encode_stream(response) if stream else json.dumps(response).encode()
            self.send_response(200)
            self.send_header("Content-Type", "text/event-stream" if stream else "application/json")
            self.send_header("Content-Length", str(len(wire)))
            self.end_headers()
            self.wfile.write(wire)

    ThreadingHTTPServer(("127.0.0.1", args.port), Handler).serve_forever()


if __name__ == "__main__":
    main()
