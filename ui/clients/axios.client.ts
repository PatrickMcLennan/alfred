import axios from 'axios';
import { makeUseAxios } from 'axios-hooks';

export const axiosClient = axios.create({
  baseURL: `/api`,
  withCredentials: true,
});

export const useCustomAxios = makeUseAxios({
  axios: axiosClient,
});
