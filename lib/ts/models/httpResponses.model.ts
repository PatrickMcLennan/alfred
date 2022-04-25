import { Wallpaper } from './wallpaper.model';

export type SearchWallpapersResponse = {
  images: Wallpaper[];
  total: number;
  message: string;
};
