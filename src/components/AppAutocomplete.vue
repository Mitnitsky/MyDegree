<template>
  <div class="app-autocomplete" style="position: relative">
    <input
      ref="input"
      v-model="query"
      :placeholder="placeholder"
      :aria-label="ariaLabel"
      class="app-autocomplete-input form-control"
      type="text"
      autocomplete="off"
      @input="onInput"
      @keydown.enter.prevent="onEnter"
      @keydown.up.prevent="onArrowUp"
      @keydown.down.prevent="onArrowDown"
      @keydown.escape="close"
      @focus="showResults = results.length > 0"
      @blur="onBlur"
    />
    <ul
      v-if="showResults && results.length > 0"
      class="app-autocomplete-results list-group"
      style="
        position: absolute;
        z-index: 1000;
        width: 100%;
        max-height: 300px;
        overflow-y: auto;
        border: 1px solid #dee2e6;
        background: white;
      "
    >
      <li
        v-for="(result, i) in results"
        :key="i"
        class="list-group-item list-group-item-action"
        :class="{ active: i === selectedIndex }"
        style="cursor: pointer; padding: 6px 12px"
        @mousedown.prevent="selectResult(result)"
        @mouseenter="selectedIndex = i"
      >
        {{ getResultValue(result) }}
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  name: "AppAutocomplete",
  props: {
    search: { type: Function, required: true },
    getResultValue: { type: Function, required: true },
    placeholder: { type: String, default: "" },
    ariaLabel: { type: String, default: "" },
    autoSelect: { type: Boolean, default: false },
  },
  emits: ["submit"],
  data() {
    return {
      query: "",
      results: [],
      selectedIndex: -1,
      showResults: false,
    };
  },
  methods: {
    async onInput() {
      const results = await this.search(this.query);
      this.results = results || [];
      this.selectedIndex = this.autoSelect && this.results.length > 0 ? 0 : -1;
      this.showResults = this.results.length > 0;
    },
    onEnter() {
      if (this.selectedIndex >= 0 && this.selectedIndex < this.results.length) {
        this.selectResult(this.results[this.selectedIndex]);
      }
    },
    onArrowUp() {
      if (this.selectedIndex > 0) {
        this.selectedIndex--;
      }
    },
    onArrowDown() {
      if (this.selectedIndex < this.results.length - 1) {
        this.selectedIndex++;
      }
    },
    selectResult(result) {
      this.query = this.getResultValue(result);
      this.showResults = false;
      this.$emit("submit", result);
    },
    close() {
      this.showResults = false;
    },
    onBlur() {
      setTimeout(() => {
        this.showResults = false;
      }, 200);
    },
  },
};
</script>
