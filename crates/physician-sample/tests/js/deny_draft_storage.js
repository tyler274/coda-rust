(() => {
    const set = Storage.prototype.setItem;
    Storage.prototype.setItem = function (key, value) {
        if (String(key).startsWith('coda.physician.draft.')) {
            throw new DOMException('quota', 'QuotaExceededError');
        }
        return set.call(this, key, value);
    };
})()
