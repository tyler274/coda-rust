#!/usr/bin/env python3
"""Keep the desktop's Codex launch context and expose its owner on a private socket.

Set CODEX_CLI_PATH to this launcher for the desktop. Other CLI invocations pass
through unchanged. The desktop still owns the server's lifetime; this adapter
only translates its JSONL transport to the server's Unix WebSocket transport.
"""
import asyncio
import fcntl
import os
from pathlib import Path
import signal
import sys

from websockets.asyncio.client import unix_connect

CODEX = '/usr/lib/chatgpt/resources/codex'


async def serve(args):
    directory = Path(os.environ['XDG_RUNTIME_DIR']) / 'coda-codex'
    directory.mkdir(mode=0o700, exist_ok=True)
    socket = directory / 'app-server.sock'
    # One desktop-owned server per user. A stale socket is removable only after
    # acquiring this owner's lock; another live launcher remains untouched.
    with (directory / 'owner.lock').open('w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        socket.unlink(missing_ok=True)
        env = dict(os.environ, CODEX_CLI_PATH=CODEX)
        server = await asyncio.create_subprocess_exec(
            CODEX, *args, '--listen', f'unix://{socket}', env=env,
            stdin=asyncio.subprocess.DEVNULL, stdout=asyncio.subprocess.DEVNULL,
        )
        loop = asyncio.get_running_loop()
        stopping = asyncio.Event()
        for sig in (signal.SIGTERM, signal.SIGINT):
            loop.add_signal_handler(sig, stopping.set)
        tasks = []
        try:
            for _ in range(300):
                if server.returncode is not None:
                    raise RuntimeError(f'Codex exited during startup: {server.returncode}')
                if socket.exists():
                    break
                await asyncio.sleep(0.1)
            else:
                raise TimeoutError('Codex socket did not become available')
            async with unix_connect(path=str(socket), max_size=16 * 1024 * 1024) as ws:
                reader = asyncio.StreamReader(limit=16 * 1024 * 1024)
                await loop.connect_read_pipe(lambda: asyncio.StreamReaderProtocol(reader), sys.stdin)

                async def to_server():
                    while line := await reader.readline():
                        await ws.send(line.decode().rstrip('\n'))

                async def to_desktop():
                    async for message in ws:
                        sys.stdout.write(message + '\n')
                        sys.stdout.flush()

                tasks = [asyncio.create_task(task) for task in (
                    to_server(), to_desktop(), server.wait(), stopping.wait(),
                )]
                done, _ = await asyncio.wait(tasks, return_when=asyncio.FIRST_COMPLETED)
                for task in done:
                    task.result()
        finally:
            for task in tasks:
                task.cancel()
            if server.returncode is None:
                server.terminate()
                try:
                    await asyncio.wait_for(server.wait(), timeout=10)
                except TimeoutError:
                    server.kill()
                    await server.wait()
            socket.unlink(missing_ok=True)


if __name__ == '__main__':
    arguments = sys.argv[1:]
    if ('app-server' not in arguments or '--analytics-default-enabled' not in arguments
            or '--print-startup-config' in arguments):
        os.execv(CODEX, [CODEX, *arguments])
    asyncio.run(serve(arguments))
