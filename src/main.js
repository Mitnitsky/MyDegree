import Vue from 'vue'
import App from './App.vue'
import VModal from 'vue-js-modal'
import BootstrapVue from 'bootstrap-vue'
import Autocomplete from '@trevoreyre/autocomplete-vue'
import '@trevoreyre/autocomplete-vue/dist/style.css'
import firebase from 'firebase/app'
import VueFirestore from 'vue-firestore'

import {firebaseConfig} from './firebaseconfig'
import 'bootstrap/dist/css/bootstrap.css'

import 'bootstrap-vue/dist/bootstrap-vue.css'
//vuex
import {store} from "./store/store";

//firestore
import  'firebase/firestore'

import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import {  faSignInAlt,faSignOutAlt, faFileImport, faEnvelope, faDownload, faUpload} from "@fortawesome/free-solid-svg-icons";
import { faLinkedin,faGithub } from '@fortawesome/free-brands-svg-icons'


library.add(faSignInAlt);
library.add(faDownload);
library.add(faUpload);
library.add(faSignOutAlt);
library.add(faFileImport);
library.add(faEnvelope);
library.add(faLinkedin);
library.add(faGithub);

Vue.component('font-awesome-icon', FontAwesomeIcon);

Vue.config.productionTip = false;

Vue.use(VueFirestore);
Vue.use(Autocomplete);
Vue.use(VModal);
Vue.use(BootstrapVue);

Vue.config.productionTip = false;

// git checkout master
// git pull               # to update the state to the latest remote master state
// git merge develop      # to bring changes to local master from your develop branch
// git push origin master # push current HEAD to remote master branch

firebase.initializeApp(firebaseConfig);

new Vue({
    created() {
    },
    store,
    render: h => h(App),
}).$mount('#app');
