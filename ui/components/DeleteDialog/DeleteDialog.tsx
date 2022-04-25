import React, { FC, ReactNode } from 'react';
import { Box, Button, Dialog, DialogActions, DialogContent, DialogTitle, IconButton, Theme } from '@mui/material';
import CloseIcon from '@mui/icons-material/Close';

type Props = {
  children: ReactNode;
  handleClose: (...props: any) => any;
  open: boolean;
  deleteAction: (...props: any) => any;
  title: string;
};

const sx = {
  button: {
    flex: 1,
  },
  buttonBox: {
    display: 'flex',
    justifyContent: 'flex-start',
    alignItems: 'stretch',
    marginTop: (theme: Theme) => theme.spacing(4),
    borderTop: `1px solid white`,
  },
  closeButton: {
    backgroundColor: (theme: Theme) => theme.palette.background.default,
  },
  closeIconButton: {
    backgroundColor: 'white',
    marginRight: `2.4rem`, // x padding on the H2 in the header

    '&:hover > svg': {
      color: 'white',
    },
  },
  deleteButton: {
    backgroundColor: (theme: Theme) => theme.palette.error.main,
  },
  dialog: {
    padding: (theme: Theme) => theme.spacing(2.5),
    width: '100vw',
    backgroundColor: `rgba(0,0,0,0.5)`,
    backgroundImage: `linear-gradient(to right bottom, rgba(0,0,0,0.5), rgba(0,0,0,0.5))`,

    '& .MuiPaper-Root': {
      border: '1px solid white',
      borderRadius: '3px',
    },
  },
  body: {},
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
    // padding: (theme: Theme) => theme.spacing(2),
    marginBottom: (theme: Theme) => theme.spacing(4),
  },
  title: {},
} as const;

export const DeleteDialog: FC<Props> = ({ children, handleClose, open, deleteAction, title }) => {
  return (
    <Dialog onClose={handleClose} open={open} sx={sx.dialog}>
      <Box component="header" sx={sx.header}>
        <DialogTitle>{title}</DialogTitle>
        <IconButton onClick={handleClose} sx={sx.closeIconButton}>
          <CloseIcon sx={sx.closeIcon} />
        </IconButton>
      </Box>
      <DialogContent sx={sx.body}>{children}</DialogContent>
      <DialogActions sx={sx.buttonBox}>
        <Button onClick={handleClose} sx={{ ...sx.button, ...sx.closeButton }}>
          nah
        </Button>
        <Button onClick={deleteAction} sx={{ ...sx.button, ...sx.deleteButton }}>
          Yes
        </Button>
      </DialogActions>
    </Dialog>
  );
};
