export async function duetAppleSignIn(clientId, redirectURI) {
  if (!globalThis.AppleID?.auth) throw new Error('Load the official Apple authorization SDK before enabling sign in');
  const random = crypto.getRandomValues(new Uint8Array(32));
  const nonce = Array.from(random, b => b.toString(16).padStart(2,'0')).join('');
  const hash = new Uint8Array(await crypto.subtle.digest('SHA-256', new TextEncoder().encode(nonce)));
  const state = Array.from(crypto.getRandomValues(new Uint8Array(32)), b => b.toString(16).padStart(2,'0')).join('');
  AppleID.auth.init({clientId, redirectURI, scope:'name email', state, nonce:Array.from(hash,b=>b.toString(16).padStart(2,'0')).join(''), usePopup:true});
  const response = await AppleID.auth.signIn();
  if (response.authorization?.state !== state || !response.authorization?.id_token) throw new Error('authorization');
  return JSON.stringify({identity_token:response.authorization.id_token, nonce});
}
