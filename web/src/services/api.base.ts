import axios from "axios";

const baseApi = axios.create({
  baseURL: "https://yt.sonbn.xyz",
});

export { baseApi };
