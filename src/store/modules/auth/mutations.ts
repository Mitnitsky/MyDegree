import { MutationTree } from "vuex";
import { AuthState, AuthMutationsTypes } from "@/store/interfaces";
import { AUTH_STORE } from "@/store/constants";

export const mutations: MutationTree<AuthState> & AuthMutationsTypes = {
  [AUTH_STORE.MUTATIONS.setUserName](state, username: string): void {
    state.username = username;
  },
  [AUTH_STORE.MUTATIONS.setLoggedStatus](state, status: boolean): void {
    state.logged = status;
  },
};
