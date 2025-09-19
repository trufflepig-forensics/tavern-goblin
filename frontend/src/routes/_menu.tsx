import { createFileRoute, Outlet } from "@tanstack/react-router";

import React from "react";
import { useTranslation } from "react-i18next";
import {
    Sidebar,
    SidebarBody,
    SidebarFooter,
    SidebarHeader,
    SidebarHeading,
    SidebarItem,
    SidebarLabel,
    SidebarSection,
} from "src/components/base/sidebar";
import { SidebarLayout } from "src/components/base/sidebar-layout";
import { Navbar } from "src/components/base/navbar";
import LogoText from "src/assets/logo_text.svg?react";
import {
    ArrowRightStartOnRectangleIcon,
    ChevronUpIcon,
    CurrencyEuroIcon,
    ReceiptRefundIcon,
    UserIcon,
} from "@heroicons/react/20/solid";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import ACCOUNT_CONTEXT from "src/context/account";

/**
 * The properties for {@link Menu}
 */
export type MenuProps = {};

/**
 * Menu of the alerting tool
 */
export default function Menu(props: MenuProps) {
    const [t] = useTranslation("menu");

    const userContext = React.useContext(ACCOUNT_CONTEXT);

    return (
        <SidebarLayout
            sidebar={
                <Sidebar>
                    <SidebarHeader>
                        <LogoText
                            className={"h-20 fill-[#233F94] p-3 drop-shadow-lg dark:fill-white  dark:drop-shadow-none"}
                        />
                    </SidebarHeader>
                    <SidebarBody>
                        <SidebarSection>
                            <SidebarHeading>{t("heading.dinner")}</SidebarHeading>
                            <SidebarItem href={"/dinner"}>
                                <ReceiptRefundIcon />
                                <SidebarLabel>{t("label.overview")}</SidebarLabel>
                            </SidebarItem>
                        </SidebarSection>
                        <SidebarSection>
                            <SidebarHeading>{t("heading.personal")}</SidebarHeading>
                            <SidebarItem>
                                <CurrencyEuroIcon />
                                <SidebarLabel>{t("label.balance")}</SidebarLabel>
                            </SidebarItem>
                        </SidebarSection>
                    </SidebarBody>
                    <SidebarFooter className={"max-lg:hidden"}>
                        <Dropdown>
                            <DropdownButton as={SidebarItem}>
                                <span className="grid h-10 w-full min-w-0 grid-cols-[1fr_auto] items-center gap-3">
                                    <span className="block truncate text-sm/5 font-medium text-zinc-950 dark:text-white">
                                        {/*userContext.account.display_name*/}
                                    </span>
                                    <ChevronUpIcon className={"size-4"} />
                                </span>
                            </DropdownButton>
                            <DropdownMenu anchor={"top end"}>
                                <DropdownItem href={"/profile"}>
                                    <UserIcon />
                                    <DropdownLabel>{t("button.profile")}</DropdownLabel>
                                </DropdownItem>
                                <DropdownItem
                                    onClick={() => {
                                        /*Api.auth.logout().then(() => {
                                            userContext.reset();
                                        });*/
                                    }}
                                >
                                    <ArrowRightStartOnRectangleIcon />
                                    <DropdownLabel>{t("button.sign-out")}</DropdownLabel>
                                </DropdownItem>
                            </DropdownMenu>
                        </Dropdown>
                    </SidebarFooter>
                </Sidebar>
            }
            navbar={<Navbar></Navbar>}
        >
            <Outlet />
        </SidebarLayout>
    );
}

export const Route = createFileRoute("/_menu")({
    component: Menu,
});
