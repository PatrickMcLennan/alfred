import { Image } from './image.model';

export type SearchWallpapersResponse = {
  images: Image[];
  total: number;
  message: string;
};
