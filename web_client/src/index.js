import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter } from "react-router-dom";
import "./assets/css/index.css";
import App from "./App";
import * as serviceWorker from "./serviceWorker";

// Apollo
import { ApolloProvider } from "@apollo/client";
import { ApolloClient, InMemoryCache } from "@apollo/client";
import { createHttpLink } from "apollo-link-http";
const link = createHttpLink({
  uri: "http://192.168.1.120:3300/",
  credentials: "same-origin",
  //   useGETForQueries: false,
  //   fetchOptions: {
  //     mode: "no-cors",
  //   },
});
const client = new ApolloClient({
  cache: new InMemoryCache(),
  link: link,
});
ReactDOM.render(
  <React.StrictMode>
    <BrowserRouter>
      <ApolloProvider client={client}>
        <App />
      </ApolloProvider>
    </BrowserRouter>
  </React.StrictMode>,
  document.getElementById("root")
);

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
