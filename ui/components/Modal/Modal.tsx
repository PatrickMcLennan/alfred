import React, { FC, ReactNode, useState } from 'react';
import { Box, IconButton, Modal as MuiModal, Theme, Typography } from '@mui/material';
import CloseIcon from '@mui/icons-material/Close';

type Props = {
  children: ReactNode;
  open: boolean;
  onClose: () => any;
  subtitle?: string;
  title: string;
};

const sx = {
  backdrop: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
  },
  body: {},
  closeButton: {
    backgroundColor: 'white',

    '&:hover > svg': {
      color: 'white',
    },
  },
  closeIcon: {
    color: 'black',

    '&:hover': {
      color: 'white',
    },
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center ',
    borderBottom: '1px solid white',
    padding: (theme: Theme) => theme.spacing(2),
    marginBottom: (theme: Theme) => theme.spacing(4),
  },
  modal: {
    border: '1px solid white',
    borderRadius: '3px',
    padding: (theme: Theme) => theme.spacing(2.5),
    width: '85vw',
    backgroundColor: (theme: Theme) => theme.palette.background.default,
  },
  subtitle: {
    color: (theme: Theme) => theme.palette.primary.main,
  },
  title: {},
} as const;

export const Modal: FC<Props> = ({ children, open, onClose, subtitle, title }) => {
  const id = `${title}-modal-title`;
  return (
    <MuiModal aria-describedby={id} open={open} onClose={onClose} sx={sx.backdrop}>
      <Box sx={sx.modal}>
        <Box component="header" sx={sx.header}>
          {subtitle ? (
            <Box>
              <Typography id={id} component="h2" sx={sx.title} variant="h4">
                {title}
              </Typography>
              <Typography component="h3" sx={sx.subtitle} variant="h5">
                {subtitle}
              </Typography>
            </Box>
          ) : (
            <Typography id={id} component="h2" sx={sx.title} variant="h3">
              {title}
            </Typography>
          )}
          <IconButton onClick={onClose} sx={sx.closeButton}>
            <CloseIcon sx={sx.closeIcon} />
          </IconButton>
        </Box>
        <Box sx={sx.body}>{children}</Box>
      </Box>
    </MuiModal>
  );
};
