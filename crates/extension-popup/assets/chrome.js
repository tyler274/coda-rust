export function send(action) {
  return chrome.runtime.sendMessage({ action }).then((reply) => JSON.stringify(reply ?? {}));
}
export async function open_hint() {
  const [tab] = await chrome.tabs.query({ url: 'https://app.hint.com/*' });
  if (!tab) return chrome.tabs.create({ url: 'https://app.hint.com/' });
  await chrome.windows.update(tab.windowId, { focused: true });
  return chrome.tabs.update(tab.id, { active: true });
}
export function clock(ms) {
  return new Date(ms).toLocaleTimeString([], { hour: 'numeric', minute: '2-digit' });
}
