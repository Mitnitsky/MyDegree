<template>
  <div id="app">
    <header-nav-bar :logged="logged"></header-nav-bar>
    <div class="container-fluid" >
      <semesters-tab-view></semesters-tab-view>
      <degree-summary/>
    </div>
    <dp-footer class="footer navbar-fixed-bottom"></dp-footer>
  </div>
</template>

<script>
  import SemestersTabView from "@/components/SemestersTabView";
  import HeaderNavBar from "@/components/HeaderNavBar";
  import DpFooter from "@/components/DpFooter";
  import DegreeSummary from "@/components/DegreeSummary";

  export default {
    name: 'app',
    data() {
      return {
        logged: false
      }
    },
    components: {
      DegreeSummary,
      DpFooter,
      HeaderNavBar,
      SemestersTabView
    }
  }
</script>

