// Runs in the existing Hint page on Connect, and whenever the popup asks where Hint stands.
// It only reads, and returns no credential.
(() => {
  const fail = reason => ({ok: false, reason});
  if (location.origin !== 'https://app.hint.com') return fail('wrong_sandbox');
  try {
    const snapshot = window.hintSession?.getSnapshot();
    const root = window.fronteggApp?.store?.getState();
    const tenant = root?.auth?.tenantsState?.activeTenant;
    // Signed out comes first: a signed-out page can't show which practice it is on.
    if (snapshot?.status !== 'active' || root?.auth?.isAuthenticated !== true || !snapshot.token) {
      return fail('sign_in_required');
    }
    if (!document.body.innerText.includes('Currently viewing sandbox data.') ||
        !document.querySelector('[aria-label="Switch organization, currently Coda Health Sandbox Practice"]')) {
      return fail('wrong_sandbox');
    }
    const claims = JSON.parse(atob(snapshot.token.split('.')[1].replace(/-/g, '+').replace(/_/g, '/')));
    const context = root.root?.context || window.fronteggApp.options.contextOptions;
    // This follows the shipped FetchClient.getBaseUrl default; no arbitrary auth host.
    if (context?.baseUrl !== 'https://accounts.hint.com' ||
        (context.urlPrefix && context.urlPrefix !== 'frontegg')) return fail('auth_context_changed');
    if (typeof tenant?.tenantId !== 'string' || claims.tenantId !== tenant.tenantId ||
        typeof claims.sub !== 'string' || !Number.isFinite(claims.iat) ||
        !Number.isFinite(claims.exp) || claims.exp * 1000 <= Date.now()) return fail('identity_unavailable');
    return {ok: true, context: {tenant_id: tenant.tenantId, subject: claims.sub,
      issuer: claims.iss, audience: claims.aud, original_iat: claims.iat, original_exp: claims.exp,
      prepared_at: Date.now(), practice: 'Coda Health Sandbox Practice'}};
  } catch (_) {return fail('context_unavailable');}
})();
