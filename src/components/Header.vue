<template>
  <b-navbar toggleable="lg"
            type="dark"
            variant="dark">
    <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>
    <b-collapse id="nav-collapse"
                is-nav>
      <b-navbar-nav class="ml-auto">
        <section v-if="this.logged">
          <div class="row"
               style="margin-top: 5px;margin-bottom: 5px;">
            <p style="font-size: 20px;margin: 0;color: lightgray;margin-top: 8px"> שלום {{this.user_name}} !</p>
            <b-nav-item @click="signOut"
                        href="#"
                        right
                        style="font-size: 20px;margin-top: 0;margin-right: 5px;font-style: italic;text-decoration: underline">
              יציאה
            </b-nav-item>
          </div>
        </section>
        <section v-else>
          <b-nav-item>
            <em DIR="ltr"
                style="font-size: 20px;text-decoration: underline;"
                v-b-modal.modal-1>כניסה</em>
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
          </b-nav-item>
        </section>
      </b-navbar-nav>
    </b-collapse>
    <b-navbar-brand href="#">Degree Planer</b-navbar-brand>
    <img alt=""
         src="../assets/main_icon_white.svg"
         style="width: 48px; height: 48px;margin-right: 5px;"/>
  </b-navbar>
</template>

<script>
    import firebase from "firebase"
    import Authentication from "./HeaderAuthentication";
    import {mapFields} from 'vuex-map-fields';

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
                    this.$refs['auth-modal'].hide();
                    firebase.firestore().collection('users').doc(user.uid).get().then((doc) => {
                        if (doc.exists) {
                            this.$store.commit('fetchUserInfo', doc.data());
                            this.$store.commit('reCalcCurrentSemester');
                        } else {
                            firebase.firestore().collection('users').doc(user.uid).set(this.$store.state.user);
                        }
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
