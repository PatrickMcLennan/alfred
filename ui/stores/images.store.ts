import create from 'zustand';
import { SearchWallpapersResponse, Image } from '../../lib/ts';

type ImageStore = {
  message: string;
  widescreen_wallpapers: Image[];
  amoled_backgrounds: Image[];
  lastFetched: Date;

  removeWallpaper: (sk: string) => void;
  updateAmoledBackgrounds: (newResponse: SearchWallpapersResponse) => void;
  updateWidescreenWallpapers: (newResponse: SearchWallpapersResponse) => void;
};

export const useImages = create<ImageStore>((set) => ({
  message: ``,
  widescreen_wallpapers: [],
  amoled_backgrounds: [],
  lastFetched: new Date(),

  removeWallpaper: (sk) =>
    set((state) => ({
      ...state,
      widescreen_wallpapers: state.widescreen_wallpapers.filter((wallpaper) => wallpaper.sk !== sk),
    })),

  updateAmoledBackgrounds: (newResponse) =>
    set((state) => ({
      lastFetched: new Date(),
      amoled_backgrounds: [...state.amoled_backgrounds, ...newResponse.images],
    })),

  updateWidescreenWallpapers: (newResponse) =>
    set((state) => ({
      ...state,
      lastFetched: new Date(),
      widescreen_wallpapers: [...state.widescreen_wallpapers, ...newResponse.images],
    })),
}));
