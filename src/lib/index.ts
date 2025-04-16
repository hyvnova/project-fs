export type ParseFail = { Invalid: string } | {
    Conflicting: {
        name: string,
        causator: string
    }
} | { Unexsistent: string }