import axios, { type AxiosInstance } from "axios";

const api_client: AxiosInstance = axios.create({
    baseURL: import.meta.hot ? "http://localhost:54600/api" : "/api",
    headers: {
        "Content-Type": "application/json"
    },
    withCredentials: true,
});

export const api_client_upload: AxiosInstance = axios.create({
    baseURL: import.meta.hot ? "http://localhost:54600/api" : "/api",
    headers: {
        "Content-Type": "multipart/form-data"
    },
    withCredentials: true,
});

export default api_client;
