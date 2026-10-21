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
  const lifecycle = []; const requests = new Map(); const blockedRequests = [];
  socket.onmessage = ({data}) => {
    const message = JSON.parse(data);
    if (message.id && pending.has(message.id)) { const item = pending.get(message.id); pending.delete(message.id); message.error ? item.reject(new Error(JSON.stringify(message.error))) : item.resolve(message.result); }
    if (message.method === 'Runtime.exceptionThrown') errors.push(message.params.exceptionDetails);
    if (message.method === 'Network.responseReceived' && message.params.response.url.startsWith(origin) && message.params.response.status >= 400) failures.push(message.params.response);
    if (message.method === 'Page.lifecycleEvent') lifecycle.push(message.params);
    if (message.method === 'Network.requestWillBeSent') requests.set(message.params.requestId, message.params.request.url);
    if (message.method === 'Network.loadingFailed' && message.params.blockedReason) blockedRequests.push(requests.get(message.params.requestId));
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
  const stages = "#heroStage, #memStage, .f-today, .f-brief, .f-ask, .f-follow, .t-src, .t-you, .duet-stage";
  const snapshot = `JSON.stringify([...document.querySelectorAll(${JSON.stringify(stages)})].map(stage => stage.outerHTML))`;
  // Test-only observation: capture the exact DOM when readiness is published and
  // count completed island ticks, without changing its scheduling or callbacks.
  await call('Page.addScriptToEvaluateOnNewDocument',{source:`
    window.__motionProof = {ticks:0, ready:null};
    const interval = window.setInterval;
    window.setInterval = (callback, timeout, ...args) => interval(() => {
      callback(...args);
      if (timeout === 50) window.__motionProof.ticks++;
    }, timeout);
    const observer = new MutationObserver(() => {
      if (document.documentElement?.classList.contains('motion-ready') && window.__motionProof.ready === null) {
        window.__motionProof.ready = ${snapshot};
        window.__motionProof.readyTicks = window.__motionProof.ticks;
      }
    });
    observer.observe(document, {subtree:true, childList:true, attributes:true, attributeFilter:['class']});
  `});
  const motionTicks = async (count=10) => {
    const before = await evaluate('window.__motionProof.ticks');
    await until(`window.__motionProof.ticks >= ${before+count}`);
  };
  const staticContent = `(() => {
    const hero = document.querySelector('#heroStage');
    return hero.classList.contains('s-read') && hero.classList.contains('drawn') && !hero.classList.contains('s-disc')
      && !!hero.querySelector('.sat-s3.on') && !hero.querySelector('.pointer.show')
      && [...hero.querySelectorAll('.a-pre .a-row')].every(row => row.classList.contains('in'));
  })()`;
  const assertStatic = async (label, initial=false) => {
    assert.equal(await evaluate(staticContent),true,`${label}: complete static hero`);
    const still = initial ? await evaluate('window.__motionProof.ready') : await evaluate(snapshot);
    assert(still,`${label}: readiness snapshot exists`);
    assert.equal(await evaluate(snapshot),still,`${label}: readiness must follow the initial draw`);
    await motionTicks();
    assert.equal(await evaluate(snapshot),still,`${label}: all reduced-motion stages must stay static`);
  };
  const preference = async reduced => {
    await call('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:reduced?'reduce':'no-preference'}]});
    assert.equal(await evaluate(`matchMedia('(prefers-reduced-motion: reduce)').matches`),reduced);
  };
  const screenshots = process.env.PUBLIC_SITE_SCREENSHOTS || join(tmpdir(), 'public-site-screenshots'); mkdirSync(screenshots,{recursive:true});
  for (const [label,width,height,reduced,blocked,delayed] of [['desktop',1440,1000,false,false,false],['mobile',390,844,false,false,false],['reduced',390,844,true,false,false],['reduced-delayed',390,844,true,false,true],['no-wasm',390,844,false,true,false]]) {
    errors.length=0; failures.length=0; blockedRequests.length=0;
    await call('Emulation.setDeviceMetricsOverride',{width,height,deviceScaleFactor:1,mobile:width<600});
    await preference(reduced);
    await call('Network.setBlockedURLs',{urls:blocked?['*.wasm']:[]});
    let gate;
    if (delayed) gate = await call('Page.addScriptToEvaluateOnNewDocument',{source:`
      window.requestIdleCallback = callback => {
        window.__releaseMotion = () => callback({didTimeout:false,timeRemaining:()=>50});
        return 1;
      };
    `});
    await navigate('/');
    assert.equal(await evaluate(`document.readyState === 'complete' && !!document.querySelector('#heroStage')`),true);
    if (delayed) {
      await until(`typeof window.__releaseMotion === 'function'`);
      assert.equal(await evaluate(`document.documentElement.classList.contains('motion-ready')`),false,'delayed initialization must preserve the fallback until released');
      assert.equal(await evaluate('window.__motionProof.ticks'),0);
      assert.equal(await evaluate(`getComputedStyle(document.querySelector('h1')).opacity`),'1');
      await evaluate('window.__releaseMotion()');
      await call('Page.removeScriptToEvaluateOnNewDocument',{identifier:gate.identifier});
    }
    if (!blocked) {
      await until(`window.__motionProof.ready !== null`);
      assert.equal(await evaluate('window.__motionProof.readyTicks'),0,`${label}: readiness precedes timer ticks`);
    } else {
      await until(() => blockedRequests.some(url => url?.endsWith('.wasm')));
      assert.equal(await evaluate(`document.documentElement.classList.contains('motion-ready')`),false);
    }
    const page = await evaluate(`({text:document.body.innerText, overflow:document.documentElement.scrollWidth > innerWidth, images:[...document.images].filter(i=>!i.complete||i.naturalWidth===0).map(i=>i.src), paint:getComputedStyle(document.querySelector('[data-art]')).getPropertyValue('--paint'), heading:getComputedStyle(document.querySelector('h1')).opacity, viewport:innerWidth})`);
    assert(page.text.includes('Coda handles what happens between visits.') && page.text.includes('A physician-directed AI care team.') && page.text.includes('Consumer Health Data Privacy Policy'),label);
    assert.equal(page.overflow,false,`${label}: horizontal overflow at ${page.viewport}`);
    assert.deepEqual(page.images,[],`${label}: broken images`);
    assert(page.paint.includes('/a/') && page.paint.includes('.svg'),`${label}: unpainted landscape`);
    assert.equal(page.heading,'1',`${label}: invisible heading`);
    if(reduced) {
      await assertStatic(label,true);
      // Exercise both directions without navigation. Reduced content must settle
      // on the very next draw, then stay stable over subsequent completed ticks.
      await preference(false);
      await until(`!${staticContent}`);
      const before = await evaluate(`document.querySelector('#heroStage').innerHTML`);
      await until(`document.querySelector('#heroStage').innerHTML !== ${JSON.stringify(before)}`);
      await preference(true);
      await motionTicks(1);
      await assertStatic(`${label}: changed preference`);
    } else if(!blocked) {
      const before = await evaluate(`document.querySelector('#heroStage').innerHTML`);
      await until(`document.querySelector('#heroStage').innerHTML !== ${JSON.stringify(before)}`);
    }
    await evaluate('scrollTo(0, 700)'); await until(`document.querySelector('nav').classList.contains('scrolled')`);
    await evaluate('scrollTo(0, 0)'); await until(`scrollY === 0 && !document.querySelector('nav').classList.contains('scrolled')`);
    const shot=await call('Page.captureScreenshot',{format:'png',captureBeyondViewport:false});
    const path=join(screenshots,`${label}.png`); writeFileSync(path,Buffer.from(shot.data,'base64')); console.log(`Screenshot: ${path}`);
    assert.deepEqual(failures,[],`${label}: failed local assets`); assert.deepEqual(errors,[],`${label}: uncaught JavaScript errors`);
    if(!blocked){
      await navigate('/duet');
      await until(`window.__motionProof.ready !== null`);
      assert.equal(await evaluate(`document.body.innerText.includes('A clearer picture of you.')`),true);
      assert.equal(await evaluate('document.documentElement.scrollWidth > innerWidth'),false,`${label}: Duet overflow`);
      assert.deepEqual(failures,[],`${label}: Duet failed local assets`); assert.deepEqual(errors,[],`${label}: Duet uncaught JavaScript errors`);
    }
    console.log(`${label}: content, assets, overflow, navigation and motion fallback passed`);
  }
} finally {
  clearTimeout(deadline);
  const exited = new Promise(resolve => child.once('exit', resolve));
  if (socket?.readyState === WebSocket.OPEN) socket.send(JSON.stringify({id:999999,method:'Browser.close'}));
  else child.kill();
  await exited; socket?.close();
  try { rmSync(profile,{recursive:true,force:true,maxRetries:5,retryDelay:100}); }
  catch (error) { console.warn(`Chromium profile cleanup deferred (${profile}): ${error.message}`); }
}
