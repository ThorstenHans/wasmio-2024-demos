export function transform(input) {
    if (!input) {
        return input;
    }
    return input.split("").reverse().join("");
};