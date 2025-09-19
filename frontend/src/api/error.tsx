export enum StatusCode {
    JsonDecodeError = -1,

    Unauthenticated = 1000,
}

/**
 * The outer error the api returns. This is most likely not deal-able by the frontend
 */
export type ApiError = {
    /** The status code */
    status_code: StatusCode;
    /** The human-readable message */
    message: string;
};

/**
 * Helper function to parse errors
 *
 * @param response The response of the request
 *
 * @returns Promise of with an ApiError
 */
export async function parseError(response: Response): Promise<ApiError> {
    try {
        return await response.json();
    } catch {
        console.error("Got invalid json", response.body);
        return {
            status_code: StatusCode.JsonDecodeError,
            message: "The server's response was invalid json",
        };
    }
}