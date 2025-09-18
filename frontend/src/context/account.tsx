import React from "react";

/** The global {@link AccountProvider} instance */
let ACCOUNT_PROVIDER: AccountProvider | null = null;

/** Data provided by the {@link ACCOUNT_CONTEXT} */
export type AccountContext = {
    /** The currently logged-in account */
    account: {};

    /**  */
    reset: () => void;
};

/** {@link React.Context} to access {@link GetMeResponse account information} */
const ACCOUNT_CONTEXT = React.createContext<AccountContext>({
    account: {
        uuid: "",
        display_name: "",
    },

    /**
     * Reset the account's interface
     */
    reset: () => {},
});
ACCOUNT_CONTEXT.displayName = "AccountContext";
export default ACCOUNT_CONTEXT;

/**
 * The properties of the account provider
 */
type AccountProviderProps = {
    /** The children of the properties */
    children: React.ReactNode | Array<React.ReactNode>;
};

/** State of the account provider */
type AccountProviderState = {
    /** The account */
    account: {} | "loading";
};

/** Provider for the account information */
export class AccountProvider extends React.Component<AccountProviderProps, AccountProviderState> {
    state: AccountProviderState = { account: "loading" };

    fetching: boolean = false;

    /**
     * Fetch the account
     */
    fetchAccount = () => {
        if (this.fetching) return;
        this.fetching = true;

        this.setState({ account: "loading" });

        //Api.account.getMe().then((result) => this.setState({ account: result }));

        // Clear guard against a lot of calls
        this.fetching = false;
    };

    /**
     * Hook when the component mounts
     */
    componentDidMount() {
        this.fetchAccount();

        // Register as global singleton
        // eslint-disable-next-line @typescript-eslint/no-this-alias
        if (ACCOUNT_PROVIDER === null) ACCOUNT_PROVIDER = this;
        else if (ACCOUNT_PROVIDER === this) console.error("AccountProvider did mount twice");
        else console.error("Two instance of AccountProvider are used");
    }

    /**
     * Hook when the component will unmount
     */
    componentWillUnmount() {
        // Deregister as global singleton
        if (ACCOUNT_PROVIDER === this) ACCOUNT_PROVIDER = null;
        else if (ACCOUNT_PROVIDER === null) console.error("AccountProvider did unmount twice");
        else console.error("Two instance of AccountProvider are used");
    }

    /**
     * The render function
     *
     * @returns The JSX component
     */
    render() {
        switch (this.state.account) {
            case "loading":
                return <div></div>;

            default:
                return (
                    <ACCOUNT_CONTEXT.Provider value={{ account: this.state.account, reset: this.fetchAccount }}>
                        {this.props.children}
                    </ACCOUNT_CONTEXT.Provider>
                );
        }
    }
}
