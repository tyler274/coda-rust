export function send_message(payload_json) {
  const payload = JSON.parse(payload_json);
  return chrome.runtime.sendMessage(payload).then((reply) => JSON.stringify(reply ?? {}));
}

export async function request_origins(origins_json) {
  const origins = JSON.parse(origins_json);
  if (!Array.isArray(origins) || origins.length === 0 || !chrome.permissions?.request) {
    return true;
  }
  return Boolean(await chrome.permissions.request({ origins }));
}

export async function open_source(entry_url, tab_pattern) {
  if (tab_pattern) {
    const [tab] = await chrome.tabs.query({ url: tab_pattern });
    if (tab) {
      await chrome.windows.update(tab.windowId, { focused: true });
      return chrome.tabs.update(tab.id, { active: true });
    }
  }
  return chrome.tabs.create({ url: entry_url });
}
