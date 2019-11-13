<template>
  <div id="app">
    <header-nav-bar></header-nav-bar>
    <div class="container-fluid">
      <semesters-tab-view style="margin: 5px"></semesters-tab-view>

      <degree-summary/>
      <dp-footer></dp-footer>
    </div>
  </div>
</template>

<script>
    import SemestersTabView from "@/components/SemestersTabView";
    import HeaderNavBar from "@/components/Header";
    import DpFooter from "@/components/Footer";
    import DegreeSummary from "@/components/DegreeSummary";
    import firebase from 'firebase'

    export default {
        name: 'app',
        mounted(){

            let saved_token = localStorage.getItem("user_token");
            if(saved_token){
                firebase.auth();
            }

        },
        data() {
            return {}
        },
        components: {
            DegreeSummary,
            DpFooter,
            HeaderNavBar,
            SemestersTabView,
        }
    }
</script>

<style>
</style>
