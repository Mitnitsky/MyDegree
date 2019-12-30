<template>
  <div id="app"
       style="font-family: Alef, Roboto, Helvetica, Arial, sans-serif;min-width: 838px !important;">
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

  input[type=number]::-webkit-inner-spin-button,
  input[type=number]::-webkit-outer-spin-button {
    -webkit-appearance: none !important;
    margin: 0;
  }


  /*//Firefox fix of input fields */
  input[type=number] {
    -moz-appearance: textfield; /* Firefox arrows on numeric fields */
  }

  @supports (-moz-appearance:none) {
    /*Firefox spin-box fix*/
    SELECT {
      -moz-appearance: none !important;
      background: transparent url('data:image/gif;base64,R0lGODlhBgAGAKEDAFVVVX9/f9TU1CgmNyH5BAEKAAMALAAAAAAGAAYAAAIODA4hCDKWxlhNvmCnGwUAOw==') left center no-repeat !important;
      background-position: calc(5px) center !important;
    }

  }

  .dropdown-item {
    text-align: right !important;
  }
</style>
