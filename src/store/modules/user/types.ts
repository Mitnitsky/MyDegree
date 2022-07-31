import {
  UserActionsTypes,
  UserGettersTypes,
  UserMutationsTypes,
  UserState,
} from "@/store/interfaces";
import { CommitOptions, DispatchOptions, Store as VuexStore } from "vuex";

export type UserStoreModuleTypes<S = UserState> = Omit<
  VuexStore<S>,
  "commit" | "getters" | "dispatch"
> & {
  commit<
    K extends keyof UserMutationsTypes,
    P extends Parameters<UserMutationsTypes[K]>[1]
  >(
    key: K,
    payload?: P,
    options?: CommitOptions
  ): ReturnType<UserMutationsTypes[K]>;
} & {
  getters: {
    [K in keyof UserGettersTypes]: ReturnType<UserGettersTypes[K]>;
  };
} & {
  dispatch<K extends keyof UserActionsTypes>(
    key: K,
    payload?: Parameters<UserActionsTypes[K]>[1],
    options?: DispatchOptions
  ): ReturnType<UserActionsTypes[K]>;
};
