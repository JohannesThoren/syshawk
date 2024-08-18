import {BrowserRouter, Route, Routes} from "react-router-dom";
import Home from "./pages/Home.tsx";
import Single from "./pages/single.tsx";

export default function Router() {
    return <BrowserRouter>
        <Routes>
            <Route path={"system"}>
                <Route path={":id"} element={<Single/>}/>
            <Route index element={<Home/>}/>
                </Route>
        </Routes>
    </BrowserRouter>
}