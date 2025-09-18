import clsx from "clsx";
import type React from "react";
import { Button } from "src/components/base/button";
import { LinkProps } from "src/components/base/link";

export function Pagination({
    "aria-label": ariaLabel = "Page navigation",
    className,
    ...props
}: React.ComponentPropsWithoutRef<"nav">) {
    return <nav aria-label={ariaLabel} {...props} className={clsx(className, "flex gap-x-2")} />;
}

export function PaginationPrevious({
    href = null,
    params,
    search,
    className,
    children = "Previous",
    onClick,
}: React.PropsWithChildren<{
    href?: LinkProps["href"] | null;
    params?: LinkProps["params"];
    search?: LinkProps["search"];
    className?: string;
    onClick?: () => void;
}>) {
    return (
        <span className={clsx(className, "grow basis-0")}>
            <Button
                onClick={onClick}
                {...(href === null ? { disabled: true } : { href })}
                params={params}
                search={search}
                plain
                aria-label="Previous page"
            >
                <svg className="stroke-current" data-slot="icon" viewBox="0 0 16 16" fill="none" aria-hidden="true">
                    <path
                        d="M2.75 8H13.25M2.75 8L5.25 5.5M2.75 8L5.25 10.5"
                        strokeWidth={1.5}
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    />
                </svg>
                {children}
            </Button>
        </span>
    );
}

export function PaginationNext({
    href = null,
    params,
    search,
    className,
    children = "Next",
    onClick,
}: React.PropsWithChildren<{
    href?: LinkProps["href"] | null;
    params?: LinkProps["params"];
    search?: LinkProps["search"];
    className?: string;
    onClick?: () => void;
}>) {
    return (
        <span className={clsx(className, "flex grow basis-0 justify-end")}>
            <Button
                onClick={onClick}
                {...(href === null ? { disabled: true } : { href })}
                params={params}
                search={search}
                plain
                aria-label="Next page"
            >
                {children}
                <svg className="stroke-current" data-slot="icon" viewBox="0 0 16 16" fill="none" aria-hidden="true">
                    <path
                        d="M13.25 8L2.75 8M13.25 8L10.75 10.5M13.25 8L10.75 5.5"
                        strokeWidth={1.5}
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    />
                </svg>
            </Button>
        </span>
    );
}

export function PaginationList({ className, ...props }: React.ComponentPropsWithoutRef<"span">) {
    return <span {...props} className={clsx(className, "hidden items-baseline gap-x-2 sm:flex")} />;
}

export function PaginationPage({
    className,
    current = false,
    children,
    onClick,
    ...other
}: React.PropsWithChildren<{
    href?: LinkProps["href"];
    params?: LinkProps["params"];
    search?: LinkProps["search"];
    className?: string;
    current?: boolean;
    onClick?: () => void;
}>) {
    return (
        <Button
            {...other}
            onClick={onClick}
            plain
            aria-label={`Page ${children}`}
            aria-current={current ? "page" : undefined}
            className={clsx(
                className,
                "min-w-[2.25rem] before:absolute before:-inset-px before:rounded-lg",
                current && "before:bg-zinc-950/5 dark:before:bg-white/10",
            )}
        >
            <span className="-mx-0.5">{children}</span>
        </Button>
    );
}

export function PaginationGap({
    className,
    children = <>&hellip;</>,
    ...props
}: React.ComponentPropsWithoutRef<"span">) {
    return (
        <span
            aria-hidden="true"
            {...props}
            className={clsx(
                className,
                "w-[2.25rem] select-none text-center text-sm/6 font-semibold text-zinc-950 dark:text-white",
            )}
        >
            {children}
        </span>
    );
}
