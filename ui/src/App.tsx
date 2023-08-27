import { ChakraProvider, theme } from "@chakra-ui/react";
import { BrowserRouter } from "react-router-dom";
import Router from "./router";

import "./app.css";

export const App = () => (
  <ChakraProvider theme={theme}>
    <BrowserRouter>
      <Router />
    </BrowserRouter>
  </ChakraProvider>
);
