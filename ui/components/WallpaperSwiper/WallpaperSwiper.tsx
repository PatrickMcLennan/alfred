import React, { FC, useEffect, useState } from 'react';
import { Pagination, A11y } from 'swiper';
import { Swiper, SwiperSlide } from 'swiper/react';
import { Box, Theme, Typography } from '@mui/material';
import { axiosClient } from '../../clients';
import { WallpaperCard } from '../WallpaperCard';
import { useTheme } from '@mui/material/styles';
import useMediaQuery from '@mui/material/useMediaQuery';
import { useWallpapers } from '../../stores/wallpapers.store';

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
  h2: {
    marginBottom: (theme: Theme) => theme.spacing(2),
    paddingLeft: (theme: Theme) => theme.spacing(4),
  },
} as const;

const getSlidesPerView = (isMobile: boolean, isTablet: boolean) => {
  if (isMobile && isTablet) return 1.5;
  if (isTablet) return 3.5;
  return 4.5;
};

export const WallpaperSwiper: FC<Props> = ({ focus }) => {
  // const firstRender = useRef<boolean>(true);
  const { total, wallpapers, updateWallpapers } = useWallpapers(({ total, wallpapers, updateWallpapers }) => ({
    total,
    wallpapers,
    updateWallpapers,
  }));
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down('sm'));
  const isTablet = useMediaQuery(theme.breakpoints.down('lg'));
  const [slidesPerView, setSlidesPerView] = useState<number>(getSlidesPerView(isMobile, isTablet));

  const lazyLoad = async ({ realIndex }: { realIndex: number }) => {
    if (realIndex < total - 5) {
      return;
    }
    const newWallpapers = await axiosClient({
      method: 'POST',
      url: '/wallpapers/',
      data: { limit: 10, start_key: wallpapers[wallpapers.length - 1]?.sk ?? `` },
    });
    return updateWallpapers(newWallpapers.data);
  };

  useEffect(() => {
    setSlidesPerView(getSlidesPerView(isMobile, isTablet));
  }, [isMobile, isTablet]);

  useEffect(() => {
    if (wallpapers.length <= 10) {
      axiosClient({
        method: 'POST',
        url: '/wallpapers/',
        data: { limit: 10, start_key: wallpapers[wallpapers.length - 1]?.sk ?? `` },
      }).then(({ data }) => updateWallpapers(data));
    }
    // I only want this running after first mount, not watching dependencies
    // eslint-disable-next-line
  }, []);

  return (
    <Box aria-label="Wallpapers" component="section" sx={sx.container}>
      <Typography component="h2" sx={sx.h2} variant="h4">
        Wallpapers
      </Typography>
      <Swiper
        modules={[A11y, Pagination]}
        onSlideChange={lazyLoad}
        pagination={{
          clickable: true,
          dynamicBullets: true,
          type: `bullets`,
        }}
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
