import axios from "axios";

const baseApi = axios.create({
  baseURL: "http://yt.sonbn.xyz",
});

export { baseApi };
