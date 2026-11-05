(() => {
    const original = window.fetch.bind(window);
    window.__eventRequests = 0;
    window.fetch = (input, init) => {
        const url = input instanceof Request ? input.url : String(input);
        if (url.includes('/patient-workspaces/events')) {
            window.__eventRequests++;
            if (window.__eventRequests === 1) {
                return new Promise(resolve => {
                    window.__connectEvents = () => {
                        resolve(original(input, init));
                        return true;
                    };
                });
            }
        }
        return original(input, init);
    };
})()
