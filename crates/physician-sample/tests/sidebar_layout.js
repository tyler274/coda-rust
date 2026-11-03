// Puts the page in `theme` and reads where the sidebar's footer and recent list sit, or null before
// they render. `CdpBrowser::sidebar_layout` calls this through `Runtime.callFunctionOn`.
function (theme) {
  document.documentElement.dataset.theme = theme;
  const signOut = document.querySelector('button[aria-label="Sign out of Coda"]');
  const recent = document.querySelector('.sidebar-recent');
  if (!signOut || !recent) return null;
  const moves = (axis) => {
    recent[axis] = 1;
    const moved = recent[axis] > 0;
    recent[axis] = 0;
    return moved;
  };
  const { left, right, top, bottom } = signOut.getBoundingClientRect();
  return {
    window_width: innerWidth,
    window_height: innerHeight,
    sign_out: { left, right, top, bottom },
    sign_out_visible: signOut.checkVisibility({ visibilityProperty: true }),
    recent_rows: recent.querySelectorAll('.sidebar-recent-link').length,
    recent_visible_height: recent.clientHeight,
    recent_scrolls_down: moves('scrollTop'),
    recent_scrolls_sideways: moves('scrollLeft'),
  };
}
