// Runs in the existing Hint page on Connect, on passive sign-in checks, and when
// the popup asks where Hint stands. Reads identity and practice metadata only;
// copies no credential.
(() => {
  const fail = reason => ({ok: false, reason});
  if (location.origin !== 'https://app.hint.com') return fail('other_practice');
  try {
    const snapshot = window.hintSession?.getSnapshot();
    const root = window.fronteggApp?.store?.getState();
    const tenant = root?.auth?.tenantsState?.activeTenant;
    if (snapshot?.status !== 'active' || root?.auth?.isAuthenticated !== true || !snapshot.token) {
      return fail('sign_in_required');
    }
    const switcher = document.querySelector?.('[aria-label^="Switch organization, currently "]') ?? null;
    const rawLabel = switcher?.getAttribute?.('aria-label') || switcher?.ariaLabel || null;
    const labelPractice = typeof rawLabel === 'string' && rawLabel.startsWith('Switch organization, currently ')
      ? rawLabel.slice('Switch organization, currently '.length).trim()
      : null;
    const practice = labelPractice || (typeof tenant?.name === 'string' ? tenant.name.trim() : null);
    if (!switcher && !practice) {
      return fail('other_practice');
    }
    const resolvedPractice = practice || 'Hint Practice';
    const claims = JSON.parse(atob(snapshot.token.split('.')[1].replace(/-/g, '+').replace(/_/g, '/')));
    const context = root.root?.context || window.fronteggApp.options?.contextOptions;
    if (context?.baseUrl !== 'https://accounts.hint.com' ||
        (context.urlPrefix && context.urlPrefix !== 'frontegg')) {
      return fail('auth_context_changed');
    }
    if (typeof tenant?.tenantId !== 'string' || claims.tenantId !== tenant.tenantId ||
        typeof claims.sub !== 'string' || !Number.isFinite(claims.iat) ||
        !Number.isFinite(claims.exp) || claims.exp * 1000 <= Date.now()) {
      return fail('identity_unavailable');
    }
    return {
      ok: true,
      context: {
        tenant_id: tenant.tenantId,
        subject: claims.sub,
        issuer: claims.iss,
        audience: claims.aud,
        original_iat: claims.iat,
        original_exp: claims.exp,
        practice: resolvedPractice,
      },
    };
  } catch (_) {
    return fail('context_unavailable');
  }
})();
