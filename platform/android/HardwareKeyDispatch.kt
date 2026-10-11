package dev.dioxus.main

import android.view.KeyEvent
import android.webkit.WebView
import org.json.JSONObject

// A hardware key reaches the activity and stops there. The soft keyboard and
// paste still commit through the input connection, so this inserts only the
// keys that connection never delivers.
fun duetInsertHardwareKey(webView: WebView, event: KeyEvent): Boolean {
    if (event.action != KeyEvent.ACTION_DOWN) return false
    if (event.flags and KeyEvent.FLAG_SOFT_KEYBOARD != 0) return false
    if (event.isCtrlPressed || event.isAltPressed || event.isMetaPressed) return false
    val script = when (event.keyCode) {
        KeyEvent.KEYCODE_DEL -> "duetEditField('backward')"
        KeyEvent.KEYCODE_FORWARD_DEL -> "duetEditField('forward')"
        else -> {
            val code = event.getUnicodeChar(event.metaState)
            if (code == 0) return false
            val text = JSONObject.quote(String(Character.toChars(code)))
            "duetEditField('insert',$text)"
        }
    }
    webView.evaluateJavascript(DUET_EDIT_FIELD + script, null)
    return true
}

private const val DUET_EDIT_FIELD = """
function duetEditField(op, text) {
  var el = document.activeElement;
  if (!el || (el.tagName !== 'TEXTAREA' && el.tagName !== 'INPUT')) return;
  if (el.disabled || el.readOnly) return;
  var value = el.value || '';
  var start = el.selectionStart;
  var end = el.selectionEnd;
  if (start == null || end == null) { start = value.length; end = start; }
  var next = value;
  var caret = start;
  if (op === 'insert') {
    next = value.slice(0, start) + text + value.slice(end);
    caret = start + text.length;
  } else if (op === 'backward') {
    if (start !== end) {
      next = value.slice(0, start) + value.slice(end);
      caret = start;
    } else if (start > 0) {
      next = value.slice(0, start - 1) + value.slice(end);
      caret = start - 1;
    } else {
      return;
    }
  } else if (op === 'forward') {
    if (start !== end) {
      next = value.slice(0, start) + value.slice(end);
      caret = start;
    } else if (end < value.length) {
      next = value.slice(0, start) + value.slice(end + 1);
      caret = start;
    } else {
      return;
    }
  }
  el.value = next;
  if (el.setSelectionRange) el.setSelectionRange(caret, caret);
  el.dispatchEvent(new InputEvent('input', {bubbles: true, cancelable: true, inputType: 'insertText'}));
}
"""
