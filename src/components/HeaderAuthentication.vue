<template>
  <div id="firebaseui-auth-container"></div>
</template>

<script>
    import firebase from "firebase/app"
    import 'firebase/auth'
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
                    // eslint-disable-next-line no-unused-vars
                    signInSuccessWithAuthResult: function (authResult) {
                        return false;
                    }
                },
                'credentialHelper': firebaseui.auth.CredentialHelper.NONE
            };
            let ui = firebaseui.auth.AuthUI.getInstance();
            if (!ui) {
                ui = new firebaseui.auth.AuthUI(firebase.auth());
            }
            ui.start("#firebaseui-auth-container", uiConfig);
        },
        methods: {},
    }
</script>

<style scoped>

</style>