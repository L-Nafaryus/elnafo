import axios, { type AxiosInstance } from "axios";

const api_client: AxiosInstance = axios.create({
    baseURL: import.meta.hot ? "http://localhost:54600/api/v1" : "/api/v1",
    headers: {
        "Content-Type": "application/json"
    },
    withCredentials: true,
});

export default api_client;
