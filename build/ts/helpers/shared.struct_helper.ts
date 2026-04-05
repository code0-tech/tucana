import {Value} from "../pb/shared";

type PlainValue = null | bigint | number | string | boolean | Array<PlainValue> | object;

function toAllowedValue(value: Value): PlainValue {
    switch (value.kind.oneofKind) {
        case "nullValue":
            return null;
        case "numberValue":
            const numberValue = value.kind.numberValue;
            if (numberValue.number.oneofKind === "integer") {
                const numberResult = numberValue.number.integer
                if (Number.MAX_SAFE_INTEGER < numberResult || numberResult < Number.MIN_SAFE_INTEGER) {
                    return numberResult
                }

                return Number(numberResult)
            } else if (numberValue.number.oneofKind === "float") {
                return numberValue.number.float;
            } else  {
                throw new Error("Unsupported NumberValue number kind: " + numberValue.number.oneofKind);
            }
        case "stringValue":
            return value.kind.stringValue;
        case "boolValue":
            return value.kind.boolValue;
        case "listValue":
            return value.kind.listValue.values.map(toAllowedValue);
        case "structValue":
            const obj: { [key: string]: PlainValue } = {};
            for (const [k, v] of Object.entries(value.kind.structValue.fields)) {
                obj[k] = toAllowedValue(v);
            }
            return obj;
        default:
            throw new Error(`Unsupported Value kind: ${value.kind.oneofKind}`);
    }
}


function constructValue(value: PlainValue): Value {
    if (value === null) {
        return {kind: {oneofKind: "nullValue", nullValue: 0}};
    }
    if (typeof value === "number" || typeof value === "bigint") {
        if (Number.isInteger(value) || typeof value === "bigint") {
            return {
                kind: {
                    oneofKind: "numberValue", numberValue: {
                        number: {
                            oneofKind: "integer",
                            integer: BigInt(value)
                        }
                    }
                }
            };
        }

        return {
            kind: {
                oneofKind: "numberValue", numberValue: {
                    number: {
                        oneofKind: "float",
                        float: value
                    }
                }
            }
        };
    }
    if (typeof value === "string") {
        return {kind: {oneofKind: "stringValue", stringValue: value}};
    }
    if (typeof value === "boolean") {
        return {kind: {oneofKind: "boolValue", boolValue: value}};
    }
    if (Array.isArray(value)) {
        const listValues = value.map(v => constructValue(v));
        return {
            kind: {
                oneofKind: "listValue",
                listValue: {values: listValues}
            }
        };
    }
    if (typeof value === "object") {
        const structFields: { [key: string]: Value } = {};
        for (const [k, v] of Object.entries(value)) {
            structFields[k] = constructValue(v);
        }
        return {
            kind: {
                oneofKind: "structValue",
                structValue: {fields: structFields}
            }
        };
    }
    throw new Error(`Unsupported value type: ${typeof value}`);
}

export {
    toAllowedValue,
    constructValue,
    PlainValue
}
