import create from 'zustand';
import { SearchWallpapersResponse, Wallpaper } from '../../lib/ts';

type WallpaperStore = {
  message: string;
  wallpapers: Wallpaper[];
  total: number;
  lastFetched: Date;

  updateWallpapers: (newResponse: SearchWallpapersResponse) => void;
};

export const useWallpapers = create<WallpaperStore>((set) => ({
  message: ``,
  total: 0,
  wallpapers: [],
  lastFetched: new Date(),

  updateWallpapers: (newResponse) =>
    set((state) => ({
      ...state,
      lastFetched: new Date(),
      wallpapers: [...state.wallpapers, ...newResponse.images],
      total: state.total + newResponse.total,
    })),
}));
