// Native CDP, following physician-sample/tests/site.rs; no npm dependencies.
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {mkdtempSync, rmSync, mkdirSync, writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';
const origin = process.argv[2];
const profile = mkdtempSync(join(tmpdir(), 'public-site-browser-'));
const env = {...process.env}; delete env.LD_LIBRARY_PATH;
const child = spawn(process.env.CHROMIUM || 'chromium', ['--headless=new', '--no-sandbox', '--disable-gpu', '--remote-debugging-port=0', `--user-data-dir=${profile}`, 'about:blank'], {env, stdio: ['ignore', 'ignore', 'pipe']});
let socket;
const deadline = setTimeout(() => { console.error('Browser proof exceeded 120 seconds'); child.kill(); process.exit(1); }, 120000);
try {
  const ws = await new Promise((resolve, reject) => {
    let stderr = '';
    child.on('error', reject); child.on('exit', code => reject(new Error(`Chromium exit ${code}: ${stderr}`)));
    child.stderr.on('data', chunk => { stderr += chunk; const match = stderr.match(/DevTools listening on (ws:\/\/\S+)/); if (match) resolve(match[1]); });
  });
  const targets = await (await fetch(`http://${new URL(ws).host}/json/list`)).json();
  socket = new WebSocket(targets.find(target => target.type === 'page').webSocketDebuggerUrl);
  await new Promise((resolve, reject) => {socket.onopen = resolve; socket.onerror = reject;});
  let next = 0; const pending = new Map(); const errors = []; const failures = [];
  const lifecycle = [];
  socket.onmessage = ({data}) => {
    const message = JSON.parse(data);
    if (message.id && pending.has(message.id)) { const item = pending.get(message.id); pending.delete(message.id); message.error ? item.reject(new Error(JSON.stringify(message.error))) : item.resolve(message.result); }
    if (message.method === 'Runtime.exceptionThrown') errors.push(message.params.exceptionDetails);
    if (message.method === 'Network.responseReceived' && message.params.response.url.startsWith(origin) && message.params.response.status >= 400) failures.push(message.params.response);
    if (message.method === 'Page.lifecycleEvent') lifecycle.push(message.params);
  };
  const call = (method, params={}) => new Promise((resolve, reject) => {const id = ++next; pending.set(id, {resolve,reject}); socket.send(JSON.stringify({id,method,params}));});
  const evaluate = async expression => {const result = await call('Runtime.evaluate', {expression, returnByValue:true, awaitPromise:true}); assert(!result.exceptionDetails, JSON.stringify(result)); return result.result.value;};
  // Poll observable conditions; elapsed time alone never establishes readiness.
  const until = async condition => {for(let i=0;i<150;i++){if(await (typeof condition === 'function' ? condition() : evaluate(condition)))return; await delay(100);} throw new Error(`Timed out: ${condition}`);};
  await call('Page.enable'); await call('Runtime.enable'); await call('Network.enable');
  await call('Page.setLifecycleEventsEnabled',{enabled:true});
  const navigate = async path => {
    lifecycle.length=0;
    const navigation = await call('Page.navigate',{url:origin+path});
    assert(!navigation.errorText, navigation.errorText);
    assert(navigation.loaderId, 'navigation must create a new document');
    // Events may precede the navigate reply. Buffer them and match the new loader,
    // never a readyState/content predicate that the previous document can satisfy.
    await until(() => lifecycle.some(event => event.loaderId === navigation.loaderId && event.frameId === navigation.frameId && event.name === 'load'));
    assert.equal(await evaluate('location.pathname'),path);
  };
  const stages = "#heroStage, #memStage, .f-today, .f-brief, .f-ask, .f-follow, .t-src, .t-you, .duet-story";
  const snapshot = `JSON.stringify([...document.querySelectorAll(${JSON.stringify(stages)})].map(stage => stage.outerHTML))`;
  // The original site.js shows the hero's final frame under reduced motion.
  const staticContent = `(() => {
    const hero = document.querySelector('#heroStage');
    return hero.classList.contains('s-read') && hero.classList.contains('drawn') && !hero.classList.contains('s-disc')
      && !!hero.querySelector('.sat-s3.on') && !hero.querySelector('.pointer.show')
      && [...hero.querySelectorAll('.a-pre .a-row')].every(row => row.classList.contains('in'));
  })()`;
  // Distinct horizontal pointer positions over the next frames. A CSS transition
  // glides through many; a stylesheet that overrides it makes the pointer jump.
  const pointerPath = `new Promise(done => {
    const seen = new Set(), x = document.querySelector('#heroStage .ptr-x'), end = performance.now() + 2600;
    const step = now => { seen.add(getComputedStyle(x).transform); now < end ? requestAnimationFrame(step) : done(seen.size); };
    requestAnimationFrame(step);
  })`;
  const screenshots = process.env.PUBLIC_SITE_SCREENSHOTS || join(tmpdir(), 'public-site-screenshots'); mkdirSync(screenshots,{recursive:true});
  for (const [label,width,height,reduced,scripts] of [['desktop',1440,1000,false,true],['mobile',390,844,false,true],['reduced',390,844,true,true],['no-script',390,844,false,false]]) {
    errors.length=0; failures.length=0;
    await call('Emulation.setDeviceMetricsOverride',{width,height,deviceScaleFactor:1,mobile:width<600});
    await call('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:reduced?'reduce':'no-preference'}]});
    await call('Emulation.setScriptExecutionDisabled',{value:!scripts});
    await navigate('/');
    assert.equal(await evaluate(`document.readyState === 'complete' && !!document.querySelector('#heroStage')`),true);
    if (scripts) {
      // site.js paints the landscapes, reveals the copy and scales each stage.
      await until(`!!document.querySelector('.frame-hero .art.ready') && getComputedStyle(document.querySelector('h1')).opacity === '1'`);
      assert(await evaluate(`document.querySelector('.frame-hero .art').style.backgroundImage.startsWith('url("data:image/svg+xml')`),`${label}: unpainted landscape`);
      assert(await evaluate(`/^[\\d.]+$/.test(document.querySelector('.stage-wrap').style.getPropertyValue('--s'))`),`${label}: unscaled stage`);
    }
    const page = await evaluate(`({text:document.body.innerText, overflow:document.documentElement.scrollWidth > innerWidth, images:[...document.images].filter(i=>!i.complete||i.naturalWidth===0).map(i=>i.src), heading:getComputedStyle(document.querySelector('h1')).opacity, viewport:innerWidth})`);
    assert(page.text.includes('Coda handles what happens between visits.') && page.text.includes('A physician-directed AI care team.') && page.text.includes('Consumer Health Data Privacy Policy'),label);
    assert.equal(page.overflow,false,`${label}: horizontal overflow at ${page.viewport}`);
    assert.deepEqual(page.images,[],`${label}: broken images`);
    assert.equal(page.heading,'1',`${label}: invisible heading`);
    if (reduced) {
      assert.equal(await evaluate(staticContent),true,`${label}: complete static hero`);
      const still = await evaluate(snapshot);
      await delay(1500);
      assert.equal(await evaluate(snapshot),still,`${label}: reduced-motion stages must stay static`);
    } else if (scripts) {
      await until(`document.querySelector('#heroStage').classList.contains('s-read')`);
      const positions = await evaluate(pointerPath);
      assert(positions >= 10,`${label}: hero pointer jumped (${positions} positions) instead of gliding`);
    }
    if (scripts) {
      await evaluate('scrollTo(0, 700)'); await until(`document.querySelector('nav.nav').classList.contains('scrolled')`);
      await evaluate('scrollTo(0, 0)'); await until(`scrollY === 0 && !document.querySelector('nav.nav').classList.contains('scrolled')`);
    }
    const shot=await call('Page.captureScreenshot',{format:'png',captureBeyondViewport:false});
    const path=join(screenshots,`${label}.png`); writeFileSync(path,Buffer.from(shot.data,'base64')); console.log(`Screenshot: ${path}`);
    assert.deepEqual(failures,[],`${label}: failed local assets`); assert.deepEqual(errors,[],`${label}: uncaught JavaScript errors`);
    await navigate('/duet');
    assert.equal(await evaluate(`document.body.innerText.includes('A clearer picture of you.')`),true);
    assert.equal(await evaluate('document.documentElement.scrollWidth > innerWidth'),false,`${label}: Duet overflow`);
    // The Duet story's markup is its last frame, ending on the page her clinician receives:
    // reduced motion and no-script keep it, and duet.js otherwise plays from the start.
    const lastFrame = `[...document.querySelectorAll('.duet-story .ds-insight, .duet-story .ds-msg, .duet-story .ds-clin')].every(e => getComputedStyle(e).opacity === '1')`;
    if (scripts && !reduced) {
      await evaluate(`(() => { const r = document.querySelector('.duet-story').getBoundingClientRect(); scrollTo(0, scrollY + r.top + r.height / 2 - innerHeight / 2); })()`);
      await until(`!!document.querySelector('.duet-story .ds-msg.on')`);
    } else {
      assert.equal(await evaluate(lastFrame),true,`${label}: Duet story's last frame`);
    }
    assert.deepEqual(failures,[],`${label}: Duet failed local assets`); assert.deepEqual(errors,[],`${label}: Duet uncaught JavaScript errors`);
    console.log(`${label}: content, assets, overflow, navigation and motion passed`);
  }
  await call('Emulation.setScriptExecutionDisabled',{value:false});
} finally {
  clearTimeout(deadline);
  const exited = new Promise(resolve => child.once('exit', resolve));
  if (socket?.readyState === WebSocket.OPEN) socket.send(JSON.stringify({id:999999,method:'Browser.close'}));
  else child.kill();
  await exited; socket?.close();
  try { rmSync(profile,{recursive:true,force:true,maxRetries:5,retryDelay:100}); }
  catch (error) { console.warn(`Chromium profile cleanup deferred (${profile}): ${error.message}`); }
}
