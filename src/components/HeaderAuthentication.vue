<template>
  <div id="firebaseui-auth-container" />
</template>

<script lang="ts">
import firebase from 'firebase/compat/app'
import 'firebase/compat/auth'
import 'firebase/compat/firestore'
import * as firebaseui from 'firebaseui'
import '../../node_modules/firebaseui/dist/firebaseui.css'
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'HeaderAuthentication',
  props: {
    closeAuthModal: {
      type: Function,
      required: true,
    },
  },
  setup(props) {
    return {
      hide_auth_modal: props.closeAuthModal,
    }
  },
  mounted() {
    const hide_auth_modal = this.hide_auth_modal
    const uiConfig = {
      signInFlow: 'popup',
      signInOptions: [
        firebase.auth.GoogleAuthProvider.PROVIDER_ID,
        firebase.auth.EmailAuthProvider.PROVIDER_ID,
      ],
      callbacks: {
        // eslint-disable-next-line no-unused-vars,@typescript-eslint/no-unused-vars
        signInSuccessWithAuthResult: function (authResult) {
          hide_auth_modal()
          return false
        },
      },
      credentialHelper: firebaseui.auth.CredentialHelper.NONE,
    }
    let ui = firebaseui.auth.AuthUI.getInstance()
    if (!ui) {
      ui = new firebaseui.auth.AuthUI(firebase.auth())
    }
    ui.start('#firebaseui-auth-container', uiConfig)
  },
  methods: {},
})
</script>

<style scoped></style>
