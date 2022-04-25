import React, { FC, useCallback, useEffect, useRef, useState } from 'react';
import { Modal } from '../';
import { Box, IconButton, SxProps, Theme, Typography } from '@mui/material';
import { useNavigate, useParams } from 'react-router-dom';
import { useWallpapers } from '../../stores';
import { Wallpaper } from '../../../lib/ts';
import { axiosClient } from '../../clients';
import { formatRelative } from 'date-fns';
import { Blurhash } from 'react-blurhash';
import { motion } from 'framer-motion';
import OpenInNewIcon from '@mui/icons-material/OpenInNew';
import DeleteIcon from '@mui/icons-material/Delete';
import { DeleteDialog } from '../DeleteDialog';

type Params = {
  sk?: string;
};

type Props = {
  onClose: (...props: any) => any;
};

const sx = {
  buttonBox: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    gap: (theme: Theme) => theme.spacing(4),
    marginTop: (theme: Theme) => theme.spacing(4),
  },
  deleteDialogText: {
    textAlign: 'center',
  },
  deleteIcon: {
    backgroundColor: (theme: Theme) => theme.palette.error.main,

    '& svg': {
      color: 'white',
    },
  },
  imageWrapper: {
    position: 'relative',
    height: [200, 350, 450, 550, 550],

    '& .motion-div': {
      position: 'absolute',
      top: 0,
      right: 0,
      bottom: 0,
      left: 0,
      width: '100%',

      '& img': {
        display: 'block',
        height: `100%`,
        width: '100%',
        objectFit: 'cover',
      } as const,
    } as const,
  } as const,
} as const;

export const WallpaperModal: FC<Props> = ({ onClose }) => {
  const { removeWallpaper, wallpapers } = useWallpapers(({ removeWallpaper, wallpapers }) => ({
    removeWallpaper,
    wallpapers,
  }));
  const { sk } = useParams<Params>();
  const navigate = useNavigate();
  const hasCalled = useRef<boolean>(false);
  const [showDeleteDialog, setShowDeleteDialog] = useState<boolean>(false);
  const [imageLoaded, setImageLoaded] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const [wallpaper, setWallpaper] = useState<Wallpaper | null>(
    wallpapers.find((wallpaper) => wallpaper.sk === sk) ?? null // TS is being weird here
  );

  const ignoreWallpaper = async () => {
    if (!sk) return;
    setLoading(true);
    setShowDeleteDialog(false);
    try {
      await axiosClient({
        method: `POST`,
        url: `/wallpapers/ignore`,
        data: {
          sk,
          ignored: true,
        },
      }).then(() => {
        navigate(-1);
        removeWallpaper(sk);
      });
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

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

  return (
    <>
      <DeleteDialog
        handleClose={() => setShowDeleteDialog(false)}
        open={showDeleteDialog}
        deleteAction={ignoreWallpaper}
        title={`Add ${sk} to Ignore List?`}
      >
        <Typography sx={sx.deleteDialogText}>This can be reversed later.</Typography>
      </DeleteDialog>
      <Modal
        title={sk ?? 'Wallpaper'}
        subtitle={wallpaper?.created_at?.toString ? formatRelative(new Date(), wallpaper.created_at) : ` `}
        onClose={onClose}
        open={true}
      >
        <Box component="figure">
          <Box sx={sx.imageWrapper as SxProps}>
            <Blurhash
              hash={wallpaper?.blurhash ?? ''}
              width={`100%`}
              height={`100%`}
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
                <img src={wallpaper.url} alt={wallpaper.name} onLoad={() => setImageLoaded(true)} />
              </motion.div>
            )}
          </Box>
          <Box component="figcaption" sx={sx.buttonBox}>
            <IconButton
              aria-label="Open full size image in new tab"
              id="wallpaper-modal-open-button"
              onClick={() => window.open(wallpaper?.url, `_blank`)}
            >
              <OpenInNewIcon aria-describedby="wallpaper-modal-open-button" />
            </IconButton>
            <IconButton sx={sx.deleteIcon} onClick={() => setShowDeleteDialog(true)}>
              <DeleteIcon />
            </IconButton>
          </Box>
        </Box>
      </Modal>
    </>
  );
};
