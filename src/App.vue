<template>
  <div
    id="app"
    style="
      font-family: Alef, Roboto, Helvetica, Arial, sans-serif;
      min-width: 965px !important;
    "
  >
    <header-nav-bar />
    <div class="container-fluid">
      <semesters-tab-view style="margin: 5px" />
      <degree-summary />
      <dp-footer />
    </div>
  </div>
</template>

<script>
import SemestersTabView from "./components/SemestersTabView";
import HeaderNavBar from "./components/Header";
import DpFooter from "./components/Footer";
import DegreeSummary from "./components/DegreeSummary";

export default {
  name: "App",
  components: {
    DegreeSummary,
    DpFooter,
    HeaderNavBar,
    SemestersTabView,
  },
};
</script>

<style>
@import "./fonts/Alef/stylesheet.css";

input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none !important;
  margin: 0;
}

/*//Firefox fix of input fields */
input[type="number"] {
  -moz-appearance: textfield; /* Firefox arrows on numeric fields */
}

@supports (-moz-appearance: none) {
  /*Firefox spin-box fix*/
  select {
    -moz-appearance: none !important;
    background: transparent
      url("data:image/gif;base64,R0lGODlhBgAGAKEDAFVVVX9/f9TU1CgmNyH5BAEKAAMALAAAAAAGAAYAAAIODA4hCDKWxlhNvmCnGwUAOw==")
      left center no-repeat !important;
    background-position: calc(5px) center !important;
  }
}

.dropdown-item {
  text-align: right !important;
}

/* Bootstrap 4 → 5 compatibility: physical margin/padding utilities removed in BS5 */
.mr-auto {
  margin-right: auto !important;
}
.ml-auto {
  margin-left: auto !important;
}
.mr-0 {
  margin-right: 0 !important;
}
.ml-0 {
  margin-left: 0 !important;
}
.mr-1 {
  margin-right: 0.25rem !important;
}
.ml-1 {
  margin-left: 0.25rem !important;
}
.mr-2 {
  margin-right: 0.5rem !important;
}
.ml-2 {
  margin-left: 0.5rem !important;
}
.mr-3 {
  margin-right: 1rem !important;
}
.ml-3 {
  margin-left: 1rem !important;
}
.mr-4 {
  margin-right: 1.5rem !important;
}
.ml-4 {
  margin-left: 1.5rem !important;
}
.mr-5 {
  margin-right: 3rem !important;
}
.ml-5 {
  margin-left: 3rem !important;
}
.pr-0 {
  padding-right: 0 !important;
}
.pl-0 {
  padding-left: 0 !important;
}
.pr-1 {
  padding-right: 0.25rem !important;
}
.pl-1 {
  padding-left: 0.25rem !important;
}
.pr-2 {
  padding-right: 0.5rem !important;
}
.pl-2 {
  padding-left: 0.5rem !important;
}
.pr-3 {
  padding-right: 1rem !important;
}
.pl-3 {
  padding-left: 1rem !important;
}

/* BS4 close button compat */
.close {
  font-size: 1.5rem;
  font-weight: 700;
  line-height: 1;
  opacity: 0.5;
  background: transparent;
  border: 0;
  padding: 0;
}
.close:hover {
  opacity: 0.75;
}

/* BS4 input-group-prepend/append compat */
.input-group-prepend,
.input-group-append {
  display: flex;
}
.input-group-addon {
  display: flex;
  align-items: center;
  padding: 0.375rem 0.75rem;
  font-weight: 400;
  text-align: center;
  white-space: nowrap;
  border: 1px solid #ced4da;
  border-radius: 0.25rem;
}
</style>
