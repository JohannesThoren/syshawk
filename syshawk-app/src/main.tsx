import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import Router from "./Router.tsx";
import {QueryClientProvider} from "@tanstack/react-query";
import {query} from "./shared/query.ts";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <QueryClientProvider client={query}>
        <Router/>
    </QueryClientProvider>
  </StrictMode>,
)
