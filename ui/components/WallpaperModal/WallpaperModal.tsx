import React, { FC, useEffect, useRef, useState } from 'react';
import { Modal } from '../';
import { Box } from '@mui/material';
import { useParams } from 'react-router-dom';
import { useWallpapers } from '../../stores';
import { Wallpaper } from '../../../lib/ts';
import { axiosClient } from '../../clients';
import { formatRelative } from 'date-fns';
import { Blurhash } from 'react-blurhash';
import { motion } from 'framer-motion';

type Params = {
  sk?: string;
};

type Props = {
  onClose: (...props: any) => any;
};

const sx = {
  // image: {
  //   height: '20rem',
  //   width: '50rem',

  //   '& image': {
  //     display: 'block',
  //     height: '20rem',
  //     width: '50rem',
  //     objectFit: 'contain',
  //   },
  // },
  imageWrapper: {
    position: 'relative',
    height: 200,

    '& .motion-div': {
      position: 'absolute',
      top: 0,
      right: 0,
      bottom: 0,
      left: 0,
      width: '100%',

      '& img': {
        display: 'block',
        height: 200,
        width: '100%',
        objectFit: 'cover',
      },
    },
  },
} as const;

export const WallpaperModal: FC<Props> = ({ onClose }) => {
  const { wallpapers } = useWallpapers(({ wallpapers }) => ({ wallpapers }));
  const { sk } = useParams<Params>();
  const hasCalled = useRef<boolean>(false);
  const [imageLoaded, setImageLoaded] = useState<boolean>(false);
  const [wallpaper, setWallpaper] = useState<Wallpaper | null>(
    wallpapers.find((wallpaper) => wallpaper.sk === sk) ?? null // TS is being weird here
  );

  useEffect(() => {
    if (!wallpaper && !hasCalled.current) {
      axiosClient({
        method: 'GET',
        url: `/wallpapers/${sk}`,
      }).then(({ data }) => {
        setWallpaper(data.wallpaper);
        hasCalled.current = true;
      });
    }
  }, [hasCalled, sk, wallpaper]);

  console.log(sk);
  console.log(wallpaper);

  return (
    <Modal
      title={sk ?? 'Wallpaper'}
      subtitle={wallpaper?.created_at?.toString ? formatRelative(new Date(), wallpaper.created_at) : ` `}
      onClose={onClose}
      open={true}
    >
      <Box sx={sx.imageWrapper}>
        <Blurhash
          hash={wallpaper?.blurhash ?? ''}
          width={500}
          height={200}
          resolutionX={32}
          resolutionY={32}
          punch={1}
        />
        {wallpaper?.url && wallpaper?.name && (
          <motion.div
            animate={imageLoaded ? 'loaded' : 'loading'}
            className="motion-div"
            variants={{ loaded: { opacity: 1 }, loading: { opacity: 0 } }}
          >
            {/* <Box sx={sx.image}> */}
            <img src={wallpaper.url} alt={wallpaper.name} onLoad={() => setImageLoaded(true)} />
            {/* </Box> */}
          </motion.div>
        )}
      </Box>
    </Modal>
  );
};
