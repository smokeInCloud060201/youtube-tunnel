import axios from "axios";
import { URL_BASE_HOST } from "@/utils/app.config.ts";

const baseApi = axios.create({
  baseURL: URL_BASE_HOST,
  timeout: 30000, // 30 second timeout
});

// Request interceptor for logging (optional, can be enhanced)
baseApi.interceptors.request.use(
  (config) => {
    // Add timestamp to prevent caching issues
    if (config.method === 'get') {
      config.params = {
        ...config.params,
        _: Date.now(),
      };
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Response interceptor for error handling
baseApi.interceptors.response.use(
  (response) => response,
  (error) => {
    // Handle common errors
    if (error.response) {
      // Server responded with error status
      console.error('API Error:', error.response.status, error.response.data);
    } else if (error.request) {
      // Request made but no response received
      console.error('Network Error:', error.request);
    } else {
      // Error setting up request
      console.error('Error:', error.message);
    }
    return Promise.reject(error);
  }
);

export { baseApi };
