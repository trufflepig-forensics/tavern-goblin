import React from "react";
import { Outlet } from "react-router";

/**
 * The properties for the {@link Menu}
 */
export type MenuProps = {};

/**
 * The main menu component
 */
export default function Menu(props: MenuProps) {
    return (
        <div>
            <Outlet />
        </div>
    );
}
