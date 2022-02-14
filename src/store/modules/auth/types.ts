import {
  AuthState,
  AuthMutationsTypes,
  AuthGettersTypes,
  AuthActionsTypes,
} from "@/store/interfaces";
import { Store as VuexStore, CommitOptions, DispatchOptions } from "vuex";

export type AuthStoreModuleTypes<S = AuthState> = Omit<
  VuexStore<S>,
  "commit" | "getters" | "dispatch"
> & {
  commit<
    K extends keyof AuthMutationsTypes,
    P extends Parameters<AuthMutationsTypes[K]>[1]
  >(
    key: K,
    payload?: P,
    options?: CommitOptions
  ): ReturnType<AuthMutationsTypes[K]>;
} & {
  getters: {
    [K in keyof AuthGettersTypes]: ReturnType<AuthGettersTypes[K]>;
  };
} & {
  dispatch<K extends keyof AuthActionsTypes>(
    key: K,
    payload?: Parameters<AuthActionsTypes[K]>[1],
    options?: DispatchOptions
  ): ReturnType<AuthActionsTypes[K]>;
};
