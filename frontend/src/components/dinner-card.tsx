import React from "react";
import { useTranslation } from "react-i18next";
import { Subheading } from "src/components/base/heading";
import { Button } from "src/components/base/button";
import { Text } from "src/components/base/text";

/**
 * The properties for {@link DinnerCardOverview}
 */
export type DinnerCardOverviewProps = {};

/**
 * A card that shows a dinner
 */
export default function DinnerCardOverview(props: DinnerCardOverviewProps) {
    const [t] = useTranslation("dinner");

    /**
     * Add a member to the dinner
     */
    const addMemberToDinner = () => {};

    return (
        <div className={"rounded-lg border border-zinc-300 p-2 dark:border-zinc-700"}>
            <div className={"flex justify-between gap-2"}>
                <Subheading>Test Mahlzeit</Subheading>
                <Button onClick={() => addMemberToDinner()}>{t("button.plus-one")}</Button>
            </div>
            <Text>{t("description.cooked-by", { object: "Niklas" })}</Text>
        </div>
    );
}
