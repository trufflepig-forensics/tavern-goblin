import { createRootRoute, ErrorComponentProps, Outlet } from "@tanstack/react-router";
import { Text } from "src/components/base/text";
import React from "react";
import { Heading } from "src/components/base/heading";
import { Button, PrimaryButton } from "src/components/base/button";
import { ErrorContext } from "src/context/error-context";

/**
 * The root error component
 */
function ErrorComponent(props: ErrorComponentProps) {
    return (
        <div className={"flex h-screen w-full items-center justify-center"}>
            <div
                className={
                    "min-w-sm flex flex-col gap-6 rounded-lg border border-zinc-300 bg-white p-12 dark:border-zinc-800 dark:bg-zinc-900"
                }
            >
                <Heading>{props.error.toString()}</Heading>
                <Text>{props.info?.componentStack}</Text>

                <PrimaryButton className={"w-full"} onClick={() => props.reset()}>
                    Try again
                </PrimaryButton>

                <Button onClick={() => history.back()}>Back</Button>
            </div>
        </div>
    );
}

export const Route = createRootRoute({
    // eslint-disable-next-line
    component: () => (
        <>
            <ErrorContext />
                <Outlet />
        </>
    ),
    // eslint-disable-next-line
    errorComponent: (err) => <ErrorComponent {...err} />,
});
