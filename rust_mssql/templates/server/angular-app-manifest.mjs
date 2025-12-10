
export default {
  bootstrap: () => import('./main.server.mjs').then(m => m.default),
  inlineCriticalCss: true,
  baseHref: '/',
  locale: undefined,
  routes: [
  {
    "renderMode": 2,
    "route": "/"
  },
  {
    "renderMode": 2,
    "route": "/static"
  },
  {
    "renderMode": 2,
    "route": "/about"
  },
  {
    "renderMode": 2,
    "route": "/contact"
  },
  {
    "renderMode": 2,
    "route": "/profile"
  },
  {
    "renderMode": 2,
    "route": "/productlist"
  },
  {
    "renderMode": 2,
    "route": "/productcatalog"
  },
  {
    "renderMode": 2,
    "route": "/productsearch"
  },
  {
    "renderMode": 2,
    "route": "/**"
  }
],
  entryPointToBrowserMapping: undefined,
  assets: {
    'index.csr.html': {size: 9776, hash: '971187a933dcf223c9616c6b4aee9c3ca6b0dd5921d57412d70605b11cae7897', text: () => import('./assets-chunks/index_csr_html.mjs').then(m => m.default)},
    'index.server.html': {size: 1023, hash: '7198a568ad19ae302077aa2fed60c60c9947495cbfd10d92a8822adbfa440e14', text: () => import('./assets-chunks/index_server_html.mjs').then(m => m.default)},
    'index.html': {size: 53903, hash: '32b204e087d1c46df51d0298c1e192ee8de295e69332159bfaa820ca0d9e2d8e', text: () => import('./assets-chunks/index_html.mjs').then(m => m.default)},
    'static/index.html': {size: 41515, hash: '7c1550bed89d0faff313165c2bc6489932835f17deda8344aa2c7848775e0b0a', text: () => import('./assets-chunks/static_index_html.mjs').then(m => m.default)},
    'productsearch/index.html': {size: 46917, hash: 'a3eab544ba92d104c7a61b9f11710a72cf63ef57c8f62c6b38a844c7a303c32b', text: () => import('./assets-chunks/productsearch_index_html.mjs').then(m => m.default)},
    'contact/index.html': {size: 46843, hash: '93509c6426824086866f6e6b22d074dad023bb13122cb6d9e74510630201e896', text: () => import('./assets-chunks/contact_index_html.mjs').then(m => m.default)},
    'productcatalog/index.html': {size: 47902, hash: '80b418ea94b2f46595d838ded63f602b71cfe93114e16c7bba4e1dfd80e20ba4', text: () => import('./assets-chunks/productcatalog_index_html.mjs').then(m => m.default)},
    'productlist/index.html': {size: 52829, hash: 'a915c6f7ad1bceed931108a2d077ed38bcb17df07e419e7b6ec79b9219b5fa80', text: () => import('./assets-chunks/productlist_index_html.mjs').then(m => m.default)},
    'about/index.html': {size: 47398, hash: '5a8808e1f681db34dcba36cdd1bd6af095876dc1ed8f92ffe5d00a9730b54870', text: () => import('./assets-chunks/about_index_html.mjs').then(m => m.default)},
    'profile/index.html': {size: 61213, hash: 'e7e19872f47024db092e8e9ce3c6570df2a6feabf2232fb9b2a9b1b1900fbbfc', text: () => import('./assets-chunks/profile_index_html.mjs').then(m => m.default)},
    'styles.css': {size: 396704, hash: 'd+zBPSfIKoU', text: () => import('./assets-chunks/styles_css.mjs').then(m => m.default)}
  },
};
