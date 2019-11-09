<template>
  <b-navbar toggleable="lg"
            type="dark"
            variant="dark">
    <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>
    <b-collapse id="nav-collapse"
                is-nav>
      <b-navbar-nav class="ml-auto">
        <section v-if="logged">
          <div class="row"
               style="margin-top: 5px;margin-bottom: 5px;">
            <p style="margin: 0;color: lightgray;margin-top: 8px"> שלום {{this.$store.getters.getUserName}} !</p>
            <b-nav-item @click="signOut"
                        href="#"
                        right
                        style="margin-top: 0;margin-right: 5px;font-style: italic;text-decoration: underline">
              יציאה
            </b-nav-item>
          </div>
        </section>
        <section v-else>
          <b-nav-item>
            <em DIR="ltr"
                v-b-modal.modal-1>כניסה</em>
            <b-modal cancel-disabled
                     id="modal-1"
                     ok-title="סגור"
                     title="כניסה">
              <authentication></authentication>
            </b-modal>
          </b-nav-item>
        </section>
      </b-navbar-nav>
    </b-collapse>
    <b-navbar-brand href="#">Degree Planer</b-navbar-brand>
    <img alt=""
         src="../assets/main_icon_white.svg"
         style="width: 2%; height: 2%;margin-right: 5px;"/>
  </b-navbar>
</template>

<script>
    import firebase from "firebase"
    import Authentication from "./HeaderAuthentication";

    export default {
        components: {Authentication},
        name: "HeaderNavBar",
        computed: {
            logged() {
                return this.$store.getters.getLoginStatus;
            }
        },
        data() {
            return {}
        },
        methods: {
            signOut() {
                firebase.auth().signOut();
                this.$store.commit('setLoginStatusFalse');
                this.$store.commit('clearUserData');
            }
        },
    }
</script>
