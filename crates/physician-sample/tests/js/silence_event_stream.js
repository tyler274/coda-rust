(() => {
    // The page's change stream as a dead connection that never closed: once `__silenceEvents()`
    // is called, the stream open then still reads what the server sends, keep-alives too, and
    // hands none of it to the page. Every conversation read the page starts meanwhile is held and
    // never answered, so nothing but the page giving up that stream can bring it news. A stream
    // opened after it is left alone. `__lastChunk` is the text of the last bytes the page was handed.
    const original = window.fetch.bind(window);
    window.__eventRequests = 0;
    window.__lastBytes = null;
    window.__lastChunk = null;
    window.__silencedAt = null;
    window.__droppedAt = null;
    window.__heldReads = 0;
    let silenced = null;
    window.__silenceEvents = () => {
        silenced = window.__eventRequests;
        window.__silencedAt = performance.now();
        return silenced;
    };
    const holding = () => silenced !== null && window.__droppedAt === null;
    window.fetch = (input, init) => {
        const url = input instanceof Request ? input.url : String(input);
        if (url.includes('/patient-workspaces/events')) {
            const connection = ++window.__eventRequests;
            return original(input, init).then(response => {
                if (!response.body) return response;
                const reader = response.body.getReader();
                const body = new ReadableStream({
                    async pull(controller) {
                        for (;;) {
                            const { done, value } = await reader.read();
                            if (done) {
                                controller.close();
                                return;
                            }
                            if (silenced === connection) continue;
                            window.__lastBytes = performance.now();
                            window.__lastChunk = new TextDecoder().decode(value);
                            controller.enqueue(value);
                            return;
                        }
                    },
                    cancel(reason) {
                        if (silenced === connection) window.__droppedAt = performance.now();
                        return reader.cancel(reason);
                    },
                });
                return new Response(body, {
                    status: response.status,
                    statusText: response.statusText,
                    headers: response.headers,
                });
            });
        }
        if (url.includes('/snapshot') && holding()) {
            window.__heldReads++;
            return new Promise(() => {});
        }
        return original(input, init);
    };
})()
