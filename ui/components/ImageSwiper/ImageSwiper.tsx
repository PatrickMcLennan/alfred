import React, { FC, useCallback, useEffect, useRef, useState } from 'react';
import { Controller, A11y, Swiper as SwiperClass } from 'swiper';
import { Swiper, SwiperSlide } from 'swiper/react';
import { Box, IconButton, Theme, Typography } from '@mui/material';
import { axiosClient } from '../../clients';
import { ImageCard } from '../ImageCard';
import { useTheme } from '@mui/material/styles';
import useMediaQuery from '@mui/material/useMediaQuery';
import { useImages } from '../../stores/images.store';

import NavigateBeforeIcon from '@mui/icons-material/NavigateBefore';
import NavigateNextIcon from '@mui/icons-material/NavigateNext';
import { ImageType } from '../../../lib/ts';
import { Method } from 'axios';

type Props = {
  focus: (...props: any) => any;
  title: string;
  variant: ImageType;
};

const sx = {
  container: {
    '& .swiper-pagination': {
      backgroundColor: (theme: Theme) => theme.palette.background.default,
      padding: (theme: Theme) => theme.spacing(1),
      bottom: '0rem',
    },
    '& .swiper-pagination-bullet': {
      backgroundColor: 'white',
    },
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: `center`,
    padding: (theme: Theme) => theme.spacing(0, 4),
    marginBottom: (theme: Theme) => theme.spacing(2),
  },
  iconButton: {
    border: '1px solid white',

    '&:not(:last-of-type)': {
      marginRight: (theme: Theme) => theme.spacing(2),
    },

    '&[disabled]': {
      border: `1px solid rgba(255,255,255,0.3)`,
    },
  },
  navigationControls: {
    marginRight: (theme: Theme) => theme.spacing(4),
  },
} as const;

const getSlidesPerView = (isMobile: boolean, isTablet: boolean, isDesktop: boolean) => {
  if (isMobile && isTablet) return 1.5;
  if (isTablet) return 2.5;
  if (isDesktop) return 3.5;
  return 4.5;
};

export const ImageSwiper: FC<Props> = ({ focus, title, variant }) => {
  const swiperRef = useRef<SwiperClass>(new SwiperClass(''));
  const isCalling = useRef<boolean>(false); // Debounce new api calls when someone is scrolling quickly
  const { amoled_backgrounds, widescreen_wallpapers, updateAmoledBackgrounds, updateWidescreenWallpapers } = useImages(
    ({ amoled_backgrounds, widescreen_wallpapers, updateAmoledBackgrounds, updateWidescreenWallpapers }) => ({
      amoled_backgrounds,
      widescreen_wallpapers,
      updateAmoledBackgrounds,
      updateWidescreenWallpapers,
    })
  );
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down('sm'));
  const isTablet = useMediaQuery(theme.breakpoints.down('md'));
  const isDesktop = useMediaQuery(theme.breakpoints.down('lg'));
  const [slidesPerView, setSlidesPerView] = useState<number>(getSlidesPerView(isMobile, isTablet, isDesktop));
  const [disabledPrev, setDisabledPrev] = useState<boolean>(true);
  const [disabledNext, setDisabledNext] = useState<boolean>(false);

  const apiConfig = useCallback(() => {
    switch (variant) {
      case ImageType.AMOLED_BACKGROUND:
        return {
          method: 'POST' as Method,
          url: '/images',
          data: {
            limit: 15,
            start_key: amoled_backgrounds[amoled_backgrounds.length - 1]?.sk ?? ``,
            pk: `image|amoled_background`,
          },
        };
      case ImageType.WIDESCREEN_WALLPAPER:
      default:
        return {
          method: 'POST' as Method,
          url: '/images',
          data: {
            limit: 15,
            start_key: widescreen_wallpapers[widescreen_wallpapers.length - 1]?.sk ?? ``,
            pk: `image|widescreen_wallpaper`,
          },
        };
    }
  }, [amoled_backgrounds, variant, widescreen_wallpapers]);

  const goBack = () => swiperRef.current.slidePrev();
  const goForward = () => swiperRef.current.slideNext();

  const swipeWatcher = async ({ realIndex }: { realIndex: number }) => {
    const images = variant === ImageType.AMOLED_BACKGROUND ? amoled_backgrounds : widescreen_wallpapers;
    if (realIndex !== 0 && disabledPrev) {
      setDisabledPrev(false);
    }
    if (realIndex === 0) {
      setDisabledPrev(true);
    }
    if (realIndex !== images.length - 1 && disabledNext) {
      setDisabledNext(false);
    }
    if (realIndex === images.length - 1) {
      setDisabledNext(true);
    }
    if (realIndex < images.length - 5 || isCalling.current) {
      return;
    } else {
      isCalling.current = true;
      const newImages = await axiosClient(apiConfig());
      isCalling.current = false;
      return variant === ImageType.WIDESCREEN_WALLPAPER
        ? updateWidescreenWallpapers(newImages.data)
        : updateAmoledBackgrounds(newImages.data);
    }
  };

  useEffect(() => {
    setSlidesPerView(getSlidesPerView(isMobile, isTablet, isDesktop));
  }, [isDesktop, isMobile, isTablet]);

  useEffect(() => {
    if (
      ((variant === ImageType.WIDESCREEN_WALLPAPER && widescreen_wallpapers.length <= 15) ||
        (variant === ImageType.AMOLED_BACKGROUND && amoled_backgrounds.length <= 15)) &&
      !isCalling.current
    ) {
      isCalling.current = true;
      axiosClient(apiConfig()).then(({ data }) => {
        isCalling.current = false;
        variant === ImageType.WIDESCREEN_WALLPAPER ? updateWidescreenWallpapers(data) : updateAmoledBackgrounds(data);
      });
    }
    // I only want this running after first mount, not watching dependencies
    // eslint-disable-next-line
  }, []);

  return (
    <Box aria-label="Wallpapers" component="section" sx={sx.container}>
      <Box component="header" sx={sx.header}>
        <Typography component="h2" variant="h5">
          {title}
        </Typography>
        <Box sx={sx.navigationControls}>
          <IconButton disabled={disabledPrev} onClick={goBack} sx={sx.iconButton}>
            <NavigateBeforeIcon />
          </IconButton>
          <IconButton disabled={disabledNext} onClick={goForward} sx={sx.iconButton}>
            <NavigateNextIcon />
          </IconButton>
        </Box>
      </Box>
      <Swiper
        controller={{ control: swiperRef.current }}
        modules={[A11y, Controller]}
        onSlideChange={swipeWatcher}
        onSwiper={(swiper) => (swiperRef.current = swiper)}
        spaceBetween={10}
        slidesPerView={slidesPerView}
      >
        {variant === ImageType.AMOLED_BACKGROUND &&
          amoled_backgrounds.map((wallpaper) => (
            <SwiperSlide key={wallpaper.sk}>
              {({ isActive }) => <ImageCard isActive={isActive} focus={focus} image={wallpaper} variant={variant} />}
            </SwiperSlide>
          ))}
        {variant === ImageType.WIDESCREEN_WALLPAPER &&
          widescreen_wallpapers.map((wallpaper) => (
            <SwiperSlide key={wallpaper.sk}>
              {({ isActive }) => <ImageCard isActive={isActive} focus={focus} image={wallpaper} variant={variant} />}
            </SwiperSlide>
          ))}
      </Swiper>
    </Box>
  );
};
