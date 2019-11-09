<template>
  <div id="firebaseui-auth-container"></div>
</template>

<script>
    import firebase from "firebase"
    import * as firebaseui from "firebaseui"
    import "../../node_modules/firebaseui/dist/firebaseui.css"

    export default {
        name: "Authentication",
        mounted() {
            let uiConfig = {
                signInFlow: 'popup',
                signInOptions: [
                    firebase.auth.GoogleAuthProvider.PROVIDER_ID,
                    firebase.auth.EmailAuthProvider.PROVIDER_ID
                ],
                callbacks: {
                    signInSuccessWithAuthResult() {
                        localStorage.setItem('authenticated', true);
                    },
                },
                'credentialHelper': firebaseui.auth.CredentialHelper.NONE
            };
            let ui = firebaseui.auth.AuthUI.getInstance();
            if (!ui) {
                ui = new firebaseui.auth.AuthUI(firebase.auth());
            }
            ui.start("#firebaseui-auth-container", uiConfig);
            firebase.auth().onAuthStateChanged((user) => {
                if (user) {
                    window.console.log(user);
                    this.$store.commit('setLoginStatusTrue');
                    this.$root.$emit('bv::hide::modal', 'modal-1');
                    this.$store.commit('setUser', user);
                }
            });
        },
        methods: {},
    }
</script>

<style scoped>

</style>