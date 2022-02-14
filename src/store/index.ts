import { createStore } from "vuex";
import { IRootState } from "@/store/interfaces";
import { AuthStoreModuleTypes } from "./modules/auth/types";
import { UserStoreModuleTypes } from "./modules/user/types";
import { Module, ModuleTree } from "vuex";
import authModule from "./modules/auth";
import userModule from "./modules/user";

const modules: ModuleTree<IRootState> = {
  authModule,
  userModule,
};

const root: Module<IRootState, IRootState> = {
  modules,
};

export const store = createStore<IRootState>(root);

type StoreModules = {
  auth: AuthStoreModuleTypes;
  user: UserStoreModuleTypes;
};

export type Store = AuthStoreModuleTypes<Pick<StoreModules, "auth">> &
  UserStoreModuleTypes<Pick<StoreModules, "user">>;
