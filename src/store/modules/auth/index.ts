import { Module } from "vuex";
import { AuthState, IRootState } from "@/store/interfaces";
import { getters } from "@/store/modules/auth/getters";
import { actions } from "@/store/modules/auth/actions";
import { mutations } from "@/store/modules/auth/mutations";
import { state } from "@/store/modules/auth/state";

export const auth: Module<AuthState, IRootState> = {
  state,
  getters,
  actions,
  mutations,
};
export default auth;
