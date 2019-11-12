import Vue from 'vue'
import App from './App.vue'
import VModal from 'vue-js-modal'
import BootstrapVue from 'bootstrap-vue'
import Autocomplete from '@trevoreyre/autocomplete-vue'
import '@trevoreyre/autocomplete-vue/dist/style.css'
import firebase from 'firebase'
import VueFirestore from 'vue-firestore'

import {firebaseConfig} from './firebaseconfig'
import 'bootstrap/dist/css/bootstrap.css'

import 'bootstrap-vue/dist/bootstrap-vue.css'
//vuex
import {store} from "./store/store";
//firestore
require('firebase/firestore');

Vue.use(VueFirestore);
Vue.use(Autocomplete);
Vue.use(VModal);
Vue.use(BootstrapVue);

Vue.config.productionTip = false;

// git checkout master
// git pull               # to update the state to the latest remote master state
// git merge develop      # to bring changes to local master from your develop branch
// git push origin master # push current HEAD to remote master branch

firebase.initializeApp(firebaseConfig)
const firestore = firebase.firestore();
new Vue({
    created() {
        window.console.log(this.$firestore.users);
    },
    firestore: function() {
      return {
          users: firestore.collection('users')
      }
    },cd 
    store,
    render: h => h(App),
}).$mount('#app');
