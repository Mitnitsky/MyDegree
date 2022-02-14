import { createApp } from "vue";
import App from "./App.vue";
import { store } from "./store";
//Firebase
import firebase from "firebase/compat/app";
import "firebase/compat/auth";
import "firebase/compat/firestore";
import { getAnalytics } from "firebase/analytics";
import "./firebaseconfig";

//Element
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import He from "element-plus/es/locale/lang/he";

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
import { firebaseConfig } from "@/firebaseconfig";

library.add(faUserCircle);
library.add(faCheck);
library.add(faBan);
library.add(faShareSquare);
library.add(faEllipsisV);
library.add(faSignInAlt);
library.add(faBroom);
library.add(faMinus);
library.add(faArrowUp);
library.add(faArrowDown);
library.add(faDownload);
library.add(faEllipsisV);
library.add(faSlidersH);
library.add(faChartBar);
library.add(faTrash);
library.add(faUpload);
library.add(faSignOutAlt);
library.add(faFileImport);
library.add(faEnvelope);
library.add(faLinkedin);
library.add(faGithub);

firebase.initializeApp(firebaseConfig);
getAnalytics();


const app = createApp(App);
app.use(store);
app.use(ElementPlus, {
  locale: He,
});
app.component("FontAwesomeIcon", FontAwesomeIcon);
app.mount("#app");
