<template>
  <el-dialog
    v-model="visibility"
    custom-class="info-dialog"
    style="max-width: 550px !important"
    :show-close="false"
    center
  >
    <el-container class="info-container">
      <el-aside class="border-round-left info-side" width="275px">
        <img
          class="border-round-left"
          width="275"
          src="../assets/logo_transparent.png"
        />
      </el-aside>
      <el-container class="nested-info">
        <el-header class="info-header"> Contact Info</el-header>
        <el-main class="info-main">
          <el-row style="height: 30%" />
          <el-row class="info-row" justify="center">
            <font-awesome-icon
              :icon="['fab', 'github']"
              style="margin-right: 4px"
            />
            <a
              class="info-link"
              href="https://github.com/Mitnitsky/"
              target="_blank"
              >Github</a
            >
          </el-row>
          <el-row style="height: 5%" />
          <el-row class="info-row" justify="center">
            <br />
            <font-awesome-icon
              :icon="['fab', 'linkedin']"
              style="margin-right: 4px"
            />
            <a
              class="info-link"
              href="https://www.linkedin.com/in/vladimir-mitnitsky/"
              target="_blank"
              >LinkedIn</a
            ></el-row
          >
          <el-row style="height: 40%" />
        </el-main>
      </el-container>
    </el-container>
  </el-dialog>
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";

export default defineComponent({
  name: "MyInfoCard",
  props: {
    visible: { type: Boolean, required: true },
    setVisibility: { type: Function, required: true },
  },
  setup(props) {
    const visibility = computed({
      get(): boolean {
        return props.visible;
      },
      set(status: boolean): void {
        props.setVisibility(status);
      },
    });
    return {
      visibility,
    };
  },
});
</script>

<style>
div.info-dialog > div.el-dialog__header {
  width: 0px !important;
  height: 0px !important;
  padding: 0;
  margin: 0;
  border-top-right-radius: 5px;
  border-top-left-radius: 5px;
}

div.info-dialog > div.el-dialog__footer {
  width: 0px !important;
  height: 0px !important;
  padding: 0;
  margin: 0;
  border-top-right-radius: 5px;
  border-top-left-radius: 5px;
}

div.info-dialog > div.el-dialog__body {
  padding: 0;
  margin: 0 !important;
  max-width: 550px !important;
  width: 550px !important;
}

div.info-dialog > div.el-dialog__headerbtn {
  right: var(--el-dialog-padding-primary) !important;
  left: auto !important;
}

.border-round-left {
  border-top-left-radius: 5px;
  border-bottom-left-radius: 5px;
}

div.info-dialog {
  max-width: 550px !important;
  width: 550px !important;
  direction: ltr;
  border-radius: 5px;
}

.info-side {
  background-color: rgb(52, 58, 64);
}

.info-header:first-child {
  border-radius: calc(0.25rem - 1px) calc(0.25rem - 1px) 0 0;
}

.info-header:before {
  content: "";
  display: inline-block;
  height: 100%;
  max-height: 60px;

  vertical-align: middle;
}

.info-header {
  max-width: 275px !important;
  width: 275px !important;
  font-size: 1rem;
  font-weight: 400;
  line-height: 1.5;
  color: #212529;
  text-align: left;
  padding: 0.75rem 1.25rem;
  margin-bottom: 0;
  background-color: rgba(0, 0, 0, 0.03);
  border-bottom: 1px solid rgba(0, 0, 0, 0.125);
  background-color: #f8f9fa !important;
}

.info-link {
  font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Roboto,
    Helvetica Neue, Arial, Noto Sans, Liberation Sans, sans-serif,
    Apple Color Emoji, Segoe UI Emoji, Segoe UI Symbol, Noto Color Emoji;
  color: #007bff;
  font-size: 16px;
  text-decoration: none;
  background-color: transparent;
}

.info-row {
  height: 10%;
  justify-content: center;
  content: "";
  display: inline-block;
  vertical-align: middle;
}

section.nested-info {
  max-width: 275px !important;
  width: 275px !important;
}

section.info-container {
  max-width: 550px !important;
  width: 550px !important;
  margin: 0;
}

.info-main {
  max-width: 275px !important;
  width: 275px !important;
  align-content: center;
  display: table-cell;
  padding: 10px;
}
</style>
