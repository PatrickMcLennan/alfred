import React, { FC, useEffect, useRef, useState } from 'react';
import { Controller, A11y, Swiper as SwiperClass } from 'swiper';
import { Swiper, SwiperSlide } from 'swiper/react';
import { Box, IconButton, Theme, Typography } from '@mui/material';
import { axiosClient } from '../../clients';
import { WallpaperCard } from '../WallpaperCard';
import { useTheme } from '@mui/material/styles';
import useMediaQuery from '@mui/material/useMediaQuery';
import { useWallpapers } from '../../stores/wallpapers.store';

import NavigateBeforeIcon from '@mui/icons-material/NavigateBefore';
import NavigateNextIcon from '@mui/icons-material/NavigateNext';

type Props = {
  focus: (...props: any) => any;
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

const getSlidesPerView = (isMobile: boolean, isTablet: boolean) => {
  if (isMobile && isTablet) return 1.5;
  if (isTablet) return 3.5;
  return 4.5;
};

export const WallpaperSwiper: FC<Props> = ({ focus }) => {
  const swiperRef = useRef<SwiperClass>(new SwiperClass(''));
  const isCalling = useRef<boolean>(false); // Debounce new api calls when someone is scrolling quickly
  const { total, wallpapers, updateWallpapers } = useWallpapers(({ total, wallpapers, updateWallpapers }) => ({
    total,
    wallpapers,
    updateWallpapers,
  }));
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down('sm'));
  const isTablet = useMediaQuery(theme.breakpoints.down('lg'));
  const [slidesPerView, setSlidesPerView] = useState<number>(getSlidesPerView(isMobile, isTablet));
  const [disabledPrev, setDisabledPrev] = useState<boolean>(true);
  const [disabledNext, setDisabledNext] = useState<boolean>(false);

  const goBack = () => swiperRef.current.slidePrev();
  const goForward = () => swiperRef.current.slideNext();

  const swipeWatcher = async ({ realIndex }: { realIndex: number }) => {
    if (realIndex !== 0 && disabledPrev) {
      setDisabledPrev(false);
    }
    if (realIndex === 0) {
      setDisabledPrev(true);
    }
    if (realIndex !== wallpapers.length - 1 && disabledNext) {
      setDisabledNext(false);
    }
    if (realIndex === wallpapers.length - 1) {
      setDisabledNext(true);
    }
    if (realIndex < total - 5 || isCalling.current) {
      return;
    } else {
      isCalling.current = true;
      const newWallpapers = await axiosClient({
        method: 'POST',
        url: '/wallpapers/',
        data: { limit: 15, start_key: wallpapers[wallpapers.length - 1]?.sk ?? `` },
      });
      isCalling.current = false;
      return updateWallpapers(newWallpapers.data);
    }
  };

  useEffect(() => {
    setSlidesPerView(getSlidesPerView(isMobile, isTablet));
  }, [isMobile, isTablet]);

  useEffect(() => {
    if (wallpapers.length <= 15 && !isCalling.current) {
      isCalling.current = true;
      axiosClient({
        method: 'POST',
        url: '/wallpapers/',
        data: { limit: 15, start_key: wallpapers[wallpapers.length - 1]?.sk ?? `` },
      }).then(({ data }) => {
        isCalling.current = false;
        updateWallpapers(data);
      });
    }
    // I only want this running after first mount, not watching dependencies
    // eslint-disable-next-line
  }, []);

  return (
    <Box aria-label="Wallpapers" component="section" sx={sx.container}>
      <Box component="header" sx={sx.header}>
        <Typography component="h2" variant="h4">
          Wallpapers
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
        {wallpapers.map((wallpaper) => (
          <SwiperSlide key={wallpaper.sk}>
            {({ isActive }) => <WallpaperCard isActive={isActive} focus={focus} wallpaper={wallpaper} />}
          </SwiperSlide>
        ))}
      </Swiper>
    </Box>
  );
};
