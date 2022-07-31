import {
  AuthActions,
  AuthGetters,
  AuthMutations,
  UserActions,
  UserGetters,
  UserMutations,
} from "./enums";

export const USER_STORE = {
  GETTERS: UserGetters,
  MUTATIONS: UserMutations,
  ACTIONS: UserActions,
};

export const AUTH_STORE = {
  GETTERS: AuthGetters,
  MUTATIONS: AuthMutations,
  ACTIONS: AuthActions,
};
