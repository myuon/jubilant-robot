import{j as E,r as S,R as T,a as C}from"./vendor.cca50703.js";const L=function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const e of document.querySelectorAll('link[rel="modulepreload"]'))s(e);new MutationObserver(e=>{for(const r of e)if(r.type==="childList")for(const o of r.addedNodes)o.tagName==="LINK"&&o.rel==="modulepreload"&&s(o)}).observe(document,{childList:!0,subtree:!0});function c(e){const r={};return e.integrity&&(r.integrity=e.integrity),e.referrerpolicy&&(r.referrerPolicy=e.referrerpolicy),e.crossorigin==="use-credentials"?r.credentials="include":e.crossorigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function s(e){if(e.ep)return;e.ep=!0;const r=c(e);fetch(e.href,r)}};L();let u;const _=new Array(32).fill(void 0);_.push(void 0,null,!0,!1);function a(n){return _[n]}let g=_.length;function W(n){n<36||(_[n]=g,g=n)}function v(n){const t=a(n);return W(n),t}function f(n){g===_.length&&_.push(_.length+1);const t=g;return g=_[t],_[t]=n,t}function x(n){const t=typeof n;if(t=="number"||t=="boolean"||n==null)return`${n}`;if(t=="string")return`"${n}"`;if(t=="symbol"){const e=n.description;return e==null?"Symbol":`Symbol(${e})`}if(t=="function"){const e=n.name;return typeof e=="string"&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let r="[";e>0&&(r+=x(n[0]));for(let o=1;o<e;o++)r+=", "+x(n[o]);return r+="]",r}const c=/\[object ([^\]]+)\]/.exec(toString.call(n));let s;if(c.length>1)s=c[1];else return toString.call(n);if(s=="Object")try{return"Object("+JSON.stringify(n)+")"}catch{return"Object"}return n instanceof Error?`${n.name}: ${n.message}
${n.stack}`:s}let j=0,w=null;function m(){return(w===null||w.buffer!==u.memory.buffer)&&(w=new Uint8Array(u.memory.buffer)),w}let h=new TextEncoder("utf-8");const I=typeof h.encodeInto=="function"?function(n,t){return h.encodeInto(n,t)}:function(n,t){const c=h.encode(n);return t.set(c),{read:n.length,written:c.length}};function k(n,t,c){if(c===void 0){const i=h.encode(n),b=t(i.length);return m().subarray(b,b+i.length).set(i),j=i.length,b}let s=n.length,e=t(s);const r=m();let o=0;for(;o<s;o++){const i=n.charCodeAt(o);if(i>127)break;r[e+o]=i}if(o!==s){o!==0&&(n=n.slice(o)),e=c(e,s,s=o+n.length*3);const i=m().subarray(e+o,e+s);o+=I(n,i).written}return j=o,e}let y=null;function O(){return(y===null||y.buffer!==u.memory.buffer)&&(y=new Int32Array(u.memory.buffer)),y}let R=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});R.decode();function l(n,t){return R.decode(m().subarray(n,n+t))}function F(n,t,c,s){const e={a:n,b:t,cnt:1,dtor:c},r=(...o)=>{e.cnt++;const i=e.a;e.a=0;try{return s(i,e.b,...o)}finally{--e.cnt==0?u.__wbindgen_export_2.get(e.dtor)(i,e.b):e.a=i}};return r.original=e,r}function $(n,t,c){u._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h94b451d680eff28d(n,t,f(c))}function A(n){return n==null}function d(n,t){try{return n.apply(this,t)}catch(c){u.__wbindgen_exn_store(f(c))}}async function N(n,t){if(typeof Response=="function"&&n instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(n,t)}catch(s){if(n.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",s);else throw s}const c=await n.arrayBuffer();return await WebAssembly.instantiate(c,t)}else{const c=await WebAssembly.instantiate(n,t);return c instanceof WebAssembly.Instance?{instance:c,module:n}:c}}async function M(n){typeof n=="undefined"&&(n=new URL("/assets/wasm_bg.04bba1b7.wasm",self.location));const t={};t.wbg={},t.wbg.__wbindgen_object_drop_ref=function(e){v(e)},t.wbg.__wbindgen_cb_drop=function(e){const r=v(e).original;if(r.cnt--==1)return r.a=0,!0;var o=!1;return o},t.wbg.__wbg_instanceof_Window_c4b70662a0d2c5ec=function(e){var r=a(e)instanceof Window;return r},t.wbg.__wbg_document_1c64944725c0d81d=function(e){var r=a(e).document;return A(r)?0:f(r)},t.wbg.__wbg_getElementById_f3e94458ce77f0d0=function(e,r,o){var i=a(e).getElementById(l(r,o));return A(i)?0:f(i)},t.wbg.__wbg_addEventListener_52721772cc0a7f30=function(){return d(function(e,r,o,i){a(e).addEventListener(l(r,o),a(i))},arguments)},t.wbg.__wbg_instanceof_CanvasRenderingContext2d_3abbe7ec7af32cae=function(e){var r=a(e)instanceof CanvasRenderingContext2D;return r},t.wbg.__wbg_beginPath_733d5a9e3e769d24=function(e){a(e).beginPath()},t.wbg.__wbg_stroke_7cdcdf3d07636d76=function(e){a(e).stroke()},t.wbg.__wbg_lineTo_fde385edd804f315=function(e,r,o){a(e).lineTo(r,o)},t.wbg.__wbg_moveTo_18ace182fe51d75d=function(e,r,o){a(e).moveTo(r,o)},t.wbg.__wbg_clearRect_07caefec3496ced1=function(e,r,o,i,b){a(e).clearRect(r,o,i,b)},t.wbg.__wbg_offsetX_5da3ebf8a8cda8a4=function(e){var r=a(e).offsetX;return r},t.wbg.__wbg_offsetY_b0edbc16723a55cb=function(e){var r=a(e).offsetY;return r},t.wbg.__wbg_instanceof_HtmlCanvasElement_25d964a0dde6717e=function(e){var r=a(e)instanceof HTMLCanvasElement;return r},t.wbg.__wbg_width_555f63ab09ba7d3f=function(e){var r=a(e).width;return r},t.wbg.__wbg_height_7153faec70fbaf7b=function(e){var r=a(e).height;return r},t.wbg.__wbg_getContext_f701d0231ae22393=function(){return d(function(e,r,o){var i=a(e).getContext(l(r,o));return A(i)?0:f(i)},arguments)},t.wbg.__wbg_newnoargs_be86524d73f67598=function(e,r){var o=new Function(l(e,r));return f(o)},t.wbg.__wbg_call_888d259a5fefc347=function(){return d(function(e,r){var o=a(e).call(a(r));return f(o)},arguments)},t.wbg.__wbg_self_c6fbdfc2918d5e58=function(){return d(function(){var e=self.self;return f(e)},arguments)},t.wbg.__wbg_window_baec038b5ab35c54=function(){return d(function(){var e=window.window;return f(e)},arguments)},t.wbg.__wbg_globalThis_3f735a5746d41fbd=function(){return d(function(){var e=globalThis.globalThis;return f(e)},arguments)},t.wbg.__wbg_global_1bc0b39582740e95=function(){return d(function(){var e=global.global;return f(e)},arguments)},t.wbg.__wbindgen_is_undefined=function(e){var r=a(e)===void 0;return r},t.wbg.__wbindgen_object_clone_ref=function(e){var r=a(e);return f(r)},t.wbg.__wbindgen_debug_string=function(e,r){var o=x(a(r)),i=k(o,u.__wbindgen_malloc,u.__wbindgen_realloc),b=j;O()[e/4+1]=b,O()[e/4+0]=i},t.wbg.__wbindgen_throw=function(e,r){throw new Error(l(e,r))},t.wbg.__wbindgen_rethrow=function(e){throw v(e)},t.wbg.__wbindgen_closure_wrapper54=function(e,r,o){var i=F(e,r,14,$);return f(i)},(typeof n=="string"||typeof Request=="function"&&n instanceof Request||typeof URL=="function"&&n instanceof URL)&&(n=fetch(n));const{instance:c,module:s}=await N(await n,t);return u=c.exports,M.__wbindgen_wasm_module=s,u.__wbindgen_start(),u}const p=E.exports.jsx,U=E.exports.jsxs;function B(){return S.exports.useEffect(()=>{M()},[]),U("main",{style:{width:1024,height:768},children:[p("canvas",{id:"control-canvas",width:1024,height:768}),p("canvas",{id:"paint-canvas",width:1024,height:768})]})}T.render(p(C.StrictMode,{children:p(B,{})}),document.getElementById("root"));
