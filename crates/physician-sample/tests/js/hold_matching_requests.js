(() => {
    const fetch = window.fetch.bind(window);
    window.__hold = '/practice-day';
    window.__held = [];
    window.fetch = (input, init) => {
        const answer = fetch(input, init);
        const url = typeof input === 'string' ? input : input.url;
        if (window.__hold && new RegExp(window.__hold).test(url)) {
            return new Promise(resolve => window.__held.push(() => resolve(answer)));
        }
        return answer;
    };
    window.__release = () => {
        window.__hold = null;
        const held = window.__held.splice(0);
        held.forEach(release => release());
        return held.length;
    };
})()
