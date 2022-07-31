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
  faBan,
  faBroom,
  faChartBar,
  faCheck,
  faDownload,
  faEllipsisV,
  faEnvelope,
  faFileImport,
  faMinus,
  faShareSquare,
  faSignInAlt,
  faSignOutAlt,
  faSlidersH,
  faSun,
  faTrash,
  faUpload,
  faUserCircle,
} from "@fortawesome/free-solid-svg-icons";
import { faGithub, faLinkedin } from "@fortawesome/free-brands-svg-icons";
import { firebaseConfig } from "@/firebaseconfig";

library.add(
  faUserCircle,
  faCheck,
  faBan,
  faShareSquare,
  faEllipsisV,
  faSun,
  faSignInAlt,
  faBroom,
  faMinus,
  faArrowUp,
  faArrowDown,
  faDownload,
  faEllipsisV,
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

export const db = firebase.firestore();
export const auth = firebase.auth();

const app = createApp(App);
app.use(store);

app.use(ElementPlus, {
  locale: He,
});
app.use(VueFinalModal());
app.component("AutoComplete", Autocomplete);
app.component("FontAwesomeIcon", FontAwesomeIcon);
app.mount("#app");
