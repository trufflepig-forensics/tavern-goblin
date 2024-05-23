import {createBrowserRouter, createRoutesFromElements, Route} from "react-router-dom";
import React from "react";
import Menu from "./views/menu";
import Error from "./views/error";


/**
 * An element in the router
 */
class PathElement {
    readonly path;
    readonly element;

    /**
     * The constructor for a path element
     *
     * @param path The path for the route
     * @param element The element that should be rendered
     */
    constructor(path: string, element: React.ReactElement) {
        this.path = path;
        this.element = element;
    }

    getRoute() {
        return <Route path={this.path} element={this.element}></Route>;
    }
}

export const ROUTER = {
    HOME: new PathElement("/", <div></div>),
};

export const router = createBrowserRouter(createRoutesFromElements(
    <Route element={<Menu/>} errorElement={<Error/>}>
        {Object.values(ROUTER).map((x) => x.getRoute())}
    </Route>
));