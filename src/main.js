import Vue from 'vue'
import App from './App.vue'
import VModal from 'vue-js-modal'
//Firebase
import firebase from 'firebase/app'
import 'firebase/firestore'
import VueFirestore from 'vue-firestore'
import {firebaseConfig} from './firebaseconfig'
//Bootstrap-vue
import BootstrapVue from 'bootstrap-vue'
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'
//Autocomplete
import Autocomplete from '@trevoreyre/autocomplete-vue'
import '@trevoreyre/autocomplete-vue/dist/style.css'
//vuex
import {store} from "./store/store";

import {library} from '@fortawesome/fontawesome-svg-core'
import {FontAwesomeIcon} from '@fortawesome/vue-fontawesome'
import {
    faDownload,
    faEnvelope,
    faSlidersH,
    faFileImport,
    faSignInAlt,
    faSignOutAlt,
    faUpload,
    faBroom,
    faMinus,
    faArrowUp,
    faArrowDown,
    faUserCircle,
    faEllipsisV
} from "@fortawesome/free-solid-svg-icons";
import {faGithub, faLinkedin} from '@fortawesome/free-brands-svg-icons'

library.add(faUserCircle);
library.add(faEllipsisV);
library.add(faSignInAlt);
library.add(faBroom);
library.add(faMinus);
library.add(faArrowUp);
library.add(faArrowDown);
library.add(faDownload);
library.add(faSlidersH);
library.add(faUpload);
library.add(faSignOutAlt);
library.add(faFileImport);
library.add(faEnvelope);
library.add(faLinkedin);
library.add(faGithub);

Vue.use(VueFirestore);
Vue.use(Autocomplete);
Vue.use(VModal);
Vue.use(BootstrapVue);
Vue.component('font-awesome-icon', FontAwesomeIcon);
Vue.config.productionTip = false;

firebase.initializeApp(firebaseConfig);

new Vue({
    created() {
    },
    store,
    render: h => h(App),
}).$mount('#app');
