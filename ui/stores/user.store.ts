import create from 'zustand';

type UserStore = {
  isLoggedIn: boolean;
  userAuthEvent: (newLoggedInStatus: boolean) => void;
};

export const useUser = create<UserStore>((set) => ({
  isLoggedIn: false,
  userAuthEvent: (newLoggedInStatus) =>
    set((state) => ({
      ...state,
      isLoggedIn: newLoggedInStatus,
    })),
}));
