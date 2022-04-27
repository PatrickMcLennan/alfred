import React, { FC, useState } from 'react';
import { Avatar, Box, Card, CardActionArea, CardHeader, SxProps } from '@mui/material';
import { Image, ImageType } from '../../../lib/ts';
import { Blurhash } from 'react-blurhash';
import { formatRelative } from 'date-fns';
import { motion } from 'framer-motion';

type Props = {
  focus: (...props: any) => any;
  isActive: boolean;
  image: Image;
  variant: ImageType;
};

const sx = {
  active: {
    border: '1px solid white',
  },
  card: {
    cursor: 'grab',

    '&:active': {
      cursor: 'grabbing',
    },
  },
  cardActions: {
    display: 'flex',
    justifyContent: 'flex-end',
    paddingTop: 0,
  },
  cardHeader: {
    position: `relative`,
    // paddingBottom: 0,
    paddingTop: [`2.8rem`, `2.8rem`, `1.6rem`, `1.6rem`, `1.6rem`],
  },
  cardHeaderAvatar: {
    position: [`absolute`, `absolute`, `static`, `static`, `static`],
    top: [`-2rem`, `-2rem`, 0, 0, 0],
    marginRight: [0, 0, `1.6rem`, `1.6rem`, `1.6rem`],
  },
  imageLoaded: {
    opacity: 1,
  },
  imageLoading: {
    opacity: 0,
  },
  imageWrapper: {
    position: 'relative',
    height: [150, 150, 150, 150, 150],

    '& .motion-div': {
      position: 'absolute',
      top: 0,
      right: 0,
      bottom: 0,
      left: 0,
      width: '100%',

      '& img': {
        display: 'block',
        height: '100%',
        width: '100%',
        objectFit: 'cover',
      },
    },
  },
  backgroundHeight: {
    height: [350, 350, 350, 350, 350],
  },
  wallpaperHeight: {
    height: [150, 150, 150, 150, 150],
  },
} as const;

export const ImageCard: FC<Props> = ({ isActive, focus, image, variant }) => {
  const [activeStyles, setActiveStyles] = useState<boolean>(isActive);
  const [loaded, setLoaded] = useState<boolean>(false);

  const revertStyles = () => setActiveStyles(isActive);
  return (
    <Card
      variant="outlined"
      onBlur={revertStyles}
      onFocus={() => setActiveStyles(true)}
      onMouseOver={() => setActiveStyles(true)}
      onMouseLeave={revertStyles}
      sx={{ ...sx.card, ...(activeStyles ? sx.active : {}) }}
    >
      <CardActionArea onClick={() => focus(image.sk)}>
        <Box
          sx={
            {
              ...sx.imageWrapper,
              ...(variant === ImageType.WIDESCREEN_WALLPAPER ? sx.wallpaperHeight : sx.backgroundHeight),
            } as SxProps
          }
        >
          <Blurhash hash={image.blurhash} width={500} height={150} resolutionX={32} resolutionY={32} punch={1} />
          <motion.div
            animate={loaded ? 'loaded' : 'loading'}
            className="motion-div"
            variants={{ loaded: { opacity: 1 }, loading: { opacity: 0 } }}
          >
            <img src={image.thumbnail_url} alt={image.name} onLoad={() => setLoaded(true)} />
          </motion.div>
        </Box>
        <CardHeader
          avatar={<Avatar sx={sx.cardHeaderAvatar as SxProps}>&#10003;</Avatar>}
          subheader={formatRelative(new Date(image.created_at), new Date())}
          sx={sx.cardHeader as SxProps}
          title={image.name}
        />
      </CardActionArea>
    </Card>
  );
};
