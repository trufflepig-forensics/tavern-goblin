import { createFileRoute } from "@tanstack/react-router";
import React, { useEffect } from "react";
import { useTranslation } from "react-i18next";
import HeadingLayout from "src/components/base/heading-layout";
import { Heading, Subheading } from "src/components/base/heading";
import { Text } from "src/components/base/text";
import { Button, PrimaryButton } from "src/components/base/button";
import { PlusCircleIcon } from "@heroicons/react/20/solid";

/**
 * The properties for {@link ReceiptManagementOverview}
 */
export type ReceiptManagementOverviewProps = {};

/**
 * Overview for managing receipt
 */
export default function ReceiptManagementOverview(props: ReceiptManagementOverviewProps) {
    const [t] = useTranslation("dinner");

    const [dates, setDates] = React.useState<Date[]>([]);

    const now = new Date();
    const month_name = now.toLocaleString("default", { month: "long" });
    const year = now.getFullYear();
    const startOfWeek = new Date(year, now.getMonth(), now.getDate() - now.getDay() + 1);

    useEffect(() => {
        const nextFiveWeekdays = Array.from({ length: 5 }, (_, idx) => {
            const d = new Date(startOfWeek);
            d.setDate(d.getDate() + idx);
            return d;
        });
        setDates(nextFiveWeekdays);
    }, [startOfWeek.getTime()]);

    return (
        <HeadingLayout heading={t("heading.dinner-overview")}>
            <div className={"flex flex-none items-center justify-between px-6 py-4"}>
                <Heading>
                    <time dateTime={"2022-01"}>{month_name + " " + year}</time>
                </Heading>
            </div>
            <div className={"grid grid-cols-5 gap-2"}>
                {dates.map((d) => (
                    <div
                        className={
                            "flex items-center justify-center rounded-lg border border-zinc-300 p-2 dark:border-zinc-700"
                        }
                        key={d.getTime()}
                    >
                        <Subheading>
                            <time dateTime={d.toISOString()}>
                                {d.toLocaleDateString(undefined, {
                                    weekday: "short",
                                    day: "2-digit",
                                })}
                            </time>
                        </Subheading>
                    </div>
                ))}
            </div>
        </HeadingLayout>
    );
}

export const Route = createFileRoute("/_menu/dinner/")({
    component: ReceiptManagementOverview,
});
