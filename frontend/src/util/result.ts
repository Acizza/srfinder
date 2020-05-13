export interface Ok<T> {
  kind: "ok";
  value: T;
}

export function ok<T>(value: T): Ok<T> {
  return { kind: "ok", value };
}

export interface Err<T> {
  kind: "err";
  value: T;
}

export function err<T>(value: T): Err<T> {
  return { kind: "err", value };
}

type Result<T, E> = Ok<T> | Err<E>;

export default Result;
