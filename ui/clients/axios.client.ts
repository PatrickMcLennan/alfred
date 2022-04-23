import axios, { AxiosRequestConfig } from 'axios';

export const axiosClient = axios.create({
  baseURL: `/api`,
  withCredentials: true,
});

export const axiosFetcher = (config: AxiosRequestConfig) => axiosClient(config).then(({ data }) => data);
