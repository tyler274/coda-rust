  function rng(a){ return function(){ a|=0; a=a+0x6D2B79F5|0; var t=Math.imul(a^a>>>15,1|a); t=t+Math.imul(t^t>>>7,61|t)^t; return ((t^t>>>14)>>>0)/4294967296; }; }

  var PALETTES = {
    mist:{ sky:['#C9D3CC','#DEE3DC','#EEEDE4','#F3EFE5'], cloud:'#FFFFFF', shade:'#B9C4BD',
           ridges:['#B7C2BC','#9DADA4','#7E9387','#5E7768'], water:['#E3E6DE','#C7D0CA'], bank:'#3F5747', tree:['#34493C','#4A6352','#5C7562'], grain:.20 },
    sage:{ sky:['#C3CEC6','#D8DFD7','#E9EAE1','#F1EEE4'], cloud:'#FBFBF6', shade:'#AFBDB4',
           ridges:['#AEBBB3','#93A69A','#73897B','#56705F'], water:['#DCE2DA','#BFCAC2'], bank:'#3C5444', tree:['#31463A','#46604F','#59735F'], grain:.22 },
    dawn:{ sky:['#D6D6CB','#E8E3D5','#F2E9D9','#F5EEDF'], cloud:'#FFFDF6', shade:'#CFC9B9',
           ridges:['#C7C6B8','#A9B2A4','#889A8B','#647C6B'], water:['#EDE8DC','#D3D3C6'], bank:'#47604E', tree:['#3A5141','#506A56','#66806A'], grain:.20 },
    deep:{ sky:['#4D6457','#6F8476','#97A598','#B4BCAE'], cloud:'#C9CFC2', shade:'#5E7366',
           ridges:['#7E9184','#62786A','#4A6152','#33493B'], water:['#8E9E92','#5D7264'], bank:'#22332A', tree:['#1C2B22','#2A3D31','#36493C'], grain:.26 }
  };

  function ridgePath(r, W, H, base, amp, peaks, jag){
    var P = [];
    for(var i=0;i<peaks;i++) P.push({x:r()*W*1.2-W*.1, h:(.35+r()*.65)*amp, w:(.07+r()*.16)*W});
    var d = 'M0,'+H+' ';
    for(var x=0;x<=W;x+=8){
      var y = 0;
      for(var k=0;k<P.length;k++){ var p=P[k], u=(x-p.x)/p.w; y = Math.max(y, p.h*Math.exp(-u*u)); }
      y += (Math.sin(x*.031+peaks)*.5+Math.sin(x*.0123+amp)*.5)*jag + (r()-.5)*jag*.5;
      d += 'L'+x+','+(base-y).toFixed(1)+' ';
    }
    return d+'L'+W+','+H+' Z';
  }

  function paint(name, seed){
    // Soft-focus landscape: haze, distant ridges and still water, no hard edges.
    var p = PALETTES[name] || PALETTES.mist, r = rng(seed||1), W=1600, H=1000, horizon=600;
    var s = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 '+W+' '+H+'" preserveAspectRatio="xMidYMid slice">';
    s += '<defs>'
      + '<linearGradient id="sk" gradientUnits="userSpaceOnUse" x1="0" y1="0" x2="0" y2="'+H+'">'
      + '<stop offset="0" stop-color="'+p.sky[0]+'"/><stop offset=".42" stop-color="'+p.sky[1]+'"/><stop offset=".58" stop-color="'+p.sky[2]+'"/><stop offset=".64" stop-color="'+p.sky[3]+'"/>'
      + '<stop offset=".7" stop-color="'+p.water[0]+'"/><stop offset="1" stop-color="'+p.water[1]+'"/></linearGradient>'
      + '<filter id="f40" x="-20%" y="-20%" width="140%" height="140%"><feGaussianBlur stdDeviation="40"/></filter>'
      + '<filter id="f14" x="-10%" y="-10%" width="120%" height="120%"><feGaussianBlur stdDeviation="14"/></filter>'
      + '<filter id="f7" x="-10%" y="-10%" width="120%" height="120%"><feGaussianBlur stdDeviation="7"/></filter>'
      + '<filter id="f24" x="-10%" y="-10%" width="120%" height="120%"><feGaussianBlur stdDeviation="24"/></filter>'
      + '<filter id="grain"><feTurbulence type="fractalNoise" baseFrequency=".9" numOctaves="2" seed="'+(seed%31)+'" stitchTiles="stitch"/><feColorMatrix values="0 0 0 0 .08  0 0 0 0 .12  0 0 0 0 .1  0 0 0 1.2 -.5"/></filter>'
      + '</defs>';
    s += '<rect width="'+W+'" height="'+H+'" fill="url(#sk)"/>';
    // drifting cloud light
    s += '<g filter="url(#f40)">';
    for(var c=0;c<7;c++){
      s += '<ellipse cx="'+(r()*W).toFixed(0)+'" cy="'+(80+r()*300).toFixed(0)+'" rx="'+(220+r()*380).toFixed(0)+'" ry="'+(50+r()*80).toFixed(0)+'" fill="'+p.cloud+'" opacity="'+(.3+r()*.35).toFixed(2)+'"/>';
    }
    s += '</g>';
    // ridges: far ones dissolve into haze, the nearest stays soft
    var bases=[horizon-20, horizon+4, horizon+22], amps=[300,210,110], peaks=[5,6,7], blur=['f14','f7','f7'], op=[.55,.7,.85];
    for(var i=0;i<3;i++){
      s += '<path d="'+ridgePath(r,W,H,bases[i],amps[i],peaks[i],2)+'" fill="'+p.ridges[i]+'" opacity="'+op[i]+'" filter="url(#'+blur[i]+')"/>';
      s += '<rect x="-100" y="'+(bases[i]-50)+'" width="'+(W+200)+'" height="90" fill="'+p.sky[3]+'" opacity=".35" filter="url(#f24)"/>';
    }
    // still water with a faint mirrored ridge and mist on the shoreline
    s += '<rect x="-100" y="'+(horizon+20)+'" width="'+(W+200)+'" height="'+(H-horizon)+'" fill="url(#sk)" opacity=".9" filter="url(#f14)"/>';
    s += '<g opacity=".18" filter="url(#f14)" transform="translate(0,'+(2*(horizon+24))+') scale(1,-1)"><path d="'+ridgePath(rng(seed+7),W,H,horizon+24,150,6,2)+'" fill="'+p.ridges[2]+'"/></g>';
    s += '<rect x="-100" y="'+(horizon-6)+'" width="'+(W+200)+'" height="60" fill="'+p.sky[3]+'" opacity=".55" filter="url(#f14)"/>';
    // a soft shore in one corner gives depth without detail
    var left = r() > .5, sx = left ? 0 : W;
    s += '<ellipse cx="'+sx+'" cy="'+(H+40)+'" rx="'+(560+r()*200).toFixed(0)+'" ry="'+(230+r()*60).toFixed(0)+'" fill="'+p.ridges[3]+'" opacity=".55" filter="url(#f24)"/>';
    s += '<rect width="'+W+'" height="'+H+'" filter="url(#grain)" opacity="'+(p.grain*.7).toFixed(2)+'"/>';
    s += '</svg>';
    return 'url("data:image/svg+xml;charset=utf-8,'+encodeURIComponent(s)+'")';
  }

