import { GetterTree } from "vuex";
import { AuthState, IRootState } from "@/store/interfaces";
import { AuthGettersTypes } from "@/store/interfaces";
import { AUTH_STORE } from "@/store/constants";

export const getters: GetterTree<AuthState, IRootState> & AuthGettersTypes = {
  [AUTH_STORE.GETTERS.USERNAME]: (state: AuthState) => {
    return state.username;
  },
  [AUTH_STORE.GETTERS.LOGGED]: (state: AuthState) => {
    return state.logged;
  },
};
