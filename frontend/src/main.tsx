import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import {
  QueryClient,
  QueryClientProvider,
} from 'react-query'
import { ThemeProvider } from "@/components/theme-provider.tsx";
import "./index.css";

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ThemeProvider>
      <QueryClientProvider client={queryClient}>
        <App />
      </QueryClientProvider>
    </ThemeProvider>
  </React.StrictMode>
);
