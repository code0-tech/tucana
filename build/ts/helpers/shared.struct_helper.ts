import {Value} from "../pb/shared.struct_pb.js";

export type PlainValue = null | number | string | boolean | Array<PlainValue> | object;

export function toAllowedValue(value: Value): PlainValue {
    switch (value.kind.oneofKind) {
        case "nullValue":
            return null;
        case "numberValue":
            return value.kind.numberValue;
        case "stringValue":
            return value.kind.stringValue;
        case "boolValue":
            return value.kind.boolValue;
        case "listValue":
            return value.kind.listValue.values.map(toAllowedValue);
        case "structValue":
            const obj: {[key: string]: PlainValue} = {};
            for (const [k, v] of Object.entries(value.kind.structValue.fields)) {
                obj[k] = toAllowedValue(v);
            }
            return obj;
        default:
            throw new Error(`Unsupported Value kind: ${value.kind.oneofKind}`);
    }
}


export function constructValue(value: PlainValue): Value {
    if (value === null) {
        return {kind: {oneofKind: "nullValue", nullValue: 0}};
    }
    if (typeof value === "number") {
        return {kind: {oneofKind: "numberValue", numberValue: value}};
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
        const structFields: {[key: string]: Value} = {};
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
