import{j as E,r as S,R as T,a as C}from"./vendor.cca50703.js";const L=function(){const r=document.createElement("link").relList;if(r&&r.supports&&r.supports("modulepreload"))return;for(const e of document.querySelectorAll('link[rel="modulepreload"]'))s(e);new MutationObserver(e=>{for(const t of e)if(t.type==="childList")for(const o of t.addedNodes)o.tagName==="LINK"&&o.rel==="modulepreload"&&s(o)}).observe(document,{childList:!0,subtree:!0});function c(e){const t={};return e.integrity&&(t.integrity=e.integrity),e.referrerpolicy&&(t.referrerPolicy=e.referrerpolicy),e.crossorigin==="use-credentials"?t.credentials="include":e.crossorigin==="anonymous"?t.credentials="omit":t.credentials="same-origin",t}function s(e){if(e.ep)return;e.ep=!0;const t=c(e);fetch(e.href,t)}};L();let u;const _=new Array(32).fill(void 0);_.push(void 0,null,!0,!1);function a(n){return _[n]}let l=_.length;function W(n){n<36||(_[n]=l,l=n)}function v(n){const r=a(n);return W(n),r}function f(n){l===_.length&&_.push(_.length+1);const r=l;return l=_[r],_[r]=n,r}function x(n){const r=typeof n;if(r=="number"||r=="boolean"||n==null)return`${n}`;if(r=="string")return`"${n}"`;if(r=="symbol"){const e=n.description;return e==null?"Symbol":`Symbol(${e})`}if(r=="function"){const e=n.name;return typeof e=="string"&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=x(n[0]));for(let o=1;o<e;o++)t+=", "+x(n[o]);return t+="]",t}const c=/\[object ([^\]]+)\]/.exec(toString.call(n));let s;if(c.length>1)s=c[1];else return toString.call(n);if(s=="Object")try{return"Object("+JSON.stringify(n)+")"}catch{return"Object"}return n instanceof Error?`${n.name}: ${n.message}
${n.stack}`:s}let j=0,w=null;function m(){return(w===null||w.buffer!==u.memory.buffer)&&(w=new Uint8Array(u.memory.buffer)),w}let h=new TextEncoder("utf-8");const I=typeof h.encodeInto=="function"?function(n,r){return h.encodeInto(n,r)}:function(n,r){const c=h.encode(n);return r.set(c),{read:n.length,written:c.length}};function k(n,r,c){if(c===void 0){const i=h.encode(n),b=r(i.length);return m().subarray(b,b+i.length).set(i),j=i.length,b}let s=n.length,e=r(s);const t=m();let o=0;for(;o<s;o++){const i=n.charCodeAt(o);if(i>127)break;t[e+o]=i}if(o!==s){o!==0&&(n=n.slice(o)),e=c(e,s,s=o+n.length*3);const i=m().subarray(e+o,e+s);o+=I(n,i).written}return j=o,e}let y=null;function O(){return(y===null||y.buffer!==u.memory.buffer)&&(y=new Int32Array(u.memory.buffer)),y}let R=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});R.decode();function g(n,r){return R.decode(m().subarray(n,n+r))}function F(n,r,c,s){const e={a:n,b:r,cnt:1,dtor:c},t=(...o)=>{e.cnt++;const i=e.a;e.a=0;try{return s(i,e.b,...o)}finally{--e.cnt==0?u.__wbindgen_export_2.get(e.dtor)(i,e.b):e.a=i}};return t.original=e,t}function $(n,r,c){u._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h94b451d680eff28d(n,r,f(c))}function A(n){return n==null}function d(n,r){try{return n.apply(this,r)}catch(c){u.__wbindgen_exn_store(f(c))}}async function N(n,r){if(typeof Response=="function"&&n instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(n,r)}catch(s){if(n.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",s);else throw s}const c=await n.arrayBuffer();return await WebAssembly.instantiate(c,r)}else{const c=await WebAssembly.instantiate(n,r);return c instanceof WebAssembly.Instance?{instance:c,module:n}:c}}async function M(n){typeof n=="undefined"&&(n=new URL("/jubiliant-robot/assets/wasm_bg.6be9dc0b.wasm",self.location));const r={};r.wbg={},r.wbg.__wbindgen_object_drop_ref=function(e){v(e)},r.wbg.__wbg_log_682923c8ea4d4d53=function(e,t){console.log(g(e,t))},r.wbg.__wbindgen_cb_drop=function(e){const t=v(e).original;if(t.cnt--==1)return t.a=0,!0;var o=!1;return o},r.wbg.__wbg_instanceof_Window_c4b70662a0d2c5ec=function(e){var t=a(e)instanceof Window;return t},r.wbg.__wbg_document_1c64944725c0d81d=function(e){var t=a(e).document;return A(t)?0:f(t)},r.wbg.__wbg_getElementById_f3e94458ce77f0d0=function(e,t,o){var i=a(e).getElementById(g(t,o));return A(i)?0:f(i)},r.wbg.__wbg_addEventListener_52721772cc0a7f30=function(){return d(function(e,t,o,i){a(e).addEventListener(g(t,o),a(i))},arguments)},r.wbg.__wbg_instanceof_CanvasRenderingContext2d_3abbe7ec7af32cae=function(e){var t=a(e)instanceof CanvasRenderingContext2D;return t},r.wbg.__wbg_beginPath_733d5a9e3e769d24=function(e){a(e).beginPath()},r.wbg.__wbg_stroke_7cdcdf3d07636d76=function(e){a(e).stroke()},r.wbg.__wbg_lineTo_fde385edd804f315=function(e,t,o){a(e).lineTo(t,o)},r.wbg.__wbg_moveTo_18ace182fe51d75d=function(e,t,o){a(e).moveTo(t,o)},r.wbg.__wbg_clearRect_07caefec3496ced1=function(e,t,o,i,b){a(e).clearRect(t,o,i,b)},r.wbg.__wbg_offsetX_5da3ebf8a8cda8a4=function(e){var t=a(e).offsetX;return t},r.wbg.__wbg_offsetY_b0edbc16723a55cb=function(e){var t=a(e).offsetY;return t},r.wbg.__wbg_instanceof_HtmlCanvasElement_25d964a0dde6717e=function(e){var t=a(e)instanceof HTMLCanvasElement;return t},r.wbg.__wbg_width_555f63ab09ba7d3f=function(e){var t=a(e).width;return t},r.wbg.__wbg_height_7153faec70fbaf7b=function(e){var t=a(e).height;return t},r.wbg.__wbg_getContext_f701d0231ae22393=function(){return d(function(e,t,o){var i=a(e).getContext(g(t,o));return A(i)?0:f(i)},arguments)},r.wbg.__wbg_newnoargs_be86524d73f67598=function(e,t){var o=new Function(g(e,t));return f(o)},r.wbg.__wbg_call_888d259a5fefc347=function(){return d(function(e,t){var o=a(e).call(a(t));return f(o)},arguments)},r.wbg.__wbg_self_c6fbdfc2918d5e58=function(){return d(function(){var e=self.self;return f(e)},arguments)},r.wbg.__wbg_window_baec038b5ab35c54=function(){return d(function(){var e=window.window;return f(e)},arguments)},r.wbg.__wbg_globalThis_3f735a5746d41fbd=function(){return d(function(){var e=globalThis.globalThis;return f(e)},arguments)},r.wbg.__wbg_global_1bc0b39582740e95=function(){return d(function(){var e=global.global;return f(e)},arguments)},r.wbg.__wbindgen_is_undefined=function(e){var t=a(e)===void 0;return t},r.wbg.__wbindgen_object_clone_ref=function(e){var t=a(e);return f(t)},r.wbg.__wbindgen_debug_string=function(e,t){var o=x(a(t)),i=k(o,u.__wbindgen_malloc,u.__wbindgen_realloc),b=j;O()[e/4+1]=b,O()[e/4+0]=i},r.wbg.__wbindgen_throw=function(e,t){throw new Error(g(e,t))},r.wbg.__wbindgen_rethrow=function(e){throw v(e)},r.wbg.__wbindgen_closure_wrapper60=function(e,t,o){var i=F(e,t,20,$);return f(i)},(typeof n=="string"||typeof Request=="function"&&n instanceof Request||typeof URL=="function"&&n instanceof URL)&&(n=fetch(n));const{instance:c,module:s}=await N(await n,r);return u=c.exports,M.__wbindgen_wasm_module=s,u.__wbindgen_start(),u}const p=E.exports.jsx,U=E.exports.jsxs;function B(){return S.exports.useEffect(()=>{M()},[]),U("main",{style:{width:1024,height:768},children:[p("canvas",{id:"control-canvas",width:1024,height:768}),p("canvas",{id:"paint-canvas",width:1024,height:768})]})}T.render(p(C.StrictMode,{children:p(B,{})}),document.getElementById("root"));