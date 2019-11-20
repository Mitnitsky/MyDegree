<template>
  <b-navbar toggleable="lg"
            type="dark"
            variant="dark">
    <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>
    <b-collapse id="nav-collapse"
                is-nav>
      <b-navbar-nav align="start">
        <template v-if="this.logged">
          <b-nav-text href="#"
                      style="font-size: 18px;color: lightgray;margin-left: 10px;">
            שלום {{this.user_name}} !
          </b-nav-text>
          <font-awesome-icon icon="sign-out-alt"
                             size="lg"
                             style="margin-left: 5px;margin-top: 10px;color: lightgray"/>
          <b-nav-item DIR="ltr"
                      style="font-size: 18px;text-decoration: underline;color: lightgray"
                      @click="signOut">יציאה
          </b-nav-item>
        </template>
        <template v-else>
          <font-awesome-icon icon="sign-in-alt"
                             rotation="180"

                             size="lg"
                             style="color: lightgray;margin-left: 5px;font-size: 20px;text-decoration: underline;margin-top:10px"
                             v-b-modal.modal-1/>
          <b-nav-brand href="#"
                       style="color: lightgray;text-decoration-line: underline">
            <b-nav-item v-b-modal.modal-1>כניסה</b-nav-item>
          </b-nav-brand>
          <!--          <b-nav-item DIR="ltr"-->
          <!--                      style="font-size: 20px;text-decoration: underline;"-->
          <!--                      v-b-modal.modal-1>כניסה-->
          <!--          </b-nav-item>-->
          <b-modal header-bg-variant="primary"
                   header-text-variant="white"
                   hide-footer
                   hide-header-close
                   id="modal-1"
                   ok-title="סגור"
                   ref="auth-modal"
                   size="md"
                   title="כניסה">
            <authentication></authentication>
            <b-button @click="hideModal"
                      block
                      class="mt-3"
                      variant="outline-primary">סגור
            </b-button>
          </b-modal>

        </template>
      </b-navbar-nav>
      <b-navbar-nav class="mr-auto">
        <b-navbar-brand href="#"
                        mar
                        style='font-family: "Arial", “Helvetica Neue”, Helvetica, Arial, sans-serif;'>
          Degree Planer
          <img alt=""
               src="../assets/main_icon_white.svg"
               style="width: 48px; height: 48px;margin-right: 5px;"/>
        </b-navbar-brand>
      </b-navbar-nav>
    </b-collapse>

  </b-navbar>
</template>

<script>
    import firebase from "firebase/app"
    import Authentication from "./HeaderAuthentication";
    import {mapFields} from 'vuex-map-fields';
    import 'firebase/auth'
    import 'firebase/firestore'

    export default {
        components: {Authentication},
        name: "HeaderNavBar",
        computed: {
            ...mapFields([
                'user_name',
                'logged'

            ])
        },
        mounted() {
            firebase.auth().onAuthStateChanged((user) => {
                if (user) {
                    localStorage.setItem('authenticated', 'true');
                    this.logged = true;
                    this.user = user;
                    this.user_name = user.displayName;
                    if (this.$refs['auth-modal']) {
                        this.$refs['auth-modal'].hide();
                    }
                    let uid = firebase.auth().currentUser.uid;
                    firebase.firestore().collection('users').doc(uid).get().then((doc) => {
                        if (doc.exists) {
                            this.$store.commit('fetchUserInfo', doc.data());
                            this.$store.commit('reCalcCurrentSemester');
                        } else {
                            firebase.firestore().collection('users').doc(uid).set(this.$store.state.user).catch(error => {
                                // eslint-disable-next-line no-console
                                console.log('ErrorHeader - ' + error.message);
                            });
                        }
                    }).catch(error => {
                        // eslint-disable-next-line no-console
                        console.log('ErrorHeader2 - ' + error.message);
                    });
                }
            });
        },
        data() {
            return {}
        },
        methods: {
            signOut() {
                firebase.auth().signOut();
                localStorage.setItem('authenticated', 'false');
                this.logged = false;
                this.$store.commit('clearUserData');
            },
            hideModal() {
                this.$refs['auth-modal'].hide();
            }
        },
    }
</script>

<style>
  @import "../fonts/Alef/stylesheet.css";
</style>