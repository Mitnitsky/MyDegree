import { createApp } from "vue";
import App from "./App.vue";
import { store } from "./store";
//Firebase
import firebase from "firebase/compat/app";
import "firebase/compat/firestore";
import "firebase/compat/analytics";
import "./firebaseconfig";

//Element
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import VueFinalModal from "vue-final-modal";
import Autocomplete from "vue3-autocomplete";
// Optional: Import default CSS
import "vue3-autocomplete/dist/vue3-autocomplete.css";
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
firebase.analytics();

export const db = firebase.firestore();
export const auth = firebase.auth();

const app = createApp(App);
app.use(store);

app.use(ElementPlus, {
  locale: He,
});
app.use(VueFinalModal());
app.component("Autocomplete", Autocomplete);
app.component("FontAwesomeIcon", FontAwesomeIcon);
app.mount("#app");
