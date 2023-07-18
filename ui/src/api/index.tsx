import axios from "axios";

import projectsApi from "./projects";
import versionsApi from "./versions";

const instance = axios.create({
  baseURL: process.env.REACT_APP_API_URL,
  timeout: 30000,
});

// instance.interceptors.request.use(
//   function (config) {
//     const token = localStorage.getItem("token");
//     if (config.headers && token) {
//       config.headers["Authorization"] = `Bearer ${token}`;
//     }
//     // Do something before request is sent
//     return config;
//   },
//   function (error) {
//     // Do something with request error
//     return Promise.reject(error);
//   }
// );

const api = {
  projects: projectsApi(instance),
  versions: versionsApi(instance),
};

export default api;
