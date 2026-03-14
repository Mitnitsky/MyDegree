module.exports = {
  content: [
    "src/**/*.rs",
    "index.html",
    "style.css",
    "firebase_bridge_src.mjs",
  ],
  css: ["bootstrap.rtl.min.css"],
  output: "bootstrap.purged.css",
  // Rust uses .class("btn btn-primary") — PurgeCSS default extractor
  // handles space-separated class strings inside quotes.
  defaultExtractor: (content) => content.match(/[\w-]+/g) || [],
  // Bootstrap needs these dynamic classes preserved
  safelist: {
    standard: [
      /^modal/,
      /^fade/,
      /^show$/,
      /^collapse/,
      /^collapsing/,
      /^dropdown/,
      /^offcanvas/,
      /^toast/,
      /^tooltip/,
      /^popover/,
      /^carousel/,
      /^spinner/,
      /^visually-hidden/,
    ],
  },
};
