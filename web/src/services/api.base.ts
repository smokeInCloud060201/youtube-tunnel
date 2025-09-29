import axios from "axios";
import { URL_BASE_HOST } from "@/utils/app.config.ts";

const baseApi = axios.create({
  baseURL: URL_BASE_HOST,
});

export { baseApi };
