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
  socket.onmessage = ({data}) => {
    const message = JSON.parse(data);
    if (message.id && pending.has(message.id)) { const item = pending.get(message.id); pending.delete(message.id); message.error ? item.reject(new Error(JSON.stringify(message.error))) : item.resolve(message.result); }
    if (message.method === 'Runtime.exceptionThrown') errors.push(message.params.exceptionDetails);
    if (message.method === 'Network.responseReceived' && message.params.response.url.startsWith(origin) && message.params.response.status >= 400) failures.push(message.params.response);
  };
  const call = (method, params={}) => new Promise((resolve, reject) => {const id = ++next; pending.set(id, {resolve,reject}); socket.send(JSON.stringify({id,method,params}));});
  const evaluate = async expression => {const result = await call('Runtime.evaluate', {expression, returnByValue:true, awaitPromise:true}); assert(!result.exceptionDetails, JSON.stringify(result)); return result.result.value;};
  const until = async expression => {for(let i=0;i<150;i++){if(await evaluate(expression))return; await delay(100);} throw new Error(`Timed out: ${expression}`);};
  await call('Page.enable'); await call('Runtime.enable'); await call('Network.enable');
  const screenshots = process.env.PUBLIC_SITE_SCREENSHOTS || join(tmpdir(), 'public-site-screenshots'); mkdirSync(screenshots,{recursive:true});
  for (const [label,width,height,reduced,blocked] of [['desktop',1440,1000,false,false],['mobile',390,844,false,false],['reduced',390,844,true,false],['no-wasm',390,844,false,true]]) {
    errors.length=0; failures.length=0;
    await call('Emulation.setDeviceMetricsOverride',{width,height,deviceScaleFactor:1,mobile:width<600});
    await call('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:reduced?'reduce':'no-preference'}]});
    await call('Network.setBlockedURLs',{urls:blocked?['*.wasm']:[]});
    await call('Page.navigate',{url:origin+'/'});
    await until(`document.readyState === 'complete' && !!document.querySelector('#heroStage')`);
    if (!blocked) await until(`document.documentElement.classList.contains('motion-ready')`);
    else {await delay(2000); assert.equal(await evaluate(`document.documentElement.classList.contains('motion-ready')`),false);}
    const page = await evaluate(`({text:document.body.innerText, overflow:document.documentElement.scrollWidth > innerWidth, images:[...document.images].filter(i=>!i.complete||i.naturalWidth===0).map(i=>i.src), paint:getComputedStyle(document.querySelector('[data-art]')).getPropertyValue('--paint'), heading:getComputedStyle(document.querySelector('h1')).opacity, viewport:innerWidth})`);
    assert(page.text.includes('Coda handles what happens between visits.') && page.text.includes('A physician-directed AI care team.') && page.text.includes('Consumer Health Data Privacy Policy'),label);
    assert.equal(page.overflow,false,`${label}: horizontal overflow at ${page.viewport}`);
    assert.deepEqual(page.images,[],`${label}: broken images`);
    assert(page.paint.includes('/a/') && page.paint.includes('.svg'),`${label}: unpainted landscape`);
    assert.equal(page.heading,'1',`${label}: invisible heading`);
    if(reduced) {
      assert.equal(await evaluate(`matchMedia('(prefers-reduced-motion: reduce)').matches`),true);
      const still = await evaluate(`document.querySelector('#heroStage').innerHTML`);
      await delay(500);
      assert.equal(await evaluate(`document.querySelector('#heroStage').innerHTML`),still, 'reduced-motion hero must stay static');
    } else if(!blocked) {
      const before = await evaluate(`document.querySelector('#heroStage').innerHTML`);
      await until(`document.querySelector('#heroStage').innerHTML !== ${JSON.stringify(before)}`);
    }
    await evaluate('scrollTo(0, 700)'); await until(`document.querySelector('nav').classList.contains('scrolled')`);
    await evaluate('scrollTo(0, 0)'); await delay(300);
    const shot=await call('Page.captureScreenshot',{format:'png',captureBeyondViewport:false});
    const path=join(screenshots,`${label}.png`); writeFileSync(path,Buffer.from(shot.data,'base64')); console.log(`Screenshot: ${path}`);
    assert.deepEqual(failures,[],`${label}: failed local assets`); assert.deepEqual(errors,[],`${label}: uncaught JavaScript errors`);
    if(!blocked){
      await call('Page.navigate',{url:origin+'/duet'}); await until(`document.readyState === 'complete' && document.body.innerText.includes('A clearer picture of you.')`);
      assert.equal(await evaluate('document.documentElement.scrollWidth > innerWidth'),false,`${label}: Duet overflow`);
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
