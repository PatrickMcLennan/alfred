import React, { FC, useState } from 'react';
import { Avatar, Box, Card, CardActions, CardHeader, IconButton } from '@mui/material';
import { Wallpaper } from '../../../lib/ts';
import { Blurhash } from 'react-blurhash';
import { formatRelative } from 'date-fns';
import { motion } from 'framer-motion';
import ZoomInIcon from '@mui/icons-material/ZoomIn';

type Props = {
  focus: (...props: any) => any;
  isActive: boolean;
  wallpaper: Wallpaper;
};

const sx = {
  active: {
    border: '1px solid white',
  },
  cardActions: {
    display: 'flex',
    justifyContent: 'flex-end',
  },
  imageLoaded: {
    opacity: 1,
  },
  imageLoading: {
    opacity: 0,
  },
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

export const WallpaperCard: FC<Props> = ({ isActive, focus, wallpaper }) => {
  const [loaded, setLoaded] = useState<boolean>(false);
  return (
    <Card variant="outlined" sx={{ ...(isActive ? sx.active : {}) }}>
      <CardHeader
        avatar={<Avatar>N</Avatar>}
        subheader={formatRelative(new Date(), wallpaper.created_at)}
        title={wallpaper.name}
      />
      <Box sx={sx.imageWrapper}>
        <Blurhash hash={wallpaper.blurhash} width={500} height={200} resolutionX={32} resolutionY={32} punch={1} />
        <motion.div
          animate={loaded ? 'loaded' : 'loading'}
          className="motion-div"
          variants={{ loaded: { opacity: 1 }, loading: { opacity: 0 } }}
        >
          <img src={wallpaper.thumbnail_url} alt={wallpaper.name} onLoad={() => setLoaded(true)} />
        </motion.div>
      </Box>
      <CardActions sx={sx.cardActions}>
        <IconButton onClick={() => focus(wallpaper.sk)}>
          <ZoomInIcon />
        </IconButton>
      </CardActions>
    </Card>
  );
};
