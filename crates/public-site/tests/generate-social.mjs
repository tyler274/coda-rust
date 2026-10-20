// Reproducible, read-only translation of the scheduling ImageResponse reference.
// Run in Nix from the repository root; output is a checked-in public-site asset.
import {readFileSync,writeFileSync,mkdtempSync,mkdirSync,rmSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {tmpdir} from 'node:os';
import {runInNewContext} from 'node:vm';
import {execFileSync} from 'node:child_process';
const source=readFileSync('submodules/kytos/app/scheduling/dev/opengraph-image.tsx','utf8');
let html=source.slice(source.indexOf('      <div'),source.lastIndexOf('      </div>')+'      </div>'.length);
html=html.replace(/style=\{\{([\s\S]*?)\}\}/g,(_,body)=>{
  const style=runInNewContext('({'+body+'})');
  return 'style="'+Object.entries(style).map(([key,value])=>`${key.replace(/[A-Z]/g,c=>'-'+c.toLowerCase())}:${typeof value==='number'&&!['flex','fontWeight','lineHeight'].includes(key)?value+'px':value}`).join(';')+'"';
});
const logo=readFileSync('crates/public-site/assets/brand/coda-health-logo-transparent.png').toString('base64');
html=html.replace('src={logoUrl}',`src="data:image/png;base64,${logo}"`);
const directory=mkdtempSync(join(tmpdir(),'public-site-og-'));
const output=resolve('crates/public-site/assets/scheduling/dev/opengraph-image.png');
mkdirSync(resolve('crates/public-site/assets/scheduling/dev'),{recursive:true});
try {
  writeFileSync(join(directory,'image.html'),`<!doctype html><html><style>*{box-sizing:border-box}html,body{margin:0;width:1200px;height:630px}body>div{height:630px!important}</style><body>${html}</body></html>`);
  const env={...process.env};delete env.LD_LIBRARY_PATH;
  execFileSync(process.env.CHROMIUM||'chromium',['--headless=new','--no-sandbox','--disable-gpu','--hide-scrollbars','--no-pdf-header-footer','--force-device-scale-factor=1','--window-size=1200,630',`--screenshot=${output}`,`file://${join(directory,'image.html')}`],{env,stdio:'inherit'});
  console.log(output);
}finally{rmSync(directory,{recursive:true,force:true});}
