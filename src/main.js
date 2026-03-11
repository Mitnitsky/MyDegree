import { createApp } from "vue";
import App from "./App.vue";
//Firebase
import firebase from "firebase/app";
import "firebase/firestore";
import "firebase/analytics";
import { firebaseConfig } from "./firebaseconfig";
//Bootstrap-vue-next
import * as BVN from "bootstrap-vue-next";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue-next/dist/bootstrap-vue-next.css";
//vuex
import { store } from "./store/store";

import { library } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import {
  faArrowDown,
  faArrowUp,
  faBroom,
  faBan,
  faDownload,
  faEllipsisV,
  faEnvelope,
  faFileImport,
  faMinus,
  faChartBar,
  faTrash,
  faShareSquare,
  faCheck,
  faSignInAlt,
  faSignOutAlt,
  faSlidersH,
  faUpload,
  faUserCircle,
} from "@fortawesome/free-solid-svg-icons";
import { faGithub, faLinkedin } from "@fortawesome/free-brands-svg-icons";

library.add(
  faUserCircle,
  faCheck,
  faBan,
  faShareSquare,
  faEllipsisV,
  faSignInAlt,
  faBroom,
  faMinus,
  faArrowUp,
  faArrowDown,
  faDownload,
  faSlidersH,
  faChartBar,
  faTrash,
  faUpload,
  faSignOutAlt,
  faFileImport,
  faEnvelope,
  faLinkedin,
  faGithub
);

firebase.initializeApp(firebaseConfig);
firebase.analytics();

const app = createApp(App);
app.use(store);
app.use(BVN.createBootstrap());

// Register all bootstrap-vue-next components globally
Object.entries(BVN).forEach(([name, component]) => {
  if (/^B[A-Z]/.test(name) && typeof component === "object" && component) {
    app.component(name, component);
  }
});

// Register all bootstrap-vue-next directives globally
const directiveMap = {
  vBModal: "b-modal",
  vBToggle: "b-toggle",
  vBTooltip: "b-tooltip",
  vBPopover: "b-popover",
  vBScrollspy: "b-scrollspy",
  vBColorMode: "b-color-mode",
};
Object.entries(directiveMap).forEach(([exportName, directiveName]) => {
  if (BVN[exportName]) {
    app.directive(directiveName, BVN[exportName]);
  }
});

app.component("FontAwesomeIcon", FontAwesomeIcon);
app.mount("#app");
