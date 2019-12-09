<template>
  <div id="app" style="font-family: Alef, Roboto, Helvetica, Arial, sans-serif;min-width: 810px !important;">
    <header-nav-bar/>
    <div class="container-fluid">
      <semesters-tab-view style="margin: 5px"/>
      <degree-summary/>
      <dp-footer/>
    </div>
  </div>
</template>

<script>
    import SemestersTabView from "./components/SemestersTabView";
    import HeaderNavBar from "./components/Header";
    import DpFooter from "./components/Footer";
    import DegreeSummary from "./components/DegreeSummary";

    export default {
        name: 'app',
        components: {
            DegreeSummary,
            DpFooter,
            HeaderNavBar,
            SemestersTabView,
        }
    }
</script>

<style>
  @import "./fonts/Alef/stylesheet.css";
</style>
