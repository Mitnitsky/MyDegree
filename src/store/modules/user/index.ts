import { Module } from "vuex";
import { IRootState, UserState } from "@/store/interfaces";
import { getters } from "@/store/modules/user/getters";
import { actions } from "@/store/modules/user/actions";
import { mutations } from "@/store/modules/user/mutations";
import { state } from "@/store/modules/user/state";

export const user: Module<UserState, IRootState> = {
  state,
  actions,
  getters,
  mutations,
};
export default user;
